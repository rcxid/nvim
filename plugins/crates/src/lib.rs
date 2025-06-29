use mlua::prelude::*;

use plugin::Plugin;

pub struct CratesPlugin<'lua> {
    pub name: &'lua str,
    pub plugin: LuaTable,
    pub runtime: &'lua Lua,
}

impl<'lua> Plugin<'lua> for CratesPlugin<'lua> {
    type Instance = CratesPlugin<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        Ok(CratesPlugin {
            name: "crates",
            plugin: lua.create_table()?,
            runtime: lua,
        })
    }

    fn init(&self) -> LuaResult<()> {
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
