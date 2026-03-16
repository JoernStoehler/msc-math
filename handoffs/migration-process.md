# Migration Process

How to get from the current codebase to the target state defined in `migration-target.md`.

---

## Approach

Each subagent reads old code for mathematical knowledge, then writes fresh code to the target spec. No copy-pasting.

**Branch state:** The migration branch has the target file structure already in place — all target `.rs` files exist as empty stubs, all `mod.rs` files have correct `pub mod` + `#[cfg(test)] #[path = "..."]` declarations, and `lib.rs` has all `pub mod` declarations (no re-exports yet — #16 adds those in wave 4). Old files at renamed paths still exist on disk but are undeclared (harmless).

**Reading old code:** Subagents read old code via `git show main:crates/src/{path}`. The old code on `main` is the read-only reference.

**Writing new code:** Subagents overwrite the empty stubs at target paths. No `mod.rs` editing needed — all declarations are already in place.

**Workspace:** Wave 1 branches from the scaffold commit. Wave 2 branches from wave 1's merged result. Etc. Old orphaned files are cleaned up after all waves complete.

Each subagent gets:
1. Both `migration-target.md` and `migration-process.md` (full context)
2. Old code via `git show main:crates/src/{path}` for files listed in "reads"
3. For wave 2+: the new code from previous waves (already on their branch)

---

## Subagent assignments

### Wave 1 — no dependencies (start immediately, in parallel)

| # | Scope | Reads (old code) | Writes (new files) |
|---|-------|-------------------|--------------------|
| 1 | `kkt/mod.rs` + `kkt/qp_assembly.rs` | `kkt/mod.rs`, `kkt/augmented.rs` | `kkt/mod.rs` (types: QP, Solution, Verdict, classify_margin), `kkt/qp_assembly.rs`, `kkt/qp_assembly_test.rs` |
| 5 | `algorithms/facet_adjacency.rs` | `algorithms/hk2017/mod.rs` lines 200-254 | `algorithms/facet_adjacency.rs`, `algorithms/facet_adjacency_test.rs` |
| 8a | `geom/` core types | All `geom/*.rs` source and test files (read for mathematical knowledge) | `geom/polytope.rs`, `geom/polytope_test.rs`, `geom/symplectic_form.rs`, `geom/symplectic_form_test.rs`, `geom/volume.rs`, `geom/volume_test.rs`, `geom/volume_properties_test.rs`, `geom/polygon.rs`, `geom/polygon_test.rs`, `geom/lagrangian_product.rs`, `geom/lagrangian_product_test.rs`, `geom/cross_product_4d.rs`, `geom/cross_product_4d_test.rs` |
| 8b | `geom/` infrastructure + top-level files | `geom/validation.rs`, `geom/rational.rs`, `geom/vertex_enumeration.rs`, `geom/qhull.rs`, `geom/known_polytopes.rs`, `geom/test_utils.rs`, `constants.rs`, `dataset.rs`, `random.rs` + their test files | `geom/validation.rs`, `geom/validation_test.rs`, `geom/rational_arithmetic.rs`, `geom/rational_arithmetic_test.rs`, `geom/vertex_enumeration.rs`, `geom/qhull.rs`, `geom/known_polytopes.rs`, `geom/test_utils.rs`, `constants.rs`, `dataset.rs`, `dataset_test.rs`, `random.rs`, `random_test.rs` |
| 9 | `geom/reeb_trajectory.rs` + `geom/skeleton.rs` + tests | `geom/reeb_trajectory.rs`, `geom/skeleton.rs`, `geom/reeb_trajectory_test.rs`, `geom/skeleton_test.rs` | `geom/reeb_trajectory.rs`, `geom/reeb_trajectory_test.rs`, `geom/skeleton.rs`, `geom/skeleton_test.rs` |
| 13 | Experiment READMEs | All 16 `experiments/*/README.md` + experiment source code (to extract findings) | Standardized READMEs per template. Focus on sparse ones: random-sweep, pentagon-perturb, unknown-predicates, gradient-descent |
| 15 | Meta-layer cleanup | `.claude/skills/*/SKILL.md`, `CLAUDE.md` | Fixed contradictions (4), dropped archaeology skill, dropped plain-% category, added `thesis/lookup.sh` |

### Wave 2 — depends on wave 1

| # | Scope | Reads (old code) | Writes (new files) | Depends on |
|---|-------|-------------------|--------------------|------------|
| 2 | `kkt/saddle_point_solver.rs` + `constraint_solver.rs` + `beta_feasibility.rs` + `projection_solver.rs` + tests | `kkt/augmented.rs`, `kkt/constraint_solver.rs`, `kkt/margin_search.rs`, `kkt/projection_solver.rs` | `kkt/saddle_point_solver.rs`, `kkt/saddle_point_solver_test.rs`, `kkt/constraint_solver.rs`, `kkt/constraint_solver_test.rs`, `kkt/beta_feasibility.rs`, `kkt/beta_feasibility_test.rs`, `kkt/projection_solver.rs`, `kkt/projection_solver_test.rs` | #1 |
| 3 | `kkt/rational_solver.rs` + test | `kkt_rational.rs` (578 lines), `kkt_rational_test.rs` | `kkt/rational_solver.rs`, `kkt/rational_solver_test.rs` | #1 |
| 4 | `algorithms/capacity_accumulator.rs` + test | `hk2017/mod.rs` lines 88-170, `billiard/mod.rs` lines 98-177 | `algorithms/capacity_accumulator.rs`, `algorithms/capacity_accumulator_test.rs` | #1 |
| 12 | Test splitting (geom) | `geom/vertex_enumeration_test.rs` (429 lines) | `geom/vertex_enumeration_test.rs`, `geom/vertex_enumeration_linalg_test.rs`, `geom/construction_validation_test.rs` | #8b |
| 14 | Experiment import updates | `experiments/gradient-descent/kkt_instrumented.rs`, `experiments/sys-optimization/sys_optimization.rs`, `experiments/hko-neighborhood/hko_neighborhood.rs`, `experiments/omega-obstacle/omega_obstacle.rs`, `experiments/kkt-inertia/kkt_inertia.rs`, `experiments/q-error/q_error.rs`, `experiments/visualization/viz_export.rs` | Update all experiment binaries that import from renamed modules (e.g. `kkt::augmented` → `kkt::saddle_point_solver`, `kkt::augmented::build_kkt_system` → `kkt::qp_assembly::build_augmented_system`). The 4 experiments with duplicated code also delete ~1500 lines of inline code and replace with library imports. | #1, #5 |

**Note on wave 2 sequencing:** #2 and #3 are independent (no shared files — mod.rs declarations are pre-written). All wave 2 subagents can run in parallel.

### Wave 3 — depends on accumulator (#4) and adjacency (#5)

| # | Scope | Reads (old code) | Writes (new files) | Depends on |
|---|-------|-------------------|--------------------|------------|
| 6 | `algorithms/hk2017/` | `hk2017/mod.rs`, `hk2017/permutations.rs`, `hk2017/test_dataset.rs` | `algorithms/hk2017/mod.rs`, `algorithms/hk2017/permutations.rs`, `algorithms/hk2017/permutations_test.rs`, `algorithms/hk2017/generate_capacity_fixtures.rs` | #1, #4, #5 |
| 7 | `algorithms/billiard/` | `billiard/mod.rs`, `billiard/enumerate.rs`, `billiard/lagrangian.rs`, `billiard/bench_kkt.rs` | `algorithms/billiard/mod.rs`, `algorithms/billiard/capacity_test.rs`, `algorithms/billiard/block_enumeration.rs`, `algorithms/billiard/facet_classification.rs`, `algorithms/billiard/kkt_benchmark.rs` | #1, #4, #5 |

### Wave 4 — depends on wave 3

| # | Scope | Reads (old code) | Writes (new files) | Depends on |
|---|-------|-------------------|--------------------|------------|
| 10 | `hk2017/orbit_recovery.rs` + test | `hk2017/recover.rs`, `hk2017/recover_test.rs` | `algorithms/hk2017/orbit_recovery.rs`, `algorithms/hk2017/orbit_recovery_test.rs` | #6 |
| 11 | Test splitting (hk2017) | `hk2017/hk2017_test.rs` (707 lines), `hk2017/sensitivity_test.rs` (360 lines), `hk2017/capacity_properties_test.rs` (411 lines) | `algorithms/hk2017/literature_test.rs`, `algorithms/hk2017/kkt_edge_cases_test.rs`, `algorithms/hk2017/pruning_test.rs`, `algorithms/hk2017/regression_test.rs`, `algorithms/hk2017/conformality_test.rs`, `algorithms/hk2017/symplectic_invariance_test.rs`, `algorithms/hk2017/capacity_derivative_test.rs` | #6 |
| 16 | `lib.rs` (final) | Current `lib.rs` | Final `lib.rs` with all re-exports per migration-target.md spec | #6, #7 |
| 18 | `algorithms/tube/` | `tube/mod.rs`, `tube/tube_test.rs` | `algorithms/tube/mod.rs`, `algorithms/tube/capacity_test.rs` | — |

### Cleanup — after all waves

| # | Scope | Action |
|---|-------|--------|
| 17 | Delete orphaned old files | Remove all old files that are no longer declared in the module tree: `kkt_rational.rs`, `kkt_rational_test.rs`, `algorithms/hk2017/square_product_diagnostic.rs`, `geom/lib_test.rs`, `geom/cross_product.rs`, `geom/cross_product_test.rs`, `geom/symplectic.rs`, `geom/symplectic_test.rs`, `geom/rational.rs`, `geom/rational_test.rs`, `kkt/augmented.rs`, `kkt/margin_search.rs`, `algorithms/hk2017/hk2017_test.rs`, `algorithms/hk2017/sensitivity_test.rs`, `algorithms/hk2017/capacity_properties_test.rs`, `algorithms/hk2017/recover.rs`, `algorithms/hk2017/recover_test.rs`, `algorithms/hk2017/test_dataset.rs`, `algorithms/billiard/enumerate.rs`, `algorithms/billiard/lagrangian.rs`, `algorithms/billiard/bench_kkt.rs` |

---

## Dependency graph

```
Wave 1 (parallel):  1    5    8a  8b  9    13   15
                    │    │    │   │
Wave 2 (parallel):  2  3 4    12  14
                    │    │
Wave 3 (parallel):  6    7
                    │
Wave 4 (parallel):  10  11  16  18

Cleanup:            17
```

All subagents within a wave run in parallel (no shared file edits — mod.rs is pre-written).

---

## Per-subagent instructions template

Each subagent receives a prompt like:

```
You are rewriting module {X} for the symplectic crate.

Read both migration-target.md and migration-process.md for full context.
Your assignment is subagent #{N}.

## Old code to read (for mathematical knowledge)
{file paths — read via `git show main:crates/src/{path}`}

## What to write
{output file paths — overwrite the empty stubs already at these paths}

## Tests that must pass after this wave
cargo test --lib -p symplectic
(Tests run at the wave gate after ALL subagents in the wave complete, not after your work alone.)

## Rules
- Read old code via `git show main:crates/src/{path}`. Write fresh code to the spec.
- Overwrite the empty stubs at target paths. Do NOT edit mod.rs or lib.rs — declarations are pre-written.
- Every pub item gets a doc comment (see documentation strategy in migration-target.md).
- Every file gets a header (see file header template in migration-target.md).
- Do not copy-paste old code. Understand it, then write the new version.
- If you discover something the spec doesn't cover, write a TODO comment and continue.
- If a test references a module that doesn't exist yet (written by a later wave), mark the test `#[ignore]` with a comment naming the dependency. The later wave's subagent will un-ignore it.
```

---

## Verification

Two gates after each wave: automated (tests) and review (quality).

### Gate 1: Automated

```bash
cd crates/ && cargo test --lib
cd crates/ && cargo clippy --lib -- -D warnings
```

After all waves additionally:

```bash
cd experiments/ && cargo build
ruff check experiments/
cd thesis/ && latexmk
```

If any test fails: the subagent that owns the failing module investigates.

### Gate 2: Review

After each wave, spawn review subagents to check what the automated gate can't:

- **File headers**: Does every .rs file start with the header template from migration-target.md?
- **Doc comments**: Does every pub item have a doc comment with mathematical correspondence?
- **Progressive disclosure**: Does each mod.rs list every file with a one-line description?
- **Naming**: Do file names match the module tree in migration-target.md?
- **No copy-paste**: Is the code freshly written or does it have obvious artifacts from the old code (old comments, old variable names, old TODOs)?
- **Mathematical fidelity**: For solver/algorithm modules, does the new code implement the same mathematical algorithm as the old code? Tests verify behavior on tested inputs only — they don't catch subtle algorithmic changes that happen to pass the test suite.
- **Test headers**: Does every test file have the proposition/strategy header?

Review findings go back to the responsible subagent for fixes before the next wave starts. This is the feedback loop that catches quality issues the test suite can't.

---

## Risk mitigations

1. **Interface mismatches:** Wave ordering ensures downstream subagents read upstream output. Within a wave, parallel subagents sharing types (e.g. #1 and #8a both use Polytope4D) read the same old code on `main`, so they agree on the API. If a subagent changes an interface, the compile gate catches the mismatch.

2. **Mathematical bugs:** Tests catch regressions on tested inputs but cannot prove general correctness. The Gate 2 review must also check that the new code implements the same mathematical algorithm as the old code — not just that it passes the same tests. For critical modules (saddle_point_solver, capacity_accumulator, orbit_recovery), Jörn reviews the mathematical logic.

3. **Scope creep:** Each subagent has explicit file lists. Out-of-scope discoveries → TODO comment.

4. **lib.rs / mod.rs:** Pre-written in the scaffold. No subagent edits these. #16 rewrites `lib.rs` with re-exports in wave 4.

5. **Duplicate test names:** Subagents that split test files (#11, #12) must delete the old monolithic test file.

---

## Wave 1-2 execution log

Deviations from the plan, discovered during execution. Subagents for waves 3+ should read this.

### Agent scope limits

**#8a stalled after writing polytope.rs (17KB).** 13 files was too much for one agent. Redispatched remaining 10 files as a separate agent. **Lesson for waves 3+: keep agents to ≤8 files.**

### Behavioral changes discovered by review

1. **`saddle_point_solver.rs:302-304` — constraint verification logic changed.** Old code (`augmented.rs`) used `||` (reject if EITHER residual exceeds threshold). New code uses `&&` (reject only if BOTH exceed). This is more permissive. **Status: needs fix before wave 3.**

2. **`rational_solver.rs:78-81` — beta positivity check added.** Old code returned `Some(result)` even with non-positive beta components (caller filtered). New code returns `None` if any beta ≤ 0. This is a deliberate behavioral improvement but changes the API contract. **Status: document as intentional, verify callers handle None.**

### Visibility fixes applied

- `geom/rational_arithmetic.rs`: `omega0_rational` and `rational_to_f64` changed from `pub(super)` to `pub(crate)`. Needed for cross-module access from `kkt/rational_solver.rs`.
- `kkt/rational_solver.rs` still has local copies of these functions (stale after the visibility fix). **Status: needs cleanup — delete local copies, import from `crate::geom::rational_arithmetic`.**

### Experiment import updates (#14)

- Updated 18 `.rs` files under `experiments/` to new module paths.
- Added local function copies for `build_kkt_system` and `q_from_beta` in 2 experiments where signatures changed. These are interim — should be cleaned up after wave 4 when library APIs stabilize.
- 4 binaries compile; 14 fail on unwritten wave 3+ modules (expected).

### Additional rename: `reeb_vector` → `reeb_direction`

Agent #9 renamed `reeb_vector()` to `reeb_direction()` — not in the original spec but a reasonable improvement (it returns a direction, not the full vector field R_i = (2/h_i)J₀n_i). Updated in experiments by #14.

### Wave 3 concern: can #6 and #7 run in parallel?

Billiard is conceptually hk2017 with custom enumeration. In the old code, billiard imports adjacency functions from hk2017 — but those are now in `facet_adjacency` (wave 1). The remaining question: does billiard need anything else from hk2017/mod.rs (e.g. permutation utilities)? If so, #7 depends on #6 and they can't be parallel. **To investigate before dispatching wave 3.**
