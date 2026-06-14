# f64 Capacity

This experiment asks:

> For the `datascience/` empirical pipeline, where inputs are rounded `f64`
> dual vertices and the goal is large-scale capacity/sys exploration rather
> than theorem-grade certification per row, can most exact-backed
> geometry/capacity work be replaced by a pure-f64 pipeline that either returns
> a trusted-enough capacity value with diagnostics or explicitly classifies the
> row as needing exact fallback?

The README is not evidence for the answer. Regenerate JSONL and summaries, read
the measured f64 path, and decide from current artifacts.

## Checkpoint Scope

This checkpoint adds the development experiment packet for the following
candidate f64 policy for thesis-scale datascience scans:

1. for product-labelled inputs, round block-structured off-block drift to exact
   `0.0`;
2. optionally remove near-redundant facets with an explicit containment bound,
   either by the product-factor policy or by the generic single-band policy;
3. validate the post-preprocessing f64 dual vertices with `lp_origin_vertex`;
4. compute capacity on the post-preprocessing row with
   `product_billiard_or_hk`;
5. keep ambiguity diagnostics and route rows needing stronger witnesses to
   exact fallback.

This is not theorem-grade certification of every original row. It is a fast
empirical method that either computes the original f64 row, or computes an
explicit preprocessed row whose f64-reported containment/distortion diagnostics
remain visible. The formal implication from a valid containment bound to the
reported distortion factors is recorded in `formal/`; the f64 computation of
the bound is still an ordinary floating-point diagnostic.

Current local runs support these family-level interpretations. Re-run the
commands below before treating them as current evidence.

- Generic random rows are the strongest case for pure f64 and can be treated as
  the main viability evidence for random empirical scans.
- Random products should use product rounding and product sigma enumeration.
- Ascent product endpoints sometimes contain nearly redundant factor facets.
  Product near-redundant facet removal resolved the one observed fallback row
  in the current 200-row retained sample. On the other preprocessed rows,
  generic near-redundant facet removal removed the same facets as product
  near-redundant facet removal. It did not replace the product policy because
  it refused the fallback row on 4D vertex/incidence indeterminacy.
- HKO2024 and HKO-like highly degenerate inputs should remain degenerate stress
  fixtures or exact-fallback cases, not targets for a clean f64-only claim.

## Evidence Surfaces

- `f64-capacity-scan --input-source generated` generates rounded f64 inputs,
  validates them with f64 predicates, and runs f64 capacity only when validation
  accepts the row.  Pass `--audit-generated all` to exact-audit generated rows
  after the f64 decision has been recorded.
- `generated_random_f64` is a rejection-sampled source: raw independent f64
  dual-vertex samples are drawn until the exact-backed datascience cache accepts
  the H-rep. The accepted raw attempt id is recorded in `generated_attempt`;
  exact generation is input preparation, not part of the measured f64 path.
  `generated_product_f64` constructs Lagrangian products from f64-only bounded
  random-angle planar factors with equal positive heights, so invalid-product
  noise is not the intended signal for that family.
- `--validation-policy strict|lp_origin_vertex|lp` selects the measured f64
  validation and transition-pruning policy. `lp_origin_vertex` is the candidate
  default: it uses LP for the origin decision and vertex-scan geometry for
  facet presence/intersection. `strict` keeps the original origin predicate.
  `lp` also uses LPs for facet-existence and facet-intersection decisions.
  Those sub-millisecond geometry choices are retained as evidence surfaces;
  default selection should prioritize trust and clarity once they are below the
  E2E cost floor.
- `--capacity-method product_billiard_or_hk|transition_pruned_hk` selects the
  measured f64 sigma enumeration policy. `product_billiard_or_hk` is the scan
  default: scan preprocessing first tries tolerant block-product detection and
  rounds detected off-block drift to exact `0.0`; capacity then uses the
  billiard sigma stream for products and otherwise falls back to
  transition-pruned HK. `transition_pruned_hk` keeps the generic HK path as a
  comparison and fallback evidence surface.
- `f64-capacity-scan --input-source artifacts` replays retained datascience rows
  and hard fixtures as an audit surface for the retained empirical population.
- `f64-capacity-analyze` summarizes validation coverage, capacity outcomes,
  ambiguity counters, audit agreement, fallback causes, and timing.
- `f64-capacity-near-singular` keeps the near-singular vertex diagnostic for
  retained artifacts.
- `f64-capacity-benchmark` times the pure-f64 capacity path on retained
  artifacts; exact recomputation is still not attempted there.

Exact-backed data may appear as audit labels after a row has been generated,
validated, and classified by the measured f64 path. Exact arithmetic must not
decide f64 validation acceptance, f64 capacity enumeration, or f64 capacity
output.

## Commands

Generated-f64 audited smoke scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source generated \
  --generated-samples-per-facet 1 \
  --audit-generated all \
  --output /tmp/f64-capacity-generated-audited-smoke.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-generated-audited-smoke.jsonl \
  --json-output /tmp/f64-capacity-generated-audited-smoke-summary.json
```

Integrated smoke scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --output /tmp/f64-capacity-smoke.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-smoke.jsonl \
  --json-output /tmp/f64-capacity-smoke-summary.json
```

Targeted development scan:

Use named rows for debugging mechanism changes. Prefer one row, or a small
diverse set, before running family-level scans.

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 0 \
  --source-id-filter ascent_product_60:F10,ascent_product_131:F10,ascent_product_3222:F10 \
  --output /tmp/f64-capacity-dev-rows.jsonl
```

Targeted near-redundant facet-removal comparison:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 0 \
  --family-filter ascent_product_endpoint \
  --source-id-filter ascent_product_60:F10,ascent_product_131:F10,ascent_product_3222:F10 \
  --output /tmp/f64-capacity-product-facet-removal-off.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 0 \
  --family-filter ascent_product_endpoint \
  --source-id-filter ascent_product_60:F10,ascent_product_131:F10,ascent_product_3222:F10 \
  --near-redundant-facet-removal product \
  --near-redundant-facet-removal-delta 1e-8 \
  --audit-preprocessed all \
  --output /tmp/f64-capacity-product-facet-removal-on.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 0 \
  --family-filter ascent_product_endpoint \
  --source-id-filter ascent_product_60:F10,ascent_product_131:F10,ascent_product_3222:F10 \
  --near-redundant-facet-removal generic \
  --near-redundant-facet-removal-delta 1e-8 \
  --audit-preprocessed all \
  --output /tmp/f64-capacity-generic-facet-removal-on.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-generic-facet-removal-on.jsonl \
  --json-output /tmp/f64-capacity-generic-facet-removal-on-summary.json
```

Full generated scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source generated \
  --generated-samples-per-facet 100 \
  --generated-seed 99599604 \
  --audit-generated all \
  --output /tmp/f64-capacity-generated-full.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-generated-full.jsonl \
  --json-output /tmp/f64-capacity-generated-full-summary.json
```

Generic-HK generated comparison scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source generated \
  --generated-samples-per-facet 20 \
  --generated-seed 99599604 \
  --audit-generated all \
  --capacity-method transition_pruned_hk \
  --output /tmp/f64-capacity-policy-generic-hk-generated20.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-policy-generic-hk-generated20.jsonl \
  --json-output /tmp/f64-capacity-policy-generic-hk-generated20-summary.json
```

Full-LP generated comparison scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source generated \
  --generated-samples-per-facet 20 \
  --generated-seed 99599604 \
  --audit-generated all \
  --validation-policy lp \
  --output /tmp/f64-capacity-policy-lp-generated20-v2.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-policy-lp-generated20-v2.jsonl \
  --json-output /tmp/f64-capacity-policy-lp-generated20-v2-summary.json
```

Retained-artifact scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 0 \
  --output /tmp/f64-capacity-artifacts-full.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-artifacts-full.jsonl \
  --json-output /tmp/f64-capacity-artifacts-full-summary.json
```

Targeted hard-family retained scan:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --family-filter ascent_general_endpoint,ascent_product_endpoint,hko2024_f64 \
  --max-rows-per-family 0 \
  --output /tmp/f64-capacity-artifacts-hard-families.jsonl
```

Full-LP artifact subset:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 20 \
  --validation-policy lp \
  --output /tmp/f64-capacity-policy-lp-artifacts20-v2.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-policy-lp-artifacts20-v2.jsonl \
  --json-output /tmp/f64-capacity-policy-lp-artifacts20-v2-summary.json
```

Generic-HK artifact subset:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-scan -- \
  --input-source artifacts \
  --max-rows-per-family 20 \
  --capacity-method transition_pruned_hk \
  --output /tmp/f64-capacity-policy-generic-hk-artifacts20.jsonl
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-analyze -- \
  --input /tmp/f64-capacity-policy-generic-hk-artifacts20.jsonl \
  --json-output /tmp/f64-capacity-policy-generic-hk-artifacts20-summary.json
```

Near-singular diagnostics:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-near-singular -- \
  --max-events 2000 > /tmp/f64-near-singular-events.jsonl
```

Retained-artifact timing benchmark:

```bash
cargo run -p exp-dev-f64-capacity --release --bin f64-capacity-benchmark -- \
  --max-rows-per-family 0 \
  --repetitions 3 \
  --output /tmp/f64-capacity-benchmark.jsonl
```

## Row Contract

Rows contain both validation and capacity/audit fields.

- `validation_status` is `accepted_decisive`, `accepted_ambiguous`,
  `rejected`, or `fallback_required`.
- `validation_policy` is `strict`, `lp_origin_vertex`, or `lp`.
- `capacity_method` is `product_billiard_or_hk` or `transition_pruned_hk`.
  The analyzer groups by both `validation_policy` and `capacity_method`.
- `original_facet_count`, `facet_count`, and `product_rounding_*` fields record
  explicit product preprocessing. Capacity is computed on the row's
  post-preprocessing vertices; product rounding is not hidden inside
  `capacity_f64_only_with_policy_and_method_profiled`.
- `near_redundant_facet_removal_policy`, `near_redundant_facet_removal_status`,
  `removed_original_facets`, `near_redundant_facet_removal_delta_bound`, and the
  capacity/volume/sys ratio bounds record optional near-redundant facet removal.
  Facet removal changes the polytope; it is justified only by the reported
  containment/distortion bounds.
- Capacity labels are object-specific:
  `original_artifact_capacity_label` is for the original artifact row,
  `preprocessed_audit_capacity_label` is exact-backed capacity of the
  post-preprocessing row when requested, and `preprocessed_f64_capacity` is the
  measured f64 capacity of the post-preprocessing row.
- `preprocessed_f64_vs_original_artifact_*` and
  `preprocessed_audit_vs_original_artifact_*` compare against the original row
  through the reported near-redundant-facet distortion bound when product
  rounding did not change coordinates. If product rounding changed coordinates,
  the numeric error fields remain populated when labels exist, but the
  `*_within_bound` field is unavailable because no rounding distortion bound is
  reported. These are not same-polytope equality checks.
- `preprocessed_f64_vs_preprocessed_audit_*` is the same-polytope f64-vs-exact
  comparison when exact audit of the preprocessed row is requested.
- `validation_reasons` records f64 validation predicates and does not replace
  raw counters.
- `origin_status` and `facet_extremality_status` are tri-state strings:
  `true`, `false`, or `indeterminate`.
- `origin_lp_status`, `origin_lp_max_min_lambda`, and
  `origin_lp_max_abs_residual` are diagnostic fields. Under `strict`, they do
  not change validation acceptance. Under `lp_origin_vertex` and `lp`, the
  origin LP plus a rank check is the origin-interior validation rule.
- `facets_without_definite_vertex_count` and
  `facets_without_possible_vertex_count` diagnose facet-extremality coverage.
- Capacity runs only for `accepted_decisive` and `accepted_ambiguous` rows.
- `sigma_count` counts the number of candidate sigma words tested by the
  exhaustive sigma search. It is not a sequential optimization iteration count.
- `trust_class = clean` requires decisive validation and a clean capacity row.
- `accepted_ambiguous` rows may be useful, but they cannot be classified as
  `clean`.
- Analyzer columns named `*_validation_bundle_time_ms` time the complete f64
  validation stage. They are not origin-only sub-timers.
- Analyzer columns named `*_capacity_bundle_time_ms` include only rows where
  f64 capacity ran. Rows that stop after validation are counted in
  `capacity_not_run_rows`.
- `audit_capacity_label` and `artifact_capacity_label` are comparison labels.
  They are not inputs to the measured f64 validation or capacity decisions.
- `exact_audit_status` is `not_requested`,
  `exact_valid_capacity_success`, `exact_valid_capacity_failure`,
  `exact_validation_rejected`, or `exact_audit_error`.
- `exact_audit_time_ms` and `exact_audit_reasons` describe exact audit only.
  Exact audit is outside the measured f64 path.

## Interpreting Results

Use the analyzer to answer these questions by family:

- What fraction of generated rows are accepted decisively, accepted
  ambiguously, rejected, or marked fallback-required?
- Conditional on f64 validation acceptance, how often does f64 capacity succeed?
- Where audit labels exist, how often does f64 capacity agree within the
  configured tolerance?
- Are f64 fallback/rejected generated rows exact-valid under the audit path, or
  rejected by exact validation too?
- Which validation predicates, capacity predicates, or degeneracies cause
  fallback?
- Is f64 runtime low enough to make a datascience accelerator useful?

Generic random rows are expected to be the strongest case for pure f64.
Products of random polygons are thesis-relevant datascience inputs and should
remain in the experiment matrix. HKO2024 is a highly degenerate stress fixture,
not a typical datascience row; HKO-like generators may reasonably use the
stronger f64-exact-fallback path instead of a reject-if-inadmissible f64-only
path.

Known product rows may arrive with numerical off-block drift. Product
preprocessing only rounds rows that are already block-structured within a
relative minor-block tolerance. Retained `random_product` and
`ascent_product_endpoint` rows are rounded at load time because their producers
own the product structure. If a product-labeled retained row is not
block-structured within the tolerance, loading fails instead of silently
projecting it to a different polytope.

`--near-redundant-facet-removal product` is an explicit product-only
preprocessing policy. It rounds product blocks, works in the two 2D factors,
and removes only factor facets with a reported set-level bound
`P_original <= P_processed <= (1 + delta_bound) P_original`. The requested
`--near-redundant-facet-removal-delta` is treated as an aggregate budget over
all removed facets in a row.

`--near-redundant-facet-removal generic` is a generic 4D policy. It uses
the f64 vertex/incidence scan and removes a facet only when one retained facet
guards every definite vertex of the removed facet within the requested
tolerance. It refuses rows with bounded near-singular or ambiguous-incidence
geometry because then the f64 vertex list is not a trusted list of all vertices
of the facet being tested.

The default is `--near-redundant-facet-removal none`. Use
`--audit-preprocessed all` to exact-audit the preprocessed row after the
measured f64 decision has been recorded. The formal implication from a valid
`delta_bound` to the reported capacity, volume, and sys distortion factors is
recorded in `formal/near-redundant-facet-removal-bounds.tex`
(`rem:near-redundant-facet-removal-experiment-contract` and
`cor:facet-removal-capacity-volume-sys-bounds`). The single-band/multi-band
comparison is recorded in `cor:four-dimensional-facet-band-comparison`. The
current f64 computation of `delta_bound` is an ordinary f64 diagnostic, not an
outward-rounded certificate.

Scan preprocessing happens before f64 validation. Validation and capacity
therefore report on the post-preprocessing row. Original-artifact labels remain
comparison labels for the original row; they are not used as same-polytope
labels after facet removal.

## Current Local Run Notes

These `/tmp` files are not tracked evidence. Re-run the commands before relying
on the result.

Development runs should use `--source-id-filter` on one to five rows.
Bounded family scans are evidence runs. Full retained scans are justified only
when a thesis table or promotion decision needs full-population rates; do not
run them merely to increase statistical power.

Current near-redundant facet-removal dev rows:

- `/tmp/f64-generic-redundancy-target-none.jsonl`
- `/tmp/f64-generic-redundancy-target-product.jsonl`
- `/tmp/f64-generic-redundancy-target-generic.jsonl`
- `/tmp/f64-generic-redundancy-ascent-product200-none.jsonl`
- `/tmp/f64-generic-redundancy-ascent-product200-product.jsonl`
- `/tmp/f64-generic-redundancy-ascent-product200-generic.jsonl`
- `/tmp/f64-generic-redundancy-random20-generic.jsonl`
- `/tmp/f64-generic-redundancy-random-product20-generic.jsonl`

The targeted rows `ascent_product_60:F10`, `ascent_product_131:F10`, and
`ascent_product_3222:F10` are mechanism checks, not population evidence. In the
current runs, the no facet-removal scan classifies all three as
`degenerate_value_agrees`. This changed after `bounded_near_singular` was
tightened to require a feasible near-singular point. With
`--near-redundant-facet-removal product` and
`--near-redundant-facet-removal generic`, both policies remove the same
facet from `ascent_product_60:F10` and `ascent_product_131:F10`, with
`delta_bound < 8e-9`. Both preprocessed f64 values and preprocessed exact-audit
values are inside the original-polytope distortion budget.

Bounded retained-product matrix:

- `random_product`, 20-row sanity with `generic`: facet removal
  triggers on 0 rows; all 20 remain `degenerate_value_agrees`.
- `random`, 20-row sanity with `generic`: facet removal triggers
  on 0 rows; all 20 remain `clean`.
- `ascent_product_endpoint`, 200 rows: product near-redundant facet removal triggers on 4
  rows and removes the one remaining `fallback_required` row. Generic
  single-band facet removal triggers on 3 rows and leaves that same row as
  `fallback_required` because the relevant 4D vertex/incidence geometry is
  indeterminate. This is the current reason to keep the product-only policy in
  addition to the generic one.

Current local coverage artifacts:

- `/tmp/f64-capacity-fixed-generator-generated20.jsonl`
- `/tmp/f64-capacity-fixed-generator-generated20-summary.json`
- `/tmp/f64-capacity-fixed-generator-artifacts200.jsonl`
- `/tmp/f64-capacity-fixed-generator-artifacts200-summary.json`
- `/tmp/f64-capacity-fixed-generator-hard-families-full.jsonl`
- `/tmp/f64-capacity-fixed-generator-hard-families-full-summary.json`
- `/tmp/f64-capacity-fixed-generator-artifacts-full.jsonl`
- `/tmp/f64-capacity-fixed-generator-artifacts-partial14111-summary.json`
- `/tmp/f64-capacity-fixed-generator-random-product-full.jsonl`

Observed local wall-time scale for the current evidence bundle:

- generated audited scan, 480 rows: about 4 minutes; measured f64 capacity
  18.5 seconds, measured exact audit 128.6 seconds;
- retained random plus most random-product rows: about 10 minutes before
  interruption, 14,111 parseable rows; measured f64 capacity 679.6 seconds;
- retained hard families, 8,186 rows: about 6.5 minutes; measured f64 capacity
  368.6 seconds;
- analyzer runs are negligible compared with scan runs.

The interrupted retained random/product scan covered all `random` rows and
10,015 `random_product` rows with no fallback or disagreement. A separate
random-product-only partial reached 10,200 rows with no fallback or
disagreement. Full retained `random_product` tail completion is background-run
work, not an interactive blocker for the current policy decision.

The old strict origin predicate left exact-valid generated product rows in f64
fallback. Decision-level timing supported replacing that part by an LP origin
decision while keeping vertex-scan facet presence/intersection. This is the
`lp_origin_vertex` policy and is now the candidate default.

Full `lp` remains useful as a comparison and repair surface. It also decides
facet presence/intersection by LPs. On the local retained-artifact production
profile it produced the same value-producing product/HKO behavior as
`lp_origin_vertex`, but validation was slower and full-LP transition pruning
did not improve the product KKT bottleneck.

The current product bottleneck is not validation. With `lp_origin_vertex`,
retained product rows are accepted, but generic HK candidate solving dominates
runtime. The product-aware `product_billiard_or_hk` method cuts that cost
substantially on retained products while preserving the generic HK fallback for
non-products. Use the scan and benchmark commands above before relying on local
runtime numbers. Reusable performance-comparison targets belong in
`experiments/performance/` and are not part of this checkpoint.

## Promotion Readiness

Before promoting this into a library or `sys-datascience`, require:

- reviewed generated-f64 scans and retained-artifact scans;
- reviewed analyzer summaries of coverage, ambiguity, fallback causes, audit
  agreement, and timing;
- an explicit policy for how `accepted_ambiguous` rows are used downstream;
- an exact fallback path for `fallback_required` rows;
- no exact arithmetic inside the promoted f64 validation/capacity path.
