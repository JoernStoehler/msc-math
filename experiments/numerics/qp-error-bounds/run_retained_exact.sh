#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
OUT="experiments/numerics/qp-error-bounds/artifacts/retained-exact"
rm -rf "$OUT"
mkdir -p "$OUT"
cargo run --release --manifest-path experiments/numerics/qp-error-bounds/Cargo.toml \
  --bin qp-retained-exact -- "$OUT"
REVISION="$(git rev-parse HEAD)"
if ! git diff --quiet -- \
  experiments/numerics/qp-error-bounds/src/retained_exact.rs \
  experiments/numerics/qp-error-bounds/analyze_retained_exact.py \
  experiments/numerics/qp-error-bounds/validate_retained_exact.py \
  experiments/numerics/qp-error-bounds/test_retained_exact.py \
  experiments/numerics/qp-error-bounds/run_retained_exact.sh; then
  REVISION="$REVISION-dirty"
fi
CONTENT_ID="$(sha256sum \
  experiments/numerics/qp-error-bounds/src/retained_exact.rs \
  experiments/numerics/qp-error-bounds/analyze_retained_exact.py \
  experiments/numerics/qp-error-bounds/validate_retained_exact.py \
  experiments/numerics/qp-error-bounds/test_retained_exact.py \
  experiments/numerics/qp-error-bounds/run_retained_exact.sh \
  | sha256sum | cut -d' ' -f1)"
REVISION="$REVISION" CONTENT_ID="$CONTENT_ID" python3 - "$OUT" <<'PY'
import json, os, sys
from pathlib import Path
out = Path(sys.argv[1])
(out / "manifest.json").write_text(json.dumps({
    "command": "bash experiments/numerics/qp-error-bounds/run_retained_exact.sh",
    "producer": "qp-retained-exact",
    "producer_version": "retained-exact-v1",
    "schema_version": "qp-retained-exact-v1",
    "source_revision": os.environ["REVISION"],
    "source_content_id": os.environ["CONTENT_ID"],
    "target_input_kind": "stored_binary64_rational; intended algebraic target unavailable",
    "window_definition": "exact [minimum, 21/20 * minimum]",
}, indent=2) + "\n")
PY
python3 experiments/numerics/qp-error-bounds/analyze_retained_exact.py "$OUT"
python3 experiments/numerics/qp-error-bounds/validate_retained_exact.py "$OUT"
python3 experiments/numerics/qp-error-bounds/test_retained_exact.py "$OUT"
