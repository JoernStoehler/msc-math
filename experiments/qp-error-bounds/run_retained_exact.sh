#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
OUT="experiments/qp-error-bounds/artifacts/retained-exact"
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  echo "retained-exact runner requires a clean producing tree before output deletion" >&2
  exit 2
fi
SOURCE_COMMIT="$(git rev-parse HEAD)"
SOURCE_TREE="$(git rev-parse HEAD^{tree})"
rm -rf "$OUT"
mkdir -p "$OUT"
cargo run --release --manifest-path experiments/qp-error-bounds/Cargo.toml \
  --bin qp-retained-exact -- "$OUT"
SOURCE_COMMIT="$SOURCE_COMMIT" SOURCE_TREE="$SOURCE_TREE" python3 - "$OUT" <<'PY'
import json, os, sys
from pathlib import Path
out = Path(sys.argv[1])
(out / "manifest.json").write_text(json.dumps({
    "command": "bash experiments/qp-error-bounds/run_retained_exact.sh",
    "producer": "qp-retained-exact",
    "producer_version": "retained-exact-v1",
    "schema_version": "qp-retained-exact-v1",
    "source_revision": os.environ["SOURCE_COMMIT"],
    "source_tree": os.environ["SOURCE_TREE"],
    "source_content_id": os.environ["SOURCE_TREE"],
    "source_content_id_kind": "git_tree_oid",
    "source_snapshot_contract": "reachable clean source commit/tree captured before output deletion; generated artifact is not recursively hashed",
    "artifact_commit_contract": "commit this generated directory as a separate child of source_revision",
    "target_input_kind": "stored_binary64_rational; intended algebraic target unavailable",
    "window_definition": "exact [minimum, 21/20 * minimum]",
    "timing_scope": {
        "candidate_generation_ms": "route enumeration and f64 solves; excludes fixture/exact-geometry setup and compilation",
        "current_minimasafe_ms": "ordinary MinimaSafe aggregation/fallback; excludes candidate generation and compilation",
        "retained_exact_ms": "exact resolution of every retained candidate; excludes candidate generation and compilation",
        "exact_all_reference_ms": "complete supplied-stream exact enumeration, solving, and sorting; excludes fixture setup and compilation",
        "analysis_validation": "Python analysis/validation and compilation are excluded from all row timers",
    },
}, indent=2) + "\n")
PY
python3 experiments/qp-error-bounds/analyze_retained_exact.py "$OUT"
python3 experiments/qp-error-bounds/validate_retained_exact.py "$OUT"
python3 experiments/qp-error-bounds/test_retained_exact.py "$OUT"
