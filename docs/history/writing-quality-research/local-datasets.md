# Local writing/review dataset inventory

Read-only investigation, 2026-09-18. Root: `/workspaces/msc-math`. No session logs, credentials, or external research were read. No thesis production was resumed. This report is the only created artifact. Paths below are relative to that root unless absolute.

## Finding

There is a small, unusually task-relevant corpus, not mainly unrelated scientific writing: actual agent-produced thesis prose, generated for this project and judged by the intended human. It includes completed positive chapter judgments, local objections within passing prose, comfortable-but-narratively-failing text, and a whole-thesis FAIL. It is not a ready-made independent benchmark: author/reviewer overlap, feedback-conditioned revisions, voluntary annotations, and selection during deadline work substantially constrain inference.

The useful unit is an immutable reviewed PDF plus its bounded feedback record and pre-feedback predictions, not the current chapter source. Several exact human-reading PDFs still live in `.git/codex/`; the ordinary review archive preserved judgments but not all corresponding PDF inputs. All five hashes in the next section were recomputed and match the recorded identity where one was already recorded.

## Strongest packets

### 1. HKO completed PASS, annotated (best positive control)

- Text: `.git/codex/hko-writing/chapter-preview.pdf`, seven pages, SHA256 `c5b4508fb226971d2c5d68d8bd8f8b32cdd727272dfc3b562a1e3f993c176202`.
- Human: `docs/review-evidence/calibration/hko-reading-feedback.md`.
- AI prediction: `docs/review-evidence/calibration/hko-reading-prediction.md`.
- Authoring provenance: `.git/codex/hko-writing/README.md`, associated TeX, `memories/authoring-workflow.md`.
- Author: fresh agent prose from existing mathematical inputs, integrated and extended by coordinator; human project context precedes production. Exact upstream human/AI authorship of each mathematical input is not exhaustively established here.
- Reviewer: Jörn, direct messages recorded by coordinator. Predictor was a coauthor, already aware of earlier DS feedback; prospective relative to HKO comments, not blind.
- Intent/selection: intended high-quality deliverable during deadline work, not artificial errors. Selected as a chapter for actual review, not randomly sampled writing. Exact generation budget not recorded in inspected files.
- Labels: completed chapter-level PASS, explicitly allowing improvements. Seven localized annotations: CAS attribution; premature witness IDs; missing worked-example purpose; unsolicited negative alternative; skipped product/non-product inference; coordinates before geometry; code-advertisement tail.
- Crucial: PASS does NOT make every sentence clean; unmentioned predicted concerns are unadjudicated. Only one of the seven specific human annotations was predicted originally; the later removal of another issue followed prior feedback and cannot count as a frozen prediction.
- Current reconstructed HKO/candidate source may contain later repairs. Use frozen PDF for evaluation.

### 2. DS two-page sample: prose positive, narrative negative (best prospective mismatch)

- Text: `.git/codex/ds-first-wave/writing/sample.pdf`, SHA256 `01d63be3d72999230e98c0b5fb6f89dcba33d532188d83894f4ad01a9f62c58c`.
- Human: `docs/review-evidence/human-feedback/ds-reading-2026-09-14.md` (first section is quoted user wording; later sections are explicitly coordinator relays).
- AI predictions: `docs/review-evidence/calibration/ds-sample-reading-astra.md`, `ds-sample-layout-luna.md`; comparison `ds-sample-comparison.md`.
- Provenance: `.git/codex/ds-first-wave/writing/source-map.md`, `sample.tex`; agent author plus coordinator shortening opening/closing framing, grounded in research ledgers and human-supplied research purpose.
- Intent/selection: deliberately short prospective prose sample, not complete methods exposition; full historical samples/rules/methods omitted intentionally. Not engineered to contain X. Intended good writing under a sprint context; exact budget unknown.
- Labels: comfortable/passing sentence-level writing, chaotic narrative; human explains desired statistical-discovery-to-mathematical-insight sequence. Not full-chapter PASS.
- Predictor: coordinating Astra with earlier project/preference context, no sample feedback yet. Five minor predictions, none fully individually adjudicated; major narrative objection missed. Luna layout approval is AI-only, no human layout ground truth.
- Follow-ons: negative opening feedback, a positive first-sentence judgment, and a requested comma/clarification. Exact revised sources exist as `chapter-opening-v2.tex`, `chapter-before-narrative-feedback.tex`, `chapter.tex`, but feedback-to-revision identity requires care; do not attach all three feedback rounds to sample.pdf.

### 3. Fixed whole-thesis reading: negative openings + borderline positive chapter

- 17:00 input: `.git/codex/thesis-review-1700/build/main.pdf`, 81 pages, SHA256 `8b1b8d2d54203af83f2e01f2b684c040ba52e845d0f23256ca9117c2f511e5d6`.
- 17:30 input: `.git/codex/thesis-review-1730/main.pdf`, 86 pages, SHA256 `356a0721f38f90b1f7cc34785fc3ef26a0f25e1a804cf487992078e4f70b415b`; sibling README confirms frozen binary, not standalone source bundle.
- Human record: `docs/review-evidence/calibration/whole-thesis-reading-1700.md`, including explicit heading for switch to 17:30 input.
- AI: `qp-prose-preflight-1736.md`; retrospective `1800-checkpoint-retrospective.md`.
- Author: assembled agent-written/revised manuscript with human research inputs and online human feedback incorporated; exact chain varies by passage and is not fully encoded. Abstract explicitly coordinator-authored.
- Intent/selection: actual deadline submission/review, high quality intended within time constraints, naturally occurring defects. Reader selectively stopped/skipped preliminaries, then completed Chapter 4.
- Human labels: abstract naming/detail/contribution defects; introduction scope/motivation/detail problems; preliminaries rejected after early prose issues; Chapter 4 completed PASS, writing borderline, suitable mathematical audience. Withdrawn Chapter 4 annotation is NOT a confirmed defect.
- Review scope mismatch itself is data: mathematical-only preflight explicitly excluded style; cannot treat its silence as a failed prose detector. Later QP suggestions inherited feedback and introduced mathematical imprecision needing correction.
- Preservation caveat: accepted Chapter 4 is in 17:30 PDF, not automatically current recovered Chapter 4.

### 4. Whole-thesis FAIL with two AI self-reviews and Pro review

- Text: `docs/source-recovery/evidence/frozen.pdf`, 86 pages, SHA256 `d7dae9a78dffc87fe89f3005bed9b6b51d28728fa90e1a5c811bc09b236e1695`.
- Recovered source and identity: `docs/source-recovery/README.md`, dependency-manifest.json, verification.json; text/render equivalence of recovered build recorded.
- Human whole-thesis FAIL is explicitly reported in `docs/review-evidence/calibration/self-review-2026-09-15-pre-pro.md`; earlier 18:00 retrospective says no PASS by deadline, which alone is NOT a FAIL. These refer to different knowledge times.
- AI: `docs/review-evidence/blind-self-review-2026-09-15/REPORT.md`, FREEZE.txt; `calibration/self-review-2026-09-15-pre-pro.md`; `pro-review-2026-09-15/thesis_review/` (35 findings, 3 separate verification questions, annotated PDF, machine-readable findings and annotation index).
- Comparison: `calibration/pro-comparison-2026-09-15.md`.
- Reviewer provenance: Pro is AI review, not independent human labeling. Self-reviews withheld Pro feedback but knew human FAIL and earlier feedback; one authored/integrated parts. “Blind” does not mean blind to all preference/context/history. Roughly ten-minute review constraints explicitly documented.
- Human/AI hybrid status: human requested/provided AI report; no inspected evidence that every Pro finding was human adjudicated. Later pentagon proof acceptance is one bounded human check, not endorsement of all Pro findings.
- Use: rich hypothesis-generation and source/meaning regression cases. Not 35 extra human writing labels or multiple independent whole-thesis human grades.

## Additional packets, lower immediate evaluation value

### September 7 HKO authoring trial

`docs/review-evidence/calibration/hko-authoring-trial-2026-09-07/README.md` records three frozen candidates: questions, obstructions, mechanism. Selection was explicit workflow development on a small central valid argument, not a known error. All are AI-authored; separate AI meaning/flow readers supplied feedback. Human receipt/reading is reported, but no returned human annotations or acceptance label is established for these exact artifacts. Revised two-page human-reading PDF SHA256 `759d8f6357e5694edf1401434c5ee6bedcc28cf12e13326a4da89fb7ad0cf536`; later repaired PDF `a62f84dd8de88b0e12e6d9ce2c23fa1ccf49910b4b5ab822d0ef6ebab689ce45` was NOT requested for human review. Useful as workflow-comparison outputs, not human-supervised training pairs. Meaning-review correction preserved only as an offending sentence plus corrected final candidate, not full before draft.

### September 16 pentagon rejection and v2

`docs/pentagon-chapter/user-reading-feedback.md` preserves human rejection triggers and later acceptance of the proof while rejecting writeup. Text authored by agents from AI-proposed analytic proof plus prior sources; intended ready high-quality chapter, not adversarial flaws. Initially served PDF was overwritten by a scope revision: exact original input is not reliably paired with all comments. Quoted spans remain usable local cases with surrounding-context limits.

`docs/pentagon-chapter-v2/review.md` records five retrospectively selected passages with hidden source/labels and agent judgments: two excerpts from accepted HKO, three rejected pentagon features. Agent missed implementation-first opening. Labels then revealed and reviewer prompt revised; same reviewer approved new full chapter and did narrow math regression review. Thus five passages are selected contrasts, NOT random samples; two accepted-chapter excerpts are NOT separately human-approved sentences; post-label v2 review is not held out. v2 has no human writing judgment. Useful failure hypotheses, not validation of a detector.

## Coverage, unknowns, and leakage

- Strong human judgment covers two completed chapter-level PASSes (HKO; Chapter 4 borderline), one two-page prose/narrative split assessment, one reported whole-thesis FAIL, and selected passage-level objections/approvals. This is not exhaustive coverage of all thesis pages or all human dislikes.
- Multiple comments from the same reader, same manuscript family and same sprint are correlated. Revisions condition on earlier comments. Split by artifact lineage/topic/feedback time, not randomly by sentence.
- Existing root context has already seen essentially all prominent labels; a no-history evaluator can hide labels, but source filenames, summaries and prompts can leak them. Keep source and ground-truth packets separate if testing prospectively.
- Missing annotation is not clean/no-X. PASS is not no-X. Proof approval is not prose approval. AI agreement is not human ground truth.
- Original human-written scientific prose with independent human review was not located in this scoped local inventory. Literature papers exist but were not inspected/classified; publication is not a Jörn quality label. Most local prose is agent-generated or agent-revised with human mathematical context and feedback, rather than pure human prose.
- No deliberate synthetic-error corpus or systematically sampled best/worst pool was located. Natural failures were selected by human attention; HKO authoring alternatives and five-passage diagnostic were deliberately selected for workflow exploration.
- Exact model versions, token/time budgets, complete prompt chains, and full authorship transformations are often unknown from these records. Recovering them would require bounded session-log investigation; unnecessary for initial use of the strongest fixed packets.

## Suggested immediate use

Start from HKO PASS+annotations, DS prose/narrative split, and 17:30 Chapter 4/preliminaries contrasts. These give true task-local variation including acceptable imperfection, not only a pile of bad examples. Preserve whole input context and frozen prediction dates. Use Pro/agent-only cases to brainstorm detector hypotheses and mathematical regression checks, not as human acceptance labels. Before prospective evaluation, choose a disjoint writing lineage or newly generated held-out material: these historical packets are already prominent development evidence.
