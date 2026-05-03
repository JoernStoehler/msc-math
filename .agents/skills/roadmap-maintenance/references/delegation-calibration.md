# Delegation Calibration

Append-only reference for project-management task packets whose actual difficulty taught us something useful about delegation.

## How To Update

Add a new dated entry at the top, below this section. Do not rewrite old entries except to fix a factual error, and if you do, say what was corrected. Keep each entry compact and evidence-based.

Record tasks that changed calibration: unexpectedly easy, unexpectedly hard, wrong execution context, wrong output contract, surprising Jörn gate, or a verification surface that was weaker or stronger than expected. Do not log routine successful tasks unless they teach a reusable lesson.

Use this template:

```markdown
## YYYY-MM-DD - Short Task Name

- Prompt:
  - Original packet path or enough prompt text to reconstruct scope.
- Repo state:
  - Main commit before delegation:
  - Worktree or branch:
  - Dirty state or parallel edits:
- Expected difficulty:
  - Why it looked shallow or deep:
  - Expected failure mode:
- Actual output:
  - Answer or report path:
  - Commit(s):
  - Files changed:
  - Verification run:
- Outcome:
  - Done / partially done / wrong surface / needed Jörn / needed top-level integration.
- Calibration takeaway:
  - What future PM sessions should infer.
```

## 2026-04-15 - Research Migration Coverage Matrix

- Prompt:
  - `/tmp/1.md`: audit whether live experiments were covered by migrated `research/**/design/*.md` notes and whether live `logbook.md` remnants remained.
- Repo state:
  - Main commit before delegation: approximately `28ac0da2` was reached during the surrounding audit session; the exact start commit should be checked from the delegated worktree if needed.
  - Worktree or branch: `audit-research-migration-coverage`, answer in `/tmp/1-answer.md`.
  - Dirty state or parallel edits: multiple audit worktrees were active; main later received `296a6511`.
- Expected difficulty:
  - Looked shallow because it was read-only inventory plus path existence checks.
  - Expected failure mode: confusing formal-file coverage with research-note migration coverage, or over-editing `TASKS.md`.
- Actual output:
  - Answer/report path: `/tmp/1-answer.md`; detailed ignored report stayed in the audit worktree.
  - Commit(s): top-level integration commit `296a6511` changed the stale `TASKS.md` pointer for the deleted `experiments/hko-local-maximum/subdifferential-lp/` path to `research/hko-local-maximum/design/subdifferential-lp.md`.
  - Files changed by top-level integration: `TASKS.md`.
  - Verification run: delegate reported 33/33 live experiment directories covered by research notes, no live `logbook.md` remnants, and only `experiments/verification/algorithm-comparison/profiling/logbook.jsonl` as live logbook-named data.
- Outcome:
  - Done. Jörn was not needed for the audit or the tracker propagation.
- Calibration takeaway:
  - This was easier than requiring top-level/Jörn coordination. Similar read-only migration coverage matrices can be delegated end-to-end; the top-level session only needs to verify any small durable tracker propagation. In the current repo that means `ROADMAP.md` or the relevant `tasks/*.md`, not legacy `TASKS.md`.
  - The delegate used level 1 subagents from a level 0 session. In hindsight, a top-level session could likely have started this as a level 1 subagent task; the delegate's nested helpers would then have been level 2, still below the nesting cutoff. The task did not rely on level 0 access, did not need Jörn interaction that the top-level session could not handle, and finished in a few minutes.

## 2026-04-15 - Legacy `RESULTS.md` Freshness Audit

- Prompt:
  - `/tmp/2.md`: audit each legacy `RESULTS.md` bullet against local artifacts and identify stale, pending, or contradicted claims.
- Repo state:
  - Main commit before delegation: approximately the same audit window as above; exact start commit should be checked from local history if needed.
  - Worktree or branch: intended `audit-results-freshness`, but the agent ran in `/workspaces/msc-math` on `main`.
  - Dirty state or parallel edits: main had active audit work; the task should have used an isolated worktree.
- Expected difficulty:
  - Looked moderately shallow because it was evidence mapping plus optional obvious status fixes, with no thesis prose.
  - Expected failure mode: accidentally interpreting thesis claims rather than only classifying freshness.
- Actual output:
  - Answer/report path: `/tmp/2-answer.md`.
  - Commit(s): `28ac0da2` (`Refresh visualization audit notes`) on `main`.
- Files changed: `research/visualization.md`, `experiments/visualization/main/main.rs`.
  - Verification run: delegate reported no legacy `RESULTS.md` changes needed, no contradictions found, and `cargo build -p visualization --release --bin visualization` passing for the stale visualization note.
- Outcome:
  - Substantively done, but wrong execution context. The useful patch was small and relevant; the process violated the intended worktree isolation.
- Calibration takeaway:
  - Freshness audits can be delegated, but prompts must make cwd/worktree discipline explicit. Include: run `pwd` first; if it is not the assigned worktree, stop before editing. Also distinguish audit output from optional patch output so agents do not overrun into full pre-merge work for tiny comment fixes.
  - The delegate used level 1 subagents from a level 0 session. In hindsight, a top-level session could likely have started this as a level 1 subagent task; the delegate's nested helpers would then have been level 2, still below the nesting cutoff. The task did not rely on level 0 access, did not need Jörn interaction that the top-level session could not handle, and finished in a few minutes. The observed failure was cwd discipline, not nesting.
