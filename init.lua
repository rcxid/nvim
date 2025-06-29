-- os.execute("wezterm cli set-tab-title nvim")
require("settings")
--require("update").setup()
require("config.lazy")
require("config.lsp")

local status_ok, _nvim_lib = pcall(require, "nvim_lib")
if status_ok then
  print("nvim_lib is loaded!")
else
  print("nvim_lib not fount!")
end
