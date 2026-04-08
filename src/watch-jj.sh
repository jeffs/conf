#!/bin/sh

viddy -n 1 "
  2>&1 jj diff --summary | 2>&1 pathtree --color=always
  2>&1 jj --color=always
"

# Or, without pathtree (which is from rust-kart):
#
# viddy -n 1 "
#   2>&1 jj --color=always diff --summary
#   2>&1 jj --color=always
# "
