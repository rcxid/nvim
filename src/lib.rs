use mlua::prelude::*;

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let (comment_lib_name, comment) = comment::lib(lua)?;
    exports.set(comment_lib_name, comment)?;
    exports.set("setup", lua.create_function(setup)?)?;
    Ok(exports)
}

fn setup(lua: &Lua, (): ()) -> LuaResult<()> {
    let globals = lua.globals();
    let plugins = lua.create_table()?;
    globals.set("plugins", plugins)?;
    globals.set("used_memory", lua.create_function(used_memory)?)?;
    Ok(())
}

fn used_memory(lua: &Lua, (): ()) -> LuaResult<String> {
    let used_memory = lua.used_memory();
    let used_memory_format = if used_memory >= 1024 {
        if used_memory >= 1024 * 1024 {
            format!("{:.2}MB", used_memory as f32 / 1024.0 / 1024.0)
        } else {
            format!("{:.2}KB", used_memory as f32 / 1024.0)
        }
    } else {
        format!("{}B", used_memory)
    };
    Ok(format!(
        "rust nvim library used memory: {}",
        used_memory_format
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
