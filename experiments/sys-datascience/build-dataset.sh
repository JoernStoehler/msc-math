#!/usr/bin/env bash
# Build the current sys-landscape datascience tables into the retained table
# directory.
#
# Default output:
#   experiments/sys-datascience/tables/
#
# Use this for method waves. Do not ask each method executor to rebuild a
# private /tmp dataset.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DEFAULT_TABLES_REL="experiments/sys-datascience/tables"
TABLES_ARG="${1:-$DEFAULT_TABLES_REL}"
if [[ "$TABLES_ARG" = /* ]]; then
  TABLES_DIR="$TABLES_ARG"
else
  TABLES_DIR="$ROOT/$TABLES_ARG"
fi
mkdir -p "$TABLES_DIR"

echo "Building sys-landscape datascience tables"
echo "  repo:        $ROOT"
echo "  tables dir: $TABLES_DIR"
echo

cargo run -p exp-sys-landscape --release --bin sys-dataset -- --out-dir "$TABLES_DIR"

echo
echo "Wrote tables:      $TABLES_DIR"
echo "Check tables with:"
echo "  uv run --script experiments/sys-datascience/fingerprint-dataset.py $TABLES_ARG"
