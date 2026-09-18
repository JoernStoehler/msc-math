# Review UX research, 2026-09-18

## Objective and user evidence

Minimize total Jörn-time per item to complete usable feedback, including reading, marking, explaining, delivery and later clarification. Initial learning is cheap when amortized. Jörn uses a Chromebook with keyboard and trackpad. Speaking reactions felt awkward in past attempts; he bets against its efficiency/effectiveness but is open to an experiment. These are interface preferences, not judgments of any prose sample. The first HKO packet remains unreviewed.

Three delegated research tasks explored interaction design, Chromebook tools and feedback extraction. This is a shortlist and research synthesis, not a platform decision or an implemented capture pipeline.

## Distinct interaction choices

1. **Mark now, explain only when useful.** Read rendered prose, mark noteworthy passages, optionally comment locally, give a final reaction. Minimize switching from reading to explanation. Bare marks may leave consequential ambiguities, but they are already useful location evidence.
2. **Anchored short comments.** Select or place cursor and type a fragment. Better at preserving the immediate reason; incurs repeated composing and UI transitions. No requirement to write complete sentences.
3. **Direct edits as examples.** If the desired wording is already clear, change it rather than explaining the change. Agent retains the diff; a replacement is not automatically a general editorial rule.
4. **Section-end debrief.** Read a coherent section uninterrupted, then comment while context is fresh. Candidate compromise between constant interruption and forgetting local reactions at document end.
5. **Marks followed by agent interpretation.** Agent reconstructs likely issues and asks only where an ambiguity changes a consequential decision. User time includes any resulting rereading. Interpretations never replace the original bare marks or become human-confirmed diagnoses by default.
6. **Comparison.** Read two versions and judge preference, with ties/neither permitted. Useful for choosing repairs; doubles some reading and does not establish either version is acceptable. Not a substitute for full document feedback.
7. **AI-prepared issues for correction.** Potentially cheap recognition task, but changes the task and anchors attention. Reserve for assisted repair checking, not independent reader evaluation.
8. **Spoken comments with location capture.** Could reduce typing but historically awkward for Jörn. Optional experiment, low priority. Continuous think-aloud differs from occasional comments and may slow the primary task.

For full-feedback requests, stopping after the first decisive defect is incomplete unless the user explicitly chooses a screening task. Unmarked text remains unjudged; explicit coverage records what was read without inventing certification.

## Concrete surfaces

| Surface | What is established | Open limitation |
|---|---|---|
| Micro / CriticMarkup | Local bindings map Ctrl-k to `criticComment`; Lua inserts `{>><<}` and moves cursor inside. Raw comments and direct-edit diffs are straightforward to preserve. | Point comments lack explicit ranges; raw mathematical notation may slow reading. No new pane or editing copy opened yet. |
| PDF annotation | Standard annotation metadata, geometry and underlying page text are extractable with PyMuPDF. Preserve crops for formula or ink ambiguity. | No ready extraction pipeline was found in the bounded local search; base Python lacks fitz, pypdf and pdfplumber. Chromebook app export has not been round-trip tested. |
| Hypothesis | Official docs describe select+h for highlights, select+a for comments, Page Notes, and exports including private highlights. | Bare highlights are private, even in a group. Export/authenticated access is required. Serving/extension setup and actual per-mark ergonomics remain untested. |
| Rendered reader with paragraph markers | Design possibility: click/key to mark a paragraph, optional range selection and comments; avoid exact selection when paragraph scope is enough. | Custom implementation, not an existing delivered tool. Coarse anchors may create extra clarification. |
| Screen/audio recording | ChromeOS supports window capture with microphone and click/key display. | Transcription and passage anchoring are not validated here; user's experience weighs against priority. |

## Evidence and its limits

- [Fox, Ericsson and Best meta-analysis](https://pubmed.ncbi.nlm.nih.gov/21090887/): concurrent verbal reporting generally increased task times; unprompted think-aloud and requests for explanations had different effects. This does not establish the best review modality for mathematical prose.
- [RichReview research](https://www.microsoft.com/en-us/research/publication/richreview-blending-ink-speech-and-gesture-to-support-collaborative-document-review/): formative evidence supports combining pointing and voice in document review; not a demonstrated current Chromebook product or universal time saving.
- Audio-feedback studies do not supply a universal speed advantage: [Evans and Palacios](https://aisel.aisnet.org/mcis2010/28/) report 40–50% less time, whereas [Voelkel and Mello](https://www.tandfonline.com/doi/full/10.11120/beej.2014.00022) report longer preparation and more feedback. These concern teacher feedback, not this task; the latter was accessible to the researcher through indexed text, with direct access blocked.
- [Bouwer et al.](https://www.frontiersin.org/journals/education/articles/10.3389/feduc.2018.00086/full) found faster pairwise than analytic writing assessment in their study. This is evidence about assessment, not complete actionable prose feedback.
- [Hypothesis annotation types and shortcuts](https://web.hypothes.is/help/whats-the-difference-between-an-annotation-and-a-highlight/), [exports](https://web.hypothes.is/help/exporting-and-importing-annotations/): concrete browser alternative with recoverable marks and comments.
- [PyMuPDF annotation API](https://pymupdf.readthedocs.io/en/latest/annot.html), [page API](https://pymupdf.readthedocs.io/en/latest/page.html): highlight recovery requires document text under individual quadrilaterals; annotation appearance text is not the highlighted passage. One bounding rectangle may include unrelated lines.
- [CriticMarkup specification](https://github.com/CriticMarkup/CriticMarkup-toolkit): `{>>comment<<}` and `{==highlight==}` are available syntax. Preserve raw source; do not require comments with every highlight.
- [ChromeOS screen recording](https://support.google.com/chromebook/answer/10474268?hl=en), [Screencast](https://support.google.com/chrome/a/answer/11972236?hl=en): native capture exists, but extraction feasibility is distinct from complete end-to-end validation.
- [W3C Web Annotation model](https://www.w3.org/TR/annotation-model/): exact quote plus prefix/suffix and position provides stronger anchoring than position alone. Retain immutable original and feedback bytes regardless of normalized representation.

Hypothesis specifics: [keyboard documentation](https://web.hypothes.is/help/creating-annotations-using-only-your-keyboard/) describes focus behavior; [local PDF instructions](https://web.hypothes.is/help/annotating-locally-saved-pdfs/) require extension permissions and selectable text. Annotations live separately from the PDF; returning the PDF alone does not return them. Export uses sidebar Share → Export, then format/export or clipboard. Its hosted PDF proxy must reach the source, so a private tailnet URL cannot be assumed to work. Initial setup is amortizable; repeated export and selection costs remain relevant.

[Kami](https://help.kamiapp.com/kami-help-center/comment-tool) supplies anchored voice comments as a ready-made alternative, with paid-feature and export uncertainties; not prioritized given Jörn's voice experience. Native [Chrome PDF help](https://support.google.com/chrome/answer/16215622?hl=en&co=GENIE.Platform%3DDesktop) and [Gallery help](https://support.google.com/chromebook/answer/12453461?hl=en) establish annotation tools but do not establish semantic highlight extraction on this device. Test one returned annotation before relying on it.

## Current recommendation, not a decision

Favor a silent rendered-reading workflow with bare marks, optional short comments and one final debrief as a starting hypothesis. Keep Micro as a genuine competitor, especially for prose-heavy passages and keyboard operation. Browser annotation adds a concrete option beyond PDF transfer. Consider paragraph-level marking if precise trackpad selection proves a recurring cost. Keep voice optional.

Compare total human time through closure, not just time to press Done. A small trial should expose ordinary prose and some mathematics; use different comparable passages to reduce rereading advantages and avoid claiming rigorous results from a tiny convenience sample. Record follow-up effort and whether feedback was sufficient for the intended decision. Do not build a new annotation app merely to conduct this comparison.

## Candidate routing after UX discussion

Root's reconciled plan at `/workspaces/msc-math/.worktrees/review-workflow-design/docs/review-workflow-design/reconciled-plan.md` recommends the existing full two-page DS reconstruction for absolute structural acceptability. This is more consequential than another local HKO comparison because the historical HKO chapter already passed. Treat DS as the next candidate, not a scheduled review or an instruction to interrupt the interface discussion. HKO remains available for a short interface check. No new human label exists.

Provenance correction from that plan: HKO's added derivative disclaimer was partly induced by shared repair instructions and evaluation criteria. Negative wording itself was not required; its human acceptability remains unknown. Do not characterize it as wholly unsolicited in later packets or interpretations. The existing exact candidate text remains unchanged.

## Tested outcome and reusable entry point

The [human-prose-review skill](../../.agents/skills/human-prose-review/SKILL.md) now owns the reusable exchange. It contains short invocation commands, with substantial serving and API mechanics in two scripts. PDF uses the official Hypothesis/PDF.js viewer pinned to commit `87d50c92347d3d3b8034a18cc10282af1189931b`; HTML embeds Hypothesis and Markdown is rendered first. No extension is required.

Jörn confirmed PDF opening/annotation on his device. Authenticated API retrieval recovered its test comment and all 12 earlier HTML annotations; the latter matched the pasted export across eight substantive fields. The packaged helpers were additionally checked for PDF/HTML/Markdown delivery, MathML output, viewer dependencies, restricted file serving and exact retrieval of the 12 comments. These checks preserve the tested mechanisms, but are not a new browser/device acceptance test of every generated link. The API token stays outside the repository and served page.

Current user flow: open link, annotate, say done. Copy/export is a fallback. The earlier proposed desk requirement and Markdown-only candidate were superseded; the inbox remains useful for parallel producer coordination.
