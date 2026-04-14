#!/usr/bin/env bash
# Check for migration-era TODO wording that should not re-enter code comments.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

shopt -s nullglob
TARGETS=(crates/dev-* crates/exp-* crates/library/src)
shopt -u nullglob

# Banned only when attached to TODO comments.
PATTERN='TODO:.*(subagent #[0-9]+|\bwave [0-9]+\b|re-exported from top-level `symplectic::`|modules not yet written in migration|dropped during migration)'

echo "Checking for banned TODO phrases..."
if rg -n --pcre2 -e "$PATTERN" "${TARGETS[@]}" --glob '*.rs' --glob '*.tex'; then
  echo
  echo "Found banned TODO phrase(s). Rewrite as:"
  echo "  TODO: Missing <what>; implement in <path/symbol>; acceptance: <condition>."
  exit 1
fi

echo "No banned TODO phrases found."
