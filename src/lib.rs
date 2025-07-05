use comment::Comment;
use mlua::prelude::{Lua, LuaResult, LuaTable};
use plugin::{Plugin, RootPlugin};
use project::Project;
use session::Session;

#[mlua::lua_module]
fn nvim_lib(lua: &Lua) -> LuaResult<LuaTable> {
    let root = RootPlugin::try_new(lua)?;
    let comment = Comment::try_new(lua)?;
    let session = Session::try_new(lua)?;
    let project = Project::try_new(lua)?;
    // 插件注册
    root.register(comment)?;
    root.register(session)?;
    root.register(project)?;
    // 全局注册
    root.register_to_global()?;
    Ok(root.plugin().to_owned())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
