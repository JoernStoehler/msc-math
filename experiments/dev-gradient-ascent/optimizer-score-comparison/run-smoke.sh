#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
ROOT=$(cd -- "$SCRIPT_DIR/../../.." && pwd)
if [[ $# -eq 0 ]]; then
  mkdir -p /tmp/sys-ds-research-lines/optimizer
  OUT=$(mktemp -d /tmp/sys-ds-research-lines/optimizer/first-score-comparison-XXXXXX)
else
  OUT=$1
fi
mkdir -p "$OUT"

DIAGNOSTIC="$ROOT/experiments/dev-sys-prediction/facet-scale-baseline-error/branch-diagnostic"
PANEL="$ROOT/experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl"
BIN="$ROOT/target/release/dev-gradient-ascent-local-geometry-probe"

if [[ ! -f "$DIAGNOSTIC/branch-set-diagnostic.jsonl" || ! -f "$PANEL" ]]; then
  echo "required current panel inputs are missing" >&2
  exit 2
fi

{
  printf 'cargo build --release -p exp-dev-gradient-ascent\n'
  cargo build --release -p exp-dev-gradient-ascent
} 2>&1 | tee "$OUT/build.log"

run_shard() {
  local role=$1 skip=$2 label=$3
  local shard="$OUT/$role"
  mkdir -p "$shard"
  printf '%q ' "$BIN" \
    --diagnostic-dir "$DIAGNOSTIC" \
    --polytope-table "$PANEL" \
    --out-dir "$shard" \
    --steps 1e-3,1e-4 \
    --max-fixtures-per-label 1 \
    --skip-fixtures-per-label "$skip" \
    --degeneracy-labels "$label" \
    --trace-iterations 1 \
    --direction-model candidate-window \
    --write-step-ranking-audit \
    --audit-iterations 0 \
    --audit-step-policies fixed \
    --audit-policy-proposal-limit 6
  printf '\n'
  "$BIN" \
    --diagnostic-dir "$DIAGNOSTIC" \
    --polytope-table "$PANEL" \
    --out-dir "$shard" \
    --steps 1e-3,1e-4 \
    --max-fixtures-per-label 1 \
    --skip-fixtures-per-label "$skip" \
    --degeneracy-labels "$label" \
    --trace-iterations 1 \
    --direction-model candidate-window \
    --write-step-ranking-audit \
    --audit-iterations 0 \
    --audit-step-policies fixed \
    --audit-policy-proposal-limit 6 >"$shard/producer.stdout"
}

{
  echo '# producer commands'
  run_shard mechanism-f6be 0 narrow_gap
  run_shard ordinary-random-sample 2 large_gap
  run_shard equality-f43d 1 narrow_gap
} | tee "$OUT/commands.txt"

python3 "$SCRIPT_DIR/analyze.py" \
  --out-dir "$OUT" \
  --mechanism-dir "$OUT/mechanism-f6be" \
  --ordinary-dir "$OUT/ordinary-random-sample" \
  --equality-dir "$OUT/equality-f43d"

echo "$OUT"
