# Migration review

Updated 2026-09-05. This is a working review surface for Jörn, not agent
instructions. Add comments with Ctrl+K, then save with Ctrl+S. Durable facts and
procedures live at the linked owners below.

## Aim and scope

Make the project navigable and its expected tools predictable, with enough
reliable context to avoid costly wrong turns. Producing a passing thesis or
designing a chapter-writing workflow is not this migration's completion gate.

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

## Remaining work and ownership

[INSTALL.md](../INSTALL.md) documents the Sage setup commands, sources,
verification and alternative-route constraints. Sage 10.9 is installed in the
current sandbox. Fresh-SSH exact arithmetic, the full HKO verifier and a
50-case pentagon prefix passed; the full pentagon certificate was not rerun.

Global skills, documentation and memory migration belongs to DevOps. Its
2026-09-05 handoff reports the agreed sandbox installation complete, with no
consumer-side action needed. [Environment details](development-environments.md)
record the installed locations and update semantics. This project will not
install an independent global system.

After cleanup, recover concrete data-science gaps and recommend which to close
or report as unfinished. Administrative status and prose quality have not been
reassessed in this migration.

The first source review distinguishes three issues, not one acceptance gate:

- P2's registered prepared table lacks six active ridge columns. Its README
  supplies a rebuild route; that route still needs verification. This is a
  reproduction-path issue, not evidence that the retained result is false.
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
