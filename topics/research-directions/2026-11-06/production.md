# Revision and checking record

The current reading copy is [report.html](report.html), generated from [report.md](report.md). Its math and fonts are embedded, so it does not need network access or JavaScript.

## The first two rejected deliveries

The first delivery contained unrendered mathematical notation. Jörn relayed a failing readability grade; the grader skipped the other categories because even partial assessment required too much effort. The separate text review had not included a rendered inspection.

The second delivery fixed rendering and several mathematical statements, but Jörn again relayed a failing readability grade. His example was “The strongest portfolio is not a sequence of record-breaking polytopes.” The report still made readers interpret unsupported evaluations and project shorthand before understanding the research. The reviews had missed that problem. Neither of those approvals establishes acceptance of the report.

## Whole-prose rewrite (v5)

The prose was rewritten throughout. It now begins with the mathematical contrast between local HKO rigidity and simultaneous-cut improvement. Each direction explains the question, the argument that makes it plausible, and the obstacle that remains. The detailed cut coordinates and finite-word optimization are in appendices; the main discussion keeps their scientific consequences.

The rewrite retains the substantive corrections from the previous revision: the established facet-splitting theorem and its rank assumptions; the exact quaternionic action and its anti-symplectic generator; explicit feasible weights and the global maximum in the capacity formula; and the distinction between upper bounds, certificates of improvement and local-maximum criteria.

A new reader diagnosis identified recurring prose defects across the report. A separate review of the rewrite rejected v4 for four remaining issues: treating nonpositive first derivative as an obstruction to all improvement; overstating what the eleven/seventeen-facet results locate; omitting the exact flow-graph independence condition; and omitting relevant established symmetry conclusions. Those were corrected. The reviewer then approved v5, with a scope limited to the reading and reasoning checks performed. The receipts are in [review.md](review.md). No user grade for this version is established.

## Executed checks on the v5 reading copy

- All 170 mathematical expressions passed KaTeX 0.16.22 with strict parsing and errors treated as failures. A scan found no remaining raw pseudo-LaTeX patterns of the kinds that caused the first rejection.
- The HTML was loaded from disk in headless Chrome. At widths of 980 and 390 pixels the page had no horizontal overflow; the narrow-screen check found no overflowing prose paragraphs. Long display equations can scroll within their own blocks.
- The DOM contained 170 rendered math elements and no KaTeX error elements. No script or externally loaded font/style resource is needed.
- Parent inspected screenshots at every main section and appendix opening, including the cut calculation, local-maximum proof, planar covering argument, symmetry discussion, product transfer and optimization definition. The fresh reviewer also inspected rendered passages. These checks cover representative views, not every pixel of the long document.
- Local source links and contents anchors resolved. The reading copy provides a collapsible contents list; the opening prose remains visible before the technical setup.

Representative v5 screenshots and inspection metadata are in [evidence/](evidence/). Renderer source and dependency lockfile are in [render/](render/). The [file guide](README.md) gives rebuilding instructions.

## Scientific and cost limits

The report uses selective source reading, worked deductions and independent investigation. No new numerical capacity campaign or full certificate replay was run. It is not an exhaustive literature search, a novelty certification, or an independent proof audit of every cited theorem. Scientific claims retain their local or external source qualifications.

The original production used three returned investigation workers after a restart cancelled the first set. Revisions used formatting, substantive-audit and reader-review work, plus parent rewriting and rendering. All workers finished. The wall-clock span from the original start to v5 was approximately fifty minutes, including the restart and gaps between user responses; it is not a measurement of active computation time. Total tokens and billing are unavailable. No research campaign, publication, external message or thesis adoption was launched.

The two failures consumed additional production and reader effort. Rendering success and review approval are recorded as specific checks, not substitutes for the report's usefulness to its reader.

## Layered reading revision (v6, current)

After a further readability rejection concerning narrowed scope and the burden of reading technical details, the current report separates the main reading from optional technical notes. The opening describes the broader programme: inequalities and deformations, planar covering, trajectory dynamics, methods and explanatory work. The main report explains the cut and covering mechanisms, unresolved implications, recommendations and conditions that would change them; readers need not reconstruct that argument from the notes. Targeted links lead to the relevant derivations and exact hypotheses.

A fresh reviewer read the main report standalone, inspected representative rendered sections and the mobile opening, and checked the targeted note destinations. Two wording changes qualified strict negativity to nonzero speeds and removed an unsupported claim that a particular check was cheapest. Review v6 approved the resulting report and outgoing delivery. The receipt is retained in [review.md](review.md). This is bounded approval, not user acceptance or a full theorem audit.

The current main-page inspection recorded 22 rendered math elements, zero KaTeX errors and desktop width 980 matching its viewport. Its representative screenshots and inspection metadata are retained in `evidence/v6/`; the earlier evidence remains labeled v5 above. The reviewer also inspected the mobile view. No new scientific campaign was run.

## Fourth delivery — rejected; proposal reconstruction active

Jörn's latest grade accepts the prose quality but fails readability because the
content does not function as a research proposal: track motivations, approaches,
partial progress and cross-track exchanges are not recognizable, titles conflate
different questions, and terminology is insufficiently explained. Completeness
also fails because the structure prevents assessing coverage. The v6 review did
not catch this failure. Its approval does not establish proposal readiness.

The reconstruction separates scientific questions from approaches, enabling
methods and individual experiments. A focused local source inventory has
recovered omitted empirical/geometric motivations and negative results. A
self-contained ChatGPT.com GPT-6 Pro relay prompt has been prepared and separately
reviewed to challenge
the specification and idea-space coverage before another report is delivered.
No Pro submission or response is established yet. The parent session owns
integration; the existing report remains the rejected v6, not an accepted
proposal. No numerical research campaign has been launched.

The [planning packet](planning/pro-prompt.txt) and [local evidence inventory](planning/local-evidence-inventory.md) retain the inputs for the pending reconstruction. The inventory is provisional, not the selected proposal outline.

## Fifth proposal revision — integrated Pro return

Jörn relayed a GPT-6 Pro return ZIP after the fourth rejection. The returned agenda
proposes organizing the programme around seven scientific questions and cleanly
separating questions from approaches, shared tools and investigations. The main
report now presents an overview, seven visible track cards, concrete candidate
investigations, a handoff table naming exchanged objects, coverage objections and
reallocation evidence. The detailed returned agenda and source ledger are retained
separately. Pro explicitly did not run experiments or independently audit proofs.
Its technical deductions and literature claims retain source/read-depth limits;
they need verification before being used as project results.

The main and technical Markdown were converted to KaTeX-compatible delimiters
and rendered offline with the existing build script. The main report had 28 math
expressions and the technical report 108; rendering completed without parser
errors. Headless Chrome inspected the main report: 28 math elements, zero KaTeX
errors, desktop width 980 at a 980 viewport, and representative screenshots for
all 12 section headings. At 390px viewport the document measured 446px wide
(the inspector found no overflowing prose paragraphs); likely display-equation
scroll regions require visual review. This records renderer evidence only; the
new proposal has not received reader approval or full source/proof audit.

The integration worker subsequently located the mobile overflow in wide tables,
wrapped them in scroll containers and reran the check: document and viewport
width both 390px, no overflowing prose paragraphs. It checked the complete
labeled-pairing deduction and the cover certificate direction against the
returned mathematical arguments; remaining external claims stay attributed.
Separate structural review approved proposal-v7 against the user's track,
progress, motivation and exchange criteria. This is not user acceptance.


## Main integration — 6 November 2026

Jörn authorized low-effort integration into main. The source thread's packet was
uncommitted; it was copied without importing its older shared planning snapshots.
The Markdown remains the maintained source and both standalone HTML copies are
retained. Dependency manifests, previously excluded by the repository's ignore
rules, are explicitly retained so `npm ci` works from a fresh checkout.
Integration installed the locked dependencies and rebuilt both documents: 28 and
108 formulas parsed successfully. The main HTML reproduced exactly; the technical
HTML differed only in generated table wrappers and CSS, so it was refreshed with
the current renderer. Planning-inventory relative links were corrected after
relocation into its subdirectory. Local Markdown link destinations were checked.
These are integration checks; the prior scientific and visual reviews remain
attributed historical evidence, and no fresh mathematical audit was performed.
