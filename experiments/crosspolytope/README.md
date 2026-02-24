# Crosspolytope Capacity Computation

## Goal

Compute the EHZ capacity of the 4D crosspolytope (hyperoctahedron), filling in the
placeholder value in `crates/src/geom/known_polytopes.rs:127`.

## Background

The 4D crosspolytope is `conv{±e₁, ±e₂, ±e₃, ±e₄}`, dual to the hypercube. It has:
- **16 facets** (one per sign vector in {±1}⁴)
- **8 vertices** (±eᵢ)
- **Non-simple**: each vertex lies on 8 facets (a simple 4-polytope has exactly 4)

No literature value for c_EHZ(crosspolytope) is known to us. The placeholder in
`known_polytopes.rs` uses `capacity: 1.0` and is excluded from all validation tests.

## Timing estimate (2026-02-24)

Extrapolated from `experiments/benchmark/benchmark.jsonl` (F=5..12, pruned A3, release mode):

| Algorithm | Estimated time (F=16) | Basis |
|-----------|----------------------|-------|
| A0 (unpruned) | ~800 days | Super-exponential: ratio(F) ≈ F−1.18, giving ~3.7 trillion iterations |
| A3 (pruned) | ~4 hours (range 1–13h) | 4.59× per facet from F=8..12 exponential fit |

The crosspolytope is non-simple, which may help pruning (the two non-simple polytopes
in the ablation dataset at F=10 had ~2× fewer A3 iterations than random generic polytopes).
However, the crosspolytope's high symmetry (hyperoctahedral group, order 384) could go
either way — it might create many equivalent Reeb orbits that pruning can't distinguish,
or the symmetry might mean most orbit types are quickly pruned.

**Conclusion:** A3 (pruned) in release mode is feasible as a one-off computation.

## Experiment plan

### Phase 1: Compute capacity

Write a standalone Rust binary `crosspolytope.rs` that:
1. Constructs the crosspolytope using `known_polytopes::crosspolytope()`
2. Calls `ehz_capacity()` (which uses pruned A3 by default)
3. Prints: capacity value, iteration count, wall-clock time, orbit details (σ, β)
4. Writes result to `crosspolytope.jsonl`

Run in release mode with generous timeout:
```bash
cd experiments/ && timeout 14h cargo run --release --bin crosspolytope
```

### Phase 2: Update known_polytopes.rs

After obtaining the capacity value:
1. Replace `capacity: 1.0` placeholder with the computed value
2. Update `source:` to reference this experiment
3. Remove the exclusion from validation tests in `test_dataset.rs`
4. Regenerate the test fixture: `cargo test --release regenerate_test_dataset -- --ignored`
5. Run the full test suite to verify

### Phase 3: Cross-checks

- Compute `sys = c² / (2·vol)` for the crosspolytope
- Check: does the crosspolytope satisfy Viterbo's conjecture (sys ≤ 1)?
- The hypercube (dual polytope) has capacity 4.0 and sys = 1.0 exactly.
  By Mahler's inequality, the crosspolytope might have interesting sys properties.
- Record the orbit structure: how many facets does the minimum-action orbit visit?
  The crosspolytope's high symmetry suggests the orbit may have a nice geometric interpretation.

### Phase 4 (optional): Symmetry analysis

The hyperoctahedral group acts on the crosspolytope. The minimum-action orbit either:
- Is unique up to symmetry (typical case), or
- Has a continuous family of minimizers (degenerate case, possible due to non-simplicity)

If the orbit count is unusually large or there are near-ties, document this.

## Files

| File | Purpose |
|------|---------|
| `crosspolytope.rs` | Rust binary: compute capacity |
| `crosspolytope.jsonl` | Output: computed capacity and metadata |
| `README.md` | This file |

## Dependencies

- `symplectic` crate (for `ehz_capacity`, `known_polytopes`, `volume`)
- Release mode required (debug mode is infeasible at F=16)
