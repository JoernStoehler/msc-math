# Monitoring Report: YYYY-MM-DD

**Session:** (slug or description)
**Checks run:** (list which checks were executed)
**Trigger:** (what prompted this run — merge, weekly, request, etc.)

---

## Changes Since Last Report

```
(paste `git log --oneline` since the date of the previous report)
```

---

## Check Results

### Check 1: Boundedness Detection Agreement

**Status:** (PASS / FAIL / SKIPPED)
**Key metric:** (N polytopes tested, agreement rate)
**Notes:** (any observations)

### Check 2: Volume Algorithm Agreement

**Status:** (PASS / FAIL / SKIPPED)
**Key metric:** (N polytopes tested, max relative error)
**Notes:**

### Check 3: Repo Invariants

**Status:** (PASS / FAIL / SKIPPED)
**Key metric:** (N tests passed, clippy clean yes/no)
**Notes:**

### Check 5: Build & Test Performance

**Status:** (PASS / FAIL / SKIPPED)

| Measurement | Time | Notes |
|---|---|---|
| Hot build (no-op) | | |
| Full test suite | | |
| geom | | |
| hk2017 | | |
| billiard | | |
| tube | | |
| datasets | | |
| Clippy | | |
| Cold build | | (if measured) |

**Hotspot analysis:** (which crate/test dominates, what % of total)
**Comparison to previous:** (faster/slower/same, by how much)

### Check 6: Stale Processes

**Status:** (PASS / FAIL / SKIPPED)
**Key metric:** (N stale processes found, total CPU %)
**Notes:**

---

## Alert Threshold Breaches

(List any breaches, or "None")

---

## Findings

(Unexpected observations, new patterns, things to investigate)

---

## Suggested Actions

(Concrete next steps, if any — e.g., "investigate why hk2017 tests are 5x slower", "add #[ignore] to slow property tests")

## Suggested Skill Changes

(Improvements to the monitoring skill itself — new checks, adjusted thresholds, workflow tweaks)
