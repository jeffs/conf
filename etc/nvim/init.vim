" This is my vimrc.  There are many vimrcs like it, but this one is mine.

" Use space as leader key so right pinky needn't reach for backslash.
let mapleader = " "

" Save hundreds of left-pinky keystrokes per day.
inoremap <C-g> <Esc>

" Swap ; with : (which is Shift+;) to save hundreds more left-pinky keystrokes per day.
" nnoremap : ;
" nnoremap ; :
" nnoremap q: q;
" nnoremap q; q:
" vnoremap : ;
" vnoremap ; :
" vnoremap q: q;
" vnoremap q; q:

" This one saves all fingers involved in C-^ _except_ the left pinky.
nnoremap <Tab> <C-^>

" Toggle line numbering or wrapping.
nnoremap <silent> <C-n> :setlocal number!<CR>
nnoremap <silent> <M-z> :setlocal wrap!<CR>

" Edit the previous/next file.
nnoremap <silent> <Left> :previous<CR>
nnoremap <silent> <Right> :next<CR>

" Open a brace or paren delimited block.
"
" TODO:
" * Automatically close open parens around the new block.
" * Do the right thing in shell scripts:
"   -  {} for functions
"   -  do....done for loops,
"   -  then....fi for if-statements
" * Do the right thing in VimL:
"   -  if....endif
inoremap <silent> <C-j> <Esc>:s/\s*{\=\s*$//<CR>A {<CR>}<C-o>O
inoremap <silent> <C-Down> <Esc>:s/\%(\s*\)\@<=\%((\s*\)\=$//<CR>A(<CR>);<C-o>O

" Buffer management.
nnoremap <Leader>q :q<CR>
nnoremap <Leader>w :up<CR>
autocmd TermOpen * nnoremap <buffer> <nowait> q :bw!<CR>
autocmd TermOpen * nnoremap <buffer> <Leader>q :bw!<CR>

" Line wrapping
let &showbreak = nr2char(0x230a)        " prefix soft-wrapped lines with ⌊
set breakindent breakindentopt+=shift:2 " indent wrapped lines
set linebreak                           " don't wrap lines

" Folds
set foldtext=getline(v:foldstart).printf('\ (%d)',v:foldlevel)
    " format text for closed folds

" Case sensitivity
set ignorecase  " case-insensitive search
set smartcase   "       unless the pattern includes a capital letter
set infercase   "       adjusting automplete results to match pattern
set nohlsearch  " don't highlight search results

" Offline storage
set hidden                      " allow hiding of buffers with unsaved changes
set undofile                    " preserve undo history between sessions
set undodir=~/var/nvim/undo
set viewdir=~/var/nvim/view
set viminfo+=n~/var/nvim/info
set backupdir=~/var/nvim/back,.
set directory=~/var/nvim/swap,.
set spellfile=~/var/nvim/spell/spellfile.utf8.add

" Desktop integration
set mouse=a
set visualbell                  " flash instead of blinking

" NOTE: This setting breaks rectangular yank/put.
set clipboard+=unnamedplus      " map "+ register to system clipboard

" Convenience
nnoremap <C-l> :syntax sync fromstart<CR>
nnoremap Y y$

" Abbreviations
" 🤮 U+1F92E            Face Vomiting
" ☯️  U+262F, U+FE0F     Yin Yang Emoji, Variant Form
" 🖖 U+1F596            Vulcan Salute
" ⌘  U+2318             Place of Interest Sign 
" ⇧  U+21E7             Level 2 Select Key
abbrev 0vomit    🤮
abbrev 0yin_yang ☯️
abbrev 0vulcan   🖖
abbrev 0cmd      ⌘
abbrev 0shift    ⇧

" Git integration
nnoremap <silent> <Leader>gdi :bel split term://git diff %<CR>

" Tmux integration: Send a line, paragraph, or selection to the bottom pane.
nnoremap <silent> <C-j>     :.w      !tmux-send<CR><CR>j
nnoremap <silent> <Leader>e vap:w    !tmux-send<CR><CR>}
vnoremap <silent> <Leader>e :w       !tmux-send<CR><CR>

" Plugin integration
set runtimepath+=~/opt/fzf  " https://github.com/junegunn/fzf
nnoremap <silent> <Leader>o :FZF<CR>
nnoremap <Leader>n :NERDTreeToggle<CR>
nnoremap <Leader>u :UndotreeToggle<CR>
vnoremap <silent> <Leader>t :<C-u>execute "'<,'>!tabulate" v:count<CR>
let g:is_bash = 1
let g:user_emmet_mode='i'

" ctermbg=NONE should do the trick, since all I want is for iTerm transparency
" to work; but for some reason nvim uses guibg, even in the terminal.
"
" The automd, for whatever reason, doesn't always execute when I expect it to.
"
"   autocmd Colorscheme * hi Normal ctermbg=NONE guibg=NONE
"
" I'm disabling this for now anyway, because I'm not currently using iTerm.
"
"   hi Normal ctermbg=NONE guibg=NONE
"
" Even with the above, the area below the text is opaque in elflord.
colorscheme elflord

function Autowrite()
    autocmd TextChanged,TextChangedI <buffer> silent write
endfunction

" If ever Vim won't output italics in a terminal that supports them, try this:
" set t_ZH=[3m
" set t_ZR=[3m
