# Broad discovery toolbox

2026-09-18. Research synthesis by empirical root with independent assessments
from `broad_discovery_agenda`, `retained_mechanisms`,
`pentagon_partner_research`, and `toolbox_brainstorm`. This pass gathers and
judges tools; it does not report new empirical discoveries or launch producers.
The [shared evidence brief](observations-for-toolbox.md) distinguishes existing
results from unexplored possibilities.

## Main judgment

The strongest expansion combines **new measurement families, structured
representations and different discovery objectives**. Running another model on
the old ridge/count columns would leave the main information gap intact.
Conversely, collecting many new columns without investigating identities,
relations, equality cases, geometric distinctions and controlled responses
would leave their mathematical value largely unexplored.

Exploration does not need a specific conjecture in advance. A reasonable first
payoff is an interpretable distinction or candidate relation that changes what
we measure or construct next. A result confined to one convenient sampling law,
a prettier embedding or a faster evaluator is not sufficient by itself.

## Measurements: what information could be added?

Most inexpensive geometric quantities below are invariant under **linear**
symplectic maps, not arbitrary symplectomorphisms. Every feature needs its own
translation, scaling, relabeling and transformation contract. Representative-
dependent geometry remains useful when labeled honestly; the old table's
invariant-only engineering restriction does not define this research scope.

| Family | Information beyond current columns | Inputs and effort | Assessment |
|---|---|---|---|
| Centered dual pairing matrix and its extremum | Interactions between supporting facets, including nonadjacent ones | Facets and an explicit affine-equivariant center; quadratic in facet count | First tier. Keep the matrix and extremal witnesses, not only new quantiles. The symmetric-body c_J comparison has additional hypotheses. |
| Centered primal pairing matrix | Global arrangement of vertices, beyond their covariance and ridge-area multiset | Vertices; quadratic in vertex count; permutation handling for comparisons | First tier. Spectra alone are redundant with vertex covariance; arrangement and distributions need separate treatment. |
| Uniform-volume covariance and higher moments | Bulk mass distribution, with continuity under small geometric changes | Verified triangulation/integration, or volume-weighted simplex sampling | First tier. Covariance extends an existing family; higher moments may add genuinely different information. Sampling error must accompany tail estimates. |
| Symplectic polar geometry | Dual bulk versus dual extremes; primal/dual mismatch | Centered facet covectors give polar vertices; hull volume and moments | First tier. Volume product is affine invariant, a useful control rather than specifically symplectic information. |
| Incidence decorated with geometric data | How comparable ridge areas or pairings are arranged | Retained faces/incidence with areas or pairings | First tier on small retained examples. Compare against counts and histograms before graph learning. The old negative incidence test does not exhaust this representation. |
| John/Löwner ellipsoid spectra and contacts | Supporting extremal directions versus bulk inertia | New small convex optimization and checked containment | Second tier. Affine-equivariance gives linear-Sp invariance of Williamson spectra; optimization and degeneracy require care. |
| Optimized symplectic projection geometry | Linear positioning and projection constraints | Projected hulls plus noncompact frame optimization | Second tier. Sampled/local optima are not exact invariant values or certified optimal bounds. |
| Actions, gaps, rotation/index and orbit geometry | Dynamics beyond minimum action | Reliable orbit reconstruction and completeness/index contracts | Selective, scientifically distinct. Existing QP candidates are not an action spectrum; facet-word multiplicities are not orbit counts. |
| Higher capacity sequences | Multiple intrinsic symplectic scales | Established formulas on suitable structured domains; new machinery for general polytopes | Longer-term/control populations. The kth shortest orbit is not automatically the kth capacity. |
| Support/width fields, asymmetry, mixed volumes and ordinary geometry | Shape directions and responses not captured by invariant scalars | Explicit geometry, controlled operations, sometimes integration | Include deliberately. A quantity need not itself be symplectic invariant to reveal a mathematical relation. |

For a centered body with normalized polar vertices a_i, the dual extremum is
max_ij |omega(a_i,a_j)|. Its reciprocal has length-squared homogeneity. The
centrally symmetric c_J convention and comparison are given in
[Berezovik](https://arxiv.org/html/2310.14998); a nonsymmetric extension of the
numerical expression does not automatically inherit the theorem. Ordinary
cylindrical capacity would duplicate EHZ for convex bodies in R4 by
[Abbondandolo–Edtmair–Kang, Corollary 1](https://arxiv.org/html/2412.01777v1).
Linearized cylindrical capacity is a different quantity.

The dynamical route has an actual computational precedent in
[Chaidez–Hutchings](https://arxiv.org/pdf/2008.10111v2), with important boundary
orbit and rotation conditions. Higher capacities require their own machinery;
see [Hutchings](https://arxiv.org/abs/1005.2260) for ECH capacities. These are
source leads with explicit scope, not interfaces implemented in this desk.

### Mathematical redundancy checks before collecting more columns

The following are direct linear-algebra deductions, not novelty claims.
Write omega(x,y)=x^T J y and center all vertices at an affine-equivariant point g.

* W_ij=omega(v_i-g,v_j-g)/sqrt(vol K) is scale-free and linear-Sp invariant.
  With consistent labels, its unnormalized matrix determines any spanning
  configuration up to a linear symplectic map: equal pairing matrices give the
  same linear relations, and the induced map preserves every pairing. An
  unlabeled comparison still needs permutations. This is a rich representation,
  not a claim that an easy distance between such matrices solves shape comparison.
* For the unnormalized vertex matrix X, W=X^T J X has the same nonzero eigenvalues
  as JXX^T. Its spectrum therefore contains only the two Williamson frequencies
  of the corresponding vertex second moment, up to the vertex-count factor.
  Adding matrix eigenvalues would not create a new feature family.
* For independent centered uniform-volume samples X,Y with covariance C,
  E[omega(X,Y)^2]=-tr(CJCJ)=2(nu_1^2+nu_2^2). All odd moments vanish by swapping
  X and Y. Fourth moments and distribution tails can probe information beyond C;
  whether they do so usefully remains to be tested. These are different from
  moments of the finite vertex measure.
* In K times the regular pentagon, continuous covariance is
  diag(C_K,sigma^2 I). Its Williamson frequencies are
  sigma sqrt(lambda_i(C_K)); their ratio is just planar inertia anisotropy.
  It is useful geometry but not an additional independent measurement there.
* Primal–polar dot-product quantities and the polar volume product can be affine
  invariants. Rewriting them with J does not make them specifically sensitive to
  symplectic structure. They are valuable comparison measurements nonetheless.

## Discovery methods: different mathematical outputs

| Method | Output worth inspecting | Main false success | First use and branch rule |
|---|---|---|---|
| Empirical inequality frontier | A few complementary bounds with improvement and near-equality witnesses | Fitting the sampled envelope or rediscovering an identity | Use small dimensionally valid expressions across distinct families; seek deliberate violations when an interpretable candidate appears. Stop grammar expansion if complexity replaces explanation. |
| Explicit and implicit algebraic discovery | Short formulas, factorizations, relations without a designated target | Spurious near-null polynomials on a narrow generator | Constrain homogeneity; inspect residual versus complexity and leave-family-out behavior. Known identities are positive controls, not discoveries. |
| Conditional relations/regimes | Different laws separated by a mathematical condition | Partitioning until every source fits | Start with geometry and residuals, then examine boundary cases along paths. Upgrade when a split becomes a natural predicate; stop source memorization. |
| Matched descriptions and descriptor fibers | A reproducible degree of freedom invisible to one representation | Approximate nearest neighbors called an exact collision | Find contrasts, then construct controlled counterparts. Seek a separating family or missing feature rather than a narrative about two examples. |
| Structured matrix/graph/field analysis | Interpretable arrangements or modes missing from scalar marginals | Embeddings driven by size, labels or arbitrary coordinates | Compare simple graph/field summaries and linear modes first; nonlinear kernels or graph models are options if those miss visible structure. |
| Controlled response analysis | Joint changes, transition geometry, piecewise laws | Forced symmetry/concavity sold as a pattern | Follow rotation, truncation, polarity and Minkowski operations; inspect entire curves and geometric changes, including capacity-free responses. |
| Adaptive acquisition | New cases that change a relation, hypothesis or representation | Optimizing numerical error or repeatedly finding one degeneracy | Begin with coverage; later mix model disagreement, violation search and independent controls. Simplify exceptional constructions before scaling up. |

Inequality-frontier filtering is motivated by the Dalmatian approach:
retain a bound that survives the current examples and improves a retained bound
somewhere. This is a finite-data heuristic, not a proof or independence guarantee
([TxGraffiti](https://arxiv.org/html/2409.19379v1)).
Symmetry/separability-informed symbolic search has precedent in
[AI Feynman](https://arxiv.org/abs/1905.11481v2); its formula-rediscovery benchmark
does not imply that capacity has a short formula in our columns. Interpreting
representation effects, rather than only prediction scores, is illustrated by
[polytope dimension learning](https://arxiv.org/html/2207.07717).
Candidate disagreement can guide later acquisition
([Medina–White](https://arxiv.org/abs/2305.10379v3)); we need actual competing
expressions before building that machinery.

These methods work together without six separate software projects. A shared
feature table with units and transformation metadata, raw structured objects,
and a small record of candidate relations/witnesses are sufficient starting
interfaces. Model fitting is a discovery aid. A neural representation or large
symbolic search is not automatically more informative than a simple comparison.

## Combining tools in the active populations

**Pentagon partners:** the ten fits add directional bottlenecks beyond their
minimum. Dual weights describe local support sensitivity; one selected optimal
solution may be nonunique. Three active dual weights generally reflect LP
dimension, not orbit complexity. Rotation-response curves have forced pi/5
periodicity, but arbitrary partners need not have even response curves. The
existing two-bounce ceiling adds difference-body geometry. Pair these with
support fields, continuous moments and polar geometry, then use relation mining,
matched descriptions and response analysis. Contact labels are not billiard
orbit labels. This remains one population, not the whole campaign.

**Generic 4D bodies:** pair primal/dual matrices with bulk geometry and decorated
incidence. Compare descriptions without making sys the default response. The
fourteen retained geometries can support representation checks but are too small
and selected to establish general statistical relations. Use a deliberately
heterogeneous collection when acquisition becomes necessary; retain geometries
and family lineage rather than only scalar rows.

**Structured alternatives:** self-polar constructions offer a distinct class
with known-capacity examples and nontrivial volume questions; suitable toric
domains offer higher-capacity controls beyond polytope representations. Neither
means all self-polar bodies have the same capacity or arbitrary polytopes are
toric. These are candidates for coverage, not automatic new campaigns. The
[literature map](../../open-thesis-literature/README.md)
is in a sibling worktree.

## Independent brainstorm: populations organized by operations

Two additional ideas survived an independent pass that was explicitly asked
to challenge the feature-list framing.

**Zonotopes and segment interactions.** Let Z=sum_i [-v_i,v_i], with spanning
generators in R4, and W_ij=omega(v_i,v_j). These are centrally symmetric bodies
with unrestricted generator count, usually outside the old Lagrangian product
representation. Segment addition/removal gives explicit controlled operations.
In four dimensions,

    vol(Z) = 16 sum_{i<j<k<l} |W_ij W_kl - W_ik W_jl + W_il W_jk|.

This is the determinant/Pfaffian volume identity, not a discovery. It makes
pairing arrangements and cancellation interpretable. Compare responses to adding
one or two segments, using volume and pairing structure before requiring capacity.
Potential output is a relation or separating construction about symplectic
interaction versus volume response. Treat a generating list as construction
data: splitting one segment into collinear summands changes the matrix without
changing the body. Canonical merging or representation controls are necessary.
Facet growth is the principal capacity-evaluation risk. Recent primary leads
already give adverse examples to volume log-submodularity
([first paper](https://arxiv.org/abs/2608.07702),
[second paper](https://arxiv.org/abs/2608.12681)); those source constructions need
inspection before reuse, and their known volume result is not ours to rediscover.

**From symplectic to Lagrangian product position.** Fix arbitrary planar factors
A,B and preserve their Euclidean Cartesian product while changing the ambient
orthogonal complex structure. One explicit path is

    J_theta = [[cos(theta) J2, sin(theta) I],
               [-sin(theta) I, -cos(theta) J2]],   J_theta^2 = -I.

Equivalently, rotate the body in a fixed symplectic space. At the endpoints the
factor planes are symplectic and Lagrangian respectively. Distances, volume and
face lattice remain fixed throughout. The expected endpoint capacity descriptions
are minimum factor area and Minkowski billiards; their convention contract must
be checked before implementation. Relative phase adds further parameters, so
theta is not the whole orientation space for arbitrary factors.

Pairing fields, branch geometry and response analysis could discover which factor
properties control transition types, or refute natural endpoint-based principles
on a broad class. One regular-polygon plot would not suffice. This differs from
the old sampling-budget comparison, but capacity away from endpoints remains an
expensive dependency. Retain as a scientifically organized construction, not a
reason for an immediate large orientation sweep.

Self-polar constructions remain a third, distinct population. A more surprising
later control would use small nonlinear Hamiltonian deformations of smooth
strictly convex bodies: true capacity and volume stay fixed while merely linear
symplectic descriptors can change. Maintaining convexity and controlling
polyhedral approximation make that a costly representation audit, not a first
dataset. A bare catalog of operations was downgraded: without informative
measurements it would mostly reproduce known monotonicity.

## Recommended implementation order, not a producer launch

1. Define a compact feature/representation contract and implement the inexpensive
   pairing/polar measurements alongside the active partner geometry interface.
   Add continuous moments where integration is already accessible; check known
   identities and symmetries on explicit bodies. This is new information, not a
   full historical-table rebuild.
2. Make both an inequality/algebraic pass and a cross-representation/response
   pass possible on the same small heterogeneous data. Keep capacity-free
   questions and generic 4D bodies visible. No exact conjecture is a prerequisite.
3. Let the first interpreted observations choose selective additions: canonical
   ellipsoids for contact-versus-bulk questions, dynamics for genuine orbit
   questions, new families or counterexample constructions for missing coverage.

Planning estimate: a first useful combined representation/discovery session is
hours, not the milliseconds of the partner evaluator; allow roughly half a day
for a minimal generic-plus-product pass once geometry inputs are selected.
The main uncertainty is interpretation, not model runtime. Expect several
redundancies and generator artifacts, plausibly a few useful contrasts, and no
assurance of a new theorem. Do not implement every table row before inspecting
anything. End the first session with interpreted findings and explicit missing
information; extend only the parts that changed the scientific picture.

No additional taste questionnaire is needed to complete this toolbox pass.
Computation speed alone and narrowly parameterized extra cases are not assigned
independent scientific value. Existing production restrictions remain in force;
the gathering request does not silently authorize a large data rebuild.
