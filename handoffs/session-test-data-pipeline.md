# Session: Test Data Pipeline Restructuring

**Goal:** Default test suite < 2 min (currently ~7 min). Full suite (with `--ignored`) < 10 min.

**Worktree:** Yes. Branch from local `main`.

## Context

Default test suite takes 7 min because fixture-consuming tests regenerate all 33 polytope capacity values every run. The fix: write capacity fixtures to a checked-in JSON file, tests load from JSON instead of recomputing.

## Profiling data (cargo nextest, 2026-03-17)

| Test | Time | Fix |
|------|------|-----|
| `catalog_determinism` | 162s | Load from JSON |
| `fixture_staleness_check` | 158s | Load from JSON |
| `literature_capacity_values` | 98s | Load from JSON |
| `volume_scales_with_fourth_power` | 92s | Fewer proptest cases in default |
| 5 fixture-consuming tests | 85-89s ea | Load from JSON |
| `random_polytopes_pass_validation` | 46s | Fewer proptest cases in default |

## Work items

1. `generate_capacity_fixtures` writes to `fixtures/capacity_fixtures.json` (on-disk, checked in)
2. 7 fixture-consuming tests load JSON instead of regenerating (85-98s → <1s each)
3. Staleness detection: semantic (expected values) + generator hash (WARNING not ERROR)
4. `known_polytopes` constructors → `LazyLock` caching (~50ms × 30 tests)
5. Proptest: fewer cases in default, full cases in `#[ignore]`

## Skills to load

- `data-pipeline` — caching strategies, fixture generation, staleness detection
- `rust-conventions` — coding style
- `rust-tests` — testing philosophy, fixtures, test organization

## Invariant

`cargo test --lib` must pass with zero failures before and after.

## Deliverable

- All changes committed on the worktree branch
- Report: before/after timing comparison
- Any issues or surprises noted
