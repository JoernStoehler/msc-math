#!/usr/bin/env bash
set -euo pipefail

command -v cargo >/dev/null || {
  printf 'error: Codex universal Rust runtime is unavailable\n' >&2
  exit 1
}

cargo fetch --locked
