# Resume here

Use branch **`thesis/resume-20260918`**, currently checked out at
`/workspaces/msc-math/.worktrees/candidate-assembly-integration`.
The directory name is retained for the idle agent; there is no competing
integration branch. The prior local review service is unavailable, so use the
durable PDF named below. `main` is retained with Jörn's
uncommitted work untouched.

## Start points

- **Current obligations:** [docs/WORK_REMAINING.md](docs/WORK_REMAINING.md).
  This is the single reconciled work list, including repaired/withdrawn findings
  and unresolved scientific selection. Historical plans are not assignments.
- **Selected manuscript:** [thesis/main.tex](thesis/main.tex).
  Build `sh thesis/build.sh`; read `thesis/build/main.pdf`.
  There are no candidate/recovered/legacy manuscript alternatives in the live tree.
- **Scientific support and human feedback:** [docs/README.md](docs/README.md).
- **Frozen handoff PDF:** `docs/resume/thesis-resume.pdf` (diagnostic only).
  `docs/resume/build-verification.json` records its inputs and hash;
  `python3 docs/resume/verify.py` checks retained evidence and those inputs.

## Acceptance, resources and interaction

The thesis is unfinished and has not been submitted for final grading. PASS means
submission-ready with only minor issues Kai could leave Jörn to fix in about two
hours. Borderline is FAIL. Final grading is one-time; diagnostic feedback is
separate. No university submission or public release is authorized.

The completion attempt was stopped. Consolidation and cleanup were subsequently
authorized; they do not authorize a new autonomous run. The former deadline was
2026-09-18 22:00 UTC. The last observed quota was 6% at 16:59:57 UTC, not a current
balance. The additional shadow-API ceiling was $3,000, not permission to exhaust
it. Neither the prior 2% nor 5% completion estimate was supported; both were
retracted. No reliable authoring/reviewer workflow was established.

Jörn reads the async queue. Necessary requests must stand alone, identify the
artifact and exact judgment/consequence, and state the agent's expectation.
His last reliable availability cutoff was 17:00 UTC; later brief presence was
not an extension. Check current availability rather than assuming another review
window. The current handoff is not a final grading submission.

## Sessions and recovery

[docs/resume/sessions.json](docs/resume/sessions.json) records exact UUIDs, local
session-log paths and last observed panes. Main coordinator:
`01a0b153-5091-7333-935c-f08856210097`; replacement:
`01a0b4e9-b856-7de2-b42a-39d3526743b9` (last observed idle at w26:p8).
Empirical, reviewer and human-review desk UUIDs are included. Inspect current
Herdr identity before messaging; panes are not durable identities.

The selected manuscript's old-to-new path map is
`docs/resume/layout-migration.json`. Superseded manuscript trees are in Git at
`35f29db4:thesis/`, not alternate edit targets. Attributed review records and old
plans are explicitly historical under `docs/history/`.

Only main and the resume branch/worktree remain. Full original branch history
and 108 untracked/generated files are preserved at
`/workspaces/archived/workspaces-root/msc-math-session-20260918/`.
Its standalone bundle restore recovered all 43 exact original heads. Eleven
obsolete review servers were stopped; old URLs should not be reused.
`docs/resume/cleanup-result.json` records recovery hashes and removed paths.

[External dependencies](docs/resume/external-dependencies.md) documents registered
raw datasets and the older ridge-analysis replay whose caches must be regenerated.
The thesis build requires no old worktree or retained temporary input. Session
logs live in `~/.codex/sessions/`; the handoff itself depends on no temporary file.
