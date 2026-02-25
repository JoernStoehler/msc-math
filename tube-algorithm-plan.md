# Plan: Ship §3 via incremental Rust-first approach

## Context

The notes→LaTeX approach failed because agents had to do too many things at once: understand Jörn's math, fill gaps, write LaTeX, maintain consistency across definitions, and anticipate review needs. Each concern degraded the others, producing "incoherent/wrong" definitions that required O(errors) rounds of Jörn's review time to fix.

**New approach:** notes → Rust code → writeup of assumptions the code makes. This separates concerns and gives agents automatic feedback (compiler, tests, assertions) instead of relying on Jörn for every verification step.

## Why this is better

| Concern | Notes→LaTeX (failed) | Notes→Rust→writeup (proposed) |
|---------|---------------------|-------------------------------|
| Math precision | Agent fills gaps from general knowledge → corruption | Compiler/tests reject wrong math immediately |
| Feedback loop | Only Jörn can verify → expensive | Tests verify → cheap |
| Assumptions | Hidden in LaTeX prose, easy to miss | Explicit in types, asserts, doc comments |
| Incremental progress | Each edit risks breaking everything | Each function is independently testable |
| Agent cognitive load | 5 concerns at once | 1 concern at a time |

## What exists in the crate

**Ready to use:**
- `Polytope4D` with normals, heights, vertices (`geom/polytope.rs`)
- `omega0()`, `j4()`, `j2()` (`geom/symplectic.rs`)
- `reeb_vector()` = J₀n_i direction, without 2/h_i factor (`geom/reeb_trajectory.rs`)
- `Skeleton::compute()` → `Ridge { facets: [i,j], vertices }` = 2-faces (`geom/skeleton.rs`)
- `build_directed_adjacency_matrix()` (`algorithms/hk2017/mod.rs`, `pub(crate)`)
- `solve_kkt()` (`kkt.rs`, `pub(crate)`)

**Needs building:**
- 2D polygon intersection (H-rep → H-rep)
- Affine map type (2D → 2D, or 4D → 4D restricted)
- Step map Φ_{ijl}
- Tube data structure (Start, End, φ, a, ρ)
- Tube extension
- Closing + fixed point computation
- Rotation number computation
- Pruning predicates
- Top-level tube search

## Incremental build order

Each step: implement in Rust → test → document assumptions in doc comments.

### Step 1: Precomputation types
- `DirectedSkeleton`: from `Skeleton` + `omega0` signs, produce directed edges i→j
- Test: directed edges are antisymmetric, cover all ridges, agree with existing `build_directed_adjacency_matrix`
- Assumption doc: "F_i→F_j iff ω₀(n_i,n_j) > 0 and F_i∩F_j is a 2-face"

### Step 2: Step map
- `StepMap { phi: Affine4D, delta_a: AffineScalar }` for a triple (i,j,l)
- Formula: Φ(x) = x + t(x)R_j where t(x) = (h_l - ⟨x,n_l⟩) / ⟨R_j,n_l⟩
- Test: Φ(x) ∈ H_j ∩ H_l for any x ∈ H_i ∩ H_j. Φ is affine. delta_a(x) > 0.
- Test against known polytopes: step maps compose correctly
- Assumption doc: "denominator ⟨R_j,n_l⟩ = (2/h_j)ω₀(n_j,n_l) > 0 since F_j→F_l"

### Step 3: Tube data structure
- `TubeData { start: Polygon2D, end_set: Polygon2D, phi: Affine2D, action: AffineScalar2D, rotation: f64, sigma: Vec<usize> }`
- Initialization for k=2: start = end = full 2-face polygon, phi = id, a = 0, ρ = 0
- Test: initialization produces non-empty start/end matching Ridge vertices
- Assumption doc: "Start ⊂ F_{σ(1)}∩F_{σ(2)}, End ⊂ F_{σ(k-1)}∩F_{σ(k)}"

### Step 4: 2D polygon intersection
- Intersect two convex polygons in H-rep (both in the same 2-face coordinate system)
- Test: intersection of two known polygons, empty intersection detection, subset detection
- This is a utility needed by tube extension

### Step 5: Tube extension
- Given TubeData + next facet l: compute End', Start', φ', a', ρ'
- Test: extended tube is subset of original (Start' ⊆ Start). Non-empty when expected.
- Cross-check: for small polytopes, compare tube extension results against brute-force trajectory enumeration

### Step 6: Rotation number
- Compute Δρ_{jl} for each directed edge j→l
- Based on CH2021 transition matrix ψ_{F_{jl}} = positive elliptic ∈ Sp(2)
- Test: Δρ ∈ (0, 1/2) for all directed edges. Rotation is additive.
- Assumption doc: "uses CH2021 Cor. 2.22; transition matrix is positive elliptic"

### Step 7: Pruning predicates
- `is_empty(tube) -> bool`
- `exceeds_action_bound(tube, c_star) -> bool`
- `exceeds_rotation_bound(tube) -> bool`
- `has_repeated_facet(sigma) -> bool`
- Test: pruning correctly prunes and doesn't prune known cases

### Step 8: Closing + fixed point
- Close tube by extending with σ(1), σ(2)
- Find fixed points of φ'' in Start'
- Compute action at fixed points
- Test: on known polytopes, closing produces orbits matching HK2017 results
- Cross-check: `ehz_capacity()` via tube algorithm == `ehz_capacity()` via HK2017

### Step 9: Top-level search
- DFS over directed edges with pruning
- Returns c_EHZ(K) for symplectic polytopes
- Test: matches HK2017 on all test polytopes

## LaTeX writeup (after Rust works)

Once the Rust code works and tests pass, the §3 writeup becomes:
- Each definition = what the corresponding Rust type/function does
- Each lemma = what the corresponding test verifies
- Each proof = why the code's assumptions are justified
- Assumptions listed in doc comments → transferred to LaTeX

This inverts the failed approach: instead of writing math and hoping it's right, we document verified code.

## What this session should do

This session has already spent significant Jörn time. The remaining useful work:
1. **Commit** the mechanical fixes made to tube-algorithm.tex (action formula, citation corrections, GAP markers)
2. **Decide** whether to start Step 1 (DirectedSkeleton) in this session or defer to a fresh session

## Verification

- Each step: `cargo test --lib` passes
- Final: `tube_capacity(K) == ehz_capacity(K)` for all test polytopes
- LaTeX: `cd thesis/ && latexmk && ./check-build.sh`
