#!/usr/bin/env bash
# Build the current random/product sys-landscape datascience prepared tables
# into the retained prepare directory.
#
# Default output:
#   experiments/sys-datascience/prepare/
#
# Use this for method waves. Do not ask each method executor to rebuild a
# private /tmp dataset.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DEFAULT_TABLES_REL="experiments/sys-datascience/prepare"
TABLES_ARG="${1:-$DEFAULT_TABLES_REL}"
if [[ "$TABLES_ARG" = /* ]]; then
  TABLES_DIR="$TABLES_ARG"
else
  TABLES_DIR="$ROOT/$TABLES_ARG"
fi
mkdir -p "$TABLES_DIR"

echo "Building random/product sys-landscape datascience tables"
echo "  repo:        $ROOT"
echo "  prepare dir: $TABLES_DIR"
echo "  mode:        random/product retained"
echo

cargo run -p exp-sys-datascience --release --bin sys-dataset -- \
  --random-only \
  --out-dir "$TABLES_DIR"

echo
echo "Wrote prepared tables: $TABLES_DIR"
echo "Check prepared tables with:"
echo "  uv run --script experiments/sys-datascience/fingerprint-dataset.py $TABLES_ARG"
