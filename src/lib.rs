use mlua::prelude::*;

use comment::Comment;
use plugin::{Plugin, RootPlugin};

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    setup(lua)?;
    lua.create_table()
}

fn setup(lua: &Lua) -> LuaResult<()> {
    let root = RootPlugin::try_new(lua)?;
    let comment = Comment::try_new(lua)?;
    // 插件注册
    root.register()?;
    comment.register()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
