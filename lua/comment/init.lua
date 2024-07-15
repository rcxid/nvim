local config = require("comment.config")

local status_ok, nvim_lib = pcall(require, "nvim_lib")
if not status_ok then
  print("nvim_lib not fount!")
  return
end

local M = {}
local comment_lib = nvim_lib.comment

-- copy from nvim-treesitter
local function visual_selection_range()
  local _, csrow, cscol, _ = unpack(vim.fn.getpos("'<"))
  local _, cerow, cecol, _ = unpack(vim.fn.getpos("'>"))
  if csrow < cerow or (csrow == cerow and cscol <= cecol) then
    return {
      start_row = csrow - 1,
      start_col = cscol - 1,
      end_row = cerow - 1,
      end_col = cecol
    }
  else
    return {
      start_row = cerow - 1,
      start_col = cecol - 1,
      end_row = csrow - 1,
      end_col = cscol
    }
  end
end

function M.get_comment_string()
  local filetype = vim.bo.filetype
  return config.default.comment_strings[filetype]
end

function M.comment_toggle()
  local comment_string = M.get_comment_string()
  if comment_string ~= nil then
    local line = vim.api.nvim_get_current_line()
    local new_line = comment_lib.comment_toggle_line(comment_string, line)
    vim.api.nvim_set_current_line(new_line)
  end
end

function M.comment_toggle_multiline()
  local comment_string = M.get_comment_string();
  if comment_string ~= nil then
    local range = visual_selection_range()
    local start_row = range["start_row"]
    local end_row = range["end_row"] + 1
    local lines = vim.api.nvim_buf_get_lines(0, start_row, end_row, false)
    local new_lines = comment_lib.comment_toggle_multiline(comment_string, lines)
    vim.api.nvim_buf_set_lines(0, start_row, end_row, false, new_lines)
  end
end

local function set_keymap()
  -- set keymap
  local option = {
    noremap = true,
    silent = true,
  }
--   vim.api.nvim_set_keymap(
--     "n",
--     "<C-g>",
--     ":lua require('comment').comment_toggle()<CR>",
--     option
--   )
  vim.api.nvim_set_keymap(
    "v",
    "<C-g>",
    ":lua require('comment').comment_toggle_multiline()<CR><CR>",
    option
  )
end

function M.setup(config)
  -- comment setting
  if config ~= nil then
    local comment_strings = config["comment_strings"]
    if comment_strings ~= nil then
      local default_comment_strings = M.config["comment_strings"]
      for key, value in pairs(comment_strings) do
        default_comment_strings[key] = value
      end
    end
  end

  set_keymap()
end

return M
