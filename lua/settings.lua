local option = vim.o

option.number         = true         -- 开启行号
option.relativenumber = true         -- 开启相对行号
option.expandtab      = true         -- 使用空格填充tab
option.tabstop        = 4            -- tab长度
option.shiftwidth     = 4            -- tab缩进
option.softtabstop    = 4
option.splitbelow     = true
option.splitright     = true
option.encoding       = 'utf-8'      -- 字符编码
option.fileencoding   = 'utf-8'      -- 写到文件的字符编码
option.autoindent     = true         -- 开启自动缩进
option.cursorline     = true         -- 高亮所在行
option.colorcolumn    = '121'        -- 开启右侧参考线
option.scrolloff      = 8            -- jk移动时光标下上方保留8行
-- smartindent     = true;
-- hidden          = true;
-- pumheight       = 10;
-- sidescrolloff   = 8;

vim.g.mapleader = " "
