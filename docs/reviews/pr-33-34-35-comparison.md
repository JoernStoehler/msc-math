# PR 33 vs 34 vs 35 — comparison and merge decision

## Scope baseline
All three branches fork from commit `c169c4f` and address module split/refactor work around geometry vertex enumeration and related solver organization.

## Common ground
- All three preserve successful library buildability after refactor-level file movement.
- All three keep `geom::vertex_enumeration` functionality and tests in place.
- PR 33 and PR 34 both replace `geom/vertex_enumeration.rs` with a directory module.
- PR 35 keeps `geom/vertex_enumeration.rs` as the entrypoint and introduces submodules inside that file's module.

## Key differences

### PR 33
- Focused split of vertex enumeration into:
  - `boundedness.rs`
  - `enumerate.rs`
  - `linear_algebra.rs`
  - `pipeline.rs`
  - `prefilter.rs`
  - nested tests directory
- Test outcome: fails `cargo test --release --lib` (3 failing tests) with unbounded cases returning `RedundantFacet(..)` instead of `Unbounded`.

### PR 34
- Follow-up refactor to PR 33 structure with clearer responsibilities:
  - `exact_linalg.rs` (renamed/specialized exact linear algebra)
  - `irredundancy.rs` (renamed from pipeline semantics)
  - flattened tests into `tests.rs`
- Also updates adjacent geometry integration points:
  - `geom/qhull.rs`
  - `geom/validation.rs`
  - `geom/math.tex` (math-code correspondence docs)
- Test outcome: passes `cargo test --release --lib` and `cargo clippy --lib -- -D warnings`.

### PR 35
- Broader "responsibility scaffolding" across multiple modules:
  - `geom/polytope/*` split
  - `kkt/rational_solver/*` split
  - `kkt/saddle_point_solver/*` split
  - `geom/vertex_enumeration` internal split via `core`, `linear_solver`, `test_helpers`
- Much smaller net line churn than PR 33/34, oriented to ownership boundaries rather than vertex-enumeration-only follow-ups.
- Test outcome: passes `cargo test --release --lib` and `cargo clippy --lib -- -D warnings`.

## Decision
Merge **PR 34** and close **PR 33** and **PR 35**.

Reasoning:
1. PR 33 is disqualified by red tests.
2. PR 34 directly addresses the same vertex-enumeration refactor as PR 33 and includes the review follow-up fixes.
3. PR 35 is valid, but it is a different breadth-level change (cross-cutting scaffolding) and overlaps less with the specific vertex-enumeration review thread that PR 33/34 share.
4. PR 34 includes corresponding `math.tex` updates, which keeps the project math-code correspondence tighter for this refactor.

## Verification commands used
- `git fetch https://github.com/JoernStoehler/msc-math.git pull/33/head:pr-33 pull/34/head:pr-34 pull/35/head:pr-35`
- `git diff --name-status <merge-base>..<pr-branch>` per PR
- `cargo test --release --lib` in each PR worktree
- `cargo clippy --lib -- -D warnings` in PR 34 and PR 35 worktrees
