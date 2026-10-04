# Current handoff

**4 October 2026: thesis execution is stopped by Jörn.** A preservation
checkpoint and concrete continuation packet are in
[restart-2026-10-04.md](restart-2026-10-04.md). Read that first. It records the
actual worktree/branch, original deadline and scope, unintegrated edits,
reported checks, known defects and an explicitly untested orchestration proposal.
The packet is not itself restart authorization or evidence of thesis acceptance.

The earlier September cleanup-only authorization and sole-worktree description
are superseded. Current selected manuscript: `thesis/main.tex`. Historical
human PASS and university submission status remain unknown; the current user
asked for PDF/repository quality, then stopped execution for this handoff.

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
