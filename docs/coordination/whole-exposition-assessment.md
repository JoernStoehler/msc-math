# Whole-thesis exposition gap assessment

Assessment date: 18 September 2026. Artifact: frozen 93-page
`docs/whole-review/thesis-v3-writing.pdf` (SHA-256
`249f597a4094add726f077b83538301ce7394cd42d62420e3970aee4fd683fda`).

This is a diagnostic assessment of reader-facing mathematical exposition, not a
submission verdict. It excludes the data-science and AI-use chapters, which have
separate owners. It also does not independently re-prove the theorems, validate
the implementations, or replace the one-time human grading of the final PDF.

## Decision-level result

The abstract is not ready at the requested exposition bar. Its mathematical
content can be retained, but its final paragraph needs an abstract-level rewrite
that establishes a hierarchy among the empirical results rather than inventorying
them. The introduction and conclusion need targeted sentence-level scope and
navigation repairs; they do not presently require structural rewriting.

The chapter-region findings below distinguish local repairs from regions where a
larger rewrite is justified. A prior cold reading that found only two minor issues
is treated as bounded evidence, not as a human-writing pass: the present reading
uses the stricter bar requested after that calibration proved too permissive.

## Abstract (printed p. 1)

**Assessment: fail; targeted rewrite of the abstract, especially its final
paragraph.** The repair should be two or three coherent paragraphs and can keep
the same scientific claims. It does not require restructuring the thesis.

1. **Medium — undefined abbreviation.** “Our main theorem proves that the HKO
   body is a local maximizer” uses `HKO` without introducing the abbreviation,
   although the preceding sentence spells out the authors' names. Define
   “Haim--Kislev--Ostrover (HKO) body” on first use. Targeted edit.
2. **Medium — implicit comparison.** “Nearby equality occurs precisely under
   translations, positive dilations and linear symplectic transformations” does
   not say nearby to which body or equality in which bound. State that equality
   in the local systolic-ratio bound for bodies near the HKO body has exactly
   these cases. Targeted edit.
3. **Medium — vague antecedent.** “A flow-graph algorithm ... gives a second
   method under explicit regularity assumptions” does not say what it is a
   second method for. Name capacity computation or closed-trajectory search.
   Targeted edit.
4. **High — result is not stated.** “Geometric selection enriches fresh samples”
   omits the target of enrichment. State that selection increased the incidence
   or distribution of high computed systolic ratios, at the strength actually
   supported by Chapter 8. Targeted edit.
5. **High — interpretive claim has no named outcome.** The covariance-balancing
   sentence says the experiment supports “a contribution from affine changes in
   factor shape” without saying a contribution to the ratio distribution or to
   the ridge--ratio association. Name the outcome and retain the sample-bound
   qualification. Targeted edit.
6. **High — inventory replaces hierarchy.** The last paragraph gives six results
   in rapid succession: ridge association, selection, weakening under
   conditioning, covariance balancing, two exact rotation laws, and optimizer
   comparison. The principal empirical answer and its limitation are not
   distinguishable from method detail. Compress around one leading empirical
   result, one qualification, and the negative search outcome; present the exact
   family laws as constraints on interpretation. This needs paragraph-level
   rewriting, not isolated synonym changes.
7. **Medium — negative search result is buried.** “None of the exploratory
   searches produced a new numerical ratio above one” is the main outcome of the
   search strand but appears only after an implementation comparison. Integrate
   it into the sentence that introduces the search result.

## Introduction (printed pp. 5--8)

**Assessment: targeted edits suffice.** The introduction supplies motivation,
states the two principal theorems early, distinguishes their scopes, and gives a
usable reading route.

1. **Medium — false deictic navigation (p. 6, Section 1.1).** “The theorem below
   establishes this balance throughout a neighborhood” points to a theorem in
   Chapter 7, roughly forty pages later. Name and reference the local-maximality
   theorem directly. Targeted edit.
2. **High — opaque empirical outcome (p. 7, Section 1.4).** “Selection based on
   geometric measurements also enriches fresh samples” repeats the abstract's
   omission of what the samples are enriched for. State the measured outcome.
   Targeted edit.
3. **High — sample evidence stated generically (p. 7, Section 1.4).** “Affine
   shape variation of the factors contributes to the broad association” precedes
   the finite-sample evidence but reads as a population-independent conclusion.
   Restrict the claim immediately to the tested sample. Targeted edit.
4. **Medium — evaluative paraphrase obscures the result (p. 7, Section 1.4).**
   “This identifies a useful shape deformation” adds an undefined judgment after
   the concrete 314-of-320 result. Delete it or restate exactly what the
   transformation did in the experiment. Targeted edit.
5. **Medium — empty transition (p. 8, Section 1.4).** “Direct local search gives a
   separate positive result” classifies the outcome before stating it and does
   not tell the reader what was positive. Lead with the matched-start comparison.
   Targeted edit.

## Conclusion (printed p. 77)

**Assessment: targeted edits suffice; optional compression.** The conclusion
does more than announce completion: it restates the theorem scopes, records
quantitative empirical outcomes, and ends with open questions. Its length is
high for the amount of synthesis, but a chapter rewrite is not necessary.

1. **Medium — method generalized beyond the evidence.** “The local proof also
   gives a useful way to work at a singular optimization problem” presents a
   broadly useful method on the basis of this instance. State what the proof does
   at HKO, or frame the construction as a possible strategy rather than an
   established general method. Targeted edit.
2. **Low — category mismatch.** “Two mathematical questions follow directly”
   introduces one mathematical local-extremality question and one partly
   empirical, sampling-dependent explanation question. “Two open questions” is
   accurate. Targeted edit.
3. **Low — repetition.** The finite-method and empirical paragraphs substantially
   repeat the introduction and chapter summaries. A 15--25 percent compression
   would improve emphasis, but this is optional and should not displace the
   concrete fixes above.

## Regional coverage and findings

### Chapters 2--3: preliminaries and generalized Reeb orbits (pp. 8--20)

**Assessment: targeted notation repairs suffice. Confidence: high after checking
the apparent page-break issue directly.**

- **Medium, p. 19, Lemma 3.7 proof:** the formulas use `|I_r|`, `|I_s|`, and
  `|I_i|` without defining this as segment duration. Define it or use the
  established dwell-time notation.
- **Low/medium, p. 11:** “the Reeb direction associated later with a support
  row” is a vague forward reference before support rows are introduced. Name
  the normalized facet row or defer the sentence.

### Chapter 4: Haim--Kislev quadratic program (pp. 21--28)

**Assessment: targeted proof insertions suffice. Confidence: high.**

- **Medium, p. 21:** after defining `\beta_i^{\rm here}`, the text immediately
  uses bare `\beta_i`. Explicitly drop the superscript.
- **High, p. 25:** “Rudolf's nonsmooth billiard theorem gives a minimizing
  Minkowski billiard...” introduces the central billiard/dual-polygon objects
  without a local definition or a statement of the imported theorem's usable
  input and output. Add a compact definition and theorem consequence.
- **High, p. 25:** “These relations say precisely that the alternating broken
  line ... is a generalized characteristic” hides the normal-cone calculation
  and why the line lies on the product boundary. Supply that bridge.
- **Medium, p. 25:** `\ell_{K_p}(q)` is not defined, and the support function of
  a `q`-displacement under the `q/p` pairing is unexplained. Define the length
  and the identification.
- **Medium, p. 26:** “Apply the splitting--merging--rescaling part of the proof of
  Theorem 3.4” makes the reader recover the operation from an earlier proof.
  The following paragraph does preserve the block invariant explicitly, so no
  logical step is missing. Add a one-sentence cyclic-word description before
  that paragraph; a structural rewrite is not justified.
- **Medium, pp. 27--28:** connect positive objective to nonzero mass in both
  factors; state why a sufficiently small dependence perturbation stays
  nonnegative; and say that the two vertex replacements preserve the positive
  global maximum. These are three targeted one-sentence proof bridges.

### Chapter 5: flow graph (pp. 29--34)

**Assessment: local structural rewrite of the genericity proof plus targeted
clarifications. Confidence: high.**

- **Medium, p. 30:** “concatenate primitive tubes for
  `(s_1,\ldots,s_k,s_1,s_2)`” does not explain why the extra two labels close
  the tube. Explain that a primitive consumes an ordered triple and returns to
  the initial codimension-two section.
- **Medium, p. 30:** “the active word records exactly the positive-duration
  pieces” leaves the relation between zero-time tube passages and the strict
  active word implicit. State that zero-time passages are omitted.
- **Medium, pp. 30--31:** Example 5.2 sends its exact rational data only to
  “the flow-graph data described in Section 14.” Give a canonical artifact path
  and distinguish illustrative rounded quantities from certified ones.
- **Medium, p. 32:** “The two arguments preceding the theorem” should name the
  strict transition-sign and short-word linear-independence arguments.
- **High, p. 33, Proposition 5.7:** the replacement
  `x,A,B,A,y \mapsto x,A,y` uses undefined `x,y`, domains, and “middle
  transition.” Give a displayed primitive-map identity and explain why equality
  on `\ker a_A` preserves the surrounding return map.
- **High, p. 33:** the specializations `(A,B)^m,C,D,E` appear to contradict the
  definition of a simple word. State that they are algebraic witnesses for the
  unrestricted rational function, not searched words.
- **Medium/high, p. 33:** “The common denominator domain is a nonempty
  Zariski-open subset of an irreducible affine parameter space” never names the
  parameter space or establishes nonemptiness. Define both and connect
  nonemptiness to the explicit witness. List the claimed nonzero pairings or
  point to an exact reproducible calculation.

### Chapters 6--7: variation and HKO local maximality (pp. 35--48)

**Assessment: targeted certificate signposting; the exact finite certificate
and slope-to-neighborhood argument can remain. Confidence: high after direct
source verification.**

- **Medium, pp. 43--45, Lemma 7.4:** “The exact computations for all twenty-six
  were verified using a computer algebra system” initially reads as procedural
  substitution for proof. Direct source verification shows, however, that the
  main text supplies the exact assignment table, formulas for every row, the
  predicate list, and the implication to Lemma 7.4, while Appendix B prints the
  verifier. This is an intentional finite certificate, not a missing proof.
  Add one orienting sentence mapping the four groups of checks to the lemma's
  touching, symmetry, rank, and positive-relation clauses.
- **Medium, pp. 43--45:** explain why 26 bounds are natural for the 25-dimensional
  transverse quotient and why singular seven-facet feasible sections give
  smooth bounds even when stationary optimizer branches are unavailable.
- **Low, p. 45:** the exact witness table is four pages later. The source already
  gives an explicit page cross-reference, but a numbered caption would improve
  navigation; relocation is unnecessary.
- **Medium, p. 45:** map each finite predicate explicitly to independence, rank,
  symmetry annihilation, and the positive relation in Lemma 7.4.
- **Low/medium, pp. 37--44:** identify the KKT block matrix used for the IFT;
  connect the slice `S` to the later symmetry complement; and add the derivative
  check in the worked seven-facet quadratic. Targeted bridges.

### Chapters 9--11: exact product families (pp. 60--67)

**Assessment: targeted proof expansions; no structural rewrite. Confidence:
high after direct source verification.**

- **Medium, pp. 61--62:** show the four ordered symplectic pairings behind
  “sum to `2u\cdot v`,” and replace “This angular dependence determines the
  value” with the explicit endpoint interpolation formula and coefficient sign.
- **Medium, p. 63:** spell out the successive bilinear maximization at closure-
  polytope vertices, the 2/4/6 alternating-group count, why two groups have zero
  value, and the `q`-prefix calculation. These are separate short bridges.
- **Medium, p. 64:** justify the positive-triple angular-gap fact by the
  origin-in-triangle argument.
- **Low/medium, p. 64:** “It follows ... that the convex hull of the four-group
  matrices is exactly `\rho D`” is terse. The surrounding text already supplies
  both directions (all grouped sums lie in `sD`; every generator is feasible),
  so add only the explicit bilinear convex-combination sentence.
- **Medium, p. 64, Lemma 10.3:** define the cyclic cut and the steps represented
  by `v,z,w` before the six-case table.
- **Medium, p. 65:** justify the polar-decagon identity and slow the
  automorphism/equality classification by one or two sentences. Direct source
  verification shows that the paragraph beginning “Every equality body in this
  family” already gives the symplectic map, rescaling, angle congruences, and
  reflection, so this is density rather than a missing argument.
- **Medium, p. 66:** identify the reversed planar order, derive the mixed-weight
  factor `4t(1-t)`, map the three terms of (47) to symplectic-form blocks, and
  state the capacity squeeze instead of “the bounds agree.”
- **High, p. 66:** “Polygonal approximation and continuity of capacity extend
  ... to arbitrary planar convex bodies” is the only bridge to the theorem's
  full generality. Name Hausdorff approximation, explain convergence of the
  relevant quantities, and cite/state capacity continuity.

### Chapters 12--14 and Appendix B (pp. 68--73, 90--91)

**Assessment: targeted additions and artifact clarification; no section-level
rewrite is justified. Confidence: high after direct source verification.**

- **Medium, pp. 68--69:** the visualization chapter explains projection
  mechanics and reports no reliable pattern, but never gives Figure 11 a
  reader-verifiable analytical purpose. State what each panel illustrates and
  what the reader must not infer.
- **Medium, pp. 70--71:** Section 13 rigorously states its contracts and proves
  the inverse-defect lemma, but has no worked certified interval, action
  decision, or capacity output. One compact input/result example would make the
  contract concrete. The procedure is the chapter's subject and is not a reason
  for structural rewriting.
- **Medium, p. 72:** audit counts (249 KKT systems, 88/1271 comparisons, 88 and
  10,240 products) lack artifact identifiers. Attach each to its canonical
  packet/path and say whether it is tracked or external.
- **Medium, pp. 72--73 and 91:** Section 14 advertises
  `experiments/hko-local-maximum/theorem/`, while Appendix B names
  `thesis/candidate/recovered/hko-cas-appendix/hko_core.py`; the latter is absent
  from the cited public commit, which contains a different verifier. Section 14
  already says later additions live only in the working repository, so the
  availability claim is not false. Add an explicit mapping between the
  appendix's exposition program and the public verifier.
- **Medium, pp. 90--91:** the 26 assignments break across pages into bare rows;
  add a continuation marker/line numbers or move the full listing to the artifact.
- **Medium, p. 91:** add a plain-language boundary between program-verified
  rank/kernel/symmetry facts and the geometric implications supplied by Lemmas
  7.2--7.7.

| Region | Coverage | Current judgment | Largest repair scope |
|---|---|---|---|
| Abstract | Complete close read | Fail | Abstract-level rewrite of final paragraph |
| Introduction | Complete close read | Repairable | Targeted sentence edits |
| Chapters 2--3 | Complete delegated read | Repairable | Targeted proof bridges |
| Chapter 4 | Complete delegated read | Repairable | Targeted proof insertions |
| Chapter 5 | Complete delegated read | Repairable | Local rewrite of Proposition 5.7 proof |
| Chapters 6--7 | Complete delegated read + lead verification | Repairable | Targeted certificate signposting |
| Chapter 8 | Excluded: separate DS owner | Unknown here | Not assessed |
| Chapters 9--11 | Complete delegated read + lead verification | Repairable | Targeted proof bridges |
| Chapters 12--14 and Appendix B | Complete delegated read + lead verification | Repairable | Worked example + artifact mapping |
| Chapter 15 | Excluded: separate AI owner | Unknown here | Not assessed |
| Conclusion | Complete close read | Repairable | Targeted sentence edits; optional compression |
| Appendix A | Excluded with DS | Unknown here | Not assessed |

## Smallest disjoint repair batches

These batches are source-disjoint and can be assigned in parallel after the
coordinator selects the recovery slice. They are not authoring assignments yet.

1. **Front/back matter:**
   `recovered/thesis-candidate/00-abstract.tex`,
   `01-introduction.tex`, and `14-conclusion.tex`. These repairs need no new
   author decision: foreground the two mathematical restrictions and the
   negative search outcome, with the ridge signal and exact-family reversal as
   one qualified empirical strand. Optional conclusion compression can be
   skipped.
2. **Foundations:** `02-preliminaries.tex` and
   `03-generalized-reeb-orbits-polytopes.tex`. Targeted proof bridges only; no
   missing author choice.
3. **Quadratic program:** `04-quadratic-program.tex`. Product-reduction
   signposting plus local definitions; no missing author choice, but the Rudolf
   theorem summary must stay within the cited theorem's actual statement.
4. **Flow graph:** `05-flow-graph.tex`. Rewrite only Proposition 5.7's compressed
   algebraic witness argument and add local navigation/artifact references; no
   missing author choice if the exact artifact path exists.
5. **Variation/HKO:** `recovered/thesis-candidate/06-variation.tex` and
   `07-hko.tex`, coordinated with the existing appendix source
   `recovered/hko-cas-appendix/hko-cas-appendix.tex`. The certificate is already
   complete; the repair is reader orientation, not mathematical reconstruction.
6. **Exact families:** `09-rotated-regular-polygons.tex`,
   `pentagon-affine-draft/section.tex`, and
   `product-rotation-draft/section.tex`. Targeted proof bridges; no missing
   author choice.
7. **Computation/reproducibility:** `11-numerics.tex`, the visualization and
   availability sources, and `recovered/hko-cas-appendix/hko-cas-appendix.tex`.
   The active `main.tex` names `10-visualization-3d.tex` and
   `12-published-code-data.tex`, but those paths are absent in the current
   checkout; only `legacy/` copies are present. Resolve their active source
   ownership before assigning this batch. Artifact-path reconciliation may also
   require a repository-publication decision; the worked §13 example does not.

## Unknowns and limits

- No claim is made about Chapters 8 or 15 or Appendix A in this report.
- The region reads assess what a mathematically prepared MSc reader can follow;
  they do not re-run certificates or verify cited literature.
- Page references are printed PDF pages in the frozen v3 artifact.
- The final one-time human grade must be applied only after selected repairs are
  integrated and a new final PDF is frozen.
