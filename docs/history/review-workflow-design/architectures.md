# Review architecture search space

Research design, 2026-09-18. These are unactivated candidate workflows, not instructions for thesis production and not validated detectors. No detector runs were performed for this report. The intended outcome is a review process that finds consequential writing failures, avoids harmful criticism, and recognizes acceptable imperfections at affordable cost. Finding an eloquent criticism is a different outcome.

The design uses the repository's [harness-engineering skill](../../.agents/skills/harness-engineering-v2/SKILL.md). Existing evidence and its transfer limits are recorded in [detector research](../writing-quality-research/detectors.md), [authoring research](../writing-quality-research/authoring-workflows.md), and [local datasets](../writing-quality-research/local-datasets.md). None establishes a generally reliable reviewer for this thesis.

## What is being changed?

The useful search space is larger than reviewer role names. An architecture determines:

- **Information:** source text, preceding context, task requirements, mathematical facts, feedback examples, other reviews; what is hidden and when it is revealed.
- **Unit of work:** sentence, paragraph, subsection, chapter, relation between distant passages, complete reader journey.
- **Operation:** locate, reconstruct, compare, predict, edit, judge, consolidate; these need not share one call.
- **State:** independent calls, sequential state, a shared extracted artifact, or a mixture.
- **Output:** spans, questions, dependency edges, edit scripts, judgments, or narrative review.
- **Selection:** routing, escalation, union, deduplication, adjudication; selection can itself lose the useful signal.

Hypotheses such as attention dilution, habitual review templates, implicit time economizing, and hindsight repair predict different treatment responses. They are not known facts about model internals. A successful intervention may have several explanations: separating aspects also changes prompt length, total computation, expected output, and opportunities to notice defects.

The phrase "one-shot" below means one requested review with fixed supplied material. One Codex task is not necessarily one model request: tool access, retries, continuations, and orchestration can add calls. Final review length does not measure reasoning-token cost. Report observed latency, usage, model/effort, input lengths, tools, and actual continuation counts when available; do not infer hidden reasoning from the review or request access to private chain of thought. Explicit intermediate artifacts below are new task outputs, not claimed windows into hidden reasoning.

The [current official Astra guide](https://developers.openai.com/api/docs/guides/latest-model), inspected September 18, says `temperature` and `top_p` are unsupported and documents recurring style tendencies; it does not establish their training causes. Thus "vary temperature" is not an available Astra treatment on that API. Verify controls on the actual runner before designing their ablation. Model/effort choices must respect session authorization. Distinct prompts and contexts remain available treatment dimensions without pretending they change sampling controls.

## Candidate architectures

Each row changes an operation or its information flow. Brief sketches are experimental contracts, not a complete prompt library. They should share immutable, line-identified source packets, declared reader assumptions, and a common definition of harmful and consequential findings.

### Partition the work without losing the whole

| Architecture | Computation and useful condition | Main failure |
|---|---|---|
| **All-aspects baseline** | Full text → one review: "Identify consequential obstacles to understanding; anchor each to text and explain its reader consequence. Allow no defect." Necessary cheap comparator. | Fluent broad review may omit the important issue. |
| **Repeated generalists** | Same full task → independent reviews → union. Control for extra opportunities/computation before attributing gains to specialization. | Shared blind spots; repetitions can add confident false positives. |
| **Parallel aspects** | Full text → separate inference, motivation, ordering, terminology, redundancy reviews. Useful if breadth competes with depth. | Aspects interact; each reviewer may invent a defect to fulfill its role. |
| **Parallel spans with full-context support** | Each worker owns a span but can consult unchanged chapter context and definition locations. Useful for local density. | Full access does not ensure use; chunk boundaries lose cross-span relationships. |
| **Aspect × span allocation** | Assign targeted cells only where justified, plus a whole-section review. Useful for dense heterogeneous material. | Cartesian explosion and expensive merger; empty cells cannot imply coverage. |
| **Relationship review** | Review question→answer, claim→warrant, definition→use, promise→delivery pairs, including distant ones. | Edge extraction can omit the very missing relationship; direct-text fallback needed. |
| **Hierarchical traversal** | Chapter purpose first, subsection contribution second, local execution third, then return to the whole. | An initially wrong purpose can contaminate all later judgments. |
| **Random boundary shift** | Review overlapping windows at shifted boundaries; compare defect recovery. Useful for boundary sensitivity. | Duplicated criticism grows; context remains artificial. |

### Change sequence and state

| Architecture | Computation and useful condition | Main failure |
|---|---|---|
| **Sequential reader ledger** | Read prefixes in order; retain introduced concepts, active questions, and unresolved references. Next passage updates ledger. | Ledger mistakes accumulate; compression can erase uncertainty. |
| **Frozen next-question prediction** | Before revealing continuation, record what question the reader now expects answered; then compare actual continuation. | Simulated reader may not represent Jörn; unusual but sound organization may be penalized. |
| **Independent locate → judge → report** | Locator returns candidate spans without a polished explanation; judge checks each in context; reporter renders accepted findings. | Locator remains a recall ceiling; judge can rationalize supplied suspicion. |
| **Judge before explaining** | Commit location/severity/no-issue judgment before prose justification. Useful to test whether generating criticism creates commitment. | Early commitment can itself anchor the explanation. |
| **Observation before verdict** | Extract explicit entities, questions, and logical links first; judge only afterwards. | Extraction may silently repair the text, concealing reader burden. |
| **State reset between passes** | Second reviewer receives text and focused questions, but not first verdict. Later compare. | May merely duplicate; independent contexts do not ensure independent errors. |
| **Escalating depth** | Cheap pass routes only uncertain/high-consequence cases to focused review. | False reassurance suppresses escalation; audit a sample of apparently clean text. |
| **Reverse dependency audit** | Start from the conclusion and trace required prerequisites backwards, then compare their presentation order. | Detects logical gaps more directly than lived reading friction. |

### Make a different artifact

| Architecture | Computation and useful condition | Main failure |
|---|---|---|
| **Deterministic shared preparation** | Extract stable paragraph IDs, definitions, labels, citations, explicit references, equation boundaries once. | Bad extraction is a common-mode failure; render/source mismatches need checking. |
| **Explicit question map** | Each paragraph: question addressed, contribution, next unresolved question; mark "unclear" rather than inventing one. | A capable model may manufacture coherence absent from the prose. |
| **Proof warrant map** | For each nontrivial implication, identify the warrant and whether the reader has it. | Mathematical competence can supply missing warrants too charitably. |
| **Terminology timeline** | Track first mention, definition, prerequisites, first use, ambiguity. | Useful for a narrow family; formal definition order alone is not readability. |
| **Blind reconstruction** | Reader reconstructs the argument from text; independent comparison checks intended mathematical content and omitted distinctions. | Reconstruction can succeed despite unacceptable effort or charitable repair. |
| **Edits as detection** | Produce the smallest justified edit script, including no change; infer candidate issues from edits and judge them separately. | Editing may introduce stylistic churn; an edit is not proof of a defect. |
| **Deletion counterfactual** | Remove a sentence/paragraph; ask what reader-relevant function was lost. | Local redundancy may serve necessary global orientation. |
| **Reordering counterfactual** | Compare original ordering with a plausible alternative while holding sentences fixed. | Better alternative does not establish original FAIL. |
| **Diagram or table reconstruction** | Convert prose into a dependency diagram, contrast table, or worked example; compare lost/added information. | The conversion's own errors can masquerade as source defects. |
| **Plain-language restatement** | Restate without the source's rhetoric, then compare claim strength and meaning. | Terminological precision can be destroyed; no assumption that simpler is always better. |
| **Reader-action trace** | Mark places requiring lookup, inference, remembering a distant fact, or reinterpretation; judge necessity separately. | Counts of reader actions are not quality scores; mathematical reading legitimately requires work. |

### Change evidence, comparison, and adversarial pressure

| Architecture | Computation and useful condition | Main failure |
|---|---|---|
| **Candidate reranking** | Compare original with controlled variants for a named reader consequence; include ties and both-fail. | Familiar style preference and position bias; relative winner can still fail. |
| **Known-good preservation review** | Include accepted imperfect text; ask whether proposed edits are necessary and safe. | Overfitting to visible exemplars; accepted once does not mean all features desirable. |
| **Context ablation** | Same span with/without legitimate preceding context, never relabeling one as the other. Measures dependence on context. | Context removal deliberately creates issues; cannot report them as defects in the intact source. |
| **Authorship/label masking** | Hide prior PASS/FAIL, agent names, and selection reasons until predictions freeze. | Does not remove stylistic familiarity or contamination from previous exposure. |
| **Reader-assumption perturbation** | Vary a declared prerequisite and see which objections change. | Unreasonable reader assumptions can manufacture failure. |
| **Argument against the criticism** | Defender seeks textual evidence that resolves each candidate issue; independent judge decides. | Rhetorical persuasiveness and knowledge outside the permitted reader state can rescue bad prose. |
| **Targeted synthetic corruption** | Insert naturalistic X into an accepted span; test detection with clean twin held out. | Artificial insertions may have giveaway cues and be much easier than natural failures. |
| **Invariant perturbations** | Swap candidate order, anonymize notation consistently, change nonsemantic formatting; inspect judgment stability. | Some formatting legitimately changes readability; define invariance narrowly. |
| **Critique-completion ablation** | Review with short spans only versus full prose criticism. Tests whether reporting demands suppress coverage. | A bare span cannot reveal whether a reviewer noticed the right problem. |
| **Competing explanation trial** | For a observed failure, generate predictions from attention, reader-model, and reporting hypotheses; choose a discriminating input change. | Hypotheses can be flexible enough to explain everything unless predictions freeze first. |

## Shared preparation: useful, but not a universal semantic bottleneck

Stable source extraction, immutable hashes, paragraph IDs, and factual prerequisite lists can be prepared once and reused. These save repeated mechanics and support precise merging. Their correctness can be checked without claiming prose quality.

Generated summaries, outlines, and reader profiles are different: they are interpretations. Passing a single semantic outline to every reviewer may cheaply propagate the same mistaken account of the chapter. Compare reviewers receiving raw text against those receiving the shared interpretation. Keep raw-source access and record which findings depend on prepared artifacts. A deterministic outline made from headings is not a validated account of the argument either.

Parallel work is suitable when outputs do not depend on one another: aspect reviews, span reviews, alternate representations. Prefix-state review and locate→judge have real sequential dependencies. A hybrid can run several sequences in parallel, but costs and error dependence remain observable questions, not benefits implied by a graph shape.

## Does adding specialist A make issue X solved?

It creates a reusable capability conditional on X's scope, routing, execution, and adjudication. The useful invariant is "known X cases are now covered by a retained capability and regression examples," not "X cannot recur."

For each added specialist, measure on frozen examples:

1. **Marginal coverage:** consequential true defects found by A that the existing union missed; stratify natural versus synthetic and text type. Do not count near-duplicate phrasings as extra coverage.
2. **Marginal harm:** unsupported criticisms, destructive suggested edits, and false reassurance. Local human comments are selective; unannotated spans are not automatically negatives.
3. **Correlation:** which important defects every reviewer misses, including when all produce lengthy plausible reports. Vote count is evidence only after examining dependence and reliability.
4. **Retention through merging:** which raw findings survive, are weakened, or disappear. A rare true observation must not be removed simply because nine others failed to notice it.
5. **Cost and routing:** critical-path latency and total usage, including repeated full inputs and judgment of all candidate findings. A cheap review can create expensive adjudication.
6. **Regression and neighboring cases:** accepted text, repaired text, natural X variants, and nearby legitimate constructions. Avoid turning a successful X detector into an indiscriminate prohibition.

The merged artifact should retain source hash, span IDs, reviewer/workflow ID, precise alleged reader consequence, supporting text, and status (candidate / supported / disputed / resolved). Preserve raw findings. Deterministically co-locate overlapping spans before semantic deduplication; do not fuse distinct allegations because they share a paragraph. A merger can group and prioritize, but disputes need their evidence retained. Expressed confidence is not automatically calibrated, and majority agreement cannot substitute for source-grounded adjudication.

The portfolio needs a global relationship review even if every local chunk has specialists. Chapter promises, repeated motivations, prerequisite order, and shifts in claim strength can cross all chunk boundaries. Routing can use observable text properties, but route ambiguous cases to multiple workflows or a general pass; audit apparent negatives so the router does not become an untested recall ceiling.

## Minimal contracts for informative pilots

These sketches deliberately avoid a long universal rubric. Each candidate consumes the same immutable source and declared reader background.

**Baseline:** "Identify obstacles that materially impair this reader's understanding. For each, provide exact span IDs, the obstacle, and its consequence. Distinguish necessary mathematical effort from avoidable reconstruction. Permit no finding. Do not rewrite the passage."

**Prefix reader:** "You have only paragraphs 1–k. Record the active mathematical question, concepts whose meaning is available, and the next explanation you expect. Do not guess the unseen continuation." Freeze the artifact, then reveal k+1 to an independent comparison stage.

**Relation reviewer:** "For these claim–warrant or question–answer pairs, determine whether the relationship is recoverable from the presented text and stated prerequisites. Locate missing information rather than supplying it. If extraction omitted a relevant relation, record it."

**Edit detector:** "Propose the smallest edit, deletion, or move that removes a concrete reader obstacle. No change is allowed. State the information that must be preserved." A separate judge sees original and proposed edit, checks preservation and whether the obstacle exists; the proposal itself is not the verdict.

**Merger:** "Preserve every distinct allegation and its origin. Group duplicates without strengthening them. For contradictions, retain both and identify the textual question that decides. Do not infer safety from absent objections or a majority vote."

## Keep the search open without endlessly brainstorming

Begin with representatives of different computation graphs, not ten prompt variants in the same family: all-aspects baseline, matched-cost repeated generalists, parallel aspects, prefix reader, and one relation/edit architecture. These are proposals for later experiments, not authorization to run them now. A small initial comparison should identify missing mechanisms, not crown a universal winner.

Rebrainstorm when any of these occurs:

- All architectures miss the same human objection: consider missing reader context, wrong objective, or familiarity rather than more specialization.
- A specialist finds defects that merge discards: redesign aggregation before adding more detectors.
- Gains vanish on another section type: test the design procedure on that type instead of forcing prompt transfer.
- Longer or more elaborate reviews add prose but no important coverage: change output or operation, not just effort.
- Synthetic gains fail on natural defects: inspect corruption artifacts and replace the training/evaluation examples.
- The portfolio becomes more negative on accepted text: investigate false-positive accumulation and readiness threshold separately.
- Repair makes detector scores improve while human acceptance does not: inspect the changed passages and redesign the measured property.
- A shared prep artifact determines everyone's conclusions: add raw-text and alternative-prep branches.

Three particularly underexplored bets are **frozen prefix predictions** (remove hindsight access), **reviewing relationships rather than spans** (make global coherence explicit), and **minimal edits as candidate detection** (change the operation from producing critique to demonstrating a removable obstacle). Their failure modes differ enough to be informative. None is a substitute for testing on Jörn's actual accepted and rejected prose.
