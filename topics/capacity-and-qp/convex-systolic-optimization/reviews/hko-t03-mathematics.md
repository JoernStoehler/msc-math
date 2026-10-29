# Analytic review of the frozen t=3/10 certificate

29 October 2026, 00:45 UTC. Reviewer `/root/hko_ascent_math_audit`.

**PASS for the assembled mathematical argument and inspected checker logic.**
The remaining independent fresh-copy execution review is owned by root's
selected reviewer. I did not duplicate the approximately eight-minute exact
replay during this assignment. This distinction is execution scope, not an
unresolved mathematical branch.

Reviewed packet: `../certificate-t03/`.
Manifest SHA256:
`b648a3eb11acd5845d0e16c50ee4236e3b1302928e1308a2c6332f6c0c915dc8`.
Coverage proof SHA256:
`0b78ff54c96e52b07ae4070ff65b1a7435099795d387f7735ca17982ddab9b50`.

The detailed reasoning is retained in `math-preliminary.md`. Its initially
pending singular-feasibility and volume-reference obligations are now resolved
by the frozen packet and `volume-identity.md` respectively.

## Closed implications

1. **Global coverage:** the geometric simple-minimizer theorem supplies a
   capacity-attaining simple word in the exact coactivity/nonnegative-pairing
   graph. This avoids any unproved full-rank placement claim for arbitrary
   algebraic maximizing supports. The exact twenty-row body is within that
   theorem's general polytope scope; no sixteen-facet production contract is
   imported.
2. **Pruning and quotient:** positive tangent curvature of a prefix remains
   positive in every extension after padding the direction with zeros.
   Cyclic invariance on closure-nullspace directions validates prefix
   canonicalization. Exact transformations of all twenty rows and pairings
   validate rotation, factor exchange, and reflection with word reversal.
   The integer DFS has no unproved length cutoff and covers every surviving
   simple cycle. Floating discovery decisions do not enter acceptance.
3. **Support boundaries:** zero coordinates can be deleted from a geometric
   maximizing word. Its positive support is searched separately and satisfies
   equality KKT. Thus boundary maximizers are not lost when a larger leaf's
   stationary point is rejected. Not every arbitrary subword needs to be a
   realizable graph cycle.
4. **Singular leaves:** the constant-objective identity for a consistent KKT
   family is checked. The completed authoritative refined output accounts for
   all 1,030 such families: 984 constants at or below the target; 21 above
   the independently proved containment bound; 25 infeasible by complete
   vertex enumeration of the compact parameter polytope. There are no
   unresolved or feasible above-target singular families.
5. **Attainment:** exact_attainer.py solves the literal seven-label word
   `(0,1,6,7,3,12,9)`, checks strict positivity, closure/normalization, and
   Q=(205/1019)tau. This supplies the lower bound independently of the
   refined checker's initialized target value. The null `weights` field in
   the refined summary is therefore not a missing witness: the witness is
   explicitly stored and checked in exact-attainer.json.
6. **Volume and facets:** complete four-row enumeration and exact slack
   checks cover every vertex; active-set size four proves simplicity. The
   added union-of-active-labels assertion covers all twenty inequalities.
   At a simple vertex every independent active inequality supports a local
   facet, so this proves twenty genuine facets at this actual parameter.
   The Lawrence formula, its signed slopes and its normalization are directly
   supported by the inspected primary theorem in `volume-identity.md`.

The exact curvature output reports 77,927 exclusions passed. Integer coverage
reports 43,110 surviving cycles represented by 2,326 checked leaves, with zero
unknown cycles. These are read producer results; the foregoing items establish
why those finite predicates imply the global conclusion.

## Result and arithmetic interface

The proven upper action bound and the explicit feasible attainer coincide:

    Qmax = 205 tau/1019,
    c(K) = 1019/(410 tau) = (1019/1025)c(H).

The exact volume is

    V(K) = 300748771/52178240 - 99664289 tau^2/260891200.

The frozen final verifier compares the resulting ratio to `(3+sqrt(5))/5`
using the exact real algebraic embedding, not a floating-point difference.
Its reported decimal values are

    rho(K) = 1.0519141388294875...,
    rho(K)-rho(H) = 0.0047005433295293... .

I checked the factor conventions: the ordered objective is Q, capacity is
1/(2Q), and the four-dimensional ratio is 1/(8VQ^2). The exact field root is
explicitly restricted to 0<tau<1. The original and moved normals agree with
the earlier family at t=3/10.

## Final execution boundary and integration recommendation

The source manifest binds the graph, exclusion and leaf lists as well as all
acceptance scripts. Final execution should verify that manifest in the fresh
copy, run every route listed in README, and compare the complete resulting
receipts before promotion. verify_result.py summarizes those receipts; running
that aggregator alone is not a substitute for the underlying checks.

The old nonsingular-only and loose singular-bound diagnostics are correctly
excluded from the final acceptance route. The abandoned Fourier–Motzkin run
has no accepted output. If a future run ever finds a feasible singular family
above the target, its actual nonnegative member must be retained; that case
does not arise in this frozen result.

This fixed moderate-parameter example is worth presenting prominently once
the selected independent replay passes. It supplies a meaningful explicit
increase over HKO, unlike the deliberately tiny effective step. It does not
optimize the family or imply a global bound over other convex bodies.

## Final verifier and reader-module updates

At root's request the producer strengthened verify_result.py with explicit
equalities between the geometry and attainer volume, the reciprocal-action
and attainer capacity, and the final and attainer ratio. I read these added
assertions. They only strengthen the previously checked aggregate and retain
the same mathematical result. **PASS for this delta.** Current verifier hash:

    804750ddf801a815d9ab7b27a9220368a8b326956f10da3cb547d10a957325a0

Current source manifest hash:

    238ec061eb3750f12cf630a89fe936a42ab16ab5279a6bf690b9207654a8bc0e

I also checked the reader-facing `../moderate-hko-section.tex`, SHA256
`5166803c21d1b656524aaefa46a86cdca5e5342522dae74bef74f6ed1a68db40`.
**PASS for its mathematical constants and coverage account.** Its radical
volume, seven positive weights and capacity factor agree with the frozen
certificate. Direct algebra gives its printed difference

    (7965280+4064064 sqrt(5)) /
      [5(502711205+99664289 sqrt(5))],

which is manifestly positive. The final module distinguishes all twenty row
labels from the original ten normals, attributes cyclic kernel invariance
to closure, and explicitly handles boundary maxima through positive active
supports. The statement that dependent equality rows are removed matches the
actual checker: full-system consistency is checked before selecting independent
C rows and the corresponding entries of e.

The geometric simple-minimizer theorem has now also been checked directly in
the primary author's paper, not just the thesis. Theorem 1.5 on page 3 of
arXiv:1712.03494v3 is the precise locator; the apparently different years are
the 2017 first preprint/BibTeX key versus the 2019 journal publication.
`simple-minimizer-source.md` records the direct source and convention mapping.
The initial primary-source access limitation in the preliminary report is
therefore resolved.
