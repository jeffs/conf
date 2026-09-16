#!/bin/sh

cd ~/conf
jj diff --tool jd \
  --config 'merge-tools.jd.program=/opt/homebrew/bin/jd' \
  --config 'merge-tools.jd.diff-args=["-color","$left","$right"]' \
  --config 'merge-tools.jd.diff-invocation-mode=file-by-file' \
  etc/claude/settings.json
