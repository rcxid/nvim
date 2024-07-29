use mlua::prelude::*;

use plugins::Plugin;

pub struct CratesPlugin<'lua> {
    pub name: &'lua str,
    pub plugin: LuaTable<'lua>,
    pub runtime: &'lua Lua,
}

impl<'lua> Plugin<'lua> for CratesPlugin<'lua> {
    type Instance = CratesPlugin<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        let plugin = lua.create_table()?;
        let crates_plugin = CratesPlugin {
            name: "crates",
            plugin,
            runtime: lua,
        };
        // crates_plugin.init()?;
        Ok(crates_plugin)
    }

    fn init(&self) -> LuaResult<()> {
        todo!()
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
