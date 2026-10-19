# Independent extension audit — 19 October 2026

Reviewer: symmetric_writeup. Read-only review of
`topics/products/pentagon-arbitrary-partner/symmetrization-plateau-20261019.md`,
SHA-256 `75488edf7da505bcf1f4c27758106acc11d9f7fdf2ddc187342d2007d2ecc00e`.
The earlier scratch draft was also inspected; the retained target was reread
after corrections.
No project edits, producers or numerical checks were performed by this reviewer.
The pre-existing pentagon plateau argument was assigned to another checker;
this audit concerns the four added consequences.

## Verdict

PASS for the four added consequences in the exact retained target above.
Two precision corrections were communicated to partner_geometry and verified
in the retained target: explicitly require `0≤η<1`, and distinguish
whole-template strict gauges from extreme-template apex gauges in the odd-n
application. The fixed compact dual-polytope argument was also incorporated,
making finite piecewise-affine coverage explicit.
No claim of human acceptance or external novelty follows from this audit.

## 1. Finite-template transfer lemma

For `0≤t≤1/2`, `s=1−2t` and `S=(K−K)/2`, one has exactly
`K_t=sK+(1−s)S`. If `B⊆S` and `η≤2t=1−s`, then
`ηB⊆(1−s)S`, using `0∈S` and nonnegative scales.
For a first-type template `T⊆ηB`, any `m∈K` gives
`sm+T⊆sK+(1−s)S`. For the second type, `d∈2B⊆K−K` permits
`a,b∈K` with `a−b=d`; the draft's exact endpoint formulas and apex
containment give the required fitting translation. Every template may use
its own translation. For the upper half, apply the same argument to `−K`
at `1−t`, whose interpolation body is `K_t`.

Resolved domain precision: the retained hypothesis is `0≤η<1`. A negative η
would invalidate the dilation containment and interval statement. In the
intended applications the constructed η is positive.

## 2. Every fixed odd n≥5

Read the actual placements in
`topics/products/pentagon-arbitrary-partner/odd-regular-symmetric-partner.md`:

* Every nonobtuse template is centered in a disk of radius `rho<r`, where
  `r` is the incircle radius of B. It is therefore contained in `η_j B`
  for some `η_j<1` (for example `rho/r`).
* Each non-extreme obtuse template lies in a centered diameter disk with
  radius `<1/4<r`, giving the same first-type conclusion.
* Each extreme template has endpoints at antipodal vertices of B and an
  apex whose distance from zero is `R tan(theta)<r`. Its apex, rather than
  the whole template, has B-gauge strictly below one. It is second type.

There are finitely many direction triples and orders for fixed n, so the
maximum of these finitely many whole-template/apex gauges is `<1`.
The generic lemma therefore gives a uniform nonzero interval for every K,
for that fixed n. Scaling by W, the complete-template criterion and the
unchanged difference-body upper bound turn the covering result into capacity
saturation exactly as in the pentagon argument. No uniform-in-n margin,
explicit best constant or optimal onset is established here.

Resolved precision: the retained target explicitly separates these two groups;
the extreme whole-template gauges are exactly1.
The same reasoning applies to each factor Q in the local-calibration note:
its non-extreme placements are strict, while its exact antipodal-side
placements have strict apex margins. This is relative to that maintained
proof's finite labeled-polygon neighborhood, not an all-convex-bodies result.

## 3. Area and systolic variation

Writing mixed area as V, area(K_t) is
`[(1−t)^2+t^2]A+2t(1−t)V(K,−K)`. Since
`area(K−K)=2A+2V(K,−K)`, the displayed formula follows.
Brunn–Minkowski gives `D=area(K−K)−4A≥0`; equality says K and −K are
positively homothetic. Equal area makes that homothety a translation,
which is equivalent to central symmetry of K up to translation.
Thus D>0 exactly in the stated nonsymmetric case. On the plateau the
positive capacity is constant and area derivative is `D(1−2t)`, proving
strict ratio decrease to1/2 and strict increase afterward. The midpoint is
a strict pathwise minimum, and no interior point of the guaranteed plateau
is a local ratio maximum. The endpoints can still be candidates.

## 4. Polygon capacity and breakpoint maximum reduction

The union of the normal fans of K and −K is a finite complete normal fan
for every strict Minkowski mixture. At t=0 or1 it still describes the
endpoint body with redundant inequalities. For each normal A_i, the actual
support is `b_i(t)=(1−t)s_K(A_i)+t s_K(−A_i)`, an affine function.

For a fixed nondegenerate normal template T_j, translate it so0 lies in its
interior. Its support coefficients `s_ij=max_{v∈T_j} A_i·v` are then all
strictly positive. Its fitting LP dual is the fixed polytope

    P_j={y≥0:A^T y=0, s_j·y=1}.

It is compact because every coefficient of s_j is positive. It is nonempty
by the finite positive optimal fit and LP duality; translation variables
are free and the positive optimal scale makes the scale constraint tight.
It consequently has finitely many vertices, and

    r_j(t)=min_{y∈vertices(P_j)} b(t)·y.

This is a finite minimum of affine functions, including degeneracies and
redundant fan rows. Taking the minimum over all ten complete templates gives
the claimed finite concave piecewise-affine capacity. This argument concerns
the mathematical LP, not the floating-point evaluator's reliability.

Reflection permutes the complete template family: for closing edge vectors
u+v+w=0, switching the cyclic edge order takes
`{0,u,u+v}` to `{0,u,u+w}=u−{0,u,u+v}`. Hence
`c(−K×Q)=c(K×Q)`, and `K_(1−t)=−K_t` gives reflection symmetry along the
path. Equivalently, the source charter already records this permutation.
Concavity also follows directly by interpolating fitting translations/scales
for each template, then taking their minimum. Thus any affine piece on
`[0,1/2]` has slope b≥0. Its affine extension is a concave supporting line,
so intercept `a≥c(0)>0`. Positivity of c(0) follows from K's nonempty interior.

For `f=(a+bt)^2/[A0+D t(1−t)]`, factoring its derivative leaves the positive
factor `(a+bt)/[A0+D t(1−t)]²` times

    L(t)=2bA0−aD+D(b+2a)t.

If K is nonsymmetric, `D>0`, and `L'=D(b+2a)>0`. Thus an interior stationary
point is a strict local minimum; there is no local maximum within any
positive-length affine piece. On the finite closed interval, maxima therefore
occur at endpoints or breakpoints. The already proved nondegenerate plateau
makes1/2 a strict local minimum, so the maximum on `[0,1/2]` is attained at0
or a capacity breakpoint. If K is symmetric, the entire path is a translated
copy of K and its constant ratio is already attained at0.

This proves only the stated finite pathwise reduction. It supplies neither
an exact breakpoint-enumeration implementation nor a global shape reduction.
