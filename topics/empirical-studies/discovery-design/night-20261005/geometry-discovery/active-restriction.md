# Sparse active-pair balance for planar Lagrangian products

Date: 2026-10-04 UTC. Owner: geometry-discovery lead; derivation by its
active-restriction worker. Scope: one mathematical route, exact small examples,
no population producer or paid API. Status: agent-derived proof, independently
checked by the receiving owner on 2026-10-04; Jörn review pending. Not promoted
into the formal document or thesis.

**Result.** For a planar Lagrangian product, the convex hull of *all* active HK
weights equals the convex hull of active weights built from one closure vertex
of each factor. These weights use at most three facets of each factor. Therefore
a fixed-normal local maximum with more than three facets in either factor must
have more than one active closure pair. More generally, the active pairs must
jointly represent the geometric facet-area target below. This is an active-set
restriction, not an all-feasible closure relaxation.

The first exact example rejects a seven-facet product with ratio 8/9 and gives
an increasing support direction. Its endpoint attains ratio 1 and uses two
distinct sparse pairs. It demonstrates why preserving just one capacity winner
loses information needed by a necessary-condition consumer.

## Sources and difference from existing results

The external input is [Haim-Kislev, Theorem 1.1 and Theorem 1.5,
arXiv:1712.03494v3](https://arxiv.org/html/1712.03494v3): the finite capacity
formula and existence of a minimizing pure-facet word. The latter bounds word
length by the number of facets, not by ambient dimension plus one. Project
sign/normalization conventions are owned by
`formal/hk2017-qp-conventions.tex`.

`formal/product-qp-six-facet-reduction.tex` and
`thesis/chapters/04-quadratic-program.tex`, subsection
`quadratic-program-product-six-facet`, prove existence of one sparse maximizing
weight and explicitly distinguish that result from classifying all maximizers.
The new step here preserves the **entire active weight convex hull**. Searches
of those owners, the capacity-smoothness classification and relevant formal
sources found no existing statement of this stronger preservation property.
This is a scoped novelty check within the repository, not a literature novelty
claim.

`formal/active-orbit-facet-coverage.tex`, Theorem
`facet-coverage-active-weight-balance`, supplies the fixed-normal necessary
condition. That theorem is marked agent-proved, awaiting Jörn review. The
local-maximality consequence below inherits this review status. The hull
identity has its own direct proof.

## Exact hull-preservation proposition

Let Q and P be nondegenerate planar polygons containing zero in their interiors,
and let K = Q ×_L P. Write their dual facet vertices as u_i and v_j. Set

\[
 C_Q=\{x\ge0:\mathbf1\cdot x=1,\ \sum_i x_i u_i=0\},\qquad
 C_P=\{y\ge0:\mathbf1\cdot y=1,\ \sum_j y_j v_j=0\}.
\]

Let M > 0 be the complete HK maximum. An **active closure pair** is a pair
(x,y) of vertices of C_Q and C_P for which some ordered distinct-facet word
attains M at β = (x/2,y/2). Let E be the finite set of such pairs, with duplicate
orders identified. Let B be the set of all maximizing HK weight vectors,
irrespective of order and support length.

**Proposition.**

\[
 \operatorname{conv}B
 =\operatorname{conv}\{(x/2,y/2):(x,y)\in E\}.
\]

**Proof.** Fix any active word and weight β. Separate its factor masses as
β = (t x,(1-t)y). Same-factor pairings vanish, so its objective is

\[
 Q_\sigma(\beta)=t(1-t)B_\sigma(x,y)
\]

with B_σ bilinear. Activity and M > 0 imply 0 < t < 1 and B_σ(x,y) > 0.
Varying t is feasible and has its unique maximum at t = 1/2; hence every
active β has t = 1/2.

Decompose x and y into vertices of their support-restricted closure polytopes:
x = Σ_a λ_a x_a and y = Σ_b μ_b y_b. These polytopes are coordinate faces of
C_Q and C_P, so their vertices are also vertices of the full factor polytopes.
For the same padded word σ, bilinearity gives

\[
 M=Q_\sigma(x/2,y/2)
  =\sum_{a,b}\lambda_a\mu_b Q_\sigma(x_a/2,y_b/2).
\]

Each summand is a feasible complete HK datum, so its value is at most M.
The coefficients sum to one. Consequently every term with λ_a μ_b > 0
equals M. Deleting zero positions keeps the objective and order of retained
positions. Each pair with positive coefficient therefore belongs to E, and
β is the corresponding convex combination. This proves one inclusion;
the reverse follows because every pair in E is itself active. ∎

Each closure vertex has at most three positive coordinates by the three scalar
closure/normalization equations. The proposition does **not** say every
active vector has at most six positive coordinates: convex combinations can
have larger support. Dense tied minimizers and singular optimizer plateaux are
included, rather than omitted by a smooth-branch assumption.

## Nonuniversal geometric restriction

Work in an open fixed-normal support chamber where all inequalities define
facets. Polygon products are simple, so such chambers exist. Let h_i be the
unit-normal support numbers, ℓ_i the factor edge lengths, A_Q and A_P the
factor areas, and V = A_Q A_P. Define positive geometric probability vectors

\[
 \tau^Q_i=\frac{h_i\ell_i}{2A_Q},\qquad
 \tau^P_j=\frac{h_j\ell_j}{2A_P}.
\]

The facet-balance theorem says the target β_i = h_i S_i/(4V) lies in
conv B. Since S_i = ℓ_i A_P for q-facets and S_j = ℓ_j A_Q for p-facets,
the hull proposition yields the stronger finite interface

\[
 (\tau^Q,\tau^P)\in
 \operatorname{conv}\{(x,y):(x,y)\in E\}. \tag{*}
\]

Equivalently there must be nonnegative λ_(x,y), summing to one, with
Σ_E λ_(x,y) x = τ^Q and Σ_E λ_(x,y) y = τ^P. One common set of coefficients
must satisfy both marginals: independent membership in the hulls of the two
projections is weaker.

This is not factor-mass balance: mass 1/2 in each factor is universal here.
Nor is it all-feasible closure membership, which is universal by planar
Minkowski identities. The restriction is that **globally capacity-active
vertex pairs** must realize these particular facet-area distributions.

If Q has r facets and P has s, positivity of the target implies:

- The union of supports of the active pairs covers every facet of each factor.
- Any active-pair representation of (*) needs at least
  max(ceil(r/3), ceil(s/3), ceil((r+s)/6)) distinct pairs.
- A unique active closure pair excludes a local maximum whenever r > 3 or
  s > 3. “Unique” concerns the pair of weights, not the word: different orders
  can have the same β.

The first inequality can be strengthened when actual closure vertices have
support smaller than three, by counting their actual support unions. Also,
Carathéodory bounds a balanced representation by at most r+s−5 pairs:
the factor closure polytopes have affine dimensions r−3 and s−3, because the
dual vertices of a full-dimensional polygon affinely span the plane.
This upper bound counts pairs in a mixture, not facets of a single orbit.

A full dual-vertex local maximum must satisfy this support-subchart condition.
Satisfying (*) does not certify maximality: first-order flat directions and
normal-changing directions still need separate treatment.

## First exact calculation: an excluded near-equality product

Take the primal factors

\[
 Q=\{q_1\le1,\ q_2\le1,\ -q_1-q_2\le1\},\quad
 P=\{-1/2\le p_1\le1,\ -1/3\le p_2\le1\}.
\]

Their dual lists are
u = ((1,0),(0,1),(−1,−1)) and
v = ((1,0),(0,1),(−2,0),(0,−3)). The factor closure vertices are exactly

\[
 x=(1/3,1/3,1/3),\qquad
 y_X=(2/3,0,1/3,0),\quad
 y_Y=(0,3/4,0,1/4).
\]

For P, closure enforces y_1 = 2y_3 and y_2 = 3y_4, leaving a line segment
whose endpoints are the two displayed vertices. Thus these are all pairs;
the sparse reduction is complete.

An exact rational enumeration fixes the first active label as cyclic anchor
and checks all 4! = 24 orders for each five-facet support:

| Pair | Maximum HK Q | One maximizing cyclic word, labels 0–6 |
| --- | --- | --- |
| (x,y_X) | 1/9 | (0,1,3,2,5) |
| (x,y_Y) | 1/8 | (0,1,4,2,6) |

Here labels 0–2 are q-facets and labels 3–6 are p-facets in the displayed
dual-list order. Consequently E has one pair; c_EHZ = 4, A_Q = 9/2,
A_P = 2, V = 9 and sys = 8/9. Both p_1 facets are invisible to *every*
active weight by hull preservation, despite several maximizing words.

The geometric marginal is τ^P = (1/3,3/8,1/6,1/8), which is not y_Y.
The +p_1 facet has unit normal, support 1 and three-volume S = 6. Its inward
motion therefore has the strictly positive derivative

\[
 D\operatorname{sys}[-e_{+p_1}]
  =\frac89\frac69=\frac{16}{27}.
\]

There is also a finite exact improvement. Replace p_1 ≤ 1 by p_1 ≤ 1−t.
The triangle leg length is 3, and rectangle widths are X = 3/2−t and
Y = 4/3. For this family, the two closure-pair maxima are

\[
 M_X=\frac1{6X},\quad M_Y=\frac1{6Y},\qquad
 c=3\min(X,Y).
\]

One way to verify this formula is to project each weighted triangle dual row
onto the relevant axis: the three components are 1/3, 0 and −1/3; the two
weighted rectangle rows are 1/X and −1/X, or 1/Y and −1/Y. The cyclic
four-row interaction has maximum 2/(3X), respectively 2/(3Y), before
the factor-mass prefactor 1/4. Alternating the nonzero rows attains it.
The zero-projection triangle row can be inserted anywhere.

Thus, for 0 ≤ t < 1/6,

\[
 c=4,\quad V=9−6t,\quad
 \operatorname{sys}=\frac{16}{18−12t}.
\]

At t = 1/6, both pair maxima equal 1/8. The x-pair marginal is now
(5/8,0,3/8,0), and τ^P is their equal mixture with y_Y. Both pairs are
needed to cover the rectangle, and balance succeeds. The endpoint has V = 8
and sys = 1. Indeed, throughout this fixed-normal product family, a triangle
of leg length L has area L²/2 and a rectangle has area XY, so
sys = min(X,Y)/max(X,Y) ≤ 1; X = Y gives a global maximum in that chamber.
This chamber result makes no claim of full-chart local maximality.

## Reproduction of the bounded rational calculation

The calculation used Python's standard-library Fractions, 48 cyclic words,
no floating point, and no project producer. This self-contained code preserves
the calculation independently of its disposable scratch copy:

```python
from fractions import Fraction as R
from itertools import permutations, combinations

u = [(1,0),(0,1),(-1,-1)]
v = [(1,0),(0,1),(-2,0),(0,-3)]
a = [q+(0,0) for q in u] + [(0,0)+p for p in v]
ys = [(R(2,3),0,R(1,3),0), (0,R(3,4),0,R(1,4))]

def omega(a,b):
    return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]

for y in ys:
    beta = [R(1,6)]*3 + [R(c)/2 for c in y]
    support = [i for i,b in enumerate(beta) if b]
    first, *rest = support
    scores = []
    for tail in permutations(rest):
        word = (first,) + tail
        score = sum(beta[i]*beta[j]*omega(a[i],a[j])
                    for i,j in combinations(word,2))
        scores.append(score)
    print(len(scores), max(scores))  # 24 1/9; 24 1/8
```

Scratch transcript: `/tmp/msc-math/geometry-active/exact-example.json`.
All lasting input, arithmetic results and reproduction code are retained here.

## Adverse alternatives, falsifier and scientific implication

**Five-facet Carathéodory route fails.** A product of two triangles has one
normalized closure vector per factor, positive on all three facets. Every
active datum has equal factor masses and therefore all six coordinates
positive. A six-facet active weight cannot be replaced by a feasible active
five-facet weight there. Carathéodory controls vertices of a closure polytope,
but maximizing a quadratic form does not generally occur at such a vertex.
The useful product bound is three facets in *each* factor, from bilinearity.

**Strongest practical rival.** A numerical routine returning one winning
closure pair can appear to violate (*) at a true maximum because it omits
tied pairs. The explicit t = 1/6 endpoint demonstrates this failure with
rational inputs. Complete exact comparisons, or certified strict gaps for
every discarded closure pair, are required before rejection. A catalogue of
selected feasible upper sections, such as the HKO proof witness, is not such
a complete active catalogue.

**Falsifier.** With an exact complete active-pair graph, infeasibility of (*)
falsifies local maximality in the fixed-normal chamber. A support absent from
every pair gives an explicit inward ascent direction; the example computes
that derivative and finite improvement. A dense exact active β lying outside
the sparse active-pair hull would falsify the new hull proposition itself;
the proof identifies the decisive check: an average of feasible scores bounded
above by M cannot equal M if any positive-weight term has a strict gap.

**Scientific implication.** Products with a unique closure-pair winner and a
factor with more than three facets are excluded as local maxima, regardless
of how many orderings realize that winner. High-ratio endpoints should be
searched on loci with enough tied *support pairs* to cover the facets and
match their area marginals. This gives a precise structural feature for
endpoint discovery, stronger than counting orbit words or checking universal
factor mass. No rate of occurrence, global facet-count ceiling or unrestricted
Viterbo inequality follows.

Next dependency: selection of any empirical use and Jörn mathematical review
before thesis promotion. No larger enumeration or integration into the capacity
consumer was started within this bounded note assignment. The parent research
authorization remains separate; this sentence creates no new approval gate.
