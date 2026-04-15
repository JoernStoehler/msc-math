# Gradient Ascent General: Logbook

## Motivation

Can gradient-based optimization push sys past 1 for general (non-Lagrangian) polytopes? Previous gradient experiments found best non-HKO sys=0.905 (lagrangian_5x5_143 in large-scale-descent, deleted in fc298ce7) but were limited by the step-bound barrier (gradient ascent converges within each combinatorial cell but cannot cross boundaries). This experiment combines within-cell ascent with boundary-crossing strategies on general polytopes.

## Status

**2026-04-12: refactored for LICCA submission.** Binary now takes CLI args
(`--n --n-start --seed --out --fresh --no-db-update`); each global seed
index `i` runs with its own RNG stream `ChaCha8Rng::seed_from_u64(SEED + i)`
and is named `general_{i}`, so runs covering disjoint `[n_start, n_start+n)`
ranges (or rayon threads hitting disjoint indices inside one run) produce
byte-reproducible rows (verified locally: running `--n 1 --n-start 5`
reproduces row `general_5` byte-for-byte from an `--n 10 --n-start 0` run;
see "Byte-reproducibility re-verify" below). `--no-db-update` disables
shared-database load/save, which is required under rayon par_iter because
the old code load-modify-saved `data/polytopes.jsonl` at each run.

Production target: 10k seeds via `rayon::par_iter` on a single slurm task with
`--cpus-per-task=10` on the epyc partition. One output file `data/licca.jsonl`.

Architecture B (2026-04-12): rayon par_iter on one LICCA task,
`--cpus-per-task=10`. One output file (`licca.jsonl`). See
`vectorized-bouncing-gray.md` for the A→B decision rationale and
`peppy-hugging-melody.md` session state for history.

The old committed `gradient-ascent-general.jsonl` (N=10) is **not** byte-reproducible
under the new per-seed RNG scheme and has been superseded by `data/smoke.jsonl`.

## Predecessor

Split from `boundary-crossing-search/` (2026-04-04). That experiment combined
general and Lagrangian product gradient ascent in one binary. The Lagrangian
product part is now in `gradient-ascent-products/`.

## How to run

### Local smoke (devcontainer)

```bash
./experiments/sys-landscape/gradient-ascent-general/job-smoke.sh
cd experiments/sys-landscape/gradient-ascent-general
uv run analyze.py
```

Expect ~20 s compute. Produces `data/smoke.jsonl`, `data/smoke-trace.jsonl`,
and the six figure files. `job-smoke.sh` is plain bash with no SLURM
directives and uses the repo-local `target/release/sys-gradient-ascent-general`.
`analyze.py` picks up
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
./target/release/sys-gradient-ascent-general --n 1 --n-start 5 --fresh --out /tmp/r1.jsonl
./target/release/sys-gradient-ascent-general --n 10 --n-start 0 --fresh --out /tmp/r2.jsonl
diff <(jq -c 'select(.name=="general_5")' /tmp/r1.jsonl) <(jq -c 'select(.name=="general_5")' /tmp/r2.jsonl)
# expected: no output (byte-identical row for general_5 regardless of shard range)
```

### LICCA (production, 10k seeds)

On LICCA (login node):
```bash
cd ~/msc-math
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-gradient-ascent-general
cd experiments/sys-landscape/gradient-ascent-general
mkdir -p logs

# Test-partition dry run first (single task x 3 seeds, ~3 min, no epyc slot burnt):
sbatch -p test --time=00:03:00 --export=ALL,N=3 job.sh
squeue -u stoehljo
cat logs/ascent-general-*.out
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
    stoehljo@licca-li-01.rz.uni-augsburg.de:'~/msc-math/experiments/sys-landscape/gradient-ascent-general/data/licca.jsonl' \
    experiments/sys-landscape/gradient-ascent-general/data/
```

Then run `uv run analyze.py` locally.

### Resuming a crashed job

Resubmit `sbatch --time=HH:MM:SS job.sh`; `load_completed_names` reads the
existing `licca.jsonl` and skips already-completed seeds. Do NOT pass
`--fresh` — that would delete the partial output.

### Files

| File | Role |
|------|------|
| `main.rs` | Binary: per-seed RNG ascent + overshoot + wiggle |
| `analyze.py` | Summary table + 6 figures + Bayesian bound |
| `job.sh` | Slurm submission script (epyc, single task, 10 cores, 1-second tripwire --time) |
| `job-smoke.sh` | Local smoke script (plain bash, no SLURM) |
| `data/smoke.jsonl` | Local smoke output, 3 seeds (LFS) |
| `data/licca.jsonl` | LICCA production output (N=10000, LFS) |
| `data/licca-shard-*.jsonl` | Legacy architecture-A shard outputs (LFS) — kept for post-merge reads, not produced by current `job.sh` |
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
