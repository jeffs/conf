#!/bin/bash

# Delay before sending notification (seconds)
DELAY=30
MARKER_FILE="/tmp/claude-notify-marker"

# Load credentials from file (not versioned)
CREDS_FILE="$HOME/.claude/pushover-credentials"
if [ ! -f "$CREDS_FILE" ]; then
  exit 0  # Silently skip if no credentials configured
fi
source "$CREDS_FILE"

# Generate unique ID for this stop event
MY_ID=$$-$(date +%s%N)

# Write our ID to the marker file
echo "$MY_ID" > "$MARKER_FILE"

# Wait in background, then check if we should notify
(
  sleep "$DELAY"

  # Only notify if our ID is still current (no newer stop event)
  if [ -f "$MARKER_FILE" ] && [ "$(cat "$MARKER_FILE" 2>/dev/null)" = "$MY_ID" ]; then
    curl -s \
      --form-string "token=$PUSHOVER_APP_TOKEN" \
      --form-string "user=$PUSHOVER_USER_KEY" \
      --form-string "message=Claude Code is waiting for your input" \
      --form-string "title=Claude Code" \
      https://api.pushover.net/1/messages.json &> /dev/null
    rm -f "$MARKER_FILE"
  fi
) &
