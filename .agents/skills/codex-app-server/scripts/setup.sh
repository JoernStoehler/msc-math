#!/bin/sh
# Explicit setup only. Does not log in, start/restart a daemon, or publish sessions.
set -eu
PATH="$HOME/.local/bin:$PATH"
export PATH
case "${1:-}" in
  install)
    if ! command -v codex >/dev/null 2>&1; then
      installer=$(mktemp)
      trap 'rm -f "$installer"' EXIT HUP INT TERM
      curl -fsSL --max-time 60 https://chatgpt.com/codex/install.sh -o "$installer"
      CODEX_NON_INTERACTIVE=true sh "$installer"
    fi
    command -v codex >/dev/null 2>&1 || {
      echo 'Installer finished but codex is not on PATH; inspect installer output.' >&2
      exit 1
    }
    codex --version
    codex app-server daemon start --help >/dev/null
    ;;
  deps)
    codex_client_venv="${CODEX_HOME:-$HOME/.codex}/shell-client-venv"
    python3 -m venv "$codex_client_venv"
    "$codex_client_venv/bin/python" -m pip install 'websocket-client==1.8.0'
    printf 'Client Python: %s/bin/python\n' "$codex_client_venv"
    ;;
  *) echo 'Usage: setup.sh install|deps' >&2; exit 2 ;;
esac
