# Products: capacity, deformations and partner geometry

A Lagrangian product `P ×_L Q` places two planar convex factors in the standard
`q,p` coordinate planes. The scalar capacity problem separates into factor
closure polytopes; geometric containment then explains several exact families.
This topic owns the product proofs, regular-product producers and the
fixed-regular-factor partner study. The selected thesis is a separate consumer.

## Results and their strength

| Question | Result and canonical source | Status and boundary |
| --- | --- | --- |
| Why can planar products be computed with short words? | [Six-facet reduction](product-qp-six-facet-reduction.tex): some global HK maximizer uses at most three facets per factor, with mass one half in each. | Jörn accepted the proof on 28 July 2026; included in [Chapter 4](../../thesis/chapters/04-quadratic-program.tex). Existence of a sparse maximizer does not classify every optimizer. |
| What is the rotated regular-pentagon profile? | [Chapter 9](../../thesis/chapters/09-rotated-regular-polygons.tex) proves the secant capacity / secant-squared systolic profile by endpoint interpolation, an attaining word and symmetry. | The underlying analytic argument is accepted; human acceptance of the selected exposition is separate. The HKO endpoint capacity is an external input. |
| What survives independent factor deformations? | [Affine proof packet](pentagon-affine-products/README.md): for nonsingular `G,H`, `c(GP5 ×_L HP5)=w²/max|n_i^T G^-1 H^-T n_j|`, `w=1+cos(pi/5)`. The sharp ratio is `(3+sqrt(5))/5`, with its stated equality class. | Agent-reviewed theorem included in [Chapter 10](../../thesis/chapters/10-affine-pentagons.tex). This covers affinely regular pentagons, not arbitrary pentagons or every ten-facet body. It does not import the HKO endpoint value. Human proof acceptance and novelty are unestablished. |
| What are all odd regular rotation profiles? | [All-odd angular theorem](pentagon-affine-products/odd-regular-angular-width.md): for odd `n,m>=5`, `c(P_n ×_L R_theta P_m)=(1+h_n)(1+h_m) sec d_lcm(n,m)(theta)`. The sharp ratio is `(3+sqrt(5))/5`, with equality exactly at `n=m=5`, `theta=pi/10+k pi/5`. | New research, independently checked by [two](pentagon-affine-products/odd-regular-angular-width-audit.md) [agents](pentagon-affine-products/odd-regular-angular-width-second-audit.md), outside the selected thesis. Strict angular containment holds except for 5×5; the independent-affine extension beyond 5×5, human acceptance and novelty remain open. |
| What if only the pentagon factor is fixed? | [Symmetric-partner proof](pentagon-arbitrary-partner/symmetric-partner-lemma.md): every centrally symmetric full-dimensional planar convex partner has capacity equal to the difference-body width value. The sharp ratio is `(10+6sqrt(5))/25`, attained exactly by the prescribed decagon up to translation and positive dilation. | Agent-checked analytic theorem, covering smooth bodies as well as polygons. Jörn selected the pentagon instance; it is now integrated as [Section 11](../../thesis/chapters/10b-symmetric-partners.tex). Source/math review, build and focused page checks passed; human proof/prose acceptance and a whole-PDF PASS remain open. |
| Does that symmetric-partner theorem extend beyond pentagons? | [Odd-regular theorem](pentagon-arbitrary-partner/odd-regular-symmetric-partner.md): all odd `n>=5` have the width formula with centrally symmetric partners, a sharp constant `kappa_n<1`, and affine-image transfer. | Agent-checked analytic theorem, dated 3 October. Human acceptance, thesis selection of the generalization and publication novelty are open. It is not merely a proposed extension. |
| What does fixed-width symmetrization guarantee? | [Sharp plateau proof](pentagon-arbitrary-partner/symmetrization-plateau-20261019.md): for every full-dimensional planar convex partner `K`, `K_t=(1-t)K+t(-K)` attains the fixed-pentagon width capacity throughout `[(3-sqrt(5))/2,(sqrt(5)-1)/2]`. | New agent-checked research outside the selected thesis. The interval is sharp as a uniform guarantee: an explicit trapezoid has exactly these saturation endpoints; other partners can saturate earlier. This does not prove a global HKO ratio bound for arbitrary partners or classify its equality cases. |
| What is known for asymmetric partners? | [Restricted partner theorems](pentagon-arbitrary-partner/README.md#restricted-asymmetric-partner-theorems-5-october): the fixed-axis `diag(x,1)R_theta` normal family with arbitrary supports has sharp HKO bound and regular-only HKO-angle equality; equiangular and aligned/nonconformal slices have separate exact results. | Independently agent-reviewed on 5 October. No arbitrary GL(2), arbitrary-partner global theorem, human acceptance or manuscript promotion. The general-matrix cone reduction leaves positivity open. |
| What is the exact two-bounce class? | [Two-bounce proof](product-two-bounce-class.tex): the strong two-bounce minimum and complete mathematical `k=2` QP optimum give the same difference-body inradius. | Agent-reviewed, awaiting Jörn. Universal equality with the transition-pruned, f64-seeded implementation remains conditional on retaining and exactly resolving an equality candidate. |
| What is special about triangles? | [Triangle classification](product-triangle-bounce-classification.tex) proves strict directed triangle-cycle realization and the two-/three-bounce capacity comparison under its upstream hypotheses. | Agent-reviewed, awaiting Jörn. Its placement argument does not establish physicality for general locally admissible words. |
| Can rotating a symplectic product improve its ratio? | [Product-position chapter](../../thesis/chapters/11-product-position.tex) and [convention/endpoint packet](product-rotation/README.md) give the universal initial regime and a triangle example with a worse Lagrangian endpoint. | Exact argument with bounded rational/matrix check. The sufficient interval need not be maximal; no novelty or human prose acceptance is asserted. |

The fixed-pentagon symmetric-partner theorem, the independent affine-pentagon
bound and the [local at-most-eleven-facet maximum](../variation-and-local-maxima/README.md)
have different domains. Arbitrary symmetric partners need not be pentagons;
independent linear pentagon factors retain product structure; nearby at-most-eleven-facet
perturbations can break it. None of these results proves a global maximum over
all convex bodies or all ten-facet polytopes.

## Why the sparse reduction matters

For each ordered facet word, product closure splits into two planar normalized
closure polytopes. Their vertices have support at most three. The objective is
bilinear in their factor weights and proportional to `t(1-t)` in the factor
mass; extremality supplies a closure-vertex pair and mass `t=1/2`. Enumerating
these pairs and their cyclic interleavings recovers scalar capacity.

The production adapter is
[`capacity_4d/product.rs`](../../crates/symplectic/src/algorithms/capacity_4d/product.rs),
with exact/hybrid comparison tooling in
[the product-closure route](../../topics/capacity-and-qp/development/tools/product_closure_route/README.md).
The formal reduction does not by itself certify numerical proposal coverage,
implementation behavior, or recovery of every geometric minimizing orbit.
Exact certification of a candidate and completeness of a candidate family are
separate obligations. The two-bounce packet states the remaining stream-coverage
condition explicitly; a retained 20-row exact comparison is finite validation.

## Containment routes and the important failures

The [affine note](pentagon-affine-products/research.tex) identifies four-block
competitors with width-body pairings and the remaining six-block obstruction
with paired normalized-closure triangles. For pentagons, two explicit tensor
convex-combination identities control that obstruction for every independent
linear map. The [all-odd circle-deficit proof](pentagon-affine-products/odd-regular-angular-width.md)
now gives complete rotation profiles for every odd `n,m>=5`, with strict
containment and open labelled nonregular neighborhoods except for 5×5. It
preserves the earlier 7×7 and 5×7 arguments as proof history. These angular
results are outside the selected thesis and do not imply independent-affine
formulas beyond the separately proved pentagon case.

Three stronger shortcuts fail or remain unsupported:

- A universal difference-body capacity formula fails for two triangles: the
  note gives actual capacity `3/2` while the four-block value is `9/4`.
  The odd-regular symmetric-partner statement therefore excludes `n=3`.
- Strict exclusion of every six-block pentagon maximizer fails. Six-block
  coefficients touch edges of the four-block hull and can maximize at switching
  angles. Sparse existence cannot be promoted to a classification of all winners.
- Endpoint interpolation proves an upper bound, but interior sharpness needs a
  common feasible coefficient attaining both endpoints. Endpoint equality alone
  supplies no such witness.

A separate [product-position study](product-position-research/README.md) proves
that factor areas and both endpoint capacities need not determine the intermediate
rotation profile. Its [lift-persistence counterexample](product-position-research/lift-persistence.md)
shows that a lift can remain strictly contained and pass the local insertion
test after losing global capacity minimality. These are exact
separating constructions, not universal continuation criteria or additional
selected-thesis results.

The width formula for **two nonsymmetric odd regular factors**, odd `n,m>=5`,
is now an [agent-checked theorem](pentagon-affine-products/odd-regular-angular-width.md)
(19 October 2026). A continuous minimum-angle triangle inequality controls the
six-block obstruction for factors with at least seven sides; pentagon partners
use their isosceles closure geometry and the stated prior cases. The remaining
independent-affine question is stronger tensor containment, which angular
containment does not supply. This theorem is distinct from the proved
odd-regular-factor/centrally-symmetric-partner theorem.

For arbitrary asymmetric partners of the fixed pentagon, the normal-triangle
fitting representation enables cheap exploration, but the global minimum-area
cover remains unresolved. The [partner account](pentagon-arbitrary-partner/README.md)
preserves the local rigidity/cut/contact routes and their limitations.

## Code, retained evidence and reproduction

[Regular products](regular-products/README.md) owns Rust helpers, broad sampled
rotation sweeps, pentagon illustrations and the earlier Sage certificate.
The selected analytic rotation proof does not consume the empirical sweeps or
the Sage enumeration. The retained exhaustive route classifies 3,340 orderings
on `0<theta<pi/10`; continuity and symmetry complete the domain outside that
run. Its recorded transcript does not bind the current script's byte identity:
the inventory found a shebang-only hash difference with matching Python ASTs.
Use the packet's source/output provenance rather than claiming a fresh run.

[Fixed-factor partner geometry](pentagon-arbitrary-partner/README.md) owns the
10-template floating-point LP evaluator, collection, controlled paths and
analytic deductions. Calibration discrepancies measure those retained controls;
they are not rigorous capacity certificates for arbitrary inputs. Contact labels
are fitting contacts, not billiard-orbit labels.

The [pentagon handoff provenance](pro-handoffs/README.md) retains the original
prepared input, returned proof and independent audits. The maintained affine
source owns corrections; frozen input paths/hashes retain their historical
meaning.

The separate [variation topic](../variation-and-local-maxima/README.md) owns HKO
feasible upper sections, exact local-maximality evidence, empirical perturbations
and the failed smooth-envelope route. Its integrated F≤11 argument includes one facet birth, relative to the inherited
F10 certificate; it supplies neither a ≥12-facet theorem nor global maximality.

Producer and certificate commands live with their packets and may replace
retained outputs. The dated new proofs and Section 11 integration above are
outcomes of the authorized 19 October research swarm; reading or maintaining
this account does not authorize another producer run or further thesis scope.
