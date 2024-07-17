local M = {}

M.setup = function()
  local osName = os.getenv("os")
  if osName == nil then
    os.execute("./update.sh")
  elseif osName == "Windows_NT" then
    os.execute(".\\update.cmd")
  else
    print("osName: " .. osName)
  end
end

return M
