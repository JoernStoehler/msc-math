# Handoff: HKO Neighborhood Phase D — Second-Order Analysis

Phase C established the first-order necessary condition for local max: 0 ∈ conv(subdifferential) in F=10 (n,h)-space via LP. But 16 flat directions remain — along these, first-order information is zero and second-order analysis is needed to determine max vs saddle.

## Goal

Determine whether HKO2024 is a second-order local maximum along the 16 flat directions identified by Phase C. This is the critical gap between "first-order necessary condition" and "actual local max."

## Context

- Phase C LP (phase_c_lp_test.py) found 0 ∈ conv(44 per-orbit gradients) in a 40D effective space (50D ambient minus 10D gauge). Residual ~7e-9 (computational, not a proof).
- 16 flat directions = null space of the LP constraint that places 0 in the convex hull. Along these directions, all 44 gradient components are zero → first-order test is inconclusive.
- The dual-vertex parameterization would simplify this: clean R^40 space, no gauge. But Phase D can proceed in (n,h)-space if needed.

## Draft plan

1. **Read** hko-neighborhood/logbook.md Phase C section and phase_c_lp_test.py to understand the 16 flat directions
2. **Extract** the 16-dimensional null space basis from the LP solution
3. **For each flat direction v_i:** compute sys(HKO + ε·v_i) for ε ∈ {-0.01, -0.001, ..., 0.001, 0.01} via finite differences
4. **Fit** a quadratic sys(ε) ≈ sys₀ + c₂·ε² to each direction. If c₂ < 0 for all 16 → second-order local max along flat directions.
5. **Also check** 2D cross-sections between flat directions for mixed second derivatives

## Draft verification

- At ε=0, sys should match HKO baseline (≈1.0472)
- Quadratic fits should have R² > 0.99 if the curvature is clean
- If any c₂ > 0 or c₂ ≈ 0: that direction needs further investigation (cubic term? numerical noise?)

## Risks and unknowns

- **Numerical precision at small ε:** sys is computed via KKT solver with ~1e-11 precision. At ε=1e-4, Δsys might be O(1e-8), which is near noise floor. May need to use larger ε or higher-precision arithmetic.
- **Flat directions in (n,h)-space vs a_i-space:** The 16 flat directions were computed in (n,h)-space including gauge directions. In a_i-space there might be fewer true flat directions. Consider redoing Phase C in a_i-space first (cleaner, but depends on dual-vertex math.tex migration).
- **"16 flat directions" might be an artifact of gauge freedom.** If 10 of the 16 are gauge directions, only 6 are geometrically meaningful. Investigate before computing 16 separate quadratic fits.

## Dependency

Not strictly blocked on anything, but would benefit from dual-vertex parameterization (removes gauge ambiguity in flat direction analysis). Consider whether to do this in (n,h)-space now or wait for a_i migration.
