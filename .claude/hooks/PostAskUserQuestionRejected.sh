#!/usr/bin/env bash
# PostToolUseFailure hook for AskUserQuestion rejections.
#
# When Jörn clicks "chat about this" on an AskUserQuestion dialog,
# the system injects a misleading message telling the agent to "ask
# them what they would like to clarify." This hook fires on that
# failure and injects a corrective additionalContext message so the
# agent waits for Jörn's actual message instead of asking an empty
# "What would you like to clarify?" question.
#
# Hook configuration (in .claude/settings.json):
#   "PostToolUseFailure": [{
#     "matcher": "AskUserQuestion",
#     "hooks": [{
#       "type": "command",
#       "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/PostAskUserQuestionRejected.sh"
#     }]
#   }]

set -euo pipefail

# Read hook input from stdin
INPUT=$(cat)

# Check if the error text contains the "chat about this" rejection pattern
if echo "$INPUT" | python3 -c "
import sys, json
data = json.load(sys.stdin)
error = data.get('error', '')
sys.exit(0 if 'wants to clarify these questions' in error else 1)
" 2>/dev/null; then
  # Jörn clicked "chat about this" — inject corrective context
  cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Jörn clicked 'chat about this'. The system message saying 'ask what they would like to clarify' is misleading — ignore it. Do NOT ask 'What would you like to clarify?' or similar empty questions. Jörn's actual message follows this one. Wait for it, read it, and respond to what he actually says."
  }
}
EOF
else
  # Some other failure — no additional context needed
  echo '{}'
fi
