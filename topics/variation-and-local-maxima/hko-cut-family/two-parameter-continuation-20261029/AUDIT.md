# Scoped audit: two-parameter HKO feasible-word candidate

## Scope and inspected sources

Read the HKO family and label conventions in `topics/variation-and-local-maxima/hko-cut-family/README.md`; the derivative and ordered-action formulas in `topics/capacity-and-qp/convex-systolic-optimization/sections/hko-ascent.tex`; the complete-simple KKT conventions in `certificates/hko-ascent/exact_core.py`; the local producer `topics/variation-and-local-maxima/hko-cut-family/two-parameter-continuation-20261029/probe.py`; and `/tmp/msc-math/viterbo-continuation-20261029/two-parameter/retention-controls.json`. I checked the single candidate word and its geometry only. I did not inspect or reproduce complete capacity coverage.

## Family matrix and sign

The producer uses

```text
T = [[0, -J2], [J2, 0]],       Nnew = a*N0 + b*N0*T,
N = stack(N0, Nnew).
```

This matches the HKO cut-family labels and sign. In block coordinates, `N0*T` sends a q row `(n,0)` to `(0,J2*n)` and a p row `(0,m)` to `(-J2*m,0)`. Along the established one-parameter path, `a=1-3t/5`, `b=t*cos(pi/5)`, and `cos(pi/5)=1/(sqrt(5)-1)`, so `Nnew=N0+tD` with the listed `D_i` signs. The candidate input is `a=99/125=0.792`, `b=(7/20)cos(pi/5)=0.28315594803123156`; it is a genuine two-parameter point, since at `t=7/20` the one-parameter value would be `a=0.79`.

The row symplectic matrix is `O=N J N^T`, with `J=[[0,I],[-I,0]]`. The producer forms `H` by copying the strict upper triangle of `O` symmetrically, then calculates `Q=beta^T H beta/2`. This is exactly `sum_{r<s} beta_r beta_s omega(N_r,N_s)` in the project convention. Its KKT block is `[H,C^T; C,0]` with `C=[N_word^T; 1]` and right side `(0,...,0; 0,0,0,0,1)`, the correct stationarity equations for this quadratic under closure and unit sum.

## Exact check of the selected branch

For `a=99/125`, `b=7(sqrt(5)+1)/80`, and word `(0,6,5,2,19,11,8,4)`, I rebuilt all 20 normals in `Q(tau)` from the exact HKO normals and transformation above. Exact field elimination of the 13-by-13 KKT system gives the following weights in word order; each pair is the constant and `tau^2` coefficient:

```text
(21084374019229/211484711619848,  5711255949601/211484711619848)
(30296287895147/211484711619848,  4641360049887/211484711619848)
(21084374019229/211484711619848,  5711255949601/211484711619848)
(12149998051887/52871177904962, -1238407319936/26435588952481)
(720212711000/26435588952481,   -55669680000/26435588952481)
(720212711000/26435588952481,   -55669680000/26435588952481)
(12149998051887/52871177904962, -1238407319936/26435588952481)
(30296287895147/211484711619848,  4641360049887/211484711619848)
```

Every weight is strictly positive in the isolated real embedding `0<tau<1`; exact closure is zero and exact weight sum is one. Thus this is a true feasible positive-Q witness, not merely a near-KKT floating proposal. Its exact action is

```text
Qword = (5328025872481/26435588952481)*tau
      - (3162037824/26435588952481)*tau^3
      = 0.14638692947403203... .
```

The exact audit was a bounded solve for this one word. It does not check any other branch or compare against `Qmax`.

## Geometry control and interpretation

An independent NumPy reconstruction of the candidate rows enumerated all 4-row intersections, filtered by all 20 inequalities, rounded to `1e-10`, and applied SciPy `ConvexHull`. With the same stated tolerances (`|det|>1e-10`, feasibility `<=1+1e-8`) it returned 75 vertices, 20 numerically supported rows, and volume `5.54083616531392`, agreeing with the retained control. This is a reproducible numerical geometry control, not an exact volume certificate; the tolerance/rounding pipeline could hide close combinatorial events.

The floating scan uses 2,326 frozen leaves from the certified `t=0.3` packet plus two retained branch words. At the candidate it reports 425 numerically feasible bank words, selects the exact-feasible word above, and reports `rho_upper_estimate≈1.0527617598`. A feasible Q witness gives `Qmax>=Qword`, hence `c_EHZ<=1/(2Qword)` and (with exact volume) an upper bound on the systolic ratio. The reported ratio uses a floating volume, so it is only an approximate upper estimate. Reusing a finite word bank at this new body does not establish completeness. Since this upper estimate is above HKO's exact ratio `(3+sqrt(5))/5≈1.0472135955`, it does not establish an improvement; the actual capacity could be larger than `Qword` and the actual ratio lower.

## Reproduction/status

The candidate is present in `retention-controls.json`; the original runner is `two-parameter-continuation-20261029/probe.py`. Its default controls are `t=0.3,0.35` and its candidate is exact input `(99/125, (7/20)cos(pi/5))`. The local run remains a feasible-word-bank search only. The exact branch calculation and the independent NumPy geometry spot check were made in `/tmp/msc-math/viterbo-continuation-20261029/two-parameter/audit_candidate.py`; no source or candidate files were changed for this audit.
