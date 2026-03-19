# Test Suite Profiling

Profile the default test suite (`cargo test --lib`) to identify hot paths and track performance over time.

## Status
Active — run after any change to `Polytope4D::new()`, test fixtures, or the capacity algorithm.

## Files

| File | Purpose |
|------|---------|
| `profile.py` | Run profiling, collect per-test timings, generate figures |
| `profile.jsonl` | Per-test timing data (one line per test) |
| `logbook.jsonl` | Historical runs (one line per profiling session) |
| `test_timing.png` | Figure: test duration breakdown |

## Workflow

An agent can run this end-to-end:

```bash
cd experiments/
python3 test-profiling/profile.py
```

This will:
1. Run `cargo test --lib` with per-test timing (parses `--format json` output)
2. Write per-test results to `profile.jsonl`
3. Append a summary to `logbook.jsonl` (date, commit, wall time, CPU time, top-5 slowest)
4. Generate `test_timing.png` (bar chart of slowest tests + pie chart by module)

## When to run

- After optimizing `Polytope4D::new()` or the rational pipeline
- After changing test fixtures or fixture loading
- After adding/removing `#[ignore]` annotations
- Before reporting performance to Jorn

## Logbook

`logbook.jsonl` tracks performance over time. Each entry has:
```json
{"date": "2026-03-19", "commit": "abc1234", "wall_s": 21.4, "cpu_s": 166, "n_tests": 317, "top5": [...]}
```

Compare across entries to see whether changes helped or hurt.
