use mlua::MaybeSend;
use mlua::prelude::*;

pub const ROOT_PLUGINS_NAME: &str = "plugins";
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
    fn plugin(&self) -> &LuaTable;
    /// lua runtime
    fn runtime(&self) -> &'lua Lua;
    /// 注册方法
    fn register_function<F, A, R>(&self, name: &str, func: F) -> LuaResult<()>
    where
        F: Fn(&Lua, A) -> LuaResult<R> + MaybeSend + 'static,
        A: FromLuaMulti,
        R: IntoLuaMulti,
    {
        self.plugin()
            .set(name, self.runtime().create_function(func)?)
    }
    /// 注册异步方法
    fn register_async_function<F, A, FR, R>(&self, name: &str, func: F) -> LuaResult<()>
    where
        F: Fn(Lua, A) -> FR + MaybeSend + 'static,
        A: FromLuaMulti,
        FR: Future<Output = LuaResult<R>> + MaybeSend + 'static,
        R: IntoLuaMulti,
    {
        self.plugin()
            .set(name, self.runtime().create_async_function(func)?)
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

    fn gc_collect(lua: &Lua, (): ()) -> LuaResult<()> {
        lua.gc_collect()
    }

    /// 注册插件为全局插件
    pub fn register_to_global(&self) -> LuaResult<()> {
        self.init()?;
        let globals = self.runtime().globals();
        globals.set(self.name(), self.plugin())?;
        Ok(())
    }

    /// 注册插件
    pub fn register<P>(&self, child_plugin: P) -> LuaResult<()>
    where
        P: Plugin<'lua>,
    {
        child_plugin.init()?;
        self.plugin()
            .set(child_plugin.name(), child_plugin.plugin())?;
        Ok(())
    }
}

impl<'lua> Plugin<'lua> for RootPlugin<'lua> {
    type Instance = RootPlugin<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        Ok(RootPlugin {
            name: ROOT_PLUGINS_NAME,
            plugin: lua.create_table()?,
            runtime: lua,
        })
    }

    fn init(&self) -> LuaResult<()> {
        self.register_function("used_memory", RootPlugin::used_memory)?;
        self.register_function("gc_collect", RootPlugin::gc_collect)?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn plugin(&self) -> &LuaTable {
        &(self.plugin)
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
