# Independent DS trajectory findings — frozen initial diagnosis

Prepared 2026-09-15 without reading the earlier REPORT.md, BRIEF.md, assignment, or another independent auditor's findings. Audit only; no scientific reruns or guidance changes. Applied session-log parsing and harness-engineering-v2. These are local DS findings, not an attribution of the whole-thesis FAIL.

## Evidence and coverage

The existing corpus identifies DS root **01a09fd9-2ffe-7681-850d-139298baf0c3** plus five children: readiness, opportunities, synthesis, CEM resume and CEM review. I read the root index and focused descendant indices, then checked consequential root messages directly in raw JSONL. The indexed cutoff is 2026-09-15T16:44:07Z. This is adequate to identify decisions but not exhaustive verification of all tool execution. Initial spawn bodies and many inter-agent messages are encrypted; their contents cannot establish what instructions workers received. Inherited messages must not count as independent observations. No complete lineage guarantee beyond the provided inventory is asserted.

Citation shorthand (all line numbers are raw JSONL lines):

- **D**: `/home/agent/.codex/sessions/2026/09/14/rollout-2026-09-14T12-16-46-01a09fd9-2ffe-7681-850d-139298baf0c3.jsonl`.
- **O**: `/home/agent/.codex/sessions/2026/09/14/rollout-2026-09-14T12-26-30-01a09fe2-1ca7-7a80-afc1-997a33c5c9da.jsonl`.
- **S**: `/home/agent/.codex/sessions/2026/09/14/rollout-2026-09-14T12-26-45-01a09fe2-53e1-70c3-ba7a-8f571c37210a.jsonl`.
- **R**: `/home/agent/.codex/sessions/2026/09/14/rollout-2026-09-14T13-05-36-01a0a005-e58b-76f3-9df5-73c3edab3e2d.jsonl`.

Focused reproduction: enumerate JSONL lines; select `payload.type == message`; join `payload.content[].text`; inspect the cited line. Indices are discovery aids, not source truth. Numerical results below are reported and independently reviewed within the historical trajectory, not recomputed in this audit.

## 1. Research allocation silently narrowed the purpose

**Observed:** The useful initial opportunity portfolio prioritized tangentialization, rotation allocation, and CEM against IID (O139). Source recovery already knew a strong ridge association and the distinction between generic ridge mean and product ridge sum (S125, S183). Nevertheless, only after Jörn's 13:57–14:00 feedback did the agent explicitly distinguish finding high-ratio bodies from using statistical patterns to recover mathematical understanding (D1606, D1624, D1631, D1656). D1671 reports an existing geometric decomposition omitted from the narrative. A small immediate check then produced a geometric identity and a counterexample to a global monotonic interpretation (D1690; R429).

**Diagnosis:** Selection among scientifically different search algorithms is not enough to ensure diversity in research purpose. The omitted frame affected both prioritization and the reader's ability to understand why experiments existed. The agent's admission supports this diagnosis, but does not prove all original motivation was absent from its context. Encrypted prompts prevent such a claim.

**Earlier alternative:** At the initial portfolio choice, compare one search intervention with one explanation-oriented task using the already observed ridge signal: derive the descriptor on tractable shapes, test a tempting global claim, and use that result to choose the next experiment. That could have supplied mathematical narrative earlier at modest cost. It need not replace CEM or presume explanation work always dominates candidate search. The quick later success makes this plausible, not proof that the theorem or a PASS would have followed earlier.

**Proposed intervention/test:** Put a short project research-purpose statement and the distinction between search and explanation in the next sprint's starting brief. For each substantial new direction require the decision it can change and the existing observation motivating it; permit explicit supersession of the old frame. Test on the historical ridge evidence plus several available search pilots: does an independent agent identify a cheap geometric interpretation task and compare its value without being ordered to select it? Score quality of alternatives and resulting explanation, not keyword compliance. Overhead: a few sentences at portfolio changes, no per-tool checklist.

## 2. Readiness estimates became approval friction rather than an execution decision

**Observed:** The adapter was reported controlled by 12:42 (D344), but the first scientific wave remained awaiting an async decision at 12:46 (D417). Jörn challenged an evaluation ceiling presented as a resource estimate (D432). After several estimate exchanges, Jörn authorized action and challenged the extra permission request and serialization of independent implementation (D512). The agent explicitly identified the gate as its own interpretation of material scope change (D517). The first pilot's generation/evaluation took 1.91/1.41 seconds (D564); the complete rotation pilot reported 13.29 seconds of evaluation (D662).

**Diagnosis:** For small reversible tasks, the decision overhead was poorly matched to cost and existing authority. It is reasonable to measure an unfamiliar evaluator and check geometry controls; the evidence does not justify removing that work. The failure was turning bounded implementation into a repeatedly revised approval proposal and confusing ceiling with expectation.

**Earlier alternative:** After the adapter result, estimate edit/run/report work separately from machine runtime, execute the two bounded implementations concurrently, and serialize only evaluator calls if needed for interpretable timing. Stop/escalate on a real incompatibility or material expansion.

**Test:** Give an agent a ready evaluator, explicit bounded-task authority, and one genuinely expensive or scope-changing alternative. Success is direct execution of the small pilots with concise estimates, while correctly escalating the material alternative. Measure human interventions and time to usable result. No claim here assigns the full elapsed discussion interval to waste; concurrent useful work continued.

## 3. Retry of an execution-incomplete method was a consequential success

**Observed:** Opportunities ranked CEM as an incomplete scientific comparison, not a failed method (O139; `.git/codex/ds-first-wave/opportunities/report.md:139–168`). D719 preserved that distinction when a new run hit unsupported coordinate bounds. D801 records matching proposal checks for both methods, preserving the failed run and disclosing the change. D916 reports CEM beating IID on both candidate quality and measured generation-plus-evaluation time in all three runs. R212 independently checked 21 batches, nine updates, frozen hashes, charging and construction costs. Earlier reviewer S544 had caught evaluator-only time curves; final reporting included construction costs, which dominated runtime (D861).

**Diagnosis:** This is exactly the kind of approach-switching worth preserving: diagnose whether evidence is procedural or scientific, repair an inexpensive shared interface, and conduct a valid comparison. The resulting positive evidence is more valuable than merely reaching the old incumbent faster. It remains a three-seed descriptive result, not broad superiority.

**Intervention/test:** Preserve three result states in experiment metadata: scientifically informative result, execution-incomplete attempt, and untested proposal. In a portfolio test include an execution failure, a valid negative, and an IID-equivalent method. The agent should consider a bounded repair of the first, not automatically rerun the second, and explain why the third adds no independent search bet. Use existing metadata rather than creating another ledger. Review full costs when time efficiency is claimed; do not demand full cost modeling for every scientific claim.

## 4. Source-fidelity review did not test the narrative contract

**Observed:** The first eight-page chapter passed bounded source review after fixing wrong comparator wording and deduplication ambiguity (D1531, D1581; R360–400). Jörn then found the two-page sample comfortable at sentence level but structurally chaotic (D1606). His response explained the discovery sequence the chapter needed. The review scope was transparently bounded, so this is not evidence of a dishonest PASS; it is evidence that the chosen review did not cover a decisive quality dimension. Later narrative review caught that height/rotation pilots did not isolate a ridge-to-capacity mechanism (R473), a useful distinction beyond numerical fidelity.

**Earlier alternative:** Make the opening review test both reading comfort and whether the section sequence exposes the research question, motivation, evidence, interpretation and unresolved question. A paragraph or section-map review could have elicited the central correction before expanding the chapter. Human feedback was requested at 13:09 but became lost in Jörn's task queue and was resubmitted at 13:50 (D819, D1434); useful independent work continued meanwhile. Thus the delay is not simply author inactivity.

**Test:** Give a reviewer a factually accurate, fluent chapter with experiments ordered by implementation chronology and a contrasting evidence-to-question sequence. Require a specific reader obstruction and evidence-based revision. Assess whether it detects the first without inventing factual defects. Overhead: one short narrative check before expansion, combined with existing review rather than another standing reviewer.

## 5. Coordination needed completion semantics, not more reminders

**Observed:** The DS root ended while three workers were active (D173), and Jörn manually restarted it (D183). The correction was recorded, yet it later ended again under a hold-steady instruction (D2280, D2283). Eventually explicit ownership transfer resolved the lane, with no outstanding task or branch merge (D2335). The coordinator also acknowledged that an earlier inbox escaped notice amid human feedback. The record establishes attention demands and lifecycle ambiguity; it does not establish an exact amount of blocked worker time or a general runtime delivery guarantee.

**Alternative:** Distinguish waiting for a dependency, holding artifacts fixed, and completing the lane. An agent can keep files fixed while either doing useful independent work or explicitly handing ownership back. Repeated apologetic guidance is weaker than a completion criterion naming who owns outstanding work.

**Test:** Exercise active workers, pending human feedback, hold-steady with no independent task, and accepted ownership transfer. The agent should remain available for dependencies and close after explicit transfer without inventing activity to fill time. A compact handoff acknowledgement and pending-item owner suffice; avoid a permanent heartbeat/reporting ritual.

## Priority and uncertainty

For day two, prioritize purpose recovery and an early narrative/whole-artifact contract, then remove redundant approval gates and make lane completion explicit. Preserve bounded empirical comparisons, failure-versus-result distinctions, matched accounting, source review, and immutable human review copies. These successes should not be deleted in a blanket attempt to reduce process.

The DS slice cannot establish how much its research work displaced critical thesis writing, whether a different initial portfolio would have achieved PASS, or causal shares of the deadline miss. The root began with a handoff directing the three DS starts; their authorization and rationale predate this sampled tree. The much larger claim that “all experiments should have stopped” is unsupported here. Nor can these traces isolate model, reasoning-effort, tool, or prompt effects. The recommendations are hypotheses for cheap behavioral tests, not measured treatment effects.
