# HKO Local Maximum Decisions

- Primary proof route is exact first-order in `R^40` with a backend-agnostic witness contract. Paper-derived reductions are kept as validation/explanation until the larger exact route is executable and provably load-bearing.
- Default exact CAS is SageMath. The route is not bound to Sage implementation details as long as artifacts follow the same machine-readable contract.
- The theorem-facing field is the quartic extension `t = tan(pi/5)` with minimal polynomial `t^4 - 10 t^2 + 5 = 0`; no forced fallback to `Q(sqrt(5))`.
- Dual coordinates are handled in the `a_i`-space (`K = {x : a_i·x ≤ 1}`) to avoid gauge artifacts from `(n,h)` parameterization.
- For first-order certificates, witness artifacts must include exact active rows, row counts/ranks, kernel basis, and symmetry-inclusion checks; floating-point active-set discovery is not acceptable as theorem input.
- The 6240 directed-feasible sigma route is a valid cleanup target but is currently too slow in current exact tooling to be the default; use it only after concrete backend-speed justification.
- `subdifferential-lp/phase_c_lp_test.py` is intentionally not active because it is still bound to the pre-migration `(n,h)` output schema; refreshing it requires schema migration and likely delegation to `gradient-analysis`-produced gradient exports.
- In neighborhood falsification, retain `perturbation-neighborhood/` and `lagrangian-boundary/` as complementary ambient tests:
  - no broad blind search is the default for new counterexample claims;
  - fixed-combinatorics perturbations (`facet-splitting/`, `cut-and-ascent/`) are separate hypotheses with no `sys` improvement in committed runs.
- `pentagon-perturb.jsonl` remains historical context; analyzer entry points should rely on committed new-format smoke data or `data/licca-eps-*.jsonl` when available.
- If exact coverage stalls on asymmetry representatives, it is acceptable to pause and revisit the reduced route only after documenting that blocker, rather than introducing ad-hoc representative heuristics outside current contract.
