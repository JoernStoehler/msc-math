# Phase 2: Interesting Reeb Trajectories for Viz

**Status:** Blocked on mathematical theory development
**Priority:** Medium (thesis advisors want to see interesting orbits)
**Depends on:** Primal-dual reconstruction theory (see below)

## Context

The viz webapp currently shows:
- ✅ Vertices, edges, ridges of 4D polytopes
- ✅ Volume, capacity, systolic ratio in info panel
- ✅ Individual trajectory toggles (infrastructure ready)
- ❌ Only 1 boring placeholder trajectory per polytope

**Goal:** Replace placeholder with interesting trajectories that demonstrate:
1. Minimum-action Reeb orbits (from HK2017 algorithm)
2. Displaced trajectories showing rotation/twisting phenomenon

## Blocker: Missing Mathematical Theory

**Problem:** HK2017 algorithm outputs dual solution (S, σ, β), not the primal trajectory γ(t).

**What exists:**
- ✅ Algorithm computes (S, σ, β) = (best_subset, best_permutation, best_beta)
- ✅ Can trivially compute capacity: c_EHZ = ∑β_i
- ✅ Know which facets are visited and in what order

**What's missing:**
- ❌ No formula to reconstruct the actual trajectory γ: [0,T] → K
- ❌ Specifically: need to compute primal critical γ from dual critical z
- ❌ Then: need to compute z from algorithm's (S, σ, β)

**Required deliverable before this task can proceed:**
- Theorem in `thesis/` proving how to reconstruct γ from (S, σ, β)
- Implementation in `crates/hk2017/src/lib.rs`: `pub fn reconstruct_trajectory(...) -> Trajectory`
- Tests verifying primal-dual correspondence

## Phase 2 Tasks (Once Math Exists)

### Task 2.1: Implement Min-Action Orbit Reconstruction

**Input:** HK2017 result (S, σ, β)
**Output:** Actual trajectory γ(t) as VizTrajectory

**Files to modify:**
- `crates/datasets/src/viz_export.rs`: Call `hk2017::reconstruct_trajectory()`
- `experiments/viz/viz.js`: Render with distinctive styling (gold, thicker line)

**Acceptance criteria:**
- Min-action orbit trajectory visible in viz
- Labeled as "Min-action orbit (HK2017)" with checkbox toggle
- Info panel shows: capacity, action, facet sequence, closed/open status

### Task 2.2: Implement Displaced Trajectories

**See:** `experiments/viz/displaced-trajectory.md` for full design

**Summary:**
- For a Reeb orbit γ, compute displaced trajectory γ̃(t) = γ(t) + v(t)
- Show that v(t) "rotates" around 0, i.e., γ̃ twists around γ
- Demonstrates the geometric intuition behind the tube algorithm

**Acceptance criteria:**
- User can toggle displaced trajectory on/off
- Visually distinguishable from main orbit (different color, thinner line)
- Info panel shows displacement magnitude, rotation indicator

### Task 2.3: Remove Placeholder Trajectory

**Once tasks 2.1 and 2.2 are complete:**
- Remove the boring facet-centroid trajectory generation
- Update `generate_trajectories()` to return only interesting trajectories
- Regenerate all polytope data files

## Development Workflow

1. **Math development** (work with Jörn in separate session/worktree):
   - Prove primal-dual reconstruction theorem
   - Implement and test `reconstruct_trajectory()`
   - Merge to main

2. **Viz integration** (this task):
   - Pick up after math is merged
   - Implement tasks 2.1, 2.2, 2.3 above
   - Test with all polytopes
   - Deploy to GitHub Pages

## Success Criteria

The viz webapp shows:
- ✅ Minimum-action orbit (reconstructed from HK2017)
- ✅ Displaced trajectory demonstrating rotation
- ✅ Clear labels and toggles for each trajectory type
- ✅ No boring placeholder trajectories

Thesis advisors (Kai & Elizabeth) can:
- Compare sys>1 polytopes (HKO Pentagon) vs sys<<1 polytopes (hypercube)
- See the rotation/twisting that motivates the tube algorithm
- Understand geometric intuition behind EHZ capacity

## References

- HK2017 algorithm: `crates/hk2017/src/lib.rs`
- Current viz export: `crates/datasets/src/viz_export.rs`
- Detailed experiment design: `experiments/viz/displaced-trajectory.md`
- Tube algorithm (uses rotation): `crates/tube/src/lib.rs`

## Notes for Next Agent

- The UI infrastructure is ready (checkboxes, info panel, text input)
- The blocker is purely mathematical, not technical
- Once `reconstruct_trajectory()` exists, the viz integration is straightforward
- Test with multiple polytopes to ensure robustness (simplex, hypercube, pentagon)
- GitHub Pages auto-deploys from `docs/viz/` when merged to main
