# Test Suite Profiling: Logbook

## Motivation

Profile the default test suite (`cargo test --lib`) to identify hot paths and track performance over time. Provides data for decisions about `#[ignore]` annotations and optimization targets.

## Status

**Active** — run after any change to `Polytope4D::new()`, HK2017 smoke tests, or the capacity algorithm.

## How to run

```bash
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

Profiles 15 candidate slow tests individually, running each in isolation via `cargo test --lib -- <test_name>` with a 300-second timeout. Sequential execution avoids contention effects. The candidate list is maintained manually in `analyze.py` and should be updated when the test suite changes.

Current candidates (all from `library/`):
- `algorithms::hk2017::tests_literature::*` (6 tests)
- `algorithms::hk2017::orbit_recovery::tests::*` (4 tests)
- `algorithms::hk2017::tests_conformality::capacity_conformality_simplex`
- `algorithms::hk2017::tests_symplectic_invariance::capacity_symplectomorphism_invariance_simplex`
- `algorithms::hk2017::tests_pruning::pruned_matches_unpruned_simplex`
- `geom::volume::tests::proptests::volume_scales_with_fourth_power`
- `random::tests::proptests::random_polytopes_pass_validation`

### Figure generation

Horizontal bar chart of the top 15 slowest tests. Uses matplotlib (skipped gracefully if unavailable).

### Logbook format

Each `logbook.jsonl` entry records:
```json
{"date": "2026-04-15", "commit": "abc1234", "wall_s": 20.98, "cpu_s": 164.63, "n_tests": 337, "cores": 12, "top5": [{"test": "hypercube_capacity", "s": 10.39}, ...]}
```

Compare across entries to detect performance regressions.

## When to run

- After optimizing `Polytope4D::new()` or the rational pipeline
- After changing HK2017 smoke-test structure
- After adding/removing `#[ignore]` annotations
- Before reporting performance to Jörn

## Findings

### Most recent run (2026-04-15, commit f5d4ba18)

Full suite: 53.22s wall, 238.41s CPU, 337 tests passed, 24 ignored, 12 cores.

Top 5 slowest tests (sequential, no contention):

| Test | Duration |
|------|----------|
| `hypercube_capacity` | 27.77s |
| `breakpoint_count_consistency` | 15.74s |
| `dwell_times_positive` | 15.70s |
| `hko_pentagon_recovery` | 12.34s |
| `billiard_agrees_with_hk2017_on_small_lagrangian_products` | 4.09s |

All top-5 are HK2017 live computation smoke/regression tests. The deleted fixture tests no longer appear in the profiler candidate list.

### Historical trend (3 runs)

| Date | Commit | Wall (s) | CPU (s) | Tests |
|------|--------|----------|---------|-------|
| 2026-03-19 | 7c64e76 | 21.88 | 167.6 | 317 |
| 2026-03-20 | 0c3c3d3 | 22.16 | 164.5 | 317 |
| 2026-03-20 | c104340 | 20.98 | 164.6 | 317 |
| 2026-04-15 | f5d4ba18 | 53.22 | 238.41 | 337 |

The 2026-04-15 run uses the post-fixture-removal candidate list and a different test count, so compare it to future runs rather than the March baseline.

## Known limitations

- The candidate slow test list is maintained manually — new slow tests won't appear until the list is updated.
- Per-test timing runs each test individually, so startup overhead is included in each measurement.
- Wall-clock time depends on system load; CPU time is more stable for comparisons.
- `test_timing.png` is not committed (generated on demand).
