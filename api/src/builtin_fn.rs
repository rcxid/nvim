use mlua::prelude::LuaResult;
use mlua::Lua;

pub fn stdpath(lua: &Lua, what: &str) -> LuaResult<String> {
    let cmd = format!(r#"vim.fn.stdpath("{what}")"#);
    let path = lua.load(cmd).eval()?;
    Ok(path)
}
