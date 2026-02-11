# Monitoring: Long-Term Health Checks

Periodic validation of important invariants that don't need to run every session. These checks help detect drift, inconsistencies, and stale content over time.

## When to Run

**Suggested triggers:**
- Weekly (start of each week's sessions)
- Before major milestones (e.g., before merge to main of significant features)
- When investigating anomalies or unexpected behavior
- When Jörn requests a health check

**Who runs:** Any agent, on Jörn's request or schedule

---

## Checks

Each check defines:
- **Command** - Bash command to execute
- **Expected result** - What "passing" means
- **Alert threshold** - When to flag Jörn
- **Last run** - Date + session ID
- **Last result** - Status + notes

---

### Check 1: Boundedness Detection Agreement

**Purpose:** Verify qhull and check_bounded() agree on polytope boundedness.

**Background:** Both mechanisms should identify unbounded polytopes identically:
- Qhull uses sentinel vertices (-10.101) - undocumented but empirically 100% reliable
- check_bounded() uses explicit O(F³) positive span verification
- Both must agree; any disagreement indicates a bug

**Command:**
```bash
cd /workspaces/msc-math/crates && cargo test --package geom monitoring::boundedness_agreement -- --ignored --nocapture 2>&1 | tail -50
```

**Expected result:**
- Test completes successfully
- Output shows: "test monitoring::boundedness_agreement ... ok"
- Report shows 100% agreement on all tested polytopes

**Alert threshold:**
- ANY disagreement between qhull and check_bounded()
- Test failure or panic
- Relative error > 1%

**Last run:** (never run yet)

**Last result:** (pending first run)

---

### Check 2: Volume Algorithm Agreement

**Purpose:** Verify new volume algorithm (qhull triangulation) and old algorithm (divergence theorem) produce consistent results.

**Background:** Two independent approaches to computing volume should agree:
- volume() uses qhull triangulation (current production)
- volume_divergence() uses divergence theorem (reference implementation)
- Empirically validated on 1000+ polytopes with <5e-8 max relative error

**Command:**
```bash
cd /workspaces/msc-math/crates && cargo test --lib volume::volume_test::comprehensive_volume_cross_check -- --nocapture 2>&1 | tail -50
```

**Expected result:**
- Test completes successfully
- Output shows: "test volume::volume_test::comprehensive_volume_cross_check ... ok"
- Report shows: "1000 polytopes tested, max rel error: <5e-8"

**Alert threshold:**
- Max relative error > 1e-7
- Test failure or panic
- Any assertion failure in cross-check

**Last run:** (never run yet)

**Last result:** (pending first run)

---

### Check 3: Repo Invariants

**Purpose:** Verify fundamental repo health (tests pass, no clippy warnings).

**Background:** These are documented in CLAUDE.md as requirements that must always be true.

**Command:**
```bash
cd /workspaces/msc-math/crates && cargo test --lib && cargo clippy --lib -- -D warnings 2>&1 | tail -20
```

**Expected result:**
- All tests pass: "test result: ok. N passed; 0 failed"
- No clippy warnings

**Alert threshold:**
- ANY test failure
- ANY clippy warning or error
- Build failure

**Last run:** (never run yet)

**Last result:** (pending first run)

---

### Check 4: Issue Board Health

**Purpose:** Detect stale or inconsistent issues.

**Background:** Issues labeled `in-progress` should have active work. Issues labeled `approved` should move to `in-progress` within a reasonable window.

**Command:**
```bash
cd /workspaces/msc-math && gh issue list --state open --label in-progress --format "{{.Number}} {{.Title}} {{.UpdatedAt}}" 2>&1 | head -20
```

**Expected result:**
- All in-progress issues should have updates within last 3 days (or note of deliberate pause)
- Each in-progress issue should have a corresponding git branch

**Alert threshold:**
- In-progress issue with no updates > 7 days and no "paused" label
- In-progress issue with no corresponding branch
- Approved issue > 14 days without moving to in-progress

**Last run:** (never run yet)

**Last result:** (pending first run)

---

## Agent Instructions

### How to use this file

1. **Read the file** - Understand which checks are available
2. **Select checks** - Either:
   - Run all checks, or
   - Run checks selected by Jörn, or
   - Run checks not run in last 7 days (your choice)
3. **Execute checks** - Run each command in order, capture output
4. **Update results** - For each check:
   - Replace "(never run yet)" with today's date + session ID
   - Replace "(pending first run)" with status (PASS/FAIL) + key finding
   - Include max error, test count, or key metric
5. **Alert Jörn** - If any check fails threshold:
   - Post message with which check failed
   - Include alert threshold that was exceeded
   - Include last 20 lines of output
   - Offer to investigate further
6. **Add findings** - If unexpected, add entry to Findings Log (see below)

### Example update format

After running Check 1:

```markdown
**Last run:** 2026-02-11 (session: review-rust-complexity)

**Last result:** PASS - 875 random polytopes tested, 100% agreement, 0 disagreements
```

After a failure:

```markdown
**Last run:** 2026-02-11 (session: investigate-edge-case)

**Last result:** FAIL - 3 disagreements found on polytopes with ≤5 vertices
```

---

## Findings Log

When a check reveals an issue, add an entry here with the date, what was found, investigation, and resolution.

### 2026-02-11: Initial creation

- **Check**: All checks
- **Finding**: Baseline established (checks not yet run)
- **Status**: Pending first execution
- **Next steps**: Run all checks after Rust complexity review merge

---

## Notes for Future Maintenance

- **Monitoring tests live in:**
  - `crates/geom/src/qhull_boundedness_test.rs` (Check 1)
  - `crates/geom/src/volume_test.rs` (Check 2)
  - Standard test suite (Check 3)
  - GitHub CLI (Check 4)

- **Update this file when:**
  - Tests move to different files
  - Expected thresholds change
  - New important invariants discovered
  - Findings log entries are added

- **Interpretation:**
  - Checks 1 & 2 validate mathematical correctness across algorithm changes
  - Check 3 validates repo stability (build/test health)
  - Check 4 validates workflow health (issue tracking)
