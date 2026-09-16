#!/bin/sh

# Omitting timestamps keeps `jj log` output comfortably narrow.
readonly no_time='template-aliases."format_timestamp(timestamp)"='

# Update Git HEAD if we're in a workspace, so tools that use Git directly (like
# Helix, for its diff gutter) see any changes to the working copy.
readonly sync='~/conf/bin/jj-sync worktree'

# viddy discards stderr by default, so merge it into stdout.
readonly show_errors='exec 2>&1'

# Pathtree is available at <https://github.com/jeffs/pathtree>.
# The absolute path is used here to avoid name collisions (and a PATH search).
if [ -x ~/.cargo/bin/pathtree ]; then
  viddy -n 1 "
    $show_errors
    $sync
    jj diff --summary | ~/.cargo/bin/pathtree --color=always
    jj --color=always --config '$no_time' $@
  "
else
  viddy -n 1 "
    $show_errors
    $sync
    jj --color=always diff --summary
    jj --color=always --config '$no_time' $@
  "
fi
