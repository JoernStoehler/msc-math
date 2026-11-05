# Independent audit: HKO 75-vertex chamber and two-word envelope

Reviewer `/root/single_facet_audit`, 29 October 2026. Root owns integration,
retention, commits and cleanup. Read-only project review; no agent fanout,
service-tier change, new spending, old-certificate rerun, or repeated panel run.

## Verdict and scope

**PASS** for the chamber argument on the closed interval [3/10,2/5], the
displayed exact volume, and the maximum of the two feasible-word upper bounds.
This uses the inherited certified premise that H_(3/10) has exactly the stated
75 simple vertices. It does not establish the capacity at another parameter,
an attained improvement, or an optimum outside this interval.

The checked assertions are:

- The same 75 simple vertices are the complete vertex set throughout the
  closed interval; no untracked feasible vertex can be born.
- The README's cubic-over-cubic rational function is the actual volume there.
- V, Q7 and Q8 are finite and strictly positive throughout the interval.
- R7=1/(8VQ7^2) is strictly increasing and R8=1/(8VQ8^2) strictly decreasing.
- There is one simple Q7=Q8 crossing, at approximately
  0.3403557218556925. Thus min(R7,R8) has its unique maximum there, with
  value approximately 1.0524316650246792.

The branch feasibility needed for the upper-bound interpretation was
independently established in `AUDIT-panel-branches.md` and was not rerun here.

## Inspected sources and actual checks

Read the complete `chamber.py`, the current continuation README and the
retained scratch `chamber.json`. The reviewed source snapshots were:

    chamber.py
    e75919f1a5d61f5609cc6343e36d08f35ad9abcdc22229bfe7bd91e4e6f11c5a
    README.md
    8e32c59adb4969015f03fdb3609aff33a2b5bd3eeb42b0d9da377f368f97ce9f

Ran one frozen scratch copy `audit-chamber-source.py` of that exact producer.
It completed successfully in approximately 25 seconds. The output
`audit-chamber-replay.json` matches every field of `chamber.json`, except the
expected runtime field. This replay checks all tracked determinants, all
inactive slacks, the accepted Lawrence functionals, rational volume equality,
and the displayed symbolic expressions.

Separately ran `audit_chamber_scalars.py`, which reads the retained formulas
and checks the README's literal volume coefficients, exact closed-interval
roots and signs, derivative signs and poles, the unique simple crossover,
and the cusp value. Its output is `audit-chamber-scalars.json`. The exact
algebraic value lies strictly between the rational decimals

    1.0524316650246791 < cusp bound < 1.0524316650246794.

This independent scalar pass does not reuse the chamber producer's KKT solve
or its derivative-root output lists.

## Why tracking only these bases is complete

The first-event claim is valid, but boundedness alone is not its whole
justification. Here is a complete compactness/local-stability argument.

Let P(t) be the twenty-row body and let v_I(t) be the 75 tracked vertices.
Every tracked basis has nonzero determinant throughout the interval and
every inactive slack remains strictly positive. Hence each v_I(t) is a
continuous feasible simple vertex with exactly its four labeled active rows.
The old inequalities give a common compact outer bound H. The normals vary
continuously on a compact parameter interval and every support is one, so a
fixed small ball around the origin lies inside every P(t).

Let S be the set of parameters where P(t) is the convex hull of the tracked
vertices. The inherited complete simple-vertex certificate makes S nonempty
at t=3/10. S is relatively closed: if t_n in S tends to t_0, then for any
x in P(t_0), the point (1-epsilon)x is strictly feasible at t_0 and feasible
at t_n for all sufficiently large n. Passing to the continuously varying
convex hull and then letting epsilon tend to zero proves x lies in the
tracked convex hull at t_0. The reverse inclusion always holds.

S is also relatively open. Otherwise, at a parameter t_0 in S there would be
t_n tending to t_0 and untracked vertices x_n of P(t_n). Select four independent
active labels I at each x_n. Finiteness of the label subsets and the common
outer bound allow passage to a subsequence with the same I and x_n tending
to x_0 in P(t_0). All four I inequalities are equalities at x_0. Write x_0 as
a convex combination of tracked vertices at t_0. Since all facet slacks are
nonnegative, every tracked vertex with positive coefficient must also
saturate all four I rows. Such a vertex has exactly four active rows, so its
active set is I and its basis is nonsingular. Its unique nearby solution is
therefore v_I(t_n), contradicting that x_n was untracked.

Thus S is nonempty, relatively open and relatively closed in a connected
interval, and S is the whole interval. This also explains why singularities
of bases that were initially infeasible need not all be enumerated: they
cannot yield an extra vertex without violating one of the tracked-vertex
conditions in this argument.

## Exact root and denominator checks

The field embedding selects the unique root tau of
tau^4-10 tau^2+5 in (0.72,0.73). Mapping each polynomial coefficient into AA
and requesting all AA roots gives exact algebraic real roots, with exact
closed-interval comparisons. There is no floating endpoint filter in the
chamber code. Multiplicities are immaterial for nonvanishing; even repeated
zeros are returned and excluded.

Every active basis matrix has polynomial entries in t. Its determinant is
therefore a polynomial. Its inverse entries are cofactors divided by the
determinant. Consequently the denominators of its vertex coordinates,
inactive slacks and Lawrence slopes divide a power of this determinant
(in fact one power suffices before subsequent products). The determinant
test excludes all their possible poles throughout the closed interval.
Thus the absence of an explicit separate slack-denominator root call is
sound. A nonzero slack numerator with no interval zero and a positive exact
value at 3/10 remains strictly positive on the entire interval.

The routine's convention of returning no isolated roots for the zero
polynomial does not conceal a zero determinant or slack: both have separate
nonzero assertions. In the separate scalar pass the functions and derivatives
were also checked nonzero by their exact endpoint signs.

## Lawrence formula and possible removable singularities

For each tracked simple vertex, the determinant sign is fixed on the interval
and correctly determined at the base parameter. The accepted linear
functionals satisfy, for every vertex and every tangent direction, a nonzero
slope on the entire closed interval: `lawrence()` rejects an identically zero
slope and rejects any slope numerator having an interval root. The denominator
argument above excludes slope poles as well.

Accordingly the two accepted Lawrence sums are actual valid tangent-cone
formulas pointwise everywhere in this interval. Their equality as elements of
Q(tau)(t) is an exact consistency check. No removable-singularity extension is
needed in the reviewed implementation. The first unsuccessful trial
functionals, if any, are discarded and do not enter the returned volume.

More generally, equality of rational expressions alone would only establish
the formula away from individual slope zeros. One could then extend a reduced
formula across those exceptional points using continuity of actual volume
and a proof that no nonremovable pole remains. That alternative argument is
unnecessary here because of the stronger global slope test.

The complete simple-vertex argument licenses the Lawrence sum. Two matching
functionals would not on their own prove that the vertex list was complete.
The independent scalar pass confirmed that the reduced volume numerator and
denominator have no zeros in the interval and that its value at 3/10 is
positive. Its literal coefficients match the README exactly.

## Monotonicity and the cusp

The scalar pass derives R7 and R8 anew from V and Q7,Q8, checks equality to
the retained ratio expressions, and excludes zeros of each derivative's
numerator and denominator throughout the closed interval. Exact algebraic
comparisons establish R7'(3/10)>0 and R8'(2/5)<0. Hence their derivative signs
hold everywhere, not merely at the sampled endpoints.

Q7-Q8 has one simple zero in the interval and no pole. Its sign is positive
at 3/10 and negative at 2/5. Since both Q values and V are positive, before
the crossing the smaller ratio upper bound is R7, while afterward it is R8.
The former increases strictly and the latter decreases strictly, proving
the unique cusp maximum of min(R7,R8). The cusp is above the certified
endpoint ratio; this is remaining room in an upper bound, not an attained
capacity ratio or evidence that another body improves the counterexample.

## Producer recommendations sent to root

The mathematical conclusions above pass, but the reviewed producer has two
places where retained assertions should be made more explicit:

1. At the base enumeration, replace the conditional acceptance
   `if len(active)==4` with an assertion followed by insertion. The existing
   frozen t=3/10 certificate provides the required simple-complete premise;
   a current replay should also fail visibly rather than silently discard
   a nonsimple feasible vertex if its inputs change.
2. Assert that both derivative-root lists are empty, check their endpoint
   signs with exact embeddings, and assert the unique crossing count. The
   reviewed producer currently prints these outputs. The separate scalar
   audit supplies the exact assertions for this result, but encoding them
   in the maintained producer makes its acceptance behavior clearer.

This is scoped independent agent mathematical review, not human acceptance
or formal verification. It does not select or authorize a complete-word
search at the cusp.
