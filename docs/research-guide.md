# Probing Viterbo's conjecture: a research guide for agents

This guide explains the main mathematical questions, how the results relate,
and which sources support them. It is an initial curated account of a larger
repository, prepared during the 10 October 2026 survey. It does not claim
exhaustive coverage or replace the complete notation, proofs, code contracts,
and experiment records linked below. The [inventory](../meta-topics/repository-organization/inventory/README.md)
records the broader survey and its reading gaps; [repository organization](../meta-topics/repository-organization/README.md)
maps source ownership.

The selected human manuscript is [thesis/main.tex](../thesis/main.tex).
Selection means that a passage is included in its build, not that its proof,
writing, or integrated PDF has received human acceptance. The broader research
includes useful results, rejected methods and open routes outside that
manuscript. [Project facts](project-facts.md) record Jörn's required content
areas and scope decisions; [thesis work](coordination/thesis-work.md) records
remaining work. This guide supplies orientation, not a new work programme.

## The question and the objects

The common question is how symplectic capacity compares with volume for convex
bodies. In four dimensions the repository uses

\[
\operatorname{sys}(K)=\frac{c_{\mathrm{EHZ}}(K)^2}
 {2\operatorname{vol}_4(K)}.
\]

Here EHZ denotes the Ekeland–Hofer–Zehnder capacity. For convex bodies it is
related to the least action of a closed characteristic on the boundary.
The relevant volume–capacity inequality would bound this ratio by one.
Haim–Kislev and Ostrover's pentagon product has ratio
\((3+\sqrt5)/5>1\): the counterexample and its capacity value are external
results. The project studies its local geometry, capacity algorithms, other
product families, and what finite searches reveal. These answer different
questions; they need not become one theorem or one narrative.

Most computations present a polytope as
\(K(a)=\{x\in\mathbb R^4:a_i\cdot x\le1\}\), with the origin inside and
all listed facets genuine. Each row combines the direction and height of a
supporting hyperplane; it is also a vertex of the polar body. Coordinates are
usually \((q_1,q_2,p_1,p_2)\), with
\(\omega_0((q,p),(q',p'))=q\cdot p'-p\cdot q'\).
A Lagrangian product \(A\times_L B\) places planar factors in the q- and
p-planes, where same-factor symplectic pairings vanish. This differs from a
product in two symplectic planes, whose capacity is the smaller factor area.
The distinction matters both to proofs and to implementation dispatch.

The [preliminaries](../thesis/chapters/02-preliminaries.tex) supply the full
convex, Hamiltonian and symplectic language. Follow their component files for
normalization rather than importing a sign convention from another paper.

The reusable [exact polar geometry packet](../topics/polytope-geometry/exact-polar-geometry.tex)
collects the ordinary geometry behind these representations: boundedness via
recession directions, exact vertex enumeration, and rational determinant signs.
A row supporting a facet is individually irredundant only when no duplicate row
is present. Rank-three triples supply a one-dimensional kernel test; rank-four
of all rows is a separate condition. The 11 October corrections have agent
review, with renewed human proof review still open.

## From boundary dynamics to finite optimization

A polytope boundary is nonsmooth. Its generalized Reeb trajectories may meet
several facets at once, so a computation cannot simply assume smooth ordinary
Reeb dynamics. The key finite-computation foundation is Haim–Kislev's
simple-minimizer theorem: some least-action orbit can be represented by a
finite word of distinct facet directions. The [generalized-orbit
chapter](../thesis/chapters/03-generalized-reeb-orbits-polytopes.tex) develops
that reduction using the dual action principle. Distinct directions refer to
the finite representation, not a classification of every minimizing orbit.

For a word \(\sigma\), its nonnegative weights satisfy closure and normalization:
\(\sum\beta_k a_{\sigma_k}=0\) and \(\sum\beta_k=1\). The ordered pairings
form a quadratic objective
\(Q_\sigma(\beta)=\sum_{j<k}\beta_j\beta_k
\omega_0(a_{\sigma_j},a_{\sigma_k})\).
The Haim–Kislev formula is
\(c_{\mathrm{EHZ}}=1/(2Q_{\max})\), where the maximum includes all required
words and their feasible weights. The code and bibliography often call this
HK2017; manuscript labels also use HK2019. Search both names.

The [convention source](../topics/capacity-and-qp/proofs/hk2017-qp-conventions.tex) fixes the
word reversal and factor of two. The [QP core](../topics/capacity-and-qp/proofs/hk2017-qp-core.tex)
now makes the full-permutation/subset-word bridge explicit: zero weights may
be deleted or inserted, and closure makes cyclic rotation preserve the
objective. That bridge has separate agent review, not renewed human approval.

This explains two recurrent evidence distinctions. A positive feasible
choice gives an **upper bound** on capacity. Equality requires proving global
coverage and comparing the candidates. Likewise, a feasible finite dual curve
need not already sit on the polytope boundary; a global minimizer has the
required reconstruction, while other candidates need an additional placement
check.

For one fixed support, stationarity is a linear KKT system despite the quadratic
objective. Boundary maxima require shorter supports obtained by deleting zero
weights. Singular systems need feasibility analysis; singularity alone does
not invalidate the capacity formula. The [QP
chapter](../thesis/chapters/04-quadratic-program.tex) explains this computation
and its proof. [Conventions](../topics/capacity-and-qp/proofs/hk2017-qp-conventions.tex),
[core](../topics/capacity-and-qp/proofs/hk2017-qp-core.tex) and
[precision](../topics/capacity-and-qp/proofs/hk2017-qp-precision.tex) provide deeper formal routes.

Products admit two useful reductions. A billiard argument bounds the number
of alternating factor blocks. Separately, the
[six-facet reduction](../topics/products/product-qp-six-facet-reduction.tex) uses
linearity in each factor's closure weights: some global maximizer uses closure
vertices with at most three facets per planar factor and half the total weight
in each factor. This product-specific result survives degeneracy. It does not
assert that every minimizing orbit has six facets. The distinction explains
why a sparse capacity evaluator and an orbit/derivative evaluator can return
different witness families while agreeing on the value.

## Following trajectories through a flow graph

The flow-graph method computes geometry along facet passages instead of
optimizing dwell weights first. Constant velocity on a facet sends an incoming
two-dimensional section to an outgoing section by an affine map. A tube retains
its feasible starting polygon, endpoint map and action function. Composition
intersects the feasible domains, composes the maps and adds actions. Around a
cyclic word, a fixed point of the return map with positive segment times is a
closed orbit.

The [selected chapter](../thesis/chapters/05-flow-graph.tex) proves correctness
of idealized exhaustive exact real-arithmetic search under three explicit
regularity assumptions: intersecting facet normals have nonzero symplectic
pairing; every sublist of at most four rows is independent; and surviving
return maps have nonsingular fixed-point equations. A second result proves
these conditions generically within each nonempty open presentation chamber.
This is chamber-relative algebraic genericity, not the conjectural CH2021
claim about absence of Type 2 orbits below an action threshold. The
[CH2021 comparison](../thesis/chapters/05-flow-graph-ch2021-background-comparison.tex)
explains the difference.

The assumptions exclude ordinary Lagrangian polygon products, including HKO,
because their same-factor rows are dependent. Rational Rust tube search has
bounded agreement and reconstruction tests on selected small inputs; neither
exact arithmetic nor those tests proves implementation correspondence on every
input. The optimized flow-graph code has its own contract. Begin with the
[implementation README](../crates/symplectic/src/algorithms/flow_graph/README.md)
when changing it, and the [formal real-algorithm
route](../topics/flow-graphs/flow-graph-real-algorithm.tex) when studying correctness. The
[worked tube example](../experiments/dev-flow-graph/visualize-tube/README.md)
connects the geometry to exact retained data and figures.

## Local variation: upper sections, branch changes and certificates

Moving the facet rows allows general perturbations, including ones that destroy
product structure. Volume differentiates through facet centroids and areas in
a stable simple-combinatorics chart. Capacity is more delicate because its
optimizing word and weights can change.

The [variation chapter](../thesis/chapters/06-variation.tex) distinguishes a
smooth optimizing branch from a smooth **feasible section**. A section chooses
weights satisfying closure and positivity nearby, without requiring them to
remain optimal. Its positive Q produces a smooth ratio upper bound
\(U=1/(8VQ^2)\). If it is optimal at the base, this bound touches the true
ratio there. Several touching upper bounds can therefore prove maximality
without describing every nearby optimizer.

This is the HKO local proof's central method. Ten facet rows give forty
coordinates. Translation, dilation and linear symplectic maps supply fifteen
symmetry directions. Twenty-six feasible upper sections have derivative rank
twenty-five, annihilate those symmetries and admit a strictly positive relation
summing to zero. On the transverse quotient, that geometry forces at least one
upper bound to decrease in every nonzero direction. The written uniform-slope
and local-slice argument turns the finite derivative facts into a Hausdorff
neighborhood theorem: HKO locally maximizes the ratio among exactly ten-facet
four-polytopes, with equality precisely along its symmetry images.

[Chapter 07 and its fragments](../thesis/chapters/07-hko.tex) contain the
statement and implication. The [certificate
packet](../topics/variation-and-local-maxima/hko-local-maximum/theorem/README.md) explains the exact
Sage predicates and Rust-generated witness choices; the
[formal upper-section note](../formal/hko-feasible-section-upper-branches.tex)
is another proof route. The finite certificate is essential but does not by
itself prove the neighborhood implication. Older smooth-envelope sketches and
numerical neighborhood samples do not replace this route. The theorem fixes
ten facets; it does not settle perturbations that create extra facets.

The selected chapter also includes a separate rational hexagon–quadrilateral
product with ratio one and a non-strict ten-facet local maximum. Its equality
family has two parameters after quotienting by symmetries. Here first-order
sections leave an additional transverse direction, controlled by an exact
negative second derivative of the upper bounds. The
[non-HKO packet](../topics/variation-and-local-maxima/hko-local-maximum/non-hko/README.md) separates
exact finite checks, rational continuation along the family, and the analytic
uniform-neighborhood argument. Independent implementation audits are recorded;
this guide asserts no human acceptance for that candidate.

For search, smoothly continued KKT branches give useful gradients. Ties require
coverage and gap hypotheses before an envelope derivative follows. Finite
branch-window predictions add conditioning and step-size issues. A complete
one-sided first-order theory at arbitrary nongeneric polytopes remains open;
see [the scope note](../formal/sys-first-order-local-behavior.md).

## What the pentagon family teaches beyond a computation

For two regular circumradius-one pentagons with relative angle \(\theta\),
put \(d(\theta)=\min_k|\theta-k\pi/5|\). The
[rotation chapter](../thesis/chapters/09-rotated-regular-polygons.tex) proves
\(c=(1+\cos(\pi/5))^2\sec d(\theta)\), hence the sharp ratio
\((3+\sqrt5)/5\) at the HKO relative positions.

The current proof avoids exhaustive symbolic classification. Closure separates
by factor, so feasible weights are independent of angle. Each fixed choice
has value \(A\cos\theta+B\sin\theta\). Nonnegative endpoint interpolation
bounds every choice by the known HKO endpoint values; an explicit feasible
word attains that bound. The earlier
[3,340-word Sage classification](../topics/products/regular-products/pentagon-rotation-formula-proof/README.md)
remains valuable retained evidence and proof history, but is no longer a
dependency of the selected analytic proof.

Independent linear deformations ask a larger question: can shears or stretches
of the two pentagons beat HKO? For invertible \(G,H\), the
[affine chapter](../thesis/chapters/10-affine-pentagons.tex) expresses capacity
using only twenty-five absolute pairings:
\(c(GP\times_L HP)=(1+h)^2/m(G^{-1}H^{-T})\), where
\(h=\cos(\pi/5)\) and \(m(M)=\max_{i,j}|n_i^TMn_j|\).
The proof reduces to sparse closure weights and containment of matrix
coefficient sets, then an area bound gives the same sharp systolic maximum.
It is global within independently affinely regular factors. HKO local
maximality covers nearby nonproduct perturbations; neither theorem subsumes
the other. The [maintained research
note](../topics/products/pentagon-affine-products/README.md) contains full proofs,
equality cases, source checks and further angular results outside the selected
chapter. Arbitrary pentagons are not covered by the affine theorem.

A different extension fixes one regular pentagon Q and permits **any centrally
symmetric convex planar partner** K. The
[symmetric-partner lemma](../topics/products/pentagon-arbitrary-partner/symmetric-partner-lemma.md)
identifies capacity with a difference-body inclusion scale and gives the sharp
ratio \((10+6\sqrt5)/25<1\), attained by a specified regular decagon partner.
Its method is a finite normal-triangle covering criterion plus containment,
not the affine matrix argument. Jörn selected inclusion, but this result is
not presently a theorem passage in the selected chapters. The owner records
an independently checked agent-derived proof, without human acceptance or a
novelty determination. Global HKO optimality for arbitrary asymmetric partners
remains unresolved; local-cover and cut-lifting research notes are not that
global theorem.

An [agent-checked extension](../topics/products/pentagon-arbitrary-partner/odd-regular-symmetric-partner.md)
covers every odd regular factor with at least five sides and a centrally
symmetric partner. This does not solve the general capacity formula for two
nonsymmetric odd regular factors: outside the proved pentagon–pentagon,
pentagon–heptagon and heptagon–heptagon cases, that remains a conjectural route
in the [affine research note](../topics/products/pentagon-affine-products/research.tex).
The broader symmetric-partner extension is retained research, not a new claim
of human acceptance or manuscript inclusion.

The separate [product-position chapter](../thesis/chapters/11-product-position.tex)
shows that a product initially in symplectically orthogonal factor planes
admits a small volume-preserving rigid rotation that increases capacity.
This rotates the factor planes in four-space; it is a different question from
relative planar rotation inside a Lagrangian product.

## What finite searches establish

The empirical work asks both whether ordinary methods find new large-ratio
bodies and whether geometric measurements explain variation. Its source flow
is [polytope datasets](../topics/empirical-studies/polytope-datasets/README.md) →
[invariant tables](../topics/empirical-studies/polytope-invariant-table/README.md) →
[method packets](../topics/empirical-studies/sys-datascience/methods/README.md).
The retained random/product sample has 14,336 bodies and no recorded numerical
ratio above one. That is a bounded negative observation under named sampling
and evaluator contracts, not evidence for the disproved unrestricted inequality.

The [selected data-science chapter](../thesis/chapters/08-data-science.tex)
finds a strong broad inverse association with volume-normalized total absolute
symplectic ridge area. It weakens in selected high-ratio or low-ridge subsets.
Prediction, enrichment and controlled geometric examples address different
questions; association does not identify a cause or an improving direction.
The [current question map](../topics/empirical-studies/sys-datascience/coordination/current-question-map.md)
routes competing explanations and packet-specific outcomes. Jörn's overall
finish-versus-report scope decision for incomplete data science remains open.

Likewise, the [matched optimizer comparison](../experiments/dev-gradient-ascent/optimizer-comparison/README.md)
supports finite-budget rankings of named policies on 64 starts using a
historical heuristic numerical objective. All 448 terminal recorded values
remain below one. Further endpoint moves defeat local-maximality readings of
plateaus. Neither that ranking nor an iteration cap establishes convergence,
a certified threshold result, or superiority outside its population and budget.

## Arithmetic, reproducibility and deeper entry points

Exactness, candidate completeness and implementation correspondence are separate
requirements. Current production QP capacity treats binary64 coordinates as
exact dyadic input, uses outward bounds or exact rational fallback, and returns
capacity intervals or exact product capacities within explicit applicability
limits. It does not recover an unknown algebraic body from rounded coordinates.
A certified capacity does not certify a floating-point volume or the resulting
ratio. Historical experiments retain their historical evaluator contracts.

Use the [calculation map](capacity-calculation-map.md) to choose a route and the
[numerics chapter](../thesis/chapters/13-numerics.tex) for its mathematical
explanation. [Testing guidance](algorithm-testing.md) distinguishes unit tests,
cross-route falsifiers and theorem-specific verification. Rust reusable layers
are [algebraic numbers](../crates/algebraic-numbers/README.md),
[Euclidean polytopes](../crates/euclidean-polytopes/README.md) and
[symplectic computations](../crates/symplectic/README.md); their APIs do not
own empirical interpretation.

| Question to pursue | First deeper source |
| --- | --- |
| Why does finite capacity computation represent boundary dynamics? | [Generalized orbits](../thesis/chapters/03-generalized-reeb-orbits-polytopes.tex), then [QP](../thesis/chapters/04-quadratic-program.tex) |
| Which local maximum is proved, with what certificate? | [HKO theorem](../thesis/chapters/07-hko.tex) and its two linked packets above |
| Why is product capacity cheaper than full enumeration? | [Six-facet reduction](../topics/products/product-qp-six-facet-reduction.tex) |
| What remains open for arbitrary pentagon partners? | [Partner research](../topics/products/pentagon-arbitrary-partner/README.md) |
| Which numerical values and figures are reproducible? | [Code/data chapter](../thesis/chapters/14-code-data.tex), [artifact policy](artifacts.md), then the producing packet |
| Where is related work beyond these main routes? | [Formal findings](../formal/FINDINGS.md), [experiment findings](../experiments/FINDINGS.md), and the ongoing [inventory](../meta-topics/repository-organization/inventory/README.md) |

A plain checkout does not materialize every empirical input. New reevaluations
and recovered geometry do not retrospectively supply missing execution
receipts. Read local commands before running producers that overwrite evidence;
a thesis build only checks document integration. The guide has not surveyed
every auxiliary proof, visualization, search family, performance packet,
AI-use artifact or historical review. Those omissions are coverage gaps in this
initial account, not absence claims or judgments that the material is expendable.
