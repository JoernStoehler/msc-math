# f64 Capacity Performance

This directory is the physical home of f64-capacity-specific performance
binaries and summarizers. Use it for timing questions that are still coupled
to QP/f64 route design, validation policy, product routing, fallback policy,
or exact-audit boundary choices.

## `f64-capacity-e2e`

This target measures candidate f64 methods for producing capacity values inside
`datascience/`-style pipelines. It records one `input_acquisition` phase for
case loading or generation and one `f64_capacity_e2e` phase for each selected
input row and method. The per-method phase includes f64 product rounding, f64
validation, f64 capacity when validation accepts, and row classification. It
excludes exact audit.

The maintained methods are:

- `strict`: strict origin predicate plus generic transition-pruned HK.
- `lp_origin_vertex`: LP origin decision, vertex-scan geometry, generic
  transition-pruned HK.
- `lp_origin_vertex_product_billiard_or_hk`: LP origin decision, vertex-scan
  geometry, billiard sigma stream for detected products, and generic HK
  fallback otherwise.
- `lp`: LP origin decision, LP facet/pair transition geometry, generic
  transition-pruned HK.

Run retained-artifact smoke:

```bash
cargo run -p exp-dev-quadratic-program --release --bin f64-capacity-e2e -- \
  --mode smoke \
  --input-cohort retained_artifacts \
  --out-dir /tmp/perf-f64-capacity-smoke
```

Run generated-f64 smoke:

```bash
cargo run -p exp-dev-quadratic-program --release --bin f64-capacity-e2e -- \
  --mode smoke \
  --input-cohort generated_f64 \
  --out-dir /tmp/perf-f64-capacity-generated-smoke
```

Summarize phase events:

```bash
python3 experiments/dev-quadratic-program/performance/scripts/summarize_phase_jsonl.py \
  /tmp/perf-f64-capacity-smoke
```

Important columns include `sigma_count`,
`capacity_candidate_kkt_solve_ms`,
`capacity_candidate_kkt_solve_ms / capacity_candidate_solve_ms`,
`validation_bundle_ms`, `capacity_bundle_ms`, and the routine-level validation
and capacity subphase timers.

Run the verification-manifest cohort:

```bash
experiments/dev-quadratic-program/performance/run_manifest.sh \
  /tmp/perf-f64-capacity-manifest
```

The manifest cohort reuses generated and retained-artifact source ids from
`../verification/manifest.json`. Edge-fixture rows in that manifest are
verification-only rows and are not part of the performance smoke.

## `f64-decision-compare`

This target measures f64 decision routines directly, without bundling them into
a capacity run. Use it for origin-in-interior, facet presence, facet-pair
intersection, and omega-sign timing or decisiveness comparisons.

Run retained-artifact smoke:

```bash
cargo run -p exp-dev-quadratic-program --release --bin f64-decision-compare -- \
  --mode smoke \
  --input-cohort retained_artifacts \
  --out-dir /tmp/perf-f64-decision-compare-smoke
```

Run generated-f64 smoke:

```bash
cargo run -p exp-dev-quadratic-program --release --bin f64-decision-compare -- \
  --mode smoke \
  --input-cohort generated_f64 \
  --out-dir /tmp/perf-f64-decision-compare-generated-smoke
```

Summarize decision events:

```bash
python3 experiments/dev-quadratic-program/performance/scripts/summarize_decision_jsonl.py \
  /tmp/perf-f64-decision-compare-smoke \
  --csv /tmp/perf-f64-decision-compare-smoke/decision-summary.csv
```

## `f64-capacity-benchmark`

The older compact benchmark in `benchmark/` repeatedly runs the package's
f64-only capacity route on retained artifact cases and writes min/median/max
timings plus route classifications. Inspect its current CLI before using it:

```bash
cargo run -p exp-dev-quadratic-program --bin f64-capacity-benchmark -- --help
```

`support/` contains shared argument, JSONL, output-directory, and timing modules
for `f64-capacity-e2e` and `f64-decision-compare`; it is not a separate
experiment packet.

## Output Policy

Generated outputs should usually go under `/tmp`. Commit commands and schemas,
not generated timing artifacts.

Do not run large production profiles until local paired runs are interpretable.
Stop performance work once capacity is no longer the bottleneck or once
additional speed would not change method-table closure, rerun feasibility, or
thesis search decisions.
