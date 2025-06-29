use mlua::Lua;
use mlua::prelude::LuaResult;

pub fn stdpath(lua: &Lua, what: &str) -> LuaResult<String> {
    let cmd = format!(r#"vim.fn.stdpath("{what}")"#);
    lua.load(cmd).eval()
}

pub fn getcwd(lua: &Lua) -> LuaResult<String> {
    lua.load("vim.fn.getcwd()").eval()
}
