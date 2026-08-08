# LICCA Post-Feature Rebuild Gate

Purpose: rebuild the random/product sys-datascience tables after the
feature-space closure branch adds new prepare columns, then rerun the
random/product method packets against those rebuilt tables.

This is an evidence gate, not thesis evidence by itself. The method packet
READMEs, artifacts, and `feature-space-coverage-ledger.md` must be updated
after the rebuilt tables are available.

## Preconditions

- LICCA checkout has the exact committed branch intended for this worktree.
  Running the job from an older commit is worse than not running it.
- Canonical producer LFS files under `experiments/polytope-datasets/`
  are hydrated on LICCA.
- Do not retry the all-source retained-table rebuild locally by default. A local
  2026-06-22 rebuild loaded the canonical producer caches and was interrupted
  during table construction after the local compute/memory guard fired.
- For development, use `experiments/polytope-invariant-table/build-random-only-slice.sh smoke` or
  `experiments/polytope-invariant-table/build-random-only-slice.sh method` before any full evidence run.

## Resource BOTEC

- work units: random/product scoped build over `4096` random rows and `10240`
  random-product rows.
- execution model: one Rayon/CPU-parallel Rust process plus JSONL output.
- first production request: `32` CPUs, `32G`, `2h` on `epyc`.
- expected active Jörn time: one submission plus bounded status/log checks.
- cancel/resubmit condition: if `sacct` or the log shows immediate setup
  failure with tiny `MaxRSS`, inspect paths/LFS/checkout before resubmitting;
  if it runs for a substantial fraction of the timeout with high memory
  pressure, compare continuing versus a larger memory request before canceling.

## LICCA Login Node: Checkout And LFS

Run only after this branch has been pushed or otherwise made available to the
LICCA checkout.

```bash
cd "$HOME/msc-math"
git fetch origin sys-ds-feature-closure
git switch sys-ds-feature-closure || git switch --track origin/sys-ds-feature-closure
git pull --ff-only origin sys-ds-feature-closure
git status --short
git lfs pull --include 'experiments/polytope-datasets/*.jsonl'
```

`git status --short` should not show local changes that affect the build. If
`git switch` or `git lfs pull` reports a specific blocker, resolve that exact
path rather than cleaning broad data directories.

## LICCA Slurm Submission

```bash
cd "$HOME/msc-math"
table_jid="$(sbatch --parsable experiments/polytope-invariant-table/licca-build-retained-table.slurm.sh)"
printf 'submitted table rebuild job %s\n' "$table_jid"
```

Optional scheduler check before the real submission:

```bash
cd "$HOME/msc-math"
sbatch --test-only experiments/polytope-invariant-table/licca-build-retained-table.slurm.sh
```

## LICCA Bounded Monitoring

```bash
sacct -j "$table_jid" --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
```

```bash
cd "$HOME/msc-math"
tail -n 120 "ds-table-${table_jid}.out"
```

## LICCA Post-Run Validation

Run only after the job reaches `COMPLETED`.

```bash
cd "$HOME/msc-math"
python3 experiments/polytope-invariant-table/fingerprint-dataset.py \
  experiments/polytope-invariant-table
git status --short experiments/polytope-invariant-table
```

The fingerprint must show the expected trusted random/product counts before
method reruns are interpreted. If `sys > 1` appears, stop ordinary cleanup and
escalate the positive row.

## Local Retrieval From Devcontainer

Use this only after the LICCA validation looks plausible.

On LICCA login node:

```bash
cd "$HOME/msc-math"
tar -czf "$HOME/sys-ds-feature-closure-prepare-${table_jid}.tgz" \
  experiments/polytope-invariant-table/polytope-table.jsonl \
  experiments/polytope-invariant-table/polytope-provenance-table.jsonl
sha256sum "$HOME/sys-ds-feature-closure-prepare-${table_jid}.tgz"
```

From the local Compose workspace:

```bash
table_jid=<jobid>
scp \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  "stoehljo@licca-li-01.rz.uni-augsburg.de:~/sys-ds-feature-closure-prepare-${table_jid}.tgz" \
  /workspaces/msc-math/.worktrees/sys-ds-feature-closure/
cd /workspaces/msc-math/.worktrees/sys-ds-feature-closure
tar -xzf "sys-ds-feature-closure-prepare-${table_jid}.tgz"
```

## Local Post-Retrieval Method Reruns

```bash
cd /workspaces/msc-math/.worktrees/sys-ds-feature-closure
uv run --script experiments/polytope-invariant-table/fingerprint-dataset.py \
  experiments/polytope-invariant-table
uv run --script experiments/sys-datascience/methods/random-tail-eda/analyze.py
uv run --script experiments/sys-datascience/methods/statistical-associations/analyze.py
uv run --script experiments/sys-datascience/methods/projection-structure/analyze.py
uv run --script experiments/sys-datascience/methods/prediction-ranking/analyze.py
```

After reruns, update:

- affected method packet READMEs;
- `methods/trusted-random-product-closure-summary.md`;
- `methods/trusted-random-product-method-dispositions.md`;
- `feature-space-coverage-ledger.md`;
- review records for the method/statistics and thesis-claim gates.
