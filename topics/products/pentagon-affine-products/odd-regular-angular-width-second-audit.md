# Independent circle-deficit, scalar and application audit

19 October 2026. Reviewer: `/root/partner_geometry`; author: `/root/odd_pair`;
receiving owner: `/root`. Scratch-only bounded independent audit; root owns
retention, integration and cleanup. No child agents or running subprocesses.

Final target: [Rotation profiles for every pair of odd regular polygons](odd-regular-angular-width.md),
inspected at `/tmp/msc-math/research-swarm-20261019/odd-pair/odd-regular-angular-width.md`,
SHA-256 `4b95cadf428b81c37c8b2a776805728a529bf102249b823e0de849b53d31b434`.
The prior candidate core was checked at hash
`c91f9eb800b6842d94bb35320ccb565754182187e03570d2e7345c318c4a867a`;
the assembled final exposition, two precision fixes and new Section 8 were
subsequently inspected directly.

**Verdict: PASS for the assigned circle bound, scalar inequality, complete
downstream case application and sharp systolic corollary.** No material gap found. Section 2's geometric
extremal-angle reduction was assigned to `/root/plateau_check`, not re-audited
here; that independent checker subsequently reported PASS for the candidate core
at its stated hash. This audit directly rechecks its own assigned implications
in the final target; it does not broaden my Section 2 review scope. The combined result remains conditional on the maintained sparse
product/Haim–Kislev formula, four-block regular hull and the stated previously
reviewed pentagon/heptagon/nonagon inputs. It is agent mathematical review,
not human acceptance, a novelty determination or manuscript promotion.

## Circle deficit and normalization

For closed triples, the points `p=(x1,-x3,0)` obey
`2z=sum p_i conjugate(y_i)`. Translating all points by a common center preserves
this identity because `sum y_i=0`. Their three edge lengths are exactly
`|x1|,|x2|,|x3|`. For an obtuse/right triangle, centering the longest side gives
two radii `ell/2` and one radius `d<=ell/2`. The triangle inequality yields

```
2|z| <= (ell/2) S - (ell/2-d) |y_at_inner_vertex|
     <= (S/2)[ell-p(ell-2d)]
```

when `p<=min|yi|/S`. Hence the factor is `S/4` in the displayed bound for
`|z|`, as claimed. For an acute triangle its circumcircle gives
`2|z|<=S Rc`. Relabelling either closed triple preserves the point-triangle
side lengths and partner minimum weight; thus every alternating order is
covered, not merely one attaining order. Independent whole-triple rotations
only multiply `z` by a unit complex number.

Factor exchange is also legitimate explicitly:

```
conjugate(z(X,Y)) = z((y1,y3,y2),(x1,x3,x2)).
```

Thus the smaller-facet factor can be chosen as the point triangle without
losing any coefficient modulus or introducing a factor/sign change.

## Scalar lemma: exact independent arithmetic and analytic signs

Independently reconstructed from the unsquared inequality:
`K=qE/2`. Expanding the cleared squared difference at `t=q+w` reproduces all
five stated coefficients exactly. This was checked with a separately written
SymPy script, `check_scalar.py`, which does not import the author's producer.
Its fresh output `exact-algebra.json` reports PASS. That execution corroborates
identities and fixed rational comparisons; the interval argument was checked
analytically as follows.

`K` is increasing in `t>=q` and its minimum is strictly positive for
`0<q<=23/100`, so squaring has the correct direction. The factored constant
coefficient is positive there. For `f=c1/q`, dropping negative terms in the
derivative and using `372Q+270Q^4<87` gives `f'<-12+q<0`. Exact substitution
returns `f(Q)=50777293721/200000000000>1/4`. The proposed lower bounds for
`c2`, using the decreasing quadratic bracket, are valid despite its negative
large-interval endpoint: multiplying that negative bracket by `q<=Q` gives
the correct lower-bound direction. Its full endpoint is
`-2486740427/10^10>-1/4`; the small-interval bracket at `3/20` is
`69971/160000>0`. Also `c3>81921749/10^8>4/5`, and `c4>0`.

On the larger q interval the combined middle terms are bounded below by
`w(3/80-w/4+4w^2/5)`, whose quadratic discriminant is `-23/400`.
On the smaller interval every coefficient is nonnegative. Thus the cleared
polynomial is nonnegative for all `w>=0`, including the required geometric
subdomain. Concavity of the original difference in `v` correctly reduces its
range `0<=v<=q` to endpoints. No sampled positivity inference is used.

## Acute and parameter bounds

The acute angle-domain closure has extreme points given by permutations of
`(b,pi/2-b,pi/2)` when `b<pi/4`; sine concavity therefore supplies the stated
circumradius bound. Its conversion to `g(q)` has the correct capacity factors.
After positive denominators, `1-3q^2/2-g(q)` has exactly the stated positive
polynomial numerator. The endpoint lower bound after dropping the two positive
higher terms is positive at `Q=23/100`.

The sine/cosine Taylor bounds and monotonicity of
`(x-x^3/7)/(1-x^2/2)` validate `tan(pi/14)<23/100`. The quoted rational value
`100071/436660` is the evaluation at `9/40`, which is a valid strict upper
endpoint after `pi/14<11/49<9/40`.

The minimum side/perimeter calculation is valid for every triangle, and
`delta(a)=v(sqrt(1+v^2)-v)>=v(1-v)` is immediate from `sqrt(1+v^2)>=1`.
Thus the chosen p is always below the relevant partner weight.

## All-odd application and case coverage

For `n>=m>=7`, the parameter domain is correctly
`0<v=tan(pi/(2n))<=q=tan(pi/(2m))<23/100`. A regular closure triangle has
perimeter `sec(pi/j)` because its normalized positive weights sum to one and
each dual normal has Euclidean length `sec(pi/j)`. Its angles are positive
odd multiples of `pi/j`, so the minimum-angle hypotheses apply. The longest
side/perimeter bound is correct: the stated ratio increases with `C/2`, with
derivative proportional to `sin b>0`, and gives `ell_max=s_m` after
normalization.

In the obtuse case, division by `rho=s_n s_m/2` yields precisely
`(1-3v^2/2)/(1-v^2)`. The acute case yields `g(q)/(1-v^2)`. Both are strictly
below `cos(pi/(2n))`; the strict scalar comparison follows from the tangent
lower bound to `(1+x)^(-1/2)` and multiplication by `1-x>0`.
Since `lcm(n,m)>=n`, the resulting disk is contained in the four-block incircle
in the correct direction. Thus all six-block coefficients are strictly inside
that incircle.

Cases with a pentagon factor are exhaustive: 5x5 and 5x7 use the maintained
proved inputs; 5x9 uses the separately reviewed nonagon norm bound plus
`M5=4-sqrt5<16/9`, giving `|z/rho|^2<80/81`, below the correctly normalized
four-block incircle bound `2021/2025`. All odd partners >=11 use
`v<14/97<3/20`, `tan(pi/5)<73/100`, and
`(27/100)(1-v)>(3/2)v`; these yield the same strict radial comparison with
all constants and factor-two normalizations correct. No omitted odd factor
size remains after exchanging factors.

The reciprocal formula `c=1/(2Qmax)` then gives the claimed secant capacity
profile. Strict-containment stability applies separately to each fixed pair
except 5x5; it supplies no common perturbation radius. Angular/radial
containment does not prove independent-affine tensor containment, and the
candidate correctly excludes that stronger conclusion.

## Sharp systolic corollary and closed precision corrections

The new Section 8 is correct. Differentiating
`F(x)=x(1+cos x)^2/[pi sin(2x)]` gives the displayed log derivative; the identity
`cot x+tan(x/2)=csc x` connects its two natural forms. The estimate
`csc x-1/x<tan(x/2)` is equivalent to `cot x<1/x`, valid from `tan x>x` on the
whole required domain. Also `tan x-2tan(x/2)>0` by the double-angle formula.
Consequently f(j) strictly decreases for real j>=5. The area normalization
produces exactly `2f(n)f(m)sec^2 d_L`, and `L>=5` gives the claimed HKO bound.
Strict decrease forces n=m=5 in any equality case; maximal angular distance
then forces theta=pi/10 modulo pi/5. Conversely the retained pentagon profile
attains it. The constant simplifies directly to `(3+sqrt5)/5`; this proves
optimality within the stated rotated odd-regular class, not arbitrary pairs.

Two minor display precision issues found during the final reread are closed:
Eq. (2) now correctly uses an inequality when substituting free p<=m/S (an
equality would require p=m/S), and the m=11 endpoint now reads
`v<=tan(pi/22)` before its strict rational upper bound. Neither changed the
valid downstream argument; both are corrected in the final hashed target.

## Access and limits

Inspected the actual assigned checkout's root guidance, targeted product
sources, maintained `pentagon-affine-products/research.tex` decomposition and
regular-hull normalization, candidate §§1–7 and final assembled §§1–8, the reviewed nonagon note and its
existing independent audit. This was not an exhaustive literature search or
repository-wide implementation validation. The old pentagon/heptagon results
and nonagon step-norm certificate are imported mathematical inputs to this
application; their entire original derivations were not independently rerun
here. No numerical campaign or new domain-source edit was performed for this
critique.
