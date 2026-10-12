# Substantive repository consolidation and migration — proposed execution plan

**Status, 12 October 2026:** Jörn approved this execution programme at
08:59:24 UTC for at most 120 minutes wall time and an additional 30 weekly
quota percentage points from the fresh account-wide 5.0%-used baseline at
08:59:35.924 UTC. The Sol execution coordinator is running bounded parallel
work in this checkout. Stop before 10:59:24 UTC or 35.0% used in the same
weekly window; either limit can leave an honest partial result. Root handles
material strategic exceptions.

## Objective and evidence baseline

Make the existing research repository usable to an unfamiliar agent and
maintainable across future work: substantive topic accounts, canonical proofs
and code, producer-owned evidence, accurate claim boundaries, working consumers
and a navigable selected thesis. Consolidate demonstrated duplication and move
coherent packages into topic ownership. Migrate project-operation material by
its lasting purpose, so short-lived messages and handoffs stop polluting source
search. This is a bulk consolidation of existing work; new research directions,
algorithm-priority choices, thesis promotion and external release are separate.

Use the [inventory](README.md), its declared coverage limits, current source
owners and [organization comparison](../organization-design.md) as inputs.
The [11 October pilot](pilots/2026-10-11/README.md) already integrated shared
geometry incidence/enumeration, the explicit-format invariant-table builder,
and exact-polar/HK word-convention proof packets. Focused checks and independent
reviews passed; these uncommitted edits are the starting state, not work to
repeat. Generic exact, optimized rational and f64 validation policies differ;
the [geometry report](pilots/2026-10-11/geometry.md) records the distinction.
The writing pilot is unpromoted: its [placement note](pilots/2026-10-11/writing/placement-note.md)
shows Chapter 8 content and references a wholesale replacement would lose.

## Proposed layout and retention boundary

This is the target ownership map, refined when source and consumer checks show
the natural package boundary. A topic may contain a normal Cargo package along
with mathematics, code, evidence and interpretation; no top-level
research-versus-library split is required.

```text
msc-math/
  README.md                   route to active subject and operating entry points
  AGENTS.md                   concise loaded instructions
  .agents/skills/             discoverable project skills and helpers
  topics/                     scientific work; substantive account per useful topic
    polytope-geometry/
    capacity-and-qp/
    flow-graphs/
    products/
    variation-and-local-maxima/
    empirical-studies/
    literature/               source collection/search; interpretation links to topics
  meta-topics/                project operation, owned by purpose
    repository-organization/  inventory, migration map, relevant tools
    agent-workflow/           durable workflow knowledge and selected evidence
    project-management/       lasting obligations/decisions and their view
    development-environment/  installation-workflow.md, setup and maintenance
    thesis-submission/        administrative/release work from submit/
  thesis/                     selected reader-facing manuscript
  scripts/                    truly shared tools where cross-topic ownership fits
  Cargo.toml, config, ...     required conventional root build interfaces

/tmp/msc-math/<task>/         specific-agent briefs, exact message reviews, scratch
~/.codex/artifacts/msc-math/  artifacts intentionally prepared for Jörn
~/.cache/msc-math/artifacts/  external research snapshots with durable manifests
```

New exact message-review candidate/context/return packets go to
`/tmp/msc-math/message-reviews/` immediately under Jörn's instruction.
The project skill and other active guidance must be reconciled so future
agents do not refill `docs/coordination/message-reviews/`. For existing
packets, search live references, transfer consequential conclusions to their
scientific or meta-topic owner, and remove disposable copies from active
search. Git history and targeted review suffice for ordinary recoverable
records; do not run a forensic archive audit on every file. Retained
mathematical/code/prose reviews that substantiate a claim stay with that claim
or its evidence owner. Large external data keep producer-owned manifests.

Existing `formal/`, `experiments/`, `crates/` and scattered `docs/` content
move in coherent units after checking dependencies; don't flatten the
scientific graph into identical proof/code/data subfolders. Top-level
`thesis/` remains a distinct consumer. `papers/` is proposed under
`topics/literature/` for source collection and searches, with topic-specific
interpretation at its topic. `submit/` moves under
`meta-topics/thesis-submission/`. `INSTALL.md` content belongs at
`meta-topics/development-environment/installation-workflow.md`, linked from
the root README. Distribute the useful owner map, rationale and contracts from
`ARCHITECTURE.md` to their owners, repair incoming references, then retire
redundant copies. `AGENTS.md` and `.agents/skills/` remain at their
discovery paths as concise interfaces into maintained content. A current
worker dispatch does not become permanent merely because it concerns project
management; preserve obligations an unknown future coordinator needs and let
short-lived status records expire.

## Execution topology

A Sol coordinator owns sequencing, shared state, boundary decisions,
integration and stopping. Topic or area workers own disjoint substantive
outputs and integrate their own local child work and reviews; they are not
extra supervisors waiting on others. Each returns an integrated area result,
exact changed paths, source-backed claims, checks, review findings and
remaining gaps. A shared-interface lane is opened only when actual
cross-boundary changes need a separate owner. Its reconciliation checks APIs,
result identities, references and topic boundaries; new findings may change
a proposed destination. The coordinator resolves remaining overlap and
performs the final assembled check.

Publish bounded disjoint work units from the inventory and start ready units
in parallel. Roll local review and integration as each unit returns, instead
of holding all reviews until a common deadline. Add or rebalance integration
ownership when pending returns or unmerged conflicts show overload; raw
agent count alone does not measure coordinator capacity. Assign file ownership
and a review/stop point before mutation; everyone
shares the dirty checkout and preserves others' edits. Do not impose a fixed
two-worker cap. Use Luna for bounded mechanical splitting, moves, reference
repair, debugging and independent trace checks; Sol for mathematical/API
contract work, source interpretation and semantic integration. A fresh Astra
is useful for genuinely complex alternatives, evaluation criteria or a
disputed implication; it need not score every result or consolidate routine
context. Staged summaries to the Astra root are available for strategic
exceptions, not a mandatory gate after every worker. Worker count responds
to independent scope, live usage, review load and merge throughput, rather
than a preset number.

The lanes can overlap:

| Area owner | Substantive work and review target |
| --- | --- |
| Geometry, capacity/QP and flow foundations | Integrate the completed pilot; reconcile KKT/direct-certificate versus conditional/historical numerical material, duplicate pruning proof and empty-blocker convention; inspect remaining branch assumptions and exact/approximate interfaces. Preserve distinct policies where assumptions differ. |
| Products, variation and local maxima | Explain established results, failed routes and open extensions; repair the known missed fixed-pentagon arbitrary-partner/odd-regular route, with the selected-thesis distinction. Trace certificate and local-variation interfaces. |
| Empirical studies and producers | Inspect inventory's lightly covered branches for actual questions, populations, schema/target contracts, outputs, interpretation and costs. Consolidate reusable methods and clarify historical unknown metadata. Small frozen fixtures check code migrations; no full retained table rerun by default. |
| Project operation and literature | Move transient review/handoff material to its right retention class; build meta-topic accounts from lasting process knowledge; route source literature, installation, submission and root discovery interfaces. Keep the inventory and old-to-new map current. |

No lane owns all cross-topic dependencies. Area owners coordinate
shared crates, formal definitions, data producer contracts and selected-thesis
consumers before changing them. Physical folder moves can proceed where source
ownership is clear; account/proof/code correction can proceed in other lanes
concurrently. Bulk migration and consolidation are the priority, rather than
another global rediscovery survey.

## Phases and parallel checks

1. **Preflight and initial ownership:** record the actual dirty-tree baseline,
active sessions and file owners; read the inventory's area limits and selected
thesis result list. Confirm how wall time and weekly-quota use will be observed
   before accepting a numeric stopping rule. The first authorized mechanical
   task is a small native-snapshot reader that preserves observation time,
   window and reset, plus the account-wide used percentage; validate it
   against fresh native records. Existing subtree token scripts can attribute
   work by model but do not report weekly remaining quota. Freeze a small independent expected
coverage list from thesis main results, inventory families and active entry
points; it must not be generated solely by the migrating workers' output.
2. **Parallel area consolidation:** write or repair substantive accounts,
resolve real duplication and code/proof contracts, classify transient versus
durable material, and move coherent packages. A short area return identifies
canonical owners, evidence, caveats, consumers, tests, reviewer verdict and
unresolved work. Prioritize the KKT/certificate and old review-packet gaps,
while independent product, empirical and meta lanes advance.
3. **Cross-boundary reconciliation:** as units return, compare results for shared theorem
identities, prerequisites, code imports, producers, thesis citations and
   navigation. A shared-interface owner, when needed, can require a topic boundary correction,
not merely fix links to a wrong owner. Integrate one coherent interface change
at a time and recheck consumers.
4. **Independent coverage and regression review in parallel:** fresh Luna
   reviewers trace selected main thesis results from statement and caveat down
   through proof, Rust/Python producer entry point and retained evidence when
   applicable. Compare their returns with the independent expected-result
   list; an author's own account cannot certify trace completeness. Separate
   source reviewers inspect consequential mathematics and code/API contracts.
   Label a gap that predates migration as a **discovered gap**; label a broken
   path or changed behavior introduced by migration as a **regression** and
   repair it within scope. A missing producer chain can also reveal that a
   result is purely analytic; explain that instead of inventing code evidence.
5. **Assembled delivery:** rebuild generated inventory/dashboard views, check
   changed links and source paths, run focused Rust/formal/thesis builds where
   consumers changed, and inspect the assembled account from root navigation.
   Produce a concise old-to-new route, reviewed changes, checks and gap ledger.
   Stop optional testing once material migration risks are resolved.

Review claims are bounded. A successful build checks syntax and references;
agent review does not confer Jörn's mathematical or human thesis PASS. Small
fixtures can show behavior preservation for a changed adapter, not validate
every retained result. Do not rerun expensive producers, certificates or whole
datasets merely to demonstrate a path move.

## Are we done?

The proposed programme is complete only when:

* Every expected topic and meta-topic area has an explicit source-backed
  disposition: retained, consolidated, relocated or deferred with consequence
  and reopen condition. Material inventory coverage holes cannot be silently
  called complete.
* An unfamiliar reader can follow substantive accounts to canonical proof,
  implementation or producer and evidence where the claim requires them, with
  assumptions, caveats and current status visible. The independent thesis
  result traces are compared against their expected coverage list; failures
  are repaired or recorded as discovered pre-existing gaps.
* Moved code, proof, data and documentation consumers resolve and relevant
  behavior checks pass; migration regressions are closed. Scientific gaps
  outside cleanup remain explicit instead of being promoted or hidden.
* Temporary message-review and worker-handoff packets no longer appear as
  active repository knowledge; lasting findings and necessary references
  are preserved at their owners. Root discovery interfaces reflect the
  agreed topic/meta-topic layout.
* Independent reviews of consequential claims/interfaces and an assembled
  integration review are retained with scope and verdict. The coordinator
  hands back the changed tree, old-to-new map, checks and unresolved-gap list.

Meeting a time or quota limit before these conditions yields a **partial
result**, not a completion claim. The coordinator leaves each interrupted
area with an owner, exact state and next dependency. The current thesis's
human PASS, Chapter 8 writing promotion, new research selection and release
are outside this migration's done condition.

## Approved resources, observation and escalation

Jörn approved **120 minutes wall time and +30 percentage points of weekly
quota consumed from the fresh launch baseline**, with no automatic extension.
These are ceilings, not a target or completion forecast. There is no defensible total
runtime/spend estimate yet: inventory counts and old session telemetry
are not workload or billing measurements. The fresh native account-wide
snapshot at 08:59:35.924 UTC showed 5.0% weekly used, a 10080-minute
window and reset epoch 1792312863. The stop threshold is before 35.0% used
in that same window, as well as before 10:59:24 UTC. Native records provide
`used_percent`, `observed_at`, `window_minutes` and `resets_at`; a read-only
watch checks fresh observations. The account-wide delta includes other
sessions; reset timing, stale or missing observations and concurrent
consumption must be handled explicitly. Existing subtree token counts may
help interpret per-model work, but no calibrated token-to-quota conversion
has been established. Account for coordinator, workers, reviewers, retries and
integration, not only the first topic drafts. Monitor aggregate usage and
reserve the final portion of time for
assembly, review and handoff. Stop before either approved boundary. If the
quota signal becomes unavailable or scope threatens the limit, pause
substantive launches and report the uncertainty; do not silently infer spare
capacity.

The Sol coordinator may choose ordinary batches, parallelism and review
repairs inside the approved boundary without
repeated user approval. Escalate to root Astra for a plan failure needing
genuinely complex comparison or criterion design, and to Jörn for changed
scientific priorities, a materially expanded programme, a large additional
resource ask or an unresolved retention decision with real consequences.
Routine source checks, path repairs and recoverable Git-backed cleanup stay
with the area/coordinator owners. Crossing either approved boundary requires
a new user decision; an interrupted unit is handed off as partial work.
