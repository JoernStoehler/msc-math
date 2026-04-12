#!/usr/bin/env bash
# .claude/statusline-command.sh
# Minimal Claude Code status line: "<model> tokens:<N> worktree:<branch>".
# Reads JSON from stdin per Claude Code's statusLine command interface.

input=$(cat)

# Short model name: "claude-opus-4-6" -> "opus"
model=$(echo "$input" | jq -r '.model.id // .model.display_name // "?"' \
        | sed -E 's/^claude-//; s/-[0-9].*$//')

# Context tokens: sum of the current turn's input-side tokens (plain + cache
# creation + cache read) — this matches what /context reports.
tokens=$(echo "$input" | jq -r '
  (.context_window.current_usage.input_tokens // 0)
  + (.context_window.current_usage.cache_creation_input_tokens // 0)
  + (.context_window.current_usage.cache_read_input_tokens // 0)
')

cwd=$(echo "$input" | jq -r '.workspace.current_dir // .cwd // empty')
branch=$(git --no-optional-locks -C "$cwd" symbolic-ref --short HEAD 2>/dev/null \
         || git --no-optional-locks -C "$cwd" rev-parse --short HEAD 2>/dev/null \
         || echo "?")

printf "%s tokens:%d worktree:%s" "$model" "$tokens" "$branch"
