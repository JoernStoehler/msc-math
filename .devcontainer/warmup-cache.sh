#!/usr/bin/env bash
set -euo pipefail

# Background cache warming script.
# Populates Rust and Python dependency caches to speed up first builds.
#
# This script is designed to run in the background during postCreate.
# It does NOT block container startup.
#
# Usage (from post-create.sh):
#   nohup .devcontainer/warmup-cache.sh >> ~/.cache/warmup.log 2>&1 &

if [[ ${1:-} == "--help" || ${1:-} == "-h" ]]; then
  cat <<'EOF'
Usage: .devcontainer/warmup-cache.sh

Background cache warming.
Runs cargo fetch/build and uv sync to populate caches.

Designed to run in background (nohup ... &) during container startup.
Progress logged to ~/.cache/warmup.log.
EOF
  exit 0
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

log() {
  echo "[warmup-cache][$(date -Iseconds)] $*"
}

log "Starting cache warmup..."

# Python dependencies (only if pyproject.toml exists)
if [[ -d experiments ]] && [[ -f experiments/pyproject.toml ]]; then
  log "Warming Python cache (uv sync)..."
  if (cd experiments && uv sync --locked --extra dev); then
    log "Python cache warmed."
  else
    log "WARNING: Python cache warmup failed (non-fatal)."
  fi
else
  log "Skipping Python cache warmup (no experiments/pyproject.toml)."
fi

# Rust dependencies
if [[ -f Cargo.toml ]]; then
  log "Warming Rust cache (cargo fetch + build)..."
  if cargo fetch; then
    log "Rust fetch complete."
  else
    log "WARNING: Rust fetch failed (non-fatal)."
  fi

  # Debug build to populate target cache
  if cargo build; then
    log "Rust build complete."
  else
    log "WARNING: Rust build failed (non-fatal)."
  fi
fi

log "Cache warmup complete."
