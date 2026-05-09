<!--
Purpose: branch-level definition of done for `delete-algebraic-crate`.
Context: this branch replaces the old exact-arithmetic API/design attractor
with a generic exact arithmetic and exact linear algebra crate for thesis
closeout.
-->

# Delete Algebraic Crate Done Definition

## Status

- State: done pending merge review.
- Last updated: 2026-05-09.
- Source surfaces: `tasks/verify-thesis-done.md`, `tasks/numerics.md`,
  `tasks/reproducibility.md`, `tasks/rust-tech-debt.md`,
  `research/numerics.md`, current Rust consumers, and the new crate docs.
- Refresh when: exact arithmetic ownership, exact linear algebra ownership,
  retained thesis claims, repo promises, workspace consumers, or branch merge
  checks change.

## Goal

Replace the old exact-arithmetic API/design attractor with one small crate for
generic exact arithmetic and generic exact linear algebra.

The branch is done only when thesis work and future main-branch sessions no
longer need to re-decide exact-arithmetic ownership, public API, or merge
checks.

"Small" means that public API is justified by current thesis-closeout use or a
written near-term thesis-closeout reason. Checked examples/tests protect those
uses. They do not independently justify public API. Small does not mean line
count.

"Generic" means independent of symplectic geometry, KKT semantics, capacity
logic, orbit logic, polytope geometry, and experiment workflows.

"Site" means a crate, module, experiment packet, helper, or public API surface
that owns or consumes exact arithmetic or exact linear algebra.

"Delete" means removed before merge. A live importable path is not done merely
because it is labeled `delete`.

## Source Order

Use sources in this order:

1. Thesis closeout gates in `tasks/verify-thesis-done.md`.
2. Topic obligations in `tasks/numerics.md`, `tasks/reproducibility.md`, and
   `tasks/rust-tech-debt.md`.
3. Research state in `research/numerics.md` and retained thesis wording.
4. Current Rust consumers and workspace integration.
5. Crate-local `README.md` and `DEVELOPMENT.md`.

Crate-local docs can define the new crate contract. They do not define branch
done by themselves.

## Scope Boundary

The new crate owns generic exact arithmetic and generic exact linear algebra:

- exact scalar arithmetic;
- exact ordering;
- row reduction;
- rank;
- kernel basis;
- linear solve;
- generic definiteness or inertia, but only when justified by the Small Public
  API gate.

The new crate does not own domain logic:

- KKT semantics;
- capacity or orbit logic;
- polytope geometry;
- symplectic-specific certificates;
- experiment workflows.

Domain-specific code stays in the domain crate or experiment. The new crate
must not become a dumping ground for code that merely uses exact numbers.

## Thesis-Relevance Test

Classify every exact arithmetic and exact linear algebra site by quick checks.

A site is thesis-relevant if any of these checks says yes:

- It supports retained thesis claims, final verification, repo promises, or
  main task maps.
- It is a dependency of `crates/symplectic` or another durable crate used by
  thesis work.
- It is imported by an experiment packet that tasks or research marks active,
  mainline, or contingent during writing.
- Ambiguity about it has caused agent derailment or could block future agents
  from starting thesis work on main.

If none of those checks says yes, classify the site with a short reason:

- experiment-only;
- historical;
- future/follow-up;
- delete.

The classification must be recorded in the branch inventory below before
merge.

If a collection of sites is not clearly classified by task, research, or
thesis-facing sources, ask Jörn to classify the collection. Ask about the
collection's project role, not individual implementation details, unless the
collection-level answer is insufficient for a consumer-migration decision.

## Branch Inventory

The branch is not merge-ready until this inventory is filled.

Each row must name:

- path or path glob;
- owned or consumed operation;
- classification;
- thesis-relevance source or non-thesis-relevance reason;
- migration, deletion, or exception status;
- verification command or review action.

| path or glob | operation | classification | source/reason | status | verification |
| --- | --- | --- | --- | --- | --- |
| `crates/algebraic-numbers/` | generic exact scalar arithmetic, exact ordering, canonical coefficients, row reduction, rank, kernel basis, linear solve, negative-definite check | thesis-relevant durable crate | `tasks/rust-tech-debt.md` exact arithmetic replacement branch; `crates/symplectic` and active experiments consume it | new crate API replaces old `OrderedField`/`TanPiFifth`/`canonical_element`/`solve_square` attractor | `cargo test -p algebraic-numbers`; `cargo clippy -p algebraic-numbers --all-targets -- -D warnings` |
| `crates/symplectic/src/exact/` | exact polytope, one-sigma orbit, and derivative validation over exact scalars | thesis-relevant durable consumer | durable exact-validation support used by theorem-facing paths and branch gate | migrated to `ExactScalar`, `solve_linear_system`, `rank`; domain geometry/KKT logic stays in `symplectic` | `cargo check -p symplectic`; `cargo test -p symplectic --lib exact::` |
| `crates/symplectic/src/kkt/rational_solver.rs` | rational exact KKT solve and positive-beta feasibility over Q | thesis-relevant durable domain code | `crates/symplectic` owns KKT semantics and reusable symplectic geometry routines | generic linear solve uses `algebraic-numbers`; KKT matrix construction, Fourier-Motzkin feasibility, and rational domain workflow remain local | `cargo test -p symplectic --lib kkt::rational_solver`; `cargo check --workspace`; code review against scope boundary |
| `experiments/numerics/src/algebraic/` and `experiments/numerics/{algebraic-exactness,sage-feasibility}/` | active numerics exactness spike and Sage-feasibility input generation | thesis-relevant active experiment | `tasks/numerics.md`, `research/numerics.md`, and `experiments/numerics/README.md` keep these as active/contingent numerics evidence | arithmetic uses new crate; experiment-local catalog, field tags, KKT, and geometry remain local domain/workflow code | `cargo check -p dev-numerical-analysis` |
| `experiments/hko-local-maximum/src/exact_bank.rs`, `gradient-analysis/`, `sage-validation/` | theorem-facing exact row bank, exact diagnostics, and Sage-validation input generation | thesis-relevant active experiment | `experiments/hko-local-maximum/README.md` routes theorem-facing exact work and Sage validation through these paths | migrated from old crate public API to local field marker/helpers over the new crate and `symplectic::exact` | `cargo check -p exp-hko-local-maximum` |
| `experiments/numerics/src/algebraic/geom.rs`, `experiments/numerics/src/algebraic/kkt.rs` | experiment-owned exact geometry and KKT workflow | thesis-relevant domain code | branch scope says KKT semantics, polytope geometry, capacity/orbit logic, and experiment workflows stay outside the generic crate | generic rank and linear solve use `algebraic-numbers`; HKO/KKT/polytope workflow logic remains local | `cargo check -p dev-numerical-analysis`; `cargo test -p dev-numerical-analysis`; code review against scope boundary |
| `experiments/numerics/error-bounds/exact_solver.rs` | exact QP solve, exact multiplier recovery, and beta feasibility for numerics error bounds | thesis-relevant active experiment | numerics error-bound workflow is active thesis evidence, but not generic crate API | generic linear solves use `algebraic-numbers`; QP assembly, multiplier interpretation, and error-bound workflow remain local | `cargo test -p dev-numerical-analysis`; `cargo check --workspace`; code review against scope boundary |
| `experiments/hko-local-maximum/exact-clarke/*.py` | SymPy exact derivation scripts | thesis-relevant domain scripts, no Rust crate dependency | exact-Clarke route is theorem-facing per `experiments/hko-local-maximum/README.md` | not part of Rust exact-scalar API migration; no old `algebraic-numbers` imports | `rg` review for old Rust API imports |
| `experiments/verification/all-minimum/main.rs` | verification experiment error mapping | integration fix | workspace check exposed a missing `OrbitSearchError::InvalidGap` match arm | added the missing mapping; unrelated to exact arithmetic but needed for blocker-free workspace integration | `cargo check --workspace` |
| `crates/symplectic/API_SURFACE_TARGET.md`, `crates/symplectic/API_REFACTOR_GOAL.md` | historical/proposed API sketches mentioning `OrderedField` | future/follow-up documentation | these are API target/proposal notes, not compiled source or current API authority | left as historical terminology; current source and `crates/MAP.md` override for active navigation | `rg` review; no compiled imports |

Rows classified `delete` must be removed before merge. Rows classified
future/follow-up must be non-thesis-relevant and must not block future agents
from starting thesis work on main.

## Done Gates

The branch is done only when all gates below pass.

### 1. Thesis Role

The repo states what generic exact arithmetic and generic exact linear algebra
are needed for thesis closeout.

All exact arithmetic and exact linear algebra sites are classified by the
thesis-relevance test.

Remaining exact-arithmetic work is classified as thesis-blocking, accepted
caveat, future/follow-up, or delete.

### 2. Consumer Adoption

Every thesis-relevant exact arithmetic or exact linear algebra consumer uses
the new crate before merge.

For an operation the new crate claims to provide, "uses the new crate" means
the consumer no longer owns competing generic scalar arithmetic, ordering, row
reduction, rank, kernel, solve, definiteness, or inertia logic.

`crates/symplectic` must not retain reusable exact linear algebra if the new
crate provides the same operation. `crates/symplectic` keeps symplectic
geometry, KKT semantics, capacity/orbit logic, and symplectic certificates.

Exceptions are allowed only for non-thesis-relevant collections classified as
experiment-only, historical, future/follow-up, or delete.

### 3. Old Attractor Removed

Old public APIs and old design patterns are not kept for continuity.

A removed old surface may return only if all of these are true:

- a migrated thesis-relevant consumer needs it;
- the need cannot be met by a simpler new-crate API;
- the reason is written next to the API contract.

The branch inventory records each retired old surface or compatibility surface.

### 4. Duplicate Ownership Closed

No thesis-relevant path has a second implementation of generic exact arithmetic
or generic exact linear algebra.

Duplicates are migrated, deleted, or classified as non-thesis-relevant with a
written reason.

### 5. Small Public API

Every public item in the new crate is justified by one of these:

- a migrated thesis-relevant consumer;
- a written near-term thesis-closeout reason.

Checked examples and tests protect those uses. They do not independently
justify public API.

Otherwise the item is private or removed.

### 6. Docs And Process

`README.md` explains the ordinary consumer path: how to define a field or use a
rational scalar, how to build exact matrices/vectors, and which exact linear
algebra operations are supported.

`DEVELOPMENT.md` explains maintainer boundaries, rejected old designs, and
exact checks.

Process artifacts count only when they prevent a known failure mode:

- old-attractor relapse;
- unclear ownership;
- unsupported public API growth;
- weak review;
- future-agent derailment.

Otherwise prompts, review instructions, and skill edits are not branch-done
evidence.

### 7. Verification Contract

The merge checklist lives in this file. The branch is not merge-ready until the
checklist is filled.

Each check has:

- command or review action;
- scope;
- expected pass condition;
- reason it covers the relevant branch risk.

The checklist covers:

- new crate behavior;
- all migrated thesis-relevant consumers;
- workspace integration affected by migration;
- absence or classification of old exact-code paths.

The checklist must not defer success to an open-ended repo-wide audit, to
checking unrelated thesis artifacts, or to a broad multi-week verification
project.

| check | command or review action | scope | pass condition | covered risk |
| --- | --- | --- | --- | --- |
| crate-local tests | `cargo test -p algebraic-numbers` | new generic crate | all tests and doctests pass | scalar arithmetic/order and exact linear algebra regressions |
| crate-local lint | `cargo clippy -p algebraic-numbers --all-targets -- -D warnings` | new generic crate | no clippy warnings | unsupported public API growth or obvious Rust quality regressions |
| durable consumer check | `cargo check -p symplectic` | durable symplectic crate | package checks | `symplectic` no longer depends on removed old scalar API |
| exact durable tests | `cargo test -p symplectic --lib exact::` | `symplectic::exact` | exact module tests pass | migrated exact polytope/orbit/derivative behavior |
| rational KKT tests | `cargo test -p symplectic --lib kkt::rational_solver` | `symplectic::kkt::rational_solver` | rational KKT tests pass | durable KKT domain solver uses shared generic linear solve without behavior regression caught by tests |
| active numerics check | `cargo check -p dev-numerical-analysis` | numerics experiment package | package checks | active numerics consumers compile without old crate API |
| active numerics tests | `cargo test -p dev-numerical-analysis` | numerics experiment package | tests pass | active exact geometry, KKT, and error-bound workflows compile and pass after migration |
| HKO package check | `cargo check -p exp-hko-local-maximum` | HKO theorem/evidence package | package checks | theorem-facing exact-bank consumers compile without old crate API |
| old API import review | `rg -n "use algebraic_numbers::.*(OrderedField|TanPiFifth|canonical_element|CanonicalElement)|algebraic_numbers::(OrderedField|TanPiFifth|canonical_element|CanonicalElement|cmp_field|max_field|min_field|solve_square|rank_rows)" crates experiments -g '!target'` | Rust crates and experiments | no live Rust imports of removed public API remain | old-attractor relapse |
| duplicate generic exact-linear-algebra review | `rg -n "fn rank_rows|fn gauss_solve_with_null_space|fn back_substitute" crates experiments -g '!target'` plus diff review of hidden exact Gaussian helpers | Rust crates and experiments | no local generic rank/Gaussian helpers remain in live Rust consumers | duplicate ownership between consumers and `algebraic-numbers` |
| workspace integration | `cargo check --workspace` | all Rust workspace packages | workspace checks | cross-package compile drift after migration |
| navigation/doc review | compare `crates/MAP.md`, `crates/algebraic-numbers/README.md`, `crates/algebraic-numbers/DEVELOPMENT.md`, `tasks/rust-tech-debt.md` | maps and docs | public API names and branch role agree with source | future-agent derailment from stale docs |

### 8. Workspace Agreement

Cargo manifests, maps, task notes, crate docs, experiment notes, and current
callers agree on:

- the new generic exact arithmetic home;
- which old paths are gone;
- which exceptions are non-thesis-relevant;
- what remains future/follow-up or delete.

## Not Done

The branch is not done if any of these remain true:

- only crate-local tests pass;
- only a clean new crate exists;
- thesis-relevant consumers still use old or duplicate generic exact code;
- any live importable path is classified `delete`;
- `crates/symplectic` retains reusable exact linear algebra that the new crate
  provides;
- two live paths appear to own the same generic exact arithmetic or generic
  exact linear algebra operation;
- exact arithmetic work remains unclassified by the thesis-relevance test;
- verification burden increases or is deferred to an open-ended audit.
