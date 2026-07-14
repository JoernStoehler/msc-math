# Reader guide to the rotated-pentagon certificate

This is a non-runnable guide to
`executable_proof.sage.py`. The executable source and the retained full stdout
are the proof-packet owners; this file is only an audit route and must not be
cited as evidence.

## Mathematical checkpoints

| Reader question | Executable surface |
| --- | --- |
| What exact field contains the pentagon and rotation data? | Field setup through `C20`, `K`, `R`, `F`, and `HALF_DOMAIN_ENDPOINT`; trigonometric helpers `cos_units`, `sin_units`, `cos_theta`, and `sin_theta`. |
| Which polytope is being checked? | `pentagon_normals`, `dual_vertices`, `omega`, and `assert_facet_conventions`. The coordinate order is `(q1,q2,p1,p2)`. |
| How does one fixed word become an action? | `kkt_matrix`, `q_value`, and `solve_kkt_branch`. The normalization is `sum(beta)=1`, closure is `sum(beta_i a_i)=0`, and the action is `1/(2Q)`. |
| Where is the displayed branch checked? | `minimum_action`, `systolic_ratio_prefactor`, and `assert_formula_checks`. |
| Why are the sign decisions exact? | `real_roots_in_open_half_domain`, `sign_certificate`, `open_domain_cells`, and `sign_at`. |
| Why does the generated list match the theorem's block family? | `blocks`, `non_overlapping_selections`, and `enumerate_k_bounce_sigmas`. The preflight asserts 7,200 two-bounce and 43,200 three-bounce cyclic representatives before pruning. |
| Why is transition pruning safe on the open half-domain? | `facet_intersection_nonempty`, `transition_table_open`, and `transition_pruned_sigmas_open`. Every mixed sign is asserted to be strictly constant on the open interval. |
| How is every surviving competitor disposed of? | `classify_sigma` and `ACCEPTED_STATUSES`. Any unrecognized or non-positive-gap feasible case returns `requires_manual_review`, which the full loop rejects. |
| How is the run tied to its source and environment? | `main` rejects disabled assertions and prints the SageMath version, assertion state, source digest, and arguments before running preflight. |
| What does success mean? | `run_certificate` asserts all 3,340 distinct surviving words have accepted statuses. Only an unlimited run prints `CERTIFICATE PASSED`. |

## Proof boundary

The executable proves generic open-half-domain comparisons over the
rational-function field. KKT ranks can specialize at finitely many algebraic
parameters; roots and poles likewise form finite exceptional sets. The thesis
uses Hausdorff continuity of the EHZ capacity to fill all such interior
parameters and the endpoints. The script does not itself prove that continuity
step.

The script also assumes the theorem-level reduction from the full
Haim--Kislev family to the alternating family with two or three blocks of each
factor type.
Its 3,340/3,340 result is exhaustive relative to the generated list; the active
quadratic-program chapter supplies why that list contains a capacity maximizer.

The empirical JSONL, plots, Rust search code, and stale formal pentagon draft
are not proof inputs.

## Rerun

Prefix check:

```bash
PYTHONOPTIMIZE=0 sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50 --progress-every 0
```

Full certificate:

```bash
PYTHONOPTIMIZE=0 sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500
```

Regenerate `executable_proof.full.stdout.txt` after every source change. A
prefix run is not theorem evidence and never prints `CERTIFICATE PASSED`.
