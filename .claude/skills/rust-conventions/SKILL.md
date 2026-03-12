---
name: rust-conventions
description: Coding conventions and mathematical documentation standards for Rust code in crates/ and experiments/. Load when writing, editing, or reviewing .rs files. Covers module structure, coding style, coordinate conventions, math-code correspondence, doc comments, thesis cross-references, magic numbers, and performance claims.
---

# Rust Conventions

## Module Structure

Single crate `symplectic` with modules:
- `geom::*` — polytope types, geometry primitives
- `algorithms::hk2017` — general capacity (exponential)
- `algorithms::billiard` — Lagrangian product capacity (fast)
- `algorithms::tube` — tube algorithm (placeholder)
- `kkt` — shared KKT solver (used by hk2017 and billiard)
- `constants` — shared tolerance constants
- `random` — random polytope generation
- `dataset` — dataset serialization

**When modifying shared modules** (kkt, constants): Check all callers. Use `cargo test --lib` to verify.

## Three Capacity Algorithms

| Module            | Applies to                        | Cost                    |
|-------------------|-----------------------------------|-------------------------|
| algorithms::hk2017| All polytopes                     | Exponential in #facets  |
| algorithms::billiard| Lagrangian products only         | Fast                    |
| algorithms::tube  | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

## Coding Style

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`)
- Prefer iterator chains over `for` loops. Minimize mutable state. Use `map`, `filter`, `flat_map`.
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing
- **Coordinate convention**: (q₁, q₂, p₁, p₂) — components [0,1] = q-space (Lagrangian), [2,3] = p-space (Lagrangian), [0,2] = (q₁, p₁) symplectic plane, [1,3] = (q₂, p₂) symplectic plane. Defined in `geom/symplectic.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂) ordering.
- **No rayon inside algorithms**: Parallelism is at the dataset level, not inside capacity algorithms.

## Thesis Constraints

Polytope4D: 5-16 facets typical. Research code, March 2026 deadline. Correctness > performance.

Don't suggest: Theoretical numerical analysis, O(n²) documentation when n ≤ 16, production features unlikely to matter.
Do suggest: Critical path tests, benchmarks for claims, robustness fixes.

## Invariant

`cargo test` passes from `crates/` with zero failures.

## Math-Code Correspondence

Rust types, function signatures, and function bodies 1:1 correspond to mathematical definitions. "1:1" means literal structural correspondence, not just "inspired by."

**Verification criteria:**
- Doc comment formulas must match code's actual computation (not aspirational, not approximate)
- Invariants stated in doc comments must be enforced by types/constructors/assert!/debug_assert!
- Properties stated in doc comments must have corresponding tests

## Documentation Quality

- Definitions, lemmas, and proofs live as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/
- Quality bar: specific, correct, detailed, clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing

## Cross-References to Thesis

Format: `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` — matching LaTeX `\label{}` name exactly.

Rules:
1. Always include a one-line English description of what the referenced result says
2. Never duplicate proofs inline — comment says *what*, thesis says *why*
3. Never use rendered numbers like "Lemma 3.2" — use the label
4. Verification: grep `crates/src/` for occurrences, find the `.tex` `\label{...}`, check match

## Magic Numbers

Empirically chosen constants must have their rationale documented in code comments on the constant definition itself. Include: why that value, what data point motivated it, known limitations, and what must be re-validated if changed.

## Performance Claims

Never state performance without benchmark. "~1ms" is a claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured.

## The Core Rule (for Rust doc comments)

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code. "The data shows Y" requires reading the data. When verification is impossible, mark with a TODO comment.
