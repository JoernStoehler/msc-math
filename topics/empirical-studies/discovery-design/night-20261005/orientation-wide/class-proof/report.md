# Sharp global HKO bound and complete equality in the fixed-axis class

Scientific owner `direct_cover_lead`; receiving comparator
`scientific_compare_initial`. Root selected 2026-10-05 00:38 UTC; production
and independent review began 00:39. **The complete theorem, equality census
and physical controls were independently accepted at 01:03 UTC**, ahead of
the selected 01:23 cap. The 00:59 exact-oracle readiness gate was met.

## Theorem and exact scope

Fix the regular circumradius-one pentagon Q with outward normals
`n_i=(cos(2πi/5),sin(2πi/5))`. For every x>0, every angle θ, and every bounded
full-dimensional partner

```
K={q:(diag(x,1)R_θ n_i)·q≤h_i, i=0,...,4},
```

with arbitrary offsets, including redundant inequalities and genuine
three-/four-edge boundaries,

```
sys(K×_L Q)=c(K×_L Q)²/[2 area(K) area(Q)] ≤ (3+sqrt(5))/5.
```

The bound is sharp. Equality holds exactly when x=1,
`θ≡π/10 (mod π/5)`, and K is a translate and positive dilate of the regular
HKO partner `R_θ Q`. Modulo the fixed Q's dihedral symmetries this is the
single usual regular HKO class. There is no nonregular equality body and no
above-benchmark body in this class.

The distortion axes are fixed relative to Q. All orientations means all θ
in this two-parameter family; the theorem does not cover arbitrary GL(2)
normal transforms, all affine-normal pentagon fans, or arbitrary pentagon
partners. No literature-novelty claim or manuscript promotion is made here.

In the scaled-covector convention below, the unique capacity-one equality
class in gauge h3=h4=0 is

```
x=1, u=1/sqrt(5),
h*=(-1+3sqrt(5)/5, 1−sqrt(5)/5, -1+3sqrt(5)/5, 0, 0).
```

## Complete implication and parameter coverage

The accepted period/reflection reduction gives 0≤u≤1/sqrt(5), where
`u=tan(θ)/τ`, `τ²=5−2sqrt(5)`. Scaled covectors use
`C_u=[[1,−u],[τ²u,1]]` and `d(u)=det C_u=1+τ²u²`.
Canonical supports define Y=M'^T K for M'=diag(x,1)R_θ/cos(θ), with

```
P=Σh_i h_(i+1)−((sqrt(5)−1)/4)Σh_i²,
area(K)=P/[x d(u) sin(2π/5)],
sys=x d(u)/(5P) at capacity one.
```

Thus the exact sharp predicate is `G=P−x d(u)/(3+sqrt(5))≥0`.

First canonicalize redundant offsets to the body's actual supports. At each
fixed parameter, the independently reviewed contained-triangle/coercivity and
Brunn–Minkowski argument gives a capacity-one area minimum, and **every**
minimizing canonical support class is a vertex modulo translation. The
primary quadrilateral/triangle theorem gives ratio≤1 on every zero-edge
body, strictly below HKO. Hence a benchmark-relevant minimum has all five
edges and three independent active fit rows. The ten nonsingular fit triples
exhaust these candidates; all inactive fit and canonical edge inequalities
are retained. A feasible all-fit candidate has capacity exactly one because
an active complete fit row supplies equality.

The inherited [complete joint denominator certificate](../joint-denominators/report.md)
uses all ten genuine templates and all five closure rays. Its 21 open D-vector
formulas give 210 ten-fit occurrences and 97 distinct bilinear support formulas.
The 23 naive raw closed label representatives are matched by exact slope and
intercept equality in their actual F(alpha) embeddings to the 21 open formulas;
all 258 closed domains and 497 adjacent point incidences were checked.

Independent open clipping reconstructs all 800 degree-at-most-two event inputs
and 2,408 partition intervals. It accepts 1,416 feasible open-u components,
including 390 persistent singleton x fibers and no unbounded x fibers in the
nonzero-edge branches. The 350 intrinsically zero-edge incidences are covered
by the strict primary bound. The complete internal event catalog independently
matches every cut for 2,330 remaining domain/formula incidences: all 78 added
cut incidences, including isolated feasible cases, give 22 distinct additional
F/quadratic-F orientations. Original denominator orientation endpoints and
all their x boundaries remain included.

Complete closed replay checks 2,580 original closed-cell/basis incidences plus
78 added cut incidences: 1,700 feasible fibers and 958 empty fibers. It checks
actual ordered F(alpha) values, x>0, all endpoint and stationary cases,
excluded x=0 limits, and 190 unbounded tails with their leading/linear signs.
The closed tails belong to the full all-fit list, including zero-edge cases;
they were not omitted because the open nonzero-edge list has no tails.

## Exact signs and complete equality

The accepted open proof has 1,808 independently checked Sturm/sign queries.
For every component, a sufficient exact mode supplies its minimum: 591
concave/linear endpoint cases, 390 singleton cases, 400 lower derivative
exclusions and 35 upper derivative exclusions. Convex stationary points are
excluded by a certified whole-component endpoint-derivative sign. No general
cubic body field or global radical compositum was needed for the actual proof.
The independent engine checks Euclidean/gcd identities, multiplicity parity,
Sturm endpoint conventions, real embeddings, probes and original signed
normalization multipliers. The 224 rejected exploratory alternatives are
retained but are not used; all 1,416 final component implications pass.

Equality uses **full** root counts rather than odd-root counts alone. Every
one of the 974 endpoint-gap queries has zero interior roots, including even
roots, and none is identically zero. Their strict open endpoint gaps, combined
with the accepted minimum modes, exclude all open equality, including linear,
constant and singleton cases. No closed x-gap is identically zero. All 25
closed zero occurrences deduplicate to precisely the displayed body at
(x,u)=(1,1/sqrt(5)). After the global bound, any arbitrary-body equality becomes
an area minimum at its fixed parameters; the stronger every-minimizer vertex
statement therefore excludes nonvertex equality bodies. The strict primary
bound excludes all zero-edge equality.

Fresh direct physical reconstruction independently verifies five genuine
positive edges, ten exact primal template placements and twenty active
circuit/template dual pairs, so complete capacity is exactly one. All fifty
direct template/circuit sums agree with the accepted catalogue. Independent
K and Q vertices and shoelaces give the project ratio `(3+sqrt(5))/5` directly.
The physical metric gives equal five side squares and circumradius squares,
and Y has translated constant supports. Since x=1, undoing the coordinate
map gives the regular HKO partner; translations, dilation and the accepted
period/reflection symmetries give the stated full equality family.

## Evidence, verification and reproduction

[extraction.md](extraction.md) owns producer commands and write effects.
[review.md](review.md) owns the consequential independent derivations and
read-only replay commands. [retention.md](retention.md) distinguishes essential
certificates and checksummed inputs from workload caches, records sizes and
snapshot/math-section hashes, and states timing-byte-hash limitations.
[stage-checkpoints.md](stage-checkpoints.md) preserves intermediate measured
workloads and the bounded oracle decision.

Observed stage runtimes include formula derivation 3.953 s, open clipping
34.427 s, open proof 14.527 s, closed proof 26.356 s, equality extraction
6.564 s and physical reconstruction 0.292 s. Independent formula, domain,
sign, closure, equality and physical replays passed using separate arithmetic.
These are observed runs, not universal runtime promises. Timing/provenance
fields make whole JSON snapshot hashes differ on regeneration; they do not
change the retained mathematical statements.

This finishes the selected scientific stage. No broader class, new research
programme, manuscript edit, dashboard edit or additional computation was
started by this group after acceptance.
