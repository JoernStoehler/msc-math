---
name: monitoring
description: Use when running periodic health checks on the repo (build performance, algorithm agreement). Invoke with /monitoring to start a monitoring session.
---

# Monitoring: Periodic Health Checks

Validation of important invariants that don't need to run every session. These checks detect drift, inconsistencies, regressions, and stale content.

## When to Run

**Suggested triggers:**
- After one or more merges to main (especially for Check 5: Build Performance)
- Weekly (start of each week's sessions)
- Before major milestones
- When investigating anomalies
- When Jörn requests a health check

## Check Catalog

Each check defines: purpose, commands, expected results, and alert thresholds.

---

### Check 1: Boundedness Detection Agreement

**Purpose:** Verify qhull and check_bounded() agree on polytope boundedness.

**Background:** Both mechanisms should identify unbounded polytopes identically:
- Qhull uses sentinel vertices (-10.101) — undocumented but empirically 100% reliable
- check_bounded() uses explicit O(F^3) positive span verification
- Both must agree; any disagreement indicates a bug

**Command:**
```bash
cd /workspaces/msc-math/crates && cargo test --package geom qhull_boundedness_test::investigation::cross_check_boundedness_detection -- --ignored --nocapture 2>&1 | tail -50
```

**Expected result:**
- Test passes: "test qhull_boundedness_test::investigation::cross_check_boundedness_detection ... ok"
- 100% agreement on all tested polytopes (875/875 or similar)

**Alert threshold:**
- ANY disagreement between qhull and check_bounded()
- Test failure or panic

**Test location:** `crates/geom/src/qhull_boundedness_test.rs`

---

### Check 2: Volume Algorithm Agreement

**Purpose:** Verify volume() (qhull triangulation) and volume_divergence() (divergence theorem) agree.

**Background:** Two independent approaches to computing volume should agree:
- volume() uses qhull triangulation (current production)
- volume_divergence() uses divergence theorem (reference implementation)
- Empirically validated on 1000+ polytopes with <5e-8 max relative error

**Command:**
```bash
cd /workspaces/msc-math/crates && cargo test --lib volume::volume_test::comprehensive_volume_cross_check -- --nocapture 2>&1 | tail -50
```

**Expected result:**
- Test passes: "test volume::volume_test::comprehensive_volume_cross_check ... ok"
- "1000 polytopes tested, max rel error: <5e-8"

**Alert threshold:**
- Max relative error > 1e-7
- Test failure or panic

**Test location:** `crates/geom/src/volume_test.rs`

---

### Check 3: Repo Invariants

**Purpose:** Verify tests pass and no clippy warnings.

**Command:**
```bash
cd /workspaces/msc-math/crates && cargo test --lib && cargo clippy --lib -- -D warnings 2>&1 | tail -20
```

**Expected result:**
- "test result: ok. N passed; 0 failed"
- No clippy warnings

**Alert threshold:**
- ANY test failure, clippy warning, or build failure

---

### Check 5: Build & Test Performance

**Purpose:** Track compilation and test execution times. Detect regressions, identify hotspots.

**Suggested frequency:** After one or more merges to main (higher than other checks).

**Commands:**
```bash
# 1. Hot build (no-op, everything already compiled)
cd /workspaces/msc-math/crates && time cargo build 2>&1

# 2. Full test suite (compile + run)
cd /workspaces/msc-math/crates && time cargo test --lib 2>&1

# 3. Per-crate test times
cd /workspaces/msc-math/crates
for crate in geom hk2017 billiard tube datasets; do
  echo "=== $crate ===" && time cargo test --lib -p $crate 2>&1
done

# 4. Clippy
cd /workspaces/msc-math/crates && time cargo clippy --lib -- -D warnings 2>&1

# 5. Cold build via temp worktree (non-destructive, run occasionally)
BENCH_WT=/tmp/bench-cold-$(date +%s)
git -C /workspaces/msc-math worktree add "$BENCH_WT" HEAD --quiet
(cd "$BENCH_WT/crates" && time cargo build 2>&1)
git -C /workspaces/msc-math worktree remove "$BENCH_WT"
```

**Alert threshold:**
- Hot test suite > 2x previous baseline
- Cold build > 2x previous baseline
- Single crate >75% of total test time: investigate (may be structural)
- Single crate >90% of total test time: action required

**Note:** Structural dominance (one compute-heavy crate, lightweight others) is expected in this workspace. hk2017 runs exponential-time capacity computation while other crates have fast tests. High percentage with reasonable absolute time is acceptable.

---

### Check 6: Stale Processes

**Purpose:** Detect zombie cargo/test processes left behind by ended sessions.

**Background:** When agent sessions end or background tasks are cancelled, child processes (test binaries, qhull subprocesses) can survive and consume CPU indefinitely. The `timeout` convention (CLAUDE.md) and worktree-remove.sh cleanup help prevent this, but detection catches what prevention misses.

**Command:**
```bash
# Find cargo test binaries running longer than 30 minutes
ps aux | grep -E 'target/debug/deps/' | grep -v grep
```

**Expected result:**
- No output (no stale test processes)
- Or: only processes from active sessions (recently started, reasonable CPU)

**Alert threshold:**
- Any test binary process older than 60 minutes
- Any test binary process from a worktree that no longer exists
- Total CPU usage from test processes > 400%

---

### Check 7: Test Strategy Review

**Purpose:** Detect tests that are slower than necessary in debug, tests doing multiple things without justification, and missing debug/release split annotations.

**Background:** Debug tests run ~65x slower than release for linear-algebra-heavy code. Tests that only exercise f64 math (KKT solves, capacity checks on large polytopes) gain nothing from debug mode — Rust's debug-only checks (integer overflow, debug_assert!) don't apply to f64 paths. Meanwhile, tests that exercise index arithmetic, array construction, and error paths benefit from debug mode's overflow/panic checks.

**Criteria for each test:**

| Category | Debug value | Speed concern | Action |
|----------|-----------|--------------|--------|
| Fast in debug (<1s) | Any | None | Run in debug (default) |
| Slow in debug, exercises usize/index logic | High | Yes | Keep in debug if unique coverage; else release-only |
| Slow in debug, pure f64/capacity math | None | Yes | Mark `#[ignore]`, run release-only |
| Tests conceptually different concerns in one function | — | — | Split or document justification |

**Procedure:**

1. List all `#[test]` and `#[ignore]` functions across all crates:
```bash
cd /workspaces/msc-math/crates
grep -rn '#\[test\]' --include='*.rs' | grep -v target/
grep -rn '#\[ignore\]' --include='*.rs' | grep -v target/
```

2. For each crate, measure debug vs release test time:
```bash
for crate in geom hk2017 billiard tube datasets; do
  echo "=== $crate debug ===" && time cargo test -p $crate 2>&1 | tail -3
  echo "=== $crate release ===" && time cargo test -p $crate --release 2>&1 | tail -3
done
```

3. For any test taking >2s in debug, classify:
   - What code paths does it exercise? (f64 math? index logic? error paths?)
   - Does debug mode catch anything release wouldn't? (debug_assert!, integer overflow on usize?)
   - Is there a fast-polytope version that covers the same debug-relevant paths?

4. For any test that bundles conceptually different concerns (e.g. known-value correctness AND large-scale random cross-check in one function), flag it. Multiple assertions about the same object's behavior (capacity value, iteration count, beta positivity) are fine — that's one concern ("is the result correct").

**Expected result:**
- Every `#[ignore]` test has a comment explaining why
- No un-ignored test takes >5s in debug
- Tests checking multiple properties have justification comments
- Debug suite exercises all code paths that benefit from overflow/bounds checks
- Release-only suite covers large-polytope correctness

**Note on optimization:** `nalgebra` has `opt-level = 3` in dev profile (since 2026-02-14, commit `6e2e2af`). This optimizes linear algebra ~2-3x in debug builds. Despite this, hk2017 capacity tests still show 50-80x debug/release ratios because combinatorial search, adjacency checking, and pruning run in debug mode (intentionally — these paths need overflow/bounds checks on index arithmetic). Large debug/release ratios are expected and correct for capacity-heavy tests.

**Alert threshold:**
- Any un-ignored test >10s in debug with no debug-specific value
- Any test bundling conceptually different concerns (e.g. known-value check + large random sweep)
- Any crate's debug tests >30s total

---

## Running a Monitoring Session

### Workflow

1. **Read this file** and the latest report in `docs/monitoring/`
2. **Decide what to run** based on:
   - `git log --oneline` since last monitoring report (what changed?)
   - Which checks haven't run recently
   - Jorn's specific request (if any)
3. **Execute checks** — run commands, capture output
4. **Write report** — copy template from `templates/report.md` to `docs/monitoring/YYYY-MM-DD.md`, fill in results
5. **Summarize for Jorn** — in chat, report:
   - Which checks ran, pass/fail
   - Any alert threshold breaches (with output)
   - Comparison to previous baseline
   - Suggested actions (if any)
   - Suggested changes to this monitoring skill (if any)

### Alert protocol

If any check breaches its threshold:
- Flag it prominently in the summary message
- Include the last 20 lines of output
- Offer to investigate further
- Do NOT silently record and move on

### Report format

Reports go in `docs/monitoring/YYYY-MM-DD.md` using the template in `templates/report.md`. Each report is a snapshot — don't edit past reports.
