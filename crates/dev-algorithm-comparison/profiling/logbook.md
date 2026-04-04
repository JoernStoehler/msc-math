# Test Suite Profiling: Logbook

## Motivation

Profile the default test suite (`cargo test --lib`) to identify hot paths and track performance over time. Provides data for decisions about `#[ignore]` annotations and optimization targets.

## Status

**Active** — run after any change to `Polytope4D::new()`, test fixtures, or the capacity algorithm.

## How to run

```bash
cd crates/dev-algorithm-comparison/profiling/
uv run analyze.py
```

This runs the full pipeline:
1. Run `cargo test --lib` with wall-clock and CPU timing (parses default text output)
2. Write per-test results to `profile.jsonl`
3. Append a summary to `logbook.jsonl` (date, commit, wall time, CPU time, core count, top-5 slowest)
4. Generate `test_timing.png` (bar chart of top 15 slowest tests)

## Files

| File | Role |
|------|------|
| `analyze.py` | Run profiling, collect per-test timings, generate figure |
| `profile.jsonl` | Per-test timing data (one line per test, sorted by duration descending) |
| `logbook.jsonl` | Historical runs (one line per profiling session) |
| `test_timing.png` | Figure: bar chart of slowest tests (generated, not committed) |

## Design

### Full suite timing

Runs `cargo test --lib` wrapped in `bash -c "time ..."` to capture both wall-clock and CPU time. Parses stdout for the "test result:" summary line (passed/ignored counts) and stderr for the bash `time` user-CPU output.

### Per-test profiling

Profiles 18 candidate slow tests individually, running each in isolation via `cargo test --lib -- <test_name>` with a 300-second timeout. Sequential execution avoids contention effects. The candidate list is maintained manually in `analyze.py` and should be updated when the test suite changes.

Current candidates (all from `crates/`):
- `algorithms::hk2017::literature_test::*` (8 tests)
- `algorithms::hk2017::orbit_recovery_test::*` (4 tests)
- `algorithms::hk2017::conformality_test::capacity_conformality`
- `algorithms::hk2017::symplectic_invariance_test::*` (2 tests)
- `algorithms::hk2017::pruning_test::pruned_matches_unpruned_from_fixture`
- `geom::volume_test::proptests::volume_scales_with_fourth_power`
- `random_test::proptests::random_polytopes_pass_validation`

### Figure generation

Horizontal bar chart of the top 15 slowest tests, annotated with module paths. Uses matplotlib (skipped gracefully if unavailable).

### Logbook format

Each `logbook.jsonl` entry records:
```json
{"date": "2026-03-20", "commit": "c104340", "wall_s": 20.98, "cpu_s": 164.63, "n_tests": 317, "cores": 12, "top5": [{"test": "catalog_determinism", "s": 11.03}, ...]}
```

Compare across entries to detect performance regressions.

## When to run

- After optimizing `Polytope4D::new()` or the rational pipeline
- After changing test fixtures or fixture loading
- After adding/removing `#[ignore]` annotations
- Before reporting performance to Jörn

## Findings

### Most recent run (2026-03-20, commit c104340)

Full suite: 21.0s wall, 164.6s CPU, 317 tests passed, 12 cores.

Top 5 slowest tests (sequential, no contention):

| Test | Duration |
|------|----------|
| `catalog_determinism` | 11.03s |
| `fixture_staleness_check` | 10.78s |
| `dwell_times_positive` | 10.53s |
| `breakpoint_count_consistency` | 10.51s |
| `hypercube_capacity` | 10.39s |

All top-5 are in `algorithms::hk2017::literature_test` or `algorithms::hk2017::orbit_recovery_test`. These tests compute EHZ capacity on multiple polytopes and are inherently expensive.

### Historical trend (3 runs)

| Date | Commit | Wall (s) | CPU (s) | Tests |
|------|--------|----------|---------|-------|
| 2026-03-19 | 7c64e76 | 21.88 | 167.6 | 317 |
| 2026-03-20 | 0c3c3d3 | 22.16 | 164.5 | 317 |
| 2026-03-20 | c104340 | 20.98 | 164.6 | 317 |

Performance is stable across these commits (~21s wall, ~165s CPU).

## Known limitations

- The candidate slow test list is maintained manually — new slow tests won't appear until the list is updated.
- Per-test timing runs each test individually, so startup overhead is included in each measurement.
- Wall-clock time depends on system load; CPU time is more stable for comparisons.
- `test_timing.png` is not committed (generated on demand).
