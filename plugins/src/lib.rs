use std::string::ToString;

use mlua::{Lua, Table};
use mlua::prelude::LuaResult;

const PLUGINS_NAME: &str = "plugins";

pub trait Plugin {
    /// plugin init
    fn init(&self) -> LuaResult<()>;
    /// plugin name
    fn name(&self) -> &str;
    /// plugin table
    fn plugin(&self) -> Table;
}

pub struct Plugins<'lua> {
    name: &'lua str,
    plugin: Table<'lua>,
    runtime: &'lua Lua,
}

impl<'lua> Plugins<'lua> {
    pub fn try_new(lua: &'lua Lua) -> LuaResult<Self> {
        let plugin = lua.create_table()?;
        let nvim_plugins = Plugins {
            name: PLUGINS_NAME,
            plugin,
            runtime: lua,
        };
        nvim_plugins.init()?;
        Ok(nvim_plugins)
    }

    /// plugins name
    pub fn name() -> String {
        PLUGINS_NAME.to_string()
    }

    fn used_memory(lua: &Lua, (): ()) -> LuaResult<String> {
        let used_memory = lua.used_memory();
        let used_memory_format = if used_memory >= 1024 {
            if used_memory >= 1024 * 1024 {
                format!("{:.2}MB", used_memory as f32 / 1024.0 / 1024.0)
            } else {
                format!("{:.2}KB", used_memory as f32 / 1024.0)
            }
        } else {
            format!("{}B", used_memory)
        };
        Ok(format!(
            "rust nvim library used memory: {}",
            used_memory_format
        ))
    }

    /// 注册插件
    pub fn register(&self, plugin_name: &str, plugin: Table) -> LuaResult<()> {
        self.plugin.set(plugin_name, plugin)?;
        Ok(())
    }
}

impl<'lua> Plugin for Plugins<'lua> {
    fn init(&self) -> LuaResult<()> {
        self.plugin.set(
            "used_memory",
            self.runtime.create_function(Plugins::used_memory)?,
        )?;
        let globals = self.runtime.globals();
        globals.set(PLUGINS_NAME, self.plugin())?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn plugin(&self) -> Table {
        self.plugin.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
