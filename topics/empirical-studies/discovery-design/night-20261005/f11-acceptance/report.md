# HKO F≤11 acceptance

Receiving owner: `/root`; scientific comparison owner: `/root/scientific_compare_initial`.
Review begun 2026-10-04 22:51 UTC; accepted 23:00 UTC. This directory owns this review only; original
packet and existing experimental evidence remain immutable.

## Precise claim and current status

**ACCEPTED as a computer-assisted theorem, taking the previously accepted sharp
ten-facet certificate and primary capacity principles as inputs.** The result is an existential Hausdorff neighborhood
of the standard HKO pentagon product H in which every convex four-polytope K
with at most eleven genuine facets satisfies

`c_EHZ(K)^2 / (2 Vol_4(K)) ≤ (3+sqrt(5))/5`.

Full fresh finite replay and independent endpoint/other-face analytic review
are complete. No false or missing implication was found. This is project
acceptance of the frozen proof packet, not proof-assistant formalization or
external refereeing. The quantifier permits arbitrary old-normal motions and nonlinear/coupled
approach sequences. No numerical neighborhood radius, eleven-facet equality
classification, theorem for twelve facets, or unrestricted-body theorem follows.

Candidate source: `topics/variation-and-local-maxima/hko-local-maximum/f11-return-20261004/`. The immutable
ZIP was checked for traversal paths and extracted into a fresh dedicated scratch
directory. Two bounded independent children completed the full finite replay and
the inherited endpoint/other-face implications; they own separate reports here.
No manuscript, promoted theorem, or retained producer output is changed.

## Fresh analytic checks by the integrating reviewer

### The accepted ten-facet input is genuinely sharp

`formal/hko-feasible-section-upper-branches.tex`, positive convex certificate
lemma and finite feasible-branch proposition, gives a uniform negative margin η
on the quotient-slice unit sphere. Finitely many smooth upper functions have a
common remainder bound ≤η||h||/2. The proof therefore yields the stronger bound
`rho(P(A_H+h))/rho_H−1 ≤ −η||h||/(2 rho_H)` if its function f is the unnormalized
ratio (or −η||h||/2 under relative normalization). This is the required linear
deficit, rather than an inference from strictness alone. The transverse symmetry
chart follows from the inverse function theorem and the independent fifteen
orbit tangent directions. The existing exact ten-facet certificate is accepted
input; this review does not claim a fresh Sage replay of that older certificate.

### Duplicated normals still give capacity upper witnesses

[HK Remark 1.4](https://arxiv.org/html/1712.03494v3) expresses capacity as the
inverse supremum of polar-velocity loop action. Closure and nonnegative unit
weights produce a piecewise affine admissible loop. The ordered action is Q
with the packet's orientation; hence `c_EHZ(K) ≤ 1/(2Q)`. Normals need only
belong to K's polar; distinctness and genuineness are unnecessary. This checks
the hypothesis needed for the duplicated-label smooth sections, independently
of the distinct-facet permutation formula.

### The final compactness step covers actual sequences

Polar convergence holds in a common inner/outer ball neighborhood. A functional
uniquely exposing each of H's ten polar vertices selects a maximizing vertex
of each nearby polar converging to it. Disjoint neighborhoods make the ten
selected vertices distinct, leaving at most one extra vertex. The completed
other-face charts exhaust the strata other than duplicate vertices.

Both the extra label and the old label are genuine facets of K. In a duplicate
chart the new inequality must cut some of the moving old facet and leave some
of it. Thus its linear depth has positive maximum and negative minimum. A
radial difference with zero oscillation could not give two genuine facets.
The moving facet vertices linearly span R4, so their depth values in [−1,1]
bound the normalized D uniformly. Subsequence limits therefore exist.

For an interior limiting D, the cut volume is O(t). Monotonicity combined
with sharpness gives `r=O(t)` on any improving sequence. A strict certificate
at the *one limiting direction* supplies a finite list of positive sections
and a fixed margin m. Continuity preserves ≥m/2 on nearby D_n; finiteness
gives one feasibility radius and bounded second derivatives. This is enough
for `S_n ≤ −(m/2)t_n+O(t_n²)<0`. No global minimum as μ→0 is used.

The needed first-order volume upper estimate follows from the inherited
relative cap bound: `ΔV ≤ (1+O(t)) t ∫_{F(A)}(D·x)_+ dS/||a_0(A)||`.
A fixed labelled facet triangulation varies smoothly with A; positive part
is Lipschitz. The integral differs from the base integral by O(r), uniformly
for bounded D. Together with the smooth old volume expansion this gives the
upper version of PROOF.md (4), with remainder O(r²+rt+t²). Equality or a
two-sided differentiability claim at the duplicate constraint is unnecessary.

### The upper collar response formula has the right sign

Write g_0 and g_b for the literal Q derivatives of the two duplicate labels.
Then `c=2g_b/Q_H`, swapped `c'=2g_0/Q_H`, and aggregate
`G_0=−2(g_0+g_b)/Q_H−DV_0/V_H`. Since `−DV_0/V_H=w_0`,
`c'=w_0−G_0−c`. A cancelling convex relation yields `c'=w_0−c`.
The identity `v(D)−v(−D)=w_0·D` then transfers strictness. Swapping labels
preserves positivity/rank of the same section words, including words omitting
one label. This is an algebraic transfer of witnesses, not a capacity derivative
assumption.

### Closure correction has the required span

Every actual smooth touching upper function has zero derivative along the
simultaneous fifteen-dimensional symmetry orbit, because it bounds a constant
ratio there and touches at H. The same holds for limiting aggregate gradients.
The old anchor's invertible mass-plus-gradient minor proves rank at least 25
for gradients, while symmetry annihilation proves rank at most 25. Hence the
old rows span the whole annihilator. A mass-zero signed correction can cancel
any small residual; mixing an O(residual) positive old anchor absorbs its
negative entries. This justifies moving from closure responses to actual
positive-section certificates without asserting smoothness at zero weights.

### Exact arithmetic signs have a mathematical justification

The field polynomial `t^4−10t²+5` is Eisenstein at 5. It is irreducible and has
one root in (0,1), since it strictly decreases there from 5 to −4. The ordinary
field helper reduces products using `t^4=10t²−5`, checks inverses exactly, and
isolates the root by rational bisection. Its interval-Horner signs are
conservative: a sign is returned only if the complete rational enclosure is
strictly on one side of zero; otherwise it raises an error. A successful
replay cannot silently accept a sign unresolved by its fixed isolation width.

For the integer polynomial helper, write any element as `A+B t`, where
`A=(u+5w)−2w sqrt(5)` and `B=(v+5z)−2z sqrt(5)`. Its `sign5` first compares
signs, then squares opposite-sign terms to compare `a²−5b²`. Its outer sign
routine uses the same valid comparison of `A²−(5−2sqrt(5))B²` when A and B
have opposite signs. Squaring occurs only after distinguishing signs, so the
sign information is preserved. This derives the universal sign rule rather
than relying on its 500 random comparisons with the separate interval helper.

## Completed independent checks and retained evidence

- [finite-replay.md](finite-replay.md): full standard-library replay in a fresh copied scratch
  packet, source/environment fingerprints, exact cover and data-index checks,
  negative controls, and source audit of all acceptance predicates.
- [endpoint-audit.md](endpoint-audit.md): actual relative-scale and moving-face geometry audit of
  ENDPOINTS.md and the open-face charts; finite PASS alone is not this proof.

The earlier independent interior bridge review already derives the polygon
moment identity, coefficient homogenization/denominator clearing, and literal
section derivatives. The present reviewer checked its implications against
PROOF.md and the relevant code and found no contradictory convention. Actual
finite reconstruction/sign predicates were then accepted by the fresh replay.

## Costs and research implications

No paid API, discovery search, or retained producer was invoked. The packet's
217-second replay is an author measurement; the full fresh replay with all three
negative controls took **387.039 seconds** on Python 3.12.3. Agent inference
expenditure and monetary cost are not measured locally. Both bounded children
completed and released their assignments before the first 23:10 UTC review point.

Acceptance closes the moving-normal F≤11 research target. It leaves larger facet counts as a different
scientific problem. A useful adverse return is an exact failed predicate or
one named analytic implication, not restarting already superseded ridge/facet
searches. No such failure was found. The strongest rival next group is a
scientific investigation of actual ≥12-facet mechanisms; that is a new result
target and receives no theorem from this acceptance. The integrating root owns
domain-source/manuscript/dashboard reconciliation and any decision to pursue it.

Fresh finite results: inherited ridge 6.701 s; inherited endpoint 11.866 s;
core 262.052 s; collar 66.362 s; three controls 0.498 s, 0.195 s and 39.363 s.
Core accepted 3,258 positive touching rows, 824 exact cancelling relations and
1,968 covering tetrahedra with margin ≥10^-6. Collar accepted 257 touching
paths, twenty policies, 160 sign cells, 887 covering leaves and 195,140
nonnegative coefficients (55,928 exact zeros), proving margin μ²/1000.
All three corruptions failed at their intended predicates and produced no
false success JSON. All four positive mathematical outputs equal the author's
retained replay after excluding timing/environment fields. All 140 original
and copied source files remained unchanged; incidental bytecode stayed in
exclusive scratch. Complete fresh outputs, logs, source/output hashes,
environment and exact static cover checks are retained in [finite-replay/](finite-replay).

There is **no remaining acceptance bottleneck for this precise theorem**.
The verifier's positional policy/root-group conventions were checked exactly
for this frozen packet; making those assumptions explicit guards in a reusable
verifier is optional hardening, not a mathematical gap in the accepted input.
