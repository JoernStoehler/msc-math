# endpoint-local-max-diagnostic

## Research Question

Do sampled retained gradient-ascent endpoints still have an obvious
first-order ascent direction after quotienting out translations, scaling, and
linear symplectic directions?

Checklist anchor: local-search endpoint stability diagnostic.

## Method

This is a cheap heuristic diagnostic for the black-box search problem. It
recomputes active EHZ orbit data for a deterministic sample of retained
polytopes, builds `D_a sys` rows for the near-active orbits, forms a numerical
transversal slice to the known symmetry tangent space, and solves a maximin LP
on that slice.

The quotient model is the thesis-facing signal. Ambient strict first-order
maximality is not tested, because symmetry directions force zero one-sided
derivative in ambient coordinates.

## Inputs

- `../../prepare/polytope-table.jsonl`
- `../../prepare/polytope-provenance-table.jsonl`

The sample is deterministic:

- top 30 retained rows by `sys`;
- 10 hash-selected `gradient_ascent_general` controls;
- 10 hash-selected `gradient_ascent_products` controls;
- 10 hash-selected non-ascent controls from `random_sample` and
  `random_product_sample` when available.

Rows are deduplicated by `poly_id`; all matching selection buckets are recorded.

## Command

```bash
cargo run -p exp-sys-landscape --release --bin sys-endpoint-local-max-diagnostic
```

The command writes:

- `diagnostic.jsonl`: one row per sampled polytope;
- `summary.json`: aggregate counts and run constants.

## Observation

Last recorded local run: 2026-06-11.

Retained artifacts:

- `diagnostic.jsonl`: 59 sampled rows after deduplication.
- `summary.json`: aggregate counts and run constants.

Aggregate result:

- sampled rows: `59`;
- failures: `0`;
- quotient ascent direction found: `59`;
- deterministic step probe ran: `59`;
- step probe increased recomputed `sys`: `56`;
- `sys > 1` after step probe: `0`;
- max input `sys`: `0.9750768559799221`;
- max probed `sys`: `0.9751313886170534`.

Optimizer-mode comparison:

- sampled ascent endpoints: 10 general-mode rows and 39 Lagrangian-product-mode
  rows;
- tiny probes along the optimizer's own ascent direction increased recomputed
  `sys` for all 49 sampled ascent endpoints;
- the retained coarse line-search schedule increased `sys` for only 5 of 10
  general-mode rows and 0 of 39 Lagrangian-product-mode rows;
- all 30 `top_sys_30` rows were Lagrangian-product-mode endpoints, all 30 had
  tiny optimizer-mode increases, and none had an increase under the retained
  coarse line-search schedule.

Top-tail result:

- all 30 `top_sys_30` rows had a quotient ascent direction;
- 28 of the 30 `top_sys_30` rows had a step-probe increase.

Numerical quotient audit:

- symmetry rank was `15` for all sampled rows;
- rows with ill-conditioned or empty slice: `0`;
- max absolute `D_a sys` evaluation on symmetry tangent columns was about
  `7.6e-15`;
- max absolute quotient direction evaluation on symmetry tangent columns was
  about `2.8e-12`.

## Interpretation

This run does not support the intended endpoint-stability story. Under the
chosen active-orbit tolerance and quotient-slice model, every sampled row has
an obvious first-order quotient ascent direction, and most step probes increase
recomputed `sys`.

This weakens the use of retained gradient-ascent endpoints as evidence that
the ascent runs stopped at local-max-like endpoints. It is evidence that the
optimizer can stop even when the current first-order model still sees a small
quotient ascent step.

The optimizer-mode comparison points more specifically to the ascent line
search/stopping policy. For every sampled ascent endpoint, a tiny step along
the direction that the corresponding ascent binary would use still increases
`sys`. For product endpoints, the retained line-search schedule tries only
large fractions of the first boundary distance and misses all sampled tiny
improvements.

The run did not produce a `sys > 1` row. It also did not by itself validate a
new candidate-proposer, because the largest probed value remains below the
current `sys > 1` threshold and the probe is a tiny one-step diagnostic, not an
iterated search method.

## Validity Guards

- This is not a theorem packet and does not certify local maximality.
- The quotient slice is numerical, formed by SVD over `f64`.
- The active-orbit tolerance is relative `1e-9`, matching the current ascent
  code's near-minimum orbit policy.
- The maximin LP uses box bounds in the numerical slice coordinates.
- Step probes run only when the LP reports an obvious quotient ascent direction.
- The diagnostic is table-scoped and reads only the retained table inputs named
  above.

## Jörn Feedback

Jörn clarified that first-order maximality must be checked on a quotient or
transversal slice. Ambient strict first-order maximality is the wrong object
because symmetry directions have zero derivative.

## Related Method Folders

- `../scan-sys-gt-1/`: baseline retained-table scan for recorded `sys > 1`
  rows.

## Current Disposition

Use as a negative result for the endpoint-stability objection: the sampled
endpoints are not first-order stable under this quotient diagnostic, and they
are not stable under tiny probes along the optimizer's own ascent direction.

Do not use this packet as evidence that no candidate-proposer exists. The
opposite is the relevant caution: a follow-up could test whether iterating the
optimizer-mode tiny-step direction is a useful candidate-proposer.

## Remaining Worthwhile Questions

Worthwhile follow-up, if thesis time permits: test a minimal iterated small-step
line search on the top product rows, with stop conditions before cluster-scale
compute or broad method work. This should be a separate method packet or a
clearly marked follow-up, not hidden cleanup inside this packet.

## Predicted Stability Under Rerun

High if the retained tables, active-orbit tolerance, SVD tolerance, and LP
solver behavior are unchanged.

## Thesis Use

This packet supports a calibrated caveat: sampled retained ascent endpoints
still admit tiny improving steps under a cheap diagnostic, even in the
optimizer's own search mode. The endpoint data should not be presented as
local-max-like evidence without further qualification.

It must not be used to claim exhaustive endpoint stability, theorem-level local
maximality, or HKO-style certified strict decrease.

## Reopen Triggers

- retained tables are rebuilt;
- the ascent endpoint sample policy changes;
- derivative, capacity, or active-orbit selection code changes;
- thesis wording asks for certified local maximality rather than heuristic
  endpoint diagnostics.
