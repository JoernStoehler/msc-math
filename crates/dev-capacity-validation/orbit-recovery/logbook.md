# Orbit Recovery Validation: Logbook

## Motivation

The EHZ algorithm (Algorithm `alg:ehz`) returns a facet sequence S and weights beta. Base point recovery (Lemma `lem:base-point-recovery`) reconstructs the starting point b = gamma(0) of the corresponding Reeb orbit on the polytope boundary. This experiment validates that the recovered orbit is geometrically consistent: it closes up, stays on the boundary of K, and has the correct action.

## Status

**Complete.** 112/112 polytopes pass all validation checks. Recovery is fast (~0.025ms) and numerically stable up to F = 10.

## How to run

```bash
cd crates/dev-capacity-validation/orbit-recovery/
cargo run --release --bin axioms-orbit-recovery   # generates orbit-recovery.jsonl
python3 analyze.py                                # prints summary statistics
python3 plot_orbit_recovery.py                    # generates error plot
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: recovery + validation for each polytope |
| `analyze.py` | Python: summary statistics (printed to stdout) |
| `plot_orbit_recovery.py` | Python: error distribution plot by facet count |
| `math.tex` | Formal writeup (lemma, error table, solution dimension) |
| `orbit-recovery.jsonl` | Dataset (112 rows) |
| `orbit_recovery_errors.png` | Error distribution by facet count figure |

## Design

### Dataset

- 7 known polytopes from the literature.
- 105 random polytopes: 20 each for F = 5..8, 15 for F = 9, 10 for F = 10.
- Seed 42, h in [0.8, 1.2], ChaCha8Rng.
- Total: 112 polytopes.

### Validation procedure

For each polytope: compute c_EHZ, recover base point b from optimal (S, beta), reconstruct the orbit gamma, and measure four error metrics (per Lemma `lem:finite-orbit-verification` in math.tex):
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

1. **112/112 polytopes pass** all validation checks.

2. **Known polytopes achieve machine epsilon errors (~1e-15).** Random polytopes have errors below 1e-8 for F <= 9. At F = 10, accumulated floating-point error raises max action error to 1.83e-6, still within the 1e-5 threshold.

3. **Error scaling by facet count (max errors, random polytopes only):**
   - F=5 (N=20): closure 1.1e-9, on-facet 3.1e-10, violation 3.1e-10, action 4.4e-9
   - F=6 (N=20): closure 8.5e-11, on-facet 5.5e-12, violation 2.0e-11, action 4.5e-11
   - F=7 (N=20): closure 2.5e-11, on-facet 5.4e-12, violation 1.5e-11, action 3.7e-11
   - F=8 (N=20): closure 6.7e-11, on-facet 1.7e-11, violation 1.7e-11, action 1.2e-10
   - F=9 (N=15): closure 1.7e-10, on-facet 3.1e-11, violation 6.5e-12, action 1.6e-10
   - F=10 (N=10): closure 3.7e-7, on-facet 6.8e-8, violation 6.8e-8, action 1.8e-6

4. **Base point is generically unique:** 108/112 (96.4%) have dim = 0. The 4 exceptions are all known polytopes with product structure: hypercube (dim = 2), both symplectic products (dim = 2), and Lagrangian triangle-square (dim = 1). No random polytope exhibits non-uniqueness, consistent with this being a measure-zero condition.

5. **Recovery is fast:** mean 0.025ms, negligible compared to capacity computation (mean 4.7ms).

## Known limitations

- Only F <= 10 tested; numerical error grows with F and may exceed thresholds for larger polytopes.
- Validation thresholds chosen empirically.
- The 1e-5 action tolerance is looser than other thresholds due to accumulated rounding over the full orbit.
