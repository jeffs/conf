#!/usr/bin/env nu
#
# Print the delta in lines of Rust code on the current branch, relative to dev.

git checkout -q dev
let before = (tokei -o json | from json | get Rust.code | into int)
git checkout -q -
let after = (tokei -o json | from json | get Rust.code | into int)
$after - $before
