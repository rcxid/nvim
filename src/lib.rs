use mlua::prelude::*;

use comment::Comment;
use plugins::{Plugins, Plugin};

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("setup", lua.create_function(setup)?)?;
    Ok(exports)
}

fn setup(lua: &Lua, (): ()) -> LuaResult<()> {
    let nvim_plugins = Plugins::try_new(lua)?;
    let comment = Comment::try_new(lua)?;
    nvim_plugins.register(comment.name(), comment.plugin())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
