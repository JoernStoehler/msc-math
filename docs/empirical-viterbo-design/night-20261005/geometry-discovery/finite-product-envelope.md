# Complete finite support envelope for Lagrangian polygon products

Derived by geometry-discovery lead, 2026-10-04 23:02 UTC. This is a new agent
bridge, not thesis promotion. It applies to fixed normals within an open simple
product support chamber. It resolves a specific premise of the sibling
`../extremality-discovery/conditional-mechanism.md`; that sibling's curvature
and volume-Hessian implications retain their own review obligations.

## Exact construction

Fix each factor's outward normals. Enumerate its positive minimal normal
closure circuits. Every such circuit is a positive vector λ on two or three
facets with `sum λ_i n_i=0`; its ray is unique. These are precisely the support
rays of normalized planar closure vertices. Choice of scale for λ is arbitrary
and fixed for the following formulas. Finite enumeration needs only pairs and
triples, and is independent of the support numbers h.

For a pair of circuits λq,λp, write

```
Hq(h)=sum λq_i hq_i,   Hp(h)=sum λp_j hp_j.
```

Both are positive near the base point. For each cyclic word σ on the combined
circuit supports, the normals-form HK objective has the form

```
Qσ(r λq,s λp)=κσ r s.
```

Same-factor pairings vanish, so κσ is a fixed real constant depending only on
normals, circuits and order. Retain positive κσ. The normalization is
`Hq r+Hp s=1`. Maximizing `κσ r s` gives

```
r=1/(2Hq), s=1/(2Hp),
Qmax=κσ/(4Hq Hp),
Aσ(h)=2 Hq(h) Hp(h)/κσ.                 (1)
```

Thus every branch is an exact quadratic polynomial in h. The existing
six-facet/closure-vertex value theorem applies at **every** support vector in
the chamber. Its sparse candidates have exactly these supports: changing h
rescales a circuit but does not change its support ray. Hence

```
c_EHZ(K(h))=minσ Aσ(h)                 (2)
```

for this single finite, complete, smooth branch list throughout the chamber.
No dense optimizer classification is required. Near the base point, branches
with strictly larger action can be discarded by their finite positive gap.
All tied branches must be kept. Product structure, positive supports and open
chamber hypotheses are essential.

## Why the regularity premise is automatic here

For a sparse pair's support I, the restricted ambient normal-closure space is

```
E_I=span{(λq,0),(0,λp)}.
```

It has dimension two. The normalized tangent space has dimension one. Along
its feasible positive segment, `Q=κσ r s` is strictly concave when κσ>0:
substituting `s=(1-Hq r)/Hp` gives second derivative
`-2κσ Hq/Hp<0`. Thus the strict reduced-Hessian condition in the sibling
curvature-mobility note holds for every branch. Its complete local-envelope
premise follows from (2), including antipodal two-facet circuits and cases
with dense tied HK weights. There is no generic-normal proviso for this
fixed-normal statement.

Subject to acceptance of the sibling volume-Hessian and mobility argument,
its support-spanning consequence now becomes unconditional within this
product support chart:

```
local maximum => span{E_I: active sparse pair/order I}=ker N.        (3)
```

Equivalently the active q-circuit rays span `ker Nq`, and active p-circuit rays
span `ker Np`. These target dimensions are `r-2` and `s-2`. This is stronger
than requiring their supports to cover all facets. It is also not implied
merely by one positive geometric vector lying in their convex hull. Whether
that stronger rank condition gives a genuinely new rejection of an actual
product passing first-order balance remains to be exhibited.

The stronger curvature test is accessible without a matrix inverse. If v is
invisible to every active normalized weight, set
`aq=(λq·vq)/Hq`, `ap=(λp·vp)/Hp`. For each active branch, invisibility means
`aq+ap=0`; by (1),

```
Aσ(h+tv)=c0(1+t aq)(1+t ap)=c0-c0 aq² t².
```

Its mobility is therefore `Mσ(v)=c0 aq²`, exactly. The sibling necessary
condition specializes to

```
max_active aq² >= -D²V[v,v]/(4V0).                                  (4)
```

This provides a concrete geometric curvature comparison for a complete finite
product catalogue. Acceptance of the sibling implication still matters; the
exact branch formula itself does not require that implication.

## A control, not the sought separating witness

For regular circumscribed hexagon q normals at multiples of60° and p triangle
normals at0°,120°,240°, a bounded exact rational field calculation enumerated
all five q circuits and the single p circuit. Dot products are rational
because all normal angles are multiples of60°. All five pairs attain HK
maximum1/12, capacity6, volume18 and ratio1. The q circuits are the three
antipodal pairs plus the two alternating triples. The latter are necessary to
see the full q-closure rank4; retaining only the obvious two-bounce antipodal
pairs would incorrectly report rank3. Thus this actual geometry passes the
rank test and illustrates why sparse ties must be complete.

Scratch derivation: `/tmp/msc-math/geometry-discovery/hex-triangle.py`. It is
not used as a theorem premise. No rank-defect actual geometry has yet been
obtained; this is the sharp missing discovery obligation.

## Different normal charts

A further finite smooth envelope exists when factor normals are allowed to
move **within** their respective Lagrangian planes, provided there are no
antipodal factor pairs and no affine-collinear factor triples at the base
point. All feasible circuits then have three strictly positive barycentric
weights, with invertible3×3 systems. The sign pattern of each triple's weights
and the candidate support collection persist, and all sparse weights and
objectives vary smoothly. This is a product normal-chart extension, not an
ambient four-dimensional normal theorem. Pair births, singular triple systems,
or perturbations destroying product splitting require separate treatment.

This normal-chart observation is secondary to the completely established
fixed-normal quadratic envelope (1)–(2). Do not confuse either statement with
full local maximality of a candidate.
