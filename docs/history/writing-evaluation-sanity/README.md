# Sanity check: acquire useful writing judgments at the right scale

Research proposal, not activated workflow instructions. 18 September 2026. No new human judgments or authoring experiments were collected. This report reviews the actual Candidate D feedback and current root planning, rather than literature (a separate agent owns that search).

## Main finding

Root has treated a costly *bundle*—read a full section, discover unknown problems, explain them, and give an overall verdict—as the normal unit of human feedback. That choice makes evaluation look scarcer than it may be and favors elaborate agent surrogates prematurely. Jörn's proposed five-second sentence reaction is a credible competing unit that should be tried immediately. Its usefulness does not depend on certifying a chapter.

The evidence is one approximately 1,000-word reviewed section, taking roughly 14.5 minutes, with Jörn estimating 80% reading, 15% typing and 5% interface work. This supports the efficiency of the current longform interface; it does not estimate the cost of making a binary judgment on a sentence. Five seconds is currently Jörn's plausible estimate, not an observed end-to-end measurement. Tool delivery, noticing, context switching, reading and answering may change it. None requires a new interface project to investigate.

## Assumptions that should change decisions

### 1. Human feedback need not contain a diagnosis or repair

Recognizing that a sentence is intolerable can be much easier than explaining why or producing its replacement. Root repeatedly packages feedback as an expert editorial consultation. That wastes the ability to use the human as a fast rejector, preference selector, or source of one highlighted word. Agents can propose diagnoses and repairs afterward. If those fail, *then* ask the informative follow-up.

Consequence: request a reaction without asking why by default. A useful label is enough to direct the next agent computation. Do not require a precise definition of “slop” before using the user's discrimination; examples can reveal what distinctions are actually present.

### 2. A short screen can be valuable without being sufficient

“Sentence judgments cannot establish chapter quality” is true but does not answer whether they remove a frequent, cheap-to-recognize reason for failure. The root plan risks using this limitation to return immediately to whole-section review. Instead, use cheap screens to keep obviously bad work out of expensive review; retain occasional longer checks for organization and residual issues.

The actual feedback spans multiple scales. Ambiguous “targets” and precision can often be noticed locally. Placement of sampling detail and relevance of the conclusion require more context. The opening was explicitly judged differently depending on the preceding section. Let the issue choose the context size. Do not insist on a universal review granularity.

### 3. Complete autonomy is a deployment condition, not a restriction on preparation

The eventual eight-hour run must succeed with the agreed access to Jörn. Preparation can use rapid human reactions to choose generation methods, teach preferences and eliminate bad routes. Discovering that a production workflow needs a new human choice every paragraph would be a genuine scalability problem. Using ten quick judgments now to find a transferable intervention is not that problem.

Consequence: distinguish whether each trial is learning a reusable editorial distinction or outsourcing an individual paragraph decision. Test transfer before claiming readiness; do not ban useful interaction while trying to create readiness.

### 4. “Good critic” and “good writer” need not be a serial dependency

A source-faithful writer that frequently produces awkward sentences may still yield acceptable alternatives after one cheap human veto. A writer can improve from labeled examples even if its explicit critique remains poor. Conversely, a correct detector can repeatedly fail to generate a good repair. Test generation and selection separately; don't require a validated agent critic before sampling human preferences.

A particularly cheap alternative is to ask an author for three meaning-equivalent repairs after a rejection. If Jörn rapidly finds one acceptable, explore candidate generation plus selection. If all three preserve the rejected quality, investigate the author's representation of the task. No full-section trial is needed to distinguish these outcomes.

### 5. Richer evaluation infrastructure may now be negative-value work

The Hypothesis route works for longer passages. This conversation may support short async questions. Do not turn a microtest into immutable rendered packets, desk queues and lengthy preambles. The agent can quietly retain exact text, source path, answer and context. Desk coordination remains useful to prevent simultaneous demands on Jörn, but should not be a mandatory delivery route for every sentence.

Capability note: no async-question tool was exposed in this subagent's tool inventory. Root should check its own callable tools. Do not promise an async interaction just because it is mentioned in general instructions. If unavailable, use a short ordinary question at handoff; don't build a replacement UI or silently invoke a plan-only tool.

### 6. Disagreement-selected samples alone can miss shared blindspots

Choosing agent disagreements is useful for discriminating candidate selectors, but all agents previously missed many human concerns. Include at least one sentence all agent reviewers endorse and one ordinary sentence selected without a quality rationale. Otherwise the test can overfit what agents already recognize as difficult. Do not select only conspicuous openings or synthetic caricatures.

### 7. The master plan is not itself demonstrated progress

The twelve-route catalogue has many plausible mechanisms but no basis for ranking all of them. Another clever catalogue is less useful than a few cheap judgments that eliminate a premise. Prefer actual contemporary text and one concrete decision per question. The point is not to establish a calibrated evaluator benchmark; it is to improve and choose the production process.

## Minimal first exchange

Use one actual sentence at a time, with no model identity, workflow name or agent verdict. Supply only the purpose context that changes interpretation, usually one short line. An example question is:

> MSc thesis, empirical subsection opening. Does this sentence immediately feel unacceptable as prose? [exact sentence]

Options: `Immediate problem`, `No immediate problem`, `Need context`. No explanation required. “No immediate problem” is not a PASS label. If the user prefers their own wording (“slop / inhuman”), use it; do not infer the label's detailed cause from its name.

The first exchange should contain at most three short items, not a large battery. Choose:

1. A sentence from the newly revised DS text that an agent presently endorses. This tests whether the informed revision still has readily visible failures.
2. An alternative to that sentence produced independently while preserving its claim. A pair can test whether selection is easier than production; include `Neither` or allow a skip rather than force a winner.
3. A sentence whose acceptance plausibly depends on placement, shown with one line identifying its location. This tests the context cost rather than presuming it.

Ask independently or as a small optional batch according to Jörn's current interaction. A one-item question lowers commitment; batching amortizes switching. Neither is universally cheaper. Ask once afterward whether answering was roughly five seconds and low disruption; elapsed reply time is not active judgment time. Do not impose repeated timing questionnaires.

If a sentence is rejected, do not immediately demand an explanation. Generate plausible contrasting repairs and ask only if selecting among them changes what is done next. If all are rejected, offer a one-word highlight or a short optional explanation. This is an adaptive dialogue, not a requirement for the user to teach a complete taxonomy.

## Concrete source candidates and known context controls

Contemporary revised text, not yet human-graded here:

`/workspaces/msc-math/.worktrees/ds-human-feedback-revision/experiments/writing-quality/runs/20260918-human-feedback-revision/revised.md`, commit `556bae99`.

Possible exact micro-items (unknown human judgments):

- Empirical subsection opening: “In the sampled polytopes, a smaller total symplectic area of the two-dimensional faces usually accompanied a larger systolic ratio.”
- Immediately after that opening: “The association suggests a geometric question: what makes these face areas informative about the systolic ratio?”
- Transition to selection experiments: “Can the observed association still help select promising bodies, even when it does not prescribe a direction of deformation?”

These are convenient live candidates, not a recommended exhaustive opening-only test. Include a middle-of-section sentence once the first interaction works. The third is context-sensitive: the supplied location must mention that preceding text exhibited paths along which decreasing ridge area decreases systolic ratio.

Existing labels should inform construction without wasting a new question:

- “Random sampling can do more than return the largest systolic ratio encountered.” Jörn explicitly reconsidered this as good when preceded by a section about finding the maximum. This is evidence against treating isolated rejection as an intrinsic property of the sentence.
- “This is an analytic example outside the finite random table, so its value above one does not conflict with that table’s recorded maximum.” Rejected as misplaced, while the agent audit praised its scientific distinction. This isolates a useful disagreement between scientific fidelity and reader relevance.
- “Small ridge sum is therefore not a universal ascent objective.” Jörn called the result nice; this is not an unconditional stylistic endorsement of the sentence.

Do not manufacture opposite labels for positive/negative controls. Prior comments are qualified, and unmarked sentences were not accepted individually.

## Outcomes that change the next action

| Observation | Decision consequence |
|---|---|
| Fast, confident rejections of agent-endorsed fresh sentences | Local badness is a tractable missing signal; prioritize few-shot examples, generation alternatives and cheap screening before another long review |
| Human selects a satisfactory repair quickly | Candidate generation plus selection is worth pursuing; next test whether agents can predict that choice on another case |
| All meaning-preserving alternatives rejected | Change the production operation or ask for one diagnostic highlight; do not expand a preference tournament of equivalent outputs |
| Most replies require context | Increase to sentence plus predecessor or short paragraph for that class; do not conclude microreviews are useless for other classes |
| Verdict flips with actual intended context | Record context-conditioned examples; improve placement/purpose representation, rather than globally suppress the wording |
| Micro-items are acceptable but section still fails | Local screening worked only on a nonbinding dimension; spend next effort on relationships, figures or section organization |
| Known-label examples help but fresh cases fail | Treat the apparent detector gain as memorization or narrow transfer; do not claim general readiness |
| Response mechanics dominate or interruption is annoying | Offer a small batch or defer to the existing review desk; do not optimize the prose task around an unmeasured five-second cost |

## Recommendation

Run a tiny contemporary microreview now, in parallel with the source checks and research already underway. Use the result to choose between local generation/selection work and larger-context editorial work. Keep the next long review, but ensure it receives text that survived obvious cheap screens. This is a change in experiment granularity, not a claim that thesis quality reduces to a sentence classifier.

## Sources inspected

- Review desk `experiments/writing-quality/human-review/responses/20260918-root-contemporary-ds-reader.md`: 12 exact annotations, explicit nonacceptance, qualifications and subjective effort report.
- Original Candidate D at `easy-writing-routes/.../route-reader/final.md` and current revision cited above.
- `review-workflow-design/docs/review-workflow-design/reconciled-plan.md` and `adversarial-open-sanity.md`.
- Root conversation and requested sanity-check scope. This review is interpretation of those records, not new empirical evidence or product-mechanics research.
