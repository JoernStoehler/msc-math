# QP Route / Consumer Matrix

Status: development note for QP/HK2017 route design. This is not an approved
public API and not thesis prose. It records the current consumer classification
so route work starts from final-output needs instead of from one internal
predicate or one folder boundary.

Source surfaces checked for this classification:

- `experiments/dev-quadratic-program/README.md`
- `experiments/MAP.md`
- `tasks/current-state.md`
- `tasks/planning-notes.md`
- `thesis/numerics-content.md`
- `thesis/quadratic-program-algorithm-hk2019.tex`
- `experiments/sys-datascience/{README.md,produce/README.md,prepare/README.md,methods/README.md}`
- `experiments/sys-landscape/src/{lib.rs,datascience_cache.rs,ascent/compute.rs}`
- `experiments/sys-datascience/prepare/{prepare.rs,load_caches.rs,features.rs}`
- `crates/symplectic/src/algorithms/{mod.rs,orbit_search.rs}`
- `crates/symplectic/src/database.rs`
- `crates/symplectic/src/kkt/mod.rs`

## Design Rule

Route development is caller-first:

```text
consumer decision -> final output need -> acceptable route contract
                  -> internal predicates/bounds -> evidence/proof/performance
```

The one-sigma KKT predicate is an internal mechanism. It is useful only to the
extent that it improves a final capacity/sys output, a near-minimum orbit set,
fallback cost, or diagnostic evidence for a consumer.

Do not collapse the route family into "the QP code" or "the trusted route".
Different consumers can call or copy-edit different routes.

Do not collapse downstream uses into broad ownership buckets such as
`sys-datascience method packets`, `large search`, or `crate users`. The route
contract depends on the decision the consumer is making. For example, a
`sys > 1` threshold scan needs different capacity/`sys` reliability from a
ranking diagnostic, branch-window diagnostic, timing run, or theorem-facing
exact-value claim.

## Shared Route Contracts

Current route contracts to keep separate:

| Route | Output | Correctness contract | Normal failure/uncertainty mode | Current status |
| --- | --- | --- | --- | --- |
| Raw f64 one-sigma KKT solve | f64 beta, q, action, residuals, solver diagnostics | Diagnostic only unless wrapped by another contract | numerical failure, near-singular diagnostics, ambiguous signs | Existing crate/dev code; useful for audits and performance |
| Certified f64 one-sigma KKT solve | ternary predicates and intervals | `True`/`False` only when theorem-backed; `Indet` makes no claim | `Indet` or inapplicable proof conditions | Development target; current static-margin predicate is not a proof contract |
| Heuristic f64 capacity | scalar f64 capacity/sys plus diagnostics | Empirical route only; caller must treat labels as heuristic | reject/skip/diagnostic uncertainty | Exists in dev-f64/f64-route surfaces; not thesis-facing by itself |
| Route demonstrations | executable failure examples for simpler approaches | No route contract; code is for reading, tests, and copy-editing | intended wrong answers or safe refusals, depending on the demo | Lives in `src/route_demonstrations/`; nobody currently plans to import it |
| f64 plus exact fallback capacity | scalar/list output after resolving selected f64-uncertain candidates exactly | Only as strong as the f64 candidate filter plus the fallback policy | exact fallback failure or unresolved candidate if policy cannot resolve | Existing `OrbitGuaranteeMode` route, but current `AdmissibleF64` depends on static f64 margins |
| Retained-candidate exact minimizer/gap-window capacity | exact rational capacity and exact orbits over the retained f64 candidate set | Proof-level for the retained candidate set; global only if candidate filtering is separately safe for the claim | exact solve failure or no admissible retained orbit | Existing `CertifiedOrbitSearchResult` aggregation surface |
| Exact-all-visited-sigma rational capacity | exact rational capacity and exact orbits over every visited sigma in the chosen enumeration stream | Proof-level over the visited sigma stream and exact binary64-as-rational input | exact solve failure or no admissible orbit; high runtime on broad cases | Local `src/exact_route/` reference route for small/targeted cases |
| Algebraic/Sage route | exact results for selected algebraic inputs | Proof-level for the algebraic object being claimed | CAS/proof failure | Required for selected theorem-facing algebraic examples; not a high-throughput route |

Important current gap: the crate's ordinary `OrbitSearchResult` path resolves
only `IndeterminateF64` candidates under `OrbitGuaranteeMode`. If
`AdmissibleF64` is unsound for a candidate, exact fallback does not repair that
label. The compact KKT error audit currently treats the static-margin f64
predicate as falsified on HKO-like rows. Therefore ordinary fallback routes
must not be described as generally certified until the f64 predicate contract
or fallback policy is fixed.

## Consumer Decision Matrix

Current known decision types. Add concrete consumers when a new method packet,
caller, or thesis surface appears. A broad packet owner is not enough; name the
decision that would change if capacity, `sys`, a predicate, a candidate set, or
an orbit set is wrong.

| Consumer/context | Final output needed | Acceptable uncertainty | Route fit | Current routing check |
| --- | --- | --- | --- | --- |
| KKT/numerics audits | truth tables, exact-vs-f64 rows, error terms, capacity sensitivity diagnostics | Yes, if explicit in the audit row | raw f64 one-sigma, candidate certified-f64 predicates, exact rational oracle | Must keep baseline current behavior separate from proposed theorem predicates |
| Performance probes | timing/counter comparison for a named route | Yes, if route label is explicit | raw f64, heuristic f64 capacity, fallback, certified exact depending on benchmark | Timings are not correctness evidence unless paired with a route contract |
| Heuristic large search and candidate scoring | many labels or candidate scores for exploration | Yes, if the method treats them as heuristic and records status | heuristic f64 capacity, possibly f64-with-rejection, sampled exact calibration | Do not silently reuse heuristic labels as certified thesis evidence |
| `sys > 1` threshold scans (`scan-sys-gt-1`, random-tail positive counts, perturbation positive counts) | fast `sys` values plus enough threshold reliability that a genuine positive with meaningful margin is not hidden below 1 | Only for recorded-value EDA unless a threshold error/margin check is added | currently table/producer scalar scan over recorded `sys`; stronger claim needs capacity/volume error control or exact/certified recheck near threshold | `scan-sys-gt-1` README explicitly does not validate capacity, volume, or `sys`; do not upgrade it to a true-negative threshold claim |
| Random/product tail EDA and filtered summaries | retained-sample distribution, quantiles, maxima, top rows, source/filter breakdowns | Sample-scoped uncertainty is explicit; numerical error matters most near tail/threshold and for top-row ordering | recorded table `sys` plus source contract; exact/certified recheck if thesis wording depends on tight top-tail margins | `random-tail-eda` reports finite retained sample facts, not universal random-model truth |
| Geometry-only prediction/ranking diagnostics | held-out prediction quality and top-decile enrichment against already computed `sys` | Heuristic/in-table labels are acceptable if leakage and route status stay visible | prepared table labels for in-table diagnostics; validated proposer needs new unevaluated candidates | `prediction-ranking` explicitly says it is not a validated generated-row proposer |
| Projection/anomaly exploratory structure | PCA/cluster/anomaly structure and overlap with high `sys` tail | Heuristic/in-table labels are acceptable if not promoted to proposer evidence | prepared table labels for exploratory geometry-feature analysis | `projection-structure` explicitly says anomaly/cluster labels are not candidate proposers |
| Statistical association screening | scalar feature association with retained-table `sys` | In-table explanatory uncertainty acceptable; not feature-engineering closure | prepared table labels and feature schema audit | method owns feature-screen scope, but route work must not hide target-label status |
| Trusted random/product input filter | selected retained rows and provenance exclusions | It audits row membership, not capacity truth | prepared table/provenance filters | `trusted-random-dataset` explicitly does not validate capacity or volume |
| Local/semi-local branch behavior | finite-step `sys` changes, target-minimizer status, branch-gradient prediction, branch-window diagnostics | Yes, but branch-set/status semantics must be explicit | current local behavior producer rows, certified gap-window for load-bearing branch claims, exact one-sigma audits | scalar `best_sigma` alone is not enough; compare minimizer sets, candidate windows, KKT status, and branch gradients |
| Endpoint local-max diagnostics | quotient first-order ascent direction, active/near-active gradients, tiny step probe | Heuristic diagnostic uncertainty acceptable; not theorem-level local maximality | recomputed active orbit data and numerical quotient slice | endpoint diagnostic is evidence against endpoint stability, not certified local-max proof |
| `sys-datascience` producer payload generation | expensive computed-polytope payload: capacity, sys, sigmas, orbit scalars, provenance | Must preserve enough route/status for downstream decisions; exact need depends on the decision consuming the row | currently `capacity_auto`/`capacity_billiard` via `OrbitSearchResult` | `ComputedPolytopePayloadRow` stores `backend` but not a full route-contract label; producer schema should not erase route semantics |
| `sys-datascience` retained table materialization | method-facing reusable rows and features | Must preserve label semantics needed by method packets | selected producer/table schema chosen by method-table design | `capacity_source` records dataset source, not certification strength; retained rows currently assume scalar capacity exists |
| Verification / capacity axioms | expected scalar/list result for every in-scope test input, unless testing a failure path | No hidden uncertainty; explicit expected failure is allowed | f64+exact fallback only after f64 predicate is sound enough, certified exact minimizer/gap-window, or exact route | Do not use ordinary scalar `min_action` as the only witness unless the checked route contract says that scalar is sufficient |
| Flow-graph comparison | scalar certified QP capacity; sometimes gap-window exact orbit set | No hidden uncertainty | certified exact minimizer/gap-window over rational/stored inputs | Existing flow-graph exact tests use certified QP comparison; keep this separate from ordinary scalar route |
| HKO/local proof packets | theorem-specific exact checks and human-readable verification | No hidden uncertainty in theorem claims | topic-local exact/Sage route; generic QP routes only as support/diagnostics | HKO-like f64 rows are diagnostics, not algebraic HKO evidence |
| Regular polygon / pentagon product theorem-facing examples | exact result for the intended algebraic object | No rationalized binary64 substitute unless a transfer proof is supplied | algebraic/Sage route or explicit rational-to-algebraic transfer proof | Rational exact QP certifies the stored rational input, not the exact regular polygon unless separately connected |
| Rational/stored examples and fixtures | exact result for the stored rational input | No hidden uncertainty | exact rational one-sigma/capacity aggregation, certified exact minimizer/gap-window | Need route label to distinguish stored-input exactness from algebraic-object exactness |
| Crate ordinary API consumers | ergonomic capacity result with visible guarantee/failure mode | Depends on API chosen | multiple explicit route APIs, not one collapsed result type | `OrbitSearchResult` now exposes the scalar as the field `min_action`; this is clearer than a method named `capacity`, but the result type still needs route context |

## Current API / Schema Findings

These are inspection findings, not final design decisions.

- `OrbitSearchResult` is an interval/fallback scalar payload, not an exact
  certificate. It stores `min_action`, `min_action_lower`, `min_action_upper`,
  returned `OrbitKktData`, and iterations.
- `OrbitSearchResult` exposes `min_action` as a field. It is the route's
  selected scalar action and does not by itself state whether the value is
  heuristic, fallback-safe, exact-certified, or theorem-facing.
- `OrbitAdmissibility::AdmissibleF64` is currently produced by
  `classify_margin(min(beta))` with static f64 thresholds. The KKT error audit
  currently treats this predicate as falsified on HKO-like rows.
- `OrbitGuaranteeMode::{BoundSafe, MinimaSafe, AllSafe}` resolves
  `IndeterminateF64` candidates exactly. It does not resolve candidates already
  labeled `AdmissibleF64`, so it assumes that label is sound.
- `CertifiedOrbitSearchResult` stores exact rational capacity, exact
  minimizers, optional gap-window exact orbits, and exact resolution count.
  However, it is still fed by f64 candidate enumeration; if a f64 filter can
  discard an exact capacity candidate before certification, that must be
  addressed by the route contract or by using complete exact enumeration.
- `ComputedPolytopePayloadRow` stores scalar `capacity`, `sys`, `backend`,
  `sigmas`, and `orbit_scalars`, but not a full output-contract field.
- `OrbitScalars` records some best-orbit status, including whether the best
  orbit was exact-admissible or remained f64-indeterminate. It does not record
  a route-level guarantee or whether all capacity-relevant uncertainty was
  resolved under a theorem-backed f64 predicate.
- The retained table stage preserves `capacity_source`, but this is currently a
  dataset/source label such as `random_sample`, not a certification-strength
  label.

## Development Implications

- Route work should first make the route contract explicit in the dev-QP
  packet before optimizing an internal predicate.
- Consumer inventory must be decision-based, not owner-based. A method folder,
  crate API, or experiment packet can contain threshold, ranking, branch,
  timing, diagnostic, and theorem-facing decisions with different route
  contracts.
- A useful f64 predicate can be mostly `Indet` if final capacity/sys outputs
  are still isolated or exact fallback cost is reduced.
- A high candidate-level decided rate is not a success metric by itself.
- When evaluating a proposed predicate or bound, report downstream capacity
  outputs: scalar value/status/interval and `(sigma, action)` rows up to
  `min_action + window`.
- For theorem-facing algebraic examples, rational exact results are only
  enough for the stored rational input. Use algebraic/Sage verification or an
  explicit transfer proof for exact algebraic objects.
- For datascience, route work should expose label/status semantics. The method
  packet chooses whether heuristic, certified, mixed, or rejected-row surfaces
  support its method question.
- Do not reuse a result object merely because it has the fields a caller needs.
  Prefer decision-specific rows or result structs when the guarantee differs:
  heuristic scan rows, threshold-scan rows, branch diagnostic rows,
  exact-over-retained-candidates results, exact-over-visited-sigma results, and
  theorem/algebraic certificates are different contracts even when several
  contain an action-like scalar.
- Avoid field-alias methods and route labels whose names add guarantees the
  data does not carry. A data container with `min_action` should not gain proof
  strength through a method named `capacity`; route provenance labels should
  identify route shape without words such as `safe` or `certified` unless the
  named contract supplies that guarantee.

## Route-Design Review Gates

Use these gates when adding a consumer, moving a producer, or promoting a route
from this dev packet. They are checks, not predictions that a future consumer
will fail.

- Name the route by its contract, not by a broad word such as `exact`,
  `certified`, or `fallback`.
- State whether exact work is over retained f64 candidates or over the complete
  sigma stream used by the claim.
- State whether f64 labels are diagnostic, theorem-backed predicates, or
  unresolved values that force fallback/rejection.
- State the output needed by the consumer: scalar capacity/`sys`, interval,
  minimizers, gap-window orbits, fallback counts, rejected/indeterminate
  counts, or diagnostic rows.
- State the owner for the current maturity: dev-QP while route semantics are
  changing, crate when an importable API is stable, verification/performance
  when the route contract is settled and the artifact is reusable evidence.

Variables that should not block this route design by default:

- final datascience method-table policy;
- final public crate API naming;
- SageMath theorem packets, unless a Rust/thesis surface incorrectly relies on
  Rust rationalized input for an algebraic-object claim;
- broad f64 proof polish for predicates that no current route contract needs.

## Working Route Variants

These variants are not mutually exclusive. Different consumers can call or
copy-edit different variants.

| Variant | What it changes | Primary value | Current limitation |
| --- | --- | --- | --- |
| Keep ordinary `OrbitSearchResult` route as heuristic | No correctness upgrade; improve labels/docs only | Cheap continuity for exploratory consumers | Not certified while `AdmissibleF64` is unsound |
| Exact-resolve retained f64 candidates more aggressively | Treat all capacity-window not-certified-`False` candidates as needing exact resolution | Fixes false-positive `AdmissibleF64` among retained candidates without proving a strong f64 predicate first | Still relies on f64 candidate solve not discarding exact capacity candidates |
| Certified f64 predicate plus fallback | Use theorem-backed ternary predicates and exact fallback for `Indet`/inapplicable rows | Reduces exact fallback on well-conditioned cases; gives proof-shaped route | Proof and implementation cost; retained-candidate evidence does not prove candidate-filter safety |
| Retained-candidate exact aggregation | Exact-solve retained candidate sigmas and return exact minimizers/gap window | Strong output for rational/stored inputs when candidate filtering is safe | Candidate generation can still be the weak link |
| Exact-all-visited-sigma route for small/targeted cases | Enumerate the chosen sigma stream and exact-solve every visited sigma | Direct reference result that avoids f64 candidate-retention dependence | Too expensive for broad generated/HKO sweeps without targeted/parallel design |
| Algebraic/Sage route | Solve selected algebraic examples outside the binary64 rational route | Correct theorem-facing object for regular polygons/algebraic inputs | Separate tooling; not a high-throughput Rust route |

Current route-design implication: do not try to solve the whole problem by
optimizing the retained-candidate f64 predicate alone. A certified route needs
both a retained-candidate predicate/fallback story and a candidate-filter
safety story, unless it uses complete exact enumeration.

## Current Measurement Snapshot

The compact KKT error audit was rerun on 2026-06-22 as a survivor-level route
signal:

```bash
cargo run -p exp-dev-quadratic-program --release --bin qp-kkt-error-audit -- \
  --input-source generated \
  --generated-samples-per-facet 1 \
  --max-candidates-per-case 128 \
  --output /tmp/qp-route-generated.jsonl

cargo run -p exp-dev-quadratic-program --release --bin qp-kkt-error-audit -- \
  --input-source edge-fixtures \
  --max-rows-per-family 0 \
  --max-candidates-per-case 128 \
  --output /tmp/qp-route-edge.jsonl

cargo run -p exp-dev-quadratic-program --release --bin qp-kkt-error-audit -- \
  --input-source artifacts \
  --family-filter hko2024_f64 \
  --max-rows-per-family 1 \
  --max-candidates-per-case 128 \
  --output /tmp/qp-route-hko.jsonl

python3 experiments/dev-quadratic-program/tools/kkt_error_audit/summarize.py \
  /tmp/qp-route-generated.jsonl \
  /tmp/qp-route-edge.jsonl \
  /tmp/qp-route-hko.jsonl \
  --out-dir /tmp/qp-route-summary
```

Observed runtime/resource:

- generated sample: `1094` rows, `elapsed=0:55.38`, `maxrss_kb=54240`;
- edge fixtures: `138` rows, `elapsed=0:01.60`, `maxrss_kb=54376`;
- HKO row: `128` rows, `elapsed=0:03.18`, `maxrss_kb=54512`.

Survivor-level predicate findings from `/tmp/qp-route-summary/report.md`:

| family | current f64 false positives | verified-inverse false positives | verified-inverse capacity outcome |
| --- | ---: | ---: | --- |
| `generated_random_f64` | 0 / 256 | 0 / 256 | 8 / 8 cases isolated |
| `generated_product_f64` | 0 / 838 | 0 / 838 | 10 / 16 cases isolated, 6 not isolated |
| `edge_product` | 68 / 136 | 0 / 136 | 2 / 2 cases missing-bound blocked |
| `hko2024_f64` | 24 / 128 | 0 / 128 | 1 / 1 case missing-bound blocked |

Interpretation limits:

- This audit only covers candidates returned by the f64 candidate solve. It
  does not audit sigmas that the f64 solve rejected as `Inadmissible` or failed
  before producing `OrbitKktData`.
- Therefore it is evidence about the soundness/usefulness of predicates on
  retained f64 candidates, not evidence that candidate generation/filtering is
  complete for certified exact aggregation.
- The verified-inverse predicate is useful on generated random rows, partially
  useful on generated product rows, and too often inapplicable/blocked on HKO
  and edge-product rows. This favors comparing it against exact-resolving more
  capacity-window candidates instead of assuming better f64 predicates are the
  next best route.
- The generated run is cheap enough for edit-run-summarize loops at this
  sample size. Broader all-sigma candidate-filter audits or exact-all-visited
  sigma reference routes need separate cost measurement.

## Candidate-Filter Safety Gap

Current inspection shows a sharper unresolved gap than the survivor-level audit
can answer:

- HK unpruned enumeration is combinatorial.
- HK transition-pruned enumeration uses a transition matrix built from exact
  binary64 rational facet-intersection and omega-sign data in the compact audit
  path.
- Billiard/product enumeration is combinatorial after Lagrangian-product facet
  classification, but that classification currently checks exact zero in the
  f64 coordinates.
- Candidate solving then calls `solve_orbit_sigma_saddle_point`. Candidates
  that return `OrbitSolveError::Inadmissible` or `NumericalFailure` are not
  passed to ordinary or certified aggregation.
- `aggregate_certified_orbits_with_dual_vertices_exact` exact-solves retained
  candidate sigmas, but it does not see sigmas discarded by the f64 candidate
  solve.

Thus a certified retained-candidate route needs a filter-safety audit on
small/targeted cases: enumerate the same sigma stream, exact-solve each sigma,
and compare against the f64 retained candidate set. This is different from
improving the beta predicate on retained f64 candidates.

## Candidate-Filter Audit Snapshot

`qp-candidate-filter-audit` was added as a dev-only audit binary. It enumerates
the sigma stream, exact-solves each visited sigma over binary64-as-rational
input, and compares that exact result with the f64 single-sigma solve result.
It reports:

- exact-admissible positive-`Q` sigma count;
- f64 retained count, split into `true` and `indet`;
- f64 `inadmissible` and numerical-failure counts;
- exact-admissible sigmas discarded by f64;
- exact minimizers discarded by f64.

The first measurements:

```bash
cargo run -p exp-dev-quadratic-program --release --bin qp-candidate-filter-audit -- \
  --input-source generated \
  --generated-seed 99540836 \
  --source-id-filter \
    seed99540836:F5:sample0:attempt5000000008,seed99540836:q4:p5:attempt405000000000 \
  --output /tmp/qp-filter-generated-targeted.jsonl

cargo run -p exp-dev-quadratic-program --release --bin qp-candidate-filter-audit -- \
  --input-source edge-fixtures \
  --max-rows-per-family 0 \
  --output /tmp/qp-filter-edge.jsonl

cargo run -p exp-dev-quadratic-program --release --bin qp-candidate-filter-audit -- \
  --input-source artifacts \
  --family-filter hko2024_f64 \
  --max-rows-per-family 1 \
  --max-sigmas-per-case 500 \
  --output /tmp/qp-filter-hko-cap500.jsonl
```

Observed results:

| case set | scope | runtime | exact-positive sigmas | f64 retained | false-discarded exact positives | false-discarded exact minimizers |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| generated random `F5` | complete pruned stream, 9 sigmas | `0:16.18` together with next row | 2 | 2 | 0 | 0 |
| generated product `q4:p5` | complete pruned stream, 1294 sigmas | `0:16.18` together with previous row | 45 | 45 | 0 | 0 |
| edge fixtures | complete pruned stream, 4 cases | `0:02.98` | 30 total in two product fixtures | 187 total in two product fixtures | 0 | 0 |
| HKO rounded f64 | first 500 pruned sigmas only | `0:26.58` | 19 | 28 | 0 | 0 |

Cost/coverage observations:

- An uncapped generated-bank run did not finish before manual stop at `1:42.96`.
- A capped generated-bank run over all generated cases still did not finish
  before manual stop at `1:28.24`; targeted source IDs are the right edit-loop
  shape for this audit.
- An uncapped one-row HKO run did not finish before manual stop at `1:11.56`.
  The first-500 HKO result is diagnostic only, not exhaustive HKO evidence.
- On the measured complete small/edge cases, f64 candidate filtering did not
  discard any exact-admissible positive-`Q` sigma. This is useful evidence for
  those cases only; it does not prove the retained-candidate route safe on HKO
  or on broad generated banks.

## Reopen Triggers

Revisit this matrix when:

- `OrbitSearchResult`, `CertifiedOrbitSearchResult`, or datascience payload
  schemas change;
- a theorem-facing thesis sentence changes from rational/stored input to an
  algebraic object, or conversely;
- a new f64 predicate theorem replaces the current static-margin classifier;
- a route starts doing complete exact enumeration instead of f64 candidate
  generation plus exact certification;
- a datascience method packet deliberately chooses a new label semantics;
- flow-graph, HKO, regular-product, or verification consumers add new
  capacity-output requirements.
