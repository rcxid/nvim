local osName = os.getenv("os")
if osName == nil then
  os.execute("./update.sh")
elseif osName == "Windows_NT" then
  -- Windows System
else
  print("Other platforms!")
end

require("config.lazy")

local status_ok, nvim_lib = pcall(require, "nvim_lib")
if status_ok then
  print("nvim_lib is loaded!")
else
  print("nvim_lib not fount!")
end
