-- Grammar LSP Client for Neovim
-- Usage: Build first (cargo build --release), then :luafile init.lua

-- Get the directory of this script dynamically
local lsp_path = vim.fn.fnamemodify(debug.getinfo(1, 'S').source:sub(2), ':p:h')
local binary_path = lsp_path .. '/target/release/grammar-lsp'

-- Fallback to debug build
if vim.fn.executable(binary_path) == 0 then
  binary_path = lsp_path .. '/target/debug/grammar-lsp'
end

-- Configure diagnostic display
vim.diagnostic.config({
  virtual_text = true,
  signs = true,
  underline = true,
  update_in_insert = false,
})

-- Redirect stderr to log file for debugging
local log_file = lsp_path .. '/grammar-lsp.log'

-- Start LSP client
local client = vim.lsp.start_client({
  name = 'grammar-lsp',
  cmd = {'sh', '-c', binary_path .. ' 2>' .. log_file},
  filetypes = {'markdown'},
  root_dir = vim.fn.getcwd(),
})

if not client then
  vim.notify('Failed to start grammar-lsp', vim.log.levels.ERROR)
  return
end

-- Auto-attach to markdown buffers
vim.api.nvim_create_autocmd('FileType', {
  pattern = 'markdown',
  callback = function()
    vim.lsp.buf_attach_client(vim.api.nvim_get_current_buf(), client)
  end,
})

-- Attach to current buffer if markdown
if vim.bo.filetype == 'markdown' then
  vim.lsp.buf_attach_client(0, client)
end

print('Grammar LSP configured for Markdown files')
