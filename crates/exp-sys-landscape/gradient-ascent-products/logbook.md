# Gradient Ascent Products: Logbook

## Motivation

Can gradient-based optimization push sys past 1 for Lagrangian products? Previous experiments suggest Lagrangian products reach higher sys than general polytopes. This experiment performs projected gradient ascent on the Lagrangian product submanifold, preserving the product structure throughout optimization.

## Status

**2026-04-12: refactored for LICCA submission.** Binary now takes CLI args
(`--n --n-start --seed --out --fresh --no-db-update`); each global seed
index `i` runs with its own RNG stream `ChaCha8Rng::seed_from_u64(SEED + i)`
and is named `products_{i}`. Bucket `(q,p)` is determined by `i mod 3` where
`LAGRANGIAN_SPLITS = [(3,7),(4,6),(5,5)]`, so contiguous index ranges are
evenly distributed across buckets (10k total -> ~3333 per bucket).
`--no-db-update` disables shared-database load/save, required for concurrent
LICCA shards because the old code load-modify-saved `crates/data/polytopes.jsonl`.

Production target: 10k seeds via `--array=0-9` on the epyc partition, N_PER_SHARD=1000.
Wall-time budget in `job.sh` will be set from the local N=1000 measurement
(see "Findings" once complete).

The old committed `gradient-ascent-products.jsonl` (N=12, 3 buckets × 4) is
**not** byte-reproducible under the new per-seed RNG scheme and has been
superseded by `data/smoke.jsonl`.

## Predecessor

Split from `boundary-crossing-search/` (2026-04-04). That experiment combined
general and Lagrangian product gradient ascent in one binary. The general
polytope part is now in `gradient-ascent-general/`.

## How to run

### Local smoke (devcontainer)

```bash
cd crates/
cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-products
cd exp-sys-landscape/gradient-ascent-products
mkdir -p data
../../target/release/sys-gradient-ascent-products \
    --fresh --n 3 --n-start 0 --no-db-update \
    --out data/smoke.jsonl
uv run analyze.py
```

Expect ~15 s compute. Produces `data/smoke.jsonl`, `data/smoke-trace.jsonl`,
and the six figure files. Seed 0 lands in bucket `lagrangian_3x7`, seed 1 in
`lagrangian_4x6`, seed 2 in `lagrangian_5x5`. `analyze.py` picks up
`data/licca-shard-*.jsonl` > `data/measure.jsonl` > `data/smoke.jsonl` in
priority order.

### Local measurement run (pre-submission, N=1000)

```bash
./target/release/sys-gradient-ascent-products \
    --fresh --n 1000 --n-start 0 --no-db-update \
    --out data/measure.jsonl > /tmp/measure-products.log 2>&1 &
```

Expected wall time: ~1.2 h (single-threaded, ~4 s mean per seed).  Extract
`total_time_ms` distribution from `data/measure.jsonl`, compute mean + 99th
percentile + `SEED_TIME_BUDGET_SECS=120` cap-hit rate, use them to set
`--time=` in `job.sh`. See "Findings" for the recorded numbers.

### LICCA (production, 10k seeds)

On LICCA (login node):
```bash
cd ~/msc-math/crates
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-products
cd exp-sys-landscape/gradient-ascent-products

# Test-partition dry run (1 shard x 3 seeds, ~3 min):
sbatch -p test --time=00:03:00 --array=0-0 --export=ALL,N_PER_SHARD=3 job.sh
squeue -u stoehljo
cat logs/ascent-products-*_0.out
head data/licca-shard-0.jsonl
rm data/licca-shard-0.jsonl  # clean up dry-run data

# Production:
sbatch job.sh
squeue -u stoehljo
sacct -j <jobid> --format=JobID,State,Elapsed,MaxRSS
```

Retrieve from devcontainer:
```bash
scp -J stoehljo@xlogin.uni-augsburg.de \
    stoehljo@licca-li-01.rz.uni-augsburg.de:'~/msc-math/crates/exp-sys-landscape/gradient-ascent-products/data/licca-shard-*.jsonl' \
    crates/exp-sys-landscape/gradient-ascent-products/data/
```

Then run `uv run analyze.py` locally.

### Resuming a crashed shard

Resubmit a single shard with its array id; it resumes from whichever
rows are already in `licca-shard-<i>.jsonl`:
```bash
sbatch --array=<shard-id> job.sh
```
Do NOT pass `--fresh` — that would delete the partial output.

### Files

| File | Role |
|------|------|
| `run.rs` | Binary: per-seed RNG projected ascent + overshoot + wiggle |
| `analyze.py` | Per-bucket summary + 6 figures + Bayesian bound |
| `job.sh` | Slurm submission script (epyc, array 0-9, 1 core) |
| `data/smoke.jsonl` | Local smoke output, 3 seeds (LFS) |
| `data/measure.jsonl` | Local N=1000 measurement for wall-time budgeting (LFS) |
| `data/licca-shard-*.jsonl` | LICCA production shard outputs (LFS) |
| `gradient-ascent-products.jsonl` | Historical N=12 dataset from before the refactor — superseded, not read by the current `analyze.py` |
| `gradient_ascent_products_*.png` | Figures |

## Algorithm

Projected gradient ascent on the Lagrangian product submanifold using billiard capacity backend. Gradient direction is projected to preserve Lagrangian product structure: q-facets keep zero p-components ([2],[3] zeroed), p-facets keep zero q-components ([0],[1] zeroed).

Per seed:
1. Gradient ascent with integrated overshoot (at each iteration, tries within-bound steps AND overshoot multipliers 1.5x, 2x, 3x beyond step bound)
2. On convergence: escape rounds (up to 3 rounds x 5 wiggles per round). Each wiggle perturbs dual vertices by ~5% Gaussian noise, then re-runs gradient ascent.

## Findings from boundary-crossing-search (predecessor, 2026-03-26)

Lagrangian products: 12 seeds, mean sys 0.821, max sys 0.933. Best overall sys=0.933 (lagrangian_3x7_1). Wiggle dominated overshoot. No sys > 1 found.

## Data status

Fresh data generated 2026-04-04 after warm-start removal and split from boundary-crossing-search. 12 fresh Lagrangian products (3 splits x 4 per bucket), no warm starts (warm-start source `large-scale-descent` was deleted as superseded). Polytopes use standard master seed (42), low attempt numbers, and benefit from database caching for initial capacity/volume computation.
