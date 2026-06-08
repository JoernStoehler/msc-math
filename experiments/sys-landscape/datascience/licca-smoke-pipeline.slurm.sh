#!/bin/bash
# LICCA smoke run for the sys-landscape datascience produce -> tables surface.
# Submit from experiments/sys-landscape/datascience with:
#   sbatch licca-smoke-pipeline.slurm.sh
#
# This writes only to a temporary scratch directory. It does not touch
# canonical producer files, production shard files, or retained table files.

#SBATCH --job-name=ds-smoke-pipeline
#SBATCH --partition=test
#SBATCH --cpus-per-task=10
#SBATCH --mem=8G
#SBATCH --time=00:20:00
#SBATCH --output=%x-%j.out

# Resource justification:
# - partition=test keeps this as a bounded submit-path check.
# - The smoke path runs tiny producer inputs and table build on temp files.
# - 10 CPUs matches the existing LICCA ascent smoke scripts.
# - 8G memory is conservative headroom for Rust startup plus table smoke.
# - 20m allows current ascent startup overhead and cache/resume checks.

set -euo pipefail

cd "$HOME/msc-math"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="${SLURM_CPUS_PER_TASK:-10}"

WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/ds-smoke-pipeline.XXXXXX")"
PRODUCE_DIR="$WORKDIR/produce"
TABLES_DIR="$WORKDIR/tables"
ASCENT_BUDGET_SECS="${ASCENT_BUDGET_SECS:-1}"

mkdir -p "$PRODUCE_DIR" "$TABLES_DIR"

echo "LICCA datascience smoke pipeline"
echo "  host:                  $(hostname)"
echo "  date:                  $(date)"
echo "  repo:                  $(git rev-parse --short HEAD)"
echo "  cargo target:          $CARGO_TARGET_DIR"
echo "  rayon threads:         $RAYON_NUM_THREADS"
echo "  ascent budget secs:    $ASCENT_BUDGET_SECS"
echo "  workdir:               $WORKDIR"
echo "  produce dir:           $PRODUCE_DIR"
echo "  tables dir:            $TABLES_DIR"
echo

cargo run -p exp-sys-landscape --bin sys-dataset-random -- \
    --max-f 5 \
    --samples-per-f 1 \
    --out "$PRODUCE_DIR/random.jsonl" \
    --cache "$PRODUCE_DIR/shared-cache.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset-random-product -- \
    --max-sides 3 \
    --samples-per-bucket 1 \
    --out "$PRODUCE_DIR/random-product.jsonl" \
    --cache "$PRODUCE_DIR/shared-cache.jsonl"

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
    --n 1 \
    --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
    --out "$PRODUCE_DIR/ascent-general-endpoints.jsonl" \
    --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent -- \
    --n 1 \
    --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
    --out "$PRODUCE_DIR/ascent-general-endpoints.jsonl" \
    --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
    --n 1 \
    --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
    --out "$PRODUCE_DIR/ascent-product-endpoints.jsonl" \
    --no-db-update

cargo run -p exp-sys-landscape --bin sys-dataset-ascent-product -- \
    --n 1 \
    --seed-time-budget-secs "$ASCENT_BUDGET_SECS" \
    --out "$PRODUCE_DIR/ascent-product-endpoints.jsonl" \
    --no-db-update

: > "$PRODUCE_DIR/continuation.jsonl"
: > "$PRODUCE_DIR/continuation-cache.jsonl"

test -s "$PRODUCE_DIR/random.jsonl"
test -s "$PRODUCE_DIR/random-product.jsonl"
test -s "$PRODUCE_DIR/shared-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-general-endpoints.jsonl"
test -s "$PRODUCE_DIR/ascent-general-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl"
test -s "$PRODUCE_DIR/ascent-product-endpoints.jsonl"
test -s "$PRODUCE_DIR/ascent-product-cache.jsonl"
test -s "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl"
test "$(wc -l < "$PRODUCE_DIR/ascent-general-endpoints.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-general-cache.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-product-endpoints.jsonl")" -eq 1
test "$(wc -l < "$PRODUCE_DIR/ascent-product-cache.jsonl")" -eq 1

cargo run -p exp-sys-landscape --bin sys-dataset -- \
    --out-dir "$TABLES_DIR" \
    --produce-dir "$PRODUCE_DIR"

test -s "$TABLES_DIR/polytope-table.jsonl"
test -s "$TABLES_DIR/polytope-provenance-table.jsonl"
test -s "$TABLES_DIR/polytope-ascent-run-table.jsonl"

python3 experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py \
    --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
    --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
    --computed-polytopes "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl" \
    --computed-polytopes "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl"

echo
echo "LICCA smoke pipeline passed"
echo "  produce rows:"
wc -l "$PRODUCE_DIR"/*.jsonl
echo "  table rows:"
wc -l "$TABLES_DIR"/*.jsonl
