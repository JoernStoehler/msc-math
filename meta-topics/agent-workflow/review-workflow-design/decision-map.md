# Decisions from review experiments

This is a design map, not an established causal account of Codex. The [hypothesis register](causal-alternatives.md) supplies alternatives; the [architecture menu](architectures.md) supplies interventions; [experiments](experiments.md) specify initial contrasts.

## Objective and stages

The objective is a thesis Jörn accepts. Review is an intermediate capability whose value is better writing decisions within the available time, not more criticisms or a higher evaluator score.

Separate: source/context preparation → issue discovery → issue reporting → aggregation and prioritization → repair → human acceptance. A locally accurate reviewer may reject passing prose, miss missing material entirely, or suggest a repair that damages mathematics. Those are distinct failures and may require different workflows.

## Observable cruxes

| Observation | Live hypotheses | Cheap discriminating intervention | Decision if supported |
|---|---|---|---|
| Generic review misses recorded objections | Aspect selection, weak reader model, report compression, inadequate context, unfamiliar defect | Cue/location/context ladder with paired controls | Narrow the intervention to the layer that improves; do not infer ignorance from one miss |
| Full section fails, short passage succeeds | Coverage burden, distracting context, changed reporting priority | Same target under broader context, with matched report allowance | Use local reviewers plus a separate whole-section reviewer if narrative coverage survives |
| Isolated passage fails, contextual version succeeds | Missing prerequisites or passage purpose | Supply genuine preceding context instead of issue labels | Build a shared context packet before parallel review |
| Reviewer explains an issue when asked but misses it spontaneously | Cue-dependent recognition, reporting threshold, insufficient search | Class-cued search with no-error controls; compare uncued retrieval | Add focused search if useful precision survives; recognition alone does not establish discovery |
| Many specialists find the same things | Correlated strategy, easy-case dominance, duplicated contexts | Compare repeated generalists and distinct task representations | Stop buying redundant reviewers; diversify operations rather than role names |
| A specialist adds useful findings | Complementary capability or case-specific cue | Transfer to another natural text of the same type | Add a scoped specialist provisionally; maintain regression examples |
| Good raw findings disappear in final review | Summarization compression, majority bias, schema mismatch | Frozen reports under alternative mergers | Preserve source-linked union and separate prioritization |
| Numerous findings but poor human acceptance prediction | Detector/classifier mismatch; severities wrong; missing global dimension | Accepted annotated text and rejected comfortable prose | Separate defect inventory from acceptance prediction; do not threshold defect count |
| Synthetic success, natural failure | Artificial salience, unnatural corruption, source-model/genre shift | Match controls more closely; retain natural cases as the target | Restrict claims and workflow role; stop expanding synthetic benchmarks |
| Accurate critique but failed edit | Repair capability, underdetermined critique, incompatible constraints | Give verified objections directly to editor; inspect preserved meaning | Develop editing separately instead of adding reviewers |
| Repaired text passes detector but reads worse | Metric exploitation, new defect, distribution shift, lost content | Independent reader/meaning-preservation check on before/after | Suspend that detector's authority over repair selection; add observed failure to regression set |

No row uniquely identifies an internal mechanism. Describe results at the behavioral level unless contrasting interventions actually distinguish the alternatives. More output or longer reasoning does not establish deeper inspection; an explanation of a requested strategy does not establish its execution.

## Accumulate capability without assuming monotonic improvement

The user's proposed accumulation process is a promising candidate: retain a workflow A that handles issue X, then solve the next issue. It becomes operational with a capability registry containing: target issue and text scope, required context, input/output contract, observed successes and failures, costs, and last transfer check. This design does not instantiate that registry with untested capabilities.

New reviewers can increase recall while decreasing precision, increasing adjudication cost, or steering repair toward unnecessary rewrites. Adding A therefore solves only a scoped detection problem provisionally; it does not certify all instances of X or improve the whole pipeline automatically. Preserve raw independent reports, deduplicate by claim rather than vote count, and keep minority findings inspectable.

Start with independent specialist branches after genuinely shared preparation: stable text, paragraph IDs, mathematical prerequisites and intended reader. Avoid a common interpretive summary that tells every reviewer what the text supposedly means; that can synchronize blind spots. If preparation supplies an argument reconstruction, retain the original and label the reconstruction as an interpretation to challenge.

Use sequential work where dependence is useful: a locator proposes spans; a verifier inspects them; a merger preserves verdicts. Retain at least some uncued inspection, since the locator's omissions otherwise become invisible. Independence consumes repeated context and is not free; sequential verification can inherit anchoring. Measure the tradeoff rather than choose a universal architecture in advance.

## Eight-hour decision policy

First establish whether important near-threshold natural defects can be detected and reported usefully at low cost. Then test whether that guidance changes an editor's output favorably. Do not wait for a general detector before trying repair, and do not let successful local repair certify a chapter.

Maintain separate lanes for proof exposition, motivation/narrative, and empirical interpretation only when observed failure differences justify them. The transferable product may be the search procedure rather than a shared prompt. Whole-document review remains a separate scope: prerequisites across chapters, promises/results, repeated explanations, and sustained reader burden.

At the first checkpoint choose among: reuse a sufficient existing reviewer; add one demonstrated specialist; change representation because cueing did not help; repair aggregation; or investigate missing context. If the experimental result would not change any choice, prefer writing work or a more discriminating experiment.

Before allocating another cycle, state the unresolved choice, possible observations, resulting actions, and maximum time worth spending. These need not be numerical expected-value calculations. The purpose is to prevent attractive but decision-irrelevant testing from consuming the thesis session.
