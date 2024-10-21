-- https://neovim.io/doc/user/lua-guide.html

vim.cmd('colorscheme elflord')

vim.keymap.set({'n', 'v'}, ':', ';')
vim.keymap.set({'n', 'v'}, ';', ':')
vim.keymap.set({'n', 'v'}, 'q:', 'q;')
vim.keymap.set({'n', 'v'}, 'q;', 'q:')

vim.o.undofile = true

require("config.lazy")
require("mason").setup()

-- require('lspconfig').rust_analyzer.setup {
-- 	settings = {
-- 		['rust-analyzer'] = {
-- 			diagnostics = {
-- 				enable = true;
-- 			}
-- 		}
-- 	}
-- }
