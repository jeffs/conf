#!/bin/bash

# Load credentials from file (not versioned)
CREDS_FILE="$HOME/.claude/pushover-credentials"
if [ ! -f "$CREDS_FILE" ]; then
  exit 0  # Silently skip if no credentials configured
fi
# shellcheck source="/Users/jeff/.claude/pushover-credentials"
source "$CREDS_FILE"

# Hook context is passed as JSON on stdin
INPUT=$(cat)
NOTIFICATION_TYPE=$(echo "$INPUT" | jq -r '.notification_type // "unknown"')
MESSAGE=$(echo "$INPUT" | jq -r '.message // "Waiting for your input"')
CWD=$(echo "$INPUT" | jq -r '.cwd // ""')
PROJECT=$(basename "$CWD" 2>/dev/null || echo "")

case "$NOTIFICATION_TYPE" in
  permission_prompt) TITLE="Claude Code: Permission Needed" ;;
  idle_prompt)       TITLE="Claude Code: Waiting" ;;
  *)                 TITLE="Claude Code" ;;
esac

# Prepend project name if available
if [ -n "$PROJECT" ]; then
  TITLE="$TITLE ($PROJECT)"
fi

curl -s \
  --form-string "token=$PUSHOVER_APP_TOKEN" \
  --form-string "user=$PUSHOVER_USER_KEY" \
  --form-string "message=$MESSAGE" \
  --form-string "title=$TITLE" \
  https://api.pushover.net/1/messages.json &> /dev/null
