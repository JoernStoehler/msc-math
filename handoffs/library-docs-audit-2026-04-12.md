# Library architecture docs audit — 2026-04-12

**Task:** TASKS.md `[open] [group:docs] Library architecture docs audit`
**Scope:** `crates/library/src/` — every module, audit-only (zero source edits).
**Reader model:** a new agent opens `mod.rs` (or the single file, for leaves) and
tries to form a correct mental model *without* reading function bodies. Can they
answer these six questions from the docs alone?

1. **Purpose** — what does this module solve?
2. **File map** — which file contains what? Overlaps?
3. **Public API** — entry points, guarantees.
4. **Invariants** — coordinate convention, preconditions.
5. **Math correspondence** — `math.tex` labels for non-trivial logic.
6. **Cross-module deps** — what is consumed, what is assumed.

Severity vocabulary: `[blocker]` (reader cannot proceed without reading every
file) / `[gap]` (info exists but must be assembled) / `[nit]` (minor polish) /
`[ok]` (no action needed, called out explicitly).

---

## Summary

**Hypothesis result: mostly holds with targeted gaps.** Every `.rs` file under
`crates/library/src/` has a module-level `//!` header (7–29 lines). Every
module with non-trivial algorithms has a `math.tex` with `\label`s that source
code cross-references. The KKT module's inline type and constant doc is
particularly thorough.

**Severity counts across the audit:**

- `[blocker]` — 0
- `[gap]`     — 7
- `[nit]`     — 3
- `[ok]`      — 14 (explicit validations)

**Modules with no findings beyond `[ok]`:** `geom/`, `algorithms/billiard/`,
`algorithms/hk2017/`.

**Modules with `[gap]` findings:** top-level (lib.rs + leaves), `algorithms/`
umbrella, `algorithms/tube/`, `kkt/`.

---

## 1. Top-level crate root (`lib.rs`, `math.tex`, leaf files)

**Audit unit:** `crates/library/src/lib.rs` (5-line header, 33 lines total),
`crates/library/src/math.tex` (29 lines, aggregator only), and the leaf files
`constants.rs` (7-line header), `dataset.rs` (9), `derivatives.rs` (15),
`random.rs` (10).

**Current state.** `lib.rs` is a 33-line file: a 5-line `//!` header, seven
`pub mod` declarations, and a `// ── Re-exports: public API surface ──` section
with three sub-sections (`Types`, `Algorithms`, `Geometry utilities`). The
header paragraph describes the library as "Computes the Ekeland-Hofer-Zehnder
capacity c_EHZ(K) via exhaustive enumeration of closed Reeb orbits. Provides
exact rational geometry, KKT solvers, and named polytope constructors for
experiment pipelines." The root `crates/library/src/math.tex` has
`\input{geom/math}`, `\input{kkt/math}`, `\input{algorithms/math}` — no section
for the top-level leaf files. Each leaf file has a good self-contained header
with doccomments on public items.

**Findings:**

- **[gap] `lib.rs:1-5` header omits three of the seven submodules.** The
  2-sentence description mentions rational geometry, KKT solvers, and
  polytope constructors. It does *not* mention `dataset` (JSONL schemas),
  `derivatives` (analytical ∂c/∂a, ∂vol/∂a), or `random` (rejection sampling).
  A reader who opens `lib.rs` to orient themselves will miss three top-level
  surfaces entirely — especially `derivatives`, which holds the gradient
  formulas experiments rely on. They exist as `pub mod` declarations at
  `lib.rs:11-13` but with no surrounding prose.
- **[gap] No top-level map explains how the three big modules interact.** The
  `lib.rs` header says "rational geometry, KKT solvers" but doesn't say that
  `algorithms/` consumes `geom/` + `kkt/`, or that `kkt/` is
  context-independent by design. That dependency structure is only visible by
  reading each `mod.rs` in turn. Jörn's hypothesis holds *locally* — each
  module is well-explained — but the inter-module picture is not surfaced
  anywhere readable.
- **[gap] `derivatives.rs:15` cites `[lem:cap-derivative]` which lives in
  `algorithms/math.tex`, not a colocated `math.tex`.** This is legal (the
  top-level file has no math.tex of its own, and the root `src/math.tex`
  aggregator has no "top-level" section). But a reader who opens
  `derivatives.rs`, sees the `[lem:cap-derivative]` reference, and grep's
  for the label in `src/*.tex` will find nothing — the lemma is two directories
  away. The header already gives an absolute path
  (`crates/library/src/algorithms/math.tex`) so the information is
  reconstructible, but the cross-module placement is irregular enough to
  flag: it breaks the usual "module and its math.tex live together" pattern.
- **[gap] `src/math.tex` is not the authoritative build.** Per
  `.claude/rules/math-tex.md`, `crates/main.tex` is the authoritative
  compilation point because it resolves cross-references with experiment
  lemmas. `src/math.tex` compiles to a local subset. Neither file's header
  states this, and neither mentions the other. A reader opening
  `src/math.tex:6` sees `Build: cd crates/src && pdflatex math.tex` — a
  command that no longer reflects best practice (`latexmk` from `crates/` is
  the canonical build).
- **[nit] `lib.rs:30-32` re-exports `known_polytopes` and `test_utils` as
  modules under the `// Geometry utilities` comment.** These are sub-module
  re-exports, not utility *functions*. Mild category mismatch with the other
  entries above (`volume`, `omega0`, `lagrangian_product`, `regular_polygon_2d`
  etc.), which are individual functions.
- **[ok] Each leaf file has a self-contained header.** `constants.rs:1-7`
  explains the cross-module-tolerance rationale; `dataset.rs:1-9` names both
  row types and their serialization format; `derivatives.rs:1-15` states the
  envelope theorem setup and both formulas; `random.rs:1-9` names the Haar
  measure distribution. These are the best-documented leaves in the crate.
- **[ok] `constants.rs` uses the empirical-constant convention.**
  `EPS_FACET_INCIDENCE` has a rationale comment with f64 rounding argument
  and empirical validation citation (`constants.rs:13-17`), per
  `.claude/rules/rust.md` magic-number section.

**Disposition options for Jörn:**

- **fix-in-place:** expand `lib.rs` header to 15–20 lines — mention all seven
  submodules with one-liners, add a brief paragraph on the
  geom → kkt → algorithms dependency arrow and why `kkt` is
  context-independent. Fix `lib.rs:27` comment and the `known_polytopes`
  re-export category. Update `src/math.tex` header to point at `crates/main.tex`
  as the authoritative build. ~30 lines of new doc text, no source-code impact.
- **extract architecture.md:** add
  `crates/library/ARCHITECTURE.md` with the module dependency graph and the
  kkt-is-context-free invariant. Probably overkill unless the library grows.
- **fine as-is:** close the gap by telling new readers to open `geom/mod.rs`
  first (where the overview actually lives). Requires no edits but fails the
  new-reader simulation.

---

## 2. `geom/`

**Audit unit:** `geom/mod.rs` (18-line header), `geom/math.tex` (1041 lines,
~28 labeled lemmas/definitions plus the prefilter subsection), 15 `.rs` files.

**Current state.** `geom/mod.rs` is 37 lines: an 18-line `//!` header that
lists every one of the 15 submodules with a one-line description, followed by
`pub mod` declarations and one re-export (`QhullError`). Every submodule file
has a `//!` header (7–15 lines). `polytope.rs:1-9` documents the central
`Polytope4D` type with explicit `math.tex` references (`[def:polytope-dual]`,
`[def:polar-body]`) and a detailed invariants block at `polytope.rs:37-49`.
`symplectic_form.rs:7` explicitly states the coordinate convention.
`geom/math.tex` has labeled definitions for symplectic form, J0, EHZ capacity,
systolic ratio, Lagrangian product, symplectic product, polytope dual, face
lattice, cross product, polygon, volume, Reeb vector field, plus lemmas on
piecewise-linear Reeb trajectories, positive-span boundedness, bounded-triples
check, vertex enumeration, irredundancy, integer Cramer's rule, rational
pipeline, symplectic shoelace, and the HKO counterexample theorem.

**Findings:**

- **[ok] File map is complete and 1:1 with the directory.** `geom/mod.rs:3-18`
  lists 15 submodules; `ls geom/*.rs` returns exactly those 15 names plus
  `mod.rs`. Nothing is hidden.
- **[ok] Coordinate convention is stated in-source, matching the project
  rule.** `symplectic_form.rs:7`: "Coordinate convention: (q_1, q_2, p_1, p_2),
  where q = position and p = momentum. This convention is used consistently
  throughout the crate." This matches the instruction in
  `.claude/rules/rust.md` ("Defined in `geom/symplectic_form.rs`"). A new
  reader looking for the convention finds it in the file the rule names.
- **[ok] `Polytope4D` invariants are enforced and documented.**
  `polytope.rs:37-49` states the five invariants (F≥5, nonzero dual vertices,
  boundedness, irredundancy, exact rational precomputation) and the
  exact/f64 representation split. These match the type's private fields
  (`polytope.rs:52-60+`) and are enforced by the constructor — consistent
  with the `.claude/rules/rust.md` "types encode invariants, validated in
  `::new()`" rule.
- **[ok] Every non-trivial `.rs` file has a `math.tex` cross-reference.**
  `cross_product_4d.rs`, `symplectic_form.rs`, `polygon.rs`,
  `lagrangian_product.rs`, `volume.rs`, `reeb_trajectory.rs`,
  `rational_arithmetic.rs`, `vertex_enumeration.rs`, `polytope.rs` — all cite
  at least one label from `geom/math.tex`.
- **[ok] `geom/math.tex` has no obvious gaps vs. the code.** All 15 geom
  submodules have matching definitions or lemmas; utility files
  (`qhull.rs`, `test_utils.rs`, `facet_volume.rs`, `validation.rs`,
  `known_polytopes.rs`) have headers pointing at their supporting lemma
  where one exists and stand on their own as glue code where none is
  needed.

**Disposition:** **fine as-is.** The geom module is the best-documented
surface in the crate and sets the template other modules should match.

---

## 3. `kkt/`

**Audit unit:** `kkt/mod.rs` (18-line header, 150 lines total with public
types and constants),`kkt/math.tex` (529 lines, ~7 labeled lemmas), 6 `.rs`
files.

**Current state.** `kkt/mod.rs` has the strongest inline architecture doc in
the crate: an 18-line `//!` header with problem statement, a
"context-independent" design note, math-label cross-refs, and a six-line
submodule file map. Below the header it declares `pub mod`s, defines
`struct QP`, `enum Verdict`, `struct Solution` (all with multi-line
doccomments), and documents the threshold constants `EPS_MARGIN_TRUE`,
`EPS_MARGIN_FALSE`, `EPS_EIGEN_FLOOR` with rationale paragraphs that satisfy
the magic-number convention. `kkt/math.tex` has labels for
`lem:H-quadratic`, `lem:kkt`, `lem:well-defined`, `lem:dual-vertex-qp`,
`lem:numerical-transition-feasibility`, `lem:q-error-bound`, and
`rem:near-null-lp-search`. Every submodule file has a 11–23-line header.

**Findings:**

- **[ok] Problem statement, invariants, and file map all present at the top.**
  `kkt/mod.rs:1-18` is a template that could be copied for future modules.
- **[ok] `Verdict` carries a critical-invariant comment.**
  `kkt/mod.rs:52-55` documents the "False is never returned unless certified
  infeasible" invariant, which is load-bearing for accumulator correctness
  (the accumulator promotes Indeterminate candidates into the uncertain tier
  and asserts the gap invariant; see `algorithms/capacity_accumulator.rs`).
  A reader who ignored this would mis-model the feasibility enum.
- **[ok] Magic-number constants justified per project rule.**
  `EPS_MARGIN_TRUE` (`kkt/mod.rs:88-98`), `EPS_MARGIN_FALSE`
  (`kkt/mod.rs:100-107`), `EPS_EIGEN_FLOOR` (`kkt/mod.rs:109-124`) each have
  rationale paragraphs covering the data point, the 10× sensitivity, and
  cross-solver scope.
- **[gap] The submodule list does not flag that `rational_solver` is not in
  the main pipeline.** `kkt/mod.rs:18` reads "`rational_solver` — Exact
  rational KKT solver" — a neutral one-liner. But
  `rational_solver.rs:9-12` says: "The exact solver serves as ground truth
  for validating the f64 solver's error bounds and for computing exact
  capacity values when floating-point ambiguity is unacceptable. It is NOT
  used in the main capacity enumeration pipeline (too slow for sweeping all
  permutations)." A reader of `kkt/mod.rs` alone would assume all six
  submodules are equally load-bearing on the hot path, and waste time
  studying `rational_solver` before realizing it is a validator. One-line
  fix: append "(validation/ground-truth; not in main pipeline)" to the
  submodule entry.
- **[gap] `kkt/mod.rs` does not state where the pipeline branches between the
  saddle-point and projection solvers.** Both
  `saddle_point_solver` and `projection_solver` are listed, but the header
  does not say which one callers use, under what conditions, or whether one
  subsumes the other. Reading `projection_solver.rs:1-23` reveals a
  five-step algorithm; reading `saddle_point_solver.rs:1-15` reveals the
  augmented-system approach. They are two strategies for the same QP —
  `kkt/mod.rs` should say so, or say which is the default.
- **[ok] `beta_feasibility.rs` header explicitly names both callers** (the
  projection solver Step 4 and the saddle-point null-space search) and the
  `[rem:near-null-lp-search]` backing, which is strong cross-module
  documentation.
- **[ok] `constraint_solver.rs:7-9` explicitly calls itself "context-free
  linear algebra"** — matches the `kkt/mod.rs:5-7` context-independence claim
  and reinforces the module's design intent.
- **[nit] Formula in `kkt/mod.rs:3` says "max (1/2) beta^T H beta s.t. beta >
  0".** Our matching `qp_assembly.rs:8` / `projection_solver.rs:3` say the
  same. This is fine; but `kkt/math.tex:38` names
  `[lem:H-quadratic]` as the quadratic form, while `kkt/mod.rs:130`
  references it from `q_value` — the inline reference is correct, though
  the one-line English summary ("`Q(beta) = (1/2) beta^T H beta` where
  `H_{ij} = omega_0(a_i, a_j)`") is in the doccomment, not the header.

**Disposition:** **fix-in-place, two one-liner additions.** Flag
`rational_solver` as validation-only (1 line) and name the saddle-vs-projection
branching point (1–2 sentences at the top of the submodule list).

---

## 4. `algorithms/` umbrella (`algorithms/mod.rs`, `algorithms/math.tex`, utilities)

**Audit unit:** `algorithms/mod.rs` (10-line header, 16 lines total),
`algorithms/math.tex` (938 lines, ~25 labeled entries), `capacity_accumulator.rs`
(18-line header), `facet_adjacency.rs` (9-line header).

**Current state.** `algorithms/mod.rs:1-10` lists three algorithms and two
shared utilities. `algorithms/math.tex` has labels for `alg:ehz`,
`cor:adjacency-pruning`, `thm:conformality`, `thm:sympl-invariance`,
`lem:base-point-recovery`, `lem:lagrangian-facets`, `lem:sigma-structure`,
`thm:billiard-characterization`, `thm:bounce-bound`, `alg:billiard`,
`def:symplectic-polytope`, `def:tube`, `def:tube-data`, `def:tube-extension`,
`def:rotation-increment`, `def:tube-close`, `lem:prune-empty`,
`lem:prune-action`, `lem:prune-rotation`, `lem:prune-simple`,
`lem:fixed-point`, `alg:tube`, plus a `sec:sys-optimization` subsection with
`lem:cap-derivative`, `lem:vol-derivative`,
`prop:capacity-piecewise-smooth`, and `cor:sys-derivative`.
`capacity_accumulator.rs:1-18` explains the two-tier tracking and gap invariant.
`facet_adjacency.rs:1-9` documents the omega_0-aware directed adjacency.

**Findings:**

- **[gap] `algorithms/mod.rs:6` describes `tube` as "tube algorithm
  (placeholder)".** This is stale:
  `algorithms/tube/mod.rs` is 1364 lines of implementation with a
  `pub fn tube_capacity` entry point, full error-type classification,
  `TubeResult` struct, precomputation pass, DFS search, pruning, and
  closure solver. The `(placeholder)` tag is misleading. Per TASKS.md
  `[Jörn] [group:tube] tube-algorithm.tex` and `[blocked] Tube rotation
  formula implementation` the *mathematical writeup* has open TODOs, but the
  *code* is implemented. A new reader who trusts
  `algorithms/mod.rs` will skip reading `tube/mod.rs` entirely and fail to
  find the third working capacity algorithm. One-line fix: replace
  `(placeholder)` with `(symplectic polytopes; rotation formula TODO —
  [blocked] tube-algorithm.tex)` or similar honest status.
- **[gap] `algorithms/mod.rs:1-10` does not name the shared invariant
  across the three algorithms.** `.claude/rules/rust.md` "Algorithms"
  section says "Where domains overlap, algorithms must agree on computed
  capacity." This is a load-bearing correctness invariant that
  `algorithms/billiard/mod.rs` tests explicitly (see the
  `agrees_with_hk2017_*` tests at `billiard/mod.rs:273-339`). It should be
  mentioned in the umbrella `mod.rs` header so a reader understands *why*
  three algorithms coexist.
- **[ok] `capacity_accumulator.rs` has a template-quality header.** States
  the shared pattern, names the three callers (`ehz_capacity_unpruned`,
  `ehz_capacity`, `billiard_capacity`), defines the two tracking tiers, and
  specifies the gap invariant with a numerical justification. The rationale
  for `GAP_TOLERANCE = 1e-10` at line 22-27 cites a specific
  failure mode ("(4,4) at theta ~ 0 degrees produce gaps up to ~2.4e-11").
- **[ok] `facet_adjacency.rs` correctly cites both relevant lemmas**
  (`lem:numerical-transition-feasibility`, `cor:adjacency-pruning`).
- **[ok] `algorithms/math.tex` covers every code entry point.** `alg:ehz`,
  `alg:billiard`, `alg:tube` are all labeled algorithms with corresponding
  pruning/closure lemmas. No code function lacks a math.tex entry.
- **[nit] `algorithms/mod.rs` header does not cite
  `capacity_accumulator.rs`'s gap invariant or
  `facet_adjacency.rs`'s pruning corollary as shared-utility math labels.**
  The two utilities are named but without their own `math.tex`
  cross-references. Minor: their own headers supply this, so assembly cost
  is ~30 seconds.

**Disposition:** **fix-in-place.** Two one-line changes to
`algorithms/mod.rs`: un-stale the tube entry, add the "algorithms must agree
on overlap" invariant. ~6 new lines total.

---

## 5. `algorithms/billiard/`

**Audit unit:** `algorithms/billiard/mod.rs` (20-line header, 449 lines total
including tests), three submodule files (`block_enumeration.rs`,
`facet_classification.rs`, `kkt_benchmark.rs`, 9–11 header lines each).

**Current state.** `billiard/mod.rs:1-20` is the clearest "here is how the
algorithm works" header in the crate. It states the problem (Lagrangian
products), cites the characterization theorem, cites the bounce bound, names
the complexity win (O(n^3) vs. O(n!)), lists the three submodules with
descriptions, explicitly mirrors the hk2017 accumulator pattern, and ends
with cross-references. The file is followed by `BilliardError`,
`BilliardResult`, `billiard_capacity`, internal `solve_and_convert`, and a
five-section test module (agreement-vs-known, agreement-vs-hk2017,
error-handling, property, helpers).

**Findings:**

- **[ok] Everything the reader-simulation needs is in the first 20 lines.**
  Problem, theorem, complexity, submodule map, algorithm pattern,
  cross-refs.
- **[ok] Test organization is explicit.** The comment at lines
  194-200 says: "Tests for billiard capacity: correctness and
  cross-validation with hk2017. Proposition: ... Strategy: fixture-based
  (known polytopes), cross-algorithm (billiard vs hk2017)." Section
  comments inside the test module (`============================`) separate
  agreement-vs-known, agreement-vs-hk2017, error handling, property tests.
- **[ok] Cross-algorithm invariant tested in code.** `billiard/mod.rs:273-339`
  has four `agrees_with_hk2017_*` tests, directly verifying the invariant
  that algorithms must agree on overlapping domains. These are ignored
  in debug mode but run with `--ignored`, consistent with the project
  convention of pushing long-running cross-checks behind `--ignored`.

**Disposition:** **fine as-is.**

---

## 6. `algorithms/hk2017/`

**Audit unit:** `algorithms/hk2017/mod.rs` (23-line header, 1966 lines total
with tests), three submodule files (`permutations.rs`, `orbit_recovery.rs`,
`generate_capacity_fixtures.rs`, 13–29 header lines each).

**Current state.** `hk2017/mod.rs:1-23` states the enumeration strategy,
names both entry points (`ehz_capacity`, `ehz_capacity_unpruned`), cites the
adjacency pruning corollary, lists the submodules, and gives the exponential
complexity formula `sum_{m=2}^{F} C(F,m) * (m-1)!`. Both entry points have
doc comments with permutation-ordering convention (positive Reeb direction).
The file contains only two `pub fn`s and a handful of helpers (lines 69–242);
the remaining ~1720 lines are three test modules
(`tests_literature`, `tests_kkt_edge_cases`, and more at line 875+).

**Findings:**

- **[ok] `hk2017/mod.rs:1-23` answers every reader-simulation question.**
  Problem statement, algorithm strategy, two entry points clearly
  distinguished, submodule map, complexity, math label.
- **[ok] Permutation ordering convention is stated in source.**
  `hk2017/mod.rs:62-66` says `sigma = [a, b, c, ...]` means
  F_a → F_b → F_c → ... → F_a with `omega_0(n_sigma(k), n_sigma(k+1)) >= 0`.
  This is the kind of convention a reader would otherwise have to reverse-
  engineer from the omega-sign checks.
- **[ok] Only one internal section marker (`// ── Internal helpers ──` at
  line 204) is needed** because the production code is compact (~170 lines)
  and everything else is tests — organized into three named test modules
  (`tests_literature`, `tests_kkt_edge_cases`, ...) that act as the section
  headers. Not a documentation gap.

**Disposition:** **fine as-is.**

---

## 7. `algorithms/tube/`

**Audit unit:** `algorithms/tube/mod.rs` (19-line header, 1364 lines total
with tests), no submodules.

**Current state.** `tube/mod.rs:1-19` states the problem (symplectic
polytopes, no Lagrangian 2-faces), cites `alg:tube`, clarifies the Type 1
vs. Type 2 orbit scope (only Type 1 searched; generic polytopes have no
Type 2 minimum-action orbit per CH2021 Conj. 1.26), gives the complexity
band, and lists math-label references. The file has clear section markers
(`// ── Error types ──`, `// ── Result type ──`, `// ── Precomputed data ──`,
`// ── Public API ──`, `// ── Precomputation ──`, `// ── DFS Search ──`,
`// ── Utility functions ──`) that mirror the `math.tex` structure.

**Findings:**

- **[ok] Section markers match `math.tex` structure.** `// ── DFS Search ──`
  at line 360 corresponds to `alg:tube`, the precomputation section to
  `def:tube-data` / `def:rotation-increment`, the closure routine to
  `def:tube-close` + `lem:fixed-point`. A reader navigates the 1364-line
  file the same way they navigate the math.
- **[gap] Header does not mention the mathematical writeup's open TODOs.**
  TASKS.md flags `[Jörn] [group:tube] tube-algorithm.tex (8 TODOs)` and
  `[blocked] [group:tube] Tube rotation formula implementation`. The code
  does not warn its reader that the backing `algorithms/math.tex` tube
  section has unverified content or that the rotation formula is not the
  final form. A one-line comment of the form "[alg:tube] has 8 [TODO:
  JÖRN] markers in algorithms/math.tex — results from this module should
  not be trusted until verified" would save a reader from building on an
  unstable foundation. Compare the `.claude/rules/math-tex.md` mark-unverified
  convention.
- **[gap] Header does not state whether `tube_capacity` is currently called
  from anywhere in the pipeline.** A grep of `algorithms/mod.rs` and
  `lib.rs` re-exports shows that
  `billiard_capacity` and `ehz_capacity` are re-exported at the crate root
  but `tube_capacity` is not. The reader cannot tell from the header whether
  this is by design (tube is under active development and not yet ready for
  public re-export) or by omission. A one-line "not currently re-exported
  from `lib.rs` pending validation" would disambiguate.
- **[ok] The DFS algorithm's non-obvious pre-filters are documented.**
  `tube/mod.rs:36-44` explains that Lagrangian 2-faces trigger
  `HasLagrangian2Face`, citing `def:symplectic-polytope`. Readers understand
  the precondition from the error type alone.

**Disposition:** **fix-in-place.** Two short additions to the `//!` header:
note the unverified math (one line), note the lack of crate-root re-export
(one line). ~4 new lines total.

---

## Cross-module observations

1. **Every `.rs` file in the library has a `//!` header.** Zero missing
   module docs. Header length ranges from 7 to 29 lines; mean ~13. The
   audit-baseline hypothesis ("existing state — file headers + doccomments
   + per-module math.tex — covers architecture") is correct at the file
   level. Gaps are at the umbrella-level docs, not the leaf-level.

2. **No `README.md` or `ARCHITECTURE.md` exists anywhere under
   `crates/library/`.** The `lib.rs` header is the only top-level orientation
   material. This is a load-bearing deficit only if the inter-module graph
   (`geom` → `kkt` → `algorithms`; `kkt` context-free; `derivatives` crosses
   to `algorithms/math.tex`) needs to be surfaced somewhere. If yes, it
   needs a top-level doc or a `lib.rs` header expansion.

3. **`math.tex` labels are well-respected across modules.** Every code
   reference to a lemma uses the `[lem:label]` / `[thm:label]` /
   `[alg:label]` convention mandated by `.claude/rules/rust.md`. No invented
   labels surfaced during the audit. The only cross-module reference is
   `derivatives.rs → algorithms/math.tex`, which is irregular but correct.

4. **The `src/math.tex` aggregator is not the authoritative build.** The
   root compilation lives at `crates/main.tex` and resolves
   cross-references with experiment lemmas. Neither the `src/math.tex`
   header nor `lib.rs` mentions this; the build command
   `src/math.tex:6 "Build: cd crates/src && pdflatex math.tex"` is stale.
   The current canonical path (per `.claude/rules/math-tex.md`) is
   `cd crates/ && latexmk`.

5. **`kkt/` is the strongest-documented module;** `algorithms/` umbrella and
   `lib.rs` are the weakest. `geom/` sits in the middle: individual files
   are strong, the umbrella header is complete, but there is no overview
   beyond the file list (e.g., no statement that the `Polytope4D` type is
   the single central object and that all other geom files either build it
   or consume it).

6. **Algorithm status is documented unevenly.** `billiard` and `hk2017` are
   presented as complete; `tube` is presented as "placeholder" in
   `algorithms/mod.rs` (stale), but as a working algorithm with
   unverified math in its own `mod.rs` (partially explicit). A reader sees
   different statuses depending on which file they open first.

---

## Spot-check log

Three claims from the draft above, re-verified against the files:

**Claim 1 (from §4):** "`algorithms/mod.rs:6` describes `tube` as 'tube
algorithm (placeholder)'."
- **Source verified:** `crates/library/src/algorithms/mod.rs:6` reads
  `//! - \`tube\` — tube algorithm (placeholder)`. ✓ Exact match.
- **Implementation counterclaim verified:** `wc -l algorithms/tube/mod.rs` =
  1364 lines; `grep '^pub fn' algorithms/tube/mod.rs` returns
  `tube_capacity` and `check_symplectic`. The file is not a placeholder. ✓
- **Held.**

**Claim 2 (from §3):** "`kkt/mod.rs:52-55` documents the 'False is never
returned unless certified infeasible' invariant."
- **Source verified:** `kkt/mod.rs:53-55` reads "**Critical invariant:** False
  is never returned unless certified infeasible. When in doubt, return
  Indeterminate. The accumulator handles resolution (e.g. via rational
  fallback)." ✓ Exact wording present.
- **Load-bearing check:** `capacity_accumulator.rs:11-18` confirms the
  accumulator relies on this — it routes Indeterminate results into an
  uncertain tier and asserts a gap invariant at finalization. Correctness
  depends on the three-way discipline. ✓
- **Held.**

**Claim 3 (from §1):** "`derivatives.rs:15` cites `[lem:cap-derivative]`
which lives in `algorithms/math.tex`, not a colocated `math.tex`."
- **Source verified:** `derivatives.rs:15` reads "Mathematical
  correspondence: [lem:cap-derivative], [lem:vol-derivative] in
  crates/library/src/algorithms/math.tex" ✓ Explicit cross-directory path.
- **Label existence verified:** `grep 'lem:cap-derivative\|lem:vol-derivative'
  algorithms/math.tex` returns matches at `algorithms/math.tex:714` and
  `algorithms/math.tex:786`, inside a `\subsection{Derivatives of the
  Systolic Ratio}` at line 682. ✓
- **Top-level `src/math.tex` aggregator check:** `crates/library/src/math.tex`
  contains only `\input{geom/math}`, `\input{kkt/math}`,
  `\input{algorithms/math}` — no top-level section, confirming there is no
  colocated math.tex for the top-level files. ✓
- **Held.** (Note: this claim flagged an irregularity, not a correctness
  bug. The absolute path in the header does make the lemma findable.)

All three spot-checked claims held against the source. No corrections
required.

---

## Appendix — files read during audit

Headers and math-label tables: `lib.rs`, `math.tex`, `constants.rs`,
`dataset.rs`, `derivatives.rs`, `random.rs`, `geom/mod.rs`,
`geom/polytope.rs`, `geom/symplectic_form.rs`, `geom/math.tex` (label TOC),
`kkt/mod.rs`, `kkt/math.tex` (label TOC), `kkt/qp_assembly.rs`,
`kkt/beta_feasibility.rs`, `kkt/projection_solver.rs`,
`kkt/constraint_solver.rs`, `kkt/saddle_point_solver.rs`,
`kkt/rational_solver.rs`, `algorithms/mod.rs`, `algorithms/math.tex`
(label TOC), `algorithms/capacity_accumulator.rs`,
`algorithms/facet_adjacency.rs`, `algorithms/billiard/mod.rs`,
`algorithms/hk2017/mod.rs` (partial + structure), `algorithms/tube/mod.rs`
(partial + structure). Header-line counts sampled via
`awk '/^\/\/!/{c++; next} {exit} END{print c+0}'` for every `.rs` file under
`geom/`, `kkt/`, `algorithms/` to confirm header presence.
