#!/usr/bin/env bash
# Local command map for the sys-landscape datascience pipeline.
#
# Usage:
#   experiments/sys-datascience/pipeline.local.sh <step>
#
# Agents can read this file and run individual steps instead of executing the
# full pipeline. Default paths are smoke-sized and write to /tmp unless the step
# explicitly says it writes retained tables.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

OUT_ROOT="${OUT_ROOT:-/tmp/sys-landscape-datascience-local}"
PRODUCE_DIR="$OUT_ROOT/produce"
TABLES_DIR="$OUT_ROOT/tables"

usage() {
  cat <<'EOF'
Usage: experiments/sys-datascience/pipeline.local.sh <step>

Steps:
  smoke                Run the integrated temp-output smoke pipeline.
  random-smoke         Build a tiny random/product prepare slice.
  random-method        Build a medium random/product prepare slice.
  random-full          Build the full random/product prepare slice.
  scan-sys-gt-1        Scan temp random/product tables.
  build-tables         Build retained random/product tables from canonical producer files.

Environment:
  OUT_ROOT             Temp root for local shard/table outputs.
                       Default: /tmp/sys-landscape-datascience-local
EOF
}

step_smoke() {
  experiments/sys-datascience/smoke-pipeline.sh
}

step_random_slice() {
  local mode="$1"
  experiments/polytope-invariant-table/build-random-only-slice.sh "$mode" "$TABLES_DIR"
}

step_scan_sys_gt_1() {
  uv run --script experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py \
    --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
    --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
    --random-only
}

step_build_tables() {
  experiments/polytope-invariant-table/build-retained-table.sh
}

case "${1:-}" in
  smoke) step_smoke ;;
  random-smoke) step_random_slice smoke ;;
  random-method) step_random_slice method ;;
  random-full) step_random_slice full ;;
  scan-sys-gt-1) step_scan_sys_gt_1 ;;
  build-tables) step_build_tables ;;
  "" | -h | --help) usage ;;
  *)
    usage >&2
    exit 2
    ;;
esac
