use std::string::ToString;

use mlua::prelude::*;

const PLUGINS_NAME: &str = "plugins";
const _1K_BYTE: f64 = 1024.0;
const _1M_BYTE: f64 = 1024.0 * _1K_BYTE;
const _1G_BYTE: f64 = 1024.0 * _1M_BYTE;

pub trait Plugin<'lua> {
    /// plugin instance type
    type Instance;
    /// plugin new
    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance>;
    /// plugin init
    fn init(&self) -> LuaResult<()>;
    /// plugin name
    fn name(&self) -> &str;
    /// plugin table
    fn plugin(&self) -> LuaTable;
}

pub struct RootPlugin<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
}

impl<'lua> RootPlugin<'lua> {
    /// plugins name
    pub fn name() -> String {
        PLUGINS_NAME.to_string()
    }

    fn used_memory(lua: &Lua, (): ()) -> LuaResult<String> {
        let used_memory = lua.used_memory() as f64;
        let used_memory_format = if used_memory >= _1G_BYTE {
            format!("{:.2}GB", used_memory / _1G_BYTE)
        } else if used_memory >= _1M_BYTE {
            format!("{:.2}MB", used_memory / _1M_BYTE)
        } else if used_memory >= _1K_BYTE {
            format!("{:.2}KB", used_memory / _1K_BYTE)
        } else {
            format!("{:.0}B", used_memory)
        };
        Ok(format!(
            "rust nvim library used memory: {}",
            used_memory_format
        ))
    }

    /// 注册插件
    pub fn register_plugin<'a>(&self, plugin: impl Plugin<'a>) -> LuaResult<()> {
        self.plugin.set(plugin.name(), plugin.plugin())?;
        Ok(())
    }

    /// 注册函数
    pub fn register_function(&self, function_name: &str, function: LuaFunction) -> LuaResult<()> {
        self.plugin.set(function_name, function)?;
        Ok(())
    }
}

impl<'lua> Plugin<'lua> for RootPlugin<'lua> {
    type Instance = RootPlugin<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        let plugin = lua.create_table()?;
        let nvim_plugins = RootPlugin {
            name: PLUGINS_NAME,
            plugin,
            runtime: lua,
        };
        nvim_plugins.init()?;
        Ok(nvim_plugins)
    }

    fn init(&self) -> LuaResult<()> {
        self.register_function(
            "used_memory",
            self.runtime.create_function(RootPlugin::used_memory)?,
        )?;
        self.register_function(
            "gc_collect",
            self.runtime.create_function(|lua, (): ()| {
                lua.gc_collect()?;
                Ok(())
            })?,
        )?;
        let globals = self.runtime.globals();
        globals.set(PLUGINS_NAME, self.plugin())?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn plugin(&self) -> LuaTable {
        self.plugin.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
