# Sharp global result for the aligned nonconformal normal fan

Scientific owner: `direct_cover_lead`; receiving/comparative owner:
`scientific_compare_initial`. Selected contract: 2026-10-04 23:28 UTC.
First actual theorem and independent audit returned before 23:45 UTC.
No manuscript/formal promotion or literature novelty claim is made.

## Theorem and unique optimizer

Fix the regular circumradius-one pentagon Q with outward unit normals
`n_i=(cos(2πi/5),sin(2πi/5))`. For **every** `x=1+s>0`, put
`G_x=diag(x,1)` and let

```
K_x(h)={q:(G_x n_i)·q≤h_i, i=0,...,4}
```

be bounded with nonempty interior. Supports are arbitrary; redundant raw
inequalities and actual three-/four-sided edge-loss bodies are included.
For this entire aligned distorted-fan family,

```
sys(K_x(h) ×_L Q) ≤ B* = 3(1+sqrt(5))/10 = 0.9708203932499369...
```

The bound is sharp. Equality occurs **only** at

```
x*=sqrt(5)/3,  s*=sqrt(5)/3−1,
h*=((sqrt(5)−1)/6, (sqrt(5)+1)/6, 1/3, 0, 0),
```

after translating the capacity-one canonical support vector to the gauge
`h_3=h_4=0`. All equality bodies are precisely translations and positive
dilates of this `K_x*(h*)`. All five edges are genuine, so its product with Q
is an actual 5+5/ten-facet product.

The optimizer is **not affinely regular**. This is independently checked by
all ten cyclic/reversed affine vertex correspondences, rather than inferred
only from unequal circuit denominators. It is a new optimizer within this
selected family, with ratio below both one and the HKO benchmark
`(3+sqrt(5))/5`. It is not an HKO equality body or an arbitrary-partner local
maximum; normals outside this family are not covered.

## Complete capacity and parameter cells

Put `z=sqrt(5)`, `b=(1+z)/4`, `w=1+b`. The five normalized closure rays are
the cyclic shifts of

```
λ^(0)=(1,1,0,2b,0)/(2w).
```

Their completeness follows from the accepted regular-core argument: no
opposite pair exists, and the positive triples have cyclic gaps `(1,2,2)`.
Invertibility of G_x preserves exactly the entire closure cone, not merely
these five displayed solutions.

For the complete ten fixed-Q normal unit-perimeter templates T_j,

```
D_l(x)=max_j Σ_i λ_li h_Tj(G_x n_i)>0,
c(K_x(h)×_L Q)=min_l (λ_l·h)/D_l(x).
```

The maximum denominator is essential. Template translation leaves each
weighted support sum unchanged; translating the template to contain zero
proves positive denominators. The complete dual polytope is a positive
section of the same closure cone. Thus capacity at least one is exactly the
five inequalities `λ_l·h≥D_l(x)`, with no omitted fitting class.

The [denominator derivation](denominators.md) and
[exact envelope certificate](denominators.json) supply three pieces per
circuit and six common x cells. Define

```
A=((13−5z)x+5−z)/20, B=((1+z)x−5+3z)/20,
C=((z−2)x+z)/10, E=(3z−5)(x+1)/20,
F=((4−z)x+5−2z)/10.
```

| Circuits | First piece | Middle piece | Final piece |
| --- | --- | --- | --- |
| 0,4 | A, up to z/3 | B, up to z | x/5 |
| 1,3 | C, up to (5+4z)/11 | A, up to 2+z | (z−1)x/10 |
| 2 | E, up to (4z−5)/11 | F, up to z/3 | (z−1)x/5 |

All endpoint formulas agree. The five positive common breakpoints are
`(4z−5)/11 < z/3 < (5+4z)/11 < z < 2+z`; the first cell starts at zero
(excluded from the body theorem), and the last extends to infinity.
At `x=1`, all D_l equal `(z−1)/5`, matching the independently rerun
[accepted equiangular control](../geometry-discovery/fixed-q-rank/equiangular-capacity-core.md).

Each circuit has three positive weights. Distributing its support maxima
produces 27 affine candidates per template, 270 per circuit, 1,350 total.
Distinct counts are `71,69,49,69,71`. Exact dominance checks against **every**
raw affine candidate at both endpoints, or a left endpoint and slope on an
unbounded cell, certify the envelope over complete cells. All 50 individual
template/circuit denominators also have certified positive envelopes.

## Closed support cone and the global vertex implication

Writing `Y=G_x^T K_x(h)` gives the regular normal fan and
`area(K_x(h))=area(Y)/x`. This is a coordinate identity; changing one factor
alone is not asserted to preserve capacity or the symplectic form.

Replace redundant raw offsets by canonical supports `h_Y(n_i)`. This leaves
Y and K unchanged. Put `a=cos(2π/5)=(z−1)/4`. Canonical supports form the
closed cone

```
e_i(h)=h_(i−1)+h_(i+1)−2a h_i ≥ 0, i=0,...,4.
```

Their actual Y-edge lengths are `e_i/sin(2π/5)`. Zero edges are retained.
The kernel is exactly the two-dimensional translation space, and the quotient
has dimension three. Gauge `h_3=h_4=0` is legitimate because those two normals
are independent. The only zero-area canonical body in this cone is a point:
there are no opposite edge directions admitting a segment.

For fixed x the capacity-one feasible polyhedron has exactly the five fit
inequalities and five closed edge inequalities in this quotient. Every
feasible body contains a translate of a genuine template. A disk inside that
template gives `area(Y)≥r||p−z_0||` for every hull point p, so area sublevels
are compact modulo translation. A minimum exists. Its recession cone is the
closed type cone modulo translation; it has no lines, and every nonzero
recession body has positive area.

Minkowski addition is linear on these canonical supports, including zero
edges. Brunn–Minkowski therefore makes `sqrt(area(Y))` concave. Every minimizing
support class is a vertex: a nontrivial feasible segment through a minimum
would force equal endpoint areas and equality in Brunn–Minkowski, hence
homothetic equal-area bodies, hence translates and the same quotient class.
A minimizer has capacity exactly one, since dividing a body of capacity >1
by that capacity strictly reduces area while retaining feasibility.

These implications, including canonicalization, zero edges, coercivity,
recession and equality, received an [independent mathematical review](vertex-premise-review.md).
The finite-vertex method is known from the older fan work; the new content
here is its complete nonconformal parameter realization, sharp bound and
nonregular optimizer, not a claim to have invented vertex reduction.

## Exact finite area certificate

There are at most `C(10,3)=120` active-row triples. The constant quotient
matrix has **100 nonsingular triples and 20 singular triples**, derived
before running the parameter candidates. Across six cells this gives 600
nonsingular systems. On each cell D is affine in x; constant inverses therefore
make candidate supports affine. All ten feasibility inequalities are affine
in x and determine exact closed candidate intervals, including singletons.

| Common cell | Feasible bases on some subinterval | Distinct affine support records |
| --- | ---: | ---: |
| 0 | 15 | 12 |
| 1 | 20 | 17 |
| 2 | 20 | 11 |
| 3 | 20 | 17 |
| 4 | 29 | 23 |
| 5 | 29 | 8 |

These are interval records, not counts of simultaneously present geometric
vertices. There are 88 records total, including 19 singleton intervals and
8 unbounded intervals. All finite parameter boundaries are included using
continuous adjacent denominator formulas.

The exact quadratic area numerator is

```
P(h)=Σ_i h_i h_(i+1)−a Σ_i h_i²,
area(K_x(h))=P(h)/(x sin(2π/5)),
sys(K_x(h)×_L Q)=x/(5P(h)) at capacity one.
```

Here `area(Q)=5 sin(2π/5)/2`, so the ratio formula has no missing coordinate
Jacobian. For affine candidate supports P is quadratic in x. The sharp bound
is equivalent to

```
P(h(x)) − ((sqrt(5)−1)/6)x ≥ 0.
```

The [candidate certificate](body-candidates.json) checks this quadratic on
every exact feasible interval: endpoints, an interior stationary minimum when
needed, and limiting coefficient signs on an unbounded interval. There are
170 necessary exact checks. No gap quadratic is identically zero. Eight
zero-containing records all give `x=z/3` and deduplicate to the one support
class h*. The independently retained rational-pair/Cramer audit verifies all
600 systems, all 88 intervals and all 170 inequalities using a different
arithmetic implementation. The global-vertex implication then proves both
the bound for all supports and the complete equality statement above.

The excluded parameter limits also have been checked: every record reaching
`x=0` has strictly positive constant P coefficient, and every unbounded
record has strictly positive quadratic coefficient. Consequently the maximal
ratio in this family tends to zero as `x→0+` and `x→∞`; no missing supremum
comes from the distorted-fan endpoints.

## Exact optimizer and controls

[verify_optimizer.py](verify_optimizer.py) reconstructs its five intersections
and checks every halfplane and all five positive edge numerators. They are

```
((z−1)/3, (z−1)/6, 1/3, 1/3, (z−1)/6).
```

All five circuit sums equal their complete D_l at x*. It separately supplies
exact feasible translations of **all ten templates** inside the body. These
primal fits give capacity ≥1; a maximizing template on an active circuit gives
an exact dual upper bound 1, so capacity is exactly one. Its area and ratio are
checked directly from the vertex shoelace formula. The lasting
[optimizer certificate](optimizer-certificate.json) includes the placements,
slacks, vertices and ten failed affine-regular correspondences.

For the affine-regular exclusion, any invertible affine map between convex
pentagons preserves or reverses cyclic vertex order. Thus there are ten possible
correspondences with the regular pentagon. Three point images fix each affine
map; at least one of the remaining two images fails in every correspondence,
with exact nonzero residuals retained. This avoids an unproved normal-fan
stabilization assertion. The optimizer is genuinely non-affine-regular.

At x=1, the accepted regular-fan theorem gives the unique regular optimizer
and ratio `(5+2z)/10≈0.9472135955`, strictly below B*. Edge-loss triangles and
quadrilaterals are included in the global certificate and cannot attain B*,
because its unique equality body has five positive edges. The full family
lies strictly below one and HKO; this is a new restricted optimizer, not a
counterexample to Viterbo or an improved HKO body.

## Reproduce, costs and remaining research boundary

Run from repository root, sequentially:

```
python topics/empirical-studies/discovery-design/night-20261005/nonconformal-fan/denominators.py
uv run topics/empirical-studies/discovery-design/night-20261005/nonconformal-fan/body_candidates.py
uv run topics/empirical-studies/discovery-design/night-20261005/nonconformal-fan/verify_optimizer.py
python topics/empirical-studies/discovery-design/night-20261005/nonconformal-fan/review_candidates.py
```

The denominator script uses Python standard library; the two lead scripts
have inline SymPy dependencies. All ordering and certificates are exact in
`Q(sqrt(5))`; JSON rational arrays use the encoding stated in each file.
The three producer scripts overwrite only their respective owned JSON files;
the independent auditor writes no evidence file. Setup of the 120 constant
systems took about 2.2 seconds including uv startup, denominator certification
about 1.0 second, body-candidate arithmetic about 0.8 second, and the independent
600-system audit about 1 second locally. These are measured small retained
computations, not costs for a two-parameter extension.

The selected one-parameter scientific task is complete. No continuation is
needed to establish its theorem. The serious next rival is allowing relative
orientation, `G_x Rθ n_i`, whose regular x=1 slice includes HKO and could test
whether this finite capacity/area mechanism survives a larger class. That
would require a new allocation: exact two-variable envelopes and feasibility
regions are no longer the six one-dimensional cells. A preliminary comparison
is separate; no such larger execution is implied here. The unrestricted fixed-Q
cover problem remains unresolved. All-angle equiangular and independently
affine-regular results remain distinct inputs, not substitutes for this
nonregular-support theorem.
