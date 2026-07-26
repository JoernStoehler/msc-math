# Polytope Datasets

This directory contains the source-dataset producers and retained inputs used
by the random/product `sys` data-science experiments. It is a collection of
separate producer contracts, not one generic production framework.

The canonical generic-random and random-product producers remain together
because their retained outputs share `shared-cache.jsonl`. The newer run-local
producer can emit generic-random, random-product, or the named HKO reference
into a reviewable output directory. A future source does not belong here merely
because it produces JSONL or may be consumed by data-science methods.

The abandoned fixed-F ascent and continuation files were removed from this
surface. They should not be used for the current thesis datascience chapter.

## Producer contracts

- `sys-datascience-produce`: run-local random/product/reference producer that writes a
  reviewable output directory with `computed-polytopes.jsonl`,
  `random-samples.jsonl`, `random-product-samples.jsonl`,
  optional `reference-samples.jsonl`, and
  `produce-stats.json`.
- `sys-dataset-random`: older standalone generic random producer for
  canonical `random.jsonl`.
- `sys-dataset-random-product`: older standalone random Lagrangian-product
  producer for canonical `random-product.jsonl`.

Canonical committed producer artifacts:

- `random.jsonl`
- `random-product.jsonl`
- `shared-cache.jsonl`

The current retained random target is:

- `4096` generic random rows: `512` accepted samples for each facet count
  `F=5..12` at height interval `[0.8, 1.2]`;
- `10240` random Lagrangian-product rows: `1024` accepted samples for each
  polygon-pair bucket with `3 <= k <= m <= 6` at height interval `[0.8, 1.2]`;
- total standalone random/product rows: `14336`.

Both current producers use height interval `[0.8, 1.2]`, seed `42`, and
rejection until a valid polytope is produced.

Fresh run-local computed-polytope cache misses use
`volume_from_incidence_f64` on f64 primal vertices with exact-derived
incidence. Each payload records this as
`volume_method = "f64-from-exact-derived-incidence-v1"`. Older cache rows
without this field deserialize as `exact-rational-rounded-f64-v1` and remain
valid cache hits. When cache files contain both methods for the same polytope,
the loader accepts only volume and derived-`sys` differences within `1e-12`
relative error and prefers the current f64 row. Capacity and orbit data must
still match exactly. The compatibility tolerance is more than 100 times the
largest `9.51e-15` relative volume difference in the retained 512-row F10 audit
documented in
`experiments/sys-datascience/methods/generic-ridge-tail-stage1/README.md`.

## Run-Local Producer

Use this path for source/table iterations that should not mutate canonical
producer files:

```bash
cargo run -p exp-polytope-datasets --release --bin sys-datascience-produce -- \
  --mode smoke \
  --producers random,random-product \
  --output-dir /tmp/ds-produce-smoke-cold \
  --parallelism 4 \
  --base-cache /tmp/ds-produce-empty-cache.jsonl
```

Each run-local sample row stores a nested `source` object. The source object is
the sampling-event descriptor: producer family, bucket parameters, seed,
sample index, and rejection attempt. For example:

```json
{"producer":"random","facet_count":8,"h_min":0.8,"h_max":1.2,"seed":42,"sample_index":17,"attempt":0}
{"producer":"random-product","k":3,"m":5,"h_min":0.6,"h_max":1.4,"seed":42,"sample_index":17,"attempt":0,"bounces":4}
```

For targeted run-local samples, pass an unnamed bucket plan:

```json
{
  "buckets": [
    {"producer": "random", "facet_count": 8, "h_min": 0.8, "h_max": 1.2, "rows": 32},
    {"producer": "random", "facet_count": 8, "h_min": 0.6, "h_max": 1.4, "rows": 32},
    {"producer": "random-product", "k": 3, "m": 5, "h_min": 0.8, "h_max": 1.2, "rows": 32}
  ]
}
```

The bucket identity is the tuple of fields. Names are generated
deterministically from that tuple and the sample index. Older plan files with
separate `random` and `random_product` count arrays are still accepted; missing
height fields default to `[0.8, 1.2]`.

The high-complexity two-face-control files are dormant reusable plans, not a
current execution packet. They may be used only after a new research
decision selects the named generic/product bucket extension, supplies a current
cost and review gate, and prepares a separate LICCA handoff. Plan status is
summarized in `plans/README.md`.

The dormant diagnostic smoke plan is:

```text
plans/two-face-control-licca-smoke.json
```

Its companion rationale is:

```text
plans/two-face-control-licca-smoke.md
```

If new research reactivates this proposal, its distinct non-power-of-two
per-bucket counts make source/bucket/count mixups visible. Validation must then
use `--expected-plan-file`, not only family totals:

```bash
python3 experiments/polytope-datasets/validate-datascience-produced.py \
  --produce-dir "$SMOKE_DIR" \
  --mode smoke \
  --producers random,random-product \
  --expected-plan-file experiments/polytope-datasets/plans/two-face-control-licca-smoke.json
```

The known HKO pentagon can be emitted as a one-row reference/holdout source:

```bash
cargo run -p exp-polytope-datasets --release --bin sys-datascience-produce -- \
  --mode smoke \
  --producers known-hko-reference \
  --output-dir /tmp/ds-produce-hko \
  --parallelism 1 \
  --base-cache /tmp/ds-produce-hko-cache.jsonl
```

This writes `reference-samples.jsonl` with source
`known-hko-reference`. It is not part of random/product production counts.

Production mode uses the retained row counts above:

```bash
cargo run -p exp-polytope-datasets --release --bin sys-datascience-produce -- \
  --mode production \
  --producers random,random-product \
  --output-dir /tmp/ds-produce-production \
  --parallelism 8 \
  --base-cache /tmp/ds-produce-empty-cache.jsonl
```

Validate a produced directory before prepare or promotion decisions:

```bash
python3 experiments/polytope-datasets/validate-datascience-produced.py \
  --produce-dir /tmp/ds-produce-smoke-cold \
  --mode smoke \
  --producers random,random-product
```

## LICCA

`licca-datascience-produce.slurm.sh` is dormant run-local infrastructure; no
submission is selected. See `experiments/sys-datascience/LICCA.md`. If a new
research decision selects a random/product producer job, the new job-specific
handoff must reassess resources and build the binary on the login node before
submission so Slurm time measures the producer job, not Rust compilation:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-polytope-datasets --bin sys-datascience-produce
```

No LICCA job is currently selected. After a new named research decision and a reviewed
job-specific handoff, the generic producer lifecycle is:

```text
build producer binary
submit random/product produce
validate produce output
submit prepare
fingerprint/inspect prepare output
retrieve and review artifacts locally
```

For production-size diagnostics, pass `--plan-only`. This runs the same
producer work-plan construction and row-count reporting, then exits before
capacity computation and before writing producer JSONL rows.
