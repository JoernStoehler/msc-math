#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
OUT="experiments/numerics/qp-error-bounds/artifacts/soundness-v2"
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  echo "soundness-v2 runner requires a clean producing tree before output deletion" >&2
  exit 2
fi
SOURCE_COMMIT="$(git rev-parse HEAD)"
SOURCE_TREE="$(git rev-parse HEAD^{tree})"
rm -rf "$OUT"
mkdir -p "$OUT"
cargo run --release --manifest-path experiments/numerics/qp-error-bounds/Cargo.toml --bin qp-soundness-v2 -- "$OUT"
SOURCE_COMMIT="$SOURCE_COMMIT" SOURCE_TREE="$SOURCE_TREE" python3 - "$OUT" <<'PY'
import json, os, sys
from pathlib import Path
out = Path(sys.argv[1])
(out / "manifest.json").write_text(json.dumps({
  "command": "bash experiments/numerics/qp-error-bounds/run_soundness_v2.sh",
  "producer": "qp-soundness-v2", "producer_version": "qp-soundness-v2",
  "schema_version": "qp-soundness-row-v2", "source_revision": os.environ["SOURCE_COMMIT"],
  "source_tree": os.environ["SOURCE_TREE"], "source_content_id": os.environ["SOURCE_TREE"],
  "source_content_id_kind": "git_tree_oid",
  "source_snapshot_contract": "reachable clean source commit/tree captured before output deletion; generated artifact is not recursively hashed",
  "artifact_commit_contract": "commit this generated directory as a separate child of source_revision",
  "target_boundary": "exact targets are rational source coordinates or stored binary64 rationals; HKO intended algebraic transfer is unavailable",
  "timing_scope": "per-row timings exclude compilation and fixture construction; policy timings cover aggregation only",
}, indent=2) + "\n")
PY
python3 experiments/numerics/qp-error-bounds/analyze_soundness_v2.py "$OUT"
python3 experiments/numerics/qp-error-bounds/validate_soundness_v2.py "$OUT"
