#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-/tmp/f64-capacity-verification}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
MANIFEST="$SCRIPT_DIR/manifest.json"
mkdir -p "$OUT_DIR"

generated_source_ids="$(
  python3 - "$MANIFEST" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
print(",".join(case["source_id"] for case in manifest["cases"] if case["run"] == "generated"))
PY
)"
artifact_source_ids="$(
  python3 - "$MANIFEST" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
print(",".join(case["source_id"] for case in manifest["cases"] if case["run"] == "artifacts"))
PY
)"
edge_default_source_ids="$(
  python3 - "$MANIFEST" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
print(",".join(case["source_id"] for case in manifest["cases"] if case["run"] == "edge_default"))
PY
)"
edge_product_facet_removal_source_ids="$(
  python3 - "$MANIFEST" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
print(",".join(case["source_id"] for case in manifest["cases"] if case["run"] == "edge_product_facet_removal"))
PY
)"

cargo run -p exp-dev-quadratic-program --bin f64-capacity-scan -- \
  --input-source generated \
  --generated-samples-per-facet 1 \
  --generated-seed 99540836 \
  --audit-generated all \
  --source-id-filter "$generated_source_ids" \
  --output "$OUT_DIR/generated-scan.jsonl"

cargo run -p exp-dev-quadratic-program --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 3 \
  --source-id-filter "$artifact_source_ids" \
  --output "$OUT_DIR/artifact-scan.jsonl"

cargo run -p exp-dev-quadratic-program --bin f64-capacity-scan -- \
  --input-source edge-fixtures \
  --source-id-filter "$edge_default_source_ids" \
  --output "$OUT_DIR/edge-default-scan.jsonl"

cargo run -p exp-dev-quadratic-program --bin f64-capacity-scan -- \
  --input-source edge-fixtures \
  --source-id-filter "$edge_product_facet_removal_source_ids" \
  --near-redundant-facet-removal product \
  --near-redundant-facet-removal-delta 2e-8 \
  --audit-preprocessed all \
  --output "$OUT_DIR/edge-product-facet-removal-scan.jsonl"

python3 "$SCRIPT_DIR/compare.py" \
  --manifest "$MANIFEST" \
  --scan "$OUT_DIR/generated-scan.jsonl" \
  --scan "$OUT_DIR/artifact-scan.jsonl" \
  --scan "$OUT_DIR/edge-default-scan.jsonl" \
  --scan "$OUT_DIR/edge-product-facet-removal-scan.jsonl" \
  --out-dir "$OUT_DIR"
