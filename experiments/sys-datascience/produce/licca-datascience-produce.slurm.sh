#!/usr/bin/env bash
# Status: dormant run-local producer infrastructure. This file is not an active
# submission handoff. Use only after a new named research decision and job-specific LICCA
# review; see ../LICCA.md.
#SBATCH --job-name=ds-produce
#SBATCH --partition=epyc
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=64
#SBATCH --mem=32G
#SBATCH --time=04:00:00
#SBATCH --output=ds-produce-%j.out
#SBATCH --error=ds-produce-%j.out

set -euo pipefail

# The SBATCH values are dormant historical defaults, not a current resource
# recommendation. A new handoff must review or override them for its job size.

if [[ -n "${SLURM_SUBMIT_DIR:-}" ]]; then
  SCRIPT_DIR="$(cd -- "$SLURM_SUBMIT_DIR" && pwd)"
else
  SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fi
case "$SCRIPT_DIR" in
  */experiments/sys-datascience/produce) ;;
  *) echo "submit from experiments/sys-datascience/produce, got: $SCRIPT_DIR" >&2; exit 2 ;;
esac
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
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
PLAN_ONLY="${DATASCIENCE_PLAN_ONLY:-0}"
PLAN_FILE="${DATASCIENCE_PLAN_FILE:-}"

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
BINARY="$CARGO_TARGET_DIR/release/sys-datascience-produce"

echo "LICCA datascience produce"
echo "  host:        $(hostname)"
echo "  date:        $(date -Is)"
echo "  repo:        $(git rev-parse --short HEAD)"
echo "  mode:        $MODE"
echo "  producers:   $PRODUCERS"
echo "  cpus:        ${SLURM_CPUS_PER_TASK}"
echo "  plan only:   $PLAN_ONLY"
echo "  plan file:   ${PLAN_FILE:-<none>}"
echo "  output dir:  $OUTPUT_DIR"
echo "  base cache:  $BASE_CACHE"
echo "  cargo target:$CARGO_TARGET_DIR"
echo "  binary:      $BINARY"

if [[ ! -x "$BINARY" ]]; then
  echo "missing executable: $BINARY" >&2
  echo "build on the LICCA login node first:" >&2
  echo "  cd \"$REPO_ROOT\"" >&2
  echo "  export CARGO_TARGET_DIR=\"$CARGO_TARGET_DIR\"" >&2
  echo "  cargo build --release -p exp-sys-datascience --bin sys-datascience-produce" >&2
  exit 2
fi

cmd=(
  "$BINARY"
  --mode "$MODE" \
  --producers "$PRODUCERS" \
  --output-dir "$OUTPUT_DIR" \
  --parallelism "$SLURM_CPUS_PER_TASK" \
  --base-cache "$BASE_CACHE"
)
if [[ "$PLAN_ONLY" == "1" ]]; then
  cmd+=(--plan-only)
fi
if [[ -n "$PLAN_FILE" ]]; then
  cmd+=(--plan-file "$PLAN_FILE")
fi

"${cmd[@]}"

wc -l "$OUTPUT_DIR"/*.jsonl 2>/dev/null || true
