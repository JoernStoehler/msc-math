# Clarity QC Review — 2026-02-12

**Base commit**: `4f36233` (local `main`)
**Branch**: `claude/qc-clarity-review`
**Scope**: Repo-wide clarity of writing (code, comments, doc comments, prose). Tests excluded.
**Method**: 25 Opus 4.6 subagents, each reviewing one chunk line-by-line.

## Executive Summary

**332 findings** across 25 reviews. **5 critical, 43 major, 174 minor, 110 nit.**

The codebase is in solid B-grade shape overall — readable, well-structured, and mostly self-documenting. The two problem areas are both in the thesis LaTeX:

1. **simple-minimizer-proof.tex** (Grade C): Two proof gaps that block verification of the main theorem's proof, plus five major clarity issues with unproven claims and missing definitions.
2. **chapter-algorithm.tex Section 4** (Grade C+): The entire "Algorithm Correctness" section is empty skeletons — 10 consecutive empty definition/theorem/lemma environments.

These are expected for draft thesis content, but they dominate the critical findings.

## Grade Distribution

| Grade | Count | Files |
|-------|-------|-------|
| B | 22 | Most Rust, Python, docs, and thesis sections 1-2 |
| C+ | 1 | chapter-algorithm.tex Sec 3-5 |
| C | 2 | simple-minimizer-proof.tex, correspondence+main.tex |

## Finding Totals by Severity

| Severity | Count | % |
|----------|-------|---|
| Critical | 5 | 1.5% |
| Major | 43 | 13.0% |
| Minor | 174 | 52.4% |
| Nit | 110 | 33.1% |

## Finding Totals by Category

| Category | Count |
|----------|-------|
| missing-doc | ~80 |
| missing-context | ~55 |
| unexplained-magic | ~30 |
| clarity-blocks-verification | ~25 |
| unclear-naming | ~20 |
| inconsistent-terminology | ~20 |
| misleading-comment | ~15 |
| poor-structure | ~15 |
| stale-comment | ~10 |
| other | ~62 |

---

## Hotspots

### CRITICAL: simple-minimizer-proof.tex (Grade C — 2 crit, 5 major)

The proof of primal-dual equivalence has two verification-blocking gaps:

1. **Euler-Lagrange gap** (lines 189-207): The derivation shows p(t) = b (constant), then claims the critical point condition is "nu z(t) + b in subdifferential(...)". The nu*z(t) term appears from nowhere. The code's own GAP comment acknowledges this. Blocks verification of the entire primal-dual equivalence.

2. **Incomplete nu=1 skeleton** (lines 224-256): The argument that nu=1 at the minimizer is a bullet-point skeleton with no actual proof. The construction of a competing curve (needed to show nu != 1 implies non-minimality) is absent. Then the text continues with "Assuming nu=1..."

Additional major issues:
- Fenchel duality lemma stated without proof (used in Steps 1 and 3)
- Piecewise affine approximation claim cites HK2017 without inline proof (violates thesis conventions)
- Splitting claim (Step 2) defers to HK2017 without inline proof
- Action identity appears late, unproven, unlabeled — should be a separate lemma
- Step 2 claims I_K(z'') = T exactly, but doesn't explain the transition from approximate to exact

### CRITICAL: chapter-algorithm.tex Section 4 (Grade C+ — 2 crit, 3 major)

1. **All 10 definition/theorem bodies are empty** (lines 628-661): `def:dual-variable`, `def:dual-functional`, `def:q-function`, `def:constraint-set`, `def:action-matrix`, `thm:optimization`, `lem:domain-decomposition`, `lem:global-at-local`, `lem:local-max`, `cor:enumeration` — all empty skeletons. The algorithm in Section 1 depends on quantities defined here.

2. **Theorem `thm:orbit-existence` has no proof or citation** (lines 581-597). The QC comment mentions Rabinowitz 1978 and Weinstein 1978 but the theorem body has no citation.

3. **Visible `[JORN: ...]` TODO in the PDF** (lines 249-256): Bold-text TODO in the rendered document creates a visible hole in the proof chain.

### CRITICAL: generate_figures.py (Grade B — 1 crit, 2 major)

1. **`plot_facet_vs_capacity` docstring is wrong on 3 counts**: Says "capacity" but code plots systolic ratio; says "timing as color" but timing is a separate subplot; says "scatter plot" but produces two subplots. Reader would form incorrect understanding.

---

## Cross-Cutting Themes

### 1. Missing module-level documentation (~15 files)
The most common finding across Rust files. Module docs either don't exist, are too terse (one-line summaries), or don't explain the module's role in the project. Frequently the mathematical purpose is clear but the project-level purpose is missing.

**Affected files**: validation.rs, dataset.rs, random.rs, acceptance_sweep.rs, permutations.rs, vertices.rs, qhull.rs (module doc too narrow), known_polytopes.rs, volume.rs, cross_product.rs, main.rs (no module doc), lib.rs (datasets).

### 2. Unexplained magic numbers / constants (~30 findings)
Numeric thresholds, tolerances, and hardcoded values appear without rationale throughout the Rust crates. Common examples:
- `1e-9`, `1e-8`, `1e-10` tolerance values without explanation of how they were chosen
- `0.5..2.0` height ranges without justification
- `MAX_FACETS_BRUTEFORCE = 10` without performance analysis
- `SENTINEL = -10.101`, `TOLERANCE = 0.001` in qhull.rs without rationale for tolerance

### 3. Inconsistent Lagrangian vs. symplectic product terminology
`triangle_product` is labeled "Lagrangian product" in both `known_polytopes.rs` and `test_utils.rs`, but its normal vectors place the two triangles in symplectic planes `(q1, p1)` and `(q2, p2)`, not Lagrangian subspaces `(q1, q2)` and `(p1, p2)`. This is a clarity issue that may also hide a correctness issue — it appears in at least 3 files.

### 4. Thesis cross-references are stale or non-existent (~10 findings)
Rust doc comments reference thesis sections by number ("thesis §5, Corollary 5.3") but the thesis uses named sections ("Algorithm Correctness", "Optimizations"), not numbers. Several references point to thesis content that hasn't been written yet. The `BOUNDEDNESS_INVESTIGATION.md` reference in qhull.rs points to a non-existent file.

### 5. Notation introduced before defined (thesis LaTeX)
Several symbols appear in the thesis before their definitions:
- `Q(beta)` used in Algorithm Step 4 before being defined in Remark 3.1
- `H` (action matrix) used in the algorithm before Section 4 defines it (and Section 4 is empty)
- `sigma in S_F` in the theorem vs `sigma` as permutation of subset `S` in the algorithm — the equivalence is non-trivial and unstated

### 6. Sub-CLAUDE.md conflicts with root CLAUDE.md
Thesis/CLAUDE.md Content Rule 1 says "Do NOT write anything Jorn did not dictate." Root CLAUDE.md Role 7 says "Claude Code is perfectly capable of writing mathematical prose." These directly conflict for proof-writing tasks.

---

## Per-File Review Summaries

### Rust — crates/geom/

| File | Grade | Findings | Key issues |
|------|-------|----------|------------|
| polytope.rs | B | 0c 1M 5m 4n | EPS constants undocumented; no antiparallel normal check |
| symplectic.rs | B | 0c 0M 4m 3n | Coordinate convention subtlety in block-matrix doc; j2 lacks doc |
| cross_product.rs | B | 0c 1M 4m 3n | Sign convention not stated; dense arithmetic body opaque |
| vertices.rs | B | 0c 0M 5m 3n | Thin delegation module; thesis reference should be self-contained |
| volume.rs | B | 0c 1M 5m 4n | `simplex_volume_5` name encodes arity not math; deprecated code present |
| qhull.rs | B | 0c 2M 9m 6n | Sentinel OR-vs-AND ambiguity; `InputWriteFailed` misleading for non-write errors |
| test_utils.rs | B | 0c 1M 12m 5n | Simplex doc claims centroid translation but code translates origin; many minor doc gaps |

### Rust — crates/hk2017/

| File | Grade | Findings | Key issues |
|------|-------|----------|------------|
| lib.rs | B | 0c 5M 8m 6n | Stale thesis references; H matrix symmetry doc unclear; Q(beta) and M(K) undefined in module doc |
| permutations.rs | B | 0c 0M 4m 4n | Clean code; missing module doc and math context for why cyclic permutations matter |

### Rust — crates/datasets/

| File | Grade | Findings | Key issues |
|------|-------|----------|------------|
| validation.rs | B | 0c 0M 9m 4n | Well-structured; pipeline step descriptions imprecise |
| known_polytopes.rs | B | 0c 2M 5m 5n | Crosspolytope has unknown capacity in "known" collection; Lagrangian/symplectic terminology confusion |
| random.rs | B | 0c 1M 5m 3n | Magic 1e-10 threshold; module doc too terse |
| acceptance_sweep.rs | B | 0c 1M 4m 4n | Module doc too terse; sweep grid constants lack rationale |
| dataset.rs + main.rs | B | 0c 0M 9m 8n | Module doc not actually a doc comment; "dataset 1"/"dataset 2" undefined |
| test_dataset.rs | B | 0c 1M 12m 5n | Simplex centroid doc inconsistency; Cayley scale 0.3 unexplained; duplicated hypercube fixture |

### Thesis — LaTeX

| File | Grade | Findings | Key issues |
|------|-------|----------|------------|
| chapter-algorithm Sec 1 | B | 0c 3M 5m 4n | Q(beta) used before defined; max-vs-min unexplained; sigma scope mismatch |
| chapter-algorithm Sec 2 | B | 0c 2M 11m 5n | "Positive characteristic direction" undefined; hat/unhat notation inconsistency |
| chapter-algorithm Sec 3-5 | C+ | 2c 3M 7m 5n | Section 4 entirely empty; orbit existence unproven; visible TODO |
| simple-minimizer-proof | C | 2c 5M 5m 3n | Two proof gaps; five unproven claims; see Hotspots above |
| correspondence+main.tex | C | 0c 3M 9m 4n | Empty abstract/intro; adjacency graph row meaningless; R_i renaming unexplained |
| experiments/*.tex | B | 0c 1M 10m 7n | systolic-ratios.tex missing file header; stale comment in wrapper; stub data warning |

### Python — experiments/scripts/

| File | Grade | Findings | Key issues |
|------|-------|----------|------------|
| generate_figures.py | B | 1c 2M 5m 3n | Misleading docstring (3 errors); hardcoded output paths |
| run_dataset.py + timing_model.py | B | 0c 2M 7m 4n | run_dataset.py duplicates data; timing_model constant unexplained |

### Documentation

| File | Grade | Findings | Key issues |
|------|-------|----------|------------|
| CLAUDE.md (root) | B | 0c 0M 9m 6n | Well-organized; some sections could use examples; a few terms undefined on first use |
| Sub-CLAUDE.md files | B | 0c 3M 13m 4n | Experiments CLAUDE.md "continuous spectra" is abstract; Python-Rust contradiction; thesis content rule conflicts with root |
| knowledge-dump.md | B | 0c 4M 5m 3n | "Talk normalization" undefined jargon; H matrix definition appears inconsistent; splitting mechanism unclear |

---

## Methodology

- **Triage**: 4 Sonnet/Haiku agents classified all files by priority (HIGH/MEDIUM/LOW)
- **Deep review**: 25 Opus 4.6 agents, each assigned one chunk with explicit scope
- **Prompt files**: 6 reusable prompt files (`/tmp/review-{instructions,rust,latex,python,docs,repo-map}.md`)
- **Constraints**: Read-only. No tests, builds, or commands run. No mathematical correctness checking. Findings flagged as `clarity-blocks-verification` when unclear writing might hide correctness issues.
- **Grading**: A-F scale (see review-instructions.md). Severity: critical/major/minor/nit.
- **Duration**: ~25 minutes wall-clock for all 25 reviews

## Fixes Applied (Tier 1)

**Branch**: `claude/qc-clarity-fixes` (19 commits, forked from `claude/qc-clarity-review`)
**Scope**: ~102 Tier 1 findings — autonomously fixable doc/comment/naming clarity improvements. No mathematical content changes, no convention decisions, no structural changes.

### Commits

| # | Commit | Scope | Key changes |
|---|--------|-------|-------------|
| 1 | `9bbac56` | geom/cross_product.rs | Extract 6 named 2×2 minors from dense arithmetic |
| 2 | `535adc6` | geom/symplectic.rs | Remove redundant J₀ doc, clarify j4 binding |
| 3 | `448d8b7` | geom/volume.rs | Extract `EPS_DEGENERATE`, add triangle early-return comment |
| 4 | `896496a` | geom/vertices.rs | Add intra-doc links for Polytope4D types |
| 5 | `0349e08` | geom/qhull.rs | Inline dead ref, fix test names, add format notes |
| 6 | `3163e17` | geom/polytope.rs | Doc comments on accessors and ConstructionError |
| 7 | `607fbb7` | hk2017/lib.rs | Extract 5 magic numbers to named constants |
| 8 | `bbdde23` | hk2017/permutations.rs | Doc edge cases, return types, parameters |
| 9 | `e49de38` | datasets/validation.rs | Document EPS constants, fix misleading comment |
| 10 | `0ea621f` | datasets/known_polytopes.rs | Fix stale ref, expand docs, clarify formulas |
| 11 | `f7ee40c` | datasets/random+acceptance_sweep | Extract constant, rename `n→count`, `ok→accepted` |
| 12 | `fc06e45` | datasets/dataset+main | Field docs, `&PathBuf→&Path` idiom fix |
| 13 | `480a04d` | test_utils+test_dataset | Verification hints, clarify dataset counts |
| 14 | `c6624bc` | generate_figures.py | **CRITICAL**: Fix docstring wrong on 3 counts |
| 15 | `95daca9` | run_dataset+timing_model.py | Docstrings, `repo_root→REPO_ROOT` |
| 16 | `9cdac56` | thesis (6 .tex files) | Headers, TODO format, stale markers, caption fix |
| 17 | `3dc11a6` | CLAUDE.md + crates/CLAUDE.md | Tool name fix, concrete conventions |
| 18 | `faa8539` | knowledge-dump.md | Derivation for combinatorial identity |
| 19 | `ccd25f9` | geom/vertices.rs | Fix clippy: `///` → `//!` for module doc |

### Verification

- `cargo test`: **all pass** (107 tests, 6 ignored)
- `cargo clippy`: **clean** (only pre-existing `dead_code` warning for `check_bounded_bugs`)
- `ruff check`: **no new warnings** (pre-existing: unused `numpy` import in generate_figures.py)
- `latexmk`: **same as main** (skeleton thesis has unresolved forward refs — pre-existing)

### Review marker changes

- **Deleted**: `% Jörn: text approved (a013c1e) — entire file` in correspondence.tex (staleness rule: PDF-visible text changed — column header "Ours" → "This thesis", expanded MATLAB convention, added dash explanation)
- **No other `% Jörn:` markers affected** — all other edited .tex files had no approval markers

## Fixes Applied (Tier 2)

**Commits**: 4 additional commits after Tier 1, with Jörn's input on decisions.

| # | Commit | Fix |
|---|--------|-----|
| 20 | `e2a1701` | Rename `triangle_product` → `lagrangian_triangle_product`, add `symplectic_triangle_product` (×_S, capacity = 3√3/4 by Moser). Fix misleading "(q1, p1)" comments → "q-space (q₁, q₂)". |
| 21 | `6e83701` | Remove thesis/CLAUDE.md Content Rule 1 ("Do NOT write anything Jörn did not dictate") — per Jörn's instruction. |
| 22 | `db3071e` | Add staleness warning to knowledge-dump.md, redirect to thesis LaTeX as authoritative source. |
| 23 | `4f303b4` | Replace all numbered thesis cross-refs with label-based syntax (`def:polytope`, `alg:ehz`, etc.). |

### Verification (post Tier 2)

- `cargo test`: **all pass** (109 tests, 6 ignored — +2 tests from new `symplectic_triangle_product` fixture)
- `cargo clippy`: **clean**

## Remaining Items (Tier 3 — deferred)

1. **Thesis proof gaps** (simple-minimizer-proof.tex): Two critical findings block verification. Being addressed in a separate session.
2. **Section 4 empty skeletons** (chapter-algorithm.tex): Load-bearing definitions missing. Being addressed in a separate session.
