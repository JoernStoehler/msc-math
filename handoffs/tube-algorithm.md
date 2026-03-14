# Task: Implement tube algorithm in Rust

## Context

The tube algorithm is an alternative to HK2017 for computing EHZ capacity on symplectic polytopes (no Lagrangian 2-faces). It builds trajectory families ("tubes") incrementally via branch-and-bound, potentially making F > 10 polytope datasets feasible. The spec is written; implementation is placeholder-only.

## Scope

Implement the 9 steps from `tube-algorithm-plan.md`, each as implement → test → document:

1. **DirectedSkeleton** — compute directed edges from ω₀ signs
2. **Step map Φ_{ijl}** — affine map for single facet transition
3. **Tube data structure** — (Start, End, φ, a, ρ)
4. **2D polygon intersection** — H-rep utilities
5. **Tube extension** — compose step maps along facet sequence
6. **Rotation number computation** — CH2021 transition matrices
7. **Pruning predicates** — empty, action bound, rotation bound, simplicity
8. **Closing & fixed point computation** — check for periodic orbits
9. **Top-level search** — DFS with pruning

Final verification: `tube_capacity(K) == ehz_capacity(K)` on all test polytopes with F ≤ 10.

## Out of scope

- Thesis .tex writeup (code first, thesis follows)
- Performance optimization beyond correctness
- F > 10 dataset generation (that's a separate experiment after the algorithm works)
- Changes to the KKT solver or (n, h) parameterization

## Key files

Spec and plan:
- `/workspaces/msc-math/tube-spec.md` — detailed algorithmic spec (has open questions marked `<q>`)
- `/workspaces/msc-math/tube-algorithm-plan.md` — 9-step implementation plan with test criteria
- `/workspaces/msc-math/tube-notes.md` — Jörn's raw dictation notes

Current placeholder:
- `/workspaces/msc-math/crates/src/algorithms/tube/mod.rs` (placeholder)
- `/workspaces/msc-math/crates/src/algorithms/tube/tube_test.rs` (placeholder test)

Existing infrastructure to reuse:
- `/workspaces/msc-math/crates/src/geom/polytope.rs` — Polytope4D, H-rep
- `/workspaces/msc-math/crates/src/geom/symplectic.rs` — ω₀, J₀
- `/workspaces/msc-math/crates/src/geom/reeb_trajectory.rs` — Reeb vector (direction only; R = (2/h) J₀ n)
- `/workspaces/msc-math/crates/src/geom/skeleton.rs` — ridge enumeration
- `/workspaces/msc-math/crates/src/algorithms/hk2017/mod.rs` — directed adjacency builder (pub(crate))

Archaeology (untrusted, reference only):
- `/workspaces/msc-math/archaeology/raw/code/archive__tube.rs` (39KB, prior attempt with known bugs)
- `/workspaces/msc-math/archaeology/raw/code/reverted__tube.rs` (20KB, simplified prior attempt)

## Prior findings

- The spec has open questions (`<q>` markers) in sections 3, 5, 6, 8 — these need Jörn's input before implementing those steps. Steps 1-2 can proceed immediately.
- The archaeology code had known bugs: trivialization issues, orbit validation gaps. Don't copy it — use it for understanding the approach only.
- The `reeb_vector()` function returns J₀ n (direction only). The full Reeb vector is R = (2/h) J₀ n — callers must include the 2/h factor.
- `build_directed_adjacency_matrix()` in HK2017 is pub(crate) — may need to be made pub or duplicated for tube's DirectedSkeleton.

## Success criteria

- Each step has unit tests that pass
- `cargo test --lib` passes with zero failures
- `cargo clippy --lib -- -D warnings` passes
- Final integration test: `tube_capacity(K)` agrees with `ehz_capacity(K)` for all polytopes in the test fixtures (simplex, hypercube, crosspolytope, HKO pentagon, etc.)
- Code has doc comments with thesis cross-references where applicable
