# Width-contact selector: independent assessment and thesis disposition

17 October 2026. The [separate patterns report](../../../../../patterns-2026-10-17/patterns/upper-tail/geometric-selector/REPORT.md) owns the rule, alternatives, frozen controls and scientific evidence.

The new construction chooses supporting closures through the factor directions attaining the two-bounce width bound. It recovers three previous hard-case numerical capacities, but fails both intended generalizations: an exact feasible challenger proves at least a **9.90% capacity miss** on `random_5x5_s1_64`, and its exact ratio bound **1.164606** on `random_6x6_s1_194` exceeds HKO's `(3+sqrt(5))/5`. The latter is an obstruction to this selector's sharp upper envelope, not a counterexample to HKO maximality.

## Independent mathematical assessment

For a bounded planar polygon `P={p: a_i·p≤1}` containing the origin, with irredundant facets, set `t_i=1/(1+h_P(-a_i))`. Dotting any normalized nonnegative normal closure with an opposite support point bounds its coefficient at `a_i` by `t_i`. The two adjacent normals at an opposite support vertex realize this maximum; edge degeneracy permits a two-normal closure. The planar difference-body polar is `conv{±t_i a_i}`. This justifies selecting maximal pairings without capacity labels.

For selected closure vectors `x_0+x_1+x_2=0` and `y_0+y_1+y_2=0`, the order `A0,B0,A1,A2,B1,B2` has signed cross-sum `2 x_0·y_0`. The HK weights have factor mass `1/2`, so `Q=|x_0·y_0|/2` after possible reversal, and the resulting capacity bound is `1/|x_0·y_0|`. Thus the displayed order reproduces width; optimizing all selected orders can improve it. This is a baseline guarantee, not a tightness theorem. The [private independent mathematical assessment](independent-mathematical-assessment.txt) records the separate reviewer and the necessary planar hypotheses.

## Verification and limits

[Independent checker](independent_check.py) imports no selector code. It reconstructs rational polygon areas from feasible line intersections and a monotone hull, checks the LP anchor coefficients and all maximal anchor pairs, and evaluates the retained word using the direct four-dimensional symplectic form. It also verifies all 120 cyclic orders on each selected pair, without searching other supports.

- [Initial receipt](independent-verification.json): ten exact rational bodies, 1,200 selected-pair orders, the feasible challenger and three numerical regular controls with an independent analytic pentagon area. HKO's numerical ratio is recovered as 1.0472135955.
- [Stress receipt](stress-independent-verification.json): two more rational bodies and 240 selected-pair orders. Exact rational squaring checks `r>(3+sqrt(5))/5`.
- [Protocol receipt](protocol-verification.json): three name-hash controls and the two highest-width survivors independently match the stated selection. The latter two are a declared second-stage amendment after the first results, not held-out prediction.
- [Input bindings](input-bindings.json) identify the checked worker outputs and independent checker. The cached raw-geometry hash is checked by the script; source data are not duplicated here.

Numerical agreement on some stored capacities does not prove optimality or prediction performance. The feasible challenger alone gives the exact 9.90% obstruction without requiring historical labels or the true optimum. The twelve controls do not estimate population error. No broad capacity producer, fitting, new-body search or manuscript build was run.

## Disposition

Stop this fixed selector as a uniformly tight predictor and as a universally HKO-sharp ratio bound. Its geometric mechanism and the exact obstructions belong in research notes. Do not promote them into Chapter 8: Jörn's existing answer keeps predictor counterexamples outside the manuscript, and a failed rule does not supply the requested working upper-tail predictor. The parent insight objective remains open; no global research freeze or thesis PASS follows. Further work needs a distinct support-selection mechanism rather than another scan or tuning of this one.

The existing 105-page manuscript/Kai attachment remains unchanged. A stale PDF hash and verification pointer in the distance-to-submission guide are corrected to the already integrated local-search candidate; this is not a new build or a new human verdict.

[review: /root/intake_review]
