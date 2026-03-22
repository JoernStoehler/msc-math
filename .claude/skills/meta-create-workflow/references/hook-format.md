# Hook Script Format

How to write and maintain Claude Code hook scripts in `.claude/hooks/`.

## Registration

Hook scripts are registered in `.claude/settings.json` under the `hooks` key. Hook filenames should match the event name (e.g., `SessionStart.sh` for the `SessionStart` event).

## Current hooks

- **`SessionStart.sh`** — runs at session start. In remote (Claude Code web) environments: installs GitHub CLI if missing, exports `GH_REPO` so `gh` commands work despite the git proxy.
- **`WorktreeCreate.sh`** — replaces built-in worktree creation. Branches from local `main` instead of `origin/main` (which is frequently stale). Input: JSON with `name` field. Output: absolute path to created worktree.
- **`WorktreeRemove.sh`** — replaces built-in worktree removal with safety checks: kills stale processes left running in the worktree, warns if the branch has unmerged commits relative to local `main`.

## Script conventions

- Always `set -euo pipefail` at the top
- Diagnostics go to stderr; only structured output (paths, JSON) goes to stdout
- Input comes as JSON on stdin (use `jq` to parse)
- Check for required fields and exit with clear error messages
- Guard environment-specific behavior with checks (e.g., `CLAUDE_CODE_REMOTE`)
- Use `$CLAUDE_PROJECT_DIR` for the repo root, not hardcoded paths

## Cross-repo note

Hook scripts are shared across repos. See `meta-cross-repo-sync` skill for the sync workflow.
