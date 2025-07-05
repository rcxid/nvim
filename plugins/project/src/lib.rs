use crate::v3::FileNode;
use mlua::prelude::{Lua, LuaResult, LuaTable};
use mlua::LuaSerdeExt;
use plugin::Plugin;
use std::cmp::Ordering;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

pub mod v1;
pub mod v2;
pub mod v3;

pub struct Project<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
}

impl<'lua> Project<'lua> {
    fn list_path(lua: &Lua, path: String) -> LuaResult<LuaTable> {
        let table = lua.create_table()?;
        let file_node = FileNode::try_from_path(PathBuf::from(path.as_str()))?;
        if file_node.is_dir() {
            let mut list = fs::read_dir(path)?
                .filter_map(|x| x.ok())
                .collect::<Vec<_>>();
            list.sort_by(Self::file_sorter);
            for (index, dir_entry) in list.into_iter().enumerate() {
                let node = FileNode::try_from_path(dir_entry.path())?;
                table.set(index + 1, lua.to_value(&node)?)?
            }
        } else {
            table.set(1, lua.to_value(&file_node)?)?;
        }
        Ok(table)
    }

    fn tree_path(lua: &Lua, path: String) -> LuaResult<LuaTable> {
        let root_table = lua.create_table()?;
        let root_path = PathBuf::from(path);
        let mut root_node = FileNode::try_from_path(root_path.clone())?;
        if root_node.is_dir() {
            Self::load_dir(lua, &mut root_node, root_path)?;
        }
        root_table.set("root", lua.to_value(&root_node)?)?;
        Ok(root_table)
    }

    fn file_sorter(a: &DirEntry, b: &DirEntry) -> Ordering {
        if a.path().is_dir() {
            if b.path().is_dir() {
                a.path().cmp(&b.path())
            } else {
                Ordering::Less
            }
        } else {
            if b.path().is_dir() {
                Ordering::Greater
            } else {
                a.path().cmp(&b.path())
            }
        }
    }

    fn load_dir(lua: &Lua, parent_node: &mut FileNode, parent_path: PathBuf) -> LuaResult<()> {
        let mut buffer = fs::read_dir(parent_path)?
            .into_iter()
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();
        buffer.sort_by(Self::file_sorter);
        for child_entry in buffer {
            let child_path = child_entry.path();
            let mut child_node = FileNode::try_from_path(child_path.clone())?;
            if child_node.is_dir() {
                Self::load_dir(lua, &mut child_node, child_path)?;
            }
            parent_node.add_child(child_node);
        }
        Ok(())
    }
}

impl<'lua> Plugin<'lua> for Project<'lua> {
    type Instance = Project<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        Ok(Project {
            name: "project",
            plugin: lua.create_table()?,
            runtime: lua,
        })
    }

    fn init(&self) -> LuaResult<()> {
        self.register_function("list_path", Project::list_path)?;
        self.register_function("tree_path", Project::tree_path)?;
        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn plugin(&self) -> &LuaTable {
        &(self.plugin)
    }

    fn runtime(&self) -> &'lua Lua {
        self.runtime
    }
}

impl<'lua> Project<'lua> {}

#[cfg(test)]
mod tests {}
