# Independent audit of the all-dimensional two-paraboloid theorem

Auditor: `/root/squared_body_audit`, 28 October 2026, 23:45 UTC.
Primary candidate inspected:
`/tmp/msc-math/restart/global-connections/two-paraboloid-all-dimensions.md`.
Its Lorentz background in `higher-dimensional.md` was also inspected.

**Verdict: PASS.** The proof establishes, for d≥2 and every bounded
intersection K of two convex solid paraboloids of revolution with a common
focus,

```
Vol_d(K−K)/Vol_d(K) < κ_d,
κ_d = [2^(d+1)(d−1)+2]/d.
```

The constant is the sharp supremum. I found no failure of class preservation,
convexity, normalization, endpoint convergence, volume, or strictness. This
audit is independent of the superseded three-dimensional mixed-volume proof.
No numerical or symbolic computation is needed for this proof. It does not
establish a bound for three or more paraboloids or establish novelty.

## 1. The exact class and its boundedness

Translate the common focus to zero. A convex solid paraboloid of revolution
has the form `|y|+a·y≤c`, where `|a|=1` and c>0. In coordinates along a,
this is `a·y≤c/2−|y_perp|²/(2c)`, whose focus is zero.

If the two axes a₁ and a₂ agree, their intersection is another unbounded
paraboloid. If they differ, the intersection is bounded: on the unit sphere,

```
g(u)=max((1+a₁·u)/c₁,(1+a₂·u)/c₂)
```

is continuous and strictly positive. A common zero would force
`u=−a₁=−a₂`. Therefore every radial length in the intersection is at most
`1/min g`. Both inequalities contain a ball about zero, so K is always
full-dimensional and its volume is positive. This rules out hidden empty,
lower-dimensional, and unbounded cases within the theorem's hypotheses.

## 2. Explicit Lorentz normalization

This argument makes the packet's Lorentz claim self-contained. Let
`β_i=1/c_i`, `S₀=β₁+β₂`, and `S=β₁a₁+β₂a₂`. The two covectors
`p_i=β_i(1,a_i)` are future null for the Lorentz quadratic form
`r²−|y|²`. Their sum is strictly timelike because

```
S₀²−|S|² = 2β₁β₂(1−a₁·a₂)>0.
```

Set `w=S/S₀`, `γ=(1−|w|²)^(-1/2)`, and
`A=I+(γ−1)e eᵀ`, where e is the unit direction of w; take A=I if w=0.
The boost

```
Λ(r,y) = (γ(r+w·y), A y+γw r)
```

preserves the future light cone, and `Λ^(-T)(S₀,S)=(τ,0)`, with
`τ=S₀/γ>0`. Consequently the transformed p₁ and p₂ have opposite spatial
parts. Nullity and future orientation force each temporal part and each
spatial norm to equal τ/2. A spatial rotation R therefore puts their
inequalities into `|z|±z₁≤1` after the common scaling `z=(τ/2)R y'`.
The normalized body is exactly

```
L={ (x,z): |x|≤1/2, |z|²≤1−2|x| }.
```

The inverse boost has spatial part `A y'−γw|y'|`. Writing s=2/τ gives

```
K=s A Rᵀ T_v(L),       v=−Rw,       |v|<1,
T_v(y)=y+|y|v.
```

Here `A⁻¹γw=w` was used. The prefactor is an invertible linear map, so
the difference-volume quotient is unchanged. This proves the claimed
reduction without assuming arbitrary affine maps preserve the confocal
paraboloid class.

Conversely, for every |v|<1, an invertible spatial linear map composed with
T_v is a genuine Lorentz boost restricted to the future light cone. A boost
sends future null defining covectors to future null covectors, with positive
temporal coefficients. Thus its spatial image of L is again an intersection
of exactly two confocal paraboloids. The distinction between that image and
T_v(L), which is only affinely equivalent to it, must be retained.

## 3. The enlarged hinge family is convex

Write v=c a+b with b perpendicular to the lens axis a, and define
`F_{c,b}(y)=y−|x|(ca+b)`. For |c|<1, each of its two halves is an
invertible affine image of a half-lens, and they intersect exactly in
`D={0}×B_(d−1)`. Their first coordinates have opposite signs.

For two points in opposite halves use the packet's representatives

```
Y₊=((1−c)t,z−tb),    |z|≤√(1−2t),
Y₋=(-(1+c)r,w−rb),   |w|≤√(1−2r),
0≤t,r≤1/2.
```

If their segment meets x=0 with weight λ, its transverse norm is at most

```
λ√(1−2t)+(1−λ)√(1−2r)
  +[λt+(1−λ)r]|b|
≤ λ(1−t)+(1−λ)(1−r)+λt+(1−λ)r =1.
```

The only scalar fact is `√(1−2t)≤1−t`, valid on [0,1/2]. Hence the
crossing belongs to D. Each part of the chord belongs to its corresponding
convex half, proving convexity for every `|c|<1, |b|≤1` independently of
`c²+|b|²`. Compact Hausdorff convergence gives both endpoints c=±1.
No three-dimensional property enters this argument.

The enlargement is not an unsupported extension of the reciprocal class:
the proof establishes convexity directly, which is all that the subsequent
shadow argument needs. It never claims these enlarged images themselves
are reciprocal bodies.

## 4. Identification with the radial shadow

Every boundary point of L satisfies `|y|=1−|x|`, so
`T_v(y)=v+F_{c,b}(y)` on ∂L. For |v|<1, T_v is a global homeomorphism:

```
|T_v(y)−T_v(z)| ≥ (1−|v|)|y−z|,
```

and surjectivity follows either from the boost factorization or its
explicit inverse. Its image of L is convex by class preservation and an
invertible affine normalization. Thus its support maxima occur on
`T_v(∂L)`.

On each affine half of F(L), a linear functional reaches a maximum either
on the outer paraboloidal boundary or on the interface. An interface
maximum is also attained on the interface rim, which lies in ∂L. Thus
F(L) has support maxima on `F(∂L)`. The two convex bodies have the same
support function after translation by v and therefore agree. For |v|=1,
the assertion follows by taking interior radial limits. This carefully
avoids assuming T_v remains injective at its endpoint.

## 5. Volume and shadow convexity

On the two half-lenses, F has determinants 1−c and 1+c. Their images
have disjoint interiors for |c|<1 and the original half-volumes are equal.
Hence `Vol(F_{c,b}(L))=Vol(L)` throughout the cylinder, with endpoint
values supplied by continuity or the explicit endpoint description.

For fixed b, varying c moves every indexed point in the single direction a:
its velocity is `−|x|a`. The difference bodies are the convex hulls of
the corresponding pairwise differences, which also move parallel to a.
Their volume is convex in c. This shadow-volume fact admits the same
elementary proof in every dimension: over each fixed point of a⊥, the
right endpoint is a supremum of affine functions of c with a fixed
convex-combination constraint, while the left endpoint is an infimum.
Integrate their convex difference over the fixed projection. The
constant denominator then reduces the quotient to the two endpoints.

The earlier radial family `T_(te)(L)` has the same property, and its
volume is constant because its Jacobian is `1+t e·y/|y|`; the second
term integrates to zero on central L. The origin is a measure-zero
nondifferentiability point and the maps are Lipschitz. Endpoint volume
continuity is legitimate because all relevant bodies remain convex.

## 6. Endpoint caps, their volumes, strictness and sharpness

At c=1 the positive half maps inside D, by
`|z−tb|≤√(1−2t)+t≤1`. The other half is full-dimensional and has
coordinates `u=−2r`, `v=z−rb`. The invertible shear
`z=v−(u/2)b` and translation `U=1+u` identify it with

```
P_d={0≤U≤1, |z|²≤U}.
```

At c=−1 the analogous statement follows by reflection. Both endpoints
are bounded, have positive volume and contain the collapsed half already;
there is no extra lower-dimensional piece affecting convexity or volume.

At positive difference coordinate s, pairs of cap slices with coordinates
U and U−s contribute centered transverse balls of radius
`√U+√(U−s)`. These radii increase with U, so their union is precisely the
ball of radius `1+√(1−s)`. Consequently

```
Vol(P_d)=2ω_(d−1)/(d+1),
Vol(P_d−P_d)=2ω_(d−1)∫₀¹(1+√(1−s))^(d−1)ds.
```

Substituting t=√(1−s) makes the quotient

```
2(d+1)∫₀¹ t(1+t)^(d−1)dt
=2(d+1)[(2^(d+1)−1)/(d+1)−(2^d−1)/d]
=[2^(d+1)(d−1)+2]/d.
```

At the central lens the quotient is 2^d, whereas
`κ_d−2^d=[2^d(d−2)+2]/d>0` for d≥2. The radial shadow therefore
bounds its value at t=|v|<1 by
`(1−t)2^d+tκ_d<κ_d`. This proves strictness for every genuine lens.

For sharpness take v=t a and t↑1. The continuous images T_v(L) converge
to P_d (indeed T_a(L)=P_d exactly). Each t<1 is affinely equivalent to a
genuine bounded pair of confocal paraboloids by the reverse boost above.
Their denominator tends to the positive cap volume, so Hausdorff volume
continuity gives quotient convergence to κ_d. The limiting cap need not
belong to the original class; the theorem asserts a supremum, not equality.

## 7. Consequential simplification of the planar proof

The d=2 case can replace the entire scalar tent-integral calculation in
the existing full planar proof. A tent source has vertices
`±(1,0), ±(a,1)`, hence is a parallelogram; a constant profile gives a
rectangle. Every nondegenerate parallelogram is the intersection of two
symmetric strips. Squaring those strips gives two distinct confocal
parabolas, and squaring commutes with their intersection.

For any such source K, flatten it to `K_ε={(x,εy):(x,y)∈K}`. It remains
a parallelogram. Its square therefore has quotient below 5 by the present
two-paraboloid theorem. After the target scaling `(u,v)↦(u,v/(2ε))`, the
squared image is parameterized on fixed K by `(x²−ε²y²,xy)`, which
converges uniformly to the cap `C_K={(x²,xy):(x,y)∈K}`. Its area is
positive. Thus every generating cap has quotient at most 5.

The already-audited positive tent decomposition, linear cap area and
subadditive difference area now give the bound for every cap, and the
already-audited radial endpoint reduction gives the bound for every
squared planar body. Thin rectangles prove sharpness. No exact tent area,
maximizer classification, rational integral, or logarithmic estimate is
needed in this version of the planar proof.

This simplification alone yields the non-strict bound for limiting tent
caps. If strictness for all full-dimensional planar squared bodies is
desired, the previous audited strict tent calculation remains one route.
Do not infer strictness of a limit solely from strictness of its approximants.
