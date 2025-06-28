use mlua::prelude::*;

use comment::Comment;
use plugin::{Plugin, RootPlugin};
use session::Session;

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    let root = RootPlugin::try_new(lua)?;
    let comment = Comment::try_new(lua)?;
    let session = Session::try_new(lua)?;
    // 插件注册
    root.register()?;
    comment.register()?;
    session.register()?;
    lua.create_table()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
