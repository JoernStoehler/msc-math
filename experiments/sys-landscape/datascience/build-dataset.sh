#!/usr/bin/env bash
# Build the current sys-landscape datascience tables into the active dataset.
#
# Default output:
#   experiments/sys-landscape/datascience/dataset/
#
# Use this for method waves. Do not ask each method executor to rebuild a
# private /tmp dataset.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DEFAULT_DATASET_REL="experiments/sys-landscape/datascience/dataset"
DATASET_ARG="${1:-$DEFAULT_DATASET_REL}"
if [[ "$DATASET_ARG" = /* ]]; then
  DATASET_DIR="$DATASET_ARG"
else
  DATASET_DIR="$ROOT/$DATASET_ARG"
fi
FINGERPRINT="$DATASET_DIR/FINGERPRINT.md"

mkdir -p "$DATASET_DIR"

echo "Building sys-landscape datascience tables"
echo "  repo:        $ROOT"
echo "  dataset dir: $DATASET_DIR"
echo

cargo run -p exp-sys-landscape --release --bin sys-dataset -- --out-dir "$DATASET_DIR"

(
  cd "$ROOT"
  uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py \
    "$DATASET_ARG" > "$FINGERPRINT"
)

echo
echo "Wrote dataset:     $DATASET_DIR"
echo "Wrote fingerprint: $FINGERPRINT"
