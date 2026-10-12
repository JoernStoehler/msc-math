# A regular heptagon with a centrally symmetric partner

3 October 2026. Agent-derived analytic argument, independently checked by
`research_choices` for template completeness, placements and constants.
Not human-accepted; no publication-novelty claim or thesis inclusion selected.
The later `odd-regular-symmetric-partner.md` subsumes this case with one
unified argument for all odd regular factors of at least five sides.

This arose in bounded preliminary comparison of the pentagonal calibration
method, not in a substantial approved research programme.

Let `theta=pi/7`, `h=cos(theta)` and

```text
n_i=(cos(2pi i/7),sin(2pi i/7)),
Q={p:n_i.p<=h, 0<=i<7},
B=(Q-Q)^polar/2=conv{±n_i/[2(1+h)]},
W_Q(K)=max{r>=0:r(Q-Q)^polar⊆K-K}.
```

The factor `Q` has circumradius one. Its difference body has the fourteen
facet normals `±n_i` and height `1+h`, giving the displayed polar body.
Thus `B` is a regular fourteen-gon with circumradius
`R=1/[2(1+h)]` and incircle radius
`R cos(theta/2)=1/[4cos(theta/2)]`.

**Claim.** For every centrally symmetric planar convex body `K` with nonempty
interior,

```text
c_EHZ(K ×_L Q)=W_Q(K).
sys(K ×_L Q)<=kappa_7
  = 4(1+cos(pi/7))²/[49 sin(pi/7)sin(2pi/7)] < 1.
```

Equality in the sharp ratio occurs exactly for translates and positive dilates
of the specified oriented `B`. Symplectic changes of coordinates extend the
statement to affinely regular heptagons, with the corresponding inverse-transpose
equality partner, as in `calibration-extension-preliminary.md`.

## Covering input and enumeration

We use [Balitskiy--Mitrofanov--Polyanskii v2](https://arxiv.org/html/2603.12495v2),
Theorems 2.5 and 3.3, Definition 3.2: capacity is at least one if and only if
all oriented normal unit-length triangles translate into the partner. Their
edge vectors follow outward normals and their lengths use the support function
of `Q`. The heptagon has no opposite normals, so no degenerate normal templates.

Three distinct normal directions have a positive closure exactly when all
three cyclic gaps are less than `pi`. The positive integer gaps therefore
sum to seven and are at most three. They are permutations of `(1,3,3)` or
`(2,2,3)`. Each type has seven supports up to rotation, giving fourteen closure
triples. Each triple has two cyclic edge orders, hence twenty-eight template
translation classes. An interior angle is `pi` minus the corresponding angular
gap, so the two triangle shapes have angles

```text
(theta,theta,5theta),    (3theta,3theta,theta).
```

For closing edge vectors `u+v+w=0`, the two cyclic orders have vertex sets
`T={0,u,u+v}` and `T'={0,u,u+w}=u-T`. Because `B=-B`, a translation cover of
one also covers the other. It suffices to cover the two shapes and their
rotations.

## The obtuse shape

Its longest side has length `L`, and its other sides each have length
`L/(2cos theta)`. Every edge's support-function length is `h` times its
Euclidean length, giving `1=hL(1+1/h)=L(1+h)` and `L=2R`.

Translate the longest side's midpoint to zero. Its endpoints are antipodal
vertices of `B`; its apex has distance `R tan(theta)` from zero. The apex lies
inside the incircle since

```text
tan(pi/7)<tan(pi/5)<cos(pi/10)<cos(pi/14).
```

The middle strict inequality follows by squaring:
`5-2sqrt(5)<(5+sqrt(5))/8`. Convexity covers the whole triangle.

## The acute shape

Put `k=sin(theta/2)`. Its equal sides have length `a`, its base has length
`2ak`, and normalization gives `a=1/[2h(1+k)]`. Its circumradius is

```text
rho=a/[2sin(3theta)]
   =1/[4h(1+k)cos(theta/2)],
```

where `3theta=pi/2-theta/2`. Since `h=1-2k²`,

```text
h(1+k)-1=k(1-2k-2k²)>0.
```

Indeed `0<k<sin(pi/10)=(sqrt(5)-1)/4<1/3`, and
`1-2k-2k²>1-2/3-2/9=1/9`. Therefore `rho` is smaller than `B`'s incircle
radius. Translate the circumcenter to zero; the entire triangle lies in
the incircle and hence in `B`.

These covers prove `c(B ×_L Q)>=1`.

## Capacity transfer and sharp ratio

As in the pentagonal argument, closed back-and-forth segments give
`c(K ×_L Q)<=W_Q(K)` for every partner. Here `B-B=(Q-Q)^polar`, so
`W_Q(B)=1` and `c(B ×_L Q)=1`.
For a centered symmetric `K`, `K-K=2K` gives `W_Q(K)B⊆K`.
Monotonicity and one-factor homogeneity yield the reverse capacity inequality.

The same containment gives `area(K)>=W_Q(K)² area(B)`, with equality exactly
for the specified translate and dilate. Finally,

```text
area(B)=7R² sin(theta),
area(Q)=(7/2)sin(2theta),
1/[2 area(B)area(Q)]=kappa_7.
```

To verify the strict bound without numerical capacity evidence, elementary
Taylor inequalities together with `3<pi<22/7` give
`89/100<h<91/100` and `sin(theta)>41/100`.
Using `kappa_7=2(1+h)²/[49h sin²(theta)]` gives
`kappa_7<7296200/7330841<1`.

## Evidence boundary and next useful question

The exploratory LP in `regular-factor-calibration-probe.py/json` suggested this
extension, but the analytic argument above is its mathematical support.
No exact-arithmetic certificate or large computation is needed.
The two-shape heptagon case does not establish the formula for every regular
polygon. A general theorem would require complete treatment of the additional
triangle shapes. Its likely value and literature attribution should be compared
before committing the pending substantial programme.
