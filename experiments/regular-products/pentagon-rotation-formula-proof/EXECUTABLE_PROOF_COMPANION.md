# Pentagon Rotation Executable Proof Companion

This is a companion note for thesis writing and review. It is not thesis prose.
It collects the facts, proof structure, runtime record, and explanation advice
needed to write the thesis section without re-reading the full code and older
experiment notes.

## Status

As of 2026-06-04, the open half-domain computational exclusion has a complete
SageMath executable certificate:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

The full run completed successfully in the worktree

```text
/workspaces/msc-math/.worktrees/pentagon-field-spike
```

Run provenance:

```text
SageMath version: 10.7
worktree: /workspaces/msc-math/.worktrees/pentagon-field-spike
stdout artifact: experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

Current command to reproduce the full certificate:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500
```

Current development-prefix command:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Full-run output source:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

That artifact records the current full run:

```text
open_domain_raw_sigma_count = 3340
classified_raw_sigma_count = 3340
CERTIFICATE PASSED in 2126.34s
```

Use the stdout artifact as the source for exact status counts. Do not maintain
copied count tables in this companion.

Post-cleanup prefix checks:

```text
--limit 5: 4.08s
--limit 50: 14.09s after transition-table cleanup
```

This run supersedes an earlier `1902.57s` full run. The earlier run accepted a
coarser status named `not_strictly_feasible_open`. Review found that this was
not enough: a beta can fail to be positive on the whole open interval while
still being positive on a subinterval. The current script cuts the open interval
at all beta, `Q`, and gap zeros/poles and checks every feasible cell.

This should be enough evidence to avoid rerunning the full certificate during
ordinary thesis writing. Rerun only after changing the script, Sage version,
facet conventions, enumeration logic, or the claimed formula.

## One-Page Proof Map

Use this as the high-level order before filling in details:

1. State the formula for `sys(P_5 x_L R(theta)P_5)`.
2. Reduce the parameter range to `0 <= theta <= pi/10` using the regular
   pentagon symmetry.
3. Note that volume is invariant under rotating the second factor.
4. Compute the active 2-bounce branch and obtain the candidate value
   `((5 + 2*sqrt(5)) / 10) * sec(theta)^2`.
5. Reduce the lower-bound problem to a finite list of raw 2- and 3-bounce
   sigmas.
6. Use exact transition-sign constancy to prune to 3340 open-domain raw sigmas.
7. For each raw sigma, solve the exact KKT system over `Frac(K[t])`.
8. Cut the interval at all relevant beta, `Q`, and gap zeros/poles.
9. On every feasible cell, prove either no feasibility or positive action gap.
10. Use continuity and endpoint checks for `theta = 0` and `theta = pi/10`.
11. Reflect by symmetry to finish `0 <= theta <= pi/5`.

This is an outline for writing, not a substitute for the final thesis argument.

## Target Formula

Half-domain:

```text
sys(P_5 x_L R(theta)P_5)
  = ((5 + 2*sqrt(5)) / 10) * sec(theta)^2
```

for

```text
0 <= theta <= pi/10.
```

Full fundamental domain:

```text
sys(P_5 x_L R(theta)P_5)
  = ((5 + 2*sqrt(5)) / 10) * sec(min(theta, pi/5 - theta))^2
```

for

```text
0 <= theta <= pi/5.
```

The executable proof handles the open half-domain

```text
0 < theta < pi/10
```

via exact rational functions in `t = tan(theta/2)`. The endpoint and symmetry
arguments should be explained separately in the thesis.

## Field Choice

Expression field:

```text
Frac(K[t])
K = maximal totally real subfield of CyclotomicField(20)
t = tan(theta/2)
```

Reason:

1. `QQ` is not enough because the unrotated regular pentagon already contains
   `cos(k*pi/10)` and `sin(k*pi/10)`.
2. `t = tan(theta/2)` makes `sin(theta)` and `cos(theta)` rational functions in
   `t`.
3. Therefore KKT solutions, betas, actions, and gaps are rational functions over
   the pentagon coefficient field.
4. `CyclotomicField(40)` or `AA` is needed only for exact endpoint/sign checks
   involving `tan(pi/20)`.

## What The Code Checks

The script asserts:

1. the facet convention used in Sage matches the experiment convention;
2. the intended active branch has action
   `((1 + cos(pi/5))^2) / cos(theta)`;
3. the systolic-ratio prefactor simplifies to `(5 + 2*sqrt(5)) / 10`;
4. all ordered facet-pair transition signs are constant on
   `0 < t < tan(pi/20)`;
5. the open-domain transition-pruned raw sigma count is `3340`;
6. representative statuses behave as expected in preflight;
7. in the classification loop, every raw sigma receives one of the explicitly
   accepted final statuses.

The decisive full-run assertions are:

```python
assert len(sigmas) == 3340
assert classification.status in accepted_statuses, classification
```

The script prints `CERTIFICATE PASSED` only when no `--limit` is used.
Development runs use the same assertions with `--limit N` and print
`LIMITED PREFIX PASSED`.

## Code Symbol Dictionary

```text
t
  tan(theta/2), the function-field parameter.

K
  maximal totally real subfield of CyclotomicField(20).

F
  Fraction field Frac(K[t]).

DUALS
  ordered list of the ten dual facet vectors in coordinates (q1, q2, p1, p2).

sigma
  cyclic raw facet sequence, after block enumeration and transition pruning.

beta
  KKT convex coefficients for the active facets in sigma.

Q_sigma
  quadratic expression computed by q_value(sigma, beta).

action_sigma
  1 / (2 Q_sigma), when Q_sigma is nonzero.

minimum_action
  ((1 + cos(pi/5))^2) / cos(theta), the active branch action.

gap
  action_sigma - minimum_action.

SignCertificate
  exact sign result on the open half-domain plus endpoint label.

accepted_statuses
  the closed set of statuses allowed in a certificate or limited-prefix run.
```

## Sigma Enumeration

The proof index is raw sigmas, not canonical signatures.

Raw-sigma route:

```text
raw sigma -> KKT system -> rational branch expressions -> exact sign certificate
```

This avoids proving that a canonical quotient preserves feasibility, action
gaps, and endpoint behavior.

Counts:

```text
structural 2-bounce sigmas: 7200
structural 3-bounce sigmas: 43200
open-domain transition-pruned raw sigmas: 3340
```

The transition pruning is exact in the final script. Earlier sampled sweeps are
only sanity checks and should not be presented as proof input.

## Sign Certification

Each relevant expression is a rational function in `t`.

For an expression `f(t) = p(t)/q(t)`, the script:

1. finds real roots of `p` and `q` in `0 < t < tan(pi/20)` after conversion to
   Sage's real algebraic field `AA`;
2. samples one exact algebraic point in each resulting interval;
3. evaluates the exact sign on each interval;
4. records the endpoint sign separately.

This proves positivity or negativity on the open interval because a rational
function can change sign only at a zero or pole.

For full branch exclusion, the script forms the combined cut set from all beta
functions, `Q_sigma`, and the action gap. It checks every cell where all betas
and `Q_sigma` are positive. A branch is excluded only if no feasible cell exists
or if the gap is positive on every feasible cell.

## Status Meanings

`no_kkt_solution`:
The KKT linear system is inconsistent. There is no critical branch for this raw
sigma.

`zero_q_identity`:
The quadratic quantity `Q_sigma(t)` is identically zero, so this sigma does not
produce a finite positive action branch. The script accepts this only when the
KKT solve is not hiding a singular positive-beta branch.

`singular_kkt_forced_zero_beta`:
The KKT system is singular, but every solution has at least one beta coordinate
forced to be identically zero. This cannot be a strictly feasible branch on the
open interval.

`not_feasible_on_open_domain`:
After cutting the open interval at all beta, `Q`, and gap zeros/poles, there is
no cell where all betas and `Q` are positive. This rules out hidden feasible
subintervals.

`zero_gap_identity`:
The branch action equals the minimum action as an identity. These are symmetry
or duplicate raw-sigma representatives of the same minimum value, not lower
competitors.

`strict_gap_positive_on_feasible_open_domain`:
The branch has one or more feasible open cells, and on every feasible cell its
action minus the minimum action is positive.

`requires_manual_review`:
Fallback status for any case not covered by the previous classes. The full run
proved that this status never occurs.

## Thesis Explanation Order

Recommended order for writing:

1. State the formula and reduce to `0 <= theta <= pi/10` by pentagon symmetry.
2. State volume invariance under rotation.
3. Present the active 2-bounce branch and derive the upper bound formula.
4. Explain why the remaining task is excluding all other KKT branches on the
   open half-domain.
5. Define the finite raw-sigma proof index: 2- and 3-bounce block sigmas.
6. State the transition-sign constancy lemma on the open half-domain.
7. Explain the exact function field and why rational sign checks are decisive.
8. State that the Sage certificate classifies all 3340 raw sigmas using
   feasible-cell checks.
9. Give the six status counts and explain why each status cannot beat the
   active branch.
10. Handle endpoints and the mirrored half-domain separately.

## Likely Reader Questions

Question: Why is `QQ` not enough?

Answer to include: the pentagon normals already contain cyclotomic real
constants such as `cos(k*pi/10)`.

Question: Why is this not a numerical proof?

Answer to include: all KKT expressions are rational functions in an exact
function field, and signs are decided by exact algebraic root isolation.

Question: Why do sampled theta sweeps not enter the proof?

Answer to include: sampled sweeps motivated the formula and sanity-check the
picture, but the final certificate enumerates and signs exact branches.

Question: Why are canonical signatures not used?

Answer to include: raw sigmas give a shorter proof path. Canonical signatures
are useful for plots and human summaries, but would add an extra proof
obligation.

Question: What about endpoints?

Answer to include: the Sage classification proves the open half-domain. The
endpoint should be handled by continuity and the separately understood
endpoint tie structure.

Question: What should Kai trust?

Answer to include: the finite proof obligation is completely encoded in the
source. The code has exact field construction, exact branch solving, exact sign
certification, hard count assertions, and a cached successful full-run output.

## What To Put In Thesis

Put in thesis:

1. the theorem statement;
2. the reduction to half-domain;
3. the active branch calculation;
4. the finite-certificate method;
5. the field choice;
6. the sign-certification lemma for rational functions;
7. the final counts and status interpretation;
8. the exact command or a footnote/reference to the repository artifact.

Probably keep out of thesis unless space allows:

1. detailed raw sigma examples;
2. progress-line output;
3. all preflight-only details;
4. performance tuning discussion;
5. canonical-signature alternatives.

## Confidence And Nuance

High confidence:

1. the script completed the full 3340-sigma run;
2. the exact sign-certification method is mathematically valid for rational
   functions;
3. the field choice is adequate and explainable;
4. raw sigmas are the clearest proof index.

Needs careful thesis wording:

1. the relation between the KKT branch classification and the global billiard
   trajectory classification;
2. endpoint handling;
3. the use of continuity and genericity;
4. the meaning of singular KKT systems and forced zero betas;
5. the distinction between no feasible cell and positive gap on feasible cells.

Avoid wording:

1. do not say the script numerically samples signs;
2. do not say the sampled sweeps prove the formula;
3. do not imply `zero_gap_identity` gives new lower competitors;
4. do not hide the open-domain versus endpoint distinction.
5. do not use the superseded status `not_strictly_feasible_open` as a proof
   status.

## Review Checklist Before Merging

1. Recheck that `classify_sigma` still accepts only the explicit status set
   after any future edits.
2. Check whether the endpoint continuity argument is already strong enough in
   formal notes.
3. Check whether `formal/pentagon-rotation-capacity.tex` should reference this
   executable certificate after Jörn review.
4. Check whether a generated run log should be stored separately or whether this
   companion note is sufficient.
