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

cat <<EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "SUBAGENT RELIABILITY NOTICE ($SUBAGENT_TYPE): This result is from a subagent. Subagent answers can be overconfident, miss context, or fabricate details (especially documentation claims). Before presenting subagent findings to Jörn or building on them: (1) check file:line sources directly, (2) verify factual claims against primary sources, (3) for critical decisions, launch a second subagent to cross-check."
  }
}
EOF
