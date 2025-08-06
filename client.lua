local socket = nil
local connected =false
local uv = vim.loop

local config = {
  HOST = "127.0.0.1",
  PORT = 6969
}

local function connect()
  if connected then return end

  socket = uv.new_tcp()
  if not socket then
      print("Failed to create TCP socket")
      return
  end

  socket:connect(config.HOST, config.PORT, function (err)
    if err then
      print("Failed to establish connection with server...")
      connected = false
      return
    end
    connected = true
    print("connected to server @ " ..config.HOST.. ":" ..config.PORT)
  end)
end

local function send_keystroke(key)
  if not connected or not socket then
    return
  end
  socket:write(key, function(err)
    if err then
      print("Failed to send data...", err)
      connected = false
    end
  end)
end
-- local function get_buffer_content()
--   local lines = vim.api.nvim_buf_get_lines(0,0,-1,false)
--   return table.concat(lines, '\n')
-- end

local function on_key(key)
  --only send in insert mode
  local mode = vim.api.nvim_get_mode().mode
  if mode ~= "i" then return end

  print("Key pressed:", vim.inspect(key), "Mode:", vim.api.nvim_get_mode().mode)
  --TODO: FILTER KEYS 
  if key == "\27" then return end --do not send esc key

  print("Sending key to server:", vim.inspect(key))
  send_keystroke(key)
end

local function disconnect()
  if socket then
    socket:close()
    socket = nil
  end
  connected = false
  print("Disconnected...")
end

local function init()
  connect()

  local filename = vim.fn.bufname()
  local is_md = filename:match("%.md$") ~= nil
  if is_md then
    vim.on_key(on_key)
  end
  --cleanup on exit 
  -- vim.api.nvim_create_autocmd("VimLeavePre",{
  --   callback = disconnect()
  -- })
end

init()

--expose user cleanup
vim.api.nvim_create_user_command("KeystrokeDisconnect", disconnect, {})

