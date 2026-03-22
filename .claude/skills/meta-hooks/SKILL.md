---
name: meta-hooks
description: Conventions for Claude Code hook scripts (.claude/hooks/). Load when editing or adding hooks. Does NOT cover cross-repo sync (see meta-cross-repo-sync), skill writing (see meta-skills), or agent definitions (see meta-subagents).
---

# Hook Scripts

How to write and maintain Claude Code hook scripts.

## Related skills

- `meta-documentation` — foundational analysis and failure modes
- `meta-folder-layout` — where different kinds of project knowledge live

## Hook scripts

Hook scripts live in `.claude/hooks/` and are registered in `.claude/settings.json` under the `hooks` key.

### Current hooks

- **`session-start.sh`** — `SessionStart` hook. Runs at session start. In remote (Claude Code web) environments: installs GitHub CLI if missing, exports `GH_REPO` so `gh` commands work despite the git proxy.
- **`worktree-create.sh`** — `WorktreeCreate` hook. Replaces built-in worktree creation. Branches from local `main` instead of `origin/main` (which is frequently stale). Input: JSON with `name` field. Output: absolute path to created worktree.
- **`worktree-remove.sh`** — `WorktreeRemove` hook. Replaces built-in worktree removal with safety checks: kills stale processes left running in the worktree, warns if the branch has unmerged commits relative to local `main`.

### Hook script conventions

- Always `set -euo pipefail` at the top
- Diagnostics go to stderr; only structured output (paths, JSON) goes to stdout
- Input comes as JSON on stdin (use `jq` to parse)
- Check for required fields and exit with clear error messages
- Guard environment-specific behavior with checks (e.g., `CLAUDE_CODE_REMOTE`)
- Use `$CLAUDE_PROJECT_DIR` for the repo root, not hardcoded paths

## Cross-repo note

Hook scripts are shared across repos. See `meta-cross-repo-sync` skill for the sync workflow.
