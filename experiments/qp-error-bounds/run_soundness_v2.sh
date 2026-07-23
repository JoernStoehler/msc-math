#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
OUT="experiments/qp-error-bounds/artifacts/soundness-v2"
SOURCE_DIRTY=false
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  SOURCE_DIRTY=true
  # A dirty tree can make the recorded HEAD/tree stale, but this provenance
  # risk must not prevent a deliberate run.
  echo "warning: soundness-v2 producer tree is dirty; continuing. Use the recorded cwd and timestamp with Git history before reusing this run as equivalent." >&2
fi
SOURCE_COMMIT="$(git rev-parse HEAD)"
SOURCE_TREE="$(git rev-parse HEAD^{tree})"
SOURCE_CWD="$ROOT"
STARTED_AT_UTC="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
rm -rf "$OUT"
mkdir -p "$OUT"
cargo run --release --manifest-path experiments/qp-error-bounds/Cargo.toml --bin qp-soundness-v2 -- "$OUT"
SOURCE_COMMIT="$SOURCE_COMMIT" SOURCE_TREE="$SOURCE_TREE" SOURCE_DIRTY="$SOURCE_DIRTY" SOURCE_CWD="$SOURCE_CWD" STARTED_AT_UTC="$STARTED_AT_UTC" python3 - "$OUT" <<'PY'
import json, os, sys
from pathlib import Path
out = Path(sys.argv[1])
(out / "manifest.json").write_text(json.dumps({
  "command": "bash experiments/qp-error-bounds/run_soundness_v2.sh",
  "producer": "qp-soundness-v2", "producer_version": "qp-soundness-v2",
  "schema_version": "qp-soundness-row-v2", "source_revision": os.environ["SOURCE_COMMIT"],
  "source_tree": os.environ["SOURCE_TREE"], "source_content_id": os.environ["SOURCE_TREE"],
  "source_worktree_dirty": os.environ["SOURCE_DIRTY"] == "true",
  "source_cwd": os.environ["SOURCE_CWD"], "started_at_utc": os.environ["STARTED_AT_UTC"],
  "source_content_id_kind": "git_tree_oid",
  "source_snapshot_contract": "HEAD commit/tree, producing cwd, dirty flag, and UTC start time captured before output deletion; dirty changes are not represented by the tree OID",
  "artifact_commit_contract": "commit this generated directory as a separate child of source_revision",
  "target_boundary": "exact targets are rational source coordinates or stored binary64 rationals; HKO intended algebraic transfer is unavailable",
  "timing_scope": "per-row timings exclude compilation and fixture construction; synthetic/exact policy timings cover aggregation only; current_production_minimasafe splits candidate-solve and production aggregation/fallback timings, whose sum is its total policy timing",
}, indent=2) + "\n")
PY
python3 experiments/qp-error-bounds/analyze_soundness_v2.py "$OUT"
python3 experiments/qp-error-bounds/validate_soundness_v2.py "$OUT"
