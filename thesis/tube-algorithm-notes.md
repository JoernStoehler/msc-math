# Task: Implement tube algorithm in Rust

Stale as of 2026-05-04. Do not use this file as the tube-algorithm
specification or implementation plan. The current fillable source note is
`research/tube-algorithm.md`; use that note first, then compare old TeX/code
surfaces only as downstream or conflict material.

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
- (deleted) `tube-spec.md` — detailed algorithmic spec with open questions marked `<q>` (removed before the Codex migration)
- (deleted) `tube-algorithm-plan.md` — 9-step implementation plan with test criteria (removed before the Codex migration)
- (deleted) `tube-notes.md` — Jörn's raw dictation notes (removed in post-migration cleanup)

Current placeholder:
- `/workspaces/msc-math/library/src/algorithms/tube/mod.rs` (blocked implementation with inline tests)

Existing infrastructure to reuse:
- `/workspaces/msc-math/library/src/geom/polytope.rs` — Polytope4D, H-rep
- `/workspaces/msc-math/library/src/geom/symplectic_form.rs` — ω₀, J₀
- `/workspaces/msc-math/library/src/geom/reeb_trajectory.rs` — Reeb vector (direction only; R = (2/h) J₀ n)
- `/workspaces/msc-math/library/src/geom/skeleton.rs` — ridge enumeration
- `/workspaces/msc-math/library/src/algorithms/hk2017/mod.rs` — directed adjacency builder (pub(crate))

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
