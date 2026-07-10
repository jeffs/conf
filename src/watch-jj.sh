#!/bin/sh

# Omitting timestamps keeps `jj log` output comfortably narrow.
readonly no_time='template-aliases."format_timestamp(timestamp)"='

# Pathtree is available at <https://github.com/jeffs/pathtree>.
# The absolute path is used here to avoid name collisions (and a PATH search).
if [ -x ~/.cargo/bin/pathtree ]; then
  viddy -n 1 "
    2>&1 jj diff --summary | 2>&1 ~/.cargo/bin/pathtree --color=always
    2>&1 jj --color=always --config '$no_time'
  "
else
  viddy -n 1 "
    2>&1 jj --color=always diff --summary
    2>&1 jj --color=always --config '$no_time'
  "
fi
