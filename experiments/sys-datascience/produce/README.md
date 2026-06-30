# Random-Polytope Datascience Produce

This directory owns producer programs, producer caches, and producer outputs
for the active random/product sys-datascience slice.

The abandoned fixed-F ascent and continuation files were removed from this
surface. They should not be used for the current thesis datascience chapter.

## Active Producers

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

## Run-Local Producer

Use this path for producer/prepare iterations that should not mutate canonical
producer files:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
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

The known HKO pentagon can be emitted as a one-row reference/holdout source:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
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
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
  --mode production \
  --producers random,random-product \
  --output-dir /tmp/ds-produce-production \
  --parallelism 8 \
  --base-cache /tmp/ds-produce-empty-cache.jsonl
```

Validate a produced directory before prepare or promotion decisions:

```bash
python3 experiments/sys-datascience/produce/validate-datascience-produced.py \
  --produce-dir /tmp/ds-produce-smoke-cold \
  --mode smoke \
  --producers random,random-product
```

## LICCA

Use `licca-datascience-produce.slurm.sh` only for random/product production.
Build the binary on the login node before submitting so Slurm time measures the
producer job, not Rust compilation:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-datascience-produce
```

The intended execution flow is:

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
