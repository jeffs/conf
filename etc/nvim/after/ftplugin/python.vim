if exists('b:my_python_loaded')
        finish
endif

let b:my_python_loaded = 1

if expand('%') == '/tmp/fmt.py'
	setlocal nohidden
	nunmap q;
	nunmap q:
	nnoremap <buffer> q :q<CR>
endif


setlocal textwidth=88
setlocal number

inoremap <buffer> <silent> <Esc> <Esc>:up<CR>
nnoremap <buffer> <silent> <Esc> <Esc>:up<CR>
inoremap <buffer> <C-g> <Esc>
inoremap <buffer> <C-g> <Esc>

nnoremap <buffer> <silent> <Leader>c :up\|!mypy %<CR>
nnoremap <buffer> <silent> <Leader>i :up\|call MyPythonRepl()<CR><CR>
nnoremap <buffer> <silent> <Leader>f :call MyPythonFormat()<CR>
"nnoremap <buffer> <silent> <Leader>r :up\|!python3 %<CR>
"nnoremap <buffer> <silent> <Leader>r :up\|bel split \| te python3 -m %:h:t:r<CR>
nnoremap <buffer> <silent> <Leader>r :up\|bel split \| te python3 %<CR>
nnoremap <buffer> <silent> <Leader>t :up\|bel split \| te npx pyright %<CR>

" Open a block.
inoremap <silent> <buffer> <C-j> <Esc>A:<CR>

function MyPythonRepl()
    " Activate virtual environment, if any.
    execute '!tmux split-pane ~/pkg/py-kart/pyrep/main.py'
    execute '!tmux last-pane'
endfunction


if exists('*MyPythonFormat')
        finish
endif

function MyPythonFormat()
        update
	silent !python3 -m black --quiet % >& /tmp/fmt.py
	if -1 == match(readfile('/tmp/fmt.py'), '\S')
		edit
	else
		below split /tmp/fmt.py
	endif
endfunction
