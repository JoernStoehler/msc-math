# Session: Rigorous error analysis for f64 pre-filter in vertex enumeration

**Goal:** Produce a .tex writeup (lemma/proof style) of the correctness argument for the f64 pre-filter in `crates/src/geom/vertex_enumeration.rs`.

**Worktree:** Yes. Branch from local `main` at the test-data-pipeline worktree, or from `main` directly.

## Context

`enumerate_vertices_exact` tests all C(F,4) subsets of dual vertices to find polytope vertices. For each subset, it solves a 4×4 system A·v = 1 exactly over Q via Cramer's rule. This is expensive (BigRational arithmetic with potentially huge denominators).

A new f64 pre-filter solves the same system in f64 first. For each non-defining constraint y_i · v ≤ 1, it applies three-valued logic:
- FALSE (y_i · v_f64 ≥ 1 + margin): definitely outside → skip subset (no rational work)
- INDETERMINATE: fall through to exact rational
- TRUE: continue checking other constraints

The pre-filter can only reject, never confirm. If it incorrectly rejects a subset that contains an actual vertex (false FALSE), that's a **correctness bug** — the vertex is silently lost.

## The problem

The margin must exceed the total error |y_i · v_f64 - y_i · v_exact| where v_exact is the exact rational Cramer solution. Three error sources:

1. **Cramer solve rounding:** f64 arithmetic in det4/solve4 produces v_f64 ≠ A_f64⁻¹ · 1.
2. **Matrix rounding:** The f64 matrix A_f64 may differ from the rational matrix A_rational. This happens when rational dual vertices were constructed directly (e.g., 1/3 rounds to 0.333...15 in f64). When rationals came from `f64_to_rational` (lossless IEEE-754 conversion), A_f64 = A_rational exactly.
3. **Dot product rounding:** Computing y_i_f64 · v_f64 in f64 has its own rounding error.

The current implementation uses a residual-based margin but doesn't rigorously account for source 2.

## Deliverable

A .tex file at `crates/src/geom/math_prefilter.tex` containing:

1. Problem statement (what the pre-filter does, what "correctness" means)
2. Error decomposition (all three sources, with notation)
3. The proposed algorithm with the margin formula
4. Correctness proof: the margin exceeds the total error under stated assumptions
5. Clear statement of assumptions (when the pre-filter is safe, when it must be disabled)

The proof should cover BOTH:
- The common case: rationals from `f64_to_rational` (power-of-2 denominators, source 2 vanishes)
- The general case: rationals constructed directly (arbitrary denominators, source 2 nonzero)

## Skills to load

- `math-tex` — conventions for math.tex files
- `rust-conventions` — for reading the Rust code

## Key files to read

- `crates/src/geom/vertex_enumeration.rs` — `f64_prefilter_rejects`, `enumerate_vertices_exact`, `det4_f64`, `solve4_f64`, and the doc comments explaining the two-stage pipeline
- `crates/src/geom/rational_arithmetic.rs` — `f64_to_rational` (lossless) and `rational_to_f64` (may round)
- `crates/src/geom/polytope.rs` — `Polytope4D::new()` (f64 input), `from_rationals()` (rational input)

## Invariant

The .tex file should compile standalone (`pdflatex math_prefilter.tex`). The code in `vertex_enumeration.rs` may need updating based on the analysis — note what changes are needed but do not make them in this session.
