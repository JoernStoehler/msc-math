# Displaced Trajectory Visualization

**Status:** Planned (not yet implemented)
**Goal:** Visualize rotation/twisting of Reeb flow to motivate the tube algorithm
**Audience:** Thesis advisors (Kai Cieliebak, Elizabeth Gaar)

## Mathematical Setup

### Reeb Flow and Rotation

Given a Reeb orbit γ: [0, T] → K (where γ is a closed or periodic generalized Reeb orbit on polytope K):

1. **Original orbit**: γ(t) following the Reeb vector field R_i on facet i
2. **Displaced trajectory**: γ̃(t) starting from γ̃(0) = γ(0) + εv, where ε ≪ 1 and v ∈ ℝ⁴

**Key observation** (piecewise affine flow):
- If γ̃ has the same combinatorics as γ (visits the same sequence of facets in [0, T + δT]), then:
  ```
  γ̃(t) = γ(t) + v(t)
  ```
  where the displacement vector v(t) evolves linearly on each facet.

**Rotation phenomenon**:
- The displacement v(t) "rotates" around 0 as t varies
- This means γ̃(t) **twists around** γ(t)
- This rotation is what the **tube algorithm** exploits to compute capacity via linearized flow

### Why This Matters

- **Viterbo's conjecture** involves systolic ratio sys(K) = c_EHZ(K)² / (2 vol(K))
- The tube algorithm uses rotation of nearby trajectories to bound volume
- Visualizing this rotation makes the geometric intuition concrete for the thesis committee

## Visualization Plan

### What to Show

**Primary view** (4D → 3D via stereographic projection):
1. **Main orbit γ(t)** in distinctive color (e.g., bright blue, thicker line)
2. **Displaced trajectory γ̃(t)** in contrasting color (e.g., orange, thinner line)
3. **Animation option**: Highlight current positions γ(t₀) and γ̃(t₀) with spheres, animate t₀ from 0 to T

**Secondary view** (displacement vector evolution):
- 2D plot of ‖v(t)‖ vs t showing periodic/quasi-periodic behavior
- OR: Plot v(t) projected to a 2D plane to show rotation

### Implementation Strategy

#### Phase 1: Single Displaced Trajectory (Minimum Viable Viz)

**Input**:
- A known closed Reeb orbit γ (e.g., minimum-action orbit from HK2017)
- Small displacement vector v₀ ∈ ℝ⁴ (manually chosen, e.g., v₀ = 0.01 · e₁)

**Computation**:
1. Start from γ̃(0) = γ(0) + v₀
2. Use existing `reeb_trajectory::simulate()` to compute γ̃
3. Check combinatorics match: compare facet sequences of γ and γ̃
4. Export both trajectories to JSON with metadata flag `is_displaced: true`

**Visualization** (in experiments/viz/):
- Render both trajectories with different colors/widths
- Add toggle: "Show displaced trajectory"
- Info panel: Display displacement magnitude, rotation indicator

**Expected result**:
- For small enough ε, γ̃ should stay close to γ
- Visual inspection should show γ̃ winding around γ

#### Phase 2: Multiple Displacements (Full Rotation Visualization)

**Enhanced computation**:
- Generate k displaced trajectories: v₀ = ε · (cos(2πj/k), sin(2πj/k), 0, 0) for j = 0, ..., k-1
- Create a "tube" of trajectories around γ

**Enhanced visualization**:
- Render tube surface by connecting displaced trajectories with triangular mesh
- Animate: Show cross-section at time t moving along γ
- Color-code by rotation angle or displacement magnitude

#### Phase 3: Theoretical Connection (Thesis Integration)

**Compute and display**:
- Rotation number: How many times v(t) rotates around 0 in time T
- Compare with theoretical prediction from linearized flow
- Show agreement/discrepancy → motivation for tube algorithm refinement

## Implementation Checklist

### Data Generation (Rust)

- [ ] Add function to `reeb_trajectory.rs`: `simulate_displaced(polytope, gamma, displacement_v0) -> DisplacedTrajectory`
  - Returns both γ and γ̃, plus metadata (combinatorics match, rotation count)
- [ ] Extend `viz_export.rs`: Add `displaced_trajectories: Vec<DisplacedTrajectory>` to JSON
  - For now: Compute 1 displaced trajectory per minimum-action orbit

### Visualization (JavaScript)

- [ ] Update `viz.js`: Render displaced trajectories with distinct styling
  - Main orbit: Thick blue line
  - Displaced: Thin orange line, slightly transparent
- [ ] Add UI toggle: "Show displaced trajectories" checkbox
- [ ] Info panel: Display displacement info when showing displaced trajectories

### Validation

- [ ] **Hypercube test**: Known minimum-action orbit, compute displaced trajectory
  - Verify combinatorics match for small ε (e.g., ε = 0.01)
  - Check γ̃ stays close to γ (max distance < 0.1)
- [ ] **Simplex test**: Simple case for debugging
- [ ] **HKO Pentagon test**: Complex case to show interesting rotation

### Thesis Integration

Location: `thesis/experiments/displaced-trajectory.tex`

- [ ] Section 1: Mathematical setup (Reeb flow, displacement, rotation)
- [ ] Section 2: Visualization methodology (stereographic projection, color coding)
- [ ] Section 3: Results (screenshots for hypercube, simplex, pentagon)
- [ ] Section 4: Connection to tube algorithm (motivation, next steps)

## Technical Challenges and Solutions

### Challenge 1: Combinatorics Mismatch

**Problem**: For large ε, γ̃ may hit different facets than γ

**Solution**:
- Start with very small ε (0.001 × diameter of K)
- Binary search: Find largest ε where combinatorics still match
- Report this ε_max in thesis as "stability radius"

### Challenge 2: Non-Closed Displaced Trajectory

**Problem**: Even if γ is closed, γ̃ might not close (due to numerical drift or rotation)

**Solution**:
- Compute closure gap: ‖γ̃(T) - γ̃(0)‖
- If gap is small (< 0.01), still visualize but mark as "approximately closed"
- Discuss in thesis: Connection to monodromy and rotation number

### Challenge 3: Visualization Clarity in 4D → 3D Projection

**Problem**: Twisting might not be visible in stereographic projection

**Solution**:
- Try multiple north pole positions (e₄, e₁, diagonal)
- Add "best view" preset that maximizes visible separation
- Alternative: Animate time parameter to show motion

## Expected Deliverables

### For Thesis Committee Meeting

1. **Interactive webapp** (experiments/viz/index.html):
   - Load hypercube, show minimum-action orbit (gold)
   - Toggle on "displaced trajectory" (orange)
   - Rotate view to show twisting clearly

2. **Static figures** (thesis/experiments/):
   - Figure 1: Hypercube orbit + displaced trajectory (two views)
   - Figure 2: Displacement vector magnitude ‖v(t)‖ vs time
   - Figure 3: Comparison across polytopes (simplex, hypercube, pentagon)

3. **Thesis section** (2-3 pages):
   - Explains rotation phenomenon
   - Shows experimental validation
   - Motivates tube algorithm as next step

### Future Extensions (Post-Thesis)

- Compute rotation number analytically from linearized flow
- 3D visualization of v(t) trajectory in displacement space
- Interactive parameter tuning (adjust ε, v₀ direction)
- Connection to Maslov index and symplectic topology

## Notes and Open Questions

### Mathematical Questions

1. **Q**: Does rotation number depend on choice of v₀, or only on γ?
   - **Hypothesis**: Only depends on γ (up to orientation)
   - **Test**: Try multiple v₀ directions, compare rotation numbers

2. **Q**: What is the relationship between rotation and capacity?
   - **Hypothesis**: Higher rotation → smaller tubes → lower volume bound
   - **Test**: Compare rotation numbers for orbits with different actions

3. **Q**: Can we use rotation to distinguish orbits?
   - **Hypothesis**: Minimum-action orbit has minimal rotation
   - **Test**: Compute rotation for multiple orbits, compare with action

### Implementation Questions

1. **Q**: Should displacement be in all 4 dimensions or restricted to a subspace?
   - **Proposal**: Try both (a) v₀ ∈ q-space (Lagrangian), (b) v₀ ∈ general direction
   - **Rationale**: Lagrangian displacements preserve certain symplectic properties

2. **Q**: How to choose T (period) for non-closed orbits?
   - **Proposal**: Use max_segments from simulation as proxy for "one period"
   - **Alternative**: Detect approximate return via ‖γ(t) - γ(0)‖ < threshold

## References

- Haim-Kislev 2017: Generalized Reeb orbits and EHZ capacity
- Chaidez-Haim 2021: Tube algorithm using linearized Reeb flow
- Cieliebak-Frauenfelder 2009: Symplectic topology and action functionals

## Timeline (If Implemented)

- **Week 1**: Rust implementation (displaced trajectory simulation)
- **Week 2**: JavaScript visualization (rendering + UI)
- **Week 3**: Validation and debugging (test all polytopes)
- **Week 4**: Thesis writeup (LaTeX figures and explanations)

**Estimated effort**: 2-3 weeks full-time (or 4-6 weeks part-time)

---

**Created**: 2026-02-14
**Last updated**: 2026-02-14
**Status**: Design document, awaiting implementation
