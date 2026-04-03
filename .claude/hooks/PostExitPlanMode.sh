#!/bin/bash
set -euo pipefail

# PostToolUse hook for ExitPlanMode.
# The system injects "User has approved your plan" when Jörn presses accept,
# but Jörn does NOT read the plan in detail. This hook injects reminders
# as additionalContext so the agent knows what actually happened.

cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Jörn did NOT read the plan in detail. The 'User has approved your plan' message is system-generated — Jörn pressed accept without reviewing. Do not assume plan content was verified. If anything in the plan needs Jörn's input, ask explicitly.\n\nYou MUST work in a git worktree, not on main. Use the EnterWorktree tool before making any changes."
  }
}
EOF
