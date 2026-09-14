# Capacity and systolic-ratio calculation map

This is the repository-level map of materially different ways in which
`c_EHZ` or `sys = c_EHZ^2/(2 Vol_4)` is calculated. It is a routing aid, not a
new guarantee. Numerical and fit-for-purpose is the normal status of much of
the experimental work; arithmetic, search coverage, failure semantics, and
evidence strength must be read separately.

“Exact binary64 input” below means that the stored `f64` coordinates are
treated as the dyadic rationals they represent. It does **not** enclose an
unknown source object that was rounded to those coordinates.

## Choose from the intended result

| Intended result | Default route | Record or check |
| --- | --- | --- |
| Fresh exploratory population | Use the run-local `sys-datascience-produce` path. Use a heuristic-only f64 route when its output remains explicitly diagnostic. | Sampling contract, rejected cases, capacity method and candidate family, volume method, source revision, and dirty state. |
| Thesis-facing scalar for stored binary64 coordinates | Use `capacity_4d::capacity_from_dual_vertices`, then `capacity_value` with a claim-appropriate relative tolerance. | Exact stored input, result variant, outward bounds or exact product value, tolerance, and rerunnable check. If the claim concerns an algebraic body instead, use its theorem-specific route. |
| Orbit, branch, or derivative research | Use `qp_minimizers_from_dual_vertices`, `general_qp_action_window`, or `solve_sigma_exact` according to whether the need is tied words, a general-HK window, or one chosen word. Legacy orbit routes remain useful controls when their payload is the point. | Candidate family, window, unsupported/rejected cases, and exact witnesses used. Product closure winners do not promise all physical branches. |
| Exact theorem-specific statement | Use the topic-local Sage certificate, or the exact flow-graph route only when its input and regularity hypotheses match. | The human mathematical implication as well as verifier source, exact input/witness, complete recorded output, and reproduction command. |
| Retained historical `sys` value | Start with `experiments/polytope-datasets/retained-lineage.md`, then the named producer and consumer. | Treat absent method metadata as unrecorded. Preserve historical hashes and paths; do not retroactively apply a later evaluator contract. |

These defaults choose a calculation; they do not decide what a result means.
For example, an exact one-word solve can support branch work while providing no
global-capacity claim, and a bounded scalar may be the most appropriate input
to a numerical population analysis.

## Contract map

| Path and owner | Arithmetic model | Search / candidate-family coverage | Failure or indeterminate semantics and error control | Provenance, cost, intended use, and reader audit |
| --- | --- | --- | --- | --- |
| **Production general QP capacity** — `crates/symplectic/src/algorithms/capacity_4d/{mod,general}.rs` | Exact dyadic-rational geometry and transition signs; guarded `f64` LBLT/inverse-defect and interval predicates; lazy `BigRational` KKT fallback; outward `f64` capacity bounds. | Complete simple cyclic HK words in the exact transition-pruned graph, subject to the public limits of 16 facets, primal/dual infinity norms in `[1e-3,1e3]`, and at most 100,000 materialized cycles. Rich requests exact-resolve every tied minimizing word or every admissible word in an inclusive exact action-multiple window. | Unsupported size/geometry, candidate overflow, no positive candidate, or failed exact contender resolution are typed errors. Definite interval decisions are used; unresolved predicates fall back to exact arithmetic. `capacity_value` emits a midpoint only when the outward interval meets the requested relative error. No source-coordinate uncertainty is propagated. | This is the ordinary non-product scalar API. The implementation names its formal lemmas, has a copy-correspondence suite and QP numerics/verification packets. Warm July 2026 measurements put a representative validated `F=10` pipeline near 11 ms/input, but exact fallback can dominate transformed cases; candidate count alone is not a cost model. A reader can inspect the public types and exact fallback boundary, but the proof is distributed across code and `formal/hk2017-qp-*.tex` rather than embodied in one small certificate. |
| **Production Lagrangian-product closure route** — `capacity_4d/product.rs` | Interval-filtered `f64` support enumeration with exact `BigRational` fallback; exact rational final contender comparison and exact capacity; outward conversion bounds. | Exact structural q/p products dispatch here. It enumerates factor closure vertices supported on pairs/triples and their cyclic interleavings under the six-facet reduction. Returned words are all tied **closure-vertex-family** winners, not every physical orbit in a degenerate within-word family; no product action-window API exists. | Non-product input to the forced route and post-validation invariant failures are explicit errors. Interval overlap/invalidity triggers exact support or full exact fallback. The exact rational result controls the bounds. | Ordinary product scalar/minimizer path, backed by `formal/product-qp-six-facet-reduction.tex` and exact-vs-hybrid tooling under `experiments/dev-quadratic-program/tools/product_closure_route/`. It avoids the general candidate materialization, but runtime depends on factor support enumeration; current docs provide route-specific measurements rather than an asymptotic guarantee. The small candidate construction and exact final comparison are comparatively direct to audit, conditional on the reduction theorem. |
| **Legacy/research general HK orbit route** — `algorithms/hk2017/`, `algorithms/orbit_search.rs`, and `experiments/sys-landscape::capacity_pruned_hk2017` | `f64` saddle-point KKT solves and heuristic action windows; exact rational re-solves for candidates selected by `BoundsSafe`, `MinimaSafe`, or `AllSafe` aggregation policies. | Unpruned facet-count enumeration or transition-pruned simple HK words, depending on frontend. The `sys-landscape` general path uses the pruned stream and `MinimaSafe`. Its scalar is a global capacity only when the chosen stream and aggregation policy establish the needed coverage; its returned candidates are not automatically physical orbits. | `NoAdmissibleOrbit`, numerical failure, exact-fallback failure, and invalid gap are typed. The raw `q_error_bound` and action windows are explicitly heuristic, not proved enclosures. Selective exact fallback improves the requested minimum policy but does not turn those diagnostics into certified intervals. | Still used by orbit-sensitive ascent/derivative consumers and retained legacy artifacts; it is not the ordinary scalar API. One-word solves are cheap (about 6 microseconds in the retained July 2026 profile), while end-to-end cost is enumeration-dependent. The policy boundary is visible in source, but a reader must reconstruct which frontend and guarantee mode each consumer chose. |
| **Billiard-restricted HK orbit route** — `algorithms/billiard/` plus the shared orbit/KKT collector; `sys-landscape::capacity_billiard` | Same `f64`-first, exact-fallback aggregation as the preceding path. | Only valid structural products; enumerates feasible alternating q/p block words with two or three bounces. `capacity_auto` in the legacy `sys-landscape` layer selects it after `f64` product classification. This is a specialized sigma source, not general-HK coverage and not the production closure-vertex family. | Product-classification errors and shared orbit-search errors are explicit. The `sys-landscape` wrapper uses `MinimaSafe`; the same heuristic-window caveat applies. | Intended for product orbit/branch diagnostics and legacy `sys` evaluation. It can retain physically useful billiard words that do not appear in the sparse closure-vertex witness family. Audit requires reading both block enumeration and shared aggregation, so provenance should say “billiard” rather than merely “QP”. |
| **Exact rational flow graph** — `algorithms/flow_graph/{exact_search,exact_tube}.rs` | `BigRational` tube geometry, fixed-point resolution, actions, comparisons, and cutoffs. | Exhaustive transition-pruned simple closed words for the checked four-dimensional rational input class. The exact action cutoff is only a speedup after a positive action is known; enabled-vs-disabled tests check output agreement. This route deliberately excludes zero-omega transitions (hence common products/HKO cases), dependent facet presentations, and unresolved positive singular fixed sets. | Invalid input, unsupported zero omega, dependent presentations, unsupported singular states, word-resolution failure, and exhaustive no-positive-orbit are typed non-success outcomes. Empty tubes and proved zero/non-strict cases are exact no-orbit outcomes, not numerical indeterminacy. | The thesis-facing generic flow-graph implementation path and a comparison target for QP, not the ordinary library dispatcher. It is exact but combinatorially and algebraically expensive (the retained ignored `F=7` checks took roughly two minutes each in release mode). `algorithms/flow_graph/README.md` gives the clearest single contract, including the runtime/theorem gap; exactness does not by itself prove that gap closed. |
| **Specialized Sage theorem certificates** — notably `experiments/regular-products/pentagon-rotation-formula-proof/` and `experiments/hko-local-maximum/theorem/` | Exact ordered algebraic number fields, polynomial/rational-function arithmetic, exact signs, ranks, and identities; `AA`/intervals select the intended real embeddings or roots. Rust/`f64` may propose finite witness choices but is not trusted for acceptance. | Problem-specific. The pentagon proof classifies all 3,340 raw sigma cases on its parameter half-domain and proves a closed `sys(theta)` formula. The HKO packet checks 26 selected feasible upper branches sufficient for its quotient-local-maximality certificate; it explicitly does not catalogue every nearby minimizing branch or implement a general capacity API. | A failed exact predicate aborts the certificate. Prefix/limited pentagon runs are smoke checks and cannot print the full-pass claim. Tracked summaries/stdout are evidence only for the matching source/version and command. | These are theorem artifacts for named families/statements, not reusable capacity calculators. Their finite predicates and trust boundaries are highly auditable from the explained verifier/proof script, but the mathematical conclusion also depends on the linked formal/thesis argument and stated domain. Runtime is deliberately secondary and full runs are not part of ordinary builds. |

## `sys` consumer and retained-artifact provenance

Capacity provenance and volume provenance are independent. The shared current
data-science cache in `experiments/sys-landscape/src/datascience_cache.rs`
records both:

- current capacity: `certified-qp-minimizers-v1`, obtained from production
  `qp_minimizers`, then accepted at relative capacity tolerance `1e-10`;
- current volume: `f64-from-exact-derived-incidence-v1`;
- legacy defaults on rows lacking those fields: `legacy-orbit-search-v1` and
  `exact-rational-rounded-f64-v1`.

Thus a retained decimal `sys` is normally a fit-for-purpose numerical derived
value even when its capacity came with an exact rational action or certified
bounds. The cache preserves capacity bounds, an optional exact-capacity string,
candidate-family name, and method fields so consumers can audit that boundary.
It also checks upgraded values against legacy values, but agreement is
regression evidence, not proof that the two candidate families enumerate the
same witnesses.

Not every live experiment has migrated to that cache. In particular,
`experiments/sys-landscape/src/ascent/compute.rs` still calls the legacy
`capacity_auto` route because derivative work needs orbit payloads. Older
random/product producers also directly form `cap * cap / (2*vol)` from their
selected route. Therefore “retained sys-datascience artifact” is not a single
arithmetic or coverage contract: check each row's method metadata and producer
before making a numerical-strength claim. The trusted-table filters establish
finite dataset provenance and row validity, not a theorem about unsampled
polytopes or the prevalence of `sys > 1`.

## What is not a capacity path

- `capacity_4d::solve_sigma_exact` and `crates/symplectic/src/exact/` solve one
  caller-supplied word exactly. They provide a kernel or witness, not search
  coverage and hence not a scalar capacity by themselves.
- Derivative, orbit-recovery, predictor, and plotting code consume a selected
  capacity/orbit contract; they do not strengthen it.
- Experiment route names, “trusted” dataset labels, agreement tests, and
  tracked JSON/JSONL files do not silently upgrade heuristic intervals or
  finite sampling into certified global claims.

## Evidence entry points

- Test roles and runnable confidence layers: `docs/algorithm-testing.md`.
- Public production contract: `crates/symplectic/README.md` and
  `experiments/dev-quadratic-program/docs/capacity-architecture.md`.
- Numerical and runtime evidence:
  `experiments/dev-quadratic-program/numerics-audit/` and
  `experiments/dev-quadratic-program/performance/CURRENT_COSTS.md`.
- Cross-route regression evidence: `experiments/verification/README.md`.
- Flow-graph scope and theorem/runtime boundary:
  `crates/symplectic/src/algorithms/flow_graph/README.md`.
- Current data-science interpretation:
  `experiments/sys-datascience/coordination/current-question-map.md` and each
  producer's README/manifest.

This map intentionally makes no subjective quality ranking. A claim is easiest
to audit when its input object, finite candidate family, failure outcomes,
arithmetic boundary, producer version, and retained evidence are all named.
