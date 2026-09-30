# Current handoff

Work in the checked-out repository. On 30 September 2026, `main` at
`/workspaces/msc-math` is the only registered worktree. Confirm the current
branch/tree before continuing; old session pane and worktree directions are
recovery provenance, not instructions to restart them.

Start with [current.json](current.json) for selected work and
[thesis-work.md](thesis-work.md) for the detailed thesis backlog.
[../dashboard/index.html](../dashboard/index.html) is Jörn's work/resource view;
[../knowledge/README.md](../knowledge/README.md) routes reusable interpretation.

## Acceptance and authorization

The prior completion attempt was stopped. The current request authorizes
repository cleanup and coordination setup, not a new autonomous thesis run.
PASS means submission-ready with only minor issues Kai could leave Jörn to fix
in about two hours; borderline is FAIL. Final grading is one-time; diagnostic
feedback is separate. No current whole-PDF verdict establishes readiness.
No university submission or public release is authorized.

The retained handoff describes the thesis as unfinished and unsubmitted at that
time. Current administrative status has not been checked. Historical quota,
deadlines, availability cutoffs and the $3,000 shadow-API ceiling are not current
resource observations or renewed spending authority. Necessary questions must
stand alone with the artifact, exact judgment, consequence and expectation.

## Evidence and recovery

The selected manuscript is `thesis/main.tex`; see `thesis/README.md` and build
with `sh thesis/build.sh`. The frozen diagnostic PDF is
`docs/resume/thesis-resume.pdf`; `docs/resume/build-verification.json` records
its inputs and hash. `python3 docs/resume/verify.py` checks frozen-build and
preservation evidence. A build or preservation check does not establish PASS.
On 30 September, the frozen input record mismatches seven files already in
committed `HEAD` (HKO appendix/program, abstract, introduction, Chapter 7,
availability and conclusion). This cleanup did not change those sources or
refresh the frozen record. Treat that PDF as an older diagnostic snapshot.

Session identities are retained in `docs/resume/sessions.json`. Ordinary
resumption must not inspect raw rollouts or restart old panes or agents.
`docs/resume/layout-migration.json` maps old manuscript paths; earlier selected
trees are recoverable at `35f29db4:thesis/`. Superseded plans are indexed in
`docs/history/retired-paths.json`, not active edit targets.

The full original branch history and retained generated/untracked files were
archived at `/workspaces/archived/workspaces-root/msc-math-session-20260918/`.
The cleanup record reports recovery of all 43 original heads and shutdown of
eleven old review servers. Do not reuse old review URLs. See
`docs/resume/cleanup-result.json` and `docs/resume/external-dependencies.md` for
recovery hashes and dataset/cache dependencies. Preservation still has the
recorded limitation: 3,606 excluded `target/` and `__pycache__` files lack
individual pre-deletion inventories; later checks do not resolve that gap.

The exact pre-migration handoff, including contradictory opening directions,
is retained at
[../history/coordination-surface-migration/resume-20260930.md](../history/coordination-surface-migration/resume-20260930.md).
It supplies historical context without competing with this continuation entry.
