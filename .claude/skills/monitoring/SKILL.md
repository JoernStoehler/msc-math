---
name: monitoring
description: Use when running periodic health checks on the repo (build performance, algorithm agreement, issue board health). Invoke with /monitoring to start a monitoring session.
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
cd /workspaces/msc-math/crates && cargo test --package geom monitoring::boundedness_agreement -- --ignored --nocapture 2>&1 | tail -50
```

**Expected result:**
- Test passes: "test monitoring::boundedness_agreement ... ok"
- 100% agreement on all tested polytopes

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

### Check 4: Issue Board Health

**Purpose:** Detect stale or inconsistent issues.

**Background:** Issues labeled `in-progress` should have active work. Issues labeled `approved` should move to `in-progress` within a reasonable window.

**Command:**
```bash
cd /workspaces/msc-math && gh issue list --state open --label in-progress --format "{{.Number}} {{.Title}} {{.UpdatedAt}}" 2>&1 | head -20
```

**Expected result:**
- All in-progress issues updated within last 3 days (or have "paused" label)
- Each in-progress issue has a corresponding git branch

**Alert threshold:**
- In-progress issue with no updates > 7 days and no "paused" label
- In-progress issue with no corresponding branch
- Approved issue > 14 days without moving to in-progress

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
- Any single crate consuming >60% of total test time

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
