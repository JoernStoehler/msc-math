#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-/tmp/f64-capacity-numerics}"
VERIFY_DIR="${2:-/tmp/f64-capacity-verification}"
mkdir -p "$OUT_DIR"

if [[ $# -lt 2 ]]; then
  experiments/verification/f64-capacity/run.sh "$VERIFY_DIR"
elif [[ ! -f "$VERIFY_DIR/generated-scan.jsonl" \
  || ! -f "$VERIFY_DIR/artifact-scan.jsonl" \
  || ! -f "$VERIFY_DIR/edge-default-scan.jsonl" \
  || ! -f "$VERIFY_DIR/edge-product-facet-removal-scan.jsonl" ]]; then
  echo "missing verification scans in $VERIFY_DIR" >&2
  exit 1
fi

python3 "$(dirname "$0")/scan_to_events.py" \
  --scan "$VERIFY_DIR/generated-scan.jsonl" \
  --scan "$VERIFY_DIR/artifact-scan.jsonl" \
  --scan "$VERIFY_DIR/edge-default-scan.jsonl" \
  --scan "$VERIFY_DIR/edge-product-facet-removal-scan.jsonl" \
  --out-dir "$OUT_DIR"

python3 experiments/numerics/scripts/summarize_observations.py "$OUT_DIR"

echo "wrote $OUT_DIR/events.jsonl and $OUT_DIR/report.md"
