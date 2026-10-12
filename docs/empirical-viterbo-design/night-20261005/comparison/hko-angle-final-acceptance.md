# Final integrating acceptance: HKO-angle nonconformal fan

Owner scientific_compare_initial; receiving/selection owner root.
Accepted 2026-10-04 23:56 UTC. This is agent mathematical acceptance,
not manuscript/Jörn acceptance or external novelty closure.

For fixed regular Q, every partner represented by normals
`diag(x,1) R_(pi/10) n_i`, x>0, with arbitrary support offsets, has

    sys(K×_L Q) <= (3+sqrt(5))/5.

Equality holds precisely at x=1 for translations and positive dilations of
the regular HKO partner. Raw redundant inequalities are lowered to canonical
supports; canonical three-/four-edge boundary bodies are included. This is
a sharp global theorem within the selected distorted-fan slice, which contains
non-affinely-regular partners. It does not cover all relative angles of the
distorted fan, all arbitrary pentagon pairs or all convex four-bodies.

## Complete denominator input

Inspected [derivation](../hko-angle-nonconformal/denominators.md) and
[standard-library exact verifier](../hko-angle-nonconformal/denominators.py).
The convention is correct: computational covectors transform by
`C=[[1,−u],[τ²u,1]]`, u=1/sqrt(5), τ²=5−2sqrt(5); the common positive
cos(pi/10) scale is absorbed in arbitrary supports. Its determinant is
`d=2−2/sqrt(5)`. Canonical Y=M'^T K gives physical area
`P/(x d sin(2pi/5))` and capacity-one ratio `x d/(5P)`.
The [independent review](../hko-angle-nonconformal/review.md) verifies the
contragredient coordinate transformation, physical Jacobian and direct
shoelace at equality.

Invertibility preserves the complete five-circuit closure cone. The imported
complete ten-template theorem and positive numerator give the five constraints
`lambda_l·h>=D_l(x)`, where each D_l is the maximum over all ten template
denominators. The expansion of each three-support weighted maximum produces
27 exact affine choices per template, 1350 raw candidates total. The verifier
certifies every candidate below each retained envelope line on the whole
cell by exact endpoints or endpoint/slope inequalities; every envelope line
is attained. It checks adjacent boundary agreement and all maximizing raw
support choices at every common endpoint, including isolated ties.
All 50 separate denominators are positive. The resulting 17 circuit cells
have five common cells with boundaries 0,sqrt(5)−2,1/sqrt(5),1,sqrt(5),infinity.
Exact field sign/division comparisons preserve signs and use no decimals.

An unchanged scratch copy replayed in 1.85 s under
`/tmp/msc-math/scientific-compare-hko-angle-review/`, writing only scratch
JSON/log; its JSON exactly matches the retained producer output. The earlier
regular/aligned theorem outputs were unchanged.

## Candidate arithmetic, optimizer and equality

The independently reviewed canonical closed type cone and complete capacity
polyhedron reduce every area minimum to a vertex modulo translation.
Invertible M' preserves canonical edge-loss handling and the same
Brunn–Minkowski strict equality argument; its positive fixed determinant cannot
alter the reduction. The reviewer independently recalculated all 500
nonsingular systems, 65 distinct feasible affine records and their complete
intervals/area polynomials, including ten singleton and eleven unbounded
intervals. All 123 endpoint/stationary/tail predicates for
`P−x d/(3+sqrt(5))>=0` pass. No target polynomial vanishes identically;
every zero is at x=1, and all eight zero-containing records give one support
class `(-1+3sqrt(5)/5,1−sqrt(5)/5,-1+3sqrt(5)/5,0,0)`.
Thus neither nonvertex nor degenerate boundary equality is omitted.

The [optimizer verifier/certificate](../hko-angle-nonconformal/verify_optimizer.py)
supplies all ten primal fits and an active full-denominator dual, independently
rechecked by the reviewer. Hence capacity is exactly one, rather than merely
bounded above by an action witness. Every edge numerator is the same positive
number, giving regular canonical Y. The transformed physical vertex metric
and the inverse transform `cos(pi/10) R_(+pi/10)` confirm the regular HKO body.
The direct shoelace area has the required determinant and exact target value.
This closes the independent audit's last denominator/attainment dependencies.

## Scientific disposition

The selected mathematical slice task is complete, well before its 00:25 cap.
Root retains source/domain integration and next allocation. Another slice is
less compelling than a genuine orientation-wide class theorem or a new
geometric reduction, but full bivariate arrangement/proof complexity is still
unmeasured and unselected. The P* moving endpoint programme is a separate
selected group. No unrestricted global or novelty claim follows here.
