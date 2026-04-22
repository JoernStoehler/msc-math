---
name: rust-conventions
description: Rust conventions for `crates/**/*.rs` and `experiments/**/*.rs`, including coordinate order, mathematical invariants, formal label references, algorithm boundaries, tests, error handling, and performance claims. Use before editing or reviewing Rust code.
---

# Rust Conventions

## Standard Rust

Prefer ordinary Rust for scientific computing: correct code, explicit control
flow, plain structs/enums, and local readability.

- Start with standard Rust practices. Add repo-specific machinery only when it
  clearly pays for itself.
- Prefer KISS and YAGNI over elegant-looking abstraction.
- Prefer moderate duplication over indirection and over abstraction whenever
  copy-editing and inline context make the code easier to read.
- Do not prematurely factor out intermediate computations just to "provide"
  them to later glue code. Let callers compute what they need locally unless the
  intermediate result is independently useful.
- Prefer plain data structs first. Add checked constructors, hidden fields, or
  type-level encodings only when they prevent a real misuse pattern or make the
  API easier to use correctly.
- For plain data structs, prefer public fields over trivial accessor methods.
- When an invariant is not enforced by the type, document it briefly on the type
  or at the construction site.
- Public signatures should make data flow, mathematical domain, and failure
  modes easier to read, not harder.
- Prefer functions that say "I need X, Y, Z" over functions that force callers
  through one preselected intermediate object or pipeline shape.
- Use the clearest standard error surface for the caller: `Option`, `Result`,
  enums, `assert!`, and `panic!` all have their place.

## Overriding Custom Conventions

These rules are repo-specific and override the generic default above.

### Math-facing code vs orchestration

Distinguish mathematical code from glue code.

- Mathematical code implements a definition, lemma-shaped computation, exact
  predicate, optimizer step, geometric transformation, or other result-bearing
  operation.
- Orchestration code wires inputs and outputs together, iterates over jobs,
  formats artifacts, dispatches algorithms, or manages pipeline control flow.
- Put the proof burden, formal labels, and math-specific case distinctions on
  the mathematical code.
- Keep orchestration code simple and explicit. Do not force formal structure or
  abstraction patterns onto glue code just because nearby math code is formal.

### Math-facing code

When code correctness depends on a formal statement, cite the label in Rust with
`[lem:label]`, `[thm:label]`, `[def:label]`, and similar bracketed forms.

- Do not invent labels. If the formal statement is missing, leave a TODO naming
  the missing label.
- Do not cite rendered theorem numbers like "Lemma 3.2".
- Do not duplicate proofs in Rust comments.
- Do not force labels onto pure orchestration, obvious plumbing, or experiment
  wrappers.

Read the matching formal file before editing mathematical algorithm code:
- `crates/symplectic/src/geom/**` -> `formal/library/geom.tex`
- `crates/symplectic/src/kkt/**` -> `formal/library/kkt.tex`
- `crates/symplectic/src/algorithms/**` -> `formal/library/algorithms.tex`
- `experiments/<topic>/**` -> `formal/<topic>/*.tex` when such a file exists

Load `$formal-math-conventions` when editing formal labels or changing a
mathematical algorithm.

### File and helper boundaries

Optimize for local readability.

- Prefer one main concern per `.rs` file.
- Keep tightly coupled logic inline when extracting it would force readers to
  jump away and reconstruct the same local context.
- Extract a helper when it names a real stage, is reused, or cleanly separates a
  boundary that future edits can touch independently.
- Prefer a context-specific helper over a generic "future-proof" abstraction
  when there is only one caller.
- Do not split `A -> C` into `A -> B` plus `B -> C` unless `B` has a real life
  of its own, such as multiple callers, a distinct semantic stage, or an
  independently checkable intermediate result.

Bad smell: multiple single-use helpers with similar shapes that make one simple
function readable only by opening several files.

### Tests

Crate tests are for fast feedback during ordinary development.

- Prefer small deterministic examples, named known polytopes, exact invariants,
  and narrow regression cases.
- Move long smoke/unit test bodies into `test_*.rs` files when inline tests start
  to dominate an implementation file.
- Keep expensive random sweeps, broad validation datasets, and mathematical
  evidence generation in `experiments/`, not in default crate tests.

### Measured claims and constants

- Empirical constants need a short comment saying where they came from, what
  they were tuned against, and what to re-check if changed.
- Performance claims need a benchmark source, date, and input range.

### Experiments vs crates

Stable reusable code lives in `crates/`. Experiment-specific behavior stays in
`experiments/`.

- In an experiment package, shared helpers belong in `src/lib.rs` when multiple
  binaries use them. One-off binary-local helpers stay in that `main.rs`.
- Use semantic experiment paths; do not force balanced trees.
- Keep research interpretation and decision history in `research/`.
- Rust binaries under `experiments/**` carry machine-readable crate docs with
  `Input Artifacts:` and `Output Artifacts:`.

## Domain Knowledge

### Coordinate convention

Use `(q1, q2, p1, p2)`. Components `[0,1]` are q-space, `[2,3]` are p-space,
and `[0,2]` / `[1,3]` are the symplectic planes. The common wrong assumption is
`(q1, p1, q2, p2)`. The defining code lives in
`crates/symplectic/src/geom/symplectic_form.rs`.

### Capacity algorithms

The core capacity algorithms are `hk2017`, `billiard`, and `tube`. When their
domains overlap, they should agree on the computed capacity.

Do not add rayon inside core algorithms unless you have measured reason to break
the repo default that parallelism lives at the dataset/job level.
