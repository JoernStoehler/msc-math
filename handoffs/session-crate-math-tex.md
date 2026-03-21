# Session: Create Crate math.tex Files

**Goal:** Create `math.tex` files for each crate module directory, populating them with the lemma/definition stubs referenced by .rs doc comments.

**Worktree:** Yes. Branch from local `main`.

## Context

The new convention (decided 2026-03-17): lemma statements and proofs live in colocated `math.tex` files, not in .rs doc comments or thesis/. Currently, .rs files reference 46 labels (e.g. `[lem:kkt]`, `[def:volume]`) via doc comments, but no math.tex files exist yet. These are phantom labels.

This session creates the math.tex files and populates them with stub environments for each referenced label. The stubs contain the label, a brief statement extracted from the .rs doc comment's one-line description, and a `% [TODO: JÖRN - verify/complete]` marker. Proofs are NOT written — just the environment scaffolding.

## Files to create

### `crates/src/geom/math.tex` — 18 labels

`def:cross-product-4d`, `def:ehz-capacity`, `def:face-lattice`, `def:J0`, `def:lagrangian-product`, `def:polar-body`, `def:polygon-area`, `def:polygon-h-rep`, `def:polytope-dual`, `def:reeb-vector-field`, `def:symplectic-form`, `def:symplectic-product`, `def:volume`, `lem:piecewise-linear-reeb`, `lem:positive-span`, `lem:rational-pipeline`, `lem:vertex-enumeration`, `thm:hko-counterexample`

### `crates/src/kkt/math.tex` — 5 labels

`lem:kkt`, `lem:H-quadratic`, `lem:numerical-transition-feasibility`, `lem:q-error-bound`, `lem:well-defined`

### `crates/src/algorithms/math.tex` — 23 labels

Some overlap with geom/kkt (shared labels referenced from multiple modules). Only define each label once — in the module that owns the concept. Cross-reference from others.

**Owned by algorithms:**
`alg:ehz`, `alg:billiard`, `alg:tube`, `cor:adjacency-pruning`, `def:systolic-ratio`, `def:tube`, `def:tube-data`, `def:tube-extension`, `def:tube-close`, `def:rotation-increment`, `def:symplectic-polytope`, `lem:base-point-recovery`, `lem:fixed-point`, `lem:lagrangian-facets`, `lem:prune-action`, `lem:prune-empty`, `lem:prune-rotation`, `lem:prune-simple`, `lem:shoelace`, `lem:sigma-structure`, `rem:beta-to-tau`, `thm:billiard-characterization`, `thm:bounce-bound`, `thm:conformality`, `thm:sympl-invariance`

**Referenced but owned by other modules (don't redefine):**
`lem:kkt` (kkt), `def:ehz-capacity` (geom), `thm:hko-counterexample` (geom), `lem:numerical-transition-feasibility` (kkt), `lem:q-error-bound` (kkt)

### `crates/src/dataset_math.tex` or fold `def:systolic-ratio` into geom/math.tex

`def:systolic-ratio` is used in `dataset.rs` (top-level, no module dir). Probably belongs in geom conceptually.

## Per-label work

For each label:
1. Read the .rs doc comment that references it — extract the one-line English description
2. Create a `\begin{definition/lemma/theorem}...\label{<label>}...\end{...}` environment
3. Write the statement based on the doc comment description (brief, may be incomplete)
4. Add `% [TODO: JÖRN - verify statement, add proof]` after each environment
5. Do NOT write proofs. Do NOT invent mathematical content beyond what the doc comment says.

## Skills to load

- `math-tex` — math.tex conventions, label format, standalone compilation
- `rust-conventions` — understand the doc comment format and cross-reference rules

## Standalone compilation

Each math.tex must compile standalone. Include a minimal preamble:
```latex
\documentclass{article}
\usepackage{amsmath,amssymb,amsthm}
\newtheorem{theorem}{Theorem}
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{definition}[theorem]{Definition}
\newtheorem{corollary}[theorem]{Corollary}
\newtheorem{remark}[theorem]{Remark}
\begin{document}
...
\end{document}
```

## Also: migration cleanup (§2, items 1-2)

While touching these files, also fix:
1. **"orbit" → "candidate"** terminology in `capacity_accumulator.rs` (4 locations) and `saddle_point_solver.rs` — per MEMORY.md, not orbits until closedness established
2. **Duplicate EPS constants**: `projection_solver.rs` and `saddle_point_solver.rs` define same thresholds independently → consolidate into `kkt/mod.rs`

## Deliverable

- math.tex files created, each compiling standalone
- .rs doc comments verified to reference labels that now exist in math.tex
- Migration cleanup items 1-2 done
- All changes committed on the worktree branch
- Report: label inventory (which exist, which need Jörn's attention)
