# Independent P* vertex-cone and cut-stratum review

Reviewer: `conditional_mechanism`. Receiving owner: `extremality_discovery_lead`.
Review started 2026-10-04 23:07 UTC; first mathematical return 23:14 UTC.
Agent review, not Jörn acceptance, formalization, or thesis promotion.

## Initial verdict

**Accept the full interior vertex-cone theorem and the 24 barycentric windows**
in `../nonproduct-mechanisms.md`, for the fixed rational
base body P*. Every fixed positive cone direction has a sufficiently shallow
cut with ratio strictly below one. The common `ε ≤ 1/100` window is accepted
only for the 24 barycentric directions.

All 24 retained witnesses are checked directly. There is no remaining reliance
on “21 direct plus 3 by symmetry.” The older report wording should be updated
by the integrating owner.

The initial proposed ridge-volume bridge had coefficient gaps in the original
retained bank. At 23:16 UTC the lead's constructive adjacent-word replacement
was independently checked and accepted; it resolves those gaps for the base
body. The findings and their resolution are recorded below. The simpler
strict-capacity/cuts-extremizer argument does not depend on ridge volume.

## Evidence checked

Ran the retained exact verifier successfully:

```text
PASS: 24 full interior vertex-normal cones; exact feasible branches and positive curvature polynomials.
PASS: every barycentric cut has rho < 1 for 0 < epsilon <= 1/100.
```

Independent scratch check:
`/tmp/msc-math/pstar-independent-vertex-review/audit.py`. It enumerated all
210 four-row intersections independently, recovered exactly the 24 Cartesian
product vertices, recomputed factor areas `1` and `1/2`, expanded the HK score
by its direct pair sum, and checked the simplex cap determinant independently.
This check passed. Its reviewed packet hash is

```text
cf80ae97ce88a9e644880a8e8911795922eb16cb5d09699ec5904084bcf35e79
```

The barycentric `C` values range from `11/14` to `219/14`. In particular the
action denominator is positive at `ε = 1/100`. That guard was initially missing
from the source verifier and established independently here; the source owner
subsequently added an explicit assertion. The six absolute
incident-normal determinants are
`512/49, 1024/49, 1536/49, 128/7, 256/7, 384/7`.
The base capacity `1` is inherited from the existing exact P* proof; this
review did not rerun its full equality-family certificate.

## Continuum feasibility and action

For each retained word, `b⁰` is normalized, closed, and nonnegative; its new
coordinate is zero. The symbolic direction satisfies closure and total mass
zero on `Σ t_i = 1`, with new coordinate one. Every other zero-base coordinate
has a nonnegative linear polynomial (possibly identically zero), so it stays
nonnegative for `δ ≥ 0` throughout the closed coefficient simplex. Positive
base coordinates stay positive for sufficiently small `δ`; for each fixed
`t` this is an actual feasible section, not merely a first derivative.

The symbolic identities give

\[
 Q(b^0+\delta d)=\tfrac12-C(t)\delta^2.
\]

Each retained `C` is homogeneous quadratic with nonnegative coefficients and
at least one positive coefficient. Hence `C > 0` on the open simplex and
`C ≥ 0` on its boundary. With `δ = ε/(2C)` and small enough `ε`, both weights
and positive action denominator are valid, and

\[
 c(P_{v,t}(\varepsilon))
 \leq\frac{(1-\varepsilon\delta)^2}{1-2C\delta^2}
 =1-\frac{\varepsilon^2}{2C}.
\]

The direction of the bound is correct: a single feasible HK datum gives an
upper bound for capacity. No complete eleven-facet minimizing catalogue is
needed. The project word orientation is used consistently in the verifier and
the independent direct pair-sum check.

## Geometry and the strict ratio inequality

At a simple vertex, the incident matrix `N` is invertible. The local slack
map `y = 1 − Nx` has Jacobian absolute value `1/|det N|`. Positive cone
coefficients expose only that vertex, so a sufficiently shallow cut removes
exactly

\[
 \{y\geq0:\sum_i t_iy_i<\varepsilon\},\qquad
 \Delta V=\frac{\varepsilon^4}{24|\det N|\prod_i t_i}.
\]

There is no missing unit-normal factor: `N` consists of the actual row
normals, and its determinant accounts for their scales. Nonincident facets
have positive slack at the vertex. For fixed `t > 0`, all cap corners
`v − (ε/t_i)N⁻¹e_i` remain inside the old body for sufficiently small `ε`.
All old facets remain genuine and the new facet is a genuine simplex facet.
Its normal has nonzero q and p components, so it is not a facet of the
original q/p product presentation.

Writing `D = 1/(24|det N|∏t_i)`, the strict ratio estimate is equivalent to

\[
 \varepsilon^2<\frac{4C}{1+8DC^2},
\]

in addition to denominator positivity and feasibility. This is a strictly
positive threshold for every fixed open-simplex direction. For barycentric
directions, endpoint feasibility at `1/100` implies feasibility for all
smaller ε by convexity; the strict inequality above is monotone in ε². Thus
checking its endpoint proves the advertised entire interval, not only one
sample depth.

On any compact set `t_i ≥ η > 0`, continuity and the finite witness list yield
a common small ratio radius. Neither this argument nor the barycentric
interval supplies a radius uniform as `t_i → 0`, while D blows up and the
exposed-face/cap stratum changes. Moving old normals, coupled equality-family
motions, and multiple simultaneous cuts are outside the accepted base theorem.

## Consequential ridge-probe gaps identified during review

The initial scratch `ridge_probe.py` selected the smallest value of `C` after
setting one q coefficient and one p coefficient equal to one, then treated
that number as the coefficient of `t_q t_p`. That evaluation is insufficient:
it also includes pure q or p squares. Two selected witnesses have such extra
terms:

| Mixed ridge facet pair | Selected vertex | Actual restricted C(a,b) |
| --- | --- | --- |
| `(0,8)` | `[0,3]` | `4ab + 8b²` |
| `(5,8)` | `[0,3]` | `(72/7)ab + 8b²` |

None of the four endpoint witnesses for either of these pairs has a purely
bilinear restricted C. Also the pure-p ridge `(8,9)` has no zero-C witness
among its six endpoint vertices. Other pure-factor ridges do admit a zero-C
choice. Therefore the initial original-bank assertion “all mixed ridges have
C = k t(1−t), and all pure ridges have C = 0” is not accepted as written.
Alternate witnesses or an actual direction-dependent coefficient comparison
are needed for that stronger **ratio** argument. The lead was notified with
the exact counterchecks at 23:11 UTC.

### Accepted resolution: adjacent-word surgery, 23:16 UTC

Deleting zero-weight labels from the 24 retained base words and identifying
cyclic rotations recovers exactly 12 primitive tight cyclic words. An
independent exact check of their cyclic adjacent pairs found positive mixed
symplectic adjacencies covering **all 24** mixed q/p facet pairs. For every
such positive adjacency, with facet slice densities
\(L_a=\operatorname{length}(e_a)/\|a\|\), the values of
\(\omega(a,b)L_aL_b\) belong to

\[
 \{1/8,\ 5/24,\ 7/24,\ 3/8,\ 5/8\}.
\]

This was computed directly from the packet's row normals and polygon vertices,
not by reusing the old ridge-probe evaluation.

For a positive adjacent pair `(a,b)`, insert the new normal
`w = t a + (1−t)b` between them and change weights by
`d_a = −t`, `d_b = −(1−t)`, `d_new = 1`, with all other `d` zero. The total
weighted vector of this block is unchanged, so all interactions with other
word positions remain unchanged. Expanding just the three internal pair
terms gives exactly

\[
 Q=\tfrac12-t(1-t)\omega(a,b)\delta^2.
\]

All old weights in the primitive word are strictly positive, so this is a
feasible section for small positive δ. This construction supplies the needed
purely bilinear C without inheriting the extra pure-p-square terms found in
the original bank.

For a mixed ridge cut, slack slice densities of the two factors are
`L_a + O(y)` and `L_b + O(z)`. Integration over
`t y + (1−t)z < ε` gives

\[
 \Delta V=\frac{L_aL_b}{2t(1-t)}\varepsilon^2+O(\varepsilon^3).
\]

The ratio coefficient is consequently

\[
 \frac1{t(1-t)}\left(L_aL_b-\frac1{\omega(a,b)}\right)<0,
\]

because every checked density/symplectic product is at most `5/8 < 1`.
This proves strict ratio reduction for every fixed mixed-ridge direction.

For a pure-factor ridge, the strict-capacity bridge below is sufficient
without a zero-C witness. After adding its new normal, the fixed-normal
product capacity envelope consists of finitely many branches affine in ε:
only one factor support vector changes. It is exact for small positive ε,
and continuity extends its scalar value to `c(0) = 1`. Finite gaps therefore
give `c(ε) = 1 + ε min(active slopes)` for sufficiently small positive ε.
Since the closed-cone witness already gives strict capacity reduction, that
active slope must be strictly negative. The resulting linear capacity loss
dominates the quadratic pure-factor ridge cap volume. Thus the pure-p
`(8,9)` gap in the original zero-C claim is also resolved.

The accepted repair establishes fixed-direction base-body ratio reduction;
it does not produce a uniform radius over directions approaching different
face strata, or automatically continue all these ridges to the equality family.

## Simpler strict-capacity bridge, independent of ridge volume

For any fixed nonfacet support direction, choose a vertex of the exposed face
and express its supporting normal by nonnegative incident coefficients with
sum one. The retained identities and zero-base-coordinate positivity extend
to this closed simplex. The corresponding `C` is nonnegative.

- If `C > 0`, the same optimized choice `δ = ε/(2C)` gives `c < 1` for a
  sufficiently shallow cut, regardless of the volume-loss order.
- If `C = 0`, choose any fixed sufficiently small positive feasible `δ`.
  Then `Q = 1/2` and the action bound is `(1 − εδ)² < 1` for small ε.

For each fixed direction supported on at least two old facets, all old facets
remain genuine at sufficiently shallow depth, so the facet-form HK bound is
applicable. A facet direction instead duplicates one old normal and makes the
old inequality redundant. Handle that case using the existing fixed-ten-facet
local maximum: the shallow shifted facet has smaller volume and ratio at
most one, hence `c² ≤ 2V < 1`. Using the eleven-row facet formula without
removing the redundant row would require an additional dual-action argument.

Once strict capacity reduction is known for one shallow depth in each fixed
direction, monotonicity extends it to every deeper cut in that direction.
Apply the argument to both opposite normal directions to cover both pieces
of any hyperplane intersecting the interior. This establishes the
**cuts-extremizer** property, in the sense of
[Haim–Kislev, Definition 1.2](https://arxiv.org/html/2511.16644v1), subject to
the accepted base witnesses and existing F10 theorem. It does not establish
cuts additivity or a ratio bound for all cut depths.

Bounded witness coefficients alone can give a uniform algebraic
strict-capacity estimate by choosing `δ = kε` with
`k = 1/[2(C_max + 1)]`; for `ε ≤ 1`, the numerator-minus-denominator is at
most `−kε²`. However directions approaching facet boundaries can make old
inequalities redundant at a fixed depth. A uniform facet-form implementation
therefore still needs a geometric validity radius or a separate proof that
the same upper bound permits redundant normals. No such unrestricted uniform
boundary statement is accepted in this review.

## Equality-family continuation reviewed at 23:19 UTC

Ran `vertex-cones/verify-family.py` successfully. It checks all 24 closure and
quadratic identities over `QQ(v,s)` and all coefficient sign-stability
predicates, including identities for coefficients zero at the base. A separate
review check inspected all **1314** retained rational expressions in the base
weights, direction coefficients, and C coefficients and found no denominator
vanishing at `(v,s) = (1/2,1/12)`. This includes positive-base-weight direction
coefficients, whose individual denominators were not all explicitly guarded
in the original verifier. Reviewed family-packet hash:

```text
4260a46f68965757d67f37bf923c8b0c23a870603fc63d8f5a5363effd479065
```

**Accept the claimed family vertex continuation.** Finite regular rational
functions, strictly positive base coefficients, and identically zero base-zero
coefficients supply one common small shape-parameter patch. On it, all old
zero-base coordinates remain nonnegative and each C remains a nonzero
nonnegative homogeneous quadratic. The existing F10 proof supplies exact
family capacity one; the checker recomputes the constant factor areas. The
vertex cap geometry persists by simplicity. Therefore for every family point
in that patch and every fixed interior cone direction there is a sufficiently
small cut depth with strict ratio reduction.

This is a common **shape-parameter** neighborhood followed by a per-direction
depth quantifier, not a depth uniform over changing directions. All four
q/p cross monomials of C are strictly positive at the base and remain so on a
common patch; this also validates the stated codimension-three edge interface.
The base mixed-ridge surgery is accepted separately above. Its possible
continuation by strict margins was communicated as a route, not accepted here
as an already retained family ridge certificate.

## Final acceptance of the fixed-cut source

Read `fixed-cut-strata.md` after the constructive correction was retained.
**Accept all three statements on its stated domains:** strict ratio reduction
for sufficiently shallow fixed directions exposing codimension at least two,
ratio at most one for shallow facet-parallel cuts, and strict capacity reduction
of both pieces of every interior hyperplane cut (cuts extremizer).

The accepted bridge consists of the vertex theorem, the closed-cone capacity
response, the codimension-three volume order, the corrected mixed adjacency
construction with strict margin, the pure-factor finite affine envelope, the
existing F10 facet argument, and monotonicity for deeper cuts. The source
correctly discards its first flawed ridge probe and distinguishes strict
capacity from the ratio at arbitrary cut depths.

No full F11 Hausdorff-neighborhood theorem, moving-old-normal theorem,
coupled-direction ratio bound, cuts-additivity theorem, or symplectic-ball
classification is accepted or implied by this review. The initial numerical
ridge gaps are resolved by a new exact construction, not by loosening their
predicates. Review stops for integration at this completed bounded return.
