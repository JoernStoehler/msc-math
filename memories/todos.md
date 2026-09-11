# Current work map

Updated 2026-09-11. Non-authoritative planning view; the reasons and user
decisions behind it live in [planning context](planning-context.md).
[Outcome coverage](outcome-coverage.md) retains whole-thesis composition, long
proofs, eventual delivery and new discovery findings beyond the proposed session
portfolio. It is not an exhaustive graph or additional assignment queue.

## September 11 PM session

Jörn requests continuous delegation-led project management. Completion proxy:
explicit approval from him after reading the thesis PDF that he thinks it earns
a PASS. The PM coordinates and owns small integration writes; bounded exploration
belongs to subagents. Herdr top-level launches require his approval of the actual
prompt, because he must understand and converse with those agents. Consequential
questions/results use async questions or self-contained finals, not commentary.

Read-only orientation found no other live thesis Herdr agent and no tracked
changes at `a3f06cb4`. The PDF was refreshed after September 7 prose changes and September 11
evidence qualifications; `./thesis/check-build.sh` passed its selected PDF,
overfull-box and reference checks. Full visual/scientific review remains open. Pre-existing `tmp/` remains
untouched. The [authoring launch prompt](/tmp/msc-math-prompts.JXv7AM/authoring-session-prompt.md) is a
proposal awaiting Jörn's approval, not an active assignment. September 7 session
briefs retain historical reasoning but are not current launch instructions.

Authoring remains open. A single-crate quality calibration is also a current
user proposal: `algebraic-numbers` is a bounded candidate, not a demonstrated
bottleneck. Its [pilot prompt](/tmp/msc-math-prompts.JXv7AM/crate-quality-session-prompt.md) is prepared
as an alternative, awaiting launch approval. Read-only assessment found repeated
solver elimination, incomplete generated nullspace checks, documentation drift,
and an endpoint-rounding conversion panic inferred from source and an independent
binary64 calculation (not a Rust regression); these are scoped candidates,
not an instruction to implement all of them. Broad refactoring and shared-skill
revisions have not been established
as prerequisites for passing.

Jörn reports that even realistic timelines have led agents to declare defeat
and attempt desperate actions, with claimed three-day work taking ten minutes.
This is his observed planning failure, not measured general model performance.
No new deadline was supplied. Sequence bounded work by its contribution and
observed cost; unsupported duration estimates do not justify defeat or shortcuts.
The launch drafts above are temporary active review artifacts under `/tmp`, not
project documentation; their paths may expire after the launch decision.

## Outcome and open work

Outcome: a scientifically sound, clear, relevant thesis on probing Viterbo's
conjecture that passes Jörn's full-PDF review, ideally on its first attempt.

| Branch | State | Useful next decision or investigation |
| --- | --- | --- |
| Authoring-workflow discovery | Open; HKO comparison retained as limited reference | Establish a credible route to a candidate worth Jörn's costly final review. Further passage checks are not the default next task: a trial needs to resolve a consequential workflow uncertainty. See [planning context](planning-context.md#writing-and-review). |
| Folder layout | Bounded assessment completed; no physical moves justified | HKO proof, pentagon proof/active figures and scalar API routes were inspected. Misleading traversal restrictions and missing routes were repaired. Broader changes need concrete friction, not another blanket layout audit. |
| Tools and navigation | Named setup checks and substantial cleanup completed | Existing evidence and limits are linked below; investigate concrete remaining obstacles, not an exhaustive cleanup programme. |
| Mathematical code architecture and execution reliability | Concrete source-level candidates identified; not implemented | [Proposed sessions](team-plan.md) separate broad stable-code architecture/migration from launcher/cache reliability, with scientific contract checks before refactoring. Prior navigation checks did not settle these code questions. |
| Author summaries/memory | Detailed design deferred | Revisit as actual writing and feedback reveal useful shapes. Two pilots exist; they are not a rollout template. |
| Data-science scope | Finish-versus-report decisions unresolved; one historical-evaluator wording gap repaired | [Scope/evidence note](datascience-scope.md) separates unresolved frozen-table lineage, checked appendix aggregates and integrated wording qualifications from new research. Additional branch-aware versus nonsmooth comparison is excluded by Jörn's value judgment. |

Layout and authoring investigation can plausibly overlap; this is an agent
planning hypothesis, not an established dependency. Tool readiness does not
complete authoring-workflow discovery. Deferring summaries did not defer that
parent problem.

## Sprint disposition and next ownership

On 2026-09-07 Jörn authorized a roughly 15-minute parallel repo-improvement
sprint, with isolated worktrees and one integration worktree. Integrated results
are reflected above. The main agent owns follow-on selection; the completed
subagent tasks do not assign anyone the remaining thesis work. No research run
or comprehensive audit is queued.

The unlimited-budget sprint is finished; normal-budget work has resumed.
All implementation commits were integrated. Branches `migration/20260907-*`
retain the worker histories after removal of their clean scratch checkouts.
The unmerged process-handoff packet is superseded by global memory, not pending
integration. The changed TeX was subsequently rebuilt on September 11; full visual review
remains open.

The migration purpose is predictable tools, useful navigation and enough
accurate context for fresh agents to avoid costly wrong work. It does not
certify the thesis or settle every research question.

## Completed work: evidence owners

- [Migration review](../docs/migration-review.md): assignment-clarity repair,
  obsolete-guidance removal, scratch preservation/removal and recovery pointers.
- [INSTALL](../INSTALL.md) and [environment evidence](../docs/development-environments.md):
  named environment/Sage checks and their limits; not comprehensive reproduction.
- [HKO author context](hko-author-context.md) and
  [optimizer review route](optimizer-review-route.md): bounded memory pilots.
  [Planning context](planning-context.md#bounded-checks) records what their
  retrieval check did and did not establish.
- [Gradient method](../experiments/dev-gradient-ascent/METHOD-CANDIDATE.md) and
  [package README](../experiments/dev-gradient-ascent/README.md#directory-map):
  algorithm and recovery/support limits after charter/promotion-packet removal.
- [P2 packet](../experiments/sys-datascience/methods/standard-baseline-p2/README.md):
  completed input reconstruction with matching hashes, including unauthorized
  CPU-load incident and resource warning. No rerun queued.
- Global installation/distribution remains DevOps-owned. Jörn separately
  authorized this sprint's live shared-skill edits: memory/skill-design writing
  guidance was refined, not reorganized into a new framework. The remaining
  cross-skill design question lives in global memory
  `~/.agents/memories/process-knowledge-design.md`.

## Not assigned by this map

Broad proof audit, archive publication and administrative verification are not
current migration assignments. A local-maxima-screen rerun is not queued; the
missing historical producer state is already disclosed in the thesis.
Current deadline/submission status remains unestablished here; old pending
administrative notes are not current tasks. See
[project facts](../docs/project-facts.md) for dated scope and submission context.
