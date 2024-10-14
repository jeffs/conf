#!/usr/bin/env -S zsh -euo pipefail

readonly parent=~/git

mkdir -p $parent
cd $parent

git clone git@github.com:jeffs/rust-kart $parent/rust-kart
cargo install --path $parent/rust-kart/tmux-send

# TODO: Python package management is crap.  Replace these with Rust.
git clone git@github.com:jeffs/py-kart $parent/py-kart
for pkg in vimod yesterday; do
    echo >~/usr/bin/$pkg '#!/usr/bin/env -S zsh -euo pipefail'
    echo >>~/usr/bin/$pkg 'export PYTHONPATH="${PYTHONPATH:+$PYTHONPATH:}'$parent/py-kart'"'
    echo >>~/usr/bin/$pkg 'python -m '$pkg' "$@"'
    chmod +x ~/usr/bin/$pkg
done
