#!/bin/bash
# Merge fixed-F ascent shard outputs on LICCA.
# Submit from experiments/sys-landscape/datascience/produce with:
#   sbatch licca-merge-ascent-shards.slurm.sh
#
# This writes merged review targets. It does not promote them to the canonical
# producer filenames consumed by the table builder.

#SBATCH --job-name=ds-merge-ascent
#SBATCH --partition=epyc
#SBATCH --cpus-per-task=1
#SBATCH --mem=4G
#SBATCH --time=00:20:00
#SBATCH --output=logs/%x-%j.out

# Resource justification:
# - Merge is standard-library Python over JSONL files and should be I/O-bound.
# - 1 CPU and 4G memory are sufficient for shard-scale row-count/dedup checks.
# - 20m leaves slack for GPFS latency and large computed-polytope files.

set -euo pipefail

cd "$HOME/msc-math"

echo "LICCA datascience ascent merge"
echo "  host: $(hostname)"
echo "  date: $(date)"
echo "  repo: $(git rev-parse --short HEAD)"
echo "  extra args: ${MERGE_ARGS:-}"
echo

# Production submitters set MERGE_ARGS to `--require-cache --fresh-fixed-f`.
# Smoke submitters leave it empty because smoke shards live under
# licca-shards/general/ and licca-shards/product/.
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py \
    ${MERGE_ARGS:-}

echo
python3 experiments/sys-landscape/datascience/produce/merge-licca-ascent-shards.py \
    ${MERGE_ARGS:-} \
    --write
