# Monitoring Baselines

Last updated: 2026-02-14 (local `main` at `97455b3` after testing-convention merge)

## Check 3: Repo Invariants

| Metric | Value |
|--------|-------|
| Total tests | 185 passed, 21 ignored, 0 failed |
| Clippy | Clean (0 warnings) |

## Check 5: Build & Test Performance

| Metric | Wall Time | Notes |
|--------|-----------|-------|
| Hot build (no-op) | 0.64s | |
| Full test suite | 18.5s | All crates, debug, --lib |
| geom | 3.03s | 126 passed, 5 ignored |
| hk2017 | 12.13s | 23 passed, 6 ignored |
| billiard | 1.93s | 6 passed, 8 ignored |
| tube | 0.00s | 1 passed |
| datasets | 1.31s | 29 passed |
| Clippy | 0.24s | |

**Note:** hk2017 at 65% of total test time (down from 87% pre-refactoring). 4 slow tests now #[ignore] and run in release mode only.

## Check 6: Stale Processes

| Metric | Value |
|--------|-------|
| Stale test processes | 0 |
