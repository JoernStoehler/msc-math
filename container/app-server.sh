#!/usr/bin/env bash
set -euo pipefail
source "$(dirname -- "${BASH_SOURCE[0]}")/common.sh"

require_running() {
  local id
  id="$(container_id)"
  [[ -n "${id}" ]] && container_running "${id}" ||
    die "development container is not running; on the host run: cd ${REPO_ROOT} && just dev-start"
}

start() {
  validate
  lock
  require_running
  token_check
  compose exec -T "${SERVICE}" bash -lc \
    'set -euo pipefail
     token=/workspaces/msc-math/.app-server-token
     if tmux has-session -t codex-app-server 2>/dev/null; then
       if test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0 &&
          curl -fs --max-time 1 http://127.0.0.1:4500/readyz >/dev/null 2>&1; then
         set +e
         exit 0
       fi
       tmux kill-session -t codex-app-server
     fi
     tmux new-session -d -s codex-app-server \
       "tmux set-option -p remain-on-exit on; exec codex app-server --listen ws://0.0.0.0:4500 --ws-auth capability-token --ws-token-file $token"
     for _ in {1..120}; do
       if test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0 &&
          curl -fs --max-time 1 http://127.0.0.1:4500/readyz >/dev/null 2>&1; then
         set +e
         exit 0
       fi
       sleep 0.5
     done
     tmux capture-pane -pt codex-app-server -S -80 >&2 || true
     exit 1'
}

stop() {
  local id
  id="$(container_id)"
  [[ -n "${id}" ]] && container_running "${id}" || exit 0
  docker exec "${id}" bash -lc \
    'set -euo pipefail
     tmux has-session -t codex-app-server 2>/dev/null || exit 0
     tmux send-keys -t codex-app-server C-c
     for _ in {1..50}; do
       curl -fsS --max-time 0.2 http://127.0.0.1:4500/readyz >/dev/null 2>&1 || break
       sleep 0.1
     done
     tmux kill-session -t codex-app-server 2>/dev/null || true
     ! curl -fsS --max-time 0.2 http://127.0.0.1:4500/readyz >/dev/null 2>&1'
}

status() {
  validate
  require_running
  token_check
  compose exec -T "${SERVICE}" bash -lc \
    'set -euo pipefail
     tmux has-session -t codex-app-server
     test "$(tmux list-panes -t codex-app-server -F "#{pane_dead}")" = 0
     curl -fsS --max-time 1 http://127.0.0.1:4500/readyz >/dev/null'
}

logs() {
  validate
  require_running
  compose exec -T "${SERVICE}" tmux capture-pane -pt codex-app-server -S -200
}

case "${1:-}" in
  start) start ;;
  stop) stop ;;
  status) status ;;
  logs) logs ;;
  *) die "usage: $0 {start|stop|status|logs}" ;;
esac
