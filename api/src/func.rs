use mlua::{Error, Lua, Table};
use mlua::prelude::LuaResult;

pub struct Position {
    pub buffer: usize,
    pub row: usize,
    pub col: usize,
    pub offset: usize,
}

/// vim.fn.getpos
pub fn getpos(lua: &Lua, expr: &str) -> LuaResult<Position> {
    let start: Table = lua.load(format!(r#"vim.fn.getpos("{}")"#, expr)).eval()?;
    if start.len()? == 4 {
        let buffer: usize = start.get(1)?;
        let row: usize = start.get(2)?;
        let col: usize = start.get(3)?;
        let offset: usize = start.get(4)?;
        Ok(Position {
            buffer,
            row,
            col,
            offset,
        })
    } else {
        Err(Error::runtime("calling vim.fn.getpos error!"))
    }
}
