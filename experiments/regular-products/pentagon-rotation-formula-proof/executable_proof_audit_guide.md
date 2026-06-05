# Executable Proof Audit Guide

Status: code-audit guide, not source truth and not thesis prose.

Read this file if you need to understand why the Sage script is a credible
executable proof without reading the whole script first.

Do not read this file for:

1. exact output, counts, or runtime; use `executable_proof.full.stdout.txt`;
2. final thesis wording; use `thesis/rotated-regular-polygons.tex`;
3. drafting order, figure choices, or writing warnings; use
   `thesis/rotated-regular-polygons-content.md`;
4. empirical motivation; use `../pentagon-rotation-empirics/README.md`.

## Source Truth

The executable proof source is:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

The full run transcript is:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

That transcript is the source for exact status counts and runtime. This guide
does not duplicate the count table.

Rerun the full proof only after changing the script, Sage version, facet
conventions, enumeration logic, or the claimed formula.

## Proof Obligation

The script proves the lower-bound part of the formula on the open half-domain

```text
0 < theta < pi/10.
```

The endpoint and mirror arguments are outside the executable certificate and
belong in the thesis writeup:

1. endpoints: EHZ Hausdorff continuity and constant volume;
2. mirror: equal odd-pentagon factor-swap symmetry.

## Audit Path

Read the Sage script in this order:

1. field setup: `K`, `t`, `cos_theta`, `sin_theta`;
2. geometry setup: `pentagon_normals`, `rotate`, `dual_vertices`,
   `assert_facet_conventions`;
3. active branch checks: `minimum_action`, `systolic_ratio_prefactor`,
   `assert_formula_checks`;
4. KKT branch solve: `kkt_matrix`, `solve_kkt_branch`, `q_value`;
5. sign checks: `real_roots_in_open_half_domain`, `sign_certificate`,
   `open_domain_cells`, `sign_at`;
6. enumeration: `blocks`, `enumerate_k_bounce_sigmas`,
   `transition_table_open`, `transition_pruned_sigmas_open`;
7. classification: `classify_sigma`;
8. top-level assertions: `run_preflight`, `run_certificate`.

This order follows the mathematical dependencies. It is usually cheaper than
reading the file top to bottom.

## Exact Field

The expression field is:

```text
Frac(K[t])
K = maximal totally real subfield of CyclotomicField(20)
t = tan(theta/2)
```

Reason:

1. `QQ` is not enough because the unrotated pentagon already contains
   `cos(k*pi/10)` and `sin(k*pi/10)`.
2. The parameter `t = tan(theta/2)` makes `sin(theta)` and `cos(theta)`
   rational functions in `t`.
3. KKT solutions, betas, actions, and gaps are therefore rational functions
   over the pentagon coefficient field.
4. Sage's real algebraic field `AA` is used for exact endpoint/root comparison
   involving `tan(pi/20)`.

## Sign Certification

Each relevant expression is a rational function in `t`.

For `f(t) = p(t)/q(t)`, the script:

1. finds real roots of `p` and `q` in `0 < t < tan(pi/20)`;
2. cuts the open interval at those roots and poles;
3. samples one exact algebraic point in each cell;
4. evaluates the exact sign on each cell.

This is decisive because a rational function can change sign only at a zero or
pole.

For branch exclusion, the script combines the cut sets from all beta
functions, `Q_sigma`, and the action gap. A branch is accepted as excluded only
when either no feasible cell exists, or the gap is positive on every feasible
cell.

## Enumeration

The proof index is raw sigmas, not canonical signatures.

```text
raw sigma -> KKT system -> rational branch expressions -> exact sign certificate
```

This avoids proving that a canonical quotient preserves feasibility, action
gaps, and endpoint behavior.

Hard count checks:

```text
structural 2-bounce sigmas: 7200
structural 3-bounce sigmas: 43200
open-domain transition-pruned raw sigmas: 3340
```

The transition pruning is exact in the final script. Sampled sweeps are only
sanity checks and exposition material.

## Accepted Statuses

`no_kkt_solution`:
The KKT linear system is inconsistent.

`zero_q_identity`:
`Q_sigma(t)` is identically zero, so the sigma does not produce a finite
positive action branch. The script accepts this only after checking that a
singular positive-beta branch is not hidden.

`singular_kkt_forced_zero_beta`:
The KKT system is singular, but every solution has at least one beta coordinate
forced to be identically zero. This cannot be strictly feasible on the open
interval.

`not_feasible_on_open_domain`:
After cutting at all beta, `Q`, and gap zeros/poles, there is no cell where all
betas and `Q` are positive.

`zero_gap_identity`:
The branch action equals the minimum action identically. These are symmetry or
duplicate raw-sigma representatives of the same minimum value, not lower
competitors.

`strict_gap_positive_on_feasible_open_domain`:
The branch has feasible open cells, and the action gap is positive on every
feasible cell.

`requires_manual_review`:
Fallback status for uncovered cases. The full run proves that this status never
occurs.

## Fail-Closed Checks

The decisive script assertions are:

```python
assert len(sigmas) == 3340
assert classification.status in accepted_statuses, classification
```

The script prints `CERTIFICATE PASSED` only when no `--limit` is used.
Development runs use the same assertions on a prefix and print
`LIMITED PREFIX PASSED`.

## Not Maintained Here

Do not add these to this guide:

1. full status count tables; use the stdout artifact;
2. thesis subsection plans; use `thesis/rotated-regular-polygons-content.md`;
3. figure recommendations; use the thesis companion and empirics README;
4. historical runtime narratives; keep only the current transcript pointer;
5. long Sage excerpts; use the script itself.
