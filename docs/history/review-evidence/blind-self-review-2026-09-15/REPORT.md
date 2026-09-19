# Blind self-review before the withheld Pro review

## Boundary and verdict

Review date: 2026-09-15. Started at 14:14:06 UTC; final freeze time is recorded in FREEZE.txt. Approximately ten minutes, one agent, no delegated second opinion. No manuscript edits, experimental reruns, certificate reruns, or external review were used. I consulted the fixed PDF and our own existing claim/source/review records to distinguish prior knowledge from new observations.

Artifact: `/workspaces/msc-math/.git/codex/thesis-review-1800/main.pdf`, 86 pages.

SHA256: `d7dae9a78dffc87fe89f3005bed9b6b51d28728fa90e1a5c811bc09b236e1695`.

Jörn says Pro reviewed “the submitted candidate.” I use this latest identifiable fixed candidate as that artifact; I have not independently matched an external upload hash. All page references below refer to this PDF. The Pro review remains withheld and unread.

**This pass finds concrete revision needs but does not demonstrate a fatal error in a main theorem. It does not establish a complete diagnosis of Jörn's overall FAIL.** The clearest problems are incomplete experimental exposition and incomplete integration across chapters and the available evidence. I should not turn those into a claim that the mathematical results are false. Nor should a failure to find a proof error in ten minutes count as certification.

## What was already known before this review

- **K1 — Reader experience and narrative.** Jörn had found the first sample comfortable at sentence level but narratively chaotic. His intended discovery sequence was broad geometric measurements, statistical patterns, semantic reconstruction, tentative conjecture, and rigorous explanation. A complementary high-ratio search frame was legitimate. Later feedback rejected a confusing opening and unnecessary enumerations. The latest opening sentence received specific positive feedback; the rest of DS did not receive blanket approval. These were already explicit in `ds-first-wave/writing/jorn-reading-feedback-2026-09-14.md` and the conversation.
- **K2 — Historical empirical limits.** Missing original datasets/producer trees, numerical rather than fully certified ratios, repeated use of the same datasets, finite probe limitations, and limited seed coverage were known. Most are now honestly disclosed. Their mere existence is not a new discovery, and absent artifacts are not negative scientific results.
- **K3 — An explicit unresolved body-definition dependency.** `ds-first-wave/writing/chapter-source-map.md` said the CH2021 example needed a preceding definition/cross-reference, or its explicit six-vertex construction. The fixed PDF still calls it “the CH2021 example” on p56 without that construction. This is a known integration task left unresolved, not a newly discovered research problem.
- **K4 — Uncommitted new evidence.** `ds-first-wave/writing/ownership-fold-request.md` explicitly transferred four new uncommitted method directories and results. It did not establish their publication. No whole-thesis PASS existed. Previous chapter-level approvals and scoped source checks did not cover the assembled thesis.
- **K5 — Remaining mathematical ambition.** The ridge descriptor has a geometric interpretation, but no established general capacity explanation. This was already known and is stated in the current chapter. A successful data-science theorem is not something I can impose after the fact as a graduation requirement.

## Findings retained from this pass

### N1 — The status of branch-window evidence is not reconciled across chapters

**Priority: substantive exposition/evidence integration. Confidence: high in the textual mismatch, not a demonstrated false numerical claim. Newly identified here.**

On p38, §6.4 introduces the finite-gap branch-window model (34), then says it has not received corresponding reproducible finite-distance validation and “is not used as thesis evidence here.” On pp55–56, §8.9 describes a finite-gap model retaining branches within ten percent of the minimum, reports its outcomes, and compares it with history models. Appendix A supplies further finite-step diagnostics.

There may be a defensible distinction between the particular model in §6.4 and historical implementations whose executable provenance is incomplete. The PDF does not connect those distinctions. A reader cannot tell whether the later table addresses the earlier model, whether the earlier disclaimer is stale, or whether “evidence” changes meaning between chapters.

**Needed repair:** identify which implementation corresponds to (34), cross-reference its experiment, and distinguish retained numerical evidence from reproducible executable validation. Do not silently upgrade the historical evidence.

### N2 — Several empirical results omit method choices needed to interpret them

**Priority: substantive methods exposition. Confidence: high. Newly noticed omissions; some underlying facts were already in our ledgers.**

- **p51, feature-family comparison:** R²=.887 versus .043 is given under a “common training/test split,” but that experiment's split construction and sample sizes are absent. The preceding random-forest split is explicitly different. Our existing ledger contains the P2 counts (8,704/5,632); possession of those facts did not ensure they reached the thesis. The reader needs the held-out unit and split design to assess generalization, not just the scores.
- **pp53–54, rotation allocation:** the fourteen prescribed orthogonal rotations are never specified by planes/angles or a schedule identifier. The conclusion is correctly limited to this schedule, making the missing schedule particularly consequential. A small table or a precise accessible recipe would suffice.
- **pp52–53 and p56, intervals:** interval endpoints and some resampling mechanics are supplied, but their confidence/nominal coverage level is not stated. The seed-by-group t interval also needs a clear account of its uncertainty target and assumptions. The existing caveat that twenty groups are not twenty seeds is useful; it is not the entire inferential explanation. I have not shown these intervals numerically wrong.
- **p56, fixed diagnostic suite:** the rotated-square angle and precise triangle–hexagon construction are not supplied there; the CH body omission is the already-known K3. These choices matter when the outcome is absence of improvement around a specific body.

These are omissions in the thesis, not evidence that the underlying experiments were never specified or performed. A thesis need not print every configuration constant, but it must define the comparison or point to a stable, identifiable recipe.

### N3 — The availability account does not identify the new pilots' evidence

**Priority: potentially substantive reproducibility gap. Confidence: high for omission/local tracking status; public/submitted availability not established by this review. New cross-check of an already-known uncommitted state.**

Pages 67–68 describe tracked proof packets and selected historical evidence, explicitly naming the 1,675-row selection experiment. They do not identify the new 64-evaluation height pilot, 124-request rotation comparison, or 1,344-evaluation adaptive sampler, despite these occupying important parts of §8. A general warning that some inputs are untracked does not tell a reader how to find those experiments.

At review time, `git ls-files` returned no tracked files for `paired-tangentialization`, `orientation-allocation`, or `diagonal-cem-pilot`; `git status` listed these directories as untracked. The prior handoff also identified them as uncommitted. This establishes a local retention/publication question, **not proof that no submitted attachment or external copy exists**. I did not inspect a remote repository or the actual submission bundle.

**Needed repair:** establish the submitted/public evidence inventory, then provide exact packet locations and immutable identities for the reported pilots. If unavailable, state that particular limitation. Do not claim that an ordinary checkout contains them without checking.

### N4 — The flow-method summary loses distinctions made correctly in its chapter

**Priority: medium, attribution and evidence framing. Confidence: high in wording; no demonstrated mathematical error. Newly identified here.**

The introduction (p6) and conclusion (p70) summarize the implemented alternative as the Chaidez–Hutchings flow graph. Section 5.4 (p34) carefully says the algorithm borrows local geometry but has a different completeness argument and does not implement their pruning/classification as its capacity proof. Section 5.5 (p35) also separates the retired approximate prototype from the current rational implementation.

The p6 claim that performance made the method less useful for large searches does not identify which implementation or point to a timing comparison; §5.5 gives selected agreement tests but no corresponding performance result. This leaves the reader uncertain which method was evaluated and which conclusion belongs to it.

**Needed repair:** consistently name it as the thesis's flow-based algorithm, explain its relationship to CH, and qualify or support the performance statement. This is not an accusation of plagiarism or a finding that the correctness theorem fails.

### N5 — Implementation detail still interrupts the mathematical/experimental story

**Priority: medium editorial/pedagogical; assessment rather than a falsifiable theorem defect. New concrete examples of the previously known reader-experience problem.**

- **p30:** the useful explanation of the flow figure ends with JSON fields, raw construction-chart vertices, and a serialization boundary. The essential reader-facing content is that panels have independent coordinates and the start/return panels share a frame. Artifact schema belongs in reproduction documentation.
- **p34:** the genericity proof jumps from five explicit vectors and two determinant values to a repeated-word cancellation and irreducible/Zariski-open argument. The crucial cancellation deserves an explicit calculation, and the density step an explanation at the intended master's-reader level. I have not found the argument false.
- **p57:** the chapter reaches “What the experiments contribute,” then restarts with a substantial unnumbered adaptive-sampler specification. Keep that necessary specification beside §8.8 or in an appendix and let the synthesis end the chapter.

I do not conclude that the revised DS narrative remains as chaotic as the original sample. Its two aims and the ridge-to-width reconstruction are now explicit. The remaining defects are more local and should be described as such.

### N6 — Minor polish and pagination remain unfinished

**Priority: low. Newly observed here.**

Page 64 contains only the final three lines of the visualization section followed by essentially a full blank page; this was checked in a rendered image, not inferred from text extraction. Page 47 similarly has only the final theorem-proof paragraph in extracted text, but was not rendered in this pass. Page 52 contains “describe! its symplectic scales.” These are easy production repairs, not evidence against the research.

A text-extraction anomaly in Figure 3's Greek labels was checked against rendered p16: the Greek letters are actually rendered correctly, so that suspected defect was discarded.

## U1 — A specific unresolved certificate-audit question

**Potentially important if reachable; not a confirmed proof defect. First noticed near the end of this review.**

The classifier printed on pp78–79, and the corresponding `executable_proof.sage.py` lines 405–490, checks positivity cells of one returned stationary solution. On the nonzero-gap path, it inspects the kernel when that particular solution has no feasible cells, but does not inspect it before accepting a positive number of feasible cells. If a singular system admitted other positive solutions at parameters where the returned particular solution is not positive, those parameters would also need the gap comparison. Constancy of Q on the stationary affine space establishes a common gap at each parameter, but by itself does not establish that the particular solution detects the entire feasibility domain.

This might be harmless for the actual family: for example, every accepted nonzero-gap case may have unique beta, or the gap may be positive on the whole relevant domain. The solver's particular-solution convention may also rule out the problematic path. The retained summary records classification counts, not the rank/feasibility fact needed to settle this question. I did not establish reachability or run a new audit. Therefore this is an explicit uncertainty for subsequent checking, **not a claim that the pentagon formula or certificate is wrong**. A narrowly instrumented exact check of the accepted nonzero-gap cases could resolve it.

## Checks that did not yield a defect

I read the central normalization/reconstruction chain in §§2.6–3.3, including pure-velocity splitting, merging, rescaling and passage to a simple minimizer. I checked the structure of the six-facet product reduction, first-order branch qualifications, the HKO row-neighborhood/symmetry-slice argument, and the pentagon proof's exceptional-parameter continuity closure. No demonstrated error emerged. In particular:

- Singular fixed-word KKT systems are not simply assumed invertible; the manuscript explains constant objective value on their stationary affine solution sets.
- The HKO proof uses smooth feasible sections rather than asserting that every optimizer continues smoothly.
- The pentagon proof addresses exceptional parameter ranks by equality on a dense open set and continuity.
- The cube/width comparison really supplies a counterexample to universal negative monotonicity; it is not merely a failed statistical test.
- Bibliography and formal statements are present. The latest abstract does not claim that ordinary data science is exhausted.

These are limited reading checks, not independent theorem verification. I did not rerun or fully audit the Sage programs, verify every cited theorem against its source, inspect every figure, reproduce statistics, check every historical table, or match every source file to the fixed PDF. The review was weighted toward our DS responsibility and chapter interfaces.

## What this says about our earlier review process

The earlier DS review checked many numerical claims accurately against a claim ledger and resolved real wording errors. Its final scope explicitly excluded some source/bibliography integration. The problem is that scoped success was not followed by a complete integration check: an explicitly flagged body definition remained missing, uncommitted evidence was handed off without an established publication inventory, and cross-chapter evidence status stayed unreconciled.

This is a diagnosis supported by the examples above, not a claim to know every reason the thesis fails for Jörn. I should not call all of N1–N6 previously known merely because they were discoverable from files we already possessed. Conversely, the Pro review repeating K1–K5 would not be novel substantive information just because it phrases it better.

## Frozen comparison plan

After this report is frozen, compare each Pro finding against K1–K5, N1–N6, and U1. Distinguish: genuinely new issue; stronger evidence or sharper scope for an existing issue; better explanation/actionable repair; overlap; and unsupported or false positive. New proof defects must be checked on their merits. Do not retrospectively expand this baseline after reading Pro. A single comparison measures incremental value on this artifact and task, not general superiority of one model or interface.
