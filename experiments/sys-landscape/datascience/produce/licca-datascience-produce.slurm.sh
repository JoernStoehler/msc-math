#!/usr/bin/env bash
#SBATCH --job-name=ds-produce
#SBATCH --partition=epyc
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=64
#SBATCH --mem=0
#SBATCH --time=04:00:00
#SBATCH --output=ds-produce-%j.out
#SBATCH --error=ds-produce-%j.out

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
cd "$REPO_ROOT"

if [[ -z "${DATASCIENCE_MODE:-}" ]]; then
  echo "DATASCIENCE_MODE is required" >&2
  exit 2
fi
if [[ -z "${DATASCIENCE_PRODUCERS:-}" ]]; then
  echo "DATASCIENCE_PRODUCERS is required" >&2
  exit 2
fi

MODE="$DATASCIENCE_MODE"
PRODUCERS="$DATASCIENCE_PRODUCERS"
OUTPUT_DIR="${DATASCIENCE_OUTPUT_DIR:-$SCRIPT_DIR/licca-runs/datascience-produce-${MODE}-${SLURM_JOB_ID}}"

case "$MODE" in
  smoke|production) ;;
  *) echo "DATASCIENCE_MODE must be smoke or production, got: $MODE" >&2; exit 2 ;;
esac

mkdir -p "$OUTPUT_DIR"

if [[ -n "${DATASCIENCE_BASE_CACHE:-}" ]]; then
  BASE_CACHE="$DATASCIENCE_BASE_CACHE"
else
  BASE_CACHE="$OUTPUT_DIR/base-cache-empty.jsonl"
  : > "$BASE_CACHE"
fi

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="${SLURM_CPUS_PER_TASK}"

echo "LICCA datascience produce"
echo "  host:        $(hostname)"
echo "  date:        $(date -Is)"
echo "  repo:        $(git rev-parse --short HEAD)"
echo "  mode:        $MODE"
echo "  producers:   $PRODUCERS"
echo "  cpus:        ${SLURM_CPUS_PER_TASK}"
echo "  output dir:  $OUTPUT_DIR"
echo "  base cache:  $BASE_CACHE"
echo "  cargo target:$CARGO_TARGET_DIR"

cargo build --release -p exp-sys-landscape --bin sys-datascience-produce

"$CARGO_TARGET_DIR/release/sys-datascience-produce" \
  --mode "$MODE" \
  --producers "$PRODUCERS" \
  --output-dir "$OUTPUT_DIR" \
  --parallelism "$SLURM_CPUS_PER_TASK" \
  --base-cache "$BASE_CACHE"

wc -l "$OUTPUT_DIR"/*.jsonl
