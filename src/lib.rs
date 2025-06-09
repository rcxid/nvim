use mlua::prelude::*;

use comment::Comment;
use plugins::{Plugin, RootPlugin};

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    setup(lua)?;
    lua.create_table()
}

fn setup(lua: &Lua) -> LuaResult<()> {
    let root = RootPlugin::try_new(lua)?;
    let comment = Comment::try_new(lua)?;
    // 把代码注释插件注册到根插件上
    root.register_plugin(comment)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
