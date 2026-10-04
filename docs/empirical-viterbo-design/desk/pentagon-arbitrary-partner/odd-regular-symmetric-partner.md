# Odd regular polygons with centrally symmetric partners

3 October 2026 UTC. Agent-derived analytic argument. `research_choices`
independently checked the acute and obtuse triangle placements, the sharp
constant's monotonicity and the assembled implication chain; no material
mathematical error was found.
No human acceptance, thesis inclusion of the generalization, or publication
novelty claim is established. The pentagon instance was selected for inclusion.

## Statement

Let `n>=5` be odd, `theta=pi/n`, `h=cos(theta)`, and

```text
n_i=(cos(2pi i/n),sin(2pi i/n)),
Q={p:n_i.p<=h, 0<=i<n},
B=(Q-Q)^polar/2=conv{±n_i/[2(1+h)]},
W_Q(K)=max{r>=0:r(Q-Q)^polar⊆K-K}.
```

Then every centrally symmetric planar convex body `K` with nonempty interior
satisfies

```text
c_EHZ(K ×_L Q)=W_Q(K),
sys(K ×_L Q)<=kappa_n
 =4(1+cos(pi/n))²/[n² sin(pi/n)sin(2pi/n)] <1.
```

Equality in the sharp ratio holds exactly for translates and positive dilates
of `B`, in its specified orientation relative to `Q`.
The constants strictly decrease along odd `n>=5`, starting at
`kappa_5=(10+6sqrt(5))/25`, and tend to `8/pi²`.
For `Q'=GQ+a`, translation invariance followed by the symplectic map
`(q,p) -> (G^T q,G^-1 p)` gives the same displayed capacity formula with
`Q'`, and the same sharp ratio. The equality partner is
`t+lambda G^-T B`, for `lambda>0`.

## Reference body and imported criterion

The difference body `Q-Q` has facet normals `±n_i`, with support height `1+h`.
Its polar gives the displayed regular `2n`-gon `B`, with circumradius
`R=1/[2(1+h)]` and incircle radius

```text
r=R cos(theta/2)=1/[4cos(theta/2)].
```

The input from [Balitskiy--Mitrofanov--Polyanskii v2](https://arxiv.org/html/2603.12495v2),
Definition 3.2 and Theorems 2.5/3.3, is the translation-cover criterion:
`c(B ×_L Q)>=1` if and only if every normalized normal triangle of `Q`
translates into `B`. Its oriented edge vectors follow outward facet normals,
and their support-function lengths sum to one. For our regular `Q`, each such
length is `h` times the Euclidean edge length, so every template has Euclidean
perimeter `1/h`.

There are no degenerate normal templates, since odd `n` has no opposite normals.
Each interior angle of a nondegenerate template is `pi` minus a cyclic angular
gap between its edge directions. It is therefore a positive odd multiple of
`theta`. In particular, every angle is at least `theta`; none is a right angle.
We cover every possible shape directly, without enumerating the templates.

## Nonobtuse templates fit the incircle

For angles `A,B,C` in `[theta,pi/2]` summing to `pi`, concavity gives

```text
sin A+sin B+sin C >=1+cos theta+sin theta.
```

Indeed the angle-domain polytope has vertices given by permutations of
`(theta,pi/2,pi/2-theta)`, since `theta<=pi/5<pi/4`.
Put `c=cos(theta/2)`, `s=sin(theta/2)`, and `y=sin theta`. Then

```text
h(1+h+sin theta)-2c =2c[h(c+s)-1],
h²(c+s)²-1=y(1-y-y²)>0.
```

The last inequality follows from
`0<y<=sin(pi/5)<(sqrt(5)-1)/2`; the squared difference of the two endpoint
quantities is `(7-3sqrt(5))/8>0`. Therefore the template's circumradius is

```text
rho =1/[2h(sin A+sin B+sin C)] <1/[4cos(theta/2)]=r.
```

Translate its circumcenter to zero. The entire triangle lies in the incircle.

## Obtuse templates: one extreme shape and smaller disks

Let `alpha` be the obtuse angle, with remaining angles `beta,gamma`. Put
`m=(beta+gamma)/2<pi/4` and `d=abs(beta-gamma)/2<=m-theta`.
Both `beta` and `gamma` are odd positive multiples of `theta`, so `m` is
a positive integer multiple of `theta`.

If `m=theta`, the template has angles `(theta,theta,pi-2theta)`.
Its longest side has length `L`, the other sides have length `L/(2h)`,
and normalization gives `1=hL(1+1/h)=L(1+h)`. Thus `L=2R`.
Center this side at zero: its endpoints are antipodal vertices of `B`, since
the side follows a normal direction. Its apex has distance `R tan theta<r`.
For the strict inequality, `s=sin(theta/2)<=sin(pi/10)` and

```text
2s²+2s-1 <=(sqrt(5)-3)/4 <0
```

give `tan theta<cos(theta/2)`. Convexity covers the whole template.

Otherwise `m>=2theta`. The sine rule gives the perimeter-to-longest-side ratio
`1+cos d/cos m`, and

```text
cos d/cos m >=cos(m-theta)/cos m
             =h+tan(m)sin theta >=h+tan(2theta)sin theta.
```

This case implies `theta<pi/8`, so all denominators below are positive.
Consequently

```text
h(1+cos d/cos m) >=h+h²/(2h²-1) >2,
h+h²/(2h²-1)-2 =(h-1)(2h²-h-2)/(2h²-1)>0.
```

Thus the longest side has length `L<1/2`. An obtuse triangle lies inside the
disk having its longest side as diameter, by Thales' theorem. Center that disk
at zero; its radius `L/2<1/4<r` puts the whole template in the incircle.

These cases cover all oriented normal templates. Translation covers depend on
their underlying triangles, so both traversal orders are included.
The imported criterion now gives `c(B ×_L Q)>=1`.

## Capacity transfer and equality

A back-and-forth segment with displacement `v` has support-function length
`s_Q(v)+s_Q(-v)=s_{Q-Q}(v)`. It translates into `K` exactly when `v` lies in
`K-K`. The closed-curve criterion, after rescaling the first factor, gives
`c(K ×_L Q)<=W_Q(K)` for any partner.
For `B`, `B-B=(Q-Q)^polar`, so `W_Q(B)=1` and `c(B ×_L Q)=1`.

Center a symmetric `K`. Then `K-K=2K`, hence `W_Q(K)B⊆K`.
Monotonicity and homogeneity in one factor give the matching lower bound.
The same containment gives `area(K)>=W_Q(K)² area(B)`; equality holds exactly
for the specified translate and dilate, because proper containment of
full-dimensional convex bodies strictly increases area.

Since `area(B)=nR² sin theta` and `area(Q)=(n/2)sin(2theta)`, the resulting
sharp constant is exactly `kappa_n` displayed in the statement.

## Monotonicity and the strict systolic bound

Express the constant as

```text
kappa(theta)=(2/pi²)theta² cot²(theta/2)/cos theta.
(log kappa)'=2/theta-2/sin theta+tan theta.
```

Since `sin theta>theta cos theta`,
`1/sin theta-1/theta<=(1-cos theta)/sin theta=tan(theta/2)`.
Thus `(log kappa)'>=tan theta-2tan(theta/2)>0`.
As `n` increases, `theta=pi/n` decreases, proving the claimed strict decrease.
The pentagon endpoint value is strictly below one, and the limit is `8/pi²`.

## Scope, attribution and implications

The n=3 obstruction in `calibration-extension-preliminary.md` shows that the
reference-body formula does not extend to every odd regular polygon.
Even regular factors are centrally symmetric and belong to the existing
both-symmetric capacity theory; that case is not claimed here as new.
The present argument does not prove the conjectural width formula for two
nonsymmetric odd regular factors in `formal/pentagon-affine-products/research.tex`.

For any asymmetric partner `K`, the theorem applied to its central
symmetrization `S=(K-K)/2` gives `W_Q(K)=c(S ×_L Q)` and
`sys(K ×_L Q)<=kappa_n[1+delta(K)]`, with
`delta(K)=area(K-K)/(4area(K))-1`. Therefore `sys>1` requires
`delta(K)>1/kappa_n-1`; sufficiency or sharpness of this asymmetry-dependent
bound is not claimed.

The calibration persists in an open labeled-polygon neighborhood of each fixed
odd regular factor: `local-calibration-stability.md` proves this using an exact
support identity for the boundary-contact templates and strict-interior margins
for the others. This is a separate agent-checked deduction, not a quantified
perturbation guarantee or a claim about all convex-body neighborhoods.

This result mathematically subsumes the separate pentagon and heptagon
calibrations. It may permit one general method statement with the selected
pentagon result as a corollary. Reader-facing value, manuscript placement and
thesis selection of the generalization remain open. A narrow search and source
comparison do not establish publication novelty.
