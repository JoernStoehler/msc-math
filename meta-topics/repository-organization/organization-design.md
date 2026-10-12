# Repository organization as an agent working interface

Design reference, 12 October 2026. This report compares choices for a mixed
research repository of approximately 3,541 tracked files. It is for agents
adapting organization to real work. It does **not** activate mandatory guidance,
select a repository-wide migration, approve expenditure, or establish scientific
or human acceptance. Current authority boundaries remain in
[repository organization](README.md).

## How to use and update this report

Start with the task matrix below. Identify the task's information needs, the
agent's existing context, and whether time, total expenditure, or missed evidence
is the dominant concern. Read the corresponding structure and lifecycle choices;
inspect the actual topic before choosing an intervention. A known source does
not require reading this report first.

A local decision can record five things: task; observed retrieval or maintenance
problem; chosen workflow and supporting structure; rejected alternative and why;
condition that would change the choice. Update this report when repeated use
changes those tradeoffs, preserving dated evidence and limitations. A different
filename preference is not grounds for reopening the whole design. Keep task
assignments in coordination and scientific conclusions with their evidence owners.

## Working recommendation and uncertainties

Use **topic ownership with variable internal organization** as the candidate
working direction: related definitions, proof development, code, producers,
interpretation, figures and artifact manifests can belong together. Within a
topic, organize active exploration around questions and attempts, stable knowledge
around reusable results, and independently shared capabilities around their own
contracts. The selected thesis remains a distinct consumer. Shared foundations
and APIs keep one owner with explicit references from their users.

This fits Jörn's stated preference for mixed-artifact topics. It does not establish
that moving current files is worthwhile. The existing domain tree plus topic
entry points is a viable low-movement implementation; physical topic bundles are
an alternative where repeated joint editing or retrieval makes proximity valuable.
Conventional Cargo package internals and build dependencies remain practical
constraints. One folder tree cannot simultaneously encode logical prerequisites,
research history, data flow and thesis exposition.

Access frequency, concurrent editing, effective model costs, latency and search
noise are unmeasured. The recommendations below follow plausible mechanisms and
local observations, not a measured ranking of repository designs. Jörn’s direct
feedback on 12 October 2026 refines search delegation, lifecycle and resource choices below;
those are attributed experience-based judgments, not measured universal effects
or newly activated instructions.

## Task, context and resource priorities

| Scenario and priority | Workflow and tools | Supporting organization | Main cost/failure; change condition |
| --- | --- | --- | --- |
| Known symbol or narrow repair; owner has context | Direct `rg`, caller search, source reads | Clear names, nearby contracts, outward dependency links | Delegation adds explanation and compression costs. Broaden discovery when aliases or consumers are unclear. |
| Broad search would add substantial low-signal material to parent context | Scouts return relevant sources, reasons and coverage limits; parent follows with recommended reads or a cheap targeted query | Topic entry points, artifact-role inventory and source pointers | Compare marginal parent-context savings with briefing/integration cost. Prefer direct retrieval when essential relevance context is costly to convey, the search is tiny, or remaining results are already high-signal. |
| Discovery blocks the next action; time to a usable result matters | Proactively compare direct completion, parallel branches and brief human help; integrate provisional findings before consequential use | Bounded topic/role routes with source pointers and explicit search coverage | Include briefing, orchestration, integration and failed-work recovery. Provisional extra hits can be tolerable; consequential omissions cannot. Prefer direct work when orchestration outweighs useful parallelism. |
| Extend a proof or shared implementation | Context-rich owner follows prerequisites and consumers; fresh review targets a consequential implication/interface | Canonical definitions, typed dependencies and source/evidence links | An incomplete prerequisite package can conceal a wrong premise. Inspect decisive sources and disputed edges directly. |
| Fast-changing exploration; write effort dominates | Local owner records inputs, interpretation, uncertainty and bounded checkpoints | Question/attempt bundles with lightweight current map | Continuous polished accounts become stale and consume research effort. Consolidate when repeated consumption becomes valuable. |
| Expansion unlikely soon given relative priorities; topic may be unfinished | PM-informed disposition, then either substantive consolidation for expected readers or a lightweight checkpoint | Current priority rationale, source map, unresolved work and conditions for reopening | Queue position forecasts future work; it does not certify results or justify deletion. Do not spend heavily on polishing without likely consumption. |
| Stable topic reused across many threads; repeated orientation dominates | Maintained substantive account routes targeted source checks | Result bundles, caveats, prerequisites, producer manifests, dated scope | Stale accounts spread the same omission. Reopen the synthesis when its claim, dependency or evidence changes. |
| Triage many heterogeneous artifacts | If categories are unsettled, calibrate serially; parallelize independent batches and integrate disputes | Inventory with role, consumers, provenance and disposition reasons | Early parallelism multiplies category mistakes. Low priority can justify leaving active expansion; it does not justify deleting evidence. |
| Reproduce, migrate or assemble a release | Cross-topic owner checks interfaces; bounded workers handle independent units | Producer/output/consumer edges, external manifests, move maps and resolving references | Locally correct units can fail when assembled. Check actual consuming commands and promised artifacts. |

Jörn’s 12 October experience is that briefing a search subagent usually costs only
slightly more than formulating the searches directly, while filtering its return
can greatly reduce the originating thread’s context burden. The useful comparison
is marginal briefing and integration cost against the low-signal search material
kept out of parent context, not delegation overhead alone. Predictable exceptions
are lengthy essential context for judging relevance, tiny searches, and searches
already narrowed to high-signal results. This is his experience-based guidance;
its effect size and total resource savings have not been measured here.

The parent can directly read sources recommended by a scout, or try a cheap
full-text query based on its own guess after a miss when re-briefing is not worth
it. Source pointers and coverage limits preserve that option. Delegation can lose
a decisive exception, so compression does not replace consequential source checks.
A maintained account amortizes orientation but cannot anticipate every question.
Accounts, scouts and direct reads therefore compose; not all retrieval needs to
be outsourced.

Fresh agents can challenge inherited framing but repeat orientation and lack tacit
constraints. Context-rich owners preserve local history but may inherit mistakes.
Freshness is useful for independent discovery or a disputed inference; a fresh
reader given only one summary still shares its information omissions. Model roles
should follow observed task performance. Cheaper scouts for simple enumeration
and stronger agents for subtle dependencies are candidates, not measured savings.
Count worker usage, parent integration, tool latency, duplicated reads and repair;
short final answers and agent counts are poor cost proxies. Parallelism can reduce
elapsed time while increasing total expenditure.

Jörn’s 12 October resource preference is to give greater weight to **wall time
to a usable-quality result**, within authorized resources. He reports that agents
have underweighted this: hurried work can fail and waste its entire elapsed time;
sequential work can overlook available parallelism; and prolonged orchestration
can take longer than completing the task directly. The implication is to consider
delegation and parallelism proactively, without treating orchestration as progress
or rushing the work needed for correctness. Compare end-to-end routes, including
briefing, integration and likely rework, rather than minimizing any one component.

His illustrative comparison was an hour spent orchestrating versus 45 minutes of
direct completion, potentially 15 minutes with effective delegation, or 20 minutes
including five minutes of useful human help. These are examples, not measured
speedups, budgets or deadlines. A brief concrete request for help can be worthwhile
when it resolves an obstacle that would otherwise consume substantial time; it is
not a routine gate or an instruction to transfer ordinary agent work to Jörn.
This answers the earlier latency-versus-total-effort preference without authorizing
unbounded expenditure or making quality optional. Choices still depend on the
actual task, dependencies and resources; the preference does not make more agents
inherently faster.

## Physical patterns, nesting and granularity

| Pattern | What it makes easy | Where it fails |
| --- | --- | --- |
| Existing domain layout plus topic maps | Preserve build conventions and current paths; introduce cross-domain routes cheaply | Joint topic work still traverses locations; maps need upkeep. |
| Small flat topic | Enumerate all meaningful objects and open neighbors | Large listings force filenames to encode too many distinctions. |
| Topic → question → attempts | Compare routes, negative results and the current frontier | Established results can remain buried beneath their original motivation. |
| Topic → result → proof/evidence | Reuse a claim with its assumptions and support | Premature result names can imply stability and obscure exploration. |
| Shared method/library with linked uses | Maintain a reusable contract and find its consumers | Organizing every question by implementation hides scientific purpose. |
| Pipeline stages | Reconstruct inputs, transformations and outputs | Execution order is not explanation order; shared inputs make a graph. |

Nesting is useful when a subtree forms a coherent search or ownership boundary.
Depth without a discriminating question merely adds navigation. A topic may mix
patterns: `topics/example/attempts/route-a/` and
`topics/example/results/profile/` are illustrative paths, not selected locations.
A result reused beyond its original attempt can gain a canonical location with a
pointer from that attempt; copying the authoritative proof creates drift.

Choose file granularity by independent reuse and change. Keep a statement and
short proof together when nearly every consumer needs both. Split a producer
contract from a long exploration log when their consumers and invalidation rates
differ. Larger documents can remain efficient with stable sections; many tiny
files impose repeated titles, references and opens. Separate files reduce edit
collisions but do not prevent contradictory changes to a shared definition.

For example, a substantive topic account could have headings `## Established
result`, `## Assumptions and exclusions`, `## Why the argument works`,
`## Computational evidence`, `## Existing extensions`, and `## Open routes`.
These explain enough to choose the next source; a directory listing alone does
not. Use only sections that carry actual content. Logical relationships still
need explicit statements: proximity of a certificate and theorem does not say
whether the certificate is necessary, supplementary, or historical.

## Retrieval: ordinary tools and possible improvements

| Capability | Concrete use | Benefit and limitation |
| --- | --- | --- |
| Paths, `rg`, bounded source reads | `rg -n 'symmetric.partner\|arbitrary.partner' docs formal experiments thesis` | Low setup; searches source text. Vocabulary mismatch and implicit relationships still hide work. |
| Structured selector | Proposed query: `kind=result topic=regular-products relation=uses:<definition-id>` | Deterministically narrows recorded relationships. Missing/stale metadata causes false exclusion. |
| Section-aware reader | Proposed request: `read(path, section="Existing extensions", include_ancestors=true)` | Transfers relevant prose with identity and enclosing context. Definitions outside the section need explicit links or follow-up. |
| Fuzzy/semantic content search | Proposed query: “extensions where one factor is unrestricted” | Can bridge vocabulary differences; needs content indexing, freshness checks and evaluation of false matches/misses. Exact mathematical/code identifiers remain useful. |
| Bounded dependency reader | Proposed request: `prerequisites(<result-id>, depth=2)` or `consumers(<producer-id>)` | Builds task-shaped packages and exposes missing edges/cycles. Unbounded traversal floods context; incomplete edges cannot prove completeness. |

The proposed selector/readers and semantic search describe capabilities, not installed commands. The
[existing inventory](inventory/README.md) supports kind/area filters and text search over
records; [its builder](../../scripts/build-repo-inventory.py) searches serialized
item records, not the underlying corpus. It is a useful routing layer with declared
coverage, not full-corpus semantic retrieval. A zero-result inventory query is not
evidence that the repository lacks a result.

A small section reader could return path, section identifier, source revision,
selected text and omitted surrounding sections. A dependency reader additionally
needs typed edges, traversal rules, unresolved references and provenance. Stable
headings are more robust interfaces than memorized line offsets; readers still
need to handle duplicated headings and changed section boundaries. Introduce
such tools for recurring retrieval friction, not merely because the corpus is
large. Naming and a repaired link may solve the immediate problem more cheaply. Repeated
vocabulary failures after those repairs would make semantic retrieval worth comparing;
it does not replace typed provenance or exact source inspection.

Natural language carries mechanisms, caveats and failed-route rationale. Metadata
makes repeated distinctions queryable. A small candidate record is stable identity,
owner path, purpose, document lifecycle and typed references such as `uses`,
`produced-by`, `supports` or `supersedes`. Record scientific support separately,
with uncertainty. Existing symbols, labels or paths may supply sufficient identity without opaque IDs.
Existing inventory fields already cover many of these needs; avoid creating a second
independently maintained registry. Generate alternate
views from owned records where practical. Enrich metadata only when an actual
query needs the distinction and an owner can maintain it.

Discovery often favors recall: extra hits cost attention, but a missed extension
can cause repeated research. Promotion, retirement and publication require checking
the decisive evidence chain and contradictions, not merely finding plausible hits.
A filter should therefore retain an obvious route to broader source search.

## Lifecycle, retention and provenance

| Maintenance regime | Appropriate situation | Cost and transition |
| --- | --- | --- |
| Continuous small updates | Current owner/source map or contract with clear invalidation triggers | Cheap when changes are local; avoid polishing a full narrative after every attempt. |
| Discrete expansion-to-reading consolidation | Further expansion is unlikely soon and expected consumption warrants a substantive account, even if the topic is unfinished | Pays upfront synthesis and reconciliation cost. Reopen when new results undermine its structure or conclusions. |
| On-demand reconstruction | Rarely used material or rapidly changing exploratory branches | Avoids speculative maintenance; repeated reconstruction is the signal to retain a better account. |

Jörn’s 12 October clarification makes relative priority a practical lifecycle
signal: a PM agent with a project overview can forecast whether another thread
is likely to expand a topic soon, considering competing priorities and likely new
arrivals. His example of place “99” illustrates a distant priority, not a numeric
threshold. A topic can leave active expansion without reaching maturity or
scientific completion. Record this as a revisable forecast; changed priorities,
new evidence or a new consumer can reopen it. Whether to polish then depends on
expected reading/reuse: low-priority, rarely consumed work may need only its
status, sources, unresolved questions and disposition rationale preserved.

These regimes can coexist in one topic. A continuously maintained source map can
route to a dated synthesis and historical attempts. Consolidation means making
current understanding consumable: reconcile competing claims, explain why routes
failed, identify authoritative sources and reveal unresolved obligations. It does
not mean every attempt succeeded or the mathematics is accepted. Document states
such as exploratory, consolidated and superseded are separate from mathematical
support, computational verification, manuscript inclusion and Jörn's judgment of
readability. A polished page or successful build supplies none of the latter by
itself.

Retain useful negative findings, original purposes, important discarded approaches
and provenance where future decisions need them. Keep current entry points concise
and label historical accounts explicitly. Existing project retention constraints
already govern destructive changes; this report creates no deletion permission or
new approval gate. Git can recover tracked versions but is not a convenient sole
index of costly research rationale, nor a substitute for external dataset retention.

An external artifact needs an owned manifest or stable reference tying it to its
producer, inputs/configuration, interpretation and availability/reproduction
promise; [artifact policy](../../docs/artifacts.md) owns those details. A missing worktree
link does not establish lost data. Moves need consumer checks and a discoverable
old-to-new route where references matter. Stable logical IDs help movable internal
sources; externally cited/released paths may require stronger compatibility.
Inspect actual commitments before introducing that promise.

## What the local evidence distinguishes

The [account-to-writing pilot](inventory/pilots/2026-10-11/writing/report.md) found a bounded
agent preference for one new-account passage and avoidance of a stale-pointer
detour. Both authors still read primary sources; total cost savings and human
acceptance were not established. This supports testing substantive accounts for
particular tasks, not requiring them everywhere or attributing the result to HTML
or nesting alone.

The [archival-consumption comparison](../agent-workflow/session-postmortem-2026-10-04/tests/archival-consumption/analysis.md)
found that adding a packet to an already curated baseline increased aggregate
reading and every paired latency. It warns that summaries can supplement rather
than replace source reading. It does not compare curated topic knowledge with an
unselected raw corpus.

The [discovery observation](../../docs/history/coordination/cleanup-design-2026-10-10/discovery.md)
records a useful fresh reconstruction that omitted an existing extension. This is
a concrete recall failure worth addressing; it does not identify folder layout as
the cause. Better aliases, cross-links, a substantive account or a changed search
procedure are competing repairs.

Choose by expected net benefit: better task outcomes and avoided reconstruction
against implementation, upkeep, integration and error costs. Prefer the smaller
intervention when expected outcomes are comparable; substantial restructuring can
be justified when it addresses recurring costs that local repairs leave intact.
Representative comparisons can include claim inspection, extension discovery,
shared-consumer change and reproduction tasks against the current route. Include
missed conditions/dependencies, irrelevant reading, authoring/upkeep and integration
costs alongside elapsed time. Keep source access comparable. One improved task
can support a local choice but does not establish broad performance gains or general
model/resource advantages. That evidence limit is not a universal requirement to
run an experiment before changing organization. This report itself authorizes no
migration or evaluation programme.
