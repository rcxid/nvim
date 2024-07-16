use mlua::Lua;
use mlua::prelude::LuaResult;

/// 文件类型：文件后缀
pub fn filetype(lua: &Lua) -> LuaResult<String> {
    Ok(lua.load("vim.bo.filetype").eval()?)
}
