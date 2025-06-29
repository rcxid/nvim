use mlua::prelude::LuaResult;
use mlua::Lua;

pub mod buffer;
pub mod builtin_fn;
pub mod func;
pub mod util;

pub fn cmd(lua: &Lua, cmd: Option<String>) -> LuaResult<()> {
    if let Some(cmd) = cmd {
        Ok(lua.load(format!("vim.cmd([[{cmd}]])")).eval()?)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
