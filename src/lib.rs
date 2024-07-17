use mlua::prelude::*;

use comment::Comment;
use plugins::{Plugin, Plugins};

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    setup(lua)?;
    lua.create_table()
}

fn setup(lua: &Lua) -> LuaResult<()> {
    let plugins = Plugins::try_new(lua)?;
    let comment = Comment::try_new(lua)?;
    plugins.register(comment.name(), comment.plugin())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
