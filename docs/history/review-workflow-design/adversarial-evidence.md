# Adversarial evidence review — 2026-09-18

Bounded independent review of the current position, recognition/repair reports, frozen prompts, selected raw reviews, repair criteria, original human feedback and content audit. No new experiments or manuscript changes. I did not inspect the complete original conversations or independently validate the content audit. Those limits matter particularly for what context the historical coordinator possessed.

## Judgment

The observations justify withholding authority from the tested reviewers to predict Jörn's acceptance. They do **not yet justify making detector development the main prerequisite for productive autonomous writing**. The strongest unresolved alternative is that authoring needs a faithful editorial brief, and that the system can produce acceptable text from such a brief without independently rediscovering historical objections. Conversely, following a brief demonstrably does not guarantee acceptable prose. These alternatives concern the production route, not the existence of a universal writing faculty.

The current note acknowledges most standard limitations. The consequential risks arise when its cautious descriptions become stronger routing decisions: interpreting a missing objective as poor narrative discrimination, interpreting brief-induced prose as an autonomous bad habit, and requiring a sequence of review benchmarks without comparing its value to producing the next actual chapter.

## 1. The DS screen strongly tests agreement under withheld purpose, weakly tests narrative judgment

The [locus prompt](../../experiments/writing-quality/runs/20260918-recognition/prompts/c002-locus.txt) asks a generic MSc reader to assess research logic using only the sample. It does not supply the actual desired research objective. The [raw response](../../experiments/writing-quality/runs/20260918-recognition/raw/c002-locus.md) reconstructs an intelligible objective: select promising bodies, then modify bodies, compare outcomes. Its explanation tracks the paragraphs and distinguishes population definition, association, selection and interventions. That is positive evidence of some narrative analysis, even though its acceptance prediction disagrees with Jörn.

Jörn's [actual objection](../../experiments/writing-quality/local/evidence/docs/review-evidence/human-feedback/ds-reading-2026-09-14.md) asks for a different organizing objective: statistical discovery followed by geometric interpretation and conjecture. A reader need not infer that unique objective from the text's alternative organizing story. The objection also mentions subsections and difficulty following the existing sequence. Thus it is wrong to reduce the whole disagreement to missing private intent; it is equally wrong to assume the exact-locus manipulation supplied everything needed to test his judgment.

**Bayesian direction:** this lowers confidence that these prompts predict this reader's narrative acceptance. It also weakens a pure failure-to-look-at-the-order explanation. It provides much less discrimination between inadequate narrative skill and correct analysis of the wrong editorial target. Three reviews are repeated responses to one omitted objective, not three independent demonstrations that narrative discrimination is absent. The historical coordinator's miss despite project context supplies additional adverse evidence, but the available record does not establish that the desired organizing objective was explicit and accessible before the judgment.

**What changes my mind:** failure after receiving a purpose brief independently established before choosing the target defect, especially when the reviewer accurately states the brief and still endorses contradictory organization. Success with the brief on a new text would support a preparation fix; success only when given the exact desired paragraph sequence would remain answer-conditioned compliance.

## 2. The HKO disclaimer is partly treatment-induced, not an unsolicited independent recurrence

The [repair criteria](../../experiments/writing-quality/runs/20260918-repair/criteria.md) explicitly say: “Preserve upper-bound derivative vs actual capacity derivative distinction.” The [blind review prompt](../../experiments/writing-quality/runs/20260918-repair/blind-review.md) makes that distinction a named criterion and rewards explicit statements of it. Both authors then write a negative capacity-derivative disclaimer.

The negative wording was their choice; the instruction did not require that form. But calling the disclaimer “unsolicited” obscures a plausible direct cause. It is not two independent observations of an ineradicable authoring habit. A brief that highlights a contrast plus a judge that scores that contrast creates pressure to display it. The earlier human objection was to a different contrast, “not the reverse bound”; no new human verdict establishes that this particular disclaimer is harmful.

There is still a useful concern: correctness-preservation instructions may encourage conspicuous caveats that degrade reading. Its remedy may be better separation of preservation checks from prose requirements, rather than stronger style prohibitions or a new detector.

**What changes my mind:** continued unnecessary negatives when the brief specifies the correct affirmative claim without requesting the rival distinction, or direct human rejection of this exact sentence. Until then, report the actual wording and the confound, not a demonstrated recurrence of a rejected habit.

## 3. The positive repair result is real but smaller than its apparent replication

Concrete changes and preservation checks establish that authors can execute this supplied direction rapidly. The two author variants and criterion-informed judge share the detailed solution. Their agreement provides little additional independent evidence that the direction is sufficient or that the resulting exposition meets the user's standard. Blinding methods does not blind which candidate contains the requested answer.

Moreover, the existing [human feedback file](../../experiments/writing-quality/local/evidence/docs/review-evidence/human-feedback/ds-reading-2026-09-14.md) already records a relevant follow-up: a revised DS opening after the original objection still received criticism for a confusing first sentence and an unnecessary enumeration; a further opening received narrowly scoped approval. These are relayed judgments, not a controlled experiment, but they directly show that supplying the desired narrative can coexist with fresh defects and further useful iteration. The new repair probe adds preserved artifacts and comparison, not the first evidence that informed editing is feasible.

**Decision consequence:** the next human response with greatest relevance is on the DS repair's actual structural adequacy, rather than another small HKO compliance judgment. HKO already passed overall; successfully improving it says little about crossing the rejected-narrative threshold. The review desk should route this, without duplicate requests. A short HKO review remains useful for local preference learning, but should not be counted as evidence that the central DS uncertainty was resolved.

## 4. Do not confuse an unreliable acceptance gate with an uneconomic contributor

The recognition experiment selected known disagreement-bearing cases. It cannot estimate ordinary-workload failure rates, ensemble marginal utility, or the balance of helpful and harmful edits. The observed costs are low. These facts do not justify scaling near-identical reviewers as a gate, but neither establish that buying a few ordinary critiques alongside a different authoring route is wasteful. One useful reference finding may repay several misses if proposed edits remain adjudicated.

The stronger neglected comparison is **source-grounded composition from an agreed chapter purpose versus detect-and-repair of existing prose**. A reliable independent detector is not logically necessary for the former. The retained passing HKO chapter and comfortable DS sentences are evidence that acceptable output is possible under some historical process; reconstructing that process's actual inputs and human intervention could be more valuable than explaining every reviewer miss. This is not a recommendation to rewrite accepted material. Preserve the accepted HKO baseline and investigate the rejected or missing chapter work.

## Next action with distinct consequences

First obtain the already-prepared repair's human judgment through the desk; do not spend another full diagnostic cycle to avoid that decision. The question is whether the DS candidate now carries the intended narrative, and what consequential obstacle remains. This uses existing work and separates three outcomes:

- **DS structure accepted:** supplied direction plus authoring is a viable local production route. Next test one real, previously untested section using only its genuine purpose and factual sources, withholding historical repair solutions. Compare direct composition with critique-guided revision only if that choice remains consequential.
- **Purpose recognized but prose still rejected:** detector discovery is not the sole obstacle. Use the concrete rejection to improve authoring and verify the resulting text; do not declare recognition success a readiness gain.
- **Purpose or evidence account itself rejected:** fix the editorial/scientific brief before testing more reviewers. The proposed target was wrong or incomplete.

If a new controlled probe is wanted, use the same fresh source and the same factual/purpose brief for an ordinary critic and a reader-question reconstruction critic, then pass each resulting critique to an editor. Have the desk assess resulting text without method labels. Include a no-change option. Ordinary criticism succeeding removes the need for a special representation; reconstruction alone succeeding supports that operation; both detecting but neither repairing routes effort downstream; both failing leaves brief sufficiency and authoring capability live. Critique scores alone do not settle this choice.

## Readiness is an operational commitment, not a miniature certification study

Several successful local cycles would increase confidence but cannot supply a whole-thesis PASS rate: the target was selected after observing failures, sections share source/model/style dependencies, and chapter interfaces introduce different problems. Nor is there an evidence-based universal sample count available here. Requiring an arbitrary count risks spending preparation on a weak proxy.

A useful readiness record instead ties each outstanding chapter to an actual source, a chosen authoring route, one demonstrated outcome of comparable difficulty, and unresolved assembly/content obligations. Judge throughput on an integrated representative chapter, including retrieval, editing, mathematics checking, rendering, review and rework. The recorded minute-scale author calls are only one component. The content audit makes technical completion plausible but its uncertain estimates should not become reserve-free scheduling commitments.

My assessment would materially improve after one accepted structural repair and one successful transfer/integration with measured total turnaround. It would worsen if explicit, accurate direction repeatedly yielded rejected prose or if factual-source assembly consumed most of the available run. Neither observation currently exists at the required scope. Resume production as a bounded attempt with named uncertainties when the user wants that attempt, not because a small evaluator benchmark has certified success.
