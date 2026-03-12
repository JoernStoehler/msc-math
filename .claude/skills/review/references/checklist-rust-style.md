# Review Checklist: Rust Code Style (Phase 1)

Detection rules for Rust coding conventions in `crates/` and `experiments/`.

## 1. Colocated Test File Structure

- `foo.rs` has `foo_test.rs` in the same directory.
- Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`).

## 2. Iterator Style

- Prefer iterator chains over `for` loops. Minimize mutable state.
- Detection: flag `for` loops that could be rewritten as iterator chains (moderate confidence only — some loops are clearer as `for`).

## 3. Type Invariants

- Types should encode mathematical invariants, validated at construction.
- Detection: look for `pub` fields on types that have constructor functions — fields should typically be private with validation in `new()`.

## 4. Coordinate Convention

(q1, q2, p1, p2) — components [0,1] = q-space, [2,3] = p-space, [0,2] = (q1,p1) symplectic plane, [1,3] = (q2,p2) symplectic plane.
- Detection: flag comments or code that assume (q1, p1, q2, p2) ordering.

## 5. Cross-References to Thesis

Format: `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` in doc comments.
- Must include one-line English description.
- Must never use rendered numbers like "Lemma 3.2".
- Must never duplicate proofs inline.
- Detection: grep for `Lemma \d`, `Theorem \d`, `Definition \d` in doc comments — these should use labels.

## 6. Magic Numbers

- Empirically chosen constants must have rationale documented on the constant definition.
- Detection: grep for numeric literals in non-test code that aren't 0, 1, 2, or obvious array indices. Check if they have a comment explaining why that value.

## 7. Performance Claims

- Never state performance without benchmark.
- Detection: grep doc comments for time claims ("~1ms", "fast", "O(n^2)") — each needs a benchmark reference.

## 8. No Rayon Inside Algorithms

- Parallelism is at the dataset level, not inside capacity algorithms.
- Detection: grep for `use rayon` or `.par_iter()` inside `algorithms/` or `kkt/`.

## 9. Shared Module Changes

- If `kkt`, `constants`, or other shared modules are modified: verify all callers are checked.
- Detection: if diff touches shared modules, list all files that import from them.

## 10. Experiment Binary Conventions

For `experiments/*.rs` only:
- Library stability boundary: only stable code in `crates/`. New variants self-contained in experiment binaries.
- Copy library internals if needed, don't modify the library for experiment-specific behavior.
