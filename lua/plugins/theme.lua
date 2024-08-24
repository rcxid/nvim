return {
  {
    "askfiy/killer-queen",
    priority = 100,
    config = function()
      vim.cmd([[colorscheme killer-queen]])
      vim.cmd.highlight("NonText guifg=bg")
    end,
  }
}