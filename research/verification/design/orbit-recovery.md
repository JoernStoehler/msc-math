# Orbit Recovery Validation: Logbook

## Motivation

The EHZ algorithm (Algorithm `alg:ehz`) returns a facet sequence S and weights beta. Base point recovery (Lemma `lem:base-point-recovery`) reconstructs the starting point b = gamma(0) of the corresponding Reeb orbit on the polytope boundary. This experiment validates that the recovered orbit is geometrically consistent: it closes up, stays on the boundary of K, and has the correct action.

## Status

**Current local-first validation run available.** The experiment now defaults
to a curated 25-row target pool:

- 7 literature-known polytopes,
- 8 random shared-cache rows (one per facet-count stratum `F=5..12`),
- 10 lagrangian-product shared-cache rows (one per polygon-pair stratum).

The binary loads the three 170-row mirror candidates as read-only inputs,
writes any locally produced rows to `cache-extension.jsonl`, and records real
`solution_dim` values. The most recent local run on 2026-04-16 passed all
25/25 rows.

The older 112-row known+random run is no longer the default experiment
identity.

## How to run

```bash
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery   # generates orbit-recovery.jsonl
uv run analyze.py                                # prints summary statistics
uv run plot_orbit_recovery.py                    # generates error plot
```

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: recovery + validation for each polytope |
| `analyze.py` | Python: summary statistics (printed to stdout) |
| `plot_orbit_recovery.py` | Python: error distribution plot for non-known rows |
| `formal/verification/orbit-recovery.tex` | Formal writeup (lemma, error table, solution dimension) |
| `cache-extension.jsonl` | Experiment-owned cache overlay for locally produced rows |
| `orbit-recovery.jsonl` | Validation dataset output for the current curated target pool |
| `orbit_recovery_errors.png` | Error distribution by facet count figure |

## Design

### Dataset

Current default dataset:

- 7 literature-known low-cost polytopes from `known_polytopes::all_known()`,
  excluding the crosspolytope.
- 8 random shared-cache rows chosen by stable `Source::Random` provenance:
  one representative row per facet count `F = 5..12`.
- 10 lagrangian-product shared-cache rows chosen by exact key:
  one representative row per polygon pair `(n_1, n_2)` with `3 <= n_1 <= n_2 <= 6`.
- Total: 25 polytopes.

Lookup / production policy:

- Shared mirrors:
  - `experiments/verification/orbit-recovery/polytopes.jsonl`
  - `experiments/combinatorial-cells/polytopes.jsonl`
  - `experiments/sys-landscape/cache.jsonl`
- Shared mirrors are read-only search inputs.
- Missing local rows are written only to `cache-extension.jsonl`.
- Cache reuse requires `capacity`, non-empty `sigmas`, and
  `|sigmas[0].action - capacity| <= 1e-10`.

### Validation procedure

For each polytope: resolve one minimum-action EHZ result, recompute `beta`
from the chosen best permutation, recover the base point `b` from
optimal `(S, beta)`, reconstruct the orbit `gamma`, and measure four error
metrics (per Lemma `lem:finite-orbit-verification` in
`formal/verification/orbit-recovery.tex`):
1. **Closure error:** ||gamma(T) - gamma(0)||
2. **On-facet error:** max distance of any breakpoint from its assigned facet
3. **Inequality violation:** max violation of <n_i, gamma(t_j)> <= h_i across all facets and breakpoints
4. **Action error:** |A(gamma) - c_EHZ(K)|

### Thresholds

- 1e-6 for closure, on-facet, and inequality violation.
- 1e-5 for action error (which accumulates rounding over the full orbit).

### Solution dimension

Base point recovery solves N_S b = r, where N_S collects outward normals of active facets. dim = 4 - rank(N_S). When active normals are linearly independent, b is unique.

## Findings

Current 25-row curated run (2026-04-16):

1. **25/25 polytopes pass** the validation thresholds.

2. **The acceptance-check matrix is covered in one run:**
   - 10 exact-key hits from the lagrangian-product strata,
   - 8 provenance (`Source`) hits from the random strata,
   - 7 local computations from the known-polytopes slice.

3. **The non-unique base-point cases are now visible in the dataset.**
   Solution-dimension counts:
   - dim = 0: 21 rows
   - dim = 1: 1 row (`lagrangian_tri_sq`)
   - dim = 2: 3 rows (`hypercube`, `symplectic_triangle_product`, `symplectic_tri_sq`)

4. **Worst observed errors remain far below the thresholds.**
   - closure: `3.74e-12`
   - on-facet: `5.23e-13`
   - inequality violation: `2.70e-12`
   - action: `3.02e-14`

5. **Recovery remains cheap relative to minimum-action resolution.**
   On this run:
   - capacity/minimum-action stage: `0.4s` total, `17.0ms` mean,
   - orbit recovery stage: `0.099ms` mean.

## Known limitations

- This is a curated validation surface, not an exhaustive sweep over the full
  170-row shared cache.
- The current figure still compresses all non-known rows into one facet-count
  plot; it is useful as a quick scan, not as a complete family-by-family
  summary.
- Validation thresholds are empirical (`1e-6` geometry, `1e-5` action).
