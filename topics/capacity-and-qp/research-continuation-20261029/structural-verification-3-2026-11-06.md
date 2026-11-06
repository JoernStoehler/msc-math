# Structural verification, third pass — 6 November 2026

Independent follow-up to [pass two](structural-verification-2-2026-11-06.md),
restricted to its S19 and S21. Receiving owner: the assigning research coordinator.
This is verification evidence, not thesis acceptance or a new research assignment.
Baseline HEAD: `225f4890`. Existing shared edits and the existing PLAN/STATUS
were left untouched under the explicit one-new-file instruction.

## Verdicts

| Claim | Verdict | What was established |
| --- | --- | --- |
| S19: finite searches found no hinge ascent, smaller original-template cover, or slope violation | **Undetermined as complete historical experiments; holds for the retained candidates and radial scan checked here.** | Independently rescored nine affine-hinge candidates, all 360 radial triple/threshold cases, both saved outer-search best placements, and all twelve slope witnesses. No violating candidate. Full historical trajectories and some profiles are absent from the retained records. |
| S21: unrestricted rank-four/slope inequality, global original-template bound, arbitrary three-hinge obstruction | **Undetermined.** | The inspected sources provide reductions, special cases and finite numerical evidence, but no proof covering these universal domains. No counterexample was established here. |

## Method, sources and execution

The prior reports already compare exact proofs, finite certificate verification
and bounded search replay. Recomputing retained scores is the useful cheap next
step: repeating a random search would neither verify its historical trajectory
nor prove S21. No external literature inference is needed for this bounded
source verification. No branch producer, checker, helper or fixture was imported
or executed. Mathematical formulas, the catalogue's combinatorial chart masks,
and retained numerical candidates were inputs. A small part of the fixture
source and radial producer was inspected to resolve conventions; no functions
were copied or called. All new code was written in
`/tmp/structural-verify-3/check.py`, with output `check.log` and `hinges.json`.
The executed command was:

```sh
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  /tmp/structural-verify-3/check.py
```

The computation uses NumPy/SciPy floating arithmetic, not exact certificates.
Scratch artifacts are temporary and are not necessary links for understanding
these verdicts. Source paths below are relative to `/workspaces/msc-math`:

- Hinge definitions/results: `topics/variation-and-local-maxima/hko-cut-family/three-hinge-continuation-20261029/{README.md,probe-results.json}`.
- Normal rows: `topics/variation-and-local-maxima/hko-cut-family/README.md`.
- Quotient density: `topics/capacity-and-qp/convex-systolic-optimization/sections/hko-two-hinge-appendix.tex`.
- Orbit reconstruction: that report's `sections/multicut.tex`, placement-of-a-maximizing-word argument.
- Combinatorial chart input: that report's `certificates/hko-ascent/expected/catalogue.json`.
- Outer search: `alternative/outer-search/{README.md,results.json,results_box.json}` next to this report.
- Slope definitions/results: `topics/empirical-studies/discovery-design/desk/zonotope-research/rank-four-restart-20261029/continuation-20261029/{projection-slope.md,slope-lp-results.json,rank-four-stop-state.md}`.

Luna performed read-only source triage. Its suggestion that some retained JSONs
were absent from this checkout was contradicted by direct reads; this pass used
the live absolute checkout paths above, not scratch copies.

## S19: independently reconstructed hinge evaluation

1. Rebuild the five q support normals from their displayed formulas, and p
   normals by the specified quarter-turn. For every triple solve the three
   closure/normalization equations. Exactly five triples have positive weights.
   These independently computed closure vertices replace the saved primitives.
2. For each supplied catalogue chart, use its masks to select one closure
   vertex in each factor, assigning mass one-half to each factor. This gives
   one or two endpoint weight vectors. Check closure by construction and
   recompute the quadratic objective: every endpoint has `Q=tau/5`.
3. Solve `G beta = N_word lambda + eta*1` independently by least squares.
   The residual is below `1e-12`. With prefix sums
   `s_r=sum_(k<r) beta_k N_(sigma_k)`, reconstruct
   `x_r=(-lambda-2 J s_r)/(2Q)`, where J is the skew-form matrix.
   This follows from the source convention `mathbb J=-J`; the multiplier's
   sign agrees with the stationarity equation written here. Check each active
   facet equation and every facet inequality at every endpoint loop vertex.
   The largest observed facet violation is `5.78e-15`. Affine interpolation
   preserves feasibility; the argument of a characteristic-invariant profile
   stays constant along its segment.
4. Compute quotient coordinates directly from these positions. On q facets
   use own coordinate q and partner p; on p facets use own p and partner -q.
   This sign is required by the symplectic factor swap. Use the tangent
   `e=J2 normal/|normal|`, `xi=e.own/sin(pi/5)` and
   `eta=(e.partner-m)/h`, with `m=(1-cos(pi/5))/2`, `h=(1+cos(pi/5))/2`.
5. Each chart response is a sum of affine weights times positive parts of
   affine functions of its parameter. Split at every interior hinge zero.
   On each interval evaluate both endpoints and every concave quadratic's
   interior stationary point. This covers the entire supplied chart interval,
   rather than sampling it. Catalogue completeness is an upstream premise;
   its 362,880-order enumeration was not repeated here.
6. Independently integrate each facet profile. For `a=|alpha|>0`, the uniform
   xi average is `((d+a)_+^2-(d-a)_+^2)/(4a)`, `d=beta*y+gamma`;
   for a=0 it is `d_+`. Integrate this against the two stated density pieces,
   splitting additionally at `d=+/-a`, and multiply by `1/10`. Adaptive
   quadrature with absolute tolerance `1e-13` replaces the original fixed
   96-point rule. These tolerances are numerical settings, not rigorous errors.

| Retained affine candidate | Fresh response / total volume |
| --- | ---: |
| Adjacent q, seed 17 | 1.00022963528412 |
| Its Nelder–Mead refinement | 1.00018513458795 |
| Its Powell refinement | 1.00021707634630 |
| Adjacent q, seed 83 | 1.01870928576329 |
| q facets 0,1,3 | 1.22759532713497 |
| Mixed 0,5,1, seed 23 | 1.22172886251719 |
| Mixed 0,5,1, seed 47 | 1.21663199664325 |
| Coupling constrained, seed 29 | 1.06083660879801 |
| Coupling constrained, seed 71 | 1.07137400968908 |

All nine saved chart responses agree to ordinary floating precision. The
largest discrepancy in a saved individual facet volume is `1.33e-10`
(mixed seed 23), consistent with differing integration rules, and immaterial
to its nonascent margin. The smallest response/volume margin is still about
`0.0001851`. These are numerical confirmations, not exact sign certificates.

For radial profiles the correct quotient coefficients are
`(alpha,beta,gamma)=(0,h,m-threshold)`, since the profile is
`(e.partner-threshold)_+`. All 120 facet triples were evaluated at each of
three thresholds, using the same independently reconstructed charts:

| Threshold | Fresh total volume | Best volume-minus-response gap | Worst gap |
| --- | ---: | ---: | ---: |
| 0.5 | 0.00723606797749979 | -0.00526393202250025 | -0.0141164811787427 |
| 0.6 | 0.00370486680447989 | -0.00429513319552012 | -0.00429513319552023 |
| 0.7 | 0.00156299068313996 | -0.00293700931686006 | -0.00293700931686014 |

All gaps are negative, confirming the scan's conclusion. Saved volumes differ
by at most `1.54e-10`. At the latter two thresholds all triples tie to floating
precision; a different maximizing triple does not contradict the record.
An initial scratch run mistakenly used `(-eta-threshold)_+`; it was corrected
to the radial definition before these results. That was our setup error.

## S19: outer placements and slope witnesses

For the outer placements, independently reconstruct every positive closure
triple of the five planar normals. Both traversal orders of its weighted edge
vectors give the ten centered triangles. Add each saved best translation and
compute the hull of all thirty points, without coalescing any heights. Both
files give area `0.20081141588622725`, five hull vertices and area/HKO-area
`0.9999999999999999`. Thus the saved winning placements are confirmed.

Record consistency also checks: summing `calls_after-calls_before` over the
twelve starts gives exactly 2,652 and 2,606 respectively. The first run has
zero successful Powell flags, and the boxed run one. These checks verify the
stored summaries' internal arithmetic, not actual execution counts or timings.
There are no retained placements for all individual calls, so the assertion
that *none of those calls* found improvement cannot be independently rescored.
No new outer search was substituted for that missing historical evidence.

For each of the twelve slope witnesses, reconstruct `C=g wedge h` and saved
W, then sum literally

`B=sum_(i<j<k,l) |W_ij C_kl-W_ik C_jl+W_jk C_il|`.

Divide by `||W||_1 ||C||_1` with upper-triangular norms. All ratios agree
with their stored values within `8e-16`; their maximum is
`0.9921924333525534`. Direct six-term expansion of every four-coordinate
`W wedge C` gives maximum absolute residual `2.67e-14`. Thus these floating
witnesses support nonviolation but are not exact feasible LP certificates.
Neither their sign-ascent trajectories nor all starts were independently
recreated. The identical-profile optimization mentioned in the hinge README
has no profile vector in `probe-results.json`; its score remains unchecked.

## S21: proof obligations checked step by step

The proposed matched-projection argument has the following logical scope:

1. Its affine deficit identity gives
   `Delta(t)=Delta(0)+(2 Phi M-4B)t` for nonnegative t.
   A strictly negative slope would eventually give a counterexample.
2. Therefore a universal rank-four inequality would imply nonnegative slope.
   This necessary condition does not establish a converse. The source explicitly
   supplies no removal/induction argument for arbitrary generators.
3. The exterior formulation restricts W to `W wedge C=0` for decomposable C.
   Twelve fixed C inputs and numerically feasible W witnesses leave infinitely
   many C and W untreated. A failure to discover a violation has no universal
   implication. The already verified disjoint-support proof treats only a
   special projected-direction configuration, not overlapping supports.
4. Similarly, exact parallel-slide optima concern fixed perpendicular offsets.
   They do not minimize over the nine remaining offsets. The recorded search
   box has no proved coverage property. The Steiner relaxation lies below HKO,
   so its lower bound cannot prove the desired original-template lower bound.
   Restoring one optimizer is not a uniform restoration estimate.
5. For hinges, the constant-profile obstruction treats zero affine slopes.
   The complete radial triple scan restricts the profiles and thresholds.
   Nine nonconstant triples leave arbitrary coefficients and facet arrangements
   untreated. No argument converts those samples into a general obstruction.

These are verified gaps in the proposed implications, not counterexamples to
the targets. Settling S21 would require new universal proofs or exact violating
examples. It remains undetermined within this budget.

## Review, limits and handoff

Independent reviewer `/root/sol_review` (gpt-6.1-sol, medium effort) agreed
with both scoped verdicts and found no substantive objection. Its fresh
`/tmp/structural-verify-3/sol-independent.py` independently reconstructed five closure triangles
and all 190 chart endpoints, obtaining 50 point and 140 interval charts,
maximum body violation below `8.9e-16`, and characteristic-coordinate drift
below `1.2e-16`. It also checked density normalization and all three radial
volumes, and confirmed the p-facet sign convention. This independent review
does not certify historical search coverage or prove S21.

[review: /root/sol_review]

Only this new report is selected
for commit/push. Existing files are neither edited nor staged. The next useful
verification requires missing historical candidate data for complete S19
coverage; absent those data, a new independently logged search would be a new
experiment. S21 requires a new mathematical argument. Both parent outcomes
remain open; no PLAN/STATUS edit is authorized by this assignment.

The work is bounded by the requested roughly 30-minute / approximately $1
allowance. No reliable per-task dollar meter is exposed, so no dollar charge
is asserted. Local computations add no model API calls. Scope was deliberately
kept to saved candidate verification and proof-gap assessment.
