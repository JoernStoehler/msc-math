# Two characteristic-invariant hinges cannot give positive first variation

29 October 2026. Independently reviewed analytic result; see
`independent-audit.md`. This is an
obstruction to the selected **first-order** two-hinge mechanism. It is not a
local-maximality theorem for twelve-facet bodies and does not cover hinges
that vary along their own characteristic direction.

## Statement

Let H be the HKO product in the coordinates of `../hko-ascent-main.tex`.
On a single labeled facet F_i let

```
f_i(x)=(D_i.x)_+,       D_i.Ja_i=0,
```

and let f vanish on the other facet copies. Let ν be normalized cone-volume,
and R(f)=sup_γ∫f dμ_γ over all finite maximizing-word occupation measures
(or their weak closure). Then

```
                         R(f) >= 2 ν(f).                    (1)
```

Consequently, for two such nonnegative hinges f,g on any two distinct facets,

```
R(f+g) >= max(R(f),R(g))
       >= 2 max(ν(f),ν(g)) >= ν(f)+ν(g).
```

The exact fixed-bank derivative theorem therefore gives

```
D+ log sys(H_t) =4[ν(f)+ν(g)-R(f+g)] <=0.                   (2)
```

All affine offsets and nonnegative speeds are included. If the two cuts are
on the same facet, their actual profile is h=max(f,g), and
R(h)≥max(R(f),R(g))≥2max(ν(f),ν(g))≥ν(f)+ν(g)≥ν(h). Thus the same
nonpositive first-variation conclusion holds. No choice of two arbitrary
characteristic-invariant affine hinge cuts has a strictly positive first
systolic variation. Equation (2) does not decide a zero-derivative direction
at higher order.

## Quotient coordinates and projected volume

By simultaneous pentagon rotations and the symplectic factor exchange
S(q,p)=(J2 p,-J2 q), it suffices to treat the q-facet labeled zero.
Put r=√5, τ=tan(π/5), N=r−1, and e=J2 n_0/N. On that facet use

```
x=(e.q)/sin(π/5),
y=(e.p-m)/h,   m=(1-cos(π/5))/2,  h=(1+cos(π/5))/2.
```

The variables range over [-1,1]². Every characteristic-invariant affine
function on this facet is a x+b y+c: the normal coordinate is constant on
the facet, and the excluded partner coordinate is exactly the characteristic
direction. Under facet-uniform measure, x is uniform on [-1,1], independent
of y. The density of y is

```
k=r−2,             μ=E[y]=−1+2r/5,
p(y)=(r/10)(y+3),                 −1≤y≤k,
p(y)=((5+2r)/10)(1−y),             k≤y≤1.                  (3)
```

Indeed the partner pentagon projects to the three vertex levels −1,k,1;
its transverse slice width is affine between successive levels. The exact
projection and density values, normalization ∫p=1, and mean μ are checked in
`certify_amplification.py`. Each full facet has ν-mass 1/10, so

```
ν(f)=(1/10) E[(a x+b y+c)_+].                              (4)
```

## Four exact shortest-word families

Define

```
A=(5−r)/40,    B=(5+r)/40,    b0=r/10.
```

For each edge sign x=−1 and x=1 there are exact shortest-word families
realizing every y in the following intervals, with the indicated duration
on facet zero:

```
β_L(y)=A(y+3),       −1≤y≤k,
β_U(y)=B(1−y),        k≤y≤1.                               (5)
```

At y=k these are different weights: β_L(k)=b0 and β_U(k)=(5−r)/20.
The upper family has zero facet-zero weight at y=1; endpoint limits are
harmless. The four families are charts 115 and 148 (lower, x=+1 and −1),
and charts 72 and 19 (upper, x=+1 and −1) in the retained complete catalogue.

The new exact verifier reconstructs their endpoint weights from the five
factor-closure vertices, checks closure, nonnegativity and Q=τ/5, and solves
the full KKT system exactly. It checks every facet inequality at every loop
vertex, the constant edge coordinate x=±1, the affine y-range, and the exact
weight identity (5). All these statements hold on the full interval:
weights and reconstructed positions are affine in the parameter, the
objective is quadratic and is checked at both endpoints and the midpoint,
and containment/coordinate identities are affine. Midpoint weights are
strictly positive. No new word enumeration is needed.

Only the existence of these four shortest families is used in the proof.
The 190-chart completeness theorem is not being substituted for an
unverified response bound, and the numerical probes below are not premises.

## A one-dimensional hinge inequality

For h(y)=(b y+c)_+, write

```
I(h)=∫_{−1}^1 h(y)p(y)dy,
M(h)=max{sup_{[-1,k]} A(y+3)h(y), sup_{[k,1]} B(1−y)h(y)}.
```

We prove

```
                         I(h) <= 5 M(h).                  (6)
```

The zero and positive constant hinges are immediate: M(h)≥b0 h and
5b0=√5/2>1. By positive homogeneity, it remains to consider (z−y)_+
and (y−z)_+ for real thresholds z.

### Decreasing hinges h=(z−y)_+

If z≤−1 both sides vanish. If −1≤z≤1, I(z) is convex in z, with
I(−1)=0 and I(1)=1−μ. Its secant bound and the lower-family endpoint y=−1
give

```
I(z) <= (z+1)(1−μ)/2,
M(z) >=2A(z+1),
20A−(1−μ)=(5−r)/10>0.
```

Thus I≤5M throughout this interval.

For z≥1 the hinge is affine on the full support, so I(z)=z−μ. Put
z0=2k+3=2r−1 and z*=1+4r/5. On 1≤z≤z0 the lower-family parameter
y=(z−3)/2 lies in [-1,k], giving M(z)≥A(z+3)²/4. The exact identity

```
(5A/4)(z+3)^2−(z−μ)=(5A/4)(z−z*)^2 >=0                   (7)
```

proves the bound. For z≥z0, use y=k and M(z)≥b0(z−k).
The difference 5b0(z−k)−(z−μ) is increasing because 5b0−1>0,
and is nonnegative at z0 by (7). This covers all decreasing hinges.

### Increasing hinges h=(y−z)_+

If z≥1 both sides vanish. If z≤−1 then I(z)=μ−z and M(z)≥b0(k−z).
The difference 5b0(k−z)−(μ−z) is smallest at z=−1, where it equals
(25−9r)/10>0.

For −1≤z≤k, direct integration of the lower density in (3), or the
identity E[(y−z)_+]=μ−z+E[(z−y)_+], gives

```
I(z)=μ−z+b0[(z+1)^2+(z+1)^3/6].                           (8)
```

On [−1,0], the candidate y=k gives 5M≥5b0(k−z).
The difference of this linear bound and (8) has second derivative
−b0(z+3)<0, hence is concave. Its minimum on the interval is attained at
an endpoint. Its values there are

```
z=−1: (25−9r)/10>0,
z=0:  (210−91r)/60>0.
```

On [0,k], choose the upper-family parameter y=(1+z)/2, which belongs to
[k,1]. This gives 5M≥5B(1−z)²/4. The difference from (8) again is concave:
its second derivative is at most (25−19r)/80<0. The endpoint differences are

```
z=0: (555−233r)/480>0,
z=k: (7/12) B(1−k)^2>0.
```

Finally, if k≤z<1, only the upper density contributes. With
C=(5+2r)/10, one has

```
I(z)=C(1−z)^3/6,
M(z)>=B(1−z)^2/4,
I(z)/M(z)<=2(C/B)(1−z)/3<=8/3<5,
```

because (C/B)(1−k)=4. This completes (6). All displayed scalar signs and
the perfect-square identity are independently recomputed by the exact
checker; the concavity and interval arguments are the analytic part.

## From the scalar inequality to the two-hinge obstruction

Let f(x,y)=(a x+b y+c)_+. Convexity in x gives the endpoint bound

```
E_x f(x,y) <= [f(−1,y)+f(1,y)]/2.
```

For either edge sign, (5) and (6) imply
E_y f(±1,y)≤5M(f(±1,·))≤5R(f), since each candidate in M is the response
of an actual shortest word supported on the selected facet for the test.
Therefore E f≤5R(f). Combining with (4) gives R(f)≥2ν(f).
Facet-transitive symmetries extend the bound to every original facet.
The maximum and nonnegativity argument preceding (2) finishes the proof.

No common occupation ensemble for all hinges is asserted. The responding
member of the four families may depend on the hinge. The factor two is what
allows this direction-dependent response bound to settle two simultaneous
hinges despite that dependence.

## Computation, resource use and remaining scope

The initial absolute-gap search used about48,000 differential-evolution
evaluations across the five facet-pair symmetry types and took36seconds.
Its best values collapsed to small caps, making that objective uninformative.
A subsequent volume-normalized probe found a single-hinge ratio numerically
equal to two and no positive two-hinge gap. Those finite probes motivated
the proof but establish neither it nor nonexistence by themselves.

The exact four-family and scalar-predicate replay takes less than a second
using the existing standard-library field helper and chart data. It writes
only its requested fresh JSON path. `amplification-PASS.json` records the
actual successful run. Independent review approved the analytic inequality,
the exact finite predicates, the complete TeX module, and the same-facet
maximum-profile extension. Root owns paper integration.

This answers the selected first-order discriminator negatively. It leaves
open higher-order improvements in a zero-derivative twelve-facet direction,
two hinges lacking characteristic invariance, and other perturbation classes.
