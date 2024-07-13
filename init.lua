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
  require("comment").setup()
end
