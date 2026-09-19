# Review-workflow design portfolio

**Status: hypotheses and proposed experiments, not validated detector findings.** This directory is an inactive research design artifact. It changes no agent instructions, thesis source, or deployed workflow. The user requested a broad search before committing to a review architecture, with particular interest in one-shot coverage limits, learned task shortcuts, and accumulated parallel specialists.

The immediate goal is to discover useful review behavior quickly enough to improve a thesis within an eight-hour production session. The target remains Jörn's whole-thesis acceptance; finding recorded errors, reviewer agreement, and successful compilation are not substitutes.

## Start here

1. [Decision map](decision-map.md): what observations change the workflow and what claims they do not support.
2. [Experiments](experiments.md): a small portfolio separating coverage, diversification, and aggregation, with dataset boundaries and cost/latency comparison.
3. [Causal alternatives](causal-alternatives.md): competing behavioral explanations, including alternatives to the user's attention/shortcut hypotheses.
4. [Architectures](architectures.md): broad intervention space, including reader-state prediction, reconstruction and relational review rather than only more specialist prompts.

The recommended first portfolio is **an open-review/class-cue/exact-locus recognition diagnostic**, followed by **scope/breadth screening where search burden remains plausible** and **evidence-preserving aggregation on those same outputs**. Run **specialists versus repeated generalists** only when that comparison can change the next architecture choice. For a global narrative miss, try prefix-only reader expectation or independent argument reconstruction before merely strengthening a rubric.

## Evidence currently available

[The dataset bundle](../../experiments/writing-quality/README.md) contains seven local cases and 37 bounded human observations. Local cases overlap and share project history; feedback is selective, recorded by agents, and not an exhaustive annotation of all text. c001 is accepted HKO with localized objections; c002 is readable prose with rejected structure. These jointly prevent treating every defect as FAIL or pleasant sentences as PASS.

The four external datasets preserve different genres, authorship histories and label meanings. They offer diagnostic controls and transferable failure hypotheses, not a ready scientific-thesis acceptance oracle. Their ignored cache is already present in the dataset worker's worktree; this task neither redownloads nor duplicates it.

No model evaluations or new human judgments were collected for this design. Deterministic inspection of README/schema examples informed input selection. Suggested experimental times are explicitly unmeasured planning allowances. Realized usage, including reasoning where billed, and coordination overhead must be measured before extrapolating.

## Design constraints

- Distinguish discovery, reporting, aggregation, repair and absolute acceptance. A success at one stage does not prove the next.
- Compare parallel review at both fixed total cost and fixed wall-clock deadline. These answer different questions.
- Treat workflow instructions as interventions to test, not descriptions of faithful internal execution. Do not infer hidden attention or training causes from fluent explanations or reasoning traces.
- Preserve accepted material, natural failures and synthetic/sham controls. Unknown labels are unknown, not negatives.
- Do not inspect gold labels within the fresh reviewer input; record unavoidable lineage and prior-exposure limitations.
- Keep mathematically correct content and necessary explanations under independent checks when optimizing review or repair.

## Provenance and authority

Prepared in branch `research/review-workflow-design`, using the repository's [harness-engineering skill](../../.agents/skills/harness-engineering-v2/SKILL.md). The sibling hypothesis author began without the surrounding conversation; the architecture author explored the intervention space independently; the coordinator designed measurable contrasts and synthesized. This diversifies prompts and context, not model identity or training.

Current product controls must be verified against [OpenAI's model guide](https://developers.openai.com/api/docs/guides/latest-model) at execution time. This artifact assumes no undocumented one-call behavior, seed control, access to internal reasoning, or free reasoning tokens. No configuration activation, external publication, or large evaluation campaign is authorized by this document.
