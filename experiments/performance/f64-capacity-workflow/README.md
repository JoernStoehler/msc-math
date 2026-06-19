# f64 Capacity Workflow Performance

This packet records the workflow-shaped performance questions for the f64
capacity path. Function-level speed is useful only when it changes a
thesis-relevant producer or search workflow.

The maintained producer is `exp-performance --bin f64-capacity-e2e`. The
verification packet `experiments/verification/f64-capacity/` owns the small
representative row manifest; this packet can reuse the same source ids for
timing.

## Measurements

Use paired rows whenever possible:

- the verification manifest rows as a quick smoke cohort;
- current exact-backed sys-landscape capacity path versus f64 capacity path;
- product-aware billiard routing versus generic HK;
- strict, LP-origin, and full-LP validation policies;
- with and without near-redundant facet removal;
- cold and hot cache producer runs;
- fallback-adjusted speed, including exact/QP fallback cost for f64-refused
  rows.

The manifest smoke reports means, candidate counts, KKT shares, and row-family
status counts. For production performance claims, add median, tail latency,
throughput, source ids for slow rows, and total producer wall time. Use
bootstrap confidence intervals only if a performance claim is needed.

For each slow row family, report:

- `sigma_count`;
- `capacity_candidate_kkt_solve_ms`;
- `capacity_candidate_kkt_solve_ms / capacity_candidate_solve_ms`;
- `F! / sigma_count` when comparing against naive facet-permutation scale;
- route/fallback status, so rows that should not have run capacity are visible.

The dense-KKT/eigendecomposition interpretation of these columns is recorded in
the `f64-capacity-e2e` section of `experiments/performance/README.md`.

The manifest cohort reuses the generated and retained-artifact source ids from
`experiments/verification/f64-capacity/manifest.json`. Edge-fixture rows in
that manifest are verification-only rows and are not part of the performance
smoke.

Run the current manifest cohort:

```bash
experiments/performance/f64-capacity-workflow/run_manifest.sh \
  /tmp/perf-f64-capacity-manifest
```

## Stop Rule

Do not run large production profiles until local paired runs are interpretable.
Do not continue performance work once capacity is no longer the bottleneck or
once additional speed would not change method-table closure, rerun feasibility,
or thesis search decisions.

## Replacement/Additions

No replacement is needed for the existing performance target. The manifest
cohort exists. Remaining additions, in priority order:

1. an exact-backed/sys-datascience baseline phase on the same retained source
   ids, if migration is the live question;
2. fallback-adjusted timing once f64 fallback policy is fixed enough that the
   fallback rate is meaningful.

Avoid standalone microbenchmarks unless a completed workflow profile shows a
specific routine remains the bottleneck.
