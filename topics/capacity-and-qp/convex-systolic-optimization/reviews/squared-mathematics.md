# Independent assembled audit: sharp squared-body area bound

Auditor: `squared_body_audit`, 28 October 2026, assigned restart worktree.
Audited sources: `/tmp/msc-math/restart/global-connections/reduction.tex` and
`/tmp/msc-math/restart/value-review/tent-proof.md`, as read at 22:56 UTC.

**Conclusion:** the planar theorem and the claimed sharp constant pass this
independent mathematical audit. I found no missing geometric implication or
incorrect maximizer branch. I independently derived the reductions' key
identities, all three maximizer patterns, and a Laurent primitive; exact
polynomial checks then confirmed the endpoint simplifications. This audit
does not establish literature novelty or the four-dimensional transfer.

## Geometric implications

1. Squaring commutes with intersections of sets invariant under negation:
   a nonzero image point has exactly the two opposite roots, so its membership
   in every image implies either root lies in every source. The origin has
   its single root and causes no exception.
2. For a strip `|Re(e^{-iθ/2}z)|≤1/√r`, its squared image is exactly
   `u≤1/r−rv²/4` in target coordinates rotated by θ. Indeed its condition is
   `|y|+u≤2/r`, and squaring this inequality with the correct nonnegative
   right side gives the displayed parabola. Thus squared images of compact
   origin-symmetric convex bodies are convex. They contain a neighborhood
   of zero because their sources contain a disk.
3. The reciprocal representation can also be established directly. For
   `0∈M`, in evaluating `max_n h_M(n)<y,n>` restrict to `<y,n>≥0`, since
   `h_M≥0`. Exchange the maxima over n and x∈M. For each x, the maximum of
   `<x,n><y,n>` is `(|x||y|+<x,y>)/2`, and choosing the sign of n makes
   `<y,n>≥0`. Consequently the reciprocal is the intersection of exactly
   the parabolic regions above. This independently checks the imported
   representation. Applied to the supporting strips of K, whose parameters
   trace the squared boundary of K°, it proves `(σK)'=σ(K°)`.
4. The source stretch identity is exact. The target normalization is
   invertible only for `|t|<1`, which is sufficient: uniform convergence of
   `T_t` on the compact image gives the two endpoint caps and their convexity.
   The map preserves horizontal lines and is strictly increasing on each
   such line. Its area is `A+tB`; the nondifferentiable point at zero has
   measure zero, and the map is Lipschitz. Since the body has interior,
   `|B|<A`, so the endpoint areas remain positive.
5. The elementary shadow argument is valid, including the infinite indexing
   set: at each fixed height the feasible finite convex-combination weights
   are independent of t. Their rightmost x-coordinates are affine functions
   of t; their supremum is convex. The analogous left endpoint is concave.
   Integrating widths over the fixed vertical projection gives convex area.
   Division by the positive affine area therefore bounds the ratio at zero
   by the larger endpoint ratio.
6. A source horizontal scaling and a source vertical shear induce invertible
   linear maps of its cap. No affine equivariance of squaring was assumed.
   The upper source function b is continuous even at the projection endpoints:
   concavity supplies one inequality for the inward limit and closedness the
   other. After the shear, its common endpoint d is nonnegative because twice
   d is the length of an extreme vertical section. Thus `b≥d≥0`.
7. The cap sections and the difference-body sections in the packet are exact.
   The factor 2 in N is correct: a difference body is centrally symmetric,
   so the widths at horizontal coordinates s and −s agree. Each of its two
   endpoint maxima is linear-before-maximization in b; hence N is positive
   homogeneous and subadditive, while cap area is linear.
8. Piecewise-linear concave profiles with equal endpoints decompose into a
   nonnegative constant plus tents by their downward slope jumps. Continuous
   concave profiles admit uniform concave polygonal interpolation. Uniform
   convergence gives uniform convergence of both cap section functions and
   their endpoint maxima, so A and N converge. This covers nonsmooth bodies
   and vertical endpoint faces, without any unjustified finite-curvature
   assumption.

## Independent maximizer derivation

Write `b=1+a`, `c=1−a`, and set `p=√(u+s)`, `q=√u` so `p²−q²=s`.
On the first U branch, the H+ objective is `(p+q+s)/b` and the H− objective
is `(p+q−s)/b`. Both increase strictly with u. On the second U branch the
derivatives have respectively the signs

```
b/p+c/q−4,                 c/p+b/q−4.
```

These expressions strictly decrease with u. The derivative jumps downward
at the U kink; thus each full objective is strictly concave, and the lower
endpoint q=0 is excluded by its positive infinite one-sided derivative
unless s=1. There are precisely the stationary, kink, and p=1 cases.

For H+, the transition from kink p=a to stationarity occurs at
`q=d=ac/(3a−1)`. It is present only if `d≤a`, equivalently `a≥1/2`.
The stationary arc ends at p=1, q=`q+=c/(3−a)`.

For H−, stationarity reaches its kink q=a at p=d. It does so before p=1
exactly when `d≤1`, equivalently `a≥√2−1`. For `a≥1/2` its maximizer
starts at the kink. If a≤√2−1, it instead reaches p=1 first at
`q−=b/(3+a)`, after which the endpoint branch crosses q=a.

These implications give exactly the packet's three rows. At their shared
endpoints a=√2−1 and a=1/2 the relevant arcs have zero length, so the
tables agree. At a=0 the U first branch has zero length and the argument
remains valid. The undefined d at a=1/3 is never used in that range.

The elementary primitives also check directly. At p=1, the remaining
values are `q(1−q)/b`, or the two U expressions, and `ds=−2q dq`; these
give B, R and E. Along a kink, the value is `a+x(1−x)/b` and
`|ds|=2x dx`, giving Q. Their orientations yield the signs in every row.

## Independent stationary primitive and exact identities

With `D=2(b/c+c/b+1)`, direct expansion of the stationary parameterization
gives

```
F = [D+b(2r−r²)+c(2/r−1/r²)]/16,
s'= −[b²r+bc+bc/r²+c²/r³]/8.
```

A primitive is `G(r)−bc log(r)/64`, where

```
G(r) = −1/128 [
 −b³r⁴/4 + b²(2b−c)r³/3 + b²(D+2c)r²/2 + bc(b+D)r
 −bc(c+D)/r − c²(D+2b)/(2r²)
 −c²(2c−b)/(3r³) + c³/(4r⁴)
].
```

This was obtained by multiplying the two displayed Laurent polynomials
and integrating their nine monomials; it agrees with every coefficient
in the assembled draft. The logarithm coefficient is −bc/64.

The companion `check_identities.py` differentiates this independently
derived primitive and checks the rational endpoints by clearing denominators
and testing the resulting polynomials are zero. It does not call symbolic
integration. All three rows pass. Logarithms are checked separately on their
positive arguments, avoiding any unsafe complex-log rewriting:

* LOW: `q+q−=(1−a²)/(9−a²)`.
* MIDDLE: `q+(3a−1)/c=(3a−1)/(3−a)`.
* HIGH: `q+ / [c/(3a−1)]=(3a−1)/(3−a)`.

Thus the two exact N formulas follow from the branch table. The independent
check command was `uv run --with sympy check_identities.py`; output confirms
zero rational-part difference numerator for low, middle and high, plus the
primitive derivative and all logarithm arguments.

## Inequality, limits and sharpness

The LOW deficit estimate is valid: `a²≤3−2√2<1/5`, the polynomial
`8−8t−32t²` decreases there and is at least 128/25, while its positive
logarithmic subtraction is at most log 11. Positivity follows even from
`log11<3<128/25`.

The HIGH estimate uses the correct direction of the trapezoidal inequality
for the convex function 1/x. Its denominator `(3−a)(3a−1)` is positive
throughout the range. The remaining polynomial is
`−16(a²−3a+1)>0`, since a≥√2−1>2/5 and the quadratic decreases on
`[2/5,1]`, where it starts negative. Hence every genuine tent has strict
deficit. There is no singular limiting equality claim at a=1.

For thin rectangles, after target scaling `(u,v)↦(u,v/(2ε))`, their squares
are parameterized on the fixed square by `(x²−ε²y²,xy)` and converge
uniformly to the parabolic cap. Convex Hausdorff convergence gives area
and difference-area convergence, while the limiting cap has positive area.
Its exact ratio is 5. Thus five is the sharp supremum, without claiming a
full-dimensional squared body attains it. Degenerate origin-symmetric
sources, if allowed under the word body, have zero area on both sides and
also follow by approximation.

## Optional equality refinement

One can show equality never occurs for a full-dimensional squared body.
This is not needed for approval of the stated theorem. The concave profile
has the exact positive tent-measure representation
`b=d+∫f_a dμ(a)`, where `dμ=(1−a²)(−b'')/2` on (−1,1).
The measure is finite: its mass equals `∫(b−d)dx` because every normalized
tent has integral 1. Applying max-of-integral ≤ integral-of-max to the
section formulas gives

```
5A(b)−N(b) ≥ ∫[5A(f_a)−N(f_a)] dμ(a).
```

The integrand is strictly positive at every interior a, so a cap has
equality only when μ=0, namely b is constant. Its source is a sheared
rectangle with vertical sides. If K is not a parallelogram no direction
has that property. If K is a parallelogram, rotate the coordinates so
neither coordinate axis is parallel to an edge. Both endpoint caps then
have strict inequality, so endpoint reduction is strict for σK. Rotation
of the source rotates its square by twice the angle and preserves area.

## Limits of this audit

This proves neither novelty nor optimality of the four-dimensional constant.
The final reader-facing manuscript should keep source-backed symplectic
implications distinct from the self-contained planar theorem.

## Primary-source transfer addendum, 23:01 UTC

[Chaidez–Tanny v1](https://arxiv.org/html/2610.31906v1), Remark 5,
Lemma 2.19, Theorem 3.7, Lemma 3.8 and equations (5.2)–(5.3) confirm the
required fiber class, reversibilization and factors. Substitution gives
`c_EHZ(X)² ≤ 16·(1/2)·(5/4)·Vol(X)/4 = (5/2)Vol(X)`.
Remark 5.4 supplies nonsmooth convex approximation. Thus the semantic
transfer passes; this does not independently reprove their covering or Pu
theorem.
