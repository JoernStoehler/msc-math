<!--
Purpose: reference record for the exact-arithmetic replacement refactor.
Context: this is supporting material for `tasks/current-state.md` and
`tasks/planning-notes.md`, not a live task file and not an active checklist.
-->

# Exact Arithmetic Replacement 2026-05-10

## Status

- State: merged into `main` at `e2fcc9b1`.
- Former branch: `delete-algebraic-crate`.
- Implementation head reviewed before adding the review record: `f3dba7ec`.
- Merge-review commit: `e2fcc9b1`.

This refactor was not literal deletion of `crates/algebraic-numbers/`. It
replaced the old exact-arithmetic API/design attractor with a smaller exact
scalar and dense exact-linear-algebra crate.

Current task summaries are in `tasks/current-state.md` and
`tasks/planning-notes.md`. The current source truth is the code plus
`crates/algebraic-numbers/README.md`,
`crates/algebraic-numbers/DEVELOPMENT.md`, and `crates/MAP.md`.

## Outcome

`crates/algebraic-numbers/` now owns:

- exact scalar arithmetic for `BigRational` and statically chosen real
  algebraic fields;
- exact ordering for `Algebraic<F>`;
- dense exact row reduction, rank, kernel basis, linear solve, and
  negative-definite checks over nalgebra dynamic matrices.

Domain code remains outside the generic crate:

- KKT semantics stay in `crates/symplectic/src/kkt/` and experiment KKT modules.
- Symplectic geometry, capacity/orbit logic, and exact-validation workflows stay
  in `crates/symplectic/` or the owning experiment.
- Experiment serialization, field tags, and reporting conversions stay in the
  experiment packages.

## Verification Run

Review commands run on 2026-05-10 from the isolated branch worktree:

| check | result |
| --- | --- |
| `cargo test -p algebraic-numbers` | pass |
| `cargo clippy -p algebraic-numbers --all-targets -- -D warnings` | pass |
| `cargo run -p algebraic-numbers --example q_sqrt5_vector` | pass |
| `cargo check -p symplectic` | pass |
| `cargo test -p symplectic --lib exact::` | pass, 6 passed |
| `cargo test -p symplectic --lib kkt::rational_solver` | pass, 8 passed and 1 ignored |
| `cargo check -p dev-numerical-analysis` | pass |
| `cargo test -p dev-numerical-analysis` | pass, 27 passed |
| `cargo check -p exp-hko-local-maximum` | pass |
| `cargo check --workspace` | pass |
| retired old API import grep from the branch checklist | pass |
| duplicate generic exact-linear-algebra grep from the branch checklist | pass |

Post-merge on `main`, `cargo check --workspace` passed.

## Review Conclusion

No follow-up was required before merge. The branch completed its scoped Rust/API
ownership gate.

This does not by itself assert thesis-wide mathematical acceptance of all exact
validation claims. Future thesis-facing exact-validation claims still need the
normal topic-level proof, data, and writing checks.

## Historical Notes

The pre-merge done checklist and merge-review report used to live as separate
root task files:

- `tasks/delete-algebraic-crate-done.md`
- `tasks/delete-algebraic-crate-review-2026-05-10.md`

They were folded into this reference because task root files are for topic
bundles, while this was one refactoring packet under the broader Rust
maintainability objective.
