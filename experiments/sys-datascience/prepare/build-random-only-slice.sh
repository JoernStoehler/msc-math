#!/usr/bin/env bash
# Build scoped random/product prepared tables for development or evidence runs.
#
# Modes:
# - smoke: tiny prepare-stage feedback; validates schema and feature plumbing.
# - method: medium method-feedback slice; useful before spending a full run.
# - full: all trusted random/product rows; thesis evidence gate, prefer LICCA.

set -euo pipefail

MODE="${1:-smoke}"
OUT_DIR="${2:-}"

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
PRODUCE_DIR="${PRODUCE_DIR:-$ROOT/experiments/sys-datascience/produce}"
if [[ -z "$OUT_DIR" ]]; then
  OUT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/sys-ds-random-only-${MODE}.XXXXXX")"
fi

case "$MODE" in
  smoke)
    LIMIT_ARGS=(--max-random-rows 8 --max-random-product-rows 10)
    ;;
  method)
    LIMIT_ARGS=(--max-random-rows 512 --max-random-product-rows 1024)
    ;;
  full)
    LIMIT_ARGS=()
    ;;
  *)
    echo "usage: $0 [smoke|method|full] [out-dir]" >&2
    exit 2
    ;;
esac

cd "$ROOT"
echo "Building random-only sys-datascience prepare slice"
echo "  mode:    $MODE"
echo "  produce: $PRODUCE_DIR"
echo "  out_dir: $OUT_DIR"
echo

cargo run -p exp-sys-landscape --release --bin sys-dataset -- \
  --random-only \
  --produce-dir "$PRODUCE_DIR" \
  "${LIMIT_ARGS[@]}" \
  --out-dir "$OUT_DIR"

echo
uv run --script experiments/sys-datascience/fingerprint-dataset.py "$OUT_DIR"
echo
echo "Prepared tables: $OUT_DIR"
