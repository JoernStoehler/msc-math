# Review: Billiard algorithm for Lagrangian products

**Branch:** `claude/billiard-algorithm` at `/workspaces/worktrees/billiard-algorithm`
**Base:** local `main` at `d9ed62a`
**Date:** 2026-02-14

---

## 1. Build Verification

| Check | Status | Evidence |
|-------|--------|----------|
| `cargo test` (debug) | PASS | 6 billiard tests pass; 115 total across all crates; 2.0s billiard, 44s total |
| `cargo test --release -p billiard -- --ignored` | PASS | 8 tests pass in 1.1s (includes hk2017 agreement + pentagon) |
| `cargo clippy` | CLEAN | Zero warnings |
| `latexmk` (thesis) | PASS | Compiles; 7 undefined citations (new .bib entries needed, see §5) |
| Working tree | CLEAN | No uncommitted changes |
| Commit quality | GOOD | 7 atomic commits with descriptive messages, co-authored-by present |

## 2. What Changed (1429 LOC added)

New billiard crate implementation + thesis chapter:

| File | LOC | Purpose |
|------|-----|---------|
| `billiard/src/lib.rs` | +152 | Entry point `billiard_capacity()`, error types, result struct, adjacency matrix |
| `billiard/src/kkt.rs` | +99 | KKT solver (intentional copy from hk2017) |
| `billiard/src/enumerate.rs` | +204 | Block enumeration with lazy iteration + cyclic symmetry removal |
| `billiard/src/lagrangian.rs` | +67 | Facet classification into q-type/p-type |
| `billiard/src/lib_test.rs` | +225 | 14 tests: agreement, error handling, result properties |
| `billiard/src/bench_kkt.rs` | +211 | KKT solver variant profiling (LU vs SVD) |
| `thesis/chapter-billiard.tex` | +462 | Section 6: Lagrangian products, orbit structure, billiard characterization, algorithm |
| `crates/Cargo.toml` | +7 | nalgebra opt-level=3 for dev/test profiles |
| `thesis/main.tex` | +2 | `\input{chapter-billiard}` |

## 3. Code Quality

### KKT solver (kkt.rs)
Compared against hk2017 KKT solver: **functionally identical**. Same epsilon constants, same matrix assembly, same LU+SVD fallback strategy, same Q(β) computation. One documentation gap: hk2017 has a useful comment explaining why Q(β) uses ω₀ directly (not H) due to sign inversion. The billiard copy omits this. No correctness concern.

### Enumeration (enumerate.rs)
- Block definition (Single/Pair) correctly models the [Q|QQ] pattern from Lemma 6.5
- Both orderings of pair blocks are generated (covers within-block permutations)
- Non-overlapping selection ensures no facet reuse
- Cyclic symmetry removal is correct: fixing first q-block position divides count by k
- Heap's algorithm for permutations is standard and correct
- All closures and buffers correctly sized (no off-by-one risk)
- Lazy evaluation: no intermediate Vec collections, constant memory per recursion depth

### Lagrangian classification (lagrangian.rs)
- Classifies normals by checking which 2D subspace they lie in (q: components [0,1], p: components [2,3])
- Tolerance `EPS_LAGRANGIAN_NORMAL = 1e-10` for squared norm — reasonable
- Error handling: rejects mixed normals and < 3 facets per type

### Main entry point (lib.rs)
- Clean pipeline: classify → adjacency → enumerate blocks → solve KKT → minimize
- `billiard_capacity` returns `Result<Option<BilliardResult>, BilliardError>`
- `Ok(None)` case documented as defensive guard
- `build_adjacency_matrix` uses vertex-facet incidence with `EPS_FACET_INCIDENCE = 1e-8`

### Workspace-level change (Cargo.toml)
nalgebra `opt-level = 3` in dev/test profiles: good trade-off, measured 1.4x speedup. Affects only nalgebra internals; crate-level `debug_assert!` checks preserved.

## 4. Test Coverage

### Debug suite (6 tests, 2s)
| Test | What it covers |
|------|---------------|
| `hypercube_capacity` | Agreement with known value (4.0) |
| `triangle_product_capacity` | Agreement with known value (1.5) |
| `triangle_square_capacity` | Agreement with known value (1.5) |
| `rejects_non_lagrangian_product` | Error path: simplex has mixed normals |
| `rejects_symplectic_triangle_product` | Error path: symplectic product normals not in L_q/L_p |
| `result_properties` | Structural checks on all fast polytopes (bounce_count, beta positivity, perm length) |

### Release suite (8 tests, 1.1s)
| Test | What it covers |
|------|---------------|
| `hko_pentagon_capacity` | 10-facet pentagon against closed-form value |
| `agrees_with_hk2017_*` (×4) | Cross-algorithm agreement for all Lagrangian test polytopes |
| `billiard_iterations_polynomial` | Iteration count < 1M for pentagon (polynomial bound check) |
| `result_properties_pentagon` | Structural checks on pentagon result |
| `bench_kkt_lu_vs_svd` | LU vs SVD performance profiling |

### Test strategy assessment
The debug/release split is well-reasoned. Commit messages document the rationale: "No debug-mode value — no debug_assert!, no integer overflow on floats. Small-polytope tests cover all debug-relevant code paths (index arithmetic, block enumeration, error handling)."

The billiard crate has **zero debug_assert!** calls, which is accurate context for the test split. The crate does all its work with f64 arithmetic and nalgebra calls — no index arithmetic or overflow-prone operations benefit from debug mode.

**Note for Jörn:** No direct unit tests for `enumerate_blocks` or `enumerate_k_bounce_sigmas`. Enumeration correctness is tested only indirectly via cross-algorithm agreement with hk2017. This is adequate given the agreement passes for all 4 test polytopes, but direct enumeration count tests would add confidence.

## 5. LaTeX Chapter (chapter-billiard.tex)

### Cross-references
All 14 cross-references to chapter-algorithm.tex definitions/lemmas/theorems are valid and correctly used.

### Math verification
`% Jörn: math approved (036b845)` covers the entire file. Commit `e1345bb` says "Remove TODO and GAP markers after Joern confirmed all proofs correct." The hash `036b845` is the pre-amend version of `e1345bb` (found in reflog); content is the same. Staleness mechanism works via reflog.

### Bibliography entries
Three new citations not in .bib: `AAO2014`, `Rudolf2022`, `BezdekBezdek2009`. These are real references that need to be added. Pre-existing `hk2017`/`HK2017` citations also undefined (from chapter-algorithm.tex, not this branch).

### Structure
Chapter 6 placed correctly: after main algorithm (Section 5), before experiments. Logical flow: Lagrangian products → orbit structure → billiard characterization → algorithm.

### Code-LaTeX correspondence
| LaTeX concept | Rust implementation | Match? |
|---------------|-------------------|--------|
| Lemma 6.1: facet classification | `lagrangian.rs::classify_facets` | ✓ |
| Lemma 6.5: sigma structure ([Q\|QQ][P\|PP])^k | `enumerate.rs::enumerate_k_bounce_sigmas` | ✓ |
| Lemma 6.6: k=1 infeasibility | k starts at 2 in `billiard_capacity` | ✓ |
| Theorem 6.10: k ≤ 3 bound | k goes up to 3 in `billiard_capacity` | ✓ |
| Remark 6.11: O(n_q³ n_p³) complexity | Cyclic symmetry removal matches description | ✓ |
| Algorithm: KKT system (eq:linear-system) | `kkt.rs::solve_kkt` | ✓ |
| Algorithm: action = ½/Q(β) | `lib.rs:113: action = 0.5 / q_val` | ✓ |

## 6. Strengths

1. **Cross-algorithm agreement verified** for all 4 Lagrangian test polytopes (hypercube, triangle×triangle, triangle×square, pentagon)
2. **Clean module separation**: kkt, enumerate, lagrangian, lib each have single responsibilities
3. **Performance work is measured**: commit messages include before/after timings (21s→15s, 537ms→185ms, 144k→50.4k sigmas)
4. **Math verified by Jörn**: entire chapter has `% Jörn: math approved` marker
5. **Correct cyclic symmetry removal**: reduces sigma count by factor k without missing orbits
6. **Good defensive programming**: error types for non-Lagrangian inputs, tolerance constants named and documented

## 7. Issues

| # | Severity | Finding | Suggested fix |
|---|----------|---------|---------------|
| 1 | LOW | Approval marker hash `036b845` is pre-amend (now `e1345bb`). Works via reflog but not via `git log`. | Update marker to `e1345bb` if desired for consistency; or leave as-is since reflog resolves it |
| 2 | LOW | Missing .bib entries: `AAO2014`, `Rudolf2022`, `BezdekBezdek2009` | Add to thesis bibliography (can be done later) |
| 3 | LOW | Q(β) sign clarification comment from hk2017 missing in billiard kkt.rs | Add 3-line comment explaining ω₀ vs H sign relationship |
| 4 | NEGLIGIBLE | `build_adjacency_matrix` duplicated in bench_kkt.rs with hardcoded `1e-8` instead of `EPS_FACET_INCIDENCE` | Use the constant; or leave as-is since bench_kkt is profiling-only |
| 5 | NOTE | No `debug_assert!` in billiard crate | Intentional per test strategy. Could add e.g. symmetry check on adjacency matrix, but low value |

## 8. Pre-existing Issues

- `hk2017`/`HK2017` bibliography entries also undefined (from chapter-algorithm.tex, predates this branch)

---

## Executive Summary

**Summary of findings:**
1. All tests pass (debug + release), clippy clean, thesis compiles. Cross-algorithm agreement with hk2017 verified for all Lagrangian test polytopes.
2. Code quality is high: clean module separation, correct enumeration logic, well-documented performance optimizations with measurements.
3. LaTeX chapter math is verified by Jörn. All cross-references valid. Three bibliography entries need adding.
4. No correctness issues found. All findings are LOW/NEGLIGIBLE severity (documentation, bib entries).

**Recommendation:** Merge. The .bib entries (finding #2) can be added before or after merge — they don't block correctness.

**Time investment:** ~45min review
