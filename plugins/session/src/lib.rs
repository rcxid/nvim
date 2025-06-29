use mlua::prelude::{LuaResult, LuaTable};
use mlua::Error::RuntimeError;
use mlua::{IntoLua, Lua, Value};
use plugin::Plugin;
use rusqlite::Connection;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const PLUGIN_NAME: &str = "session";

/// nvim session管理插件
pub struct Session<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
    path: String,
    database: String,
}

pub struct SessionData {
    path: String,
    data: String,
}

impl IntoLua for SessionData {
    fn into_lua(self, lua: &Lua) -> LuaResult<Value> {
        let table = lua.create_table()?;
        table.set("path", self.path)?;
        table.set("data", self.data)?;
        Ok(Value::Table(table))
    }
}

pub struct SessionPath {
    plugin: String,
    database: String,
}

impl SessionPath {
    fn try_new(lua: &Lua) -> LuaResult<Self> {
        let data_path = api::builtin_fn::stdpath(lua, "data")?;
        let plugin_path = format!("{}/plugins/{}", data_path, PLUGIN_NAME);
        let database_path = format!("{plugin_path}/sqlite.db");
        Ok(Self {
            plugin: plugin_path,
            database: database_path,
        })
    }
}

impl<'lua> Session<'lua> {
    fn init_database(&self) -> LuaResult<()> {
        // 创建插件数据目录
        fs::create_dir_all(self.path.as_str())?;
        let conn = self.connect_database()?;
        let table_create_sql = r#"
          CREATE TABLE IF NOT EXISTS session (
            -- workspace path
            path TEXT PRIMARY KEY,
            -- session data path
            data TEXT NOT NULL UNIQUE
          )"#;
        conn.execute(table_create_sql, ())
            .map_err(|_| RuntimeError("session plugin exec sql failed!".to_string()))?;
        Ok(())
    }

    fn init_function(&self) -> LuaResult<()> {
        // 注册方法
        self.plugin.set(
            "make_session",
            self.runtime.create_function(Session::make_session)?,
        )?;
        self.plugin.set(
            "session_list",
            self.runtime.create_function(Session::session_list)?,
        )?;
        self.plugin.set(
            "clean_session",
            self.runtime.create_function(Session::clean_session)?,
        )?;
        Ok(())
    }

    fn connect_database(&self) -> LuaResult<Connection> {
        Self::_connect_database(self.database.as_str())
    }

    fn _connect_database(database: &str) -> LuaResult<Connection> {
        Connection::open(database)
            .map_err(|_| RuntimeError("session plugin connect sqlite database failed!".to_string()))
    }

    fn query_session(lua: &Lua, workspace_path: &str) -> LuaResult<SessionData> {
        let session_path = SessionPath::try_new(lua)?;
        let conn = Self::_connect_database(session_path.database.as_str())?;
        let query_sql = r#"
          SELECT
            *
          FROM session
          WHERE path = ?1;
        "#;
        let session = conn
            .query_one(query_sql, [workspace_path], |row| {
                Ok(SessionData {
                    path: row.get(0)?,
                    data: row.get(1)?,
                })
            })
            .map_err(|_| RuntimeError("session plugin query session failed!".to_string()))?;
        Ok(session)
    }

    fn make_session(lua: &Lua, (): ()) -> LuaResult<()> {
        let cwd = api::builtin_fn::getcwd(lua)?;
        let cmd = if let Ok(session) = Self::query_session(lua, cwd.as_str()) {
            format!("mks! {}", session.data)
        } else {
            let session_path = SessionPath::try_new(lua)?;
            let file_name = api::util::generate_random_string(8);
            let file_path = format!("{}/{}.vim", session_path.plugin, file_name);
            let cmd = format!("mks! {}", file_path);
            let data = SessionData {
                path: cwd,
                data: file_path,
            };
            Self::save_session(lua, data)?;
            cmd
        };
        api::cmd(lua, cmd)?;
        Ok(())
    }

    fn save_session(lua: &Lua, session: SessionData) -> LuaResult<()> {
        let session_path = SessionPath::try_new(lua)?;
        let conn = Self::_connect_database(session_path.database.as_str())?;
        let update_sql = r#"
          INSERT INTO session (path, data)
          VALUES (?1, ?2);
          -- ON CONFLICT(path) DO UPDATE
          -- SET path = ?3, data = ?4;
        "#;
        conn.execute(update_sql, (&session.path, &session.data))
            .map_err(|_| RuntimeError("session plugin save session failed!".to_string()))?;
        Ok(())
    }

    fn session_list(lua: &Lua, (): ()) -> LuaResult<LuaTable> {
        let list = Self::_session_list(lua)?;
        let table = lua.create_table()?;
        for (index, data) in list.into_iter().enumerate() {
            table.set(index + 1, data)?;
        }
        Ok(table)
    }

    fn _session_list(lua: &Lua) -> LuaResult<Vec<SessionData>> {
        let session_path = SessionPath::try_new(lua)?;
        let conn = Self::_connect_database(session_path.database.as_str())?;
        let mut stmt = conn
            .prepare("SELECT * FROM session;")
            .map_err(|_| RuntimeError("session plugin sql prepare failed!".to_string()))?;
        Ok(stmt
            .query_map([], |row| {
                Ok(SessionData {
                    path: row.get(0)?,
                    data: row.get(1)?,
                })
            })
            .map_err(|_| RuntimeError("session plugin sql query failed!".to_string()))?
            .filter_map(|x| x.ok())
            .collect())
    }

    fn clean_session(lua: &Lua, (): ()) -> LuaResult<()> {
        let list = Self::_session_list(lua)?;
        let session_path = SessionPath::try_new(lua)?;
        // 数据库删除文件系统文件不存在的记录
        let fs_not_exists = list
            .iter()
            .filter_map(|session| {
                if let Ok(true) = fs::exists(session.data.as_str()) {
                    None
                } else {
                    Some(session)
                }
            })
            .collect::<Vec<_>>();
        if fs_not_exists.len() > 0 {
            let clean_sql = format!(
                r#"
              DELETE FROM session
              WHERE path IN ({});
            "#,
                fs_not_exists
                    .into_iter()
                    .map(|x| format!("'{}'", x.path.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let conn = Self::_connect_database(session_path.database.as_str())?;
            conn.execute(clean_sql.as_str(), ())
                .map_err(|_| RuntimeError("session plugin clean session failed!".to_string()))?;
        }
        // 文件系统删除数据库中没有记录的vim文件
        let fs_exists = list
            .into_iter()
            .filter_map(|session| {
                if let Ok(true) = fs::exists(session.data.as_str()) {
                    Some(session.data)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        walk_dir_vim(session_path.plugin.as_str())
            .into_iter()
            .for_each(|session_file| {
                if !fs_exists.contains(&session_file) {
                    let _ = fs::remove_file(session_file.as_str());
                }
            });
        Ok(())
    }
}

impl<'lua> Plugin<'lua> for Session<'lua> {
    type Instance = Session<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        let path = SessionPath::try_new(lua)?;
        Ok(Session {
            name: PLUGIN_NAME,
            plugin: lua.create_table()?,
            runtime: lua,
            path: path.plugin,
            database: path.database,
        })
    }

    fn init(&self) -> LuaResult<()> {
        self.init_database()?;
        self.init_function()?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn plugin(&self) -> LuaTable {
        self.plugin.clone()
    }

    fn runtime(&self) -> &'lua Lua {
        self.runtime
    }
}

pub fn walk_dir_vim(dir_path: &str) -> Vec<String> {
    if let Ok(entries) = fs::read_dir(Path::new(dir_path)) {
        entries
            .flatten()
            .filter_map(|entry| {
                let mut res = None;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "vim" {
                            res = Some(format!("{}", path.display()));
                        }
                    }
                }
                res
            })
            .collect()
    } else {
        vec![]
    }
}
