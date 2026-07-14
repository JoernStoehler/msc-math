# Alternative-generator smoke

This owner is a breadth-first feasibility pass over the 21-item mathematical
wishlist in `/tmp/joern/sys-ds-generator-wishlist.md`. It keeps law-specific
seed, parameter, attempt, and output identities and evaluates only a tiny,
stratified smoke. A row with a finite `sys` is plumbing evidence, not evidence
that a new law transfers a retained-data result.

## Laws and normalization

The implemented product-native laws are the fresh current baseline (uniform
angles and supports in `[0.8,1.2]`), equal-support tangential factors, a
centered bounded-log-support ladder (`sigma=0,0.1,0.2`), one-factor
factorial tangential interventions, Dirichlet angular gaps (`alpha=.5,1,2,10`),
jittered regular fans, centrally symmetric strips, broken antipodal supports,
congruent factors (relative rotations `0` and `pi/12`), and inscribed circle
polygons. Every accepted factor is independently area-normalized to one before
product construction. The source `from_lagrangian_product` exact validation is
the acceptance boundary; no retained `sys`, bounce, or class-minimum value is
used for selection.

The log ladder uses a centered Gaussian clipped to `[-2,2]` before exponentiation
and subtracts the sample mean, so the product of raw supports is one. The
Dirichlet and inscribed laws reject a cyclic gap at least `pi`. Strips and
broken-antipodal factors are only applicable to even side counts. Relative
factor rotation is meaningful; common rotation is not sampled as a separate
law.

Items 4, 5, 11, 13--15, and 17--21 receive explicit report dispositions rather
than silent formula substitutions. In particular, smooth support fields,
polarity, quotient-transverse perturbations, Poisson cells, and mixed-coordinate
four-dimensional laws need APIs or schemas outside this narrow owner.

## CLI and artifacts

```text
cargo run -p exp-sys-landscape --release --bin sys-datascience-alternative-generator-smoke -- \
  --out-dir experiments/sys-datascience/methods/alternative-generator-smoke/artifacts \
  --seed 20260714 --attempts 1 --runtime-cap-ms 2000 --rows-per-law 1
```

The default breadth pass is generation/geometry-only because a direct product
capacity call exceeded the short cap during calibration. `--target` enables
the existing target backend for a deliberately tiny follow-up; rows above ten
facets remain marked `runtime_cap`.

`smoke-rows.jsonl` contains source identity, law parameters, pair bucket, facet
count, acceptance/rejection, exact-validation status, volume/capacity/sys when
available, iteration count, and generation/validation/target timings.
`batch-report.json` lists all 21 wishlist dispositions and the interpretation
boundary. Regenerate both; do not hand-edit generated JSONL or JSON.

The recorded run produced 53 rows: 29 exact-validation survivors and 24
low-acceptance rows. Validation timing (the expensive exact boundary) ranged
from sub-millisecond generation to roughly 0.5--1.0 seconds per accepted
factor pair; target timing is zero in the default run because the calibration
showed that the direct product target path exceeds the short cap. The report's
per-law totals are the source for any later cost comparison.

The target path is `capacity_auto` on each accepted smoke row. A runtime-cap
disposition is local to this tiny run. It is not a claim about large-sample
cost or distributional separation.
