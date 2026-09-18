# Evidence and reusable artifacts

Read 2026-09-18. Sources below add to the earlier [authoring review](/workspaces/msc-math/.worktrees/review-workflow-design/docs/writing-quality-research/authoring-workflows.md); that sibling-worktree link is local navigation, not a portable dependency. Recommendations in README are proposed adaptations, not replications.

## Current model-specific guidance

[Official Astra guidance](https://developers.openai.com/api/docs/guides/latest-model) describes detailed formatting/recurring phrases and recommends explicit audience/style specification. [Official September 11 guidance](https://developers.openai.com/blog/rethinking-skills-and-prompts-for-gpt-6-astra) warns that elaborate inherited instructions may hinder Astra and that scaffolds helpful for Sol/Luna may overconstrain it. This motivates comparing lean task context against our accumulated process; it is product guidance, not a controlled scientific-writing study. The [GPT-5.6 guide](https://developers.openai.com/api/docs/guides/latest-model?model=gpt-5.6) also recommends observable writing choices and representative evaluations rather than assuming maximum effort/pro mode is best. None of these sources establishes comparative thesis-writing quality for Astra, Sol or Luna. Much recommended style language is already present in our system context.

## Upstream content selection: STORM and Co-STORM

[STORM, NAACL 2024](https://arxiv.org/html/2402.14207v2), sections 3, 5, 6 and appendix B: source-grounded questions informed by multiple perspectives, conversation-informed outline, then writing. Ten experienced Wikipedia editors assessed 20 article pairs, two editors each. Organization rated at least 4/7 increased 45%→70%; preference 26/40 favored STORM. Coverage 57.5%→67.5% was not conventionally significant (p=.084). Verifiability did not improve. Editors found unsupported connections, source-tone contamination and poorer results than well-revised human articles. This is upstream-organization evidence, not publication readiness.

[Official code](https://github.com/stanford-oval/storm): modular question/answer, outline, article and polishing stages; `VectorRM` supports supplied documents. Reusing the questioning operation with checked local mathematics is simpler than deploying the full web-research pipeline. Do not copy obsolete model/sampling settings blindly.

[Co-STORM, EMNLP 2024](https://arxiv.org/html/2408.15232v1): 19 participants compared interactive source-grounded expert discussions with search/RAG. Broader/deeper information and lower perceived effort were reported, but 4 wanted more concise, target-following output. This is information-seeking evidence. It neither measures final scientific prose nor implies a user should supervise lengthy simulated debate.

## Adaptive production: WriteHERE

[WriteHERE/HRP, EMNLP 2025](https://arxiv.org/html/2503.08275v3), sections4–6 and appendix B: dynamically interleaves retrieval, reasoning and writing. On 12 long-report topics, five external volunteers compared Gemini 2.5 Pro-backed WriteHERE with Gemini Deep Research. Topic-majority preference was 7–5; mean clarity 4.3/5 for both. Reports differed substantially in length. Larger gains and ablations elsewhere are automated-judge results. This supports feasibility and architecture exploration, not decisive human superiority. No matched end-to-end cost result establishes our throughput.

[Public implementation](https://github.com/principia-ai/WriteHERE) includes `recursive/agent` prompts, task execution and example reports. Context follows dependencies and prior output rather than giving every subtask all accumulated memory. Appendix D walks through a report task. A native-harness approximation can test adaptive operation choice before importing the scheduler.

## Human comprehension as an outcome

[Hodds, Alcock & Inglis 2014](https://mjinglis.github.io/files/JRME_SelfExpl_Paper.pdf), *Self-Explanation Training Improves Proof Comprehension*: three experiments, including a 15-minute classroom intervention. Experiment 1 reported d=.950. Training attends to warrants, dependencies and overall proof mechanism. This supports a specific reading operation, not the claim that LLM readers mimic students or that adding every missing explanation improves text. The paper also discusses disappointing outcomes of some augmented-proof presentations.

[Gilabert, Martínez & Vidal-Abarca 2005](https://doi.org/10.1016/j.learninstruc.2004.12.003), *Some good texts are always better*: revisions adding causal/goal inferential connections improved recall and inference outcomes across reader-knowledge groups; improving local argument overlap helped recall but not inference. Human historical-text experiment, not mathematical or AI writing. It distinguishes explanatory structure from sentence smoothness.

[Ryba et al. 2021](https://pmc.ncbi.nlm.nih.gov/articles/PMC8430246/), *Better Writing in Scientific Publications Builds Reader Confidence and Understanding*: randomized 170 students across scientific abstract versions, with nine accessibility changes bundled. Objective comprehension 47.9%→57.9%; subjective readability 44.4%→66.3%. This supports measuring understanding independently of felt ease, not optimizing a readability score or attributing the effect to one style rule.

[Davies, Alcock & Jones 2020](https://discovery.ucl.ac.uk/10118849/1/Davies2020_Article_ComparativeJudgementProofSumma.pdf), *Comparative judgement, proof summaries and proof comprehension*: expert comparisons of student proof summaries offer an assessment route focused on logical structure and proof mechanism. Assessment evidence, not an authoring intervention. Summary quality may expose understanding that manuscript ratings do not.

## Inspectable scientific revision traces

[Manubot AI Editor, JAMIA 2024](https://greenelab.github.io/manubot-gpt-manuscript/): five manuscript case studies, human assessment on three authors' own manuscripts; supplementary files 1–3 are revision diffs, file 4 is the judge chain. Humans selectively retained expression changes and restored lost details. Examples expose invented experiments and altered meaning as well as improvements. Useful apprenticeship material and an implementation of selective changes, not independent controlled evidence of acceptance. Its paragraph-level limitation should not be copied where cross-paragraph dependencies matter. [Code](https://github.com/manubot/manubot-ai-editor).

[Friction, CHI 2025, author repository](https://github.com/zhangchaodesign/friction): N=16 human-writer study reports improved substantive feedback uptake and revisions from reflection/hints. The process groups feedback, diagnoses the underlying problem and selects a revision strategy. [Stage code](https://github.com/zhangchaodesign/friction/blob/main/components/plan/PlanProcessPrompt.tsx) is inspectable; other components contain hardcoded hints/placeholders, so a production LLM pipeline was not verified. Having an agent perform the reflective step is a proposed adaptation, not the studied intervention.

## Tempting leads with important availability or scope constraints

- [Prover–verifier games 2024](https://arxiv.org/html/2407.13692v1) trained models on grade-school math with answer ground truth, improving helpful-prover legibility to time-limited humans. Main human campaign collected 15,983 judgments, 45 seconds/problem. This establishes a possibility of small-verifier→human transfer, not that a prompted Luna reviewer is a valid proxy. Training, adversarial examples, verifier capability gap and the narrow correctness task matter.
- [Author-centered delta feedback 2025](https://www.sciencedirect.com/science/article/abs/pii/S0957417425008486) uses human reference prose to construct information-gap feedback and retrains author-cue extraction. It is not an immediately deployable reference-free editor for our new results.
- [ScienceJury institutional record](https://escholarship.org/uc/item/7dw8b901): promising abstract, dissertation embargo through December 22, 2026. Detailed accessible protocol and outcome evidence not verified; not a ready recipe.
- [SWIF²T 2024](https://arxiv.org/html/2405.20477v1) provides investigator→focused-review prompts and a human comparison, but explicitly excludes readability/clarity and motivation. Relevant to technical peer review, poorly matched to the present editorial failures.
- [Five-Phase Writing 2026](https://link.springer.com/article/10.1007/s43681-026-01304-y) advertises a 120 paired-task evaluation and large gains. Only abstract/metadata were accessible; methods and data could not be inspected. Do not treat headline effect sizes as checked evidence or a ready solution.

## Scope and search trail

Queries covered scientific/technical-writing pipelines, argument and author-cue representations, adaptive planning, reader comprehension, figure-first workflows, exemplar/personalized writing and current Astra/Sol/Luna guidance. Primary papers were opened, with methods/evaluation read where accessible; public artifacts were inspected without execution. Existing Self-Refine, ARIES, ReviseBench, PaperOrchestra, LiRA, Re3/DOC and process-oriented abstract revision findings were reused rather than re-advertised as new discoveries. Search snippets and practitioner claims of top-journal readiness were not treated as outcome validation.
