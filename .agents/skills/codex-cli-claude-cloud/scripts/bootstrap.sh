#!/bin/sh
# Idempotent Codex CLI bootstrap for a fresh Claude cloud session. Never logs in, never prints credentials.
say() { printf 'bootstrap: %s\n' "$*"; }
if ! command -v codex >/dev/null 2>&1; then
  say "codex missing; installing via official installer (chatgpt.com/codex/install.sh)"
  if CODEX_NON_INTERACTIVE=true curl -fsSL -m 60 https://chatgpt.com/codex/install.sh 2>/tmp/codex-install.log | CODEX_NON_INTERACTIVE=true sh >>/tmp/codex-install.log 2>&1; then
    PATH="$HOME/.local/bin:$PATH"; export PATH
  fi
  if ! command -v codex >/dev/null 2>&1; then
    say "official installer failed; falling back to npm (@openai/codex)"
    npm install -g @openai/codex >>/tmp/codex-install.log 2>&1 || { npm install -g --prefix "$HOME/.local" @openai/codex >>/tmp/codex-install.log 2>&1 && PATH="$HOME/.local/bin:$PATH"; }
  fi
  if ! command -v codex >/dev/null 2>&1; then say "install FAILED; see /tmp/codex-install.log"; tail -n 5 /tmp/codex-install.log; exit 1; fi
  say "NOTE: new shells may need PATH to include \$HOME/.local/bin (installer edits ~/.bashrc)"
fi
say "codex $(codex --version 2>&1 | head -n1) at $(command -v codex)"
H="${CODEX_HOME:-$HOME/.codex}"; mkdir -p "$H"
if [ -d /mnt/attach/outputs ] && [ -w /mnt/attach/outputs ]; then
  mkdir -p /mnt/attach/outputs/codex-sessions
  if [ -L "$H/sessions" ]; then say "sessions already linked -> $(readlink "$H/sessions")"
  elif [ -d "$H/sessions" ] && [ -n "$(ls -A "$H/sessions" 2>/dev/null)" ]; then say "sessions is a non-empty real dir; not touching it (move it manually if wanted)"
  elif [ -e "$H/sessions" ] && [ ! -d "$H/sessions" ]; then say "sessions is an unexpected filesystem object; not touching it"
  else [ ! -d "$H/sessions" ] || rmdir "$H/sessions" || exit 1; ln -s /mnt/attach/outputs/codex-sessions "$H/sessions"; say "sessions linked -> /mnt/attach/outputs/codex-sessions"; fi
else say "no writable /mnt/attach/outputs; sessions stay on the local disk"; fi
st="$(codex login status 2>&1 | head -n1)"; say "login status: $st"
case "$st" in *"Logged in"*) say "ready: codex-run.sh (see codex-cli-claude-cloud SKILL.md)" ;;
  *) say "NOT logged in. Do not start a login yet: follow the Login section of codex-cli-claude-cloud SKILL.md (the user must send a message naming the exact action first); this script never logs in." ;; esac
