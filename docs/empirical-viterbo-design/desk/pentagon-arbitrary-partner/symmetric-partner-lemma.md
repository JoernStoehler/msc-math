# A regular pentagon with a centrally symmetric partner

2026-09-18. Agent-derived proof, independently checked by
`pentagon_partner_research/symmetric_partner_check`; not human-accepted and no
novelty claim. This result arose while interpreting the heterogeneous collection,
not as an inference from its sample statistics.

Fix the regular pentagon Q of circumradius one, with outward facet normals
n_i=(cos(2*pi*i/5),sin(2*pi*i/5)) and facet height h=cos(pi/5).
Write D=Q-Q, w=1+h, and

```
B = (1/2) D^polar = conv{ ±n_i / [2w] : i=0,...,4 }.
W(K) = max{ r >= 0 : r D^polar ⊆ K-K }.
```

B is a regular decagon of circumradius R=1/(2w). W is the independently
available difference-body two-bounce value A2; the argument below only uses
its displayed geometric definition, so it does not depend on acceptance of
the repository's stronger billiard-class identification.

**Claim.** For every centrally symmetric planar convex body K with nonempty
interior,

```
c_EHZ(K ×_L Q) = W(K).
```

Consequently the sharp ratio over this entire partner class is

```
max_K c(K×Q)^2 / [2 area(K) area(Q)]
  = (10 + 6 sqrt(5))/25 = 0.936656314599949...
```

Equality occurs exactly for translates and positive dilates of B, with its
specified orientation relative to Q. This addresses arbitrary symmetric
partners, including smooth ones, not merely symmetric polygons of fixed size.

## Proof

We use the normal-triangle covering criterion in
[Balitskiy–Mitrofanov–Polyanskii, Theorem 3.3 and §6.1](https://arxiv.org/html/2603.12495v1).
For this Q, its ten normal unit-Q-perimeter triangles have angles pi/5, pi/5,
3*pi/5 and are rotations of one another by multiples of pi/5. Capacity is at
least one precisely when all ten translate into the partner.

First, all ten triangles fit into B. Put theta=pi/5. A template with longest
edge length L has two equal remaining edges of length L/(2 cos(theta)). Its
Euclidean perimeter is L(1+1/h). Every oriented edge is parallel to an outward
facet normal of Q, where the support function equals h times Euclidean length.
Thus unit Q-perimeter gives L(1+h)=1, or L=2R.

Translate the midpoint of its longest edge to zero. That edge is parallel to
one of the n_i, so its endpoints are two antipodal vertices of B. The third
vertex is at perpendicular distance R tan(theta) from zero. It is inside the
incircle of B, whose radius is R cos(theta/2):

```
tan(pi/5)^2 = 5 - 2 sqrt(5)
           < (5 + sqrt(5))/8 = cos(pi/10)^2.
```

Convexity places the whole triangle in B. Hence c(B×Q)>=1.

Next, for any K, c(K×Q)<=W(K). A back-and-forth segment with displacement v has
Q-length h_D(v). Such a segment fits into a translate of K exactly when
v∈K-K. The covering characterization of capacity therefore implies that
capacity at least r requires r D^polar ⊆ K-K. Equivalently, one can apply the
unit-length characterization after rescaling K by 1/r. This implication uses
only segments among the closed curves in the source's Theorem 2.5.

For B, B-B=D^polar, so W(B)=1 and c(B×Q)=1. If K is centered and symmetric,
K-K=2K. The definition of W implies W(K)B⊆K. Capacity monotonicity and
one-factor homogeneity now give

```
c(K×Q) >= W(K)c(B×Q) = W(K),
```

proving equality. Translation does not change any of these quantities.

Finally, K contains a translate of W(K)B. Therefore
area(K)>=W(K)^2 area(B), with equality only if K is that translate: proper
inclusion of full-dimensional convex bodies strictly increases area. Since

```
area(B) = 5 R^2 sin(pi/5),
area(Q) = (5/2) sin(2*pi/5),
```

the sharp normalized bound is

```
1/[2 area(B) area(Q)] = 16(1+cos(pi/5))^2/[25sqrt(5)]
                     = (10+6sqrt(5))/25.
```

B attains equality. This completes the argument.

## Interpretation and follow-up consequences

For EVERY convex K, its central symmetrization S=(K-K)/2 has S-S=K-K. The
claim therefore gives the exact interpretation

```
W(K) = c(S×Q),
c(K×Q)/W(K) = fraction of symmetrized capacity retained by K.
```

This converts the two-bounce ceiling into the capacity of a canonically
associated body. It does not make the ratio a new independent capacity or an
invariant under arbitrary symplectomorphisms; the construction uses the chosen
Lagrangian factors.

Along K_t=(1-t)K+t(-K), widths and W stay fixed. Capacity is concave in t and
symmetric about 1/2, and reaches W at t=1/2. If it already equals W at one
parameter t0<=1/2, it equals W throughout [t0,1-t0]. The observations of early
plateaus can thus be interpreted as the disappearance of the asymmetric
triangle-covering obstruction. The existence of the plateau interval as a
possibly degenerate interval is a deduction; the measured location and its
geometric dependence are empirical questions.

There is also a capacity-free necessary asymmetry condition for violating the
volume-capacity inequality. Define

```
delta(K)=area(K-K)/[4 area(K)]-1.
kappa_sym=(10+6sqrt(5))/25.
```

The symmetrization S has area(S)=(1+delta)area(K), and c(K×Q)<=c(S×Q).
Applying the sharp symmetric bound to S gives

```
sys(K×Q) <= kappa_sym [1+delta(K)].
```

Thus sys>1 requires delta>1/kappa_sym-1=0.0676274578121... . This condition
is not sufficient, and sharpness of this delta-dependent bound is not claimed.
It provides a geometric restriction without requiring the capacity of K as an
input. We make no claim that the resulting universal bound after inserting a
worst-case delta improves existing upper bounds.

This is not a theorem that all asymmetric partners have c=W. The retained
collection has strict gaps, including regular-triangle and irregular partners.
Nor does it prove the HKO partner globally optimal among arbitrary partners.
The symmetric sharp value is strictly below one and below HKO.

## Literature and numerical boundaries

The source's §4.5 asks about arbitrary half-symmetric products; it does not state
this fixed-pentagon equality. The narrow independent check found no immediately
subsuming statement. The [focused source check](source-check.md) did not find a duplicating statement
in the immediate cited literature; a publication-level novelty judgment remains
outstanding. The new
independent-affine-pentagon theorem is not an input to this proof.

`universal_symmetric_reference` in `symmetrization-v1.jsonl` retains B's
floating-point geometry, capacity, W and all ten fits. It returns
c=0.9999999999999996, W=0.9999999999999998 and the displayed ratio. These numbers
check the implementation; they are not the proof of the claim.
