# Writing detector research — 2026-09-18

Scope: bounded primary-source search, not systematic review. No thesis edits or detector implementation. No evidence located directly validates current Codex models as judges of long mathematical exposition. Research nevertheless provides usable experimental designs, provenance-rich datasets, and several interventions worth testing. It does not justify skipping target-specific detector evaluation.

## Most decision-relevant findings

### 1. StoryFeedback: direct prompting ablation; finding useful issues differs from finding the important issue

[Paper, July 2025](https://arxiv.org/html/2507.16007v1); [official data](https://github.com/google-deepmind/igen).

326 sampled seed stories from ROCStories/BIG-bench, plus synthetic sentence swaps, deletions and repeated backtranslation; approximately 1,300 texts, 83,456 model feedback pairs. Eight model variants (GPT-3.5/4, Gemini 1.5, Gemma2, Bloomz). Humans rated 1,920 feedbacks, three raters each; they wrote their own feedback before seeing model feedback.

Table 5 compares a request for bulleted feedback with the same request plus a taxonomy of writing issues. Taxonomy changes mean human-rated correctness .746→.747, largest-issue detection .679→.719, corruption relevance .406→.513, specificity .923→.844. Two-shot examples did not clearly improve results and sometimes encouraged copying example feedback. This is taxonomy-versus-none, NOT keywords-versus-full-explanation. It supports testing terse prompts rather than assuming elaborate instructions help. Human ratings concern predicted repair benefit, not observed improvements after executing edits.

Useful transfer: separately evaluate correctness, salience, omissions and false reassurance. Synthetic short fiction remains far from natural mathematical exposition. In particular, finding some real errors does not establish a useful stopping detector.

### 2. CriticEval: concrete references help, but labels are hybrids

[Paper, 2024, inspected v4](https://arxiv.org/html/2402.13764v4).

Nine tasks, 35 tested LLMs, feedback/comparison/correction/meta-feedback. LLM responses are selected into low/medium/high quality strata. GPT-4-turbo initially rates and critiques them; humans review/revise those judgments. Reference critiques are therefore human-refined AI material, not independent human judgments. Authors explicitly acknowledge anchoring risk. For math/code, rubrics evaluate correctness rather than exposition.

Providing human-annotated reference critiques improves meta-evaluation: GPT-4-turbo correlation with human ratings reaches .6618 versus human average .7903; removing references reduces average performance by 13.36 correlation points across models. This is evidence for concrete task-specific references, not proof that verbose rubric explanations beat keywords. Three humans also score 450 comparison/correction critiques. Correction has executable/ground-truth checks for math/code; open-ended correction and critique quality principally use reference-guided GPT-4 evaluation. The paper reports higher-scoring critiques leading to better corrections, but that does not validate literary quality or transfer to an MSc chapter.

### 3. Self-preference is not removed merely by hiding authorship

[Wataoka et al., October 2024](https://arxiv.org/html/2410.21819v1).

Eight judges evaluate pairs from the 33,000-dialogue Chatbot Arena dataset, with human preferences. Order is swapped. GPT-4 agrees strongly when humans prefer its response, much less when humans prefer its competitor: recalls .945 and .425, difference .520. Some smaller models exhibit reverse preference, so the effect is not universal.

Open-model analyses associate judgments with lower perplexity more strongly than human judgments do, including outputs from other models. GPT-4 perplexity was unavailable. Thus familiarity is a candidate mechanism, not an established causal explanation for GPT-4. Anonymous fresh agents can still share stylistic preferences. This study concerns short general dialogue and older models; it does not measure current Codex mathematical writing.

### 4. LLM-Rubric: dimensions contain signal, but calibration matters

[ACL 2024 paper](https://aclanthology.org/2024.acl-long.745/).

Nine rubric questions on information-seeking dialogue cover naturalness, conciseness and citation quality. A small learned network combines judge distributions, including judge-specific parameters. It predicts human overall satisfaction on a 1–4 scale with RMS error below .5, approximately half the uncalibrated baseline. Raw LLM judgments do not agree well with humans.

Useful distinction: a model may have diagnostic signal even when its final quality judgment is poor. This supports retaining dimensional observations rather than only PASS/FAIL. It is NOT evidence that an untrained rubric, few examples, or a handful of Jörn labels will yield the reported calibration gain.

### 5. Rubrik’s CUBE: explanations with human and AI provenance

[ACL 2025 paper](https://aclanthology.org/2025.acl-long.1160.pdf); [official repository](https://github.com/RubriksCube/rubriks_cube).

Approximately 26k explanations from humans and six model families, across commonsense reasoning, fallacy detection and two language tasks. Task instances are sampled/filtered, not cherry-picked solely for best prose; incorrect-answer explanations are retained. Human-written annotation subset has 440 instances. Initial evaluation subset: 920 explanations from 80 instances. Two humans plus selected GPT-4o assess a remaining 4,140 explanations; distinguish human-only labels from combined analyses.

Rubric separates explanation structure/components from qualities such as conciseness. GPT-4o assesses criteria rather than directly declaring overall good/bad. Authors report reduced self-preference, but this is not a controlled universal mitigation result. Their custom agreement scores can hide label collapse: an apparently strong judge mostly chooses one explanation type, prompting an additional metric. Useful diagnostic warning and reusable rubric/data; not evidence of expert scientific communication acceptance.

### 6. arXivEdits: natural scientific revisions, not certified quality labels

[EMNLP 2022 paper](https://aclanthology.org/2022.emnlp-main.641/).

751 full arXiv papers with human-annotated cross-version sentence alignments; 1,000 sentence pairs have fine-grained edit spans and intention labels. These are naturally occurring scientific revisions, rather than intentionally planted defects. Helpful for repair exemplars and failure taxonomies. A subsequent author version is not automatically better, and edit-intention labels are not an expert PASS/FAIL judgment. Sampling papers with revisions introduces selection toward revised work. Generation budgets and detailed mixed-authorship histories are not established by this source.

### 7. Scientific abstract process study: unusually close task, crucial proxy caveat

[June 2026 preprint](https://arxiv.org/html/2606.15583v1); [official dataset](https://huggingface.co/datasets/patrickqdasilva/process-revision-sci-write).

45 published CS abstracts and AI counterparts; 869 retained human edit sessions with keystroke traces. Randomized source/disclosure conditions; editors incentivized to obtain reviewer acceptance. Additional GPT-5.4/Claude Opus4.6 editing experiments compare generic versus rubric guidance.

Reported local improvements and persistent coherence weaknesses are measured using 25 computational linguistic features grouped into five dimensions. They are NOT direct expert ratings of resulting exposition. Rubrics improve some local metrics; high-scoring abstracts can lose coherence on these metrics. Human/AI source and disclosure histories are unusually explicit. This is a promising dataset for authoring-process hypotheses, but quoting its headline “global coherence” results as human acceptance evidence would overstate them. It cannot establish that the metric is robust under optimizing against it.

### 8. Scientific-paper feedback at scale: helpful critique does not imply readiness recognition

[Liang et al., 2023](https://arxiv.org/html/2310.01783v1).

GPT-4 feedback compared with human peer reviews on 3,096 Nature-family and 1,709 ICLR papers. Review-point overlap is 30.85%/39.23%, compared with human-human 28.58%/35.25%. Prospective survey: 308 researchers; 57.4% rate feedback helpful/very helpful. Participants also report generic comments and missing technical specificity.

Natural scientific papers/reviews, selected toward top venues; ICLR includes accepted/rejected papers, Nature corpus published papers. Human authorship/mixed AI histories are not individually established. Point overlap and perceived helpfulness do not measure full defect recall, false reassurance, final writing PASS, or successful repairs. Evidence supports using existing critic capability as a baseline; it does not support treating critique fluency as detector validity.

## Hypotheses and near-term implications (our inferences)

- No direct test found of the exact claim “writing-defect keywords activate everything a full explanation would.” Closest controlled evidence is StoryFeedback's taxonomy/no-taxonomy and zero/two-shot comparisons, which show dimension-specific gains and losses.
- Bad generation habits need not entail total inability to critique. Existing critic results contain real signal. But familiarity bias and salience failures give concrete ways author and judge errors can correlate. None identifies an RLHF/self-play causal origin for current Codex prose habits.
- Distinguish existence/localization of a defect, its consequence, its importance, completeness of detected issues, and readiness. Existing evaluations frequently cover only a subset.
- Start with a terse baseline; compare a taxonomy and concrete reference conditions on the SAME held-out target texts. Detailed prompting is a treatment to evaluate, not a prerequisite.
- Include accepted passages and naturally failed passages. Synthetic perturbations give cheap known-error tests but cannot replace natural failures near the acceptance threshold.
- Assess error salience and false reassurance, not just agreement or count of valid criticisms. Hold out some human feedback when building prompts; preserve whether labels were independent or AI-assisted.
- A different model family is a useful diversity treatment, not guaranteed independence. Anonymous same-family reviewers do not remove familiarity bias.
- There is no research-supported shortcut straight to a validated thesis detector in this search. There ARE strong shortcuts to a focused local trial: existing taxonomies, natural revision examples, controlled corruption designs and known evaluator failure modes.

## Additional close matches: natural AI errors with human span marks

### SCARECROW (ACL 2022)

[Paper](https://aclanthology.org/2022.acl-long.501/); [official data, annotation training and viewer](https://yao-dou.github.io/scarecrow/).

13k human annotations on approximately 1.3k human and model-generated English news paragraphs, with over 41k error spans. Marks carry error category, severity, explanation and antecedent when relevant. Ten categories emerged through expert analysis plus ontology-free crowd pilots; not a predeclared universal style rubric. Generation spans GPT-2/Grover/GPT-3 and varied decoding configurations, so natural errors rather than intentionally corrupted passages; human comparison texts are included. This is a strong match for learning how to elicit/localize defects in model prose. News paragraphs, lay annotators and old generators remain substantial transfer limits. Despite the paper title, the resource is not merely binary AI-authorship detection.

### SNaC (EMNLP 2022)

[Paper](https://aclanthology.org/2022.emnlp-main.29/); [official data and models](https://github.com/tagoyal/snac).

Human span annotations over 6.6k sentences in 150 generated book/movie summaries; repository reports 9.6k error annotations. BART and GPT-3 at two sizes produce the summaries. Taxonomy is derived from naturally occurring coherence failures. Released T5 classifiers evaluate the next sentence conditioned on the preceding summary, with/without span prediction. This is particularly useful for missing context/referent and discourse-continuity hypotheses: a detector can be operationalized around what a reader has already encountered, rather than a global stylistic score. It is still narrative summarization, not scientific explanation, and trained old-model detector performance cannot be assumed to transfer.
