# Alternative-generator smoke

This owner is a breadth-first feasibility pass over the 21-item mathematical
wishlist in `/tmp/joern/sys-ds-generator-wishlist.md`. It keeps law-specific
seed, parameter, attempt, and output identities and evaluates only a tiny,
stratified smoke. A row with a finite `sys` is plumbing evidence, not evidence
that a new law transfers a retained-data result.

## Laws and normalization

The implemented product-native laws are:

- the fresh current baseline (IID uniform angles and supports in `[0.8,1.2)`);
- independent equal-support factors and a centered bounded-log-support ladder
  (`sigma=0,0.1,0.2`);
- smooth log-support fields with `R=2,3`, inverse-frequency coefficient scale,
  and sampled log-support standard deviation `0.1`;
- all four paired factorial arms from the same accepted baseline normal fans;
- Dirichlet angular gaps (`alpha=.5,1,2,10`) and regular fans with zero or
  `0.1` gap-sized jitter;
- centrally symmetric strips with constant or IID widths, plus a paired
  broken-opposite-support arm and symmetric control preserving each sampled
  strip width;
- congruent factors at relative rotations zero, half a normal step, and one
  normal step; and
- inscribed circle polygons.

Every accepted factor is independently area-normalized to one before product
construction. The source `from_lagrangian_product` exact validation is the
acceptance boundary; no retained `sys`, bounce, or class-minimum value is used
for selection. Common planar rotation is fixed as gauge where convenient;
relative factor rotation remains explicit.

The log ladder clips standard Gaussian draws to `[-2,2]`, subtracts their sample
mean, and then multiplies by `sigma`, so the product of raw supports is one.
The smooth fields retain the first Fourier mode: removing it from `log h` would
not be an exact translation quotient. The Dirichlet and inscribed laws reject a
cyclic gap at least `pi`. Strip laws apply only to even side counts. Paired rows
carry a shared `pairing_id`; source IDs also include bucket, row index, and
accepted attempt.

Items 5, 11, 13--15, and 17--21 receive explicit report dispositions rather
than silent formula substitutions. They require matched-population design,
new planar geometry paths, unresolved natural choices, or generic 4D schemas
outside this narrow owner. The report distinguishes an unattempted API/schema
expansion from an observed low-acceptance result.

## CLI and artifacts

```text
cargo run -p exp-sys-landscape --release --bin sys-datascience-alternative-generator-smoke -- \
  --out-dir experiments/sys-datascience/methods/alternative-generator-smoke/artifacts \
  --seed 20260714 --attempts 64 --runtime-cap-ms 2000 --rows-per-law 4
```

The default breadth pass is generation/geometry-only. `--only-law LAW` isolates
one law for a focused rerun. `--target` opts into the existing synchronous
target backend for rows of at most ten facets; `runtime-cap-ms` classifies its
elapsed time after return, but cannot preempt that backend. Larger rows are
marked `runtime_cap` without entering it. Therefore an external process timeout
is still required for a genuinely bounded target pilot.

`smoke-rows.jsonl` contains source and pairing identity, law parameters, pair
bucket, facet count, attempts/rejections, exact-validation status, factor
support/gap CV and isoperimetric ratio, volume/capacity/sys when available,
iteration count, and generation/validation/target timings.
`batch-report.json` lists all 21 wishlist dispositions and the interpretation
boundary. Regenerate both; do not hand-edit generated JSONL or JSON.

The checked-in report's per-arm totals and coarse metric means are the source
for acceptance, cost, and first distribution-separation comparisons. These
means pool factors across the tiny named buckets; use them only to reject
obviously redundant or pathological settings, not as population estimates.
The target path, when explicitly enabled, is `capacity_auto` on each accepted
smoke row. A local runtime-cap classification is not a claim about large-sample
cost or distributional separation.
