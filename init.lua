require("config.lazy")

local initLib = false
local osName = os.getenv("os")
if osName == nil then
  os.execute("./update.sh")
  initLib = true
elseif osName == "Windows_NT" then
  print("Windows")
else
  print("Other platforms!")
end

if initLib then
  local status_ok, nvim_lib = pcall(require, "nvim_lib")
  if not status_ok then
    print("nvim_lib not fount!")
  else
    print("nvim_lib is loaded!")
  end
end
