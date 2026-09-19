# Migration review

Setup/cleanup evidence from 2026-09-05; scope clarified 2026-09-07.
This is a working review surface for Jörn, not agent
instructions. Add comments with Ctrl+K, then save with Ctrl+S. Durable facts and
procedures live at the linked owners below.

For the actual current assignment and proposal status, read
[current work and ownership](../memories/todos.md). This review is not a queue.

## Aim and scope

The wider outcome is a good thesis that passes Jörn's full-PDF review, ideally
once. Predictable tools, useful layout/navigation and authoring-workflow
discovery support that outcome. Completing the setup checks below does not
complete authoring-workflow discovery. Detailed summary-layer design is deferred
until writing and feedback needs inform it; see the
[planning context](../memories/planning-context.md).

## Research status confirmed by Jörn

- HKO local maximality is an established theorem.
- The rotated-pentagon result is an established theorem.
- Flow graph has proofs.
- Data science is moderately incomplete. Which gaps to finish versus report
  as unfinished remains unresolved; repository cleanup comes first.

These corrections are recorded in [project facts](project-facts.md), and the
data-science entry point reflects the unresolved overall scope. The initial
briefing understated the known theorem status. A future review may ask whether
a particular written argument proves its stated conclusion; a flaw in that
argument would not imply that the theorem is false.

The manuscript's exact formulations, exposition and artifact correspondence
remain distinct from these established research results. No broad proof audit
was performed during migration.

## Completed work

Obsolete agent skills and steering were removed to Git history. The empty
AGENTS.md was removed. Scratch proposals were preserved on an archive branch,
their generated outputs copied and compared, and the checkout removed.
[Recovery pointers](README.md) preserve access.

Navigation now points to active thesis and proof sources rather than presenting
quarantined writing companions as current instructions. Source-PDF links were
corrected. The thesis check command now performs a fresh build before checking
its outputs.

Astra, native search, SSH entry, Rust PATH and Micro commenting work in the
sandbox. Rust tests, Python/uv, GitHub access and R2 listing were checked.
Both host and sandbox thesis builds passed their selected mechanical checks.
These checks establish the named operations, not thesis readiness.
[Environment details and limits](development-environments.md) own the evidence.

## Setup completion and known research limitations

[INSTALL.md](../INSTALL.md) documents the Sage setup commands, sources,
verification and alternative-route constraints. Sage 10.9 is installed in the
current sandbox. Fresh-SSH exact arithmetic, the full HKO verifier and a
50-case pentagon prefix passed; the full pentagon certificate was not rerun.

Global skills, documentation and memory migration belongs to DevOps. Its
2026-09-05 handoff reports the agreed sandbox installation complete, with no
consumer-side action needed. [Environment details](development-environments.md)
record the installed locations and update semantics. This project will not
install an independent global system.

Data-science scope remains unresolved beyond the specific decision below.
Administrative status and thesis-wide prose quality have not been reassessed.

The first source review distinguishes three issues, not one acceptance gate:

- P2's registered prepared table lacks six active ridge columns. Its README
  supplies a rebuild route. The 2026-09-05 rebuild matched both expected hashes
  using an existing hash-verified source cache; fresh R2 retrieval was not
  tested. It consumed about 11 cores for six minutes without Jörn's permission.
  It finished before the attempted stop; further execution was cancelled.
  This check is complete and must not be rerun for migration verification.
- The historical optimizer comparison ranks seven implementations on 64 F10
  starts under a nominal one-second allocation rule and a heuristic evaluator.
  Complete tuning/holdout separation is not established by the retained
  provenance. A clean comparison would test transfer of the ranking; whether
  that is worth doing is a research-scope question, separate from reproduction.
- The local-maxima screen retains reconciled rows but not the exact dirty
  producer state. The thesis already discloses this. A new run would be new
  evidence, not recovery of the missing source state.

Four stale claims that generated-candidate infrastructure is absent were
corrected in the [method ledger](../experiments/sys-datascience/methods/trusted-random-product-method-dispositions.md).
Infrastructure availability does not complete the corresponding comparisons.

Resolved scope decision, Jörn 2026-09-05: the proposed comparison of branch-aware
optimization against relevant nonsmooth alternatives is relevant but not
interesting enough to delay submission by even an hour. Report this gap rather
than completing that comparison for the thesis. Other data-science scope
questions remain separate.

## Assignment clarity review

Completed 2026-09-05 under Jörn's explicit assignment A. The review followed
the root/domain entry points and their data-science/gradient planning links;
it was not a line-by-line audit of every nested research packet.

- Added one discoverable [assignment record](../memories/todos.md), separating
  assigned A, proposed B, completed checks, DevOps ownership and unassigned
  research/administrative work.
- Removed competing live "feature-complete" / "next default" interpretations
  from the data-science entry points; July closure/recommendations remain
  historical, not today's work queue.
- Corrected the gradient charter's claim that its candidate was ready for
  promotion: its own linked packet says the setup-pass evidence is not retained
  and the packet is not ready. Kept algorithm description separate from proposed
  regeneration, promotion and integration work.
- Removed the coordination entry point's dependency on the removed
  `empirical-research` skill.
- Marked the old registration-note pending action as historical; its current
  completion status is unknown, not an assigned administrative task.

No unresolved assignment conflict was found in the reviewed routes after these
repairs. Jörn subsequently approved removing the old gradient charter; it was
removed and its live links repaired. The package README records Git recovery.
The subsequent approved cleanup also removed the promotion packet and its live
links. The algorithm description retains the finite endpoint definition and
precise historical recovery pointers; unsupported numerical success/cost claims
were removed. Source and inputs in Git were distinguished from the still
unrecovered complete execution/output chain. No historical code was rerun.
Clearly frozen experiments, algorithm/source descriptions and conditional final-release
checklists were left alone; they are not assignments merely by existing.
