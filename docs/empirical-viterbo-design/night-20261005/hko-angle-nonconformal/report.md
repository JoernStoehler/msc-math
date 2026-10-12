# Sharp global HKO bound on the HKO-angle nonconformal slice

Scientific owner: `direct_cover_lead`; receiving/comparative owner:
`scientific_compare_initial`. Root-selected
[contract](../comparison/hko-angle-contract.md), gate satisfied 2026-10-04
23:45 UTC. Complete field/cell/count return supplied at 23:50; independent
bound/equality/capacity audit completed at 23:54. The comparator
[accepted the complete chain](../comparison/hko-angle-final-acceptance.md) at
23:56, including denominator completeness. Returned before the 00:25 cap.
This is checked research, not manuscript/formal promotion or a novelty claim.

## Theorem, full domain and equality

Fix the regular circumradius-one Q with outward unit normals
`n_i=(cos(2πi/5),sin(2πi/5))`. Put `θ=π/10` and `G_x=diag(x,1)`.
For **every x>0** and every bounded full-dimensional partner

```
K={q:(G_x R_θ n_i)·q≤h_i, i=0,...,4},
```

with arbitrary offsets, including redundant inequalities and genuine
three-/four-edge boundaries,

```
sys(K×_L Q) ≤ B_HKO=(3+sqrt(5))/5.
```

The bound is sharp. Equality holds **only at x=1**, and only for translations
and positive dilates of the regular HKO partner `R_(π/10)Q`.
There are no new nonregular equality bodies in this slice and no above-HKO
witness. This theorem includes non-affine-regular support choices throughout
the distorted fan, so the independently affine-regular theorem alone does
not subsume its domain. It remains a restricted 5+5 statement, not a bound for
all pentagon partners or arbitrary fixed-Q covers.

In the scaled-covector convention below, the unique capacity-one equality
support class, in gauge `h_3=h_4=0`, is

```
h*=(-1+3sqrt(5)/5, 1−sqrt(5)/5, -1+3sqrt(5)/5, 0, 0).
```

All five edges are genuine. Its product with Q has ten actual facets.
The distinction between this h and the original unit-normal offsets is the
common positive `cos(π/10)` scale, explicitly retained below.

## Correct covector convention and area normalization

Put `z=sqrt(5)`, `τ²=5−2z`, `u=1/z=z/5`. Computational points are
`(q_x,q_y/τ)` and computational covectors are `N=(n_x,τ n_y)`.
Consequently the physical pairing is their ordinary computational dot product.
For `S=diag(1,τ)`, the **covector** rotation divided by cosθ is

```
C=S R_θ S⁻¹/cosθ = [[1,−u],[τ²u,1]],
d=det C=1+τ²u²=2−2z/5=sec²θ>0.
```

The point-coordinate conjugate `[[1,−τ²u],[u,1]]` is a different matrix
and is not used to rotate these normals. The original fan normals are a
common factor cosθ times `G_x C N_i`. Absorb that factor into all arbitrary
supports; this preserves the full family of physical bodies. Every denominator
and support in the retained certificate uses this scaled convention.

In physical coordinates set `M'=G_x R_θ/cosθ`. The regular-fan body
`Y=M'^T K` has supports h and

```
area(K)=area(Y)/(x d).
```

For `a=cos(2π/5)=(z−1)/4`, define

```
P(h)=Σ_i h_i h_(i+1)−a Σ_i h_i².
```

Then

```
area(Y)=P/sin(2π/5),
area(K)=P/[x d sin(2π/5)],
sys(K×_L Q)=x d/(5P) at capacity one.
```

The last identity uses `area(Q)=5 sin(2π/5)/2`. Dropping d would be a
normalization error. The independent [review](review.md) checks the physical
Jacobian and direct adjacent-intersection shoelace formula separately.
At x=1, recovery is **K=cosθ R_(+θ)Y**, with the positive rotation.

## Complete all-ten capacity and exact parameter cells

The five normalized closure rays are cyclic shifts of

```
λ^(0)=(1,1,0,2b,0)/(2(1+b)), b=(1+z)/4.
```

They are complete for the regular fan by the accepted regular-core proof.
Invertibility of `G_x C` preserves the entire nonnegative closure cone, its
normalization and extreme rays. For the original complete ten fixed-Q templates,
which are **not** rotated for this computation,

```
D_l(x)=max_j Σ_i λ_li h_Tj(G_x C N_i)>0,
c(K×_L Q)=min_l (λ_l·h)/D_l(x).
```

Template translation drops out by closure. Translating a genuine template
to contain the origin proves positive weighted supports. Taking the maximum
of its ten denominators is required for the minimum capacity at fixed positive
numerator. Thus the five inequalities `λ_l·h≥D_l(x)` are exactly the complete
capacity-at-least-one constraints, rather than action upper bounds.

[denominators.md](denominators.md) supplies the full formulas and
[denominators.json](denominators.json) their exact certificate. The full x>0
range has five common cells with boundaries

```
0 < z−2 < z/5 < 1 < z < infinity.
```

Each circuit has 3,3,4,3,4 pieces, respectively: 17 circuit cells total.
At x=1 all five scaled denominators equal `(z−1)/5`, checked from every raw
candidate and direct geometric support, matching the HKO rotation control.

There are 27 affine vertex-choice candidates per template/circuit, 270 per
circuit and 1,350 total. Distinct affine counts are `67,67,71,47,71`.
Every reported piece is an actual candidate; every raw candidate lies below
it over the whole piece, certified at finite endpoints or by an endpoint and
slope on an unbounded cell. Individual envelopes certify positivity of all
50 template/circuit denominators. The JSON retains **all** maximizing raw
vertex choices at every common finite boundary, including isolated ties,
and checks these maxima directly from the geometric supports.

The continuous x=0 denominator limits are
`1/5,1/5,(z−1)/10,(z−1)/5,(z−1)/10`.
Their final slopes are
`(3−z)/5,(3−z)/5,(z−1)/5,2(z−2)/5,(z−1)/5`.
The body domain excludes x=0; these values certify its one-sided envelope limit.

## Global optimization and complete equality proof

First canonicalize raw supports to `h_Y(n_i)`, leaving Y, K, capacity and area
unchanged. Canonical supports lie in the closed type cone

```
e_i=h_(i−1)+h_(i+1)−2a h_i≥0.
```

Its actual regular-Y edge lengths are `e_i/sin(2π/5)`. Zero edges stay in
the domain. The translation kernel has dimension two; gauge `h_3=h_4=0`
leaves dimension three. This transfers exactly from the independently proved
[canonical vertex theorem](../nonconformal-fan/vertex-premise-review.md).

At fixed x, fitting a genuine triangle gives a disk inside every feasible K.
The disk inequality `area≥r||p−z_0||` makes area sublevels compact modulo
translation. The canonical polyhedron has recession cone equal to the closed
type cone and no quotient lines. Minkowski addition is linear on canonical
supports, including zero edges. Brunn–Minkowski makes square-root area concave.
Every global minimizer is a vertex: a nonvertex minimum would force equality
in Brunn–Minkowski between two distinct equal-area feasible bodies, which
must instead be translates and the same quotient class. Capacity at a minimum
is exactly one, since dividing a capacity >1 body by its capacity reduces area.
These contracts are included in the new [independent review](review.md).

The constant ten-row quotient matrix has 100 nonsingular and 20 singular
triples out of 120. On each of the five denominator cells, the 100 candidate
solutions have affine supports and quadratic P. All ten feasibility inequalities
are affine in x and clip their exact intervals, retaining singleton intervals.

| Common cell | Feasible bases on some subinterval | Distinct affine records |
| --- | ---: | ---: |
| 0 | 20 | 9 |
| 1 | 20 | 17 |
| 2 | 20 | 17 |
| 3 | 20 | 11 |
| 4 | 20 | 11 |

These are interval records, not simultaneous vertex counts. The complete
500-system result contains 65 records, including ten singleton intervals and
11 unbounded intervals. The [body certificate](body-candidates.json) proves
on **every** exact feasible interval

```
P(h(x)) − [d/(3+z)]x ≥ 0.
```

Endpoint and stationary-minimum tests, plus asymptotic coefficient signs,
require 123 exact evaluations. A separate rational-pair/Cramer
[read-only auditor](review_candidates.py) independently reconstructs all 500
systems and all 65 intervals and checks these 123 inequalities.
No target quadratic is identically zero. Eight zero-containing records all
have x=1 and give exactly one quotient support class h*. Together with the
strong every-minimizer-is-a-vertex statement, this excludes nonvertex equality
bodies and proves the complete equality classification.

At h*, every edge numerator equals `p=-1+3z/5>0`. Moreover
`h_i*=D+N_i·z_Y` for an exactly checked vector z_Y and `D=(z−1)/5`.
Thus Y is a translate of the regular pentagon with all supports D. At x=1,
K is a translate of `cosθ R_θ Y`, giving precisely regular HKO; scaling and
translation restore the stated equality family. The known x=1 regular theorem
is a control for this equality, while the x≠1 arbitrary-support bound is the
new domain covered by the finite certificate.

For every candidate interval reaching x=0, the constant P coefficient is
strictly positive. For every unbounded interval, its quadratic P coefficient
is strictly positive. Therefore the **optimal ratio over all supports tends
to zero** as x→0+ or x→infinity; no missing supremum or equality lies at a
parameter degeneration. Compactness used in the body proof is at each fixed x;
these parameter limits come from the finite exact coefficient checks instead
of assuming uniform coercivity.

## Direct exact HKO control and disposition

[verify_optimizer.py](verify_optimizer.py) and its
[optimizer certificate](optimizer-certificate.json) reconstruct the physical
K vertex intersections in computational coordinates. They check all five
facets genuine and direct shoelace area. Exact feasible translations and all
50 nonnegative supporting slacks place **all ten** original templates inside
K, proving capacity at least one. A maximizing template on an active closure
circuit supplies the exact dual upper bound one. Hence capacity is exactly one.
The independent auditor reconstructs those supporting checks and dual.

The direct product ratio is additionally checked from the shoelace areas of
both K and Q, with physical Jacobian τ² in their area product. It equals HKO,
without relying solely on the transformed area formula. Equal physical side
lengths and circumradii are checked with metric `diag(1,τ²)` on computational
points; the canonical constant-support translation gives a separate regularity
proof. The positive rotation is explicit.

The result is a sharp restricted global theorem, not a new optimizer beyond
HKO. No failed capacity premise or above-HKO body was found. Actual lower-edge
bodies were retained throughout and none attains equality.

## Reproduction, bounded costs and further scope

Run from repository root, sequentially:

```
python -B docs/empirical-viterbo-design/night-20261005/hko-angle-nonconformal/denominators.py
uv run docs/empirical-viterbo-design/night-20261005/hko-angle-nonconformal/body_candidates.py
uv run docs/empirical-viterbo-design/night-20261005/hko-angle-nonconformal/verify_optimizer.py
python -B docs/empirical-viterbo-design/night-20261005/hko-angle-nonconformal/review_candidates.py
```

All arithmetic and ordering are exact in Q(sqrt(5)); coefficient encoding is
specified in each JSON. Denominators use Python standard library; lead scripts
have inline SymPy dependencies. Each producer overwrites only its own new
sibling certificate. The independent auditor is read-only. No retained older
producer was rerun or changed. No paid API, manuscript/coordination edit, broad
population producer or bivariate cell decomposition was used.

Measured local costs: complete denominator and endpoint-tie certification
about 1.81 seconds; 500 candidate arithmetic about 0.53 seconds; independent
all-system/bound audit about 0.82 seconds. The first uv setup/geometry run took
about 2.2 seconds including dependency setup. These are costs for this fixed
angle only. They do not establish the cost of a two-parameter active-envelope
arrangement.

The selected slice is complete; no renewal is needed for its proof. Root's
requested further orientation/global-family comparison is preparatory only
and will be retained separately. Full x,θ optimization remains unselected.
