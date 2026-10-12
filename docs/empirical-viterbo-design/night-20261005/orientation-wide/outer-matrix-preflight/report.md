# General-matrix finite quadratic-cone preflight

Scientific owner `direct_cover_lead`, receiving comparator
`scientific_compare_initial` and selecting owner root. Root selected 2026-10-05
02:08 UTC; lead/producer actual start 02:10, reviewer 02:11. Actual transfer/count
checkpoint 02:18; absolute integrated cap 02:28. Only this new directory is
owned. No old certificate rerun, ray enumeration, redundancy LP, positivity
calculation or global theorem is part of this stage.

**Accepted at 02:21 UTC:** the complete general-matrix source-to-cone
reduction, coefficient inventory and exact witnesses pass independent review.
Quadratic copositivity and a general-matrix sharp bound remain unproved.

## Genuine all-matrix coordinates

Write A for the arbitrary invertible physical normal matrix. Put
`S=diag(1,τ)`, `τ²=5−2sqrt(5)`, with physical points `p=S p_comp` and
computational regular covectors `N=S^T n`. The independently bound general
computational matrix is

```
B=S^T A S^(−T)=S A S^(-1),
B N=S^T A n,
Y_phys=A^T K_phys,
Y_comp=B^T K_comp.
```

Thus detB=detA, and canonical supports h in Y_comp have exactly the physical
regular support values. The four entries of B are independent real variables:
using F=Q(sqrt(5)) coefficients in the scaled basis does **not** restrict the
physical matrix to F entries. In physical coordinates
`A=[[B00,τ B01],[B10/τ,B11]]`.

For canonical supports,

```
P(h)=Σh_i h_(i+1)−((sqrt(5)−1)/4)Σh_i²,
area(Y_phys)=P/sin(2π/5),
area(K_phys)=P/[|detB| sin(2π/5)],
sys(K×_L Q)=capacity² |detB|/(5P).
```

The source has the same ten genuine fixed-Q templates and five normalized
positive extreme closure circuits as the accepted class certificates.
Invertibility gives `Σλ_i B N_i=0` exactly when `Σλ_i N_i=0`. Template
supports are `max_v (B N_i)·v_comp`; every vertex value is linear in the
four entries of B.

## Linear capacity cone and determinant sign

Complete capacity≥1 is precisely all five `λ_l·h≥D_l(B)`, where each D is
the maximum over all ten templates of the weighted vertex supports. Expanding
**every** three-active-normal vertex choice for each template gives the exact
linear inequalities

`λ_l·h−Σ_i λ_(l,i) (B N_i)·v_(choice_i)≥0`.

No active-label pruning is applied. The five canonical edge rows are
`h_(i−1)+h_(i+1)−2cos(2π/5)h_i≥0`. Their translation kernel is the span of
N's two coordinate columns; all closure rows annihilate that kernel, and
`P=½ h^T E h` has the same translation invariance. Gauge h3=h4=0 leaves three
support coordinates. Together with B00,B01,B10,B11 this is a homogeneous
seven-variable polyhedral cone C.

For detB<0, use J=diag(1,−1). Since `J N_i=N_(-i)`, the same physical normal
ray set is represented by B'=BJ with positive determinant and reversed
supports h'_i=h_(-i), followed by the exact translation gauge. P is invariant
and circuits permute l→4−l. This relabels the same body rather than assuming
an unproved outer-angle symmetry. Determinant-zero matrices are not bodies;
**they need not lie on the boundary of C**. They are boundaries between the
invertible determinant-sign sectors of the admissible physical domain.
Indeed B=0 with strictly canonical positive supports lies inside C.

For any body of capacity c>0, h/c is canonical and has complete capacity one
at fixed B. Therefore a bound `P(h/c)≥|detB|/(3+sqrt(5))` would imply the
sharp ratio bound with its required c² factor. Simultaneously scaling B and
h by a positive scalar represents the same physical body; this different
scalar equivalence also preserves the homogeneous quadratic target.

## Accepted actual inventory and source controls

Independent source/arithmetic derivation finds 1,350 raw fitting inequalities
and five edge rows. Positive-scale-only exact deduplication gives **610 unique
rows**. Full constraint rank is seven, with zero lineality. An exact strict
full-dimensional witness uses B=I and physical regular constant supports one;
in h3=h4=0 gauge it is

`h=((5−sqrt(5))/2,sqrt(5),(5−sqrt(5))/2,0,0)`.

The same gauged h with B=0 is a strict witness inside C at determinant zero,
confirming the determinant-domain caveat above. Independent closure, gauge,
edge/P invariance and reflected circuit permutation passed. Exact maximum
coefficient bit-size under the reviewer's positive-magnitude normalization
is five. The final producer ran in 2.340 seconds, and full independent replay passed
in 5.971 seconds. Every one of the 1,355 raw source occurrences, its original
positive normalization divisor, the 610-row reflection permutation, rank
basis and minimum slacks were verified. Both strict witnesses have minimum
normalized slack `(5−sqrt(5))/2>0`. These checks do not certify quadratic
nonnegativity.

## Remaining preflight and next mathematical burden

The retained [cone.json](cone.json) contains the complete finite inventory,
translation/gauge and representative matrices, original normalization signs,
rank/kernel and strict witnesses. Every one of the 1,350 source-choice
coefficients projects exactly to the accepted scalar-family catalogue under
`B=[[x,−xu],[τ²u,1]]`. Its source catalogue SHA-256 is
`a5f9c24f565611dac9db9c10755c61c24b58c8a346f54fbbb49ab8d06d065240`.
The known HKO point has zero target gap and nonnegative complete fit/edge
slacks; its right-reflected negative-determinant representative has the same
body and area. No old full certificate was rerun.

The genuine next problem is quadratic copositivity of
`Q_+(h,B)=P(h)−detB/(3+sqrt(5))` over C. This is equivalent to the proposed
sharp bound on the actual determinant-positive body sector: det≤0 gives
Q_+≥0 automatically from canonical P≥0, and right reflection folds negative-
determinant physical bodies into the positive sector. For arbitrary capacity,
normalizing h/c retains the required c² factor. This is an exact reduction of
the unknown statement, not its proof.

The retained seven-variable quadratic matrix has ambient inertia
**three positive, four negative, zero zero directions**. Its support block
has principal minors `(1−sqrt(5))/4`, `(1−sqrt(5))/8`, and `1/8`, giving one
positive and two negative directions; the determinant block gives two of each.
A concrete first falsifier of an ambient PSD or squares-only candidate is
`h=(1,0,0), B=0`, where `Q_+=(1−sqrt(5))/4<0`. This vector violates the
canonical edge inequalities and is **not** a body counterexample. It explains
why unconstrained PSD is the wrong proof predicate.

No explicit nonnegative constraint-product decomposition or geometric
reduction identity was found within the selected coefficient preflight.
Such a candidate would need a concrete coefficient identity plus nonnegative
weights and a verified PSD remainder. Finite-face/KKT coverage would need all
relevant feasible faces and stationary/boundary minima; neither sampled rays
nor local Hessians would suffice. No LP/SDP, ray enumeration, face enumeration,
CAD or positivity search was performed or priced. Row count and dimension do
not supply a success probability or runtime. Root received the inertia and
first falsifier at 02:21 and parked this unpriced positivity obligation.

## Retained packet and exact replay

[extraction.md](extraction.md) owns the full source definitions and producer
write effects; [review.md](review.md) owns independent acceptance and its
inherited-input boundary. [generate.py](generate.py) writes only the new
sibling `cone.json`; [replay.py](replay.py) recomputes its mathematical payload
and compares it without writes; [review_cone.py](review_cone.py) independently
reconstructs the complete source arithmetic and writes nothing. From the
repository root:

```sh
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/outer-matrix-preflight/generate.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/outer-matrix-preflight/replay.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/outer-matrix-preflight/review_cone.py
```

All use only the Python standard library and `__file__`-relative peer inputs.
The observed cone snapshot is 857,478 bytes, SHA-256
`9f02548e305ed586b83b2709719818281847ce2d2e5b66981b35973acc07e9b6`.
The elapsed timing field makes a regenerated whole-file byte hash differ;
read-only replay compares every mathematical field after removing only
`elapsed_seconds`. These are observed runtimes, not universal promises.

The selected preflight is complete before its 02:28 cap. No transfer predicate
remains unresolved. No global general-matrix bound, equality classification,
optimizer, manuscript/dashboard change or additional research is implied.
