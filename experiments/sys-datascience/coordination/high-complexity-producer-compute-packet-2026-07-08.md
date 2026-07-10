# High-Complexity Producer Compute Packet, 2026-07-08

Use: smoke-first LICCA execution packet for the only currently promoted
producer-axis extension from the P3 scout. This is a compute handoff, not
evidence that the run has already happened.

## Purpose

The target is a named high-complexity bucket extension inside the existing
random/product producer families:

- generic random buckets `F=10,11,12`;
- random Lagrangian-product buckets `4x6`, `5x5`, `5x6`, `6x6`;
- default height interval `[0.8,1.2]`, seed `42`, current producer contract.

The thesis question is whether this named extension changes the bounded
retained-table story:

> Under the tested high-complexity generic/product buckets, do the producers
> produce a trusted `sys > 1` row or materially change the high-tail/ridge
> interpretation?

This packet does not test arbitrary random polytope distributions, arbitrary
height intervals, adaptive search, or a generated-candidate proposer.

## Plan Files

Production plan:

```text
experiments/sys-datascience/produce/plans/two-face-control-replication.json
```

Rows:

- generic random: `F=10` with `8192` rows, `F=11` with `4096` rows, `F=12`
  with `4096` rows;
- random product: `4x6`, `5x5`, `5x6`, `6x6`, each with `4096` rows;
- total work units: `32768`.

Smoke plan:

```text
experiments/sys-datascience/produce/plans/two-face-control-licca-smoke.json
```

Rows:

- generic random: `F=10` with `3` rows, `F=11` with `5` rows, `F=12`
  with `6` rows;
- random product: `4x6` with `7` rows, `5x5` with `9` rows, `5x6` with
  `10` rows, `6x6` with `11` rows;
- total work units: `51`.

Smoke rationale:

```text
experiments/sys-datascience/produce/plans/two-face-control-licca-smoke.md
```

The smoke plan is a diagnostic plumbing plan, not a statistical sample. It is
the minimal-row plan satisfying: every active production bucket appears; every
bucket count is distinct; every bucket count is at least `3`; no bucket count
is a power of two; and total row count is minimized. Exact bucket-vector
validation is required.

## Local Plan-Only Checks

Run from `.worktrees/datascience-agent-memory` on 2026-07-08:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
  --mode production \
  --producers random,random-product \
  --output-dir /tmp/sys-ds-producer-plan-only \
  --parallelism 4 \
  --base-cache /tmp/sys-ds-producer-plan-empty-cache.jsonl \
  --plan-file experiments/sys-datascience/produce/plans/two-face-control-replication.json \
  --plan-only
```

Observed: `32768` work units, split `16384` random and `16384` random-product.

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
  --mode smoke \
  --producers random,random-product \
  --output-dir /tmp/sys-ds-producer-smoke-plan-only \
  --parallelism 4 \
  --base-cache /tmp/sys-ds-producer-plan-empty-cache.jsonl \
  --plan-file experiments/sys-datascience/produce/plans/two-face-control-licca-smoke.json \
  --plan-only
```

Observed: `51` work units, split `14` random and `37` random-product.

The unified producer reads `--base-cache` through
`ComputedPolytopeCache::load_with_wal` and writes run-local payload rows to
`$DATASCIENCE_OUTPUT_DIR/computed-polytopes.jsonl`. Its `--base-cache` schema is
the computed-polytope payload schema with `poly_id`; it is not the old
`produce/shared-cache.jsonl` family-cache schema. Use an empty run-local base
cache unless intentionally reusing another `computed-polytopes.jsonl` file from
the unified producer.

Local correction check: copying `produce/shared-cache.jsonl` into
`--base-cache` was tested and fails immediately with `missing field poly_id`.
The commands below therefore create an empty run-local base cache.

## Local Smoke/Prepare Check

After the base-cache correction and smoke-plan redesign, the diagnostic smoke
plan was run locally with an empty computed-polytope base cache:

```bash
rm -rf /tmp/sys-ds-diagnostic-smoke-local
mkdir -p /tmp/sys-ds-diagnostic-smoke-local
: > /tmp/sys-ds-diagnostic-smoke-local/base-cache-empty.jsonl
timeout 900s cargo run -p exp-sys-landscape --release \
  --bin sys-datascience-produce -- \
  --mode smoke \
  --producers random,random-product \
  --output-dir /tmp/sys-ds-diagnostic-smoke-local \
  --parallelism 4 \
  --base-cache /tmp/sys-ds-diagnostic-smoke-local/base-cache-empty.jsonl \
  --plan-file experiments/sys-datascience/produce/plans/two-face-control-licca-smoke.json
```

Observed smoke producer result:

- random rows: `14`;
- random-product rows: `37`;
- computed payload rows: `51`;
- cache hits/misses: `0` / `51`;
- failures: `0`;
- max `sys`: `0.7795037488093697`;
- rows with `sys > 1`: `0`;
- local wall time at `parallelism=4`: `17586.053798` ms.

Validation command passed:

```bash
python3 experiments/sys-datascience/produce/validate-datascience-produced.py \
  --produce-dir /tmp/sys-ds-diagnostic-smoke-local \
  --mode smoke \
  --producers random,random-product \
  --expected-plan-file experiments/sys-datascience/produce/plans/two-face-control-licca-smoke.json
```

Smoke prepare also passed:

```bash
timeout 600s cargo run -p exp-sys-landscape --release \
  --bin sys-datascience-prepare -- \
  --produce-dir /tmp/sys-ds-diagnostic-smoke-local \
  --out-dir /tmp/sys-ds-diagnostic-smoke-local-prepare
```

Observed prepare/fingerprint result:

- prepared rows/provenance rows: `51` / `51`;
- active forbidden fields: `0`;
- max `sys`: `0.7795037488093697`;
- rows with `sys > 1`: `0`;
- `polytope-table.jsonl` sha256:
  `86a53cab2dde57a9271c111105b2fc099dc3fef5b6ca1cd9350b6a3e3c6a13f9`;
- `polytope-provenance-table.jsonl` sha256:
  `1fd97aea32141a4d7cd63d35b72161e42fd6f973a938769d743f3f472d51e414`;
- local prepare wall time: `3464.852601` ms.

The explicit `scan-sys-gt-1` command needs `numpy` because it imports shared
method code; use `uv run --with numpy --script ...` until that packet's inline
dependencies are fixed.

## Execution Order

Do not submit production before the smoke job validates.

Escalation rule: if any validated row has `sys > 1`, stop unrelated
sys-datascience work and promote the positive row to a verification/review
packet before interpreting aggregate statistics.

Failure rule: if smoke validation, prepare, or fingerprinting fails, do not run
production. Record the failing command, Slurm job id, log path, and first
error.

## LICCA Login-Node Build

Run on a LICCA login node:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape \
  --bin sys-datascience-produce \
  --bin sys-datascience-prepare
```

## Smoke Submission

Run from the LICCA repo checkout:

```bash
cd "$HOME/msc-math/experiments/sys-datascience/produce"
export DATASCIENCE_MODE=smoke
export DATASCIENCE_PRODUCERS='random,random-product'
export DATASCIENCE_PLAN_FILE="$HOME/msc-math/experiments/sys-datascience/produce/plans/two-face-control-licca-smoke.json"
smoke_stamp=$(date -u +%Y%m%dT%H%M%SZ)
export DATASCIENCE_OUTPUT_DIR="$HOME/msc-math/experiments/sys-datascience/produce/licca-runs/two-face-control-smoke-${smoke_stamp}"
mkdir -p "$DATASCIENCE_OUTPUT_DIR"
: > "$DATASCIENCE_OUTPUT_DIR/base-cache-empty.jsonl"
export DATASCIENCE_BASE_CACHE="$DATASCIENCE_OUTPUT_DIR/base-cache-empty.jsonl"

smoke_jid=$(sbatch --parsable \
  --partition=test \
  --cpus-per-task=4 \
  --mem=8G \
  --time=00:20:00 \
  --export=ALL,DATASCIENCE_MODE,DATASCIENCE_PRODUCERS,DATASCIENCE_PLAN_FILE,DATASCIENCE_OUTPUT_DIR,DATASCIENCE_BASE_CACHE \
  licca-datascience-produce.slurm.sh)
printf 'smoke_jid=%s\n' "$smoke_jid"
```

Monitor:

```bash
sacct -j "$smoke_jid" \
  --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
tail -n 80 "ds-produce-${smoke_jid}.out"
```

Validate smoke output:

```bash
cd "$HOME/msc-math"
python3 experiments/sys-datascience/produce/validate-datascience-produced.py \
  --produce-dir "$DATASCIENCE_OUTPUT_DIR" \
  --mode smoke \
  --producers random,random-product \
  --expected-plan-file "$DATASCIENCE_PLAN_FILE"
```

## Production Submission

Run only after smoke validation passes:

```bash
cd "$HOME/msc-math/experiments/sys-datascience/produce"
export DATASCIENCE_MODE=production
export DATASCIENCE_PRODUCERS='random,random-product'
export DATASCIENCE_PLAN_FILE="$HOME/msc-math/experiments/sys-datascience/produce/plans/two-face-control-replication.json"
produce_stamp=$(date -u +%Y%m%dT%H%M%SZ)
export DATASCIENCE_OUTPUT_DIR="$HOME/msc-math/experiments/sys-datascience/produce/licca-runs/two-face-control-production-${produce_stamp}"
mkdir -p "$DATASCIENCE_OUTPUT_DIR"
: > "$DATASCIENCE_OUTPUT_DIR/base-cache-empty.jsonl"
export DATASCIENCE_BASE_CACHE="$DATASCIENCE_OUTPUT_DIR/base-cache-empty.jsonl"

produce_jid=$(sbatch --parsable \
  --export=ALL,DATASCIENCE_MODE,DATASCIENCE_PRODUCERS,DATASCIENCE_PLAN_FILE,DATASCIENCE_OUTPUT_DIR,DATASCIENCE_BASE_CACHE \
  licca-datascience-produce.slurm.sh)
printf 'produce_jid=%s\n' "$produce_jid"
```

The production wrapper defaults are `epyc`, `64` CPUs, `32G`, and `04:00:00`.
The run has `32768` work units, about `2.3x` the retained table's `14336`
random/product rows. Treat the wall-time estimate as unvalidated until smoke
timings are available.

Monitor:

```bash
sacct -j "$produce_jid" \
  --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
tail -n 120 "ds-produce-${produce_jid}.out"
```

Validate production output:

```bash
cd "$HOME/msc-math"
python3 experiments/sys-datascience/produce/validate-datascience-produced.py \
  --produce-dir "$DATASCIENCE_OUTPUT_DIR" \
  --mode production \
  --producers random,random-product \
  --expected-random-rows 16384 \
  --expected-random-product-rows 16384
```

## Prepare And Fingerprint

Run prepare only after production validates:

```bash
cd "$HOME/msc-math/experiments/sys-datascience/prepare"
export DATASCIENCE_PRODUCE_DIR="$DATASCIENCE_OUTPUT_DIR"
export DATASCIENCE_TABLES_DIR="$HOME/msc-math/experiments/sys-datascience/prepare/licca-runs/two-face-control-production-${produce_stamp}"

prepare_jid=$(sbatch --parsable \
  --export=ALL,DATASCIENCE_PRODUCE_DIR,DATASCIENCE_TABLES_DIR \
  licca-datascience-prepare.slurm.sh)
printf 'prepare_jid=%s\n' "$prepare_jid"
```

Monitor:

```bash
sacct -j "$prepare_jid" \
  --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
tail -n 120 "ds-prepare-${prepare_jid}.out"
```

Fingerprint and scan after prepare finishes:

```bash
cd "$HOME/msc-math"
python3 experiments/sys-datascience/fingerprint-dataset.py \
  "$DATASCIENCE_TABLES_DIR"

# The fingerprint already prints max sys and sys > 1 count. Run scan-sys-gt-1
# locally after retrieval unless LICCA has numpy available.
```

## Review Standard

Minimum accepted review after production:

- attach or copy the producer validation output, fingerprint output, Slurm job
  ids, and log tails;
- record `produce-stats.json` values for rows, cache hits/misses, failures,
  max `sys`, and wall time;
- record prepared row/provenance counts, max `sys`, and `sys > 1` count;
- if no positive appears, compare the new high-complexity bucket maxima and
  ridge summaries against the retained-table story before changing thesis
  wording;
- if a positive appears, do not summarize it as aggregate evidence first:
  verify the row, source metadata, capacity backend, exact payload, and
  reproducibility.

## Claim Boundary After The Run

If production has zero positives and passes review, it can support only the
bounded sentence:

> No trusted `sys > 1` row appeared in the tested high-complexity generic
> buckets `F=10,11,12` or product buckets `4x6`, `5x5`, `5x6`, `6x6` under the
> current producer contract.

It still does not support:

- "random polytopes do not produce counterexamples";
- "standard random models fail" without naming every model;
- calibrated hit-rate or `1M` probability language;
- positive generated-candidate proposer language;
- mechanism theorem language.

## Current Disposition

This packet is ready for an operator with LICCA access. Local agents can still
review, revise, or simulate plan-only commands, but actual production execution
is an external-access step.
