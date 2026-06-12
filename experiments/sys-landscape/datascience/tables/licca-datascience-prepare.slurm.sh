#!/usr/bin/env bash
#SBATCH --job-name=ds-prepare
#SBATCH --partition=epyc
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=64
#SBATCH --mem=0
#SBATCH --time=02:00:00
#SBATCH --output=ds-prepare-%j.out
#SBATCH --error=ds-prepare-%j.out

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
cd "$REPO_ROOT"

if [[ -z "${DATASCIENCE_PRODUCE_DIR:-}" ]]; then
  echo "DATASCIENCE_PRODUCE_DIR is required" >&2
  exit 2
fi

OUTPUT_DIR="${DATASCIENCE_TABLES_DIR:-$SCRIPT_DIR/licca-runs/datascience-prepare-${SLURM_JOB_ID}}"

mkdir -p "$OUTPUT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="${SLURM_CPUS_PER_TASK}"

echo "LICCA datascience prepare"
echo "  host:        $(hostname)"
echo "  date:        $(date -Is)"
echo "  repo:        $(git rev-parse --short HEAD)"
echo "  cpus:        ${SLURM_CPUS_PER_TASK}"
echo "  produce dir: $DATASCIENCE_PRODUCE_DIR"
echo "  output dir:  $OUTPUT_DIR"
echo "  cargo target:$CARGO_TARGET_DIR"

cargo build --release -p exp-sys-landscape --bin sys-datascience-prepare

target/release/sys-datascience-prepare \
  --produce-dir "$DATASCIENCE_PRODUCE_DIR" \
  --out-dir "$OUTPUT_DIR"

python3 experiments/sys-landscape/datascience/fingerprint-dataset.py "$OUTPUT_DIR"
