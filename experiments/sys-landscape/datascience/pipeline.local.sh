#!/usr/bin/env bash
# Local command map for the sys-landscape datascience pipeline.
#
# Usage:
#   experiments/sys-landscape/datascience/pipeline.local.sh <step>
#
# Agents can read this file and run individual steps instead of executing the
# full pipeline. Default paths are smoke-sized and write to /tmp unless the step
# explicitly says it writes retained tables.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"

OUT_ROOT="${OUT_ROOT:-/tmp/sys-landscape-datascience-local}"
PRODUCE_DIR="$OUT_ROOT/produce"
TABLES_DIR="$OUT_ROOT/tables"

usage() {
  cat <<'EOF'
Usage: experiments/sys-landscape/datascience/pipeline.local.sh <step>

Steps:
  smoke                Run the integrated temp-output smoke pipeline.
  ascent-general       Run one local general LICCA-shaped ascent shard.
  ascent-product       Run one local product LICCA-shaped ascent shard.
  ascent-shards        Run both local LICCA-shaped ascent shards.
  merge-review         Review local LICCA-shaped ascent shards, no writes.
  merge-write          Write merged review targets from local LICCA-shaped shards.
  scan-sys-gt-1        Scan temp tables plus computed-polytope producer rows.
  build-tables         Build retained tables from canonical producer files.

Environment:
  OUT_ROOT             Temp root for local shard/table outputs.
                       Default: /tmp/sys-landscape-datascience-local
  ASCENT_BUDGET_SECS   Budget used by smoke.
  SEED_TIME_BUDGET_SECS Budget used by local ascent shard wrappers.
EOF
}

step_smoke() {
  ASCENT_BUDGET_SECS="${ASCENT_BUDGET_SECS:-1}" \
    experiments/sys-landscape/datascience/smoke-pipeline.sh
}

step_ascent_general() {
  OUT_ROOT="$OUT_ROOT" \
  OUT_DIR="$PRODUCE_DIR/licca-shards/general-cache-production-1024" \
  BASE_N_START=0 \
    experiments/sys-landscape/datascience/produce/licca-ascent-smoke-general.local.sh
}

step_ascent_product() {
  OUT_ROOT="$OUT_ROOT" \
  OUT_DIR="$PRODUCE_DIR/licca-shards/product-cache-production-1024" \
  BASE_N_START=0 \
    experiments/sys-landscape/datascience/produce/licca-ascent-smoke-product.local.sh
}

step_ascent_shards() {
  step_ascent_general
  step_ascent_product
}

step_merge_review() {
  python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py \
    --produce-dir "$PRODUCE_DIR" \
    --require-cache \
    --fresh-fixed-f
}

step_merge_write() {
  python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py \
    --produce-dir "$PRODUCE_DIR" \
    --require-cache \
    --fresh-fixed-f \
    --write
}

step_scan_sys_gt_1() {
  uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py \
    --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
    --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
    --computed-polytopes "$PRODUCE_DIR/licca-shards/general-cache-production-1024/general-shard-0-computed-polytopes.jsonl" \
    --computed-polytopes "$PRODUCE_DIR/licca-shards/product-cache-production-1024/product-shard-0-computed-polytopes.jsonl"
}

step_build_tables() {
  experiments/sys-landscape/datascience/build-dataset.sh
}

case "${1:-}" in
  smoke) step_smoke ;;
  ascent-general) step_ascent_general ;;
  ascent-product) step_ascent_product ;;
  ascent-shards) step_ascent_shards ;;
  merge-review) step_merge_review ;;
  merge-write) step_merge_write ;;
  scan-sys-gt-1) step_scan_sys_gt_1 ;;
  build-tables) step_build_tables ;;
  "" | -h | --help) usage ;;
  *)
    usage >&2
    exit 2
    ;;
esac
