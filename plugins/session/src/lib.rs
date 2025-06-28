use mlua::prelude::{LuaResult, LuaTable};
use mlua::Lua;
use plugin::Plugin;
use rusqlite::Connection;
use std::fs;

/// nvim session管理插件
pub struct Session<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
}

impl<'lua> Session<'lua> {
    fn init_anyhow(&self, data_path: String) -> anyhow::Result<()> {
        let plugin_path = format!("{}/{}", data_path, self.name);
        let database_path = format!("{plugin_path}/sqlite.db");
        fs::create_dir_all(plugin_path)?;
        let conn = Connection::open(database_path)?;
        let table_create_sql = r#"
          CREATE TABLE IF NOT EXISTS session (
            path TEXT PRIMARY KEY
          )"#;
        conn.execute(table_create_sql, ())?;
        Ok(())
    }
}

impl<'lua> Plugin<'lua> for Session<'lua> {
    type Instance = Session<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        Ok(Session {
            name: "session",
            plugin: lua.create_table()?,
            runtime: lua,
        })
    }

    fn init(&self) -> LuaResult<()> {
        let data_path = api::builtin_fn::stdpath(self.runtime, "data")?;
        if let Ok(_) = self.init_anyhow(data_path) {
            Ok(())
        } else {
            println!("session plugin init failed!");
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
