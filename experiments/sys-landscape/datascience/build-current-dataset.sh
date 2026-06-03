#!/usr/bin/env bash
# Build the current sys-landscape datascience tables into an owned batch path.
#
# Default output:
#   experiments/sys-landscape/datascience/batches/2026-06-03-current/dataset/
#
# Use this for method waves. Do not ask each method executor to rebuild a
# private /tmp dataset.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
DEFAULT_BATCH_REL="experiments/sys-landscape/datascience/batches/2026-06-03-current"
BATCH_ARG="${1:-$DEFAULT_BATCH_REL}"
if [[ "$BATCH_ARG" = /* ]]; then
  BATCH_DIR="$BATCH_ARG"
  DATASET_ARG="$BATCH_ARG/dataset"
else
  BATCH_DIR="$ROOT/$BATCH_ARG"
  DATASET_ARG="$BATCH_ARG/dataset"
fi
DATASET_DIR="$BATCH_DIR/dataset"
FINGERPRINT="$BATCH_DIR/FINGERPRINT.md"

mkdir -p "$DATASET_DIR"

echo "Building sys-landscape datascience tables"
echo "  repo:        $ROOT"
echo "  batch dir:   $BATCH_DIR"
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
