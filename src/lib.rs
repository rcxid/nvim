use mlua::prelude::*;

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let (comment_lib_name, comment) = comment::lib(lua)?;
    exports.set(comment_lib_name, comment)?;
    Ok(exports)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
