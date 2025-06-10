return {
  {
    "askfiy/killer-queen",
    priority = 100,
    config = function()
      vim.cmd([[colorscheme killer-queen]])
      -- 消除无文字的行左侧～
      vim.cmd.highlight("NonText guifg=bg")
    end,
  }
}