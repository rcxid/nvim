use mlua::prelude::*;
use nvim_oxi::api::opts::SetKeymapOpts;
use nvim_oxi::api::types::Mode;

use plugins::Plugin;

mod config;

/// 代码注释插件
pub struct Comment<'lua> {
    name: &'lua str,
    plugin: LuaTable,
    runtime: &'lua Lua,
}

impl<'lua> Plugin<'lua> for Comment<'lua> {
    type Instance = Comment<'lua>;

    fn try_new(lua: &'lua Lua) -> LuaResult<Self::Instance> {
        let plugin = lua.create_table()?;
        let comment = Comment {
            name: "comment",
            plugin,
            runtime: lua,
        };
        comment.init()?;
        Ok(comment)
    }

    fn init(&self) -> LuaResult<()> {
        let comment_line_func_name = "comment_line_toggle";
        let comment_multiline_func_name = "comment_multiline_toggle";
        self.plugin.set(
            comment_line_func_name,
            self.runtime.create_function(comment_line_toggle_export)?,
        )?;
        self.plugin.set(
            comment_multiline_func_name,
            self.runtime
                .create_function(comment_multiline_toggle_export)?,
        )?;
        let opts = SetKeymapOpts::builder().noremap(true).silent(true).build();
        let _ = nvim_oxi::api::set_keymap(
            Mode::Normal,
            "<C-g>",
            format!(r#":lua {}.{}()<CR>"#, self.name(), comment_line_func_name).as_str(),
            &opts,
        );
        let _ = nvim_oxi::api::set_keymap(
            Mode::VisualSelect,
            "<C-g>",
            format!(
                r#":lua {}.{}()<CR>"#,
                self.name(),
                comment_multiline_func_name
            )
            .as_str(),
            &opts,
        );
        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn plugin(&self) -> LuaTable {
        self.plugin.clone()
    }

    fn runtime(&self) -> &'lua Lua {
        self.runtime
    }
}

struct VisualSelection {
    pub start_row: usize,
    #[allow(unused)]
    pub start_col: usize,
    pub end_row: usize,
    #[allow(unused)]
    pub end_col: usize,
}

/// comment one line toggle call by nvim
fn comment_line_toggle_export(lua: &Lua, (): ()) -> LuaResult<()> {
    if let Ok(current_line) = nvim_oxi::api::get_current_line() {
        let filetype: String = api::buffer::filetype(lua)?;
        if let Some(comment_string) = config::comment_string(filetype) {
            let output = comment_line_toggle(comment_string.as_str(), current_line)?;
            let _ = nvim_oxi::api::set_current_line(output);
        }
    }
    Ok(())
}

/// comment one line toggle
fn comment_line_toggle(comment_string: &str, content: String) -> LuaResult<String> {
    let content_trim_start = content.trim_start();
    let output = if content_trim_start.starts_with(comment_string) {
        let pat_with_space = format!("{} ", comment_string);
        if content_trim_start.starts_with(pat_with_space.as_str()) {
            content.replacen(pat_with_space.as_str(), "", 1)
        } else {
            content.replacen(comment_string, "", 1)
        }
    } else {
        let index = content.find(|c: char| c != ' ').unwrap_or(content.len());
        comment_line(comment_string, content, index)
    };
    Ok(output)
}

/// comment one line
fn comment_line(comment_string: &str, content: String, index: usize) -> String {
    format!(
        "{}{} {}",
        &content[0..index],
        comment_string,
        &content[index..]
    )
}

/// uncomment one line
fn uncomment_line(comment_string: &str, content: String) -> String {
    let content_trim_start = content.trim_start();
    if content_trim_start.starts_with(comment_string) {
        let pat_with_space = format!("{} ", comment_string);
        if content_trim_start.starts_with(pat_with_space.as_str()) {
            content.replacen(pat_with_space.as_str(), "", 1)
        } else {
            content.replacen(comment_string, "", 1)
        }
    } else {
        content
    }
}

/// 获取
fn get_visual_selection(lua: &Lua) -> LuaResult<VisualSelection> {
    let start = api::func::getpos(lua, "'<")?;
    let end = api::func::getpos(lua, "'>")?;
    let start_row = start.row;
    let start_col = start.col;
    let end_row = end.row;
    let end_col = end.col;
    if start_row < end_row || (start_row == end_row && start_col <= end_col) {
        Ok(VisualSelection {
            start_row,
            start_col,
            end_row,
            end_col,
        })
    } else {
        Ok(VisualSelection {
            start_row: end_row,
            start_col: end_col,
            end_row: start_row,
            end_col: start_col,
        })
    }
}

fn comment_multiline_toggle_export(lua: &Lua, (): ()) -> LuaResult<()> {
    let filetype: String = api::buffer::filetype(lua)?;
    if let Some(comment_string) = config::comment_string(filetype) {
        let selection = get_visual_selection(lua)?;
        let start_row = selection.start_row - 1;
        let end_row = selection.end_row;
        let lines = api::buffer::get_lines(lua, 0, start_row, end_row, false)?;
        let output_lines = comment_multiline_toggle(comment_string.as_str(), lines);
        let cache_output_lines = lua.create_table()?;
        for (index, value) in output_lines {
            cache_output_lines.set(index, value)?;
        }
        api::buffer::set_lines(lua, 0, start_row, end_row, false, cache_output_lines)?;
    }
    Ok(())
}

/// comment toggle multiline
fn comment_multiline_toggle(comment_string: &str, lines: LuaTable) -> Vec<(usize, String)> {
    let mut output_lines = Vec::new();
    // check comment or uncomment
    let pairs: LuaTablePairs<usize, String> = lines.pairs();
    let (comment_flag, list, comment_index) = pairs.fold(
        (false, Vec::new(), usize::MAX),
        |(mut comment_flag, mut list, mut comment_index), pair| {
            if let Ok((index, value)) = pair {
                if !value.trim_start().starts_with(comment_string) && value != "" {
                    comment_flag = true;
                }
                let line_index = value.find(|c: char| c != ' ').unwrap_or(value.len());
                if line_index < comment_index {
                    comment_index = line_index;
                }
                list.push((index, value));
            }
            (comment_flag, list, comment_index)
        },
    );
    if comment_flag {
        // comment multiline
        for (index, value) in list {
            if value == "" {
                output_lines.push((index, value));
            } else {
                output_lines.push((index, comment_line(comment_string, value, comment_index)));
            }
        }
    } else {
        // uncomment multiline
        for (index, value) in list {
            output_lines.push((index, uncomment_line(comment_string, value)))
        }
    }
    output_lines
}

#[cfg(test)]
mod tests {
    #[test]
    fn comment_line_toggle_works() {}
}
