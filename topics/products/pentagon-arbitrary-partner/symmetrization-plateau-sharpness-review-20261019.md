# Independent audit: sharp universal pentagon plateau

19 October 2026. Reviewer `/root/plateau_check`; author
`/root/partner_geometry`; integration owner `/root`.

**Verdict: PASS.** The explicit trapezoid makes the guaranteed universal
interval optimal. This supplements the reviewer's independent PASS of the
universal constructive plateau proof. Both conclusions use the imported BMP
translation-cover criterion. Human proof acceptance and external novelty are
not established by this review.

The assembled target subsequently read was
`topics/products/pentagon-arbitrary-partner/symmetrization-plateau-20261019.md`,
from its status and exact statement through its complete section “Sharpness:
one explicit trapezoid”, at SHA-256
`a07ecd43c6015f4b5215d66f1b05b43d39e41e53446f1f64e9d6ceed25157be9`.
The stronger sharpness status and the complete inserted argument also PASS.
This addendum does not extend review to unrelated later sections.

## Setting

Fix the circumradius-one regular pentagon with outward unit facet normals
n_i=(cos(2pi i/5),sin(2pi i/5)). Write

```
h=cos(pi/5), w=1+h, R=1/(2w),
C=conv{±n_i}, B=R C=(Q−Q)^polar/2,
eta=tan(pi/5)/cos(pi/10)=3−sqrt(5).
```

The independently checked universal construction gives capacity W_Q(K) for
all K whenever eta/2<=t<=1−eta/2, along K_t=(1−t)K+t(−K).

Define the asymmetric trapezoid

```
u=sin(pi/10), c=cos(pi/10), a=(3−sqrt(5))/2=1−2u,
K=conv{(−1,0),(1,0),(a,−2c),(−a,−2c)}.
```

It has nonempty interior, 0<a<1, and unique x-support points (±1,0).

## Exact difference-body containment and width

Put q=(1−a)/(2c)=u/c=tan(pi/10). The union normal fan of K and −K has
vertical normals and the four normals (±1,±q). Direct support-function sums
give the exact central symmetrization

```
S=(K−K)/2={ (x,y): |x+qy|<=1, |x−qy|<=1, |y|<=c }.
```

For example h_K(1,−q)=1, since both the upper-right and lower-right vertices
give value one, and h_K(−1,q)=1 at the upper-left vertex. The other sloped
directions follow by horizontal symmetry and sign exchange. The vertical width
of K is 2c, giving |y|<=c in S. Its vertices are
(±1,0),(±(1+a)/2,±c), with independent signs in the second group.

Every displayed inequality is a supporting inequality of the regular decagon
C: its facet lines adjacent to the x-axis are x±tan(pi/10)y=1 and their
negatives, and its top/bottom facet lines are y=±cos(pi/10). Equivalently,

```
cos(pi/5)+tan(pi/10) sin(pi/5)=1,
sin(2pi/5)=cos(pi/10).
```

Hence C⊆S. Conversely S has x-support exactly one and C contains (±1,0),
so rC⊆S requires r<=1. Thus the largest centered decagon scale is exactly
one. Since B=R C, the definition of W_Q gives W_Q(K) R=1, or
W_Q(K)=2w. Translation of C cannot enlarge this scale in symmetric S;
the definition itself uses centered inclusion.

## Obstruction outside the interval

For every t∈[0,1], the exposed face of K_t in the positive x direction is

```
(1−t){(1,0)}+t{(1,0)}={(1,0)},
```

and similarly the negative x face is {(−1,0)}. This uses the unique exposed
points of both K and −K, and holds at the endpoints as well.

One W-scaled normal template has vertices

```
(−1,0), (1,0), (0,tau), where tau=tan(pi/5).
```

Its directed edges can be chosen as 2n_0,n_2/h,n_3/h, so they follow outward
facet normals and have total Q-length 2h+1+1=2w=W. The downward-apex version
is obtained by the other traversal order and is equally a required template.

Any translation of the upward template fitting K_t has a horizontal base of
length two. The x-support bounds force its endpoints to have x-coordinates
−1 and 1; uniqueness of the two exposed points then forces both y-coordinates
to be zero. Thus the translation must be zero. Its apex can fit only if

```
tau <= h_{K_t}(e_y)=2tc,
```

or t>=tau/(2c)=eta/2. The downward template analogously requires
tau<=2(1−t)c, or t<=1−eta/2. A failed W-scaled template gives c(K_t×_L Q)<W
by the covering criterion; the segment ceiling holds for all t.

The universal constructive proof supplies the reverse implication throughout
the closed interval. Consequently this single explicit trapezoid has exactly
the equality set

```
{t∈[0,1]: c(K_t×_L Q)=W_Q(K)}=[eta/2,1−eta/2].
```

No parameter outside this interval can belong to a larger universal guaranteed
saturation set. This proves optimality of the universal interval, including
both endpoints; it does not classify all partners with the same onset.

## Scope and process

This was an independent analytic check of the author's exact candidate,
including normal-template orientation, difference-body facets, containment,
scale, extremal-chord necessity, and both parameter directions. It requires no
floating-point LP or new computation. BMP v2 Definition 3.2, Theorems 2.5/3.3
and Section 6.1 were checked directly in this reviewer's earlier plateau task.
No exhaustive literature search or human review was performed.

The retained read-only verifier
`verify-sharp-symmetrization-threshold-20261019.py` was separately inspected.
It correctly reuses the ordered field helper, constructs the actual difference
hull, checks ten decagon vertices by hull-edge orientation, checks horizontal
support/contact and the exact threshold/template identities, and guards against
disabled assertions. An independent offline rerun with a 45-second timeout
completed in 0.45 seconds, exit 0. Its checked SHA-256 was
`fa1b57dfd7c1639cc33e2936e47154e5ff616fc97da6a145c4b691a581babba7`.
That arithmetic check supplements the analytic proof; it does not independently
implement the imported field helper.

No project files or children were created by this reviewer, and no process
remains active. Parent owns integration and cleanup; this scratch audit is the
only checker-authored output for this subtask.
