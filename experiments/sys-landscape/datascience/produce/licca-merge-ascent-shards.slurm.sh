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
#SBATCH --mem=32G
#SBATCH --time=00:20:00
#SBATCH --output=logs/%x-%j.out

# Resource justification:
# - Merge is standard-library Python over JSONL files and is mostly I/O-bound.
# - 1 CPU is enough. 32G memory is conservative for production fixed-F merges:
#   the 2026-06-09 run had 5.3G of shard JSONL and OOM-killed a 4G merge job
#   while loading/deduplicating Python dict/list rows.
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
