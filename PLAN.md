# Project plan — Probing Viterbo's conjecture

31 October 2026 · [Short status](STATUS.md)

<a id="msc-purpose"></a>
## What we are trying to achieve

**Thesis:** a supported, readable MSc thesis that Jörn can submit and pass.
**Research:** results valuable to symplectic geometers, judged by significance,
conceptual interest and usefulness for further mathematics. Research can succeed
without belonging in the thesis; a new theorem does not repair bad exposition.

The last attributed thesis status is **unsubmitted, with failing preliminary
checks** (Jörn, 23 October). No human verdict on the October whole manuscript was
found in the [check account](docs/preliminary-checks-2026-10-23.md).
The separate 28–29 October research report is delivered and integrated on main.

**Latest decision, 31 October:** Jörn stopped the completeness audit and selected
solution planning. His working judgment is that remaining missed problems may be
independent and of comparable, rather than higher, priority; this is a reason to
proceed, not a completeness guarantee. The existing account is the starting set.
This session owns solution planning, beginning with a five-minute approach report.
No research campaign, manuscript pass or implementation programme is running from
that planning assignment.

**Management audit, 31 October:** independent source audits covered scientific/
computational choices, thesis/delivery and collaboration, alongside repository-
organization comparisons. A fresh reviewer independently selected further source
checks and recovered a structured-data alternative and mixed-directory archive
contradiction; both are integrated below. This session owns main integration.
The audit repairs known omissions; it does not establish exhaustive coverage or
reliable later use of the account.

<a id="absorbed-viterbo-continuation"></a>
**Absorbed continuation results, 5 November:** the t=7/20 HKO certificate and the
other 29 October continuation results from branch `research/viterbo-continuation-20261029`
(tip `7b6463fa`) now exist in-tree as new directories: the
[continuation record](topics/capacity-and-qp/research-continuation-20261029/README.md)
and [handoff](topics/capacity-and-qp/research-continuation-20261029/DECISIONS-AND-HANDOFF.md),
the [t=7/20 certificate](topics/variation-and-local-maxima/hko-cut-family/certificate-continuation-20261029/README.md),
[two-parameter partial certificate](topics/variation-and-local-maxima/hko-cut-family/certificate-two-parameter-20261029/)
and other `*-20261029` dirs under `hko-cut-family/`. They are unreviewed beyond the
branch's own agent audits (no human or external refereeing). The branch's PLAN/STATUS,
dashboard and graph edits were not taken; its "stopped" status text above is not main's
selection. Whether to adopt these results remains undecided.

<a id="solution-planning"></a>
## Solution planning — selected 31 October

**Owner:** this session. **Current output:** the five-minute planning-method
checkpoint; four read-only planning workers returned first proposals for the batches
below. Parent integration, comparative allocation and solution-plan checks remain
active; worker recommendations are not execution selections. The completeness
audit is stopped. Use existing problems and approaches, checking a premise when
it changes a solution rather than reopening the inventory.

**Method:** one coordinator pass across the existing account, bounded parallel
option development, then joint allocation and dependency synthesis. Group by
shared decisions/consumers rather than assign one worker per issue. Every existing
item receives a planning disposition: develop, handled by a shared plan, already
actionable with a short specification, or defer with reason and re-entry condition.
Do not spend equal effort on each entry or silently discard unselected ones.

**Planning priority:** consequential harm/delay avoided; benefit to thesis or
research; number and importance of actual consumers helped; likelihood that more
reasoning changes a choice; and cost to obtain that decision, including human
attention. Compare concrete cases rather than invent numerical scores. Planning
priority differs from execution priority: a large research option may merit a
cheap discriminating plan, while a clear repair may need almost no more analysis.
Keep thesis and independent research value visible; no universal exchange rate
or thesis-first allocation is established.

| Proposed parallel batch | Existing coverage | First useful planning output |
| --- | --- | --- |
| Thesis candidate and acceptance | W01/T01–T04, with A02 personal-verification interface | A route from existing candidate/feedback and scope choices to an assessable manuscript; concrete alternatives, readiness uncertainty and human decisions. |
| Computational interfaces and reuse | Q01–Q07, M02–M05 | Compare consolidation/retention/placement options against representative scalar, ascent and complete-family consumers; specify needed guarantees and assurance, without assuming a Rust port. |
| Reproduction and delivery | E01, A01–A04; thesis batch supplies claim/candidate identity | Workload-specific replay and archive plan plus a separable administrative route. Choose retained inputs, source/history handling and extraction checks; do not make archive completion a hand-in prerequisite. |
| Research alternatives | R00–R04 and the other retained mathematical/empirical possibilities | Comparable short proposals with mathematical payoff, known obstacles, decisive next inquiry and cost uncertainty; deepen only options whose comparison could change allocation. |

M01's knowledge/review remedies accompany the consuming plans above, with explicit
checks of discovery, use and correction propagation. W02 cloud support and W03
resource tooling receive a consumer/dependency assessment before standalone
infrastructure planning; neither automatically blocks other batches. Independent
collaboration needs remain visible rather than being declared solved by this method.

**Unequal depth in practice:** compare the C++/Rust architecture boundary in depth
because it affects several consumers and future changes. Specify the already-chosen
retention of project-authored archive notes briefly, concentrating remaining
analysis on relocation versus classifier behavior. For research, plan a decisive
proof/feasibility inquiry rather than invent a schedule for discovering a theorem.

**Integrated output and stopping:** each developed plan names its intended benefit,
concrete alternatives (including retaining the status quo), recommendation and
reversal condition, first bounded action, inputs/dependencies, cost basis or unknowns,
and what success/failure/inconclusive evidence changes. Stop deepening when the next
consequential choice is actionable; defer detail that only execution can settle.
The coordinator checks conflicting assumptions, genuine dependencies, overlooked
high-value items within the accepted set and total review/integration burden.
A fresh recipient should be able to begin the named step without reconstructing
the alternatives. This checks plan usability, not whether its proposed solution
has already worked. Research and implementation execution remain separate selections.

**First returns to integrate:** thesis proposes a Chapter 8 adoption comparison,
scope decision sheet and candidate-linked personal-verification docket; computation
proposes narrow consumer contracts/adapters before ports, with normal-changing
ascent outside the fixed-normal envelope's immediate coverage; delivery proposes
one required-workload portability pilot and separate archive/submission routes;
research proposes deepening three-hinge and cone-certificate planning, with shorter
cut-family and self-polar feasibility specifications. The research shortlist's
comparative value and its deferrals still need coordinator assessment. These are
planning inputs, not accepted architecture, scope or research priorities.

## Where knowing more would most change the work

This is a proposed order of attention, not an agreed ranking of thesis versus
research. Each section gives the actual evidence, available approaches and next
decision; uncertainty is local to the claim it affects.

| Problem | Why it matters | Useful next move |
| --- | --- | --- |
| [Writing still lacks a dependable route to whole-thesis PASS](#thesis) | More agent-approved revisions can consume another costly human review without fixing the reader's problem. | Use the existing contextual Chapter 8 candidate and matched human feedback to choose the next authoring operation. |
| [Scientific scope and evidence can be confused](#scope) | New results can become accidental obligations; current computations can be mistaken for historical evidence. | Decide what the selected manuscript actually needs, especially data science and later product/HKO results. |
| [Computational contracts and assurance have gaps](#shared-computation) | A convenient migration or green test suite can change the scientific object or cover the wrong evaluator. | Separate scalar, branch, geometry and flow obligations before selecting repairs. |
| [Agents cannot easily find and compare the available C++ and Rust algorithms](#msc-q06) | They can choose an unsuitable route, miss a useful algorithm, or repeat the comparison from scratch. | Build a source-backed capability comparison at the algorithm-selection entry point; test whether agents actually find and use it. |
| [Documentation and code placement can work against ordinary search](#msc-m03) | Useful material can exist but be missed, and new work can duplicate it. | Compare placement in routinely searched module/package locations with the present routing; assess documentation and code separately. |
| [The next research direction has not been chosen](#msc-r00) | Repeating an easy finite search may have less value than a structural route or a new idea. | Compare the surviving mechanisms against the final HKO result, including stopping. |
| [Reproduction and final delivery remain incomplete](#delivery) | A buildable PDF is not a usable archive or an executable submission route. | Resolve actual input/history dependencies and individual administrative requirements. |
| [Known problems do not reliably affect subsequent decisions](#management) | Better records can still leave Jörn doing the coordination and correction. | Examine whether the next selected task uses the relevant knowledge through integration. |

<a id="thesis"></a>
## 1. Produce a thesis that passes

<a id="msc-w01"></a>
<a id="msc-t01"></a>
### Writing and assembled-thesis quality

**Problem.** Human feedback identified vague prose, missing proof bridges,
poor framing and chaotic data-science narrative. Some passages passed and later
repairs exist; neither justifies treating the entire manuscript as accepted.
The [September planning discussion](meta-topics/project-management/planning-context.md)
records the unresolved authoring problem and the condition that a strong passing
proxy precede costly final whole-thesis review. Its later disposition has not been
established here. Deferring summary design did not close the authoring problem.
Developmental feedback is different from final certification.

**Available approach.** The newer
[29 October Chapter 8 candidate](/workspaces/agent-fleet-tools/research/reports/msc-authoring-integration-2026-10-29/README.md)
already demonstrates a concrete operation: recover human decisions, preserve
accepted paragraphs, move sampling detail behind the result, inspect the passage
in context, and revise costs noticed by a fresh reader. It has a patch and paired
six-page excerpts; both scratch manuscripts built to 105 pages. It is not adopted
or human-accepted and does not establish a general writing method. Importantly,
it avoids restoring asides that later human feedback rejected.

The same packet reports an existing Chapter 14 overfull warning in both scratch
builds. Its magnitude and correspondence to the selected source remain unchecked;
inspect those before classifying it as a current failure (`thesis/check-build.sh`
rejects overfull boxes above 1 pt). The older clean-layout finding concerns the
101-page candidate, not those 105-page builds.

**Next decision.** Compare that candidate with the current Chapter 8: its baseline
hash still matches the selected source, checked on 31 October. Decide whether to
adopt the placement change, revise it, or
leave the source. Use that concrete comparison rather than restarting a generic
workflow-design exercise. For subsequent passages, distinguish a missing argument
from an explanation the reader cannot follow. Check chapter prerequisites,
conventions and connections as well as local prose; do not impose a forced single
narrative on the thesis's different contributions.

**Final acceptance.** The recorded standard is a PDF on which Kai would find only
minor problems Jörn could fix in about two hours; borderline is FAIL. The planned
sequence is Jörn satisfied → Kai feedback → incorporate feedback → submit. A fast
Kai review has not been arranged. Before requesting final PASS, resolve the
readiness-proxy condition rather than assuming agent approval satisfies it.
[Evidence and detailed obligations](meta-topics/project-management/thesis-work.md)
retain the already-completed repairs; they should not be commissioned again.

**Route to submission:** scope/evidence and composition jointly support one exact
candidate; readiness and Jörn/Kai feedback determine its revisions. Personal
verification and applicable administrative requirements also feed the submission
decision. New mathematics, complete infrastructure repair and archive publication
are not blanket prerequisites for this route.

<a id="msc-t02"></a>
### Introduction candidate

Candidate `16ca1458` still lacks a substantive reader judgment. The 26 October
reply rejected the old Questions endpoint, not the introduction. Confirm which
candidate should be assessed and use a current conversation route; then decide
accept, revise or discard. This is a bounded framing question within the writing
work, not a separate substitute for whole-thesis quality.

<a id="scope"></a>
<a id="msc-t04"></a>
### Data-science scope and interpretation

**Problem.** Jörn described data science as moderately incomplete; the unresolved
choice is what to finish and what to report as unfinished. The historical
roughly-100-method expectation is not a numerical completion quota. The chapter
needs to explain what the results teach, not merely catalogue methods or caveats.
[Current question map](topics/empirical-studies/sys-datascience/coordination/current-question-map.md)
and [project facts](docs/project-facts.md) own that scope question.

**Evidence to keep distinct:** historical population analyses; restored geometry
and schema for 14,336 bodies; current capacity corroboration comprising 14,289
accepted scalars, 46 wider intervals and one unresolved body. Volume is still
uncertified. Current evaluations cannot reconstruct old execution/cache history.
The historical 39-feature analysis and current 45-feature P2 analysis are different
objects. [Retrospective account](topics/empirical-studies/retrospective-revalidation/README.md).

**Next decision.** Compare the selected chapter's claims with the method ledger
and strongest retained results, including selection/enrichment results rather
than only failures. Choose among a qualified historical account, explicit use of
new corroboration, or a targeted new study that answers a specific remaining
question. Identify the actual scope choice for Jörn; do not ask him to rediscover
the evidence or launch more methods merely to increase coverage counts.

**Results that should guide that choice:** frozen pre-target ridge/concentration
and covariance selectors achieved bounded sub-threshold enrichment. They provide
candidate-selection baselines, not validated routes across one. Harder low-ridge
conditioning did not improve the generic panel, and controlled endpoint paths
reverse the population association. Another ordinary model on the same 45 features
or blind scale-up needs a changed representation, population or question. Compare
against the retained [selector](topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md)
and [endpoint-path](topics/empirical-studies/sys-datascience/methods/ridge-endpoint-path/README.md)
evidence rather than restarting generic method coverage.

<a id="msc-t03"></a>
### Which research belongs in the manuscript?

The [scientific-scope disposition](meta-topics/project-management/thesis-work.md#scientific-scope-disposition)
is the detailed owner. The important distinctions are:

| Material | Current disposition | Remaining choice |
| --- | --- | --- |
| Fixed-pentagon / arbitrary centrally symmetric partner theorem | Integrated in selected Section 11; bounded source/PDF checks exist. | Human exposition acceptance. |
| HKO at most eleven facets | Accepted research; selected chapter still states the ten-facet result. | Whether the stronger result earns a thesis rewrite and its review cost. |
| All-odd rotation and 22 October affine extensions (`d91d01b0`), including 5×7 | Separate research. The 5×7 independent-affine capacity formula is proved, not an open case. | Include, defer or exclude exact statements; general odd-pair affine extension and affine-family ratio optimization remain open. |
| 28–29 October convex-systolic report | Delivered separately; not automatically selected for the thesis. | Any adoption must be an explicit scope decision. |

Prepare the choices against the manuscript's purpose and reader cost, not the
assumption that every stronger result must be inserted.

<a id="repository-maintenance--31-october-2026"></a>
<a id="shared-computation"></a>
## 2. Make the computational repository dependable and useful

The broader maintenance assignment remains unfinished. Its practical outcome is
a repository in which a successor can find a result, understand its guarantee,
choose the right implementation, trace/reproduce the evidence and extend it
without changing the scientific meaning accidentally. The earlier maintenance
attempt and its proposed criteria are preserved in `aa82cf77:PLAN.md`; a helper
repair and passing compilation did not complete that assignment.

**Remaining deliverables:** a prioritized, source-backed defect proposal;
selected repairs with before/after evidence; and continuation-ready accounts
with an accurate remaining defect list. Review the choice of work and its coverage
as well as each repair. A scoped repair can be complete while the broader quality
assignment remains unfinished; one easy green check cannot substitute for it.

<a id="msc-q06"></a>
### Algorithm discovery: C++ and Rust routes have different, hard-to-find properties

| Aspect | Current account |
| --- | --- |
| **Problem — open** | Jörn reports that algorithms implemented in C++ and Rust have different properties which are not documented where agents can easily find and compare them. The problem is discovery and informed algorithm choice, not just an unresolved speed comparison. |
| **Consequence** | An agent can overlook an applicable method, choose one with the wrong guarantees or outputs, misbudget a computation, or re-investigate distinctions already known elsewhere. These are the expected downstream costs; their frequency has not been measured. |
| **Existing support and gap** | The [calculation map](docs/capacity-calculation-map.md) and local contracts contain useful distinctions. For example, general Rust production and the fixed-body Sage/C++ certificate differ in input class, coverage mechanism, reuse and timing boundaries. That example is not an inventory of all C++/Rust algorithms. The existence of these documents has not established easy discovery or a sufficient comparison. |
| **Proposed solution** | Extend the existing algorithm-selection map with a side-by-side account of each relevant implementation: mathematical problem/input class, algorithm, arithmetic and guarantees, outputs, unsupported cases, performance with timing boundaries, dependencies, reuse limits, and source/command. Link it from the entry points agents actually use. Language is one attribute, not the organizing explanation for every difference. |
| **Related approach** | A concise comparison/router can improve discovery. [Algorithm consolidation](#msc-q07) addresses the additional cost of developing separate implementations: assess reusable kernels, shared interfaces and a common implementation language alongside documentation repair. These approaches can complement each other. |
| **Next action** | Inventory the relevant C++ and Rust routes and their callers; identify which distinctions are missing, stale or merely hard to find in the current map. Include the algorithms behind the original report rather than assuming the twenty-facet certificate is its whole subject. |
| **Completion evidence** | From ordinary repository entry points, a fresh agent can find the relevant alternatives, choose for representative tasks, and explain the decisive properties with correct source references. A link check, new table, or benchmark alone does not establish that outcome. |

The [performance comparison](#msc-q04), [ascent-output mismatch](#msc-q03) and
[validation coverage](#msc-q01) are related problems, not substitutes for this one.

<a id="msc-q07"></a>
### Consolidate algorithms to reduce further-development friction

| Aspect | Current account |
| --- | --- |
| **Approach and intended benefit** | Consolidate related algorithms and reusable machinery so future improvements can share implementation, conventions, interfaces and tests. This could reduce repeated porting, parallel bug fixes, duplicated setup and the cost of understanding separate C++/Rust routes. It addresses development friction as well as discovery. These benefits are hypotheses to check against actual callers and planned extensions. |
| **Missing decision rationale** | Jörn reports that agents did not record why C++ was used instead of Rust. The existing language split is therefore not evidence that its benefits outweigh its costs. Recover the rationale where possible; otherwise explicitly reassess the choice. Do not invent a historical reason from plausible performance or convenience arguments. |
| **Options to compare** | Port useful C++ algorithms into the Rust library/experiment structure; consolidate suitable shared kernels/interfaces while retaining distinct algorithm variants; or retain C++ where a demonstrated advantage justifies a maintained boundary. A wrapper may simplify invocation without reducing duplicate implementation work. Deliberately independent controls can remain separate. |
| **Analysis needed** | Inventory the C++ routes and related Rust machinery; identify shared mathematics, differing guarantees, arithmetic, output contracts and implementation techniques. Compare extension effort, integration/testing, deployment dependencies, runtime and migration cost. Language speed, algorithmic advantage and one-off implementation convenience are different possible explanations. |
| **Next decision** | Use the bounded source investigation to propose a concrete consolidation boundary and a representative future change it would simplify. If a material uncertainty remains, choose a small equivalence/portability or matched-cost probe that can decide between the options. The output is a supported consolidation or retention proposal, not another unexplained language choice. |
| **Success condition** | A selected change becomes easier to implement and verify through the shared route while preserving required guarantees, outputs and useful independent checks. Record why retained exceptions earn their cost. A single language, fewer files or scalar agreement alone does not establish success. |

Frozen certificate packets retain their historical source/evidence identity;
new shared implementations need correspondence checks rather than silently
replacing the bytes supporting an accepted result.

**Investigation result, 31 October:** the bounded source/commit search found no
recorded C++-versus-Rust rationale. It identified two distinct C++ capabilities:
complete HKO maximizing-chart enumeration and fixed twenty-facet cycle-coverage
replay. The former was missing from the calculation map and is now exposed there.
Neither uses an inspected C++-specific mathematical dependency. This makes a Rust
port plausible, not automatically worthwhile. The
[source assessment](meta-topics/repository-organization/architecture-migration.md#cpp-rust-consolidation)
compares shared contracts, traversal kernels, ports and complete-family/KKT
interfaces, and retains the existing local case for separate readable/optimized
implementations. Next choose a representative extension and compare the concrete
work each boundary would require; the language/architecture decision remains open.

<a id="msc-m03"></a>
### Put documentation where agents already look

| Aspect | Current account |
| --- | --- |
| **Approach** | Put maintained contracts, algorithm comparisons and development explanations in locations agents already consult through normal filename/symbol search, package READMEs and module documentation. Use names and terms matching the questions callers actually ask. A new routing instruction is not a substitute for suitable placement. |
| **Benefit over the status quo** | Reduce missed alternatives, repeated orientation and dependence on remembering project-specific link chains. Relevant guarantees and rationale should be encountered while selecting or editing an implementation, rather than discovered only after a wrong choice. |
| **Evidence and uncertainty** | The crate README already points to the calculation map, yet the reported C++/Rust discovery friction remains. This does not prove that all current locations are wrong; it shows that a pointer's existence is insufficient evidence of discoverability. Actual default search behavior and the places it misses need checking. |
| **Options and tradeoffs** | Co-locate implementation contracts with modules and expose the cross-route comparison from package-level documentation; retain deeper proof/evidence at its domain owner. Compare relocation with concise local documentation pointing to one maintained detailed account. Avoid multiple authoritative copies that drift. |
| **Next action and check** | Give a fresh agent an ordinary algorithm-selection or extension task without prescribing paths. Observe its search terms, roots and files; compare those with where the needed facts live. Place the missing facts at the appropriate encounter points and repeat an equivalent task. Judge retrieval and correct use, not compliance with a newly taught navigation ritual. |

**Client-interface update, 1 November 2026:** Jörn selected standard agent-client
frontends (T3 Code, Codex TUI, Claude Code TUI, ChatGPT.com Chat). The custom
file-viewer service and delivery links were retired; use clients' built-in file
views. Renderer source and history remain preserved. This changes the delivery
surface, not the underlying source/documentation owners.

<a id="msc-m04"></a>
### Put code where agents already search for reusable implementations

| Aspect | Current account |
| --- | --- |
| **Approach** | Place reusable algorithms and kernels in conventional package/module locations, with discoverable exports, tests and examples. Keep experiment-specific selection policies and frozen evidence in their owning packages. Choose locations by actual consumers and search habits, not language alone or an aesthetic folder scheme. |
| **Benefit over the status quo** | Make existing implementation easier to find, import, test and extend; reduce accidental reimplementation and repeated edits across buried copies. Conventional build/test discovery can also reduce bespoke setup work. |
| **Evidence and uncertainty** | Reusable Rust already has crate homes, while other machinery lives in topic development and certificate packages. A concrete search defect was the legacy call-site recipe omitting `topics`, where consumers had moved. That recipe is fixed; it does not establish that changing all code locations is necessary or that documentation alone resolves the broader issue. |
| **Options and tradeoffs** | Move reusable machinery into an existing crate/module; retain a conventional standalone package when its dependencies or contract justify separation; expose a stable interface where moving code would create harmful coupling. Preserve independent controls and historical producer identities. Relocation can break imports, scripts and evidence paths, so identify actual consumers first. |
| **Next action and check** | Trace where an agent seeking the relevant implementation searches, then map reusable versus experiment-specific pieces and their callers. Propose concrete source/destination pairs and interface changes. Verify that an ordinary extension finds and reuses the implementation and that affected build/test and reproduction routes still work. A directory move alone is not success. |

These placement approaches complement [algorithm consolidation](#msc-q07):
better locations can help without merging implementations, and shared code can
still be hard to find if placed poorly. The
[repository organization account](meta-topics/repository-organization/README.md)
owns existing placement and retention contracts.

<a id="msc-q01"></a>
### Production validation covers less than the legacy campaigns

The broad property and orbit-recovery campaigns exercise legacy routes; production
`capacity_4d` has narrower independent anchors. This is a documented assurance gap,
not evidence of wrong production values. **Next:** map the six property families
to the production input/output contract and add the most consequential missing
independent checks. Keep legacy implementations as controls. Shared-kernel
agreement is not independent verification. [Testing contract](docs/algorithm-testing.md).

<a id="msc-q02"></a>
### “Exact geometry” does not mean one validity contract

Generic and optimized constructors differ: duplicate rows can pass exact facet
checks; the generic boundedness scan omits triples involving row zero and has no
separate rank-four test; checked-geometry fields can later be mutated. No corrupt
scientific result was established by the audit. **Next:** trace callers needing
bounded/irredundant inputs and test exact cases that distinguish the contracts.
Then choose between proving a shortcut sufficient, replacing it with the complete
criterion, or adding a validated wrapper. Preserve deliberate raw-kernel policies.
[Geometry contracts](topics/polytope-geometry/README.md).

<a id="msc-q03"></a>
### Ascent needs branches, not just a scalar capacity

The shared ascent state still consumes the legacy `OrbitSearchResult` and
tolerance-selected active branches. Production minimizers, especially sparse
product closure winners, do not promise all derivative branches. A mechanical
replacement can agree on capacity while changing the optimization method.
**Next:** inventory fields used by one scalar-only, one general-ascent and one
product-ascent caller. Migrate scalar callers separately; assess exact minimizer/
action-window plus one-word adapters for general ascent. Product migration needs
an accepted subdifferential contract. Preserve intentionally heuristic models and
frozen controls. [Call-site dispositions](topics/capacity-and-qp/development/docs/legacy-callsite-inventory.md).

There is already mathematical support to assess: the agent-reviewed
[active-pair result](topics/empirical-studies/discovery-design/night-20261005/geometry-discovery/active-restriction.md)
preserves the complete active HK-weight convex hull using **all** active sparse
closure pairs. A [finite quadratic envelope](topics/empirical-studies/discovery-design/night-20261005/geometry-discovery/finite-product-envelope.md)
covers tied/dense optimizers in an open fixed-normal product chamber. First match
these hypotheses and complete-tie outputs to the selected caller's derivative
needs. These results are not human-accepted thesis promotion; normal changes,
singular circuit births and nonproduct perturbations retain separate obligations.

<a id="msc-q04"></a>
### Performance reports do not yet support the reported C++/Rust comparison

This is the performance subquestion of the broader [algorithm-discovery problem](#msc-q06).

The available C++ twenty-facet program checks integer cycle coverage using
pre-certified Sage exclusions and leaves. Its few-second replay excludes roughly
360 seconds of curvature checks and 69 seconds of leaf checks. General Rust
production supports at most 16 facets and 100,000 candidate cycles. These are not
matched workloads. Separately, a July 26-input transformed-body comparison found
a 9.25× production slowdown with expensive exact KKT fallback.

**Next:** recover the original comparison's programs, inputs, guarantees and timing
boundaries. Compare equivalent kernels or end-to-end certification tasks; inspect
existing fallback instrumentation before adding counters. Symmetry, pruning and
separated certificate checking may be transferable ideas regardless of language.
Do not trade away a guarantee and call that an equivalent speedup.
[Calculation map](docs/capacity-calculation-map.md) ·
[dated costs](topics/capacity-and-qp/development/performance/CURRENT_COSTS.md).

<a id="msc-q05"></a>
### Exact flow search still has theorem-to-runtime obligations

The idealized regularity theorem does not certify every runtime singular-case
decision. Incidence/sign consistency and bounded irredundancy are partly caller
obligations; finite-orbit regularity is not a globally certified runtime predicate.
Finite QP scalar comparisons do not establish complete word recovery.
**Next:** match each selected thesis/API claim to the propositions in the
[implementation ledger](topics/flow-graphs/implementation-contract.md). Keep the
conditional contract, close a specific implication, or expand singular support
only where a consumer needs it. Exact arithmetic alone does not close this gap.

<a id="msc-m02"></a>
### Finding and reusing the research is still a substantive quality question

The guide and inventory exist, but the physical census is frozen at 10 October
and semantic coverage is selective. Recent concrete corrections exposed the
5×7 result, the twenty-facet certificate route and P2's retained inputs. Working
links alone do not establish understanding. **Next:** ask a fresh successor to
choose the right route for those cases and explain its limits from the source.
Use failures to choose further account/interface repairs.
[Research guide](docs/research-guide.md) · [inventory](meta-topics/repository-organization/inventory/README.md).

For reuse/refactoring, the [architecture analysis](meta-topics/repository-organization/architecture-migration.md)
offers KKT assembly, geometry, traversal and instrumentation candidates. Check
current callers and [completed migrations](meta-topics/repository-organization/inventory/migration-results-2026-10-12.md)
before doing those extractions again. Retained legacy code can be a necessary
independent control rather than debt to delete.

<a id="msc-m05"></a>
### Consolidate understanding, not just files

Repeated orientation can persist after code and documentation move to better
locations: an agent still has to reconstruct what a result means, how it is
supported, which attempts failed and what remains useful. The existing
[organization comparison](meta-topics/repository-organization/organization-design.md)
offers distinct approaches:

| Approach | Benefit over repeated reconstruction | Cost or condition for choosing it |
| --- | --- | --- |
| A substantive topic/result account linking statement, mechanism, assumptions, implementation and evidence | Reuses understanding across writing, research and code work; makes important negative results and abandoned alternatives discoverable. | Maintain it when claims/dependencies change. A list of files is insufficient, but duplicating every proof is unnecessary. |
| Consolidate a topic when active expansion becomes unlikely and future reading is valuable | Produces a useful handoff before the original context disappears, even when the research is unfinished. | For rarely consumed material, retain a lightweight checkpoint instead of paying for a polished synthesis. Queue position is not scientific completion. |
| Explicit producer → output → consumer and claim → support relationships | Makes affected consumers and evidence invalidation visible when a source changes; reduces missed updates after migrations or new results. | Reuse the existing inventory/owner records; add typed metadata only for a real query, rather than maintaining a second registry. |
| Direct source search plus bounded retrieval scouts | Keep low-signal search material out of the main worker's context while preserving source pointers and decisive exceptions. | Direct reads are preferable for tiny/high-signal searches or when relevance requires expensive context transfer. Compare integration cost and wall time to usable output, not worker count. |

**Next:** select a topic being reused now, such as algorithm consolidation, and
identify the explanation or dependency that the current account forces a new
agent to reconstruct. Improve that account and compare an actual successor task.
The earlier account-to-writing pilot showed a bounded reader preference, not a
measured overall cost reduction; another archival-consumption comparison found
that an extra packet increased reading and latency. More documentation is not
automatically less work.

If ordinary naming, placement and full-text search still miss relevant material,
the same comparison retains **section-aware, structured/dependency and semantic
retrieval** as candidate tools. They can address vocabulary mismatch and repeated
cross-file queries, but require maintained indexes/relationships and checks for
false exclusion. They are proposals, not installed capabilities. The current
inventory searches its records, not the full source corpus; zero inventory hits
cannot establish that a result is absent.

<a id="msc-r00"></a>
<a id="next-research-choice"></a>
## 3. Choose valuable new mathematics

<a id="msc-evidence"></a>
### Starting point: the delivered result changes the search

The [56-page report](topics/capacity-and-qp/convex-systolic-optimization/README.md)
establishes HKO's full `GL(4)` orbit maximum, a twenty-facet body with ratio
`1.0519141388294875…`, nearby seventeen-facet ascent by seven specified cuts, and
a first-order obstruction for at most two characteristic-invariant affine hinges
with all old rows retained. Seven-cut minimality is only within the fixed
nonnegative-profile cone. These results do not settle arbitrary twelve-facet
local maximality or the maximum along the cut family.

The report also contains the sharp six-simplex bound, all-dimensional
two-paraboloid difference-volume theorem and strict truncated-parallelogram
all-partner bound. The four-simplex numerical bound was already known from
Haim–Kislev's 2019 thesis. Scoped agent checks and exact replays support the
report; external novelty and human acceptance are separate questions.

**Selection problem:** no retained comparative value argument establishes that
stopping the October campaign was optimal. Recompare the following routes using
the final results, not the earlier ranking. Develop new alternatives alongside
useful known work. Current costs and resource availability need a fresh observation
when a programme is selected; old quota balances and deadlines are irrelevant.

<a id="msc-r01"></a>
### Stronger HKO descendant or an explanation of active branches

The exact `H_(3/10)` beats HKO; the maximum along `H_t` is unknown. Retained
seven/eight-label branch probes suggest a tractable first inquiry. Use feasible
branch bounds to reject unpromising parameters, then seek complete exact coverage
only for a worthwhile survivor. A feasible upper capacity bound cannot certify
a gain. The fixed-`.3` verifier hardcodes targets/counts and is not a parameter
verifier. [Continuation packet](topics/variation-and-local-maxima/hko-cut-family/README.md).

<a id="pre-reset-proposal--29-october-awaiting-current-review"></a>
One existing proposal screens both retained words at `t=7/20` and `2/5`, checking
closure, normalization, positivity, rank and objective/volume consistency. Its
45–65 minute estimate was uncalibrated; execution is not established. The expired
29 October deadline gives it no current priority. Reuse the four-case design only
if it still answers the selected question.

<a id="msc-r02"></a>
### An all-size rank-four Pfaffian inequality

The inequality is proved through seven generators and for any size with a
degree-one/two row; nonproduct eight-generator equality exists. The unrestricted
problem remains open, with no established EHZ consequence. Seek an all-size
rank-preserving reduction or exact counterexample. The all-direction projection
inequality is false; its maximum-radius repair is equivalent to the original
problem. More small graph enumerations alone do not supply the missing mechanism.
[Restart packet](topics/empirical-studies/discovery-design/desk/zonotope-research/rank-four-restart-20261029/README.md).

<a id="msc-r03"></a>
### Fewer new facets for nearby HKO ascent

Three characteristic-invariant hinges could narrow the gap between the accepted
at-most-eleven-facet local theorem and seventeen-facet ascent. First derive the
three-profile volume/occupation interface and a bounded exact search or structural
dual. The two-hinge obstruction only concerns first variation with old rows
retained; it is not a general twelve-facet theorem. Three-hinge implementation
readiness is unknown. [Two-hinge proof](topics/capacity-and-qp/convex-systolic-optimization/sections/hko-two-hinge-appendix.tex).

<a id="msc-r04"></a>
### Fixed pentagon with an arbitrary asymmetric partner

Sharp symmetric and restricted asymmetric results do not settle the global
ratio/optimizer question. Reuse the existing **18-variable translation-hull
reduction**, at most 30 vertices and compact competitive domain; the ten-direction
restricted problem is already solved exactly. Compare a verified implementation
of the applicable Jung–Ahn exhaustive algorithm with a structural contact-stratum
proof. Published real-RAM complexity is not a practical runtime promise.
[Available algorithms](topics/products/pentagon-arbitrary-partner/translation-cover-algorithms-20261019.md).

The parallel-slide inner optimizer already had a twelve-start rehearsal with no
improvement; repetition needs a changed discriminator, not a convergence claim.
The [block-Hessian lemma](topics/products/pentagon-arbitrary-partner/block-hessian-lemma-20261022.md)
restricts nondegenerate regular minima to at least ten independent contacts;
degenerate/curved strata remain a completeness obstacle.

The separate general-matrix reduction is a seven-variable, 610-row cone-positivity
problem. Compare exact constraint-product/PSD certificates with a feasible negative
witness reconstructed as a genuine all-ten-template partner. Ambient inertia
`(3+,4−)` defeats naive PSD, not cone positivity; determinant-zero points occur
inside the cone, so boundary-only singular treatment is insufficient. Costs and
success probability remain unpriced. [Disposition and options](topics/empirical-studies/discovery-design/night-20261005/comparison/final-research-disposition.md).

### Other live possibilities, not a closed shortlist

- **Odd polygon products:** general odd-pair tensor containment and the 5×7
  affine-family ratio optimum remain distinct open questions after the proved
  5×7 capacity formula. [Products](topics/products/README.md).
- **P* facet birth:** accepted direction patches do not settle all-direction F11.
  [Variation](topics/variation-and-local-maxima/README.md).
- **Beyond two paraboloids:** a new shadow-localization argument is needed;
  more samples repeat the existing obstruction. **Beyond six simplices:** the
  ordering-polytope classification is special; seek a reusable mechanism before
  larger enumeration. [Research report](topics/capacity-and-qp/convex-systolic-optimization/README.md).
- **Empirical discovery:** recover the retained structural and selection leads
  and their negative results before proposing another population search.
  [Discovery account](topics/empirical-studies/discovery.md).
  A concrete external alternative is a **symplectically self-polar benchmark**
  using constructions with known capacity, explicit volume information and linked
  author code. First choose the applicable construction, representation and useful
  comparison with current populations. Self-polarity-preserving interpolation is
  another option, but intermediate bodies can have curved boundaries. Correctly
  normalized mean-width capacity bounds, possibly optimized over linear symplectic
  maps, offer interpretable descriptors with unknown cost/sharpness. These are
  retained literature-based proposals, not implemented or independently validated
  here. [Construction, caveats and bound normalization](topics/literature/open-thesis/geometry-landscape.md).
- **Optimizer methods:** the retained seven-policy comparison is a finite-budget
  historical-heuristic ranking; continued endpoint improvements prevent interpreting
  plateaus as local maxima. Reuse the traced runner to compare gradient sampling,
  full-oracle bundle/trust-region, full MADS and nonsmooth BFGS under matched charged
  compute if this research is selected. Decisive trials remain absent. Jörn's
  recorded decision is that this comparison must not delay submission even an hour.
  [Runner and alternatives](experiments/dev-gradient-ascent/optimizer-runs/README.md) ·
  [comparison contract](experiments/dev-gradient-ascent/optimizer-comparison/README.md).

<a id="delivery"></a>
## 4. Reproduce, submit and preserve the work

<a id="msc-e01"></a>
### Long-horizon reproduction

The thirteen-figure producer/copy/PDF chain succeeded for the frozen 101-page
candidate. That does not replay original numerical production or establish a
current-PDF result. Historical execution receipts, LICCA machine/timing details,
clean-environment coverage and authenticated data retrieval remain incomplete.
Some replay wrappers require old Git objects and home-cache layouts absent from
a plain source archive.

**Next:** extend the existing [reproduction inventory](docs/reproducibility.md)
by result: exact source/input, historical or current schema, comparison criterion,
environment/access dependency, and unrecovered evidence. Restore retained P2
tables using the [checked runbook](topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/README.md)
before considering feature recomputation. Recover logs before proposing new runs.
For no-Git replay, compare frozen historical source fixtures with a narrow history
bundle and explicit input paths. A new run can corroborate a claim but cannot
manufacture an old execution receipt.

The successful second-environment thesis build after the `lmodern` repair remains
credited in the reproduction inventory. For required workloads, retain a locked
replay recipe (including relevant transitive dependencies/browser revision)
alongside a latest-compatible route. Start with one required workload and its
accepted output comparison, using the existing
[coverage proposal](docs/history/thesis-agent-review-20261022/opportunity-reproduction-map.md).
Unconstrained Python dependencies in some producers leave long-horizon drift open;
this does not erase the successful build or require rerunning every producer.

<a id="msc-a03"></a>
<a id="msc-a01"></a>
### Individual submission requirements and official completion

The actual registration/receipt, Bescheid/Merkblatt, operative deadline, title,
copies/binding, medium/channel, forms and English-language consent are not all
established. The listed regulation includes a colloquium; applicability and
completion/exemption for Jörn remain unknown. **Next:** fill one case-specific
checklist from retained records, then prepare a consolidated request for the
missing individual evidence. Do not repeat registration or infer a new deadline.
[Official-source check](docs/history/official-requirements-check-20261023.md).

Reuse the existing [unsent inquiry draft](docs/history/thesis-agent-review-20261022/pruefungsamt-anfrage-ENTWURF.md)
after correcting its overly broad AI-permission wording and authority assumptions
against the later official-source check. Separate retained-record answers from
questions genuinely requiring the examiner or Jörn; the historical draft is not
ready to send verbatim.

Submission receipt, grade recording and any applicable colloquium completion are
different outcomes. Determine whether later certificate application belongs in
the project scope. Manuscript PASS is necessary to the intended submission route
but is not an institutional passing grade.

<a id="msc-a02"></a>
### AI declaration and personal verification

University-level guidance did not settle the chair/examiner-specific interpretation.
The [disclosure](thesis/ai-use-disclosure.tex) also records incomplete personal
verification and final review. **Next:** recover applicable instructions, list
the actual unverified arguments/assertions, and prepare the concrete validation
work for Jörn. A policy interpretation or agent review cannot supply his personal
verification or signature.

<a id="msc-a04"></a>
### A usable continuation archive

The accepted direction is a continuation archive and one Zenodo record; a deposit
is not established. Rights/data disposition, reviewed PDF/source identity and
portable execution remain open. `scripts/build-release.py` checks names/bytes
without preserving Git symlink/executable semantics, so its verification can
pass while extracted interfaces are unusable.

**Next:** choose include/exclude/link/mode policies for actual release consumers,
including external skills and third-party material; resolve bulk-data access;
test selected commands from an extracted archive without Git or workstation
caches. Prepare metadata and bind the exact candidate before an authorized
publication. Archive release and university hand-in are separate obligations.
[Archive checklist](meta-topics/thesis-submission/archive-closure-checklist.md).

**Concrete data choice:** `covariance-rho-frozen-packet` remains `release:false`
in `artifacts/registry.json`. Choose the full packet, a sufficient subset, or an
explicit authenticated-R2 dependency according to whether the consumer needs
compact-result verification or row-level inspection/regeneration. The
[archive inventory](docs/history/thesis-agent-review-20261022/opportunity-archive-inventory.md)
retains this distinction; exclusion alone is not scientific invalidity.

**Policy reconciliation, 31 October:** the rights audit now follows the accepted
content-based retention policy and implemented licenses. Ordinary administration
is not categorically excluded; Software Heritage is optional and normally omitted,
not an assumed history-preservation dependency. LICENSE paths are repaired.
Payload-specific rights decisions and extracted-archive checks remain open.
In particular, `scripts/build-release.py` currently rejects every tracked path
under `topics/literature/sources/`, including project-authored analysis and indexes.
Choose relocation or content/path classification before release; do not delete
useful notes to satisfy that blocker. The rights audit now explicitly preserves
such notes and includes canonical `topics/` research material.

<a id="management"></a>
## 5. Reduce collaboration failures and wasted work

<a id="msc-m01"></a>
### Accurate records must change decisions

The [original project-repair report](docs/history/coordination/20261003-project-repair-user-report.md)
names forgotten problems/fixes, failure to notice changed conditions, unnecessary
delegation to Jörn and repeated context reconstruction. The first management
update was itself rejected as a mess despite independent approval. It does not
establish a solved tracking problem.

**Approach:** follow an actual selected task from problem selection through review,
repair and integration. Check whether its owner used the relevant prior finding,
retained the parent objective and updated the disposition when evidence changed.
If findings disappear during synthesis, compare aggregation of the existing
reports; if editing fails despite correct criticism, improve the editing step.
Another universal instruction or extra reviewer is not automatically the answer.
[Existing approach comparison](meta-topics/agent-workflow/review-workflow-design/decision-map.md).

The [retained knowledge-intervention account](docs/coordination/problem-knowledge-interventions.md)
offers three distinct remedies: **consumer retrieval at the decision** when
relevance is easy to identify; **task-specific context supplied by the assigning
agent** to reduce missed discovery/reconstruction (with omission/stale-copy risks);
and **change-triggered reconciliation** of affected consumers when premises change.
On one real decision with a prior failure and later correction, distinguish failure
to find knowledge, failure to apply retrieved knowledge, and failure to propagate
updates. Choose the remedy for the observed stage; these are not validated general
interventions.

### Discover worthwhile work before scheduling it

The repeated omissions here concern what enters the work account, not only how a
known task executes. The [shared orchestration handoff](/workspaces/agent-fleet-tools/research/reports/orchestration-handoff-2026-10-31.md)
retains planning, scheduling and prepared execution as a proposed role split;
task-preparation and inadequate-verifier repair ownership remain unsettled. Its
successor transfer was rejected, and no accepted successor is recorded there.
Assess an unhinted context-to-work proposal against consequential known omissions
and their value cases. Separately assess execution through return, continuation
and integration. Executing research merely to demonstrate startability does not
test whether the research was worth selecting.

### Prepare useful reviews and validate their remedies

The [shared review account](/workspaces/agent-fleet-tools/PLAN.md#reusable-review--29-october)
records reviewers correctly detecting a defect but recommending unsafe alternatives;
a separate candidate validator caught them. Review-brief preparation and independent
checking of consequential remedies are concrete options alongside aggregation and
editing repair. Retain the requirements, resolved target, findings, proposed fix
and integration result on a consuming review. Judge preserved requirements and
human correction burden, not approval counts. Source bloat was repaired and bounded
checks exist; improved general usefulness and the current-version comparison remain
open. This does not establish a need for another universal review gate.

**Human cost:** recoverable facts, timing management and integration belong with
agents. Ask Jörn for his actual judgment, taste or authority with the relevant
object already prepared. An accepted message submission is not delivery, and a
reviewer's approval is not evidence that the subsequent work succeeded.

### Current views and continuity

PLAN and STATUS own current work; the registry dashboards are historical.
The external Pages overview is stale for F18, F19 and the cloud-support incident;
this session has no Page-writing route. F17's context-reset diagnosis is
release-specific and current behavior is unverified. F02/F03 describe older
source/handoff incidents, not automatically present defects. Reconcile the
affected view when accessible and check the relevant current route before
reusing a mitigation. [Friction records](meta-topics/project-management/friction/README.md).
The feedback CLI remains disabled; no monitoring is running from this plan.

Existing remedies matter too: [F15](meta-topics/project-management/friction/F15.md)
was resolved by removing duplicate reports and an invented acknowledgment task;
[F16](meta-topics/project-management/friction/F16.md) uses a compact human view
with agent-owned synchronization, with later routine use still unverified.
For [F17](meta-topics/project-management/friction/F17.md), recover lost discussion
and accepted corrections as well as checking current runtime behavior: restarting
alone does not restore that context. Judge these approaches by total navigation,
maintenance and recovery burden, not by whether another summary exists.

<a id="msc-w02"></a>
### Cloud support: end-to-end outcome still unachieved

The stopped October attempt did not deliver source/setup, interaction, lifecycle,
result return, integration and the separate browser objective as a usable whole.
A Rust check, browser connection or reviewed prompt closed only a component.
On a renewed selection, compare existing components in one useful end-to-end
task before building more infrastructure; recover already-specified surfaces
instead of asking Jörn to choose them again.
[Latest local postmortem](meta-topics/agent-workflow/cloud-support-postmortem-20261014/README.md).

<a id="msc-w03"></a>
### Resource awareness and repeated context preparation

The resource-workflow attempt received human FAIL; on-demand accounting and
fork/counter repairs remain useful. The automatic hook was withdrawn for
complexity, not left awaiting approval. Recurring-data and prompt-composition
views were unfinished proposals. Later shared delivery now provides selected-source
response accounting/ledger, work/approach annotations, native/transport interval
queries, failure capture/reload and incremental sanitized Codex/Claude message
search. These are inspected delivery records/contracts, not a fresh runtime test.
OpenCode support, semantic/value attribution, quota/money conversion and broader
legacy/tree/item accounting remain incomplete; the old prompt-composition-view
proposal's disposition is still unknown.
**Next:** choose one actual allocation or repeated-context question and reuse the
[shared evidence package](/workspaces/agent-fleet-tools/docs/agent-evidence/README.md)
on demand. Develop an adapter or broader view only for a consequential unmet query,
accounting for freshness and invalidation. Do not duplicate parsers or revive the
withdrawn hook. [Local historical accounting](scripts/subtree-usage.md).

## Evidence, history and limits

The map draws on the seven topic accounts, thesis obligations/project facts,
targeted code and reproduction contracts, October review records, friction
records and the legacy registry. It is not an exhaustive proof/code audit or a
complete transcript search. Current student records, the original C++ comparison,
uninspected cross-project workflow outcomes and human acceptance remain important
unknowns. The Chapter 8 candidate is an external retained artifact, not main content.

Session logs, expired budgets and prior review receipts are deliberately outside
the live plan: `aa82cf77:PLAN.md` preserves the previous account;
[research reconciliation](meta-topics/project-management/research-discovery-reconciliation-20261029.md)
and the [HKO postmortem](docs/history/hko-research-postmortem-20261029/report.html)
preserve allocation/cost evidence. The [worktree rescue receipt](meta-topics/repository-organization/worktree-cleanup-20261029.md)
identifies unreviewed branch work that may be useful; preservation did not adopt it.
