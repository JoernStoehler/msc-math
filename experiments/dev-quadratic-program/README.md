# Dev Quadratic Program

Status: coordination and incubator packet for unresolved QP/HK2017 route,
library-surface, numerics, performance, verification, and cleanup questions
while those questions are still coupled to QP method development. The intended
development shape is local: while QP capacity routes are still being designed,
keep their implementation experiments, numerical audits, performance probes,
and route-specific verification together here instead of spreading one coupled
question across top-level numerics, performance, and verification packets.

The importable implementation surface is still `crates/symplectic/`, but do
not use the crate as the implementation owner for active QP method development.
While a QP route is being designed, debugged, instrumented, or numerically
audited, keep the QP route implementation or copy-edited variant local to this
packet. This applies to one-sigma KKT solves, sigma enumeration,
fallback/certification, predicate logic, and route instrumentation. Library use
is still fine for code outside the dev-packet domain, such as generic geometry
helpers, retained input loading, random/polytope generation, and stable utility
types. Promote stable-enough QP kernels and routes to `crates/symplectic/` only
when multiple consumers should call them as a library.

This packet now owns the QP development package directly:

- `src/`: library-like QP route development code and local route variants;
- `tools/`: generic QP development CLIs such as f64 scan, analysis, and the
  compact KKT error-bound audit;
- `verification/`: route-specific expectation manifests and comparison tools;
- `numerics/`: route-specific numerics producers and diagnostics;
- `performance/`: route-specific timing binaries and summarizers;
- `numerics-audit/`: separate generic QP/KKT numerical-error audit over
  retained context banks.

Cross-cutting experiment homes remain useful only when the question has become
stable enough to move independently of QP method development. Do not move work
there merely because it has numerical, performance, or verification flavor:

- `experiments/performance/` for runtime, memory, counters, pruning wins, and
  scaling of stable targets;
- `experiments/verification/` for reusable correctness and regression evidence,
  including capacity axioms, agreement tests, minimum-set semantics, and
  error-path checks, once the route contract being checked is settled;
- topic folders for theorem-local or thesis-slice use.

`formal/` remains separate because LaTeX proof development has its own reference
graph and cross-reference requirements. `thesis/` remains self-contained and
publication-facing; thesis text or assets should be copied deliberately rather
than depending on this development packet.

Use this packet when a QP question is still about route naming, reusable API
shape, route comparison, numerical status, fallback semantics, performance
tradeoffs, verification scope, or cleanup ownership. In particular, f64 capacity
is one QP capacity route family, not a separate top-level method owner. Its
development code, numerics, performance, and verification packets stay together
here while changes to one surface would change the interpretation of the others.

Use `docs/route-consumer-matrix.md` before route work that can affect capacity
or `sys` output semantics. The route design frame is caller-first:

```text
consumer -> final output need -> acceptable route contract
         -> internal predicates/bounds -> evidence/proof/performance
```

Do not optimize a one-sigma f64 predicate or decided-rate metric as if it were
the route target by itself. Predicate usefulness is downstream of scalar
capacity/`sys`, near-minimum `(sigma, action)` output, fallback cost, or
diagnostic value for a named consumer.

Current f64 route caution: the current static-margin `AdmissibleF64` label is
not a theorem-backed guarantee. `OrbitGuaranteeMode` exact fallback resolves
`IndeterminateF64` candidates; it does not recheck candidates already accepted
as `AdmissibleF64`. Current search state for theorem-backed f64 predicates and
Q/action bounds lives in `tools/kkt_error_audit/SEARCH-LEDGER.md`. The separate
candidate-filter question, whether f64 discards exact-positive sigmas before
certification, is tracked in `tools/candidate_filter_audit/README.md`.

Use `experiments/algorithm-comparison/README.md` for cross-algorithm comparison
reasoning that points to performance, numerics, verification/correctness,
topic, or thesis evidence homes.

## Convention Translation

Use `HK2017-COMPARISON.md` for the broad comparison between HK2017 and this
project, including the deferred project solver layer.

The current HK2017-to-project convention audit itself lives in
`formal/hk2017-qp-conventions.tex`. Use that file for normals/heights to
dual-vertices conversion, the project `J_0`/`omega_0` convention, active-word
QP orientation, and the comparison with HK2017's reversed displayed word.

This directory remains the coordination packet for QP route, naming, API, and
cleanup questions. It should point to the formal convention note rather than
owning mathematical convention truth itself.

## Algorithm Labels

The relevant labels are defined in `experiments/MAP.md`:

- `QP/enumerate/unpruned`
- `QP/enumerate/pruned`
- `QP/enumerate/billiard`
- `QP/solve/kkt/f64`
- `QP/solve/kkt/exact`
- `QP/capacity/f64`
- `QP/capacity/fallback`
- `QP/capacity/certified`
- `QP/capacity/exact`
- `QP/recover-orbit`

`QP/capacity/exact` is reserved for a full exact/CAS-backed capacity search.
It is not the ordinary crate path. Current exact crate support includes
one-sigma exact KKT solving and f64-candidate fast paths with exact fallback or
certified exact aggregation.

## Shared Capacity Code Paths

Status: proposed coordination list, not an approved API. This list records the
current route split after checking the main consumers: `crates/symplectic`,
this packet, `numerics-audit/`, `experiments/verification`,
`experiments/performance`, `experiments/sys-datascience`,
`experiments/hko-local-maximum`, `experiments/combinatorial-cells`,
flow-graph comparisons, regular-product packets, and thesis/formal surfaces.

These are the shared capacity code paths worth developing because multiple
consumers either call them directly or copy-edit them for instrumentation,
custom enumeration, or local proof packets:

- **Raw f64 one-sigma KKT solve:** `dual_vertices_f64 + sigma -> raw
  beta/q/action/residual/solver diagnostics`. No truth guarantee. Shared by
  audits, profiling, solver comparisons, f64 experiments, and local
  instrumented copies.
- **Certified f64 one-sigma KKT solve:** `dual_vertices_f64 + sigma ->
  beta/q/action intervals + True/False/Indet/Inapplicable verdicts`.
  `True/False` must be theorem-backed. Shared by certified-f64 and f64+fallback
  route development.
- **Exact one-sigma KKT solve:** `dual_vertices_exact + sigma -> exact
  beta/q/action or exact non-success`. Proof-level one-sigma oracle. Shared by
  fallback, theorem packets, exact derivatives, audits, and exact certification
  of any retained sigma.
- **HK unpruned sigma enumeration:** reference exhaustive HK candidate
  traversal. Shared for verification, small fixtures, regression, and checking
  pruning-sensitive claims.
- **HK transition-pruned sigma enumeration:** general-polytope HK traversal
  using transition pruning. Shared by ordinary capacity, sys-datascience,
  verification, and experiment copies.
- **Product/billiard sigma enumeration:** product-specialized traversal after
  product classification. Shared by product scans, regular-products,
  HKO/product samplers, and performance-sensitive consumers.
- **Heuristic f64 capacity:** f64 validation plus f64 enumeration and f64
  one-sigma solves returning the best f64 scalar with diagnostics. Empirical
  route only. Shared by random exploration, f64 viability checks, performance
  runs, and large scans where results are audited or not thesis-facing.
- **Route demonstrations:** executable examples of why tempting simpler
  routes fail. Nobody currently plans to import this code; future consumers can
  read, run, and copy-edit it when they need a simpler heuristic and do not
  care about the missing numerical guarantees. The current literal-f64
  demonstration uses strict f64 facet/transition predicates and shows a real
  transition edge being pruned by roundoff. The exact comparisons in the HKO
  demonstrations use the binary64-rounded fixture, not algebraic HKO
  coordinates. They currently separate three KKT-level f64 issues: tiny
  positive beta values that must stay indeterminate, near-singular systems
  where f64 accepts a sigma rejected by exact binary64 rational KKT, and
  `q_error_bound` values that are residual diagnostics rather than total
  binary64-exact error certificates.
- **f64 with exact fallback capacity:** f64 enumeration/solve first, then exact
  one-sigma solve for unresolved capacity-relevant candidates. This route is
  only as strong as the f64 candidate filter plus fallback policy. The active
  route-development implementation lives in `src/fallback_route/`.
- **Retained-candidate exact minimizer/gap-window capacity:** exact one-sigma
  solves/certification over the retained f64 candidate set. Proof-level over
  that retained set; global only when candidate filtering is separately safe
  for the claim. Shared by current crate certified aggregation consumers and
  route-development comparisons.
- **Exact-all-visited-sigma rational capacity:** transition-pruned or unpruned
  sigma enumeration plus exact one-sigma solves for every visited sigma,
  returning exact capacity and minimizers/gap-window orbits over that visited
  stream. Shared by small/targeted reference checks, candidate-filter safety
  audits, flow-graph scalar comparison, and thesis/verification packets needing
  a result that does not depend on f64 candidate retention.

Do not encode use sites as separate methods. For example, exact certification
of an f64-retained sigma is a use of the exact one-sigma KKT solve, not a
separate capacity path. Policy simulators, numerics audits, HKO custom
branches, and combinatorial-cell instrumentation are analysis artifacts or
caller-owned adaptations of the shared paths above.

## Open Work For This Packet

- Extend local route code only when owned consumers, audits, or the route
  matrix identify a consumer/evidence need that the current dev-QP route cannot
  answer.
- Which expert controls should remain public for experiments, and which deep
  module paths are accidental imports?
- Which route names and result fields should become stable public crate API
  after the dev packet has enough implementation/evidence for multiple
  consumers to call them?
- What result semantics should the library promise for minimizers, gap-window
  orbit sets, rejected ambiguities, and exact fallback counts?
- Which remaining QP-coupled artifacts in top-level performance or
  verification should migrate here so future work has one packet to inspect?
- Which route-specific evidence is still missing before a route may be used for
  thesis-facing scalar capacity/`sys` claims rather than diagnostics?

## Not Owned Here

- Reusable numerical methodology that is no longer coupled to QP route design
  belongs in repo-local skill/reference material when the value is
  cross-learning rather than executable evidence.
- Capacity axioms and stable regression suites can belong in
  `experiments/verification/` or cheap crate tests once the checked route
  contract is settled.
- Runtime and memory comparisons for stable targets can belong in
  `experiments/performance/`.
- HKO-specific QP use belongs in `experiments/hko-local-maximum/` unless a
  generic QP cleanup decision is being extracted.
- Method-local datascience use belongs under
  `experiments/sys-datascience/methods/<packet>/README.md`.

## Promotion Rule

Coupled QP development work may keep code, data, timing, or verification
artifacts here while those artifacts should move together with QP method
development rather than in `crates/`, a cross-cutting experiment home, or a
topic folder. Promote importable kernels/routes into `crates/symplectic` only
after their contracts are stable enough for multiple consumers to call.
