use anyhow::anyhow;
use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;
use plugin::Plugin;
use rusqlite::Connection;
use std::fs;

const PLUGIN_NAME: &str = "session";

/// nvim session管理插件
pub struct Session<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
}

pub struct SessionData {
    path: String,
    data: String,
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
    fn init_database(&self) -> anyhow::Result<()> {
        let _ = fs::create_dir_all("/Users/vision/code/project/nvim/1");
        // 创建插件数据目录
        if let Ok(session_path) = SessionPath::try_new(self.runtime) {
            let _ = fs::create_dir_all("/Users/vision/code/project/nvim/2");
            fs::create_dir_all(session_path.plugin)?;
            let conn = Connection::open(session_path.database)?;
            let table_create_sql = r#"
              CREATE TABLE IF NOT EXISTS session (
                -- workspace path
                path TEXT PRIMARY KEY,
                -- session data path
                data TEXT NOT NULL UNIQUE
              )"#;
            conn.execute(table_create_sql, ())?;
        }

        Ok(())
    }

    fn init_function(&self) -> LuaResult<()> {
        // 注册方法
        self.plugin.set(
            "make_session",
            self.runtime.create_function(Session::make_session)?,
        )?;
        Ok(())
    }

    fn query_session(lua: &Lua, workspace_path: &str) -> anyhow::Result<SessionData> {
        if let Ok(session_path) = SessionPath::try_new(lua) {
            let conn = Connection::open(session_path.database.as_str())?;
            let query_sql = r#"
              SELECT
                *
              FROM session
              WHERE path = ?1;
            "#;
            let session = conn.query_one(query_sql, [workspace_path], |row| {
                Ok(SessionData {
                    path: row.get(0)?,
                    data: row.get(1)?,
                })
            })?;
            Ok(session)
        } else {
            Err(anyhow!("session plugin get path failed!"))
        }
    }

    fn make_session(lua: &Lua, (): ()) -> LuaResult<()> {
        let cwd = api::builtin_fn::getcwd(lua)?;
        let (cmd, data) = if let Ok(session) = Self::query_session(lua, cwd.as_str()) {
            (Some(format!("mks! {}", session.data)), Some(session))
        } else {
            if let Ok(session_path) = SessionPath::try_new(lua) {
                let file_name = api::util::generate_random_string(8);
                let file_path = format!("{}/{}.vim", session_path.plugin, file_name);
                (
                    Some(format!("mks! {}", file_path)),
                    Some(SessionData {
                        path: cwd,
                        data: file_path,
                    }),
                )
            } else {
                (None, None)
            }
        };
        api::cmd(lua, cmd)?;
        if let Some(data) = data {
            let _ = Self::save_session(lua, data);
        }
        Ok(())
    }

    fn save_session(lua: &Lua, session: SessionData) -> anyhow::Result<()> {
        if let Ok(session_path) = SessionPath::try_new(lua) {
            let conn = Connection::open(session_path.database.as_str())?;
            let update_sql = r#"
              INSERT INTO session (path, data)
              VALUES (?1, ?2)
              ON CONFLICT(path) DO UPDATE
              SET path = ?3, data = ?4;
            "#;
            conn.execute(
                update_sql,
                (&session.path, &session.data, &session.path, &session.data),
            )?;
            Ok(())
        } else {
            Err(anyhow!("session plugin save session failed!"))
        }
    }
}

impl<'lua> Plugin<'lua> for Session<'lua> {
    type Instance = Session<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        Ok(Session {
            name: PLUGIN_NAME,
            plugin: lua.create_table()?,
            runtime: lua,
        })
    }

    fn init(&self) -> LuaResult<()> {
        if let Ok(_) = self.init_database() {
            self.init_function()?;
            Ok(())
        } else {
            Err(mlua::Error::RuntimeError(
                "session plugin init failed!".to_string(),
            ))
        }
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
