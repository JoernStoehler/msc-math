#!/bin/bash
#SBATCH --job-name=ascent-products
#SBATCH --partition=epyc
#SBATCH --time=02:30:00
#SBATCH --cpus-per-task=1
#SBATCH --mem=4G
#SBATCH --array=0-9
#SBATCH --output=logs/%x-%A_%a.out

# Resource justification (per .claude/skills/slurm/SKILL.md):
# | flag          | value    | why                                                         |
# |---------------|----------|-------------------------------------------------------------|
# | partition     | epyc     | wall time exceeds 4-min test partition cap                  |
# | time          | 02:30:00 | PLACEHOLDER — set from N=1000 local measurement;            |
# |               |          | logbook.md "Findings" records mean + 99th percentile        |
# |               |          | + cap-hit rate; target is 2x the mean shard wall time       |
# | cpus-per-task | 1        | Single-threaded; billiard is sequential per polytope       |
# | mem           | 4G       | Pre-refactor runs <100 MB RSS; 4G headroom                  |
# | array         | 0-9      | 10 shards * N_PER_SHARD=1000 = 10k global seeds             |
#
# Sharding model:
# - Seed i (global index) runs with ChaCha8Rng::seed_from_u64(SEED + i) and
#   is named products_{i}. Bucket (q,p) is determined by i mod 3, where
#   LAGRANGIAN_SPLITS = [(3,7),(4,6),(5,5)]. Contiguous index ranges are
#   therefore evenly distributed across buckets (10k total -> ~3333/bucket).
# - Each shard covers global indices [array_task * N_PER_SHARD, (array_task+1) * N_PER_SHARD).
# - --no-db-update: LICCA shards neither read nor write the shared polytope
#   database (crates/data/polytopes.jsonl). Avoids concurrent save race.
# - --fresh is NOT passed in production. A resubmitted crashed shard resumes
#   from its existing licca-shard-<i>.jsonl via load_completed_names.
#
# N_PER_SHARD is IMMUTABLE once this job has been first submitted. Shard i
# covers global indices [i*1000, (i+1)*1000). Changing it breaks the
# index -> seed -> bucket mapping. To extend 10k -> 20k:
#   sbatch --array=10-19 job.sh     (do NOT change N_PER_SHARD)
#
# Test-partition dry run:
#   sbatch -p test --time=00:03:00 --array=0-0 --export=ALL,N_PER_SHARD=3 job.sh

set -euo pipefail
cd "$HOME/msc-math/crates"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target

N_PER_SHARD="${N_PER_SHARD:-1000}"
EXP_DIR="exp-sys-landscape/gradient-ascent-products"
mkdir -p "$EXP_DIR/data" "$EXP_DIR/logs"

N_START=$(( SLURM_ARRAY_TASK_ID * N_PER_SHARD ))
echo "=== shard ${SLURM_ARRAY_TASK_ID}: seeds ${N_START}..$(( N_START + N_PER_SHARD )) ==="

./target/release/sys-gradient-ascent-products \
    --no-db-update \
    --n "$N_PER_SHARD" \
    --n-start "$N_START" \
    --out "$EXP_DIR/data/licca-shard-${SLURM_ARRAY_TASK_ID}.jsonl"
