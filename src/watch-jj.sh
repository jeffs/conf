#!/bin/sh

# Pathtree is available at <https://github.com/jeffs/pathtree>.
# The absolute path is used here to avoid name collisions (and a PATH search).
if [ -x ~/.cargo/bin/pathtree ]; then
  viddy -n 1 "
    2>&1 jj diff --summary | 2>&1 pathtree --color=always
    2>&1 jj --color=always
  "
else
  viddy -n 1 "
    2>&1 jj --color=always diff --summary
    2>&1 jj --color=always
  "
fi
