<!--
Purpose: merge-readiness review report for the `delete-algebraic-crate` branch.
Context: records enough evidence to decide whether follow-up work is needed
before merging the exact-arithmetic replacement branch.
-->

# Delete Algebraic Crate Review 2026-05-10

## Verdict

State: merge-ready, pending Jorn's normal merge approval.

Recommended follow-up: nothing before merge. The branch should be merged as the
exact-arithmetic replacement branch, not treated as literal deletion of
`crates/algebraic-numbers/`.

Reviewed implementation head before adding this report: `f3dba7ec`
(`delete-algebraic-crate`).

Associated done definition: `tasks/delete-algebraic-crate-done.md`.

Review stance: completion and merge-readiness review against the branch done
definition, not a new architecture audit.

## Decision Surface

| option | recommendation | reason |
| --- | --- | --- |
| Just merge | yes, after Jorn approval | done checklist passed; no blocking stale old-API references or duplicate generic exact-linear-algebra helpers were found; docs and source agree on the branch role |
| Merge after small fix | not needed | review found no blocking code, docs, or task-map fixes |
| Hold for more tests | not needed for this branch gate | the branch-specific cargo, grep, example, and doc-review checks passed; broader release/thesis verification remains outside this branch gate |
| Re-scope to literal crate deletion | no | task history explicitly accepts this as an exact-arithmetic replacement branch, and current thesis-relevant consumers use the crate |

## Review Evidence

Commands run from `/workspaces/msc-math/.codex/worktrees/delete-algebraic-crate`
on 2026-05-10:

| check | result | notes |
| --- | --- | --- |
| `cargo test -p algebraic-numbers` | pass | crate tests and doctest passed |
| `cargo clippy -p algebraic-numbers --all-targets -- -D warnings` | pass | no clippy warnings |
| `cargo run -p algebraic-numbers --example q_sqrt5_vector` | pass | example ran successfully |
| `cargo check -p symplectic` | pass | durable consumer compiles |
| `cargo test -p symplectic --lib exact::` | pass | 6 passed, 367 filtered |
| `cargo test -p symplectic --lib kkt::rational_solver` | pass | 8 passed, 1 ignored, 364 filtered |
| `cargo check -p dev-numerical-analysis` | pass | active numerics consumer compiles |
| `cargo test -p dev-numerical-analysis` | pass | 27 passed |
| `cargo check -p exp-hko-local-maximum` | pass | HKO exact-bank consumer compiles |
| `cargo check --workspace` | pass | workspace compiles |
| retired old API grep from done definition | pass | no matches in `crates` or `experiments` |
| duplicate generic exact-linear-algebra grep from done definition | pass | no matches in `crates` or `experiments` |

Non-command review passes:

- Read `tasks/delete-algebraic-crate-done.md` and checked that its status,
  inventory, and checklist match the current branch shape.
- Reviewed the public crate surface in `crates/algebraic-numbers/src/lib.rs`,
  `README.md`, and `DEVELOPMENT.md` against the small API gate.
- Reviewed targeted diffs for `linear_solve`, `row_reduction`, ordering,
  definiteness, and algebraic element storage.
- Reviewed targeted diffs for migrated consumers in `crates/symplectic`,
  `experiments/numerics`, and `experiments/hko-local-maximum`.
- Checked navigation/task agreement in `crates/MAP.md`,
  `tasks/rust-tech-debt.md`, and `tasks/MAP.md`.

Review subagents: intentionally not used. The task was bounded by an explicit
done checklist, and local review plus command verification was sufficient.

## Findings

No blocking findings.

No old live imports of the removed public API names were found:
`OrderedField`, `TanPiFifth`, `canonical_element`, `CanonicalElement`,
`cmp_field`, `max_field`, `min_field`, `solve_square`, or `rank_rows`.

No obvious live duplicate generic exact-linear-algebra helpers named
`rank_rows`, `gauss_solve_with_null_space`, or `back_substitute` remain in
`crates` or `experiments`.

The retained local exact helpers are domain/workflow code rather than generic
crate ownership conflicts:

- `crates/symplectic/src/kkt/rational_solver.rs` keeps KKT assembly and
  positive-beta feasibility.
- `crates/symplectic/src/exact/` keeps exact symplectic geometry/orbit logic.
- `experiments/numerics/src/algebraic/` keeps experiment catalog, geometry,
  KKT workflow, and conversion conveniences.
- `experiments/hko-local-maximum/src/exact_bank.rs` keeps HKO field and bank
  conveniences.

## Caveats

The branch name is misleading. The accepted task meaning is documented in
`tasks/rust-tech-debt.md` and `tasks/delete-algebraic-crate-done.md`: this
branch replaces the old exact-arithmetic API/design attractor with a smaller
generic exact-scalar and exact-linear-algebra crate.

The command `cargo test -p symplectic --lib kkt::rational_solver` reports one
ignored test. This is not a new review blocker for this branch gate because the
done definition asks for the command to pass, and it passed with the ignored
test left ignored.

This review does not claim thesis-wide mathematical acceptance of all exact
validation claims. It only says the branch completed its scoped Rust/API
ownership gate and is ready for merge approval.

## Merge Notes

Before merging, the merge operator should check the target worktree status and
preserve unrelated local changes. At review time the associated worktree was
clean before this report was added.

After merge, likely cleanup is only ordinary worktree cleanup:

- remove the isolated worktree when no longer needed;
- keep `tasks/delete-algebraic-crate-done.md` as the branch record unless Jorn
  later wants task-history pruning.
