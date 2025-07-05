use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use mlua::Lua;
use mlua::prelude::{LuaResult, LuaTable};

#[derive(Debug)]
pub enum FileNode {
    File(FileData),
    LinkFile(FileData),
    Dir(DirData),
    LinkDir(DirData),
}

#[derive(Debug)]
pub struct FileData {
    name: String,
}

impl FileData {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct DirData {
    name: String,
    children: Vec<Rc<RefCell<FileNode>>>,
}

impl DirData {
    /// 创建新目录
    fn new(name: String) -> Self {
        DirData {
            name,
            children: Vec::new(),
        }
    }

    /// 添加子节点
    fn add_child(&mut self, child: Rc<RefCell<FileNode>>) {
        self.children.push(child);
    }
}

/// 文件树结构
#[derive(Debug)]
pub struct FileTree {
    root: Rc<RefCell<FileNode>>,
}

impl FileNode {
    /// 获取节点名称
    pub fn name(&self) -> &str {
        match self {
            FileNode::File(file) => &file.name,
            FileNode::LinkFile(file) => &file.name,
            FileNode::Dir(dir) => &dir.name,
            FileNode::LinkDir(dir) => &dir.name,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileNode::File(_) => true,
            FileNode::LinkFile(_) => true,
            FileNode::Dir(_) => false,
            FileNode::LinkDir(_) => false,
        }
    }

    /// 判断是否为目录
    pub fn is_dir(&self) -> bool {
        !self.is_file()
    }

    fn add_child(&mut self, child: Rc<RefCell<FileNode>>) {
        match self {
            FileNode::File(_) => {}
            FileNode::LinkFile(_) => {}
            FileNode::Dir(dir) => dir.add_child(child),
            FileNode::LinkDir(dir) => dir.add_child(child),
        }
    }

    pub fn try_new(path_buf: PathBuf) -> std::io::Result<Rc<RefCell<Self>>> {
        let metadata = path_buf.symlink_metadata()?;
        let name = path_buf
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("/".to_string());
        let file_node = if metadata.is_file() {
            let file_data = FileData::new(name);
            if metadata.is_symlink() {
                Self::LinkFile(file_data)
            } else {
                Self::File(file_data)
            }
        } else if metadata.is_dir() {
            let dir_data = DirData::new(name);
            if metadata.is_symlink() {
                Self::LinkDir(dir_data)
            } else {
                Self::Dir(dir_data)
            }
        } else {
            unreachable!("")
        };
        Ok(Rc::new(RefCell::new(file_node)))
    }
}

impl FileTree {
    /// 从路径加载文件树
    pub fn from_path(path: &str) -> std::io::Result<Self> {
        let root_node = FileNode::try_new(PathBuf::from(path))?;
        let file_tree = FileTree {
            root: root_node.clone(),
        };
        if root_node.borrow().is_dir() {
            Self::load_directory(path, file_tree.root.clone())?;
        }
        Ok(file_tree)
    }

    /// 非递归加载目录内容
    fn load_directory(root_path: &str, root_node: Rc<RefCell<FileNode>>) -> std::io::Result<()> {
        let mut buffer = fs::read_dir(root_path)?
            .into_iter()
            .filter_map(|x| x.ok())
            .map(|x| (root_node.clone(), x))
            .collect::<Vec<_>>();
        while !buffer.is_empty() {
            let (parent_node, child_entry) = buffer.remove(0);
            let child_path = child_entry.path();
            let child_node = FileNode::try_new(child_path.clone())?;
            if child_node.borrow().is_dir() {
                fs::read_dir(child_path)?
                    .filter_map(|x| x.ok())
                    .for_each(|x| buffer.push((child_node.clone(), x)));
            }
            let mut parent_node = parent_node.borrow_mut();
            parent_node.add_child(child_node);
        }
        Ok(())
    }

    pub fn from_path_lua(lua: &Lua, root_path: String) -> LuaResult<LuaTable> {
        let root_node = FileNode::try_new(PathBuf::from(root_path.as_str()))?;
        let root_table = lua.create_table()?;
        if root_node.borrow().is_dir() {
            Self::load(lua, &root_table, PathBuf::from(root_path))?;
        } else {
            root_table.set(root_node.borrow().name(), lua.create_table()?)?;
        }
        Ok(root_table)
    }

    fn load(lua: &Lua, parent_table: &LuaTable, parent_path: PathBuf) -> LuaResult<()> {
        let mut buffer = fs::read_dir(parent_path)?
            .into_iter()
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();
        while !buffer.is_empty() {
            let child_entry = buffer.remove(0);
            let child_path = child_entry.path();
            let child_node = FileNode::try_new(child_path.clone())?;
            if child_node.borrow().is_dir() {
                let child_table = lua.create_table()?;
                Self::load(lua, &child_table, child_path)?;
                parent_table.set(child_node.borrow().name(), child_table)?;
            } else {
                parent_table.set(child_node.borrow().name(), lua.create_table()?)?;
            }
        }
        Ok(())
    }
}
