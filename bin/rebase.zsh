#!/usr/bin/env -S zsh -euo pipefail

cd $(git rev-parse --show-toplevel)
git add .
git stash
git checkout main
git stash pop
git add . # Bizarrely, stash push/pop unstages modifications.
git commit
git push
git checkout -
cd -
git rebase main
git push -f
