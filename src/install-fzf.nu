let prefix = '~/pkg/fzf' | path expand

git clone git@github.com:junegunn/fzf.git $prefix
cd $prefix
ln -s ($prefix | path join bin/fzf) ~/usr/bin/fzf 
