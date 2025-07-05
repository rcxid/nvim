use mlua::prelude::LuaResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct FileNode {
    name: String,
    path: PathBuf,
    r#type: FileType,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<FileNode>>,
}

#[derive(Serialize, Deserialize)]
enum FileType {
    File,
    LinkFile,
    Dir,
    LinkDir,
    Unknown,
}

impl FileNode {
    pub fn try_from_path(path: PathBuf) -> LuaResult<FileNode> {
        let name = path
            .file_name()
            .map(|x| x.to_string_lossy().into_owned())
            .unwrap_or("/".to_string());
        let file_type = FileType::try_from_path(path.clone())?;
        let children = if file_type.is_dir() {
            Some(Vec::new())
        } else {
            None
        };
        Ok(FileNode {
            name,
            path,
            r#type: file_type,
            children,
        })
    }

    pub fn is_dir(&self) -> bool {
        match self.r#type {
            FileType::File => false,
            FileType::LinkFile => false,
            FileType::Dir => true,
            FileType::LinkDir => true,
            FileType::Unknown => false,
        }
    }

    pub fn add_child(&mut self, node: FileNode) {
        if let Some(children) = self.children.as_mut() {
            children.push(node);
        }
    }
}

impl FileType {
    fn try_from_path(path: PathBuf) -> LuaResult<FileType> {
        let metadata = path.symlink_metadata()?;
        let file_type = if metadata.is_file() {
            if metadata.is_symlink() {
                FileType::LinkFile
            } else {
                FileType::File
            }
        } else if metadata.is_dir() {
            if metadata.is_symlink() {
                FileType::LinkDir
            } else {
                FileType::Dir
            }
        } else {
            FileType::Unknown
        };
        Ok(file_type)
    }

    fn is_dir(&self) -> bool {
        match self {
            FileType::File => false,
            FileType::LinkFile => false,
            FileType::Dir => true,
            FileType::LinkDir => true,
            FileType::Unknown => false,
        }
    }
}
