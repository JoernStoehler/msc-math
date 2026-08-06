#!/usr/bin/env bash
set -euo pipefail

readonly codex_gui_root="/workspaces/codex-gui"

if [[ ! -f "${codex_gui_root}/package.json" ]]; then
  echo "[codex-gui] Skipping startup: ${codex_gui_root} is not cloned." >&2
  exit 0
fi

if [[ ! -d "${codex_gui_root}/node_modules" ]]; then
  echo "[codex-gui] Skipping startup: run npm ci in ${codex_gui_root} first." >&2
  exit 0
fi

echo "[codex-gui] Starting the companion and Codex app-server..."
cd "$codex_gui_root"
npm run dev:up
