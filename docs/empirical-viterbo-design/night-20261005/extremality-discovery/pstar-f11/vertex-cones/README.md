# Full vertex-cone cuts of P* and its equality family

Agent-derived exact obstruction, 4 October 2026. Independent review is in
progress. The original mathematical derivation and discovery limitation are
in [../../nonproduct-mechanisms.md](../../nonproduct-mechanisms.md).

## Retained statements and checks

1. At P*, every shallow single-vertex cut with normal in the interior of that
   vertex's normal cone has systolic ratio strictly below 1. This covers all
   24 vertices and every interior normal direction. The depth radius depends
   on the direction. The 24 barycentric rays have a common exact window
   `0 < epsilon <= 1/100`.
2. The same conclusion holds throughout some common neighborhood of the
   known equality-family parameters `(v,s)=(1/2,1/12)`. The existence of this
   parameter neighborhood follows from exact rational identities and a finite
   collection of strict coefficient signs. No explicit parameter radius is
   asserted, and no direction-independent cut-depth radius is asserted.

```bash
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/vertex-cones/verify.py
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/vertex-cones/verify-family.py
```

`witnesses.json` retains all 24 base words, primitive weights and polynomial
weight variations. `family-witnesses.json` retains their rational continuation
in `(v,s)`. The first verifier rebuilds matrices from literal base normals.
The family verifier imports the existing exact family definition, independently
rebuilds closure, objective and curvature, and checks the coefficient sign
interface used below. Neither executes an optimizer or overwrites old evidence.

## Equality-family continuation

Let E(v,s)=H(v,s)×T(v) be the family in
`experiments/hko-local-maximum/non-hko/PROOF.md`, Section 4. Its exact capacity
is 1 and volume is 1/2 on some neighborhood of the base. The same vertex and
facet labels persist there. Write a vertex's incident facets in increasing
label order as `(i0,i1,i2,i3)` and use

\[
w(v,s,t)=\sum_{a=0}^3t_a a_{i_a}(v,s),\qquad t_a>0,\quad\sum_at_a=1.
\]

For every vertex, the retained word has a rational closed primitive weight
b⁰(v,s), with b⁰_new=0 and Q(b⁰)=1/2. Its positive coordinates at the base
remain positive on a common parameter neighborhood; its zero coordinates are
identically zero. All retained coefficients are regular at the base.

The retained variation d(v,s,t) has d_new=1 and is linear in t at every old
position. Closure and mass normalization hold identically modulo sum t=1.
The first Q variation vanishes identically, so

\[
Q(b^0+\delta d)=\tfrac12-C(v,s,t)\delta^2.
\]

For every old position with b⁰=0, every coefficient in d is either an
identically zero rational function or is strictly positive at the base. This
zero-identity check is essential: continuity alone would not preserve a zero
coefficient's nonnegative sign. Therefore every such coordinate remains
nonnegative for the entire closed t-simplex after shrinking the parameter
neighborhood. Nonzero b⁰ coordinates absorb small negative variations.

Likewise, every quadratic coefficient of C(v,s,t) is either identically zero
or strictly positive at the base, and C is not the zero polynomial. Therefore
C>0 on the open t-simplex throughout a common parameter neighborhood. These
claims are checked coefficient by coefficient, not by sampling t, v or s.
A finite common neighborhood suffices for all 24 witnesses and their signs.

For fixed `(v,s,t)`, choose δ=ε/(2C). Weight feasibility holds for sufficiently
small ε>0 and the same exact action bound as at P* follows:

\[
c(E(v,s)\cap\{w\cdot x\leq1-\varepsilon\})
\leq1-\frac{\varepsilon^2}{2C(v,s,t)}.
\]

Simple vertex geometry gives the exact removed volume

\[
D(v,s,t)\varepsilon^4,
\qquad D(v,s,t)=\frac1{24|\det N(v,s)|\prod_at_a},
\]

for sufficiently shallow cuts, hence the ratio is strictly below 1. The
quantifier is

\[
\exists B\ni(1/2,1/12)\quad\forall(v,s)\in B
\quad\forall\text{ vertices }v_0\quad\forall t\in\operatorname{int}\Delta_3
\quad\exists\varepsilon_0(v,s,v_0,t)>0\quad\forall\varepsilon\in(0,\varepsilon_0):
\rho<1.
\]

This is an obstruction to escaping the known family by a new vertex facet;
it does not classify unrestricted local maxima, finite-depth cuts, or coupled
changes of old facets and a new facet.

## Edge strata and the remaining endpoints

Every C polynomial also has a strictly positive coefficient on each q/p
cross monomial (two incident q normals and two incident p normals). Thus C>0
whenever both q and p mass are positive, including every face of the t-simplex
with exactly three positive coordinates. Such normals expose an edge rather
than a vertex. A sufficiently shallow edge cap has volume O(ε³), with a
positive finite coefficient for each fixed ray. The same feasible branch gives
capacity loss at order ε², and therefore every fixed shallow edge-cut ray also
has ratio below 1. The face-specific branch remains feasible because old
zero-weight coordinates have nonnegative coefficients on the closed simplex.

Normals with only two positive coordinates expose ridges. Their removed volume
is order ε² and requires an actual coefficient comparison. Cone approaches
where t depends on ε also need a uniform bridge; the full vertex-cone statement
does not silently cover those approaches. Normals at simplex vertices duplicate
an old facet and belong to the established fixed-ten-facet chart.

## Provenance and review boundary

These witnesses were discovered by reconstructing the 12 complete primitive
product maximizers, then adding old zero-weight labels at supported trajectory
points and solving closure plus zero-Q-linear-variation systems. The first
incident-only weight redistribution did not work. A full 24-case base witness
list was retained, so the verifier does not rely on the discovery's 21-case
list plus an unverified symmetry shortcut.

The family variations were obtained from exact linear systems over QQ(v,s),
using the base nonsingular pivots and preserving zero old coordinates. The
accepted data are explicit rational functions; the verifier needs no optimizer
or proposer. Scratch provenance and rejected constructions are under
`/tmp/msc-math/nonproduct-mechanisms/` and are not the sole copy of accepted
proof data.

The mathematical trust boundary is the HK feasible-action upper bound, the
existing equality-family capacity theorem, simple local polytope geometry and
correct exact algebra. The family verifier shares the existing family
constructor. Mathematical review and bibliographic novelty are separate from
its successful execution.
