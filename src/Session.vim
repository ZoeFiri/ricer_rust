let SessionLoad = 1
let s:so_save = &so | let s:siso_save = &siso | set so=0 siso=0
let v:this_session=expand("<sfile>:p")
silent only
cd ~/proj/rusty/ricer/src
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +18 ~/PATHcustom/nv
badd +22 ~/proj/rusty/ricer/Cargo.toml
badd +28 ~/.config/ricer/profiles/neon/.tmux.conf
badd +1 /tmp/fuc2
badd +14 main.rs
badd +215 ~/.config/nvim/init.vim
badd +37 ~/notes/sm1/calc3/CylinderCoords
badd +49 profile.rs
badd +30 config.rs
badd +3 ~/vimwiki/index.wiki
badd +1 ~/vimwiki/devshit.wiki
badd +1 ~/vimwiki/ideas.wiki
badd +45 ~/vimwiki/ricer\ (rust).wiki
badd +1 ~/vimwiki/camp.wiki
badd +3 ~/vimwiki/curriculums.wiki
badd +111 ~/vimwiki/chemistry.wiki
badd +49 ~/vimwiki/game\ design.wiki
badd +3 ~/vimwiki/meeting\ notes.wiki
badd +1 ~/vimwiki/it.wiki
argglobal
%argdel
$argadd main.rs
edit config.rs
set splitbelow splitright
set nosplitbelow
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 66 - ((47 * winheight(0) + 24) / 48)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
66
normal! 0
lcd ~/proj/rusty/ricer/src
tabedit ~/proj/rusty/ricer/src/main.rs
set splitbelow splitright
set nosplitbelow
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
setlocal fdm=marker
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 68 - ((25 * winheight(0) + 24) / 48)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
68
normal! 017|
lcd ~/proj/rusty/ricer/src
tabedit ~/vimwiki/ricer\ (rust).wiki
set splitbelow splitright
set nosplitbelow
set nosplitright
wincmd t
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let s:l = 40 - ((39 * winheight(0) + 24) / 48)
if s:l < 1 | let s:l = 1 | endif
exe s:l
normal! zt
40
normal! 053|
lcd ~/proj/rusty/ricer/src
tabnext 2
if exists('s:wipebuf') && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 winminheight=1 winminwidth=1 shortmess=filnxtToOFI
let s:sx = expand("<sfile>:p:r")."x.vim"
if file_readable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &so = s:so_save | let &siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
