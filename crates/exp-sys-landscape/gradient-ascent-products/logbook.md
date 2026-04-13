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
`--no-db-update` disables shared-database load/save, required under rayon
par_iter because the old code load-modify-saved `crates/data/polytopes.jsonl`.

Production target: 10k seeds via `rayon::par_iter` on a single slurm task with
`--cpus-per-task=10` on the epyc partition. One output file `data/licca.jsonl`.

Architecture B (2026-04-12): rayon par_iter on one LICCA task,
`--cpus-per-task=10`. One output file (`licca.jsonl`). See
`vectorized-bouncing-gray.md` for the A→B decision rationale and
`peppy-hugging-melody.md` session state for history.

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
`data/licca.jsonl` > `data/licca-shard-*.jsonl` (legacy) > `data/smoke.jsonl`
in priority order.

### Wall-time budget

`#SBATCH --time=` in `job.sh` is a 1-second tripwire (`00:00:01`); bare
`sbatch job.sh` dies in 1 second with slurm reason `TIMEOUT`, forcing the
submitter to set the real wall time on the CLI. Real wall time is set from
the LICCA test-partition dry run (3 seeds, ~1 min) and passed via
`sbatch --time=HH:MM:SS job.sh` (CLI `--time` overrides the `#SBATCH`
directive per slurm precedence: CLI > `#SBATCH` > env > default).

No local N=1000 run — that burns Jörn's dev-machine CPU for >2h and was
explicitly forbidden after a prior incident (see `peppy-hugging-melody.md`
session state, failure modes #1 and #2).

### Byte-reproducibility re-verify

```bash
./target/release/sys-gradient-ascent-products --n 1 --n-start 5 --fresh --out /tmp/r1.jsonl
./target/release/sys-gradient-ascent-products --n 10 --n-start 0 --fresh --out /tmp/r2.jsonl
diff <(jq -c 'select(.name=="products_5")' /tmp/r1.jsonl) <(jq -c 'select(.name=="products_5")' /tmp/r2.jsonl)
# expected: no output (byte-identical row for products_5 regardless of shard range)
```

### LICCA (production, 10k seeds)

On LICCA (login node):
```bash
cd ~/msc-math/crates
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-products
cd exp-sys-landscape/gradient-ascent-products

# Test-partition dry run (single task x 3 seeds, ~3 min):
sbatch -p test --time=00:03:00 --export=ALL,N=3 job.sh
squeue -u stoehljo
cat logs/ascent-products-*.out
head data/licca.jsonl
rm data/licca.jsonl  # clean up dry-run data

# Production (CLI --time overrides the 1-second tripwire):
sbatch --time=02:00:00 job.sh
squeue -u stoehljo
sacct -j <jobid> --format=JobID,State,Elapsed,MaxRSS
```

Retrieve from devcontainer:
```bash
scp -J stoehljo@xlogin.uni-augsburg.de \
    stoehljo@licca-li-01.rz.uni-augsburg.de:'~/msc-math/crates/exp-sys-landscape/gradient-ascent-products/data/licca.jsonl' \
    crates/exp-sys-landscape/gradient-ascent-products/data/
```

Then run `uv run analyze.py` locally.

### Resuming a crashed job

Resubmit `sbatch --time=HH:MM:SS job.sh`; `load_completed_names` reads the
existing `licca.jsonl` and skips already-completed seeds. Do NOT pass
`--fresh` — that would delete the partial output.

### Files

| File | Role |
|------|------|
| `run.rs` | Binary: per-seed RNG projected ascent + overshoot + wiggle |
| `analyze.py` | Per-bucket summary + 6 figures + Bayesian bound |
| `job.sh` | Slurm submission script (epyc, single task, 10 cores, 1-second tripwire --time) |
| `data/smoke.jsonl` | Local smoke output, 3 seeds (LFS) |
| `data/licca.jsonl` | LICCA production output (N=10000, LFS) |
| `data/licca-shard-*.jsonl` | Legacy architecture-A shard outputs (LFS) — kept for post-merge reads, not produced by current `job.sh` |
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
