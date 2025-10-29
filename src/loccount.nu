#!/usr/bin/env nu
#
# Print the delta in lines of Rust code on the current branch, relative to dev.

git checkout -q (grit trunk)
let before = (tokei -o json | from json | get Total.code)
git checkout -q -
let after = (tokei -o json | from json | get Total.code)
$after - $before
