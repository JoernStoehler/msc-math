#!/usr/bin/env bash
# Status: dormant run-local prepare infrastructure. This file is not an active
# submission handoff. Use only for a selected C3 producer run or explicit
# reproduction task after job-specific LICCA review; see ../LICCA.md.
#SBATCH --job-name=ds-prepare
#SBATCH --partition=epyc
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=64
#SBATCH --mem=16G
#SBATCH --time=02:00:00
#SBATCH --output=ds-prepare-%j.out
#SBATCH --error=ds-prepare-%j.out

set -euo pipefail

# The SBATCH values are dormant historical defaults, not a current resource
# recommendation. A new handoff must review or override them for its job size.

if [[ -n "${SLURM_SUBMIT_DIR:-}" ]]; then
  SCRIPT_DIR="$(cd -- "$SLURM_SUBMIT_DIR" && pwd)"
else
  SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fi
case "$SCRIPT_DIR" in
  */experiments/sys-datascience/prepare) ;;
  *) echo "submit from experiments/sys-datascience/prepare, got: $SCRIPT_DIR" >&2; exit 2 ;;
esac
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
cd "$REPO_ROOT"

if [[ -z "${DATASCIENCE_PRODUCE_DIR:-}" ]]; then
  echo "DATASCIENCE_PRODUCE_DIR is required" >&2
  exit 2
fi

OUTPUT_DIR="${DATASCIENCE_TABLES_DIR:-$SCRIPT_DIR/licca-runs/datascience-prepare-${SLURM_JOB_ID}}"

mkdir -p "$OUTPUT_DIR"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/hpc/gpfs2/scratch/u/stoehljo/cargo-target}"
export RAYON_NUM_THREADS="${SLURM_CPUS_PER_TASK}"
BINARY="$CARGO_TARGET_DIR/release/sys-datascience-prepare"

echo "LICCA datascience prepare"
echo "  host:        $(hostname)"
echo "  date:        $(date -Is)"
echo "  repo:        $(git rev-parse --short HEAD)"
echo "  cpus:        ${SLURM_CPUS_PER_TASK}"
echo "  produce dir: $DATASCIENCE_PRODUCE_DIR"
echo "  output dir:  $OUTPUT_DIR"
echo "  cargo target:$CARGO_TARGET_DIR"
echo "  binary:      $BINARY"

if [[ ! -x "$BINARY" ]]; then
  echo "missing executable: $BINARY" >&2
  echo "build on the LICCA login node first:" >&2
  echo "  cd \"$REPO_ROOT\"" >&2
  echo "  export CARGO_TARGET_DIR=\"$CARGO_TARGET_DIR\"" >&2
  echo "  cargo build --release -p exp-sys-landscape --bin sys-datascience-prepare" >&2
  exit 2
fi

"$BINARY" \
  --produce-dir "$DATASCIENCE_PRODUCE_DIR" \
  --out-dir "$OUTPUT_DIR"
