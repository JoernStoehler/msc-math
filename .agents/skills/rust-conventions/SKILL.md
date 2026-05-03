---
name: rust-conventions
description: Rust conventions for `crates/**/*.rs` and `experiments/**/*.rs`, including coordinate order, mathematical invariants, formal label references, algorithm boundaries, tests, error handling, and performance claims. Use before editing or reviewing Rust code.
---

# Rust Conventions

## Default Style

Default to standard Rust.

- Use plain structs/enums and explicit control flow.
- Prefer moderate duplication over indirection.
- Prefer moderate duplication over generic abstraction.
- Keep one-off logic local.
- Keep one-off experiment helpers in the same file unless another binary uses them.
- Prefer public fields on plain data structs.
- Do not add trivial getters/setters.
- Document unenforced invariants briefly on the type or at the construction site.
- Pass the direct inputs the callee reads.
- Do not replace direct arguments with a context bag, stage object, or builder when the callee only reads a few fields.
- Use `Option` for present/absent cases.
- Use `Result` when the caller can recover or needs an error message.
- Use `assert!` for internal invariants that must hold if the surrounding code is correct.
- Use `panic!` only for impossible states or one-shot experiment binaries where aborting is acceptable.

## Do Not Add

- Do not add new trait layers, generic frameworks, builder layers, or type-level encodings in routine cleanup.
- Do not factor out intermediate results just to “provide” them to later glue code.
- Do not insert an intermediate `B` when the only caller immediately forwards it to `C`.
- Do not split a file just because it is long.
- Phase comments such as `Phase 1`, `Phase 2`, or `Stage A` are not file boundaries by themselves.
- Do not split a file just because of short repeated local patterns.
- Do not over-formalize glue code because nearby math code is formal.

## File Boundaries

Treat file/module boundaries as abstractions too.

- Keep one pipeline in one file when the sections share the same constants, local data structs, and control flow.
- If changing constants, output shape, or stage order still requires reading the whole pipeline, keep section headers in one file instead of splitting.
- Split only when at least one of these is already true in the current patch:
  - the extracted code has 2+ callers
  - the extracted code owns a tracked format, schema, or checkpoint contract
  - the extracted code has its own dedicated test or verification surface
  - the extracted code is shared by another binary now
- In `experiments/**`, start with one `main.rs` pipeline and split only if one of the explicit exceptions above is already true.
- Do not split an experiment by phase when changing constants, output schema, or stage order would still require edits on both sides.

Bad smell:
- single-use helpers or files that force readers to hop around just to follow one simple pipeline

## Math vs Glue

Distinguish mathematical code from orchestration.

- Mathematical code implements a definition, exact predicate, optimizer step, geometric transformation, or other result-bearing operation.
- Orchestration code wires inputs and outputs, iterates jobs, formats artifacts, dispatches algorithms, or manages control flow.
- Put proof burden, formal labels, and math-specific case distinctions on mathematical code.
- Keep orchestration code as straight-line locals, loops, and `if`/`match`, not helper stacks or framework layers.

## Formal Labels

When correctness depends on a formal statement, cite the label in Rust with `[lem:label]`, `[thm:label]`, `[def:label]`, or similar.

- Do not invent labels.
- If the formal statement is missing, leave a TODO naming the missing label.
- Do not cite rendered theorem numbers like “Lemma 3.2”.
- Do not duplicate proofs in Rust comments.
- Do not force labels onto orchestration, obvious plumbing, or experiment wrappers.

Read the matching formal file before editing mathematical algorithm code:
- `crates/symplectic/src/geom/**` -> start with `formal/symplectic-polytope-geometry.tex`
- `crates/symplectic/src/kkt/**` -> start with `formal/ehz-kkt-system.tex`
- `crates/symplectic/src/algorithms/**` -> start with `formal/capacity-algorithms.tex`
- `experiments/<topic>/**` -> grep `formal/*.tex` for cited labels or named
  proof objects; formal files are named by the mathematics, not by experiment
  directory.

Load `$formal-math-conventions` when editing formal labels or changing a mathematical algorithm.

## Tests

Use crate tests for fast feedback.

- Prefer small deterministic examples, named known polytopes, exact invariants, and narrow regressions.
- Move tests out when the test code in a file is larger than the implementation code in that file.
- Keep broad validation sweeps, evidence generation, and expensive random search in `experiments/`.

## Measured Claims

- Give empirical constants a short note: where they came from, what they were tuned against, and what to re-check if changed.
- Back performance claims with benchmark source, date, and input range.

## Crates vs Experiments

- Keep stable reusable code in `crates/`.
- Keep experiment-specific behavior in `experiments/`.
- Put shared experiment helpers in `src/lib.rs` only when multiple binaries use them.
- Keep one-off binary-local helpers in that `main.rs`.
- Use semantic experiment paths.
- Keep research interpretation and decision history in `research/`.
- Put `Input Artifacts:` and `Output Artifacts:` in crate docs for Rust binaries under `experiments/**`.

## Domain Knowledge

### Coordinate Convention

Use `(q1, q2, p1, p2)`.

- `[0,1]` are q-space.
- `[2,3]` are p-space.
- `[0,2]` and `[1,3]` are the symplectic planes.
- The common wrong assumption is `(q1, p1, q2, p2)`.
- The defining code is `crates/symplectic/src/geom/symplectic_form.rs`.

### Capacity Algorithms

The core capacity algorithms are `hk2017`, `billiard`, and `tube`.

- When their domains overlap, they should agree.
- Do not add rayon inside core algorithms unless measured evidence justifies breaking the repo default that parallelism lives at the dataset/job level.
