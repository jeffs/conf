#!/bin/bash

MARKER_FILE="/tmp/claude-notify-pending"
DELAY_SECONDS=20

# Generate unique ID for this stop event
ID=$(uuidgen)
echo "$ID" > "$MARKER_FILE"

# Background job: wait, then notify if still pending
(
  sleep "$DELAY_SECONDS"
  if [ -f "$MARKER_FILE" ] && [ "$(cat "$MARKER_FILE" 2>/dev/null)" = "$ID" ]; then
    ~/.claude/claude-notify.sh
    rm -f "$MARKER_FILE"
  fi
) &

exit 0
