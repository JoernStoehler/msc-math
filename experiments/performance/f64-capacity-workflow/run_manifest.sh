#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-/tmp/perf-f64-capacity-manifest}"
MANIFEST="${2:-experiments/verification/f64-capacity/manifest.json}"

source_ids="$(
  python3 - "$MANIFEST" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
print(",".join(
    case["source_id"]
    for case in manifest["cases"]
    if case["run"] in {"generated", "artifacts"}
))
PY
)"

cargo run -p exp-performance --release --bin f64-capacity-e2e -- \
  --mode smoke \
  --input-cohort all \
  --generated-samples-per-facet 1 \
  --generated-seed 99540836 \
  --source-id-filter "$source_ids" \
  --method-filter product_billiard_or_hk \
  --out-dir "$OUT_DIR"

python3 - "$MANIFEST" "$OUT_DIR/phase-events.jsonl" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
expected = {
    case["source_id"]
    for case in manifest["cases"]
    if case["run"] in {"generated", "artifacts"}
}
observed = set()
with open(sys.argv[2], encoding="utf-8") as handle:
    for line in handle:
        if not line.strip():
            continue
        row = json.loads(line)
        if row.get("phase") == "f64_capacity_e2e" and row.get("status") == "ok":
            observed.add(row.get("source_id"))
missing = sorted(expected - observed)
extra = sorted(observed - expected)
if missing or extra:
    if missing:
        print("missing manifest source ids: " + ",".join(missing), file=sys.stderr)
    if extra:
        print("unexpected manifest source ids: " + ",".join(extra), file=sys.stderr)
    raise SystemExit(1)
PY

python3 experiments/performance/scripts/summarize_phase_jsonl.py "$OUT_DIR" \
  --csv "$OUT_DIR/summary.csv" > "$OUT_DIR/summary.tsv"

echo "wrote $OUT_DIR/phase-events.jsonl and $OUT_DIR/summary.tsv"
