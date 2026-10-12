# Independent review of the uniform facet-fraction theorem

2026-10-04 23:36 UTC. Reviewer: `conditional_mechanism`. Receiving owner:
`extremality_discovery_lead`. Reviewed `facet-fraction/README.md`, `verify.py`,
and `certificate.json`. No implementation ownership or producer-output changes.

**Accept the stated restricted theorem:** for the fixed rational base P*,

\[
 -\tfrac14<c<\tfrac34,\qquad 0<\tau\leq\tfrac1{100},\qquad
 K=P_*\cap\{(0,2-2c\tau,\tau,0)\cdot x\leq1\}
 \quad\Longrightarrow\quad \rho(K)<1.
\]

The same τ interval is valid for the entire open c interval, including coupled
approaches to its endpoints. No mathematical gap found. Acceptance is independent
agent review relative to the HK formula and the existing P* geometry/capacity
proof, not Jörn review, formalization, or a novelty claim.

## Independent executable evidence

A read-only scratch audit,
`/tmp/msc-math/pstar-independent-vertex-review/facet-fraction.py`, independently
reconstructed the eight-label normal closure, total mass, direct HK pair sum,
and weight-corner signs from the retained certificate. It integrated the momentum
moments directly and **reconstructed each polynomial from the retained tensor
Bernstein basis**, rather than reusing the producer's monomial-to-Bernstein
conversion. All three reconstructed polynomials agree exactly with the cleared
capacity-volume expressions. The audit passed.

Reviewed certificate hash:

```text
1b756f8645efc373c1a5a5e71b253669e6c1f189a1653e240deec2b614890e50
```

The verifier writes `certificate.json`; this review used the retained file
without invoking that write. The independent audit imports no retained producer.
Its mathematical inputs and decisive checks are recorded below.

## Feasible section and positive action

The word `(0,10,6,4,8,2,1,9)` has eight distinct labels. Its certificate weights
satisfy normalization and closure **identically** in `(r,c,τ)`. The new normal
and its fixed weight `3r/4` make the closure correction exactly linear in τ;
no higher-order closure remainder is omitted.

With `η = 3/4 − c` and `r = min(η/6,1/12)`, we have `0 < r ≤ 1/12` and
`η − 3r > 0`. Coordinates divisible by r have strictly positive multiaffine
quotients on the c,τ rectangle; all other coordinates have strictly positive
corner values on the r,c,τ rectangle. These are genuinely multiaffine
polynomials, so corner interpolation proves positivity throughout the box.
Tending to `r = 0` at the excluded right endpoint does not invalidate any
interior weight.

The direct score agrees with

\[
 Q=\tfrac12+\tfrac{3r}{4}(\eta-3r)\tau
       -\tfrac{21}{64}r^2(4c+3)\tau^2.
\]

Its positive linear term and the bound

\[
 Q\geq\tfrac12-\tfrac{21}{64}\left(\tfrac1{12}\right)^2
                       6\left(\tfrac1{100}\right)^2>0
\]

establish the required positive HK action on the full domain. Since all
inequality heights are one, the feasible datum gives exactly
`c(K) ≤ 1/(2Q)` and `ρ(K) ≤ 1/(8VQ²)`. This is an upper bound, requiring no
complete capacity catalogue.

## Exact cap geometry over all cut fractions

The denominator `d = 2 − 2cτ` satisfies `d ≥ 397/200 > 0`. At momentum p,
the top-q cut has depth

\[
 h(p)=\frac{\tau(p_1-c)}d
\]

when `p₁ > c`, and removes nothing when `p₁ ≤ c`. Since `p₁ − c < 1`,
`h < (1/100)/(397/200) < 1/2` uniformly. Thus the slice never reaches the
hexagon's middle vertices, and its width is exactly `5/6 + 2h/3`. Integrating
that width gives removed q area `5h/6 + h²/3`, with no asymptotic remainder.

The retained momentum quadrilateral has exact vertical slice densities
`1 + 4p₁` on `[-1/4,0]` and `1 − 4p₁/3` on `[0,3/4]`. Independent integration
agrees with all four retained moment formulas. It follows on the **whole**
parameter interval that

\[
 V=\tfrac12-\frac56\frac{\tau I_1}d
                -\frac13\frac{\tau^2 I_2}{d^2}.
\]

This uses the negative-c piece as well as the right-vertex triangular piece;
it does not extend a local moment formula beyond its geometric domain.

## Bernstein proof and removed factors

The cleared polynomial is exactly

\[
 d^2(8VQ^2-1)
 =8\left(\tfrac12d^2-\tfrac56\tau d I_1
                    -\tfrac13\tau^2I_2\right)Q^2-d^2.
\]

The three η intervals cover `[0,1]`; the formula changes at `η = 1/2` for
the adaptive weight, and at `η = 3/4` for the momentum moment. The formulas
agree at their shared endpoints. Division removes `τη²` on the first interval
and τ on the other two, each strictly positive in the genuine-cut domain.
The reduced expressions have only constant rational denominators.

After affine mapping of η to `[0,1]` and `τ = u/100`, their degrees are
`(8,5)`, `(6,5)`, `(6,5)`. All 138 retained tensor Bernstein coefficients are
strictly positive. Independent reverse expansion verifies their complete
polynomial identities, their degrees, and the three recorded exact minima.
The tensor Bernstein basis is nonnegative and sums to one on the closed unit
box, so each reduced polynomial is strictly positive there. Restoring the
strictly positive removed factors and d² gives `8VQ² > 1`, hence `ρ(K) < 1`.

At τ zero or η zero these are algebraic limit checks only. The theorem does
not invoke an eleven-genuine-facet HK formula at those degenerate limits.

## Eleven genuine facets and accepted scope

The old top-q facet retains an open momentum part `p₁ < c`, while the new
facet has an open part `p₁ > c` and lies strictly below the old top. Both exist
for every c in the open stated interval. The new normal has nonzero q and p
components and is distinct from every old pure-factor normal.

The four sloped q facets retain open portions near the hexagon's middle,
where the new inequality is strictly satisfied uniformly in the parameter
box. The bottom q facet remains strictly below the cut. Each p facet retains
an open q neighborhood of zero because `τp₁ ≤ 3/400 < 1` there. Thus all ten
old facets and the additional mixed facet are genuine for the entire domain.
The origin remains an interior point. The excluded c endpoints correctly
account for loss of the old top facet at `c = −1/4` and redundancy of the new
facet at `c = 3/4`.

This is a uniform theorem for this particular p₁ tilt of one facet at the
fixed base body. It does not establish arbitrary facet tilts, moving old
normals, equality-family drift, multiple new facets, or a full F11 neighborhood.
The separate finite-bank failure and stationary-continuum provenance are not
needed as theorem premises and were not re-audited in this narrow review.
Review complete; stop for integration.
