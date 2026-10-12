# What adopting f7e0 would change

**I recommend adopting f7e0 as the basis for the next work.** It now contains
main’s research and evidence, with a more coherent organization and some
repairs. The remaining thesis and scientific work is substantial, but most of
that work was already required on main.

This compares main’s preserved state with the updated f7e0 branch. The missing
research and broken download path from the first review have been repaired.

## The main gain: less reconstruction at the start of work

Related explanations, proofs, experiments and evidence now live together by
scientific topic. For example, the local-maxima account explains which HKO and
non-HKO results exist, what supports them and which extensions remain open.
An agent has a starting point for understanding the result, rather than only a
list of files to investigate.

Repeated geometry and data-processing work also has fewer separate
implementations to keep consistent. The expected payoff is less repeated
orientation and fewer duplicate repairs. We have not measured the time saved
for you or for agents.

## The tradeoff: clearer accounts need upkeep

The topic accounts, inventory and thesis dashboard summarize underlying work.
When a result changes, an agent must update the affected summaries as well as
the source. Otherwise the easier entry points become misleading. This is
ongoing maintenance for the agents doing the work; you should not have to
coordinate individual file updates.

Sharing implementations also reduces the independence of some geometry
cross-checks: two routes can agree because they use the same calculation.
When stronger evidence is needed, the reviewing agent needs a separate check
of those shared parts.

Some analysis rules are also stricter. A new run can reject inputs or fitting
targets that the old version accepted. Existing result files were preserved,
but rerunning affected analyses may produce different selections. The next
analyst owns checking what changed before using a rerun as evidence.

## What new review work comes with f7e0

- **Changed mathematical arguments:** some existing geometry proofs were
  corrected or given more precise assumptions. Agents reviewed the repairs;
  your acceptance of the revised proofs is still outstanding. This matters
  when relying on those arguments.
- **Changed PDF layout:** moved paths affected the working thesis layout.
  An agent needs to check the affected PDF and update its review references
  before presenting it as reviewed. This adds layout checking, not a new
  obligation for you to recheck the entire repository.
- **Reopening old sessions:** an agent now restores archived work rather than
  opening its old checkout. That recovery cost occurs only when the older work
  is needed; the archives remain local.

## What adoption would leave unfinished

The thesis still omits the selected symmetric-partner result. Human PASS,
settling the data-science scope, stronger validation of numerical volumes and
historical results, and complete reproduction remain open. These gaps existed
on main. The flow theorem and bounded checks also do not certify the search
program for every input. Reorganization makes these limits easier to locate;
it does not resolve them.

The preservation checks, focused tests and independent reviews support adopting
the reorganized repository. They do not establish the truth of every result or
thesis acceptance. My recommendation concerns the working repository.

## Supporting sources

This comparison concerns f7e0 `ad88f728` and the former main checkpoint
`185155fe`. Jörn approved adoption at 15:57 UTC; main now includes the reviewed
branch and this rewritten comparison. The evidence and maintenance limits above
remain unchanged.
The earlier technical reconciliation report is preserved in Git at
`ad88f728:meta-topics/repository-organization/main-reconciliation-2026-10-12.md`.

- [Research path and identity checks](main-reconciliation-paths-2026-10-12.json)
  retain the main-evidence preservation and moved-source identities.
- [Local-maxima account](../../topics/variation-and-local-maxima/README.md)
  illustrates the new scientific entry points.
- [Shared geometry and checking limits](inventory/pilots/2026-10-11/geometry.md)
  explain which cross-checks now share calculations.
- [Geometry changes and review limits](inventory/pilots/2026-10-11/foundations.md)
  identify the changed arguments and outstanding human review.
- [Analysis changes and checks](../../topics/empirical-studies/migration-review-2026-10-12.md)
  distinguish stricter acceptance rules from retained historical evidence.
- [Flow-search account](../../topics/flow-graphs/README.md) separates the
  conditional theorem from the implementation’s supported behavior.
- [Thesis work](../project-management/thesis-work.md) and
  [PDF baseline](../../docs/dashboard/thesis-sources.json) own the remaining
  manuscript and review obligations.
- [Recovery record](inventory/old-worktree-recovery-2026-10-12.json) identifies
  the local archives. Checksums and Git bundle verification passed; this review
  does not add an off-machine recovery promise.

No producers, full theorem replays, dataset downloads or full PDF visual review
were performed for this rewrite. The original snapshot review remains a
historical record; its repaired defects are not current adoption blockers.

[review: /root/readability_reviewer]
