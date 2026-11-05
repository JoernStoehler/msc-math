# HKO2024: source and open-question map

Inventory date: 16 October 2026. This map records the research state found at
the start of this run in `/workspaces/msc-math-hko-2026-10-16`; new results of
this run have their own notes in this directory. Its purpose is to prevent
rediscovery of solved questions and to distinguish genuinely open quantifiers.

The normalization is
`sys(K)=c_EHZ(K)^2/(2 Vol_4(K))`. Write `H` for the Haim–Kislev–Ostrover
regular-pentagon Lagrangian product. Then `sys(H)=(3+sqrt(5))/5`.
The counterexample and this benchmark are external results, not new results
of this project or run.

## Source coverage

Read: repository `README.md`, `AGENTS.md`, `CLAUDE.md`,
`docs/research-guide.md`, all seven topic-account READMEs, the local-maxima
certificate and F≤11 acceptance accounts, the independent-affine and
arbitrary-partner accounts, and the linked recent discovery synthesis and
selected research notes below. Searches used HKO, Haim–Kislev/Ostrover,
pentagon, local maximum, equality, rigidity, symmetry, open and conjecture
across topic, formal and experiment source accounts. This is a semantic
inventory of research-bearing source packets, not a listing of every fixture,
generated figure, copied bibliography or benchmark mention.

The cached primary HKO source is
[`counterexample.tex`](../../literature/sources/hko2024/counterexample.tex).
Its explicit discussion questions and dynamical remarks were inspected.
The [literature account](../../literature/README.md) warns that the cached
snapshot has incomplete upstream-revision metadata. The dated
[extrema search](../../literature/open-thesis/extrema.md) and
[products search](../../literature/open-thesis/products.md) record primary
editions and reading depth. Their negative novelty searches are not publication
priority certificates. A bounded live check on 16 October reopened the
[HKO arXiv record](https://arxiv.org/abs/2405.16513) (v3, 20 November 2025)
and the [BMP paper](https://arxiv.org/html/2603.12495v2) (v2, 19 August 2026).
BMP Theorem 1.2 proves the product inequality for any quadrilateral factor or
affinely regular hexagon factor and an arbitrary convex planar partner.
Its §6.1 still explicitly leaves regular-pentagon cover optimality unproved;
Question 1.3 asks for the sharp planar-product constant. This check is not
a forward-citation survey or a claim that no later resolution exists.

## What the original paper asks

The cached HKO paper, subsection “Discussion and Open Questions”, records:

1. Does Viterbo's inequality hold for centrally symmetric convex domains?
2. What is the maximal convex-body systolic ratio in each dimension, which
   bodies maximize it, and what dynamics do maximizers have?
3. How many distinct normalized capacities exist on convex domains, with
   “distinct” interpreted in the paper's specified functional sense?

The same source discusses comparison with dynamically convex domains and
their maximal systolic constant. Its final dynamics section observes a dense
open family of closed trajectories for the HKO product and related regular
polygon products, and motivates further nonsmooth Besse-type study. This is
not an assertion that all generalized HKO trajectories are closed or that HKO
is generalized Zoll.

The paper already explains that smooth convex counterexamples follow by
approximation. Thus “must a counterexample be a polytope?” was answered before
this run. Its ratio-one/non-ball remark also rules out treating ratio one as
a characterization of symplectic balls.

## Questions already answered in this repository

| Question | Supported answer | Scope and source |
| --- | --- | --- |
| Is HKO a local maximum among ten-facet bodies? | Yes; equality locally consists of translation, positive dilation and linear symplectic images. | General four-dimensional ten-facet perturbations, including loss of product structure. [Topic account](../README.md), [feasible-section proof](../../../formal/hko-feasible-section-upper-branches.tex), [exact theorem packet](../hko-local-maximum/theorem/README.md). |
| Does adding one facet destroy that theorem? | No: an existential Hausdorff neighborhood has the same upper bound for all bodies with at most eleven genuine facets. | Arbitrary moving old normals, nonlinear approach sequences and coupled facet-birth scales. [Accepted F≤11 theorem](../hko-local-maximum/f11-acceptance-20261005.md). Eleven-facet equality and an explicit radius are not supplied. |
| Can rotating the two regular pentagons beat HKO? | No; the complete secant capacity / secant-squared ratio profile is proved. | Relative planar rotation in a Lagrangian product. [Rotation account](../../products/regular-products/pentagon-rotation-formula-proof/README.md), [affine account](../../products/pentagon-affine-products/README.md). The current analytic proof replaces exhaustive symbolic enumeration as the reader-facing dependency. |
| Can independent affine deformations of the regular pentagons beat HKO? | No; the sharp global bound and equality class are proved. | Both factors remain affinely regular. For `M=G^-1 H^-T`, capacity is `(1+cos(pi/5))²/max_ij |n_i^T M n_j|`. Equality requires `M=t R_(pi/10) U` with the stated decagon-dihedral `U`. [Maintained proof](../../products/pentagon-affine-products/research.tex). |
| Can a centrally symmetric partner of one fixed regular pentagon violate the inequality? | No; a sharper constant `(10+6sqrt(5))/25<1` is proved. | Arbitrary centrally symmetric planar convex partners, including smooth partners. The unique sharp partner is a prescribed regular decagon up to translation/dilation. [Proof](../../products/pentagon-arbitrary-partner/symmetric-partner-lemma.md). This does not solve the centrally symmetric four-body question above. |
| Does that symmetric-partner theorem extend to odd regular factors? | Yes, for every odd `n>=5`, with sharp constants below one. | [Odd-regular proof](../../products/pentagon-arbitrary-partner/odd-regular-symmetric-partner.md), and [local persistence](../../products/pentagon-arbitrary-partner/local-calibration-stability.md) for labelled nonregular factors near them. This is not the two-asymmetric-factor problem. |
| Must a partner on a regular five-normal fan be regular to maximize the ratio? | Yes, in that class: arbitrary supports and all angles have the sharp HKO bound, with regular HKO equality only. | Edge-loss boundaries included. [Equiangular theorem](../../products/pentagon-arbitrary-partner/equiangular-partner-global-20261005.md). |
| Does the preceding statement survive fixed-axis distortion? | Yes for normals `diag(x,1) R_theta n_i`, all `x>0`, every angle, arbitrary supports. | Equality requires undistorted regular HKO. The distortion axes are fixed relative to the regular factor. [Orientation-wide theorem](../../products/pentagon-arbitrary-partner/orientation-wide-global-20261005.md). |
| Is HKO the only ten-facet local maximum? | No at the level of all ratios: a rational hexagon–quadrilateral product `P*` is a different local maximum of ratio one. | It has 24 vertices versus HKO's 25 and a two-parameter equality family modulo symmetries. [Result](../hko-local-maximum/non-hko/RESULT.md), [proof](../hko-local-maximum/non-hko/PROOF.md). It gives no new violation. |
| Can nonlinear cotangent lifts readily create inequivalent convex equality bodies from HKO? | The natural cotangent ansatz is obstructed: with at least three unoriented momentum-normal directions, convexity forces the base diffeomorphism to be affine. | [Cotangent-rigidity proof](../../empirical-studies/discovery-design/night-20261005/extremality-discovery/cotangent-rigidity.md). Pure fiber translation is similarly forced affine. General mixed Hamiltonian maps remain outside this obstruction. |

“Accepted” above follows each source's project/agent-review status. It does not
assert human refereeing, proof-assistant formalization or novelty. The F≤11
result is a retained research theorem; the selected thesis still states F=10.
No thesis files are edited by this inventory.

## Genuinely open questions and their exact missing scope

| Open problem at run start | What is already available | Missing implication or quantifier |
| --- | --- | --- |
| HKO local maximality among arbitrary convex four-bodies | Sharp F=10 and accepted F≤11 neighborhoods. | Two or more facet births, unbounded facet counts, smooth/nonpolyhedral perturbations. Neither pointwise cuts nor finite sampled cuts settle a uniform Hausdorff neighborhood. |
| Eleven-facet local equality classification | F≤11 inequality. | Whether every sufficiently nearby equality body is an affine-symplectic HKO image. The accepted account explicitly leaves this unclassified. |
| Quantified local stability | A linear ten-facet transverse deficit and existential F≤11 neighborhood. | A useful explicit Hausdorff radius, uniform deficit in a stated geometric distance, and its equality-normalization constants. |
| Global maximum among arbitrary four-dimensional convex bodies | HKO benchmark and special-family bounds. | A universal upper bound equal to the HKO value or a larger-ratio example; classification of global equality. |
| Global maximum among arbitrary pentagon pairs | Independent-affinely-regular theorem and local general ten-facet theorem. | Both factors arbitrary non-affinely-regular pentagons. A ten-facet local theorem is not a global product theorem. |
| Fixed regular pentagon with arbitrary asymmetric convex partner | Ten-template translation-fit formulation; symmetric, equiangular and fixed-axis distorted-fan theorems. | Global minimum-area cover of all ten templates, with unrestricted partner geometry. [Partner account](../../products/pentagon-arbitrary-partner/README.md). |
| Fixed regular pentagon with arbitrary general-matrix regular normal fan | A complete homogeneous seven-variable cone with 610 distinct inequalities and an exact quadratic target. | Nonnegativity of `Q_+=P-det(B)/(3+sqrt(5))` on the positive-determinant cone sector, and equality. Ambient inertia `(3+,4−,0)` excludes a naive ambient PSD proof but does not refute cone positivity. [Cone reduction](../../products/pentagon-arbitrary-partner/general-matrix-cone-reduction-20261005.md). |
| Local rigidity against arbitrary fixed-pentagon partners | A placement theorem is written, with an exact finite certificate. | Independent formalization/review of the continuum signed-polygon estimate remains open in the maintained account. [Local-cover note](../../products/pentagon-arbitrary-partner/local-cover-rigidity.md), [finite packet](../../products/pentagon-arbitrary-partner/local-cover/README.md). Do not silently promote it to an accepted global theorem. |
| Generalized-Zoll status and systole-space topology of HKO | Many minimizing orbits, dense-open periodic dynamics, cut-additivity results in specified sectors. | Complete generalized-orbit-space/topological information; local maximality in a bounded-facet space is not itself a generalized-Zoll criterion. [Extrema literature map](../../literature/open-thesis/extrema.md). |
| Global HKO cuts-extremality/additivity description | The dated extrema map records numerical evidence and a near-boundary additivity theorem for specified normal sectors. | Every proper cut at arbitrary depth/direction is a different quantifier from a shallow-cut systolic inequality; cut capacity loss alone gives no ratio-loss rate. [Recorded comparison](../../literature/open-thesis/extrema.md). |
| Two nonsymmetric odd regular factors beyond `5×5`, `5×7`, `7×7` | Those cases have proved angular profiles. | General angular coefficient containment; arbitrary independent-affine formulas would need a stronger tensor-containment step. [Affine research](../../products/pentagon-affine-products/research.tex). |

The broader centrally symmetric-convex-body and distinct-capacity questions
remain external research problems in the inspected accounts. This inventory
makes no claim of a fresh literature-wide nonresolution check.

## Failed or superseded routes worth preserving

* The old smooth-envelope/Hessian proof sketch does not establish the HKO
  neighborhood theorem. The current proof uses positive feasible HK sections,
  which need not be optimizing KKT branches. Its exact certificate has 26
  sections, rank 25 after quotienting 15 symmetries, and a strictly positive
  zero relation. The [smooth-only rank diagnostic](../hko-local-maximum/smooth-only-rank-defect/README.md)
  found rank 23 from its 44 branches; it is a bounded failure of that bank,
  not an impossibility theorem for all smooth-only approaches.
* Historical one-cut vertex/edge lifting and the old
  [eleven-facet hypotheses](../eleven-facet-extension-hypotheses.md) are
  superseded as open F≤11 obligations by the accepted full proof. Their
  partial derivations remain provenance, not an argument that F≤11 is open.
* The scalar shortcut `(1+delta)(c/A2)²<=sqrt(5)/2` is false. An exact
  quadrilateral witness appears in the
  [asymmetry counterexample](../../empirical-studies/discovery-design/night-20261005/extremality-discovery/asymmetry-counterexample.md).
  The missing difference-body area excess is essential to that reformulation;
  restoring it does not itself prove the global HKO bound.
* Facet coverage and first-order balance alone are insufficient. The symmetric
  decagon is an exact covered/balanced nonmaximum, with a capacity-preserving
  area-decreasing support perturbation. The
  [final discovery synthesis](../../empirical-studies/discovery-design/night-20261005/comparison/final-research-disposition.md)
  routes this control and the complete fixed-normal active-dual criterion.
* Strict exclusion of all six-block pentagon maximizers is false: such
  coefficients can touch the four-block hull and tie at switching angles.
  Existence of a sparse maximizer does not classify every minimizer.
* A universal difference-body formula fails for two triangles. The odd-regular
  symmetric-partner theorem excludes `n=3` for a mathematical reason, not just
  absent computation. See the [products account](../../products/README.md).
* Negative neighborhood samples, 20 unsuccessful eleven-facet ascent
  placements and 32 quotient rays are finite evidence about their named
  sampling/evaluator contracts. They neither strengthen the proved facet
  count nor establish convergence, star-shapedness or absence of thin
  exceptional sets. [Local packet](../hko-local-maximum/README.md).

## A constructive quantitative corollary selected in this run

One useful distinction is between features necessary for **violating** the
inequality and those required for **maximizing** the ratio. Regularity, exact
product structure and polygonal corners can be necessary for a proposed
equality classification while being unnecessary for violation, because the
HKO ratio is strictly above one and capacity/volume are continuous.

The selected quantitative question asks for explicit non-affinely-regular or
nonproduct perturbations which remain counterexamples, using an elementary
capacity-monotonicity/volume bound and independent geometry checks. If
`(1-epsilon)H ⊆ K ⊆ (1+epsilon)H`, then

`sys(K) >= sys(H) ((1-epsilon)/(1+epsilon))^4`.

Thus every such body violates the inequality whenever
`epsilon < (sys(H)^(1/4)-1)/(sys(H)^(1/4)+1)`.
The inequality follows directly from quadratic capacity homogeneity and
four-dimensional volume scaling. It supplies a concrete guaranteed-open
counterexample neighborhood without implying that its members are local
maxima. The new contribution sought here is a rational general-position
fixture with an exact inclusion and nonproduct certificate. Openness and
smooth counterexample existence are already known from HKO and are not
claimed as new. The fixture's capacity need not equal HKO's for the proof
of violation.
