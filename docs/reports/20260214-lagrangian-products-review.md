# Review: Lagrangian products experiment infrastructure + validation refactoring

**Branch:** `claude/lagrangian-products-experiment` at `/workspaces/worktrees/lagrangian-products`
**Base:** local `main` at `d9ed62a` (merge-base: `a47930b`)
**Date:** 2026-02-14

## Build Verification

- **Tests:** All pass. 113 geom, 29 datasets, 25 hk2017 (debug, ~170s total)
- **Clippy:** Clean (0 warnings on library code; 1 pre-existing `dead_code` warning in test code for `check_bounded_bugs` field)
- **Working tree:** Clean
- **Note:** `comprehensive_volume_cross_check` ran for ~125s in debug because this branch predates `d9ed62a` (which marks it `#[ignore]`). After rebasing, geom tests will drop from 125s to ~2s.

## Branch Structure

9 branch-specific commits after merge-base `a47930b`:

| # | Commit | Summary |
|---|--------|---------|
| 1 | `e82736e` | Core infrastructure: polygon.rs, lagrangian_product.rs, sweep binary, Python script |
| 2 | `2ebf3a8` | LaTeX experiment section skeleton with lemmas |
| 3 | `6de7add` | LaTeX findings: pentagon sweep + polygon grid data |
| 4 | `c440edb` | LaTeX findings: random products results |
| 5 | `0689c8d` | Refactor: move boundedness/irredundancy into Polytope4D::new() |
| 6 | `760c853` | Regression tests for all Polytope4D error paths |
| 7 | `e0df08e` | Error-path tests for polygon, lagrangian_product, qhull |
| 8 | `a2d4812` | Resolve LaTeX TODOs from Jörn's review |
| 9 | `362bfd8` | Polish LaTeX, Rust, Python |

Commits are atomic, descriptive, and all include Co-Authored-By. Good commit quality.

**Rebase needed:** 14 commits on main since merge-base. No functional conflicts expected (branch adds new files/modules), but the `#[ignore]` fix on `comprehensive_volume_cross_check` needs to be picked up.

## Deletion Verification

| Deleted Code | LOC | Purpose | Replacement | Verdict |
|---|---|---|---|---|
| datasets/validation.rs body | ~175 | check_bounded(), kernel_of_three(), check_irredundant(), affine_rank() | geom/validation.rs (121 LOC) + Polytope4D::new() constructor checks | ✓ Same algorithm, cleaner location |
| datasets/validation_test.rs helpers | ~30 | make_polytope(), check_irredundant tests | Direct Polytope4D::new() calls; geom/validation_test.rs covers the functions | ✓ |
| hk2017 fixture (capacity_dataset.json) | 441+/441- | Random polytope test data | Regenerated with stricter constructor (rejects redundant facets) | ✓ Documented in commit message |

The refactoring is clean: validation logic moved from a downstream crate (`datasets`) into the type's constructor (`Polytope4D::new()` in `geom`). This is better architecture — invariants are enforced at the point of construction.

## Code Review

### Rust — New geom modules (1096 LOC added)

**polygon.rs (126 LOC):** Clean 2D polygon constructors.
- `regular_polygon_2d`: Correct. Heights = inradius = R·cos(π/n). Angle convention (π/2 + 2πk/n) documented and tested against HK-O pentagon.
- `rotate_polygon_2d`: Correct. Heights unchanged (rotation preserves distance from origin). Standard 2D rotation matrix.
- `random_polygon_2d`: Angles sorted, heights in [h_min, h_max]. Panic conditions well-documented and tested.
- `polygon_area`: Shoelace formula on consecutive-halfplane intersections. Returns None for degenerate cases. ✓

**lagrangian_product.rs (42 LOC):** Minimal and correct. Embeds P normals as (n_P, 0, 0, 0) and Q normals as (0, 0, n_Q) in (q₁, q₂, p₁, p₂) coordinates. Heights concatenated. Delegates to Polytope4D::new().

**validation.rs (121 LOC):** `check_bounded` and `find_redundant_facet` moved from datasets — same algorithm.

**polytope.rs changes (+22/-5):** Added `Unbounded` and `RedundantFacet` variants to `ConstructionError`. Constructor now calls `check_bounded` and `find_redundant_facet`. Clean.

**qhull.rs (90 LOC added):** `parse_fp_output` and `parse_fa_output` parsers with comprehensive tests (12 test cases covering all error paths).

**Test coverage:** 72 new tests across polygon (26), lagrangian_product (9), polytope error paths (13), validation (7), qhull parsing (17). Thorough.

### Rust — Sweep binary (311 LOC)

`lagrangian_sweep.rs`: Three subcommands + `all`. Configuration constants well-documented.

- **pentagon-sweep**: 361 points over [0°, 90°]. Correct.
- **polygon-grid**: Adaptive resolution by facet count. Fundamental domain computed via lcm. Correct.
- **random-products**: 500 samples, deterministic seed. Gracefully skips failures. Uses `f64::NAN` for `angle_deg` which serializes as JSON `null` (verified: serde_json 1.x maps NaN→null). Works but `Option<f64>` with `#[serde(skip_serializing_if = "Option::is_none")]` would be more idiomatic.

### Python (230 LOC)

Clean matplotlib plotting with deterministic RNG for jitter (`np.random.default_rng(42)`). Loads individual JSONL files or splits from combined "all" file. Summary stats printed to stderr. No issues.

### LaTeX (281 LOC)

Well-structured experiment section:

- **Definition** of Lagrangian product: correct.
- **Fundamental domain lemma**: Proof uses symplecticity of block-diagonal rotations and mirror symmetry. Correct.
- **Results table**: 10 pairs, values consistent with commit messages.
- **Findings**: 5 numbered findings with quantitative backing.

**Minor issues:**
1. Line 110: `% TODO: add \ref to algorithm section once it exists` — dangling TODO.
2. Citations are inline text ("Balitskiy (2020)", "Chaidez--Hutchings (2021)") rather than `\cite{}`. Acceptable if bibliography not yet set up.

### Mathematical Correctness

Spot-checked key claims:
- vol(P ×_L Q) = area(P)·area(Q): Fubini on complementary Lagrangian subspaces. ✓
- (4,4) at 45°: c = 2√2, vol = 4, sys = 8/8 = 1. ✓
- (3,6) at 0°: c = 3√3/2, vol = 27/8, sys = (27/4)/(27/4) = 1. ✓
- Fundamental domain [0, π/lcm(n,m)]: Periodicity from n,m-fold symmetries, halved by mirror symmetry. ✓

### Pipeline Consistency

- Rust binary outputs JSONL → Python reads JSONL → figures → LaTeX references figures. ✓
- Column names match: `angle_deg`, `sys`, `capacity`, `facet_count`, `n1`, `n2` used consistently. ✓
- LaTeX table values match commit-message summaries. ✓

## Strengths

1. **Architectural improvement**: Moving validation into `Polytope4D::new()` is the right call — invariants enforced at construction, impossible to have an invalid `Polytope4D`.
2. **Excellent test coverage**: Every error path tested. Mathematical properties tested (volume = product of areas, rotation preserves area/volume, HK-O convention match).
3. **Clean pipeline**: Rust binary → JSONL → Python → figures → LaTeX. Reproducible with deterministic seeds.
4. **Good LaTeX**: Clear mathematical setup, lemma with proof, concrete findings backed by data.
5. **Commit quality**: Atomic, descriptive, all co-authored.

## Issues

| # | Severity | Issue | Suggested Fix |
|---|----------|-------|---------------|
| 1 | **Moderate** | Branch 14 commits behind main. `comprehensive_volume_cross_check` not `#[ignore]`, adding ~125s to test suite. | Rebase onto main |
| 2 | Minor | LaTeX TODO line 110: `% TODO: add \ref to algorithm section` | Remove or replace with forward reference |
| 3 | Minor | `f64::NAN` for `angle_deg` in random products — works (→JSON null) but non-idiomatic | `Option<f64>` with serde skip |
| 4 | Minor | `dead_code` warning for `check_bounded_bugs` in qhull_boundedness_test.rs | Prefix with `_` or `#[allow(dead_code)]` |

No correctness issues found. No pre-existing issues on main relevant to this branch.

## Executive Summary

**Summary of findings:**
1. Branch needs rebasing onto main (14 commits behind) — this will fix the slow test issue automatically
2. No correctness issues. Mathematical claims, code, and pipeline are all consistent
3. Clean architectural refactoring (validation into constructor) + comprehensive new experiment infrastructure
4. Minor: one dangling LaTeX TODO, non-idiomatic NaN sentinel

**Recommendation:** Rebase onto main, then merge. The rebase is the only blocking item — it picks up the `#[ignore]` fix and prevents merge conflicts from accumulating. The two minor LaTeX/Rust style issues can be fixed opportunistically.

**Time investment:** ~60min review
