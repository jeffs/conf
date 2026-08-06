#!/bin/sh

# Omitting timestamps keeps `jj log` output comfortably narrow.
readonly no_time='template-aliases."format_timestamp(timestamp)"='

# A workspace jj does not colocate has no Git HEAD of its own, so tools that
# read the repo through git -- Helix's diff gutter, say -- see an unchanged
# workspace.  Each pass puts HEAD back on `@-`; it says nothing where there is
# nothing to do.  `:` stands in until the command has been built.
if [ -x ~/conf/bin/head-sync ]; then
  readonly sync='~/conf/bin/head-sync'
else
  readonly sync=':'
fi

# viddy displays only what its command writes to stdout, and discards the rest,
# so the shell running that command points its own stderr at stdout up front.
# Every command below inherits that stderr, and so reports its errors in the
# pane, even where its stdout has been sent somewhere else.
readonly show_errors='exec 2>&1'

# Pathtree is available at <https://github.com/jeffs/pathtree>.
# The absolute path is used here to avoid name collisions (and a PATH search).
if [ -x ~/.cargo/bin/pathtree ]; then
  viddy -n 1 "
    $show_errors
    $sync
    jj diff --summary | ~/.cargo/bin/pathtree --color=always
    jj --color=always --config '$no_time'
  "
else
  viddy -n 1 "
    $show_errors
    $sync
    jj --color=always diff --summary
    jj --color=always --config '$no_time'
  "
fi
