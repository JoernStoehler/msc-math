# Gradient Correctness: Logbook

## Motivation

The library provides analytical gradients ∂sys/∂a_k via `capacity_derivatives_a` and `volume_derivatives_a`. These are used by multiple optimization and analysis experiments. This experiment validates the gradients under increasingly adversarial conditions to understand where they're reliable and where they break down.

## Status

**Q1-Q4 complete.** Data generated 2026-03-26. Figures produced.

## Research questions

1. **Generic polytopes:** Does the analytical gradient match finite differences? What sampling strategy works in R^{4F} — along gradient, random directions, coordinate-aligned? How does accuracy depend on FD step size and polytope dimension?

2. **Non-generic geometry:** Is the gradient correct for Lagrangian products (which have symmetry-degenerate orbits) and other polytopes with symmetry groups? What about polytopes where multiple orbits achieve near-identical action?

3. **Near-degeneracy:** What happens when the gap between the best and second-best orbit action is small? How does the gradient behave as the minimizer becomes non-unique?

4. **Redundant halfspaces:** If we introduce a halfspace that barely cuts the polytope, what happens to the gradient for that facet?

## Design notes

- A shared validation harness (analytical vs FD comparison, error metrics) should serve all phases.
- Question 3 likely needs instrumentation beyond just "different polytopes" — e.g., logging the action gap, tracking which orbit the solver picks on each FD perturbation.
- Existing `capacity_derivatives_a_fd` and `volume_derivatives_a_fd` in `crates/src/derivatives.rs` provide FD baselines. Currently tested only on the hypercube.
- Check the derivative lemmas in `experiments/sys-optimization/math.tex` and `crates/src/derivatives.rs` for what assumptions the formulas rely on.

## How to run

```bash
cd experiments/
cargo run --release --bin gradient_correctness        # all phases
cargo run --release --bin gradient_correctness -- q2 q3 q4  # skip Q1
python3 gradient-correctness/analyze.py               # figures + summary
```

## Results (2026-03-26)

### Dataset sizes

| Phase | Rows | Polytopes | Time |
|-------|------|-----------|------|
| Q1 generic | 4680 | 120 (F=5..10, 20 each) × 13 eps × 3 targets | ~45 min (F=10 dominates) |
| Q2 non-generic | 48 | 16 Lagrangian products (F≤8) × 1 eps × 3 targets | 14s |
| Q3 near-degenerate | 219 | 73 at F=6 (binned by action gap) × 1 eps × 3 targets | 37s |
| Q4 barely-cutting | 150 | 10 base × 5 δ values × 1 eps × 3 targets | 25s |

### Key findings

**1. Capacity gradient: validated to machine-precision accuracy.**

Analytical ∂c/∂a_k matches FD to median relative error ~2e-9 across all 4 phases (gc_q1_step_sweep.png, gc_summary.tex). Step-size sweep shows textbook V-curve with sweet spot at eps≈3e-6, confirming O(eps²) central-difference truncation balanced against O(machine_eps/eps) roundoff.

**2. Volume and sys gradients: FD validation breaks on small facets.**

Median errors for ∂vol/∂a_k and ∂sys/∂a_k are 5e-3 to 2e-2 at eps=1e-5, with P95 reaching 1.0 (gc_q1_dimension.png). Investigation shows the max_rel_error is driven by facets with tiny analytical gradient norms — the FD returns zero for these because `Polytope4D::from_f64` on the perturbed dual vertices changes the combinatorial type (vertex appears/disappears). When filtering facets below 1% of max gradient norm, volume median error drops to 2.6e-3 and max to 8.8e-2.

The volume step-sweep sweet spot is shifted to eps≈3e-4 (vs 3e-6 for capacity), reflecting that volume is computed via qhull vertex enumeration and is only piecewise smooth in dual vertex space. This is not a bug in the analytical gradient — it's a fundamental FD limitation for piecewise-smooth functions.

**3. Non-generic geometry (Q2): no structural issues.**

Lagrangian products (regular, rotated, random) show comparable or better errors than generic polytopes for all three targets (gc_q2_nongeneric.png). Symmetry-degenerate orbits do not cause gradient degradation.

LP(5,5) and LP(4,5) (F=9-10) were excluded from Q2 due to runtime — already covered by Q1 generic at F=9-10.

Random LP construction frequently fails (Unbounded, RedundantFacet) at small polygon counts. Only LP(4,4) produced valid random instances.

**4. Near-degeneracy (Q3): orbit switching correlates with large errors.**

When the action gap between best and second-best orbit is small (< 1e-4), FD perturbations sometimes switch the optimal orbit, making the capacity gradient prediction incorrect for those polytopes (gc_q3_gap_vs_error.png, gc_q3_orbit_switching.png).

For polytopes without orbit switching, capacity errors remain ~1e-9 regardless of gap size. For polytopes with orbit switching, capacity errors jump to ~1e-1 — this is expected behavior at non-smooth points of the capacity function.

**5. Barely-cutting facets (Q4): volume gradient degrades.**

As δ→0 (facet becomes near-redundant), volume and sys FD errors increase systematically (gc_q4_delta_vs_error.png). Capacity errors remain low (~1e-9) because barely-cutting facets are rarely in the optimal orbit.

### Known Q-correction panic

Many FD perturbations (especially on Lagrangian products) trigger the known Q-correction panic in `saddle_point_solver.rs:504-509`. These are caught via `catch_unwind` and result in NaN FD components. This does not affect the analytical gradient — it only means some FD perturbations cannot be evaluated.

## Predecessor experiments

This experiment supersedes the gradient validation aspects of sys-optimization (Phases 1, 2, 4). The optimization aspects of sys-optimization (Phase 3) move to a separate experiment.
