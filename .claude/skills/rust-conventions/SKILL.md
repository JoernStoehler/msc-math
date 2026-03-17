---
name: rust-conventions
description: Coding conventions and mathematical documentation standards for Rust code in crates/ and experiments/. Load when writing, editing, or reviewing .rs files. Covers module structure, coding style, coordinate conventions, math-code correspondence, doc comments, math.tex cross-references, magic numbers, and performance claims.
---

# Rust Conventions

## Module Structure

Single crate `symplectic` with modules:
- `geom::*` — polytope types, geometry primitives (polytope, skeleton, symplectic_form, volume, polygon, lagrangian_product, cross_product_4d, validation, rational_arithmetic, vertex_enumeration, qhull, reeb_trajectory, known_polytopes, test_utils)
- `algorithms::hk2017` — general capacity (exponential), with submodules: permutations, orbit_recovery, generate_capacity_fixtures
- `algorithms::billiard` — Lagrangian product capacity (fast), with submodules: block_enumeration, facet_classification, kkt_benchmark
- `algorithms::tube` — tube algorithm (placeholder)
- `algorithms::capacity_accumulator` — certified/uncertain candidate tracking (shared by hk2017 and billiard)
- `algorithms::facet_adjacency` — undirected + directed facet adjacency matrices (shared by hk2017 and billiard)
- `kkt` — shared KKT solver infrastructure: QP struct, Solution, Verdict, classify_margin
  - `kkt::qp_assembly` — polytope + permutation -> QP matrices (C, d, H) or augmented system
  - `kkt::saddle_point_solver` — (m+5)x(m+5) eigendecomposition solver
  - `kkt::constraint_solver` — Cx=d particular solution + null space via SVD
  - `kkt::beta_feasibility` — max-margin LP for beta>0 in affine solution set
  - `kkt::projection_solver` — null space projection, reduced objective optimization
  - `kkt::rational_solver` — exact rational KKT solver
- `constants` — shared tolerance constants
- `random` — random polytope generation
- `dataset` — dataset serialization

**When modifying shared modules** (kkt, constants, algorithms::capacity_accumulator, algorithms::facet_adjacency): Check all callers. Use `cargo test --lib` to verify.

## Three Capacity Algorithms

| Module            | Applies to                        | Cost                    |
|-------------------|-----------------------------------|-------------------------|
| algorithms::hk2017| All polytopes                     | Exponential in #facets  |
| algorithms::billiard| Lagrangian products only         | Fast                    |
| algorithms::tube  | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

## Coding Style

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Test modules are declared in the parent `mod.rs` (not in the source file itself) via `#[cfg(test)] #[path = "foo_test.rs"] mod foo_test;`.
- A source file may have multiple test files (e.g. `volume_test.rs`, `volume_properties_test.rs`), each covering a single concern.
- Prefer iterator chains over `for` loops. Minimize mutable state. Use `map`, `filter`, `flat_map`.
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing
- **Coordinate convention**: (q₁, q₂, p₁, p₂) — components [0,1] = q-space (Lagrangian), [2,3] = p-space (Lagrangian), [0,2] = (q₁, p₁) symplectic plane, [1,3] = (q₂, p₂) symplectic plane. Defined in `geom/symplectic_form.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂) ordering.
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

## Mathematical Documentation

Lemma statements and proofs live in the module's `math.tex` file. Rust doc comments never contain proofs — they contain code-math correspondence only. Load the `math-tex` skill for full math.tex conventions.

**Agent rule:** Read the module's `math.tex` before editing `.rs` files in that module.

**Cross-references in .rs doc comments:**

Format: `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` — matching the `\label{}` in the module's `math.tex` exactly.

Rules:
1. Always include a one-line English description of what the referenced result says
2. Never duplicate proofs — doc comment says *what the code does*, math.tex says *why it's correct*
3. Never invent labels that don't exist in a math.tex file — if the lemma isn't written yet, add a `// TODO: add [lem:...] to math.tex` comment
4. Never use rendered numbers like "Lemma 3.2" — use the label
5. Verification: grep for the label in `math.tex` files, confirm it exists

## Magic Numbers

Empirically chosen constants must have their rationale documented in code comments on the constant definition itself. Include: why that value, what data point motivated it, known limitations, and what must be re-validated if changed.

## Performance Claims

Never state performance without benchmark. "~1ms" is a claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured.

## The Core Rule (for Rust doc comments)

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code. "The data shows Y" requires reading the data. When verification is impossible, mark with a TODO comment.
