#!/bin/bash
#SBATCH --job-name=ascent-general
#SBATCH --partition=epyc
#SBATCH --time=04:00:00
#SBATCH --cpus-per-task=1
#SBATCH --mem=4G
#SBATCH --array=0-9
#SBATCH --output=logs/%x-%A_%a.out

# Resource justification (per .claude/skills/slurm/SKILL.md):
# | flag          | value    | why                                                         |
# |---------------|----------|-------------------------------------------------------------|
# | partition     | epyc     | wall time exceeds 4-min test partition cap                  |
# | time          | 04:00:00 | PLACEHOLDER — set from N=1000 local measurement;            |
# |               |          | logbook.md "Findings" records mean + 99th percentile        |
# |               |          | + cap-hit rate; target is 2x the mean shard wall time       |
# | cpus-per-task | 1        | Single-threaded; no rayon in repo (rust.md forbids)         |
# | mem           | 4G       | Pre-refactor runs <100 MB RSS; 4G headroom                  |
# | array         | 0-9      | 10 shards * N_PER_SHARD=1000 = 10k global seeds             |
#
# Sharding model:
# - Seed i (global index) runs with ChaCha8Rng::seed_from_u64(SEED + i) and
#   is named general_{i}. Polytope generation and wiggle() share that stream,
#   so seed i's row is byte-reproducible regardless of which shard owns it.
# - Each shard covers global indices [array_task * N_PER_SHARD, (array_task+1) * N_PER_SHARD).
# - --no-db-update: LICCA shards do NOT read or write the shared polytope
#   database (crates/data/polytopes.jsonl). Local runs without the flag
#   continue to update the cache. Rationale: 10 concurrent shards would race
#   on load-modify-save.
# - --fresh is NOT passed in production. A resubmitted crashed shard resumes
#   from its existing licca-shard-<i>.jsonl via load_completed_names.
#
# N_PER_SHARD is IMMUTABLE once this job has been first submitted. Shard i
# covers global indices [i*1000, (i+1)*1000). Changing N_PER_SHARD would
# break the index -> seed mapping and corrupt resume. To extend 10k -> 20k:
#   sbatch --array=10-19 job.sh     (do NOT change N_PER_SHARD)
#
# Test-partition dry run (fast, verifies the plumbing on LICCA hardware):
#   sbatch -p test --time=00:03:00 --array=0-0 --export=ALL,N_PER_SHARD=3 job.sh
# Dry runs produce data/licca-shard-0.jsonl with 3 rows; delete before
# production submission.

set -euo pipefail
cd "$HOME/msc-math/crates"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target

N_PER_SHARD="${N_PER_SHARD:-1000}"
EXP_DIR="exp-sys-landscape/gradient-ascent-general"
mkdir -p "$EXP_DIR/data" "$EXP_DIR/logs"

N_START=$(( SLURM_ARRAY_TASK_ID * N_PER_SHARD ))
echo "=== shard ${SLURM_ARRAY_TASK_ID}: seeds ${N_START}..$(( N_START + N_PER_SHARD )) ==="

./target/release/sys-gradient-ascent-general \
    --no-db-update \
    --n "$N_PER_SHARD" \
    --n-start "$N_START" \
    --out "$EXP_DIR/data/licca-shard-${SLURM_ARRAY_TASK_ID}.jsonl"
