#!/usr/bin/env -S zsh -euo pipefail
# 
# Watch for new windows appearing. This is meant to help identify mysterious
# flashing windows.
#
# This is works by listing open applications, saving the list to a file,
# then doing it again a fraction of a second later and diffing to see which
# applications are new. I'm not sure this _actually_ works, because it would
# have to catch the app in the act; hence the short sleep between `lsappinfo
# list` calls.

readonly dir='/tmp/watch-windows'

cleanup() {
  rm -rf "$dir"
  exit 0
}

trap cleanup INT TERM

rm -rf "$dir"
mkdir "$dir"

echo "Watching for new windows... (Ctrl+C to stop)"

while true; do
  lsappinfo list 2>/dev/null | grep -oE '"[^"]+"' | sort -u > "$dir/new"
  if [ -f "$dir/old" ]; then
    diff "$dir/old" "$dir/new" | grep '^>' | while read line; do
      echo "$(date '+%H:%M:%S'): $line"
    done
  fi
  cp "$dir/new" "$dir/old"
  sleep 0.2
done
