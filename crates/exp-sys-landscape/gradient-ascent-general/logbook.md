# Gradient Ascent General: Logbook

## Motivation

Can gradient-based optimization push sys past 1 for general (non-Lagrangian) polytopes? Previous gradient experiments found best non-HKO sys=0.905 (lagrangian_5x5_143 in large-scale-descent, deleted in fc298ce7) but were limited by the step-bound barrier (gradient ascent converges within each combinatorial cell but cannot cross boundaries). This experiment combines within-cell ascent with boundary-crossing strategies on general polytopes.

## Status

**2026-04-12: refactored for LICCA submission.** Binary now takes CLI args
(`--n --n-start --seed --out --fresh --no-db-update`); each global seed
index `i` runs with its own RNG stream `ChaCha8Rng::seed_from_u64(SEED + i)`
and is named `general_{i}`, so shards covering disjoint `[n_start, n_start+n)`
ranges produce byte-reproducible, disjoint rows (verified locally: running
`--n 1 --n-start 1` reproduces row `general_1` byte-for-byte from an
`--n 3 --n-start 0` run). `--no-db-update` disables shared-database load/save,
which is required for concurrent LICCA shards because the old code
load-modify-saved `crates/data/polytopes.jsonl` at each run.

Production target: 10k seeds via `--array=0-9` on the epyc partition, N_PER_SHARD=1000.
Wall-time budget in `job.sh` will be set from the local N=1000 measurement
(see "Findings" once complete).

The old committed `gradient-ascent-general.jsonl` (N=10) is **not** byte-reproducible
under the new per-seed RNG scheme and has been superseded by `data/smoke.jsonl`.

## Predecessor

Split from `boundary-crossing-search/` (2026-04-04). That experiment combined
general and Lagrangian product gradient ascent in one binary. The Lagrangian
product part is now in `gradient-ascent-products/`.

## How to run

### Local smoke (devcontainer)

```bash
cd crates/
cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-general
cd exp-sys-landscape/gradient-ascent-general
mkdir -p data
../../target/release/sys-gradient-ascent-general \
    --fresh --n 3 --n-start 0 --no-db-update \
    --out data/smoke.jsonl
uv run analyze.py
```

Expect ~20 s compute. Produces `data/smoke.jsonl`, `data/smoke-trace.jsonl`,
and the six figure files. `analyze.py` picks up
`data/licca-shard-*.jsonl` > `data/measure.jsonl` > `data/smoke.jsonl` in
priority order.

### Local measurement run (pre-submission, N=1000)

Before submitting the LICCA job, run:

```bash
./target/release/sys-gradient-ascent-general \
    --fresh --n 1000 --n-start 0 --no-db-update \
    --out data/measure.jsonl > /tmp/measure-general.log 2>&1 &
```

Expected wall time: ~1.5 h (single-threaded, ~5 s mean per seed). Extract
`total_time_ms` distribution from `data/measure.jsonl`, compute mean + 99th
percentile + `SEED_TIME_BUDGET_SECS=120` cap-hit rate, and use them to set
`--time=` in `job.sh`. See "Findings" for the recorded numbers.

### LICCA (production, 10k seeds)

On LICCA (login node):
```bash
cd ~/msc-math/crates
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-general
cd exp-sys-landscape/gradient-ascent-general

# Test-partition dry run first (1 shard x 3 seeds, ~3 min, no epyc slot burnt):
sbatch -p test --time=00:03:00 --array=0-0 --export=ALL,N_PER_SHARD=3 job.sh
squeue -u stoehljo
cat logs/ascent-general-*_0.out
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
    stoehljo@licca-li-01.rz.uni-augsburg.de:'~/msc-math/crates/exp-sys-landscape/gradient-ascent-general/data/licca-shard-*.jsonl' \
    crates/exp-sys-landscape/gradient-ascent-general/data/
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
| `run.rs` | Binary: per-seed RNG ascent + overshoot + wiggle |
| `analyze.py` | Summary table + 6 figures + Bayesian bound |
| `job.sh` | Slurm submission script (epyc, array 0-9, 1 core) |
| `data/smoke.jsonl` | Local smoke output, 3 seeds (LFS) |
| `data/measure.jsonl` | Local N=1000 measurement for wall-time budgeting (LFS) |
| `data/licca-shard-*.jsonl` | LICCA production shard outputs (LFS) |
| `gradient-ascent-general.jsonl` | Historical N=10 dataset from before the refactor — superseded, not read by the current `analyze.py` |
| `gradient_ascent_general_*.png` | Figures |

## Algorithm

Free gradient ascent in R^{4F} on general polytopes using HK2017 capacity backend. No gradient projection (unlike the Lagrangian product variant in gradient-ascent-products/).

Per seed:
1. Gradient ascent with integrated overshoot (at each iteration, tries within-bound steps AND overshoot multipliers 1.5x, 2x, 3x beyond step bound)
2. On convergence: escape rounds (up to 3 rounds x 5 wiggles per round). Each wiggle perturbs dual vertices by ~5% Gaussian noise, then re-runs gradient ascent.

## Findings from boundary-crossing-search (predecessor, 2026-03-26)

General polytopes: 10 seeds, mean sys 0.823, max sys 0.901. Warm starts from gradient-descent: mean delta 0.065. Wiggle dominated overshoot (41/42 seeds across all categories). No sys > 1 found.

## Data status

Fresh data regenerated 2026-04-04 after warm-start removal and split from boundary-crossing-search. 10 fresh general polytopes, no warm starts (warm-start source `large-scale-descent` was deleted as superseded). Polytopes use standard master seed (42), low attempt numbers, and benefit from database caching for initial capacity/volume computation.
