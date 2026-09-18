# Scientific-writing solution search

2026-09-18. Research proposal, not activated instructions or a validated authoring workflow. No thesis edits, experiments, or new requests for human review were made. This broadens the earlier writing-quality investigation rather than replacing its evidence.

## What changes the plan

The literature supports trying **better upstream composition and information selection**, not making a reliable general critic a prerequisite. It also offers a different review object: what a reader can reconstruct or do after reading, rather than whether a reviewer says the prose is good. Neither route guarantees acceptance.

There is no located controlled evidence that an Astra/Sol/Luna workflow produces acceptable mathematical thesis chapters. Current official guidance supplies a useful competing hypothesis: accumulated procedural and stylistic instructions can obstruct capable models. Our existing system already contains much of the official writing advice; repeating that advice is not a materially new intervention. See [evidence and artifacts](evidence.md).

The strongest new positive LLM evidence is about article organization and long-report construction, with important negative results on unsupported synthesis and no consistent superiority in clarity. Human proof-reading and scientific-reading experiments instead support measuring explanatory understanding. These evidence classes should not be pooled into a claim that any proposed workflow has already worked for this thesis.

## Four trials worth doing

These are independently useful comparisons, not a mandatory sequence. They can use the existing tool harness; installing research software is unnecessary for a first trial. Time allowances below are proposed caps, not measured runtimes. Use the same supported content, reader prerequisites, section purpose and neighboring material within a comparison. Avoid making Jörn adjudicate many near-identical versions.

### A. A capable autonomous composer with a lean task context

Give a fresh author a small source dossier, the real intended reader, the scientific purpose, adjacent sections, and the required artifact. Allow it to inspect original sources, choose its own working process and revise the file. Do not supply our history of failed workflows, the rejected prose, or a long menu of avoidance rules. Preserve applicable authority and mathematical constraints; this is not permission to bypass system instructions.

Compare with the same model performing the current more-prescriptive workflow. This tests the *benefit of our scaffolding*, not whether prompting in general works. A fresh thread changes conversation history, not all inherited system guidance; record the actual difference.

**Why now:** our interventions may be part of the problem, and a new strong model is not the same treatment as an older model inside the same recipe. Official advice motivates the comparison but is not scientific-writing outcome evidence.

**Decision-changing result:** a usable section that makes better relevance/emphasis choices without editorial rescue would justify a larger autonomous assignment. Identical failure would reduce this route's priority. Budget: two bounded 15–20-minute authoring attempts; no new infrastructure.

### B. Source-grounded editorial inquiry before composition

Before drafting, let several substantive perspectives question the sources: the mathematician seeking the result's mechanism, the reader tracing what was observed versus proved, and the reader deciding why a method or experiment matters. Each answer must use the source material; unanswered questions become explicit research/explanation gaps. Then one author selects what deserves space and writes from the resulting answers plus originals.

This differs from the earlier dialogue-to-transcription trial: the inquiry changes *what knowledge gets selected and connected*, and is not itself the prose scaffold. It also differs from retrieving more citations indiscriminately. Stop source exploration when the question is answered; don't import an encyclopedic coverage objective into a thesis section.

**Decision-changing result:** questions expose missing scientific connections before prose generation, and the resulting section avoids irrelevant detail without losing support. If the dossier simply grows and the prose becomes broader but less focused, discard or narrow the treatment. Budget: a 10-minute inquiry followed by a 15-minute author; source inquiry can run in parallel, selection cannot.

### C. Adaptive section production rather than a fixed outline pipeline

Give an author ownership of a whole explanatory section and explicit ability to alternate: retrieve an omitted fact, work out a pedagogical derivation, construct a figure, draft a part, reorganize already-written text, and stop pursuing a subproblem that ceased to matter. Keep the scientific dependencies and adjacent prose visible. Ask for a short operation record to reveal where useful effort went, not a detailed reasoning transcript.

This is a lightweight trial of WriteHERE's separation of retrieval, analysis and composition with planning interleaved with execution. Do not require implementation of its task graph or copying every module. Compare against a fixed-outline author, keeping total work bounded. This is a different variable from A: A removes prescribed process, whereas C tests a particular adaptive process; they may converge in behavior.

**Decision-changing result:** the author notices and resolves a substantive explanatory need while writing, with a better integrated result than the fixed plan. More recursive planning without improved text is a negative result. Budget: a 20–30-minute section attempt. The full published framework has computational overhead and does not establish an eight-hour thesis schedule.

### D. Edit against reader products, not praise or defect lists

Prepare a few consequential questions independently of a candidate's wording: state the exact conclusion and scope; reconstruct why the proof works; distinguish observation from explanation; apply the result to one boundary case. A fresh reader receives the candidate and stated prerequisites. Freeze an unguided structural summary before supplying the questions, since the questions can themselves teach the omitted distinctions. Then collect answers and compare with the checked scientific account. Include a prerequisites-plus-questions/no-passage baseline on an initial trial to expose answers obtainable without reading. Where it succeeds by supplying unstated knowledge, distinguish that from what the text taught.

Use the discrepancies to edit explanations or figures. Compare this with ordinary critique on the same material. Proof passages and empirical accounts need different questions; that is acceptable. A weaker model is an available experimental reader, not automatically a valid MSc-reader substitute. Repeated success can reflect pretraining knowledge, and repeated failure can reflect an incapable reader.

**Decision-changing result:** this finds and repairs important miscommunication that the original scientific-fidelity review missed, with eventual human confirmation. It does not measure all relevance, elegance or audience-fit failures. Budget: question preparation plus two short reader runs and a bounded revision, initially about 15–20 minutes.

## Keeping these trials informative

- Retain original baselines, raw outputs and actual source packets. A pipeline can improve one dimension and damage another.
- Separate mathematical preservation from communication success. Known-fact checks are useful rejection filters, not the entire grade.
- Do not rank candidates using the same unvalidated prose judge that directed their optimization. Agent comparisons can expose concrete differences; human acceptance remains external evidence.
- Existing human annotations can inform the brief, but testing a new case after doing so is different from discovering objections blind. Record which task is being attempted.
- Track source preparation, authoring, review, interpretation, human attention and integration separately. No located study supplies our end-to-end cost forecast. Requesting four human readings would erase much of the benefit of cheap parallel generation; choose comparisons with visibly different outcomes first.

## Additional routes and why they are not the first four

**Scientific edit apprenticeship:** Manubot supplies actual original/revised manuscript diffs and contextual prompts. They are useful examples of selective edits and preservation failures, not proof that imitating their wording yields Jörn's standard. Human-authored text plus human edit selection is materially different from editing AI notes.

**Feedback interpretation before revision:** Friction asks writers to reflect and generate their own fixes from hints. Useful if feedback is obeyed literally while its purpose is missed. Replacing the human reflection with another agent remains an untested adaptation; it cannot be cited as autonomous-writing validation.

**Figure/argument-first construction:** likely worth testing on the DS account because actual feedback asks for figures and visible scientific distinctions. This search did not find strong controlled evidence establishing that a figure-first workflow improves LLM mathematical exposition. Treat it as a task-grounded experiment, not a literature-proven default narrative order.

**Training a smaller verifier or finetuning an editor:** prover–verifier research supports the possibility of human-legibility transfer, but its training and ground-truth requirements differ sharply from prompt-only agent review. Do not start such infrastructure before cheaper inference-time trials answer whether it is needed.

**A universal score, longer defect rubric, or more same-family reviewers:** no new evidence here resolves their known limitations. Their availability does not justify making them the prerequisite or stopping rule.

## Remaining uncertainty

The decisive gap is transfer from these mechanisms to the actual reader, material and acceptance threshold. We do not know how much failure arises from source selection, inherited instructions, scientific understanding, long-form control, or editorial judgment. The four trials change those factors differently; they need not all be attempted before substantial writing. A successful direct-composition route could make a general detector unnecessary. Conversely, a useful reader-product check could improve every authoring route without certifying the whole manuscript.

The search is bounded, not systematic. It covered current official model guidance, primary long-form writing systems and artifacts, human proof/scientific comprehension studies, revision interfaces and personalized/author-centered generation. It did not establish current-model causal mechanisms, a universally best model, or an absolute PASS rate. Code and prompts were inspected as sources, not installed or executed.
