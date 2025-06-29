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
    /// lua runtime
    fn runtime(&self) -> &'lua Lua;
    /// 注册插件
    fn register(&self) -> LuaResult<()> {
        self.init()?;
        let globals = self.runtime().globals();
        globals.set(self.name(), self.plugin())?;
        Ok(())
    }
}

pub struct RootPlugin<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
}

impl<'lua> RootPlugin<'lua> {
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
}

impl<'lua> Plugin<'lua> for RootPlugin<'lua> {
    type Instance = RootPlugin<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        Ok(RootPlugin {
            name: PLUGINS_NAME,
            plugin: lua.create_table()?,
            runtime: lua,
        })
    }

    fn init(&self) -> LuaResult<()> {
        self.plugin.set(
            "used_memory",
            self.runtime.create_function(RootPlugin::used_memory)?,
        )?;
        self.plugin.set(
            "gc_collect",
            self.runtime.create_function(|lua, (): ()| {
                lua.gc_collect()?;
                Ok(())
            })?,
        )?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
