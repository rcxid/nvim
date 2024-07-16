use mlua::{Lua, Table};
use mlua::prelude::LuaResult;

/// 文件类型：文件后缀
pub fn filetype(lua: &Lua) -> LuaResult<String> {
    let filetype = lua.load("vim.bo.filetype").eval()?;
    Ok(filetype)
}

/// vim.api.nvim_buf_get_lines
/// 获取多行内容
pub fn get_lines(
    lua: &Lua,
    buffer: usize,
    start_row: usize,
    end_row: usize,
    strict_indexing: bool,
) -> LuaResult<Table> {
    let lines = lua
        .load(format!(
            r#"vim.api.nvim_buf_get_lines({}, {}, {}, {})"#,
            buffer, start_row, end_row, strict_indexing,
        ))
        .eval()?;
    Ok(lines)
}

/// vim.api.nvim_buf_set_lines
/// 设置多行内容
pub fn set_lines(
    lua: &Lua,
    buffer: usize,
    start_row: usize,
    end_row: usize,
    strict_indexing: bool,
    lines: Table,
) -> LuaResult<()> {
    let global = lua.globals();
    let lines_name = "output_lines";
    global.set(lines_name, lines)?;
    lua.load(format!(
        r#"vim.api.nvim_buf_set_lines({}, {}, {}, {}, {})"#,
        buffer, start_row, end_row, strict_indexing, lines_name,
    ))
    .exec()?;
    global.raw_remove(lines_name)?;
    Ok(())
}
