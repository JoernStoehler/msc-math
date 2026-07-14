#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
OUT="$ROOT/experiments/numerics/qp-error-bounds/artifacts/broad"
REVISION="$(git -C "$ROOT" rev-parse HEAD)"
if [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
  REVISION="$REVISION-dirty"
fi
cd "$ROOT"
SOURCE_CONTENT_ID="$(sha256sum \
  experiments/numerics/qp-error-bounds/src/main.rs \
  experiments/numerics/qp-error-bounds/analyze.py \
  experiments/numerics/qp-error-bounds/validate.py \
  experiments/numerics/qp-error-bounds/test_wide.py \
  experiments/numerics/qp-error-bounds/formula_inventory.json \
  experiments/numerics/qp-error-bounds/coverage_ledger.json \
  experiments/numerics/qp-error-bounds/README.md \
  | sha256sum | cut -d' ' -f1)"
rm -rf "$OUT"
QP_SOURCE_REVISION="$REVISION" QP_SOURCE_CONTENT_ID="$SOURCE_CONTENT_ID" cargo run -p exp-qp-error-evidence --release -- "$OUT"
cp "$ROOT/experiments/numerics/qp-error-bounds/formula_inventory.json" "$OUT/formula_inventory.json"
cp "$ROOT/experiments/numerics/qp-error-bounds/coverage_ledger.json" "$OUT/coverage_ledger.json"
python3 "$ROOT/experiments/numerics/qp-error-bounds/analyze.py" "$OUT"
python3 "$ROOT/experiments/numerics/qp-error-bounds/validate.py" "$OUT"
python3 "$ROOT/experiments/numerics/qp-error-bounds/test_wide.py" "$OUT"
