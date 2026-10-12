# Supplied-input next operation: the moving right endpoint of the p1 tilt

Prepared by the integrating lead, 2026-10-04 23:50 UTC. Proposed continuation,
not selected execution. The completed packet proves the open direction patch
and the whole fraction interval at fixed P*. This operation addresses their
remaining endpoint interface, rather than repeating cut samples.

## Exact target and its domain

Use the maintained full old-normal chart

    A=A(v,s)+zW(v,s)+Yy,
    u(v)=1−v+v²,
    D(v,eta)=(0,−2(u(v)−eta),1,0),
    a_new=a_0(A)+tau D(v,eta).

Prove that one common neighborhood and positive eta,tau windows give ratio
strictly below1 for every genuine eleven-facet body of this form, including
arbitrary coupled limits `eta→0`, `tau→0`, and old-normal/equality-family
motion. Only the old normals are unrestricted in the full chart; the new
tilt has this specified p1 form in the transported group coordinates.
At the equality base, the right p vertex is `(u(v),0)` and the cap fraction
is eta. The centering must follow u(v), rather than incorrectly holding the
base value3/4 while v moves.

This is a moving endpoint theorem for a specified tilt family, **not** the
full all-direction F11 theorem. Other tilt components, other old facets and
other limiting support strata remain outside it.

## Supplied derivation inputs

- `topics/variation-and-local-maxima/hko-local-maximum/non-hko/code/family_exact.py`: explicit
  equality normals/vertices, volume gradient, stationary system, old-section
  continuation and symmetry columns.
- `facet-split/moving-old/certificate.json` and its verifier: symbolic
  stationary continuum over QQ(v,s,r), the full18 annihilators, q-only W jet,
  regular five-column closure minor, and strict base gradient balance.
- The existing F10 proof: full chart, constant rank22, positive normalized
  old cancellation, and uniform cusp/quadratic transverse estimates.
- `facet-fraction/`: adaptive `r=eta/6`, exact duplicated section and cap
  moments at the base, with complete Bernstein positivity over the fraction
  interval. These base formulas are controls, not moving-body cap premises.
- `transverse-cap-reduction.md`: independently checked conditional algebra
  reducing all unresolved paths to `|z|=O(sqrt(tau)eta^(3/2))` and
  `|y|=O(tau eta³)` once the next cap estimate is established.

## Concrete proof operations and stopping predicates

1. Prove uniformly, for `e=|z|+|y|`, the geometric bound
   `DeltaV<=C tau(eta+e)³`. Use a tubular chart about the old facet: the
   equality cap is a strip in its q-edge times the p-corner wedge. Under old
   normal motion the relevant vertex heights change byO(e), its bounded
   edge/intersection matrices stay regular, and its facet cap measure must
   be bounded byO((eta+e)²). Integrate cut depthO(tau(eta+e)). Prove those
   bounds, rather than infer them from the fixed-body moment formula.

2. Compute the family splitting response from the supplied QQ(v,s,r)
   stationary continuum and choose `r=k eta`, `delta=theta r` with fixed
   positive k,theta valid on a compact family patch. Prove the capacity
   response is at least `c1 eta²`. Extend a small fixed gamma balance
   uniformly to `r=0`: the added gradient is regular there and lies in the
   old rank22 rowspace; a fixed augmented rank23 minor and the strict old
   null relation can provide bounded positive old coefficients. Obtain the
   actual bounds, or retain the exact obstruction.

3. In the relative region forced by operation1 and F10, prove positive
   section feasibility. Constant-sized weights stay positive; weights of
   sizeeta must have correctionsO(e+tau eta), so `e/eta→0` is sufficient.
   Factor the rational section formula and its mixed derivatives to prove
   the split remainder is `O(tau eta e+tau² eta²)`. An unfactored
   `O(tau e)` estimate is insufficient. This is the decisive exact predicate.

4. Combine the old-witness region and the new balanced-witness region.
   Independently review the cap estimate, the factorizations and the
   quantifiers. Stop with the accepted restricted endpoint theorem or the
   first exact failed premise; a failed premise is not an improving body.

## Proposed ownership, bounded resources and uncertainty

The receiving science comparator/root would select this operation. Reuse the
existing two specialists: production owns `endpoint-moving-normal/` with a
derivation, exact formulas and small read-only verifier; independent review
owns `independent-endpoint-moving-normal-review.md`. The lead integrates the
accepted statement and residual domain. No paid API, new producer, broad
search or retained experiment replacement is needed.

A proposed cap is35minutes for derivation and exact factor checks, followed by
10minutes for independent review/integration; return an actual intermediate
after20minutes and stop by45minutes even if the endpoint theorem remains open.
These are proposed bounds, not a new allocation or a probability estimate.
The cap bound appears approachable from simple-polytope geometry; the scaled
mixed remainder is less certain and is the sharp stopping predicate. The
full all-direction F11 extension would still require a separate decomposition
and witnesses for q-edge/p2 tilt and the other support-stratum transitions.
