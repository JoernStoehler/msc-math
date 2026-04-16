# HK-O Pentagon Perturbations: Logbook

## Motivation

Viterbo's conjecture was disproved by Haim-Kislev and Ostrover (2024) via an explicit 10-facet pentagon counterexample with sys > 1. This experiment probes the local stability of that counterexample: does sys remain above 1 under small geometric perturbations, or is the counterexample an isolated point?

## Status

**2026-04-12: refactored for LICCA submission.** Binary now takes CLI args (`--n --eps --out --seed`); scale target is 10k samples per eps bucket across three eps scales (0.001 / 0.01 / 0.1), run on LICCA as a single-task slurm job. Pre-refactor findings (100 samples at eps=0.01, all retained sys > 1) stand as the starting point; they are superseded by the new LICCA run when that completes.

Old artifact `pentagon-perturb.jsonl` (committed N=101 dataset) is kept under its original name for historical reference but is not read by the new `analyze.py`. The analyzer reads `data/licca-eps-*.jsonl` when present and otherwise falls back to `data/smoke-eps-*.jsonl`.

## How to run

### Local smoke (devcontainer)

```bash
./experiments/hko-local-maximum/perturbation-neighborhood/job-smoke.sh
cd experiments/hko-local-maximum/perturbation-neighborhood
uv run analyze.py
```

Expect ~3 seconds total compute. Produces `data/smoke-eps-*.jsonl`
(three files, 21 rows each = 20 perturbed + 1 baseline), a 3-panel
histogram, stats table, and PCA table. `job-smoke.sh` is plain bash with no
SLURM directives and uses the repo-local `target/release/hko-perturbation`.

### Resume semantics

`job.sh` wipes all three bucket files via `File::create` in `main.rs`;
resubmit restarts from scratch. No `--fresh` flag; no
`load_completed_names`. If a LICCA run is killed partway, resubmitting
throws out whatever partial output exists and starts over.

### Seed policy

Three buckets share `--seed=41`. Bucket i starts from the same ChaCha8
state, but `try_perturb` retries on `Polytope4D::from_f64` failure (rare
at eps=0.001, more frequent at eps=0.1), so the u64 streams desynchronize
after the first rejection. Samples are not guaranteed to share directions
across buckets — treat the three buckets as independent draws at three
magnitudes. Historical artifact of single-eps code that was parameterized
post-hoc; not a correctness issue, but callers must not assume paired
samples.

### LICCA (production, 10k per bucket)

On LICCA (login node):
```bash
cd ~/msc-math
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-hko-local-maximum --bin hko-perturbation
cd experiments/hko-local-maximum/perturbation-neighborhood
mkdir -p logs

# Test-partition dry run first (~3 min, no epyc slot burnt):
sbatch -p test --time=00:03:00 --export=ALL,N_PER_BUCKET=3 job.sh
squeue -u stoehljo
cat logs/hko-perturbation-*.out
head data/licca-eps-*.jsonl
rm data/licca-eps-*.jsonl  # clean up dry-run data

# Production:
sbatch job.sh
squeue -u stoehljo
sacct -j <jobid> --format=JobID,State,Elapsed,MaxRSS
```

Retrieve from devcontainer:
```bash
scp -J stoehljo@xlogin.uni-augsburg.de \
    stoehljo@licca-li-01.rz.uni-augsburg.de:'~/msc-math/experiments/hko-local-maximum/perturbation-neighborhood/data/licca-eps-*.jsonl' \
    experiments/hko-local-maximum/perturbation-neighborhood/data/
```

Then run `uv run analyze.py` locally on the retrieved data.

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: generates perturbed polytopes and computes sys |
| `analyze.py` | Python: 3-panel histogram of sys values per eps, summary stats, PCA at eps=0.01 |
| `formal/hko-local-maximum/perturbation-neighborhood.tex` | Formal writeup (input by `formal/main.tex`) |
| `job.sh` | Slurm submission script (epyc, 1 core, 30 min) |
| `job-smoke.sh` | Local smoke script (plain bash, no SLURM) |
| `data/licca-eps-*.jsonl` | LICCA production output, 10k per bucket (LFS) |
| `data/smoke-eps-*.jsonl` | Local smoke output, 20 per bucket (LFS) |
| `pentagon-perturb.jsonl` | Historical N=100 dataset, not read by current analyze.py |
| `pentagon_perturb_sys_hist.png` | 3-panel histogram figure |
| `pentagon_perturb_stats.tex` | LaTeX table: per-bucket summary statistics |
| `pentagon_perturb_pca.tex` | LaTeX table: PCA components for eps=0.01 bucket |

## Design

- **Base polytope:** `known_polytopes::hko_pentagon()` (10 facets, the HKO2024 counterexample)
- **Perturbation method:** Uniform noise per component of the 4D dual vertices
  - Each of the 4 components of each dual vertex a_i perturbed by delta ~ Uniform[-eps, eps]
  - No renormalization; the perturbed vertices define the new polytope directly via `Polytope4D::from_f64`
- **Sample count per bucket (LICCA):** 10,000 perturbed + 1 baseline (one per eps sweep)
- **Eps grid:** 0.001 / 0.01 / 0.1 (two orders of magnitude, small enough that perturbations land in the same combinatorial cell, large enough at eps=0.1 to enter the nonlinear regime)
- **Algorithm:** HK2017 pruned only
- **Seed:** 41 (same master RNG per invocation; rows within a bucket are deterministic)
- **Identity:** `(eps, name)` — `name` alone is not unique across buckets, so analyze.py groups by `row.eps` first

## Findings

### Historical single-eps dataset (`pentagon-perturb.jsonl`)

These findings are historical only. They are verified against the committed
`pentagon-perturb.jsonl` artifact (101 rows) and are not what the current
`analyze.py` reads.

1. **All recorded perturbations retain sys > 1.** The committed historical dataset stays above the conjectural threshold.
2. **Historical summary statistics:** min = 1.0142, max = 1.0472, mean = 1.0289, std = 0.0051.
3. **The best recorded value matches the unperturbed HKO2024 value**, which is consistent with local-maximality heuristics in this sampled neighborhood but is not a proof.
4. **Historical PCA on the 40-dimensional perturbation vector** (4 components per facet x 10 facets) shows no dominant direction: top 5 components explain 6.51%, 6.07%, 5.79%, 5.40%, 4.96% of variance respectively.

### Current committed smoke pipeline (`data/smoke-eps-*.jsonl`)

The current analyzer reads the smoke or LICCA bucket files, not
`pentagon-perturb.jsonl`. The committed smoke files are sanity-check artifacts,
not thesis-facing production evidence.

1. `eps = 0.001` smoke rows stay close to HKO2024: min 1.0446, max 1.0472 over 21 perturbed rows.
2. `eps = 0.01` smoke rows remain above 1 in the committed sample: min 1.0210, max 1.0472 over 21 perturbed rows.
3. `eps = 0.1` smoke rows cross below 1 in the committed sample: min 0.8142, max 1.0472 over 21 perturbed rows.
4. The intended production result is still the LICCA 10k-per-bucket run; until that exists, keep historical and smoke findings explicitly separate.

## Known limitations

- Perturbation amplitude is small (epsilon = 0.01 per component); larger amplitudes are untested.
- Only random perturbations; directed perturbations along the sys gradient are not tested here (see sys-optimization).
- Facet count is fixed at F = 10. Facet-splitting perturbations (F = 11+) are not explored.
- All perturbations preserve the Lagrangian product structure implicitly (normals stay near the original product structure). True general perturbations breaking LP structure are not tested.

## Dead ends / deferred directions

- **Larger perturbation amplitudes** (epsilon = 0.05, 0.1): would explore further from HKO2024 but risk breaking polytope structure. Deferred.
- **Directed perturbations along sys-gradient:** this idea evolved into the sys-optimization experiment instead.
- **Facet-splitting (F = 11+):** splitting one facet into two explores directions invisible in the LP ambient space. Tests whether HKO2024 is a local max in the larger ambient space. Deferred.
- **Dense 2D slice sampling:** 1000-10000 points in a 2D slice (e.g., two PCA directions) could map the sys = 1 level set. Not attempted.

## Related experiments

- **sys-optimization:** Computes analytical gradients of sys and takes directed steps. The "directed perturbation" idea from this experiment's ideation.
- **gradient-descent:** Scales up sys-optimization to ~1000 polytopes.
