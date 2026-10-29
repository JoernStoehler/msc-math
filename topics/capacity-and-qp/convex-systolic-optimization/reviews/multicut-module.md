# Review of the frozen multicut variation module

**Verdict: APPROVE.** I found no consequential mathematical issue in the
compact proof or the duplicate-word example. I made no changes to
`multicut-variation.tex`.

## Duplicate-word calculation

I checked the two displayed row-3 midpoints against the KKT reconstruction in
`duplicate_exact.py` at (q=0,1/3,1); all four coordinate differences vanish
exactly in its number field. The script's exact response values at
(q=0,1/3,2/3,1) interpolate to

\[
\frac{2-\sqrt5}{4}q^2+\frac{-19+9\sqrt5}{16}q+
\frac{195-77\sqrt5}{240},
\]
with zero cubic coefficient. The interpolation is exact: the displayed
midpoints are affine in (q), the two weights are affine in (q), and the
segment integral is quadratic in each midpoint, so the total response has
degree at most three. The script's four exact samples therefore determine it.
The centroid (c_3=(-1/4,\tau(\tau^2-9)/8,0,0)), the variance correction
\(T^2b^2|a_3|^2/3=(5-\sqrt5)/24\), the vertex
\(q_*=(7-\sqrt5)/8\), and the endpoint gap
\((\sqrt5-1)/128) agree with the formulas. I ran the checker using
`uv run --with sympy==1.14 python duplicate_exact.py`.

## Proof and scope

The compact argument retains the essential steps of the audited theorem:
finite-bank value sensitivity via a positive full-rank minor (without a
Hessian assumption), reduced-cost insertion for orbit placement, the
self-pairing term in the midpoint derivative, exact refinement at cut-profile
switches, and the radial volume derivative. The finite repeated-word bound
is correctly obtained from Clarke duality; it is used for comparison and does
not enlarge the sensitivity optimization beyond a fixed finite row bank.
The product rank argument correctly uses positive action to force both
Lagrangian factors and then separates closure across them.

The convex-order separation proof retains compactness, common-ensemble
quantifiers, and the common-constant argument for inward cuts. Both examples
are carefully scoped: the pure-word obstruction is only to finite ensembles,
and the repeated-word calculation refutes endpoint convexity without claiming
that those endpoints are globally best among all simple words or that the
systolic ratio increases.

The standalone output reports five pages (`module-check.pdf`). This review
checks the mathematical content and the exact duplicate calculation; it does
not claim a visual typography audit of every page.
