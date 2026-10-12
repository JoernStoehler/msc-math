# Capacity and quadratic programming

The Haim–Kislev formula reduces polytope EHZ capacity to a finite search over
facet words and nonnegative weights. This topic owns the convention translation,
fixed-word KKT algebra, exact quadratic-program structure, numerical enclosure
proofs and transition-pruning argument. Its central distinction is between a
stationary witness for one word and the global scalar capacity. A numerical
solver diagnostic, an exact stationary witness and a complete certified search
are three different outputs.

The importable implementation remains in
[`symplectic::algorithms::capacity_4d`](../../crates/symplectic/src/algorithms/capacity_4d/mod.rs).
The [`QP development packet`](development/README.md)
retains route comparisons, instrumented copies, rejected alternatives and
numerical/performance evidence. Those coupled experiment assets were relocated intact
without regenerating retained data. The selected reader-facing treatment remains
[`thesis Chapter 4`](../../thesis/chapters/04-quadratic-program.tex); reorganizing
proof development does not promote new thesis claims or confer human approval.

## Formula, word conventions and the KKT witness

Let the bounded full-dimensional four-polytope have an irredundant normalized
facet presentation `a_i · x <= 1`. For an active traversal word sigma, closure
and normalization are `C beta = d`, with four dual-coordinate rows, one all-ones
row and `d=(0,0,0,0,1)`. The symmetric action matrix has
`H_ij=omega_0(a_sigma(i),a_sigma(j))` for `i<j`, zero diagonal, and
`Q=beta^T H beta/2`. The capacity is `1/(2 Q_max)`, where the maximum is over
the complete word and feasible-weight family, not one selected word.

The [convention proof](proofs/hk2017-qp-conventions.tex) translates the
normal/height weights and reverses HK2017's displayed order to active traversal
order. The [QP core](proofs/hk2017-qp-core.tex),
`lem:hk2017-full-to-subset-words`, proves that zero-weight deletion/insertion
and cyclic rotation preserve the scalar family maximum. It does not identify
fixed named words before reversing their order. The 11 October pilot added this
bridge in an `unverified` wrapper; that status is retained.

The [KKT algebra](proofs/ehz-kkt-system.tex), `lem:kkt`, solves
`H beta + C^T lambda = 0` together with `C beta=d`. A strictly positive solution
is stationary on its support, and every exact solution of the same KKT system
has the same Q (`lem:well-defined`), including a singular affine solution
family. Stationarity alone does not establish a fixed-word maximum. In this
sign convention the normalization multiplier xi satisfies `2Q+xi=0`.

For scalar capacity, per-word second-order maximality is unnecessary when
candidate coverage is complete: every positive witness is feasible and has
Q at most the global maximum; a global maximizer becomes a positive stationary
witness after zero entries are removed. This is
`rem:kkt-witness-versus-capacity`. A *global* maximizer realizes a minimum-action
generalized Reeb orbit through the dual-action correspondence (`fact:hk2017`).
An arbitrary positive KKT witness does not determine physical placement on the
boundary or a complete family of minimizing orbits.

## Direct certificates, conditional precision and historical diagnostics

The [precision entry point](proofs/hk2017-qp-precision.tex) separates two source
packets while preserving their labels and their order in the formal document.

| Source | What it establishes | Required boundary |
| --- | --- | --- |
| [Direct KKT certificate](proofs/direct-kkt-certificate.tex) | Outward residual and inverse-defect enclosures; strict beta signs; Q/action intervals via xi; certified positive-curvature exclusions and cyclic inheritance | Exact KKT matrix for the supplied binary64 coordinates, supported arithmetic, finite outward bounds, defect norm strictly below one |
| [Conditional precision](proofs/conditional-precision.tex) | Conditional perturbation links, exact Taylor identities and retained proposals/diagnostics | The computable beta proposal has unproved constants and an invalid denominator substitution; the runtime Q diagnostic is not a certificate |
| [KKT residual lemma](proofs/ehz-kkt-system.tex), `lem:q-error-bound` | An exact-residual correction estimate assuming a nonsingular symmetric KKT matrix and a positive spectral lower bound | A rounded residual and a thresholded computed eigenvalue do not supply those hypotheses |

For the direct route, outward bounds `e >= ||I-KR||_inf` with `e < 1`, and
`r >= ||K xhat-b||_inf` imply invertibility and the solution enclosure
`rho=||R||_inf r/(1-e)`. Assembly error between K and its rounded matrix is
included explicitly. The normwise variant is cheaper; an indeterminate result
passes to the tighter entrywise variant and then to exact rational fallback.
A singular system cannot pass this invertibility test and instead needs the
exact stationary-family/positivity solve. Failure to certify is not a rejection
of the exact candidate.

The [production general route](../../crates/symplectic/src/algorithms/capacity_4d/general.rs)
already uses this direct route. Its arithmetic contract includes gradual
underflow, outward rounding of the bounds, a pinned matrix-product/factorization
context, and no changes to the floating-point environment during the route.
When gradual underflow is unavailable it exact-resolves the original stream,
bypassing floating certificates and curvature pruning. The formal contract
must be checked again after relevant dependency, compiler, target or size
changes. A proof of the enclosure theorem does not independently audit every
machine implementation of that contract.

The older [saddle-point f64 solver](../../crates/symplectic/src/kkt/saddle_point_solver.rs)
uses static beta, eigenvalue and residual thresholds. Its `q_error_bound` and
`q_corrected` fields retain a historical audit signal; they are not outward
certificates for the exact binary64-input QP. The
[projected solver](../../crates/symplectic/src/kkt/projection_solver.rs) likewise
computes numerical stationarity and treats threshold-discarded directions as
flat; only exact null directions preserve Q. Its projected residual estimate
has a narrower numerical-model meaning. These kernels were retained as distinct
controls rather than equated with the production certified route.

The [exact rational solver](../../crates/symplectic/src/kkt/rational_solver.rs)
uses exact rank and Fourier–Motzkin positivity search in a singular family.
Its success means a positive exact KKT witness; Q can still be nonpositive,
and consumers must check its exact sign before taking a reciprocal. Exact
input provenance and coverage remain outer obligations. In particular,
exact certification of an f64-retained candidate list is weaker than exact
resolution of a justified complete candidate stream.

## Pruning and the empty blocking set

The [pruning account](proofs/search-pruning-correctness.tex) distinguishes:
A0, all words; A1, nonempty facet intersection; A2, A1 plus the directed
nonnegative symplectic sign; A3, A2 plus strict slack on every blocking facet.
These refinements are at least as restrictive, and may coincide for particular
classes. For simple four-polytopes, including products of polygons, intersecting
facet pairs share a ridge, so A2 and A3 coincide by `cor:ridge-sufficiency`.
The production [transition graph](../../crates/symplectic/src/algorithms/facet_adjacency.rs)
uses A2. A2 is a necessary prefilter generally; it does not claim that every
retained edge is a realizable transition. Retaining extra words cannot increase
a feasible Q beyond the full HK maximum.

The single canonical transition lemma now lives in the
[KKT packet](proofs/ehz-kkt-system.tex), with both labels
`lem:numerical-transition-feasibility` and `lem:transition-feasibility`.
The formerly duplicated pruning proof had the same hypotheses and blocking set,
so it now refers to that lemma. Its criterion is nonnegative directed sign and
a point in `F_i intersection F_j` strictly slack for
`B={k != i,j: omega(a_i,a_k)<0 or omega(a_j,a_k)>0}`.

When B is empty, the strict inequalities are vacuous **but the facet
intersection must still be nonempty**. Take any positive slack parameter rather
than the minimum of an empty set. If every directional constraint derivative
is zero, choose any positive travel time rather than divide by zero; otherwise
choose time below slack divided by the maximum derivative magnitude. This repairs
the missing convention in the removed proof and the zero denominator in the
retained proof. The lemma remains `unverified`; independent agent review is
recorded in [the migration review](reviews/migration-2026-10-12.md).

Certified curvature pruning has a different purpose: it excludes a word as an
interior maximum, not as a positive stationary witness. The
[direct certificate](proofs/direct-kkt-certificate.tex) constructs a genuine
tangent direction with positive quadratic curvature and shows inheritance by
cyclic extension. Scalar minimization can discard those words. General action
windows above capacity must retain positive stationary words, so production
disables this scalar-only pruning for those requests.

## Output contracts and evidence owners

| Request or owner | Meaning and limit |
| --- | --- |
| `capacity_4d::capacity_from_dual_vertices` | Certified scalar for the exact dyadic polytope represented by binary64 input; outward general bounds or exact rational product value |
| `capacity_4d::qp_minimizers` | All tied discrete words in its named candidate family; general HK or product closure vertices, not every within-word or physical-orbit family |
| `capacity_4d::general_qp_action_window` | One exact stationary witness per retained general word within an exact capacity multiple; scalar curvature exclusions are disabled when the exact multiple is greater than one |
| `capacity_4d::solve_sigma_exact` | Requested exact word payload; no global coverage conclusion |
| [Product six-facet proof](../products/product-qp-six-facet-reduction.tex) and [product implementation](../../crates/symplectic/src/algorithms/capacity_4d/product.rs) | KKT-free scalar closure route; product mathematics is owned by the [products topic](../products/README.md) |
| [Development verification](development/verification/README.md) | Route expectation/comparison evidence, including selected experiment/production correspondence |
| [Candidate-filter audit](development/tools/candidate_filter_audit/README.md) and [KKT error search ledger](development/tools/kkt_error_audit/SEARCH-LEDGER.md) | Retained heuristic-filter and numerical-search limitations; historical findings do not override the selected production contract |
| [Production anchors](../../crates/symplectic/tests/production_capacity_anchors.rs) and [public API checks](../../crates/symplectic/tests/public_capacity_api.rs) | Small regression/API evidence; not a full retained-data regeneration or proof audit |

The scalar production input boundary includes at most 16 facets, dual and primal
infinity norms in `[1e-3,1e3]`, and at most 100,000 general candidate cycles.
Checked construction proves the input geometry contract; arbitrary mutation of
public geometry fields does not. Binary64 input certification is not a bound
on rounding distortion from a source algebraic polytope.

## Rust package and scientific ownership

The complete [QP development package](development/README.md) lives with this
scientific topic, including selected-route controls, audits, fixtures and
producer-specific numerical/performance evidence. Its Cargo package names and
internal layout are unchanged. Production `symplectic::kkt` and `capacity_4d`
remain physically within `crates/symplectic`: they share crate-private
interfaces, symplectic geometry helpers and public consumers across topics.
This is an explicit package boundary, not a claim that Rust imports prevent
source relocation. Extracting a separate kernel package would require interface
work; a source-only external module adapter would instead make the crate depend
on source outside its conventional tree. Neither was selected in this cleanup.
Topic ownership of the scientific account therefore coexists with conventional
crate ownership of importable production code; physical topic relocation of
that module remains deferred.

## Source migration and reproduction

Five former `formal/` files are compatibility inputs to their same-named
canonical sources in `proofs/`: `hk2017-qp-conventions.tex`,
`hk2017-qp-core.tex`, `hk2017-qp-precision.tex`, `ehz-kkt-system.tex`, and
`search-pruning-correctness.tex`. The precision source additionally inputs
`direct-kkt-certificate.tex` and `conditional-precision.tex`. Existing labels
and `formal/main.tex` input order remain valid; edit the topic sources, not the
stubs. The [retained foundations/correspondence index](evidence/README.md) routes
source-specific September checks and unapplied proposal patches. The
[proof/evidence migration review](reviews/migration-2026-10-12.md) records checks,
semantic changes and unresolved boundaries. The later complete
[development package relocation review](reviews/development-migration-2026-10-12.md)
records package wiring, retained data hashes and the focused compile/four-fixture
correspondence gate.

Build the combined proof-development document from its established directory:

```sh
(cd formal && latexmk -pdf -interaction=nonstopmode -halt-on-error main.tex)
```

That checks source paths, syntax and references, not mathematical truth.
No large certificate, performance sweep or retained dataset was regenerated
for this consolidation. The pre-existing Type A/Type B near-null LP rationale,
conditional beta proposal and broad minimizer-family completeness questions
remain explicit, rather than receiving a stronger status through relocation.
