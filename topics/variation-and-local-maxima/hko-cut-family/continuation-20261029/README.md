# HKO rational-panel continuation

This continuation tests two retained feasible characteristic words at
`t=7/20, 2/5, 9/20`. It adds exact KKT solutions and exact volume evaluations
to the previously unexecuted panel in the parent HKO handoff. The calculation
does **not** enumerate all words at these parameters, so every displayed ratio
is an upper bound for `rho(H_t)`, not a capacity value.

`panel.py` reconstructs the 20 support rows over `Q(tau)`, solves the equality
KKT system for each literal word, checks exact closure/normalization, and
requires all weights positive. It enumerates the feasible four-row vertices
and computes volume by the Lawrence tangent-cone formula, checked with two
generic linear functionals as in the frozen `hko-t03/exact_geometry.py`.
The two repeated volume calculations per parameter agree exactly. Output is
retained outside the repository at
`/tmp/msc-math/viterbo-continuation-20261029/hko/panel.json`.

| `t` | word | exact `Q` | exact volume | feasible-word `rho` upper bound |
|---:|---|---|---|---:|
| 7/20 | 7-label `(0,1,6,7,3,12,9)` | `(395/1961) tau` | `-2199353203 tau^2/5781662400 + 6644112217/1156332480` | 1.0525519384 |
| 7/20 | 8-label `(0,6,5,2,19,11,8,4)` | see JSON | same as above | 1.0523455989 |
| 2/5 | 7-label `(0,1,6,7,3,12,9)` | see JSON | `-1968673 tau^2/5198400 + 5954147/1039680` | 1.0531573781 |
| 2/5 | 8-label `(0,6,5,2,19,11,8,4)` | see JSON | same as above | 1.0517836035 |
| 9/20 | 7-label `(0,1,6,7,3,12,9)` | see JSON | `-613821503 tau^2/1628542400 + 1858768317/325708480` | 1.0537400754 |
| 9/20 | 8-label `(0,6,5,2,19,11,8,4)` | see JSON | same as above | 1.0510639132 |

Each body had 75 vertices and all 20 listed rows active. Compared to the
certified `rho(H_{3/10})=1.0519141388294875...`, the 8-word feasible bounds at
`2/5` and `9/20` lie below the benchmark. This rules out those two sampled
bodies as improvements; it says nothing about unsampled parameters. At `7/20`
the bound remains above the benchmark and leaves room. The gap of about
`0.00043` at `7/20` is numerically meaningful for selecting the next parameter,
but it is not an actual improvement because a feasible word is only an upper
bound.

`branches.py` derives the KKT weights and `Q` symbolically as rational
functions in `t` over `Q(tau)`; output is in the sibling `branches.json` under
the scratch run. It confirms the 7-word branch has simple rational `Q(t)` and
the 8-word branch has a quadratic-over-quadratic `Q(t)`. Sage's exact-real
root solver finds no weight-zero or denominator roots for either branch in
`(0,1)`. Since sampled weights are positive, each KKT solution remains
strictly feasible throughout that interval. The two `Q` branches cross once,
at `t≈0.3403557218556924`; for larger `t`, the 8-word branch gives the
stronger capacity upper bound. This is a crossover of these two witnesses,
not a complete-word transition. A complete search and better volume/capacity
control are still needed to locate any optimum or establish an improved body.

## Exact chamber formula on `[3/10,2/5]`

`chamber.py` enumerates the 75 feasible simple vertices at `t=3/10`, then
tracks each of those exact active four-row bases through the closed interval.
For every basis it checks that its determinant has no real zero there and
that all 16 inactive slacks are strictly positive throughout: each slack is a
rational function in `Q(tau)(t)`, its denominator has no interval zero, its
numerator has no interval root, and its sign at `3/10` is positive. The body
is bounded throughout because it remains inside the fixed bounded HKO body
`H`. Hence no vertex can be born or disappear without one of these bases
becoming singular or acquiring an additional active row. This first-event
argument proves the same 75 simple vertices persist throughout the interval.
The mechanism is elementary but should receive independent review before
acceptance as a proof.

The exact Lawrence formula, independently evaluated with two linear
functionals, gives

```text
V(t) = (A3*t^3 + A2*t^2 + A1*t + A0) /
       (t^3 - (40/3)*t^2 + (325/9)*t - 250/9),
A3 = -30379*tau^2/43200 + 26027/2880,
A2 =  6521*tau^2/1080  - 6265/72,
A1 = -25655*tau^2/1728 + 379705/1728,
A0 =  3125*tau^2/288 - 15625/96.
```

Combining this volume with `Q_7(t)` and `Q_8(t)` gives feasible-word ratio
upper bounds `R_j(t)=1/(8 V(t) Q_j(t)^2)`. Their exact derivative numerators
have no real roots on the interval: `R_7` is strictly increasing (derivative
at `3/10` is `0.01312475146`), while `R_8` is strictly decreasing (derivative
at `2/5` is `-0.01298575024`). The Q branches cross exactly once, at
`t*=0.3403557218556925...`; hence the stronger of the two bounds,
`min(R_7,R_8)`, has its maximum at this cusp. Its value is
`1.0524316650246792...`, versus `1.0519141388294875...` at the certified
`t=3/10` body. This is a useful parameter for a complete-word search: the
remaining possible gain is about `0.000518` in the ratio bound. It remains an
upper-bound mechanism, not an attained improvement or a capacity lower bound.
The simple rational `t=17/50` gives a nearly cusp-sized minimum bound
`1.0524272049215315` (`R_7`; `R_8=1.052434687499301`).
Exact symbolic outputs and root checks are in the scratch file
`/tmp/msc-math/viterbo-continuation-20261029/hko/chamber.json`.

Reproduce with the repository Sage interpreter:

```sh
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/continuation-20261029/panel.py
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/continuation-20261029/branches.py
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/continuation-20261029/chamber.py
```

The panel took approximately one minute on the workstation. This artifact has
not yet received an independent mathematical review. It is not part of the
selected thesis or the frozen report.
