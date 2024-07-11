use mlua::prelude::*;

/// comment lib
pub fn lib(lua: &Lua) -> LuaResult<LuaTable> {
    let comment = lua.create_table()?;
    comment.set(
        "comment_toggle_line",
        lua.create_function(comment_line_toggle)?,
    )?;
    comment.set(
        "comment_toggle_multiline",
        lua.create_function(comment_toggle_multiline)?,
    )?;
    Ok(comment)
}

/// comment one line toggle
fn comment_line_toggle(_: &Lua, (comment_string, content): (String, String)) -> LuaResult<String> {
    let content_trim_start = content.trim_start();
    let output = if content_trim_start.starts_with(comment_string.as_str()) {
        let pat_with_space = format!("{} ", comment_string);
        if content_trim_start.starts_with(pat_with_space.as_str()) {
            content.replacen(pat_with_space.as_str(), "", 1)
        } else {
            content.replacen(comment_string.as_str(), "", 1)
        }
    } else {
        let index = content.find(|c: char| c != ' ').unwrap_or(content.len());
        comment_line(comment_string.as_str(), content, index)
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

/// comment toggle multiline
fn comment_toggle_multiline<'a>(
    lua: &'a Lua,
    (comment_string, content): (String, LuaTable),
) -> LuaResult<LuaTable<'a>> {
    let output = lua.create_table()?;
    // check comment or uncomment
    let pairs: LuaTablePairs<usize, String> = content.pairs();
    let (comment_flag, list, comment_index) = pairs.fold(
        (false, Vec::new(), usize::MAX),
        |(mut comment_flag, mut list, mut comment_index), pair| {
            if let Ok((index, value)) = pair {
                if !value.trim_start().starts_with(comment_string.as_str()) && value != "" {
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
                output.set(index, value)?;
            } else {
                output.set(
                    index,
                    comment_line(comment_string.as_str(), value, comment_index),
                )?;
            }
        }
    } else {
        // uncomment multiline
        for (index, value) in list {
            output.set(index, uncomment_line(comment_string.as_str(), value))?;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn comment_line_toggle_works() {
    }
}
