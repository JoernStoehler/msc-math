#!/usr/bin/env bash
# Spawn a Codex CLI session inside a fresh git worktree.
#
# Codex CLI has no native worktree support (verified from docs: no /worktree
# slash command, no `codex worktree` subcommand). Parallel Codex sessions
# therefore need a shell helper that (1) creates a git worktree, (2) seeds a
# trust entry in ~/.codex/config.toml so Codex will load that worktree's
# .codex/config.toml and project skills, and (3) starts Codex there.
#
# Creates Codex worktrees under .codex/worktrees/<name>.
#
# Usage:
#   scripts/codex-worktree.sh <name> [branch]
#     <name>   required — directory under .codex/worktrees/
#     [branch] optional — branch to check out; defaults to <name>
#
# Example: scripts/codex-worktree.sh lemma-cleanup
#   → git worktree add .codex/worktrees/lemma-cleanup lemma-cleanup
#   → appends trust entry for /workspaces/msc-math/.codex/worktrees/lemma-cleanup
#   → cd there, exec codex

set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "usage: $(basename "$0") <name> [branch]" >&2
  exit 64
fi

NAME=$1
BRANCH=${2:-$NAME}

REPO_ROOT=$(git rev-parse --show-toplevel)
WORKTREE_REL=".codex/worktrees/${NAME}"
WORKTREE_ABS="${REPO_ROOT}/${WORKTREE_REL}"

if [[ ! -d "$WORKTREE_ABS" ]]; then
  (cd "$REPO_ROOT" && git worktree add "$WORKTREE_REL" "$BRANCH")
fi

CODEX_USER_CONFIG="${CODEX_HOME:-$HOME/.codex}/config.toml"
mkdir -p "$(dirname "$CODEX_USER_CONFIG")"
touch "$CODEX_USER_CONFIG"
if ! grep -qF "projects.\"${WORKTREE_ABS}\"" "$CODEX_USER_CONFIG"; then
  printf '\n[projects."%s"]\ntrust_level = "trusted"\n' "$WORKTREE_ABS" >> "$CODEX_USER_CONFIG"
fi

cd "$WORKTREE_ABS"
exec codex
