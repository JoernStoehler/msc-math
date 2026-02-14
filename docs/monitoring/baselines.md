# Monitoring Baselines

Last updated: 2026-02-14 (local `main` at `d7e0e6d`)

## Check 3: Repo Invariants

| Metric | Value |
|--------|-------|
| Total tests | 189 passed, 15 ignored, 0 failed |
| Clippy | Clean (0 warnings) |

## Check 5: Build & Test Performance

| Metric | Wall Time | Notes |
|--------|-----------|-------|
| Hot build (no-op) | 0.64s | |
| Full test suite | 42.8s | All crates, debug, --lib |
| geom | 2.93s | 126 passed, 5 ignored |
| hk2017 | 37.43s | 27 passed, 2 ignored |
| billiard | 2.16s | 6 passed, 8 ignored |
| tube | 0.13s | 1 passed |
| datasets | 1.42s | 29 passed |
| Clippy | 0.24s | |

**Note:** hk2017 at 87.5% of total test time is structural (exponential capacity computation). Absolute time (37.4s) is acceptable.

## Check 6: Stale Processes

| Metric | Value |
|--------|-------|
| Stale test processes | 0 |
