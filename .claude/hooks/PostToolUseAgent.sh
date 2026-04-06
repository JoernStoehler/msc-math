#!/usr/bin/env bash
# PostToolUse hook for Agent tool calls.
#
# Phase 1: Logging only — writes hook input to a log file so we can
# observe what PostToolUse receives when the Agent tool completes.
#
# Phase 2 (if logging works): Output additionalContext as a reliability
# blurb injected into the calling agent's context.
#
# Hook configuration (in .claude/settings.json):
#   "PostToolUse": [{
#     "matcher": "Agent",
#     "hooks": [{
#       "type": "command",
#       "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/post-agent-blurb.sh"
#     }]
#   }]

set -euo pipefail

LOG_FILE="/tmp/post-agent-hook.log"

# Read hook input from stdin
INPUT=$(cat)

# Log timestamp and full input
echo "=== $(date -Iseconds) ===" >> "$LOG_FILE"
echo "$INPUT" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

# Phase 2: Inject additionalContext reliability blurb
SUBAGENT_TYPE=$(echo "$INPUT" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tool_input',{}).get('subagent_type','unknown'))" 2>/dev/null || echo "unknown")
IS_ASYNC=$(echo "$INPUT" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tool_response',{}).get('isAsync',False))" 2>/dev/null || echo "False")

if [ "$IS_ASYNC" = "True" ]; then
  ASYNC_NOTICE=" outputFile is a JSONL session transcript, not agent output. The completion notification contains the actual result."
else
  ASYNC_NOTICE=""
fi

cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "SUBAGENT RELIABILITY NOTICE ($SUBAGENT_TYPE): Subagent answers can be overconfident or miss context. Cheap-to-check facts (file existence, grep results, data values) are worth verifying directly. A second subagent can cross-check if warranted.${ASYNC_NOTICE}"
  }
}
EOF
