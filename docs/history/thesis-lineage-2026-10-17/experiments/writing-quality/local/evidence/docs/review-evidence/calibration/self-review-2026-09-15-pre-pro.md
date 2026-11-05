# Ten-minute self-review, before seeing Pro's review

Review began 2026-09-15 14:36:17 UTC. Substantive review ended before the
14:46:17 cutoff; the freeze clock check returned 14:46:18 UTC. No Pro review
had been received or read. This header's timestamp correction changes no findings.
Subject: the frozen September 14 18:00 checkpoint, 86 pages.
PDF SHA256: `d7dae9a78dffc87fe89f3005bed9b6b51d28728fa90e1a5c811bc09b236e1695`.
All page references below are the printed PDF page numbers.

## Conditions and scope

Jörn has already judged the whole thesis FAIL, while describing it as much
closer than previous candidates. His Pro review was withheld. I have not read
it. This reviewer authored/integrated parts of the thesis and knows the earlier
feedback, so this is a context-informed self-review, not a blind experiment.
The prior FAIL judgment also means my overall grade is not an independent
prediction. Individual passage-level findings can still be compared.

I read the PDF's extracted text across all chapters and appendices, with deeper
attention to data science, the pentagon argument, and the computational
appendices. Some large extraction outputs were truncated; this is not a claim
of exhaustive line-by-line verification. I visually inspected pages 9, 45, 46,
52, 58, 63, 81, 83 and 84. I also read the pentagon classifier's source and
retained summary and ran the bounded diagnostic described below. I did not
rerun either full theorem certificate or externally verify the bibliography.
No thesis source or frozen PDF was edited. Existing feedback records were read
only to distinguish previous findings from new ones.

“New” means identified in this review relative to the feedback records and
context I inspected. I did not search every past agent session, and do not claim
that no earlier agent ever noticed one of these points.

## Overall assessment

I would not recommend approving the whole writeup on this review. The strongest
obstacle I can substantiate is the empirical chapter's failure to make its
central scientific observation inspectable and memorable, together with missing
method detail needed to reproduce the headline prediction result. Several
other sections retain the voice of an internal verification report. These are
not interchangeable with mathematical falsehood: I did not establish that a
main theorem is false, or find a counterexample to one, in this review.

I cannot explain Jörn's entire FAIL judgment without his remaining reasons.
Nor does a ten-minute pass certify the mathematics in sections where I found
no new objection.

## Findings

### F1. The reader never gets to see the central data-science pattern

**Priority: high exposition issue. Confidence: high.** Pages 50–51, Sections
8.3–8.4, and the synthesis on page 57.

The chapter reports a Spearman correlation of −0.938, a Pearson correlation of
−0.205, within-group correlations, prediction scores, and subsequent selection
effects. There is no plot of ridge sum against systolic ratio anywhere in
Chapter 8. There is also no displayed fitted relationship or clear account of
its shape, spread, and outliers. The gap between rank and linear correlation
is itself interesting, but the reader is only told that the relationship is
not well described by a straight line.

Consequently the mathematical reader cannot inspect the phenomenon that the
chapter proposes to interpret. The exact edge–width identity explains the
descriptor, not its association with capacity. The cube/HKO comparison refutes
universal inverse monotonicity, but no restricted conjecture or explicitly
failed quantitative extrapolation replaces it. The text honestly acknowledges
that last limitation; a new theorem is not necessary for repair.

**Repair:** show the existing measurements in a group-aware scatter plot,
including known reference bodies where appropriate; describe the actual shape
and failure of the relationship; organize the retained observations around
the resulting mathematical question. If no precise conjecture was reached,
state that outcome clearly instead of letting the experiment inventory stand
in for the missing interpretation. Do not manufacture a conjecture history.

**Novelty of detection:** the broad narrative/discovery-versus-search problem
was already identified by Jörn. The concrete absence of a visual or functional
description of the central pattern in this revised chapter is a new finding
in this pass, not a newly discovered mistake class.

### F2. The headline prediction result is not reproducible from the writeup

**Priority: high evidence/exposition issue. Confidence: high for missing
specification; no claim that the score is wrong.** Page 50, Section 8.3,
and pages 67–68, Section 12.

The random-forest experiment says it used 39 descriptors (27 combinatorial,
12 ridge summaries) and a split by capacity-source label and facet count,
with 8,192 training and 6,144 test rows. The full feature definitions, actual
held-out group identities, and fitting configuration are not given here or
in the empirical appendix. A reader cannot reconstruct what was trained from
those counts. The gradient-boosting comparison then uses a different feature
list and split without specifying them sufficiently either.

Section 12 explicitly says that the method-facing invariant table and source
datasets are unavailable in a plain checkout. The qualifications are honest,
but do not supply the missing methods/evidence. The historical-target caveat
in Section 8.1 further limits interpreting the scores as relationships with
mathematical capacity. This is a reproducibility limitation, not proof of
leakage, invalid targets, or a false correlation.

**Repair:** provide a compact feature/split/configuration specification and
identify the exact retained data and producer version for these claims. If the
required inputs cannot be recovered, narrow or demote the affected result and
put the strongest reproducible observation in the foreground. Do not merely
add another caveat to the current paragraph.

**Novelty of detection:** missing historical provenance and release material
were already known. The particular inability to reconstruct the reported
prediction experiments from the feature/split description is a sharpened
finding, not a claim to have newly discovered the underlying availability gap.

### F3. The DS chapter still reads largely as an experiment ledger

**Priority: medium-to-high exposition issue. Confidence: high.** Pages 52–57,
Sections 8.5–8.11, especially the adaptive-sampler details after the chapter's
summary; contrast the very brief synthesis on page 70.

The sequence moves through concentration filtering, Williamson covariance,
generic-body transfer, tangential products, paired height changes, rotations,
adaptive sampling, seven optimizers, and endpoint probes. Each gets detailed
counts, allocation rules, caveats, and sometimes timings. The final paragraphs
recapitulate those outcomes rather than establish which two or three findings
the mathematician should carry forward. After “What the experiments
contribute,” the chapter resumes implementation detail including Gaussian
coordinates, variance floors, tie-breaking, admission bounds, and literal seeds.

This is not an objection to reporting negative results. It is an objection to
the relative space and ordering given to explanatory outcomes and experimental
bookkeeping. The two research aims are now explicit, but the organization has
not fully followed through on their distinction.

**Repair:** separate the geometric-discovery argument from the search-method
comparison; group related tests around a question and takeaway. Move the
adaptive implementation recipe and most allocation/provenance detail to an
appendix or precise method reference. Retain the qualifications necessary to
interpret each result. A short comparison table can replace repeated numeric
inventories without pretending unlike experiments share one benchmark.

**Novelty of detection:** recurrence of the already-known narrative/detail
problem. The placement of the post-summary recipe is a concrete new example.

### F4. The pentagon proof repeatedly switches from mathematics to audit prose

**Priority: medium exposition issue. Confidence: high.** Pages 60–62 and
75–79, especially the paragraphs beginning “The classifier is fail-closed”
and “The unlimited run checks”.

The proof has a sensible mathematical outline and a useful worked branch.
It then asks the reader to track classifier exit labels, source digests,
assertion configuration, success markers and a software trust boundary in the
middle of the argument. Those operational facts do not by themselves explain
the finite comparison. Repeating that various computations are not proof inputs
also interrupts the main line.

**Repair:** isolate a finite-computation lemma specifying exactly what was
verified for the complete word family, give its mathematical use in the proof,
and move the executable identification/invocation details into the appendix.
Keep the singular-system and exceptional-parameter reasoning in the proof:
those qualifications are mathematically necessary, unlike repeated reporting
of successful program execution.

**Novelty of detection:** new occurrence of Jörn's already-known objection to
code-advertisement/administrative prose. Not a new theorem defect.

### F5. The HKO appendix is executable code, but not yet a good annotated lesson

**Priority: medium exposition issue. Confidence: high.** Pages 83–85, Appendix C.

The blocks are linked to the right lemmas and contain the assignments, which
is genuine progress. But the code itself remains compressed: `section` combines
conversion, several assertions and setup on semicolon-packed lines; `drow`
uses short recycled names and a long return expression for the ratio derivative.
The displayed wrapping sometimes breaks strings or splits a method invocation.
A mathematically mature reader still has substantial work to identify each
mathematical object and its check inside the listing. “Complete” is not the
same as readable or copy-and-run from PDF extraction.

**Repair:** expand the existing program with descriptive names, ordinary line
structure and short comments at the actual checks, retaining the same formulas
and exact data. Separate the large assignment table from the explanatory
program spine. Supply an exact companion source identifier. Rerun that same
source after reformatting; no new witness search is needed.

**Novelty of detection:** Jörn had explicitly requested an annotated CAS
program. This finding evaluates the delivered remedy; it is not a claim that
the appendix is missing or that its successful execution was fabricated.

### F6. The introductory proof summary loses the symmetry qualification

**Priority: medium/local exposition correction. Confidence: high.** Page 6,
Section 1.2: “For each nearby direction, at least one of these bounds decreases.”

The stated mechanism is transverse to symmetry. On a symmetry orbit the ratio
is constant, so a dominating touching bound cannot fall below the base ratio;
Lemma 7.4 also explicitly makes all derivative rows vanish on the symmetry
tangent space. The summary should not suggest strict first-order decrease in
every direction. The full proof does not make this mistake.

**Repair:** say that the bounds detect decrease transverse to the symmetries,
and that a local symmetry decomposition extends the comparison to the whole
neighborhood. This is a small repair, not evidence against Theorem 7.1.

**Novelty of detection:** new passage-level finding in this pass.

### F7. Some remaining presentation defects are directly visible

**Priority: low, not standalone FAIL grounds. Confidence: high.**

- Page 9, Figure 1: the tangent-plane label is partly hidden by the action
  annotation box. Text extraction's strange Greek characters elsewhere were
  not treated as visible corruption: inspected rendered figures use correct
  symbols.
- Page 64 contains only the three-line tail of the visualization discussion;
  page 80 contains only the short lead-in to the branch plot. These are awkward
  page breaks, not evidence that the plots or mathematics are wrong.
- Page 52 switches to `J = -J_0` for Williamson eigenvalues without explaining
  the convention change. The pair of eigenvalues `±iν` is unchanged, so this
  is unnecessary notation friction rather than a sign error in the invariant.

**Repair:** targeted layout changes and reuse `J_0` where the sign is immaterial.
**Novelty of detection:** new specific examples; layout/notation quality were
already-known review categories.

## Investigated concern, not a confirmed theorem error

The singular-system classifier printed on page 79 tests feasibility of one
particular stationary solution on sign cells. It computes the kernel in some
failure branches, but the final positive-gap return has no explicit kernel
guard. In a general parameter-dependent singular KKT problem, positivity of
one chosen representative need not describe feasibility of the whole affine
solution set. Thus the blanket sentence that every unresolved singular case
becomes manual review deserves an explicit justification for this family.

I inspected `executable_proof.sage.py` at source SHA256
`5cb004f3df24a27cb5bf6a6bd4b2f281e17edf834bbbb25895f88c31f239aed4`,
which matches the retained full transcript. A diagnostic generated all 3,340
words and specialized their KKT systems at the exact interior value `t=1/10`.
Among solvable systems, 470 had a kernel that could change beta; each had zero
quadratic value at that parameter and the solver's chosen beta was not strictly
positive. No movable-beta system with nonzero Q was found. The diagnostic
completed normally.

This weakens the suspicion substantially. It is not a full symbolic proof over
the interval and does not establish that the acceptance logic is wrong on any
actual word. I therefore do not count this as an established mathematical
defect. A clean follow-up is to verify that all positive-gap accepted branches
have unique beta generically, or otherwise verify their affine feasibility
sets. The retained full run takes about 2,614 seconds and was not rerun within
this ten-minute budget.

## Previously known items that are not new findings

- The wrong abstract ratio name, the older opening objections, missing HKO
  lemmas/figures/program, and the incorrect AI-history specification were
  already reported. The frozen candidate includes repairs; they are not counted
  again as newly discovered omissions.
- HKO and Chapter 4 had earlier chapter-level PASS judgments. Those do not
  establish whole-thesis approval, but neither should they be erased by this
  review.
- Broad historical-data provenance, immutable-release, novelty-attribution and
  final personal-review limitations were already recorded before this review.
- The revised abstract/introduction/conclusion and DS chapter did not already
  have human PASS judgments.

## Comparison with the withheld review

Keep this report unchanged after receiving Pro's review. Match findings by
actual passage and reasoning, not just topic. Distinguish an already-known
problem, a new occurrence of a known class, a new problem, a sharper diagnosis,
and a useful repair. A speculative concern is not a detection unless verified;
Pro may add value by resolving one as well as by finding another issue.
Jörn's judgment of whether each exposition finding matters is still needed.
This single unequal-budget comparison cannot establish a general model ranking
or that human review has become replaceable.
