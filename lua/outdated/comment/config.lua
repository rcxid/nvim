local M = {}

-- default comment config
M.default = {
  comment_strings = {
    c = "//",
    cpp = "//",
    lua = "--",
    rust = "//",
    sql = "--",
    sh = "#",
  }
}

return M
