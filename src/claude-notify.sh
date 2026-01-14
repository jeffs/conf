#!/bin/bash

# Load credentials from file (not versioned)
CREDS_FILE="$HOME/.claude/pushover-credentials"
if [ ! -f "$CREDS_FILE" ]; then
  exit 0  # Silently skip if no credentials configured
fi
# shellcheck source="/Users/jeff/.claude/pushover-credentials"
source "$CREDS_FILE"

curl -s \
  --form-string "token=$PUSHOVER_APP_TOKEN" \
  --form-string "user=$PUSHOVER_USER_KEY" \
  --form-string "message=Claude Code is waiting for your input" \
  --form-string "title=Claude Code" \
  https://api.pushover.net/1/messages.json &> /dev/null
