# A sharp HKO bound for partners of limited Minkowski asymmetry

Research date: 16 October 2026. This note proves a restricted arbitrary-partner
bound and equality classification. It does not prove global optimality for all
partners. The imported sharp symmetric-partner theorem is
[the maintained proof](../symmetric-partner-lemma.md); external novelty and human
proof acceptance have not been assessed.

Let Q be the circumradius-one regular pentagon whose facet normals are
n_i=(cos(2πi/5),sin(2πi/5)). Write

    h=cos(π/5), w=1+h, B=(Q−Q)^polar/2,
    κ=(10+6√5)/25, ρ_HKO=(3+√5)/5.

The systolic ratio convention is
ρ(K)=c_EHZ(K×_L Q)²/[2 area(K) area(Q)]. All partners are planar compact convex
bodies with nonempty interior. Their Minkowski asymmetry is

    s(K)=min{ s≥1 : −(K−z) ⊆ s(K−z) for some z∈int(K) }.

This quantity is invariant under translations and invertible linear maps. The
minimum exists: for a bounded feasible sequence of s-values the corresponding
centers are bounded, and a limiting center cannot lie on the boundary because
one support denominator would vanish while the opposite width stays positive.
Feasibility is closed at any interior limit.

## Theorem

For every partner K,

    ρ(K) ≤ κ [1+s(K)]/2.                                      (1)

Consequently, if s(K)≤sec(π/5)=√5−1, then

    ρ(K) ≤ ρ_HKO.                                            (2)

Equality in (2) holds precisely for

    K=z+t R_(π/10+kπ/5) Q,    t>0, k∈Z.                     (3)

Thus arbitrary convex partners with at most the regular pentagon's Minkowski
asymmetry satisfy the sharp HKO bound, and their equality partners are exactly
the HKO regular positions. This class includes smooth and arbitrarily many-sided
partners; it is not restricted to a prescribed normal fan.

A partner violating the volume–capacity inequality must satisfy

    s(K)>2/κ−1 = (15√5−29)/4 = 1.13525491562421… .                          (4)

Any partner exceeding HKO must satisfy s(K)>√5−1. These are necessary
conditions, not sufficient conditions.

## Proof of the inequality

Translate a minimizing center to zero and put s=s(K), A=area(K),
S=(K−K)/2. The planar mixed-area identity is

    area(K+tL)=area(K)+2t V(K,L)+t² area(L),   t≥0.

For polygons it follows by summing trapezoid/edge area contributions, and for
convex bodies by polygon approximation and area continuity. Since −K⊆sK,

    K+t(−K) ⊆ (1+st)K.

Comparing the linear coefficients at t=0 gives V(K,−K)≤sA. Hence

    area(S)/A = [A+V(K,−K)]/(2A) ≤ (1+s)/2.                 (5)

The symmetric-partner theorem gives c(S×Q)=W(K) and
c(S×Q)²/[2 area(S)area(Q)]≤κ. Its segment/width upper bound gives
c(K×Q)≤W(K). Combining these statements with (5) proves (1).

Finally,

    κ(1+sec(π/5))/2 = (3+√5)/5,

which proves (2); (4) follows from (1).

## Proof of rigidity

Suppose equality holds in (2). Every inequality in the preceding chain must
be equality. In particular s=sec(π/5)>1, S is a positive homothet of B, and

    V(K,−K)=s area(K).                                      (6)

The symmetric theorem supplies S=tB with its specified orientation; it allows
no arbitrary extra rotation. Thus K−K is a regular decagon in that orientation.
A Minkowski summand of a polygon is a polygon: on every vertex normal cone
of K−K, its support function is linear, and the two convex functions h_K and
h_−K summing to that function must themselves be linear there. Their normal
fans therefore coarsen the decagon's fan. All edge normals of K belong to the
five antipodal pairs of decagon edge normals.

For a polygon centered as above, the mixed-area edge formula is

    V(K,−K)=(1/2) Σ_edges e length(e) h_−K(u_e),
    area(K)=(1/2) Σ_edges e length(e) h_K(u_e),

where u_e is the outward unit normal. Inclusion −K⊆sK makes every summand
s h_K(u_e)−h_−K(u_e) nonnegative, so (6) forces it to vanish at every edge.
K cannot have both u and −u as edge normals: that would require both
h_K(−u)=s h_K(u) and h_K(u)=s h_K(−u), impossible for s>1 and positive supports.

Every edge of the decagon K−K must come from an edge of K or its reflection.
There is therefore exactly one K-edge from each antipodal pair, and every such
edge has the same length as the corresponding decagon edge. Those five lengths
are equal. Let ζ=exp(iπ/5), and choose consecutive representatives of the five
antipodal decagon edge directions. Closure of K gives

    ε_0+ε_1 ζ+ε_2 ζ²+ε_3 ζ³+ε_4 ζ⁴=0,    ε_j∈{−1,1}.

The minimal polynomial of ζ over Q is
Φ_10(x)=x⁴−x³+x²−x+1. Our degree-four polynomial must be a scalar multiple of
Φ_10, so its signs alternate. The selected oriented edge directions are spaced
by 2π/5, and their lengths agree. Consequently K is a regular pentagon. The
specified decagon orientation forces its relative angle to Q to be π/10 modulo
π/5, proving (3).

Conversely a regular pentagon has s=sec(π/5): its centered facet inequalities
show inclusion at that value. If a smaller value had some feasible center,
averaging its five rotated centers would give center zero by convexity of the
feasible-center inequalities, contradicting the centered support ratio 1/h.
The [maintained affine-pentagon capacity theorem](../../pentagon-affine-products/README.md)
at angle π/10 gives
ρ=ρ_HKO. This proves the converse and completes the theorem.

## Relation to existing evidence

The existing symmetric-partner proof already gives
ρ(K)≤κ area(K−K)/[4 area(K)]. The new step is the mixed-area estimate (5),
which turns that bound into a sharp HKO theorem on the explicit class
s(K)≤sec(π/5), with the cyclotomic equality classification above. The scalar
bound in (1) is not claimed sharp at every intermediate s. In particular,
partners with s>sec(π/5) remain outside (2), and neither their existence nor
this upper bound supplies a counterexample.

## Corollary for independent affine factors

For invertible G,H, the symplectic map diag(H^T,H^−1) sends
(GQ)×_L(HQ) to (H^TGQ)×_L Q. Its first factor has Minkowski asymmetry
sec(π/5), by affine invariance. Theorem (2) therefore recovers the sharp ratio
bound for all independent affine images of two regular pentagons. Equality
requires H^TGQ to be a scaled regular pentagon at the HKO relative position,
equivalently the conformal/dihedral condition in the maintained affine theorem.
This gives an alternative proof of its ratio bound and equality statement; it
does not give its exact capacity formula for nonmaximizing affine products.

## Computational controls

[verify_asymmetry.py](verify_asymmetry.py) imports the repository's existing
normal-triangle capacity evaluator and independently solves the Minkowski
asymmetry LP. For K={x:A x≤b}, set y=(s+1)z and solve

    minimize s, subject to A y−b s≤−h_K(−A), s≥1.

These inequalities are precisely −(K−z)⊆s(K−z); translations of K need not
contain the origin. The script also forms K−K from all vertex differences and
checks the mixed-area bound separately from capacity. Twenty retained controls
include regular 3/5/7-gons, the symmetric extremal decagon, a rectangle, one
irregular pentagon, a polygon sampled from a smooth support curve and twelve
seeded random hulls. Every raw input is retained in [controls.json](controls.json).
The script checks known exact regular-polygon asymmetries, HKO/decagon sharp
ratios and affine/translation invariance of its asymmetry LP. These are
floating-point controls, not exact capacity certificates or proofs of (1).

Reproduce from repository root, writing fresh output to scratch:

```sh
uv run --with numpy==2.5.4 --with scipy==1.18.1 python \
  topics/products/pentagon-arbitrary-partner/research-20261016/verify_asymmetry.py \
  --output /tmp/msc-math/hko-20261016/partner/replayed-controls.json
```

The first run used Python 3.13.3, NumPy 2.5.4 and SciPy 1.18.1. All twenty
controls passed; the largest capacity LP primal/dual residual was 5.0×10^−15.
Runtime was 0.38 seconds on the assigned checkout. Timing is an observation,
not a reproduction guarantee. The script overwrites only its explicit output
file and otherwise writes no retained results.
