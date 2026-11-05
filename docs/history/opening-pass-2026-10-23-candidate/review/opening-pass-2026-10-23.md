# Opening pass, 2026-10-23

Scope: abstract, introduction, opening of the DS chapter. Causes: 1 = AI style/vague/filler/signposting; 2 = narration/chaos; 3 = depends on material outside the PDF / undefined term. Each entry is a replaced passage (may span several sentences).

## 1. chapters/00-abstract.tex (cause 1,2)
Old:
> The placement of product planes supplies another source of variation:
> every symplectic product of two full-dimensional planar convex bodies admits
> a capacity-increasing rigid rotation, while the Lagrangian endpoint can
> have smaller capacity. The proofs and computations use finite capacity
> formulas, their first-order variation, and an implementation of
> Haim--Kislev's quadratic program.

New:
> Rotating the planes of a product also changes capacity:
> every symplectic product of two full-dimensional planar convex bodies admits
> a capacity-increasing rigid rotation, while the Lagrangian endpoint can
> have smaller capacity. The proofs rest on finite capacity
> formulas and their first-order variation; the computations use an
> implementation of Haim--Kislev's quadratic program.

## 2. chapters/00-abstract.tex (cause 3)
Old:
> symplectic ridge area has a strong inverse association

New:
> the total symplectic area of the two-dimensional faces, normalized by volume, has a strong inverse association

## 3. chapters/01-introduction.tex (cause 1)
Old:
> investigates these questions through local analysis, finite algorithms and
> data science. This is the sense in which we continue to probe Viterbo's
> conjecture: although its proposed bound is false, the geometric relationship
> between symplectic capacity and volume remains to be understood.

New:
> investigates these questions through local analysis, finite algorithms and
> data science. Although the proposed bound is false, the geometric relationship
> between symplectic capacity and volume remains to be understood, and the
> thesis continues to probe it.

## 4. chapters/01-introduction.tex (cause 2)
Old:
> The mathematical results identify two ways of changing the counterexample
> that cannot improve its ratio: sufficiently small ten-facet perturbations
> and arbitrary independent linear deformations of its regular-pentagon
> factors. They also exhibit a distinct non-HKO local maximum in the same
> ten-facet space, at ratio one. These are different restrictions and examples:
> the first is local in the full ten-facet space, the second is global within a
> product family, and the third is a separate product body with a non-strict
> local maximum. Finite capacity formulas make these questions accessible to
> proof. The empirical

New:
> The mathematical results are of three kinds. First, sufficiently small
> ten-facet perturbations of the counterexample cannot improve its ratio; this
> is local in the full ten-facet space. Second, arbitrary independent linear
> deformations of its regular-pentagon factors cannot improve it; this is global
> within a product family. Third, a distinct non-HKO local maximum exists in the
> same ten-facet space, at ratio one; it is a separate product body with a
> non-strict local maximum. Finite capacity formulas make these statements
> provable. The empirical

## 5. chapters/01-introduction.tex (cause 1)
Old:
>  The general counterexample does not
> invalidate this equivalence for the restricted family. Capacity--volume
> questions therefore retain significance beyond the original universal bound.

New:
>  The general counterexample does not
> invalidate this equivalence for the restricted family.

## 6. chapters/01-introduction.tex (cause 1)
Old:
> The finite task is to construct enough
> such bounds and check their derivatives. Feasible weights in Haim--Kislev's
> capacity formula provide the bounds. An algebraic computation establishes the derivative
> conditions; the proof explains why those conditions imply the geometric
> local maximum.

New:
> It therefore suffices to construct enough
> such bounds and check their derivatives. Feasible weights in Haim--Kislev's
> capacity formula provide the bounds. An algebraic computation establishes the derivative
> conditions, and the transversality and uniform-estimate argument above turns
> them into the geometric local maximum.

## 7. chapters/01-introduction.tex (cause 2)
Old:
> We derive the finite formula from the dual action principle, expanding
> Haim--Kislev's argument for a master's-level reader. We state the imported
> variational results and explain the existence of a minimizing curve with a
> finite list of distinct facet directions.

New:
> The finite formula follows from the dual action principle
> (Section~\ref{sec:preliminaries}), which expands Haim--Kislev's argument. It
> states the imported variational results and shows that a minimizing curve with
> a finite list of distinct facet directions exists.

## 8. chapters/01-introduction.tex (cause 3)
Old:
> Symplectic ridge area gives a strong inverse association with the computed
> systolic ratio in the sampled populations.

New:
> In the sampled populations, the total symplectic area of the two-dimensional
> faces (the ridges), normalized by the square root of the four-dimensional
> volume, has a strong inverse association with the computed systolic ratio.

## 9. chapters/01-introduction.tex (cause 1)
Old:
> The later sections document visualizations, numerical methods, code and data,
> and reflect on the use of AI. The appendices contain computational detail
> for checking or reproducing the results.

New:
> The later sections cover visualizations, numerical methods, code and data,
> and the use of AI. The appendices contain the computational detail needed to
> check or reproduce the results.

## 10. chapters/08-data-science.tex (cause 2)
Old:
> We sample random polytopes and compute a broad range of readily available
> geometric quantities, including symplectic invariants and measurements
> that can change under symplectic transformations. We look for statistical
> patterns relating these quantities to the systolic ratio, then ask what
> geometry might explain them. Such patterns can suggest a conjecture, but
> an explanation must also account for the bodies on which the pattern fails.

New:
> This section reports statistical patterns between the systolic ratio and
> geometric quantities of random polytopes. The quantities include symplectic
> invariants and measurements that can change under symplectic
> transformations. Such patterns can suggest a conjecture, but
> an explanation must also account for the bodies on which the pattern fails.

## 11. chapters/08-data-science.tex (cause 3)
Old:
> A strong association concerns the total symplectic area of the
> two-dimensional faces, normalized by volume. In the broad samples,

New:
> The total symplectic area of the two-dimensional faces, normalized by
> $\sqrt{\operatorname{vol}_4}$ (the quantity $R$ defined below), is strongly associated with the
> systolic ratio. In the broad samples,

## 12. chapters/08-data-science.tex (cause 1,3)
Old:
> Changes to the shapes of polygon factors
> support a geometric contribution to the broad association, without
> establishing its cause.

New:
> Area-preserving linear changes to the polygon factors of products
> weaken the association, which supports a geometric contribution to it
> without establishing its cause.
> % CHECK: 'weaken the association' pools the 314/320 covariance-balancing result; confirm it is stated that way in the DS chapter.

## 13. chapters/08-data-science.tex (cause 1)
Old:
> These findings also distinguish two tasks in the search for large systolic
> ratios. A geometric measurement can select a population with larger ratios
> on average without indicating how to improve a body already in that
> population. The final part of the chapter examines search procedures that
> use capacity evaluations to propose further improvements, and the limits
> of numerical tests for local optimality.

New:
> These findings distinguish two tasks in the search for large systolic
> ratios. A geometric measurement can select a population with larger ratios
> on average without indicating how to improve a body already in that
> population. Search procedures that use capacity evaluations to propose
> improvements, and the limits of numerical tests for local optimality, are
> treated at the end of the chapter.

## 14. chapters/08-data-science.tex (cause 2)
Old:
> A separate reevaluation now supplies

New:
> A separate reevaluation supplies

## Restructure pass (introduction and abstract), 2026-10-23, later session

Basis: `docs/preliminary-checks-2026-10-23.md` (main) and its source
`docs/history/review-evidence/calibration/whole-thesis-reading-1700.md`.
Jörn's 14 Sep points there are relayed by an agent, not quoted: "should make
the thesis contribution explicit", proof summary "too detailed", DS overview
"too detailed", "missing literature review", wants the "indirect significance
of Viterbo's conjecture" and the expository contribution named. The phrase
"contribution unclear, too much detail" is the preliminary-checks doc's
summary, not his verbatim wording.

New structure of `chapters/01-introduction.tex` (was ~1985 words, 4 pp.; now
2 pp., PDF pp. 5-6 of `thesis/build/main.pdf`):
1. Conjecture as one inequality for convex K in R^4; why it matters (Gromov
   width / stronger capacity conjecture, least action, symmetric Mahler via
   AKO2014); status (HKO counterexample) and two open questions.
2. 1.1 Contributions: numbered list, ten items, each one sentence with type
   and section/theorem reference.
3. 1.2 Relation to the literature (AEK2024, AB2023, HaimKislev2025Zoll).
4. 1.3 Outline.
No bibliography entry added; all citations were already in the intro.

### What moved where
- "Capacities, dynamics and convex geometry" paragraphs on Gromov width,
  cylindrical capacity, Edtmair's surface-of-section result, AEK coincidence,
  AKO/Mahler: moved verbatim (minor connective edits) to a new unnumbered
  `\subsubsection*{Context: capacities and the volume bound}` at the end of
  the Viterbo inequality in `chapters/02-preliminaries-ehz-capacity.tex`
  (label `subsec:preliminaries-capacity-context`). A one-sentence version of
  each point stays in the introduction.
- AB2023 Zoll paragraph, Haim--Kislev cuts paragraphs and the
  capacity/volume balance display: moved to new
  `\subsection{Relation to other local results}` at the end of
  `chapters/07-hko.tex` (label `subsec:hko-related-local-results`), with
  `K_HKO` renamed `K_0` to match that chapter.
- HKO proof-strategy paragraph (upper bounds, negative derivatives, uniform
  estimate): deleted; it duplicates "The proof strategy" in
  `chapters/hko-lemma-geometry.tex`.
- Hexagon--quadrilateral details (24 vertices, certificate): deleted; in
  Theorem `thm:non-hko-ten-facet-local-maximum` and its closing remark.
- "From boundary dynamics to finite calculations" (dual action, three-facet
  reduction, flow-graph cross-check, pentagon profile, affine extension,
  symmetric partner, product rotation): reduced to contributions 3-8;
  details already in Sections 2-5, 9-12.
- DS subsection (ridge association, selection, 314/320 covariance
  balancing, edge-width laws, 64-start local-search comparison): reduced to
  contribution 10; all numbers are in `chapters/08-data-science.tex`
  (l.388 for 314/320, l.530 for 64 starts, eq. `ds-pentagon-ridge-capacity`).
- "How to read" paragraph: replaced by a six-sentence outline.
- Abstract: same claims reordered as statement, status, theorems, methods,
  data; removed the selection, covariance-balancing and branch-retention
  sentences (detail Jörn called too much for an abstract on 14 Sep); added
  the flow-graph sentence's correctness qualifier; removed "Thus", "also".

### Contribution claims and where each was checked
1. HKO local max, ten facets, equality cases: Theorem
   `thm:hko-ten-facet-local-maximum` statement, `chapters/07-hko.tex` l.16-22.
2. Hexagon--quadrilateral non-strict local max, ratio 1, two-parameter
   family: Theorem `thm:non-hko-ten-facet-local-maximum`, `07-hko.tex` l.65-81.
3. Rotated pentagon formula and maximum: Theorem
   `thm:rotated-pentagon-product-formula`, `09-rotated-regular-polygons.tex`
   l.53-65.
4. GP x HP capacity formula, bound (3+sqrt5)/5, equality cases: Theorem
   `thm:pentagon-affine-capacity`, `10-affine-pentagons.tex` l.22-40.
5. Symmetric partner: capacity = lambda_B(K) (decagon scale), bound
   (10+6sqrt5)/25<1: Theorem `thm:pentagon-symmetric-partner`,
   `10b-symmetric-partners.tex` l.38-47.
6. Rotation out of symplectic position increases capacity; Lagrangian
   endpoint smaller in triangle example: Theorem
   `thm:product-initial-rotation` and Example `ex:product-endpoints`,
   `11-product-position.tex` l.1-47, 159-181.
7. Expanded HK derivation; at most three facets per factor: dual action
   Theorem `thm:preliminaries-clarke-dual-action` (`02-duality.tex`), HK QP
   theorem (`04-quadratic-program.tex` l.85), Theorem
   `thm:quadratic-program-product-six-facet` (l.501-507). "Expand" follows
   the old intro's own wording and Jörn's 14 Sep request; I did not claim
   "self-contained" because the preliminaries import variational results.
8. Flow-graph correctness under regularity; rational implementation agrees
   with QP on selected 5-7 facet inputs: Theorem
   `thm:exact-flow-graph-correctness` (`05-flow-graph.tex` l.256) and
   l.466-470. No performance claim made (body disclaims one).
9. First variation: Proposition `prop:first-order-volume-variation`, Lemma
   `lem:first-order-feasible-section-upper-function`, Proposition
   `prop:first-order-regular-branch` (`06-variation.tex` l.48, 121, 185); the
   HKO bounds U_sigma in `hko-lemma-bounds.tex` l.10-17 are the same
   feasible-section functions; the chapter intro (l.9-15) states both uses.
10. Ridge-area association, weakening under selection, no ratio above one:
   `08-data-science.tex` l.12-19, l.557, l.578-580.
No contribution claim was dropped for lack of support; none was added
beyond what the old introduction or abstract already stated.

### Two-minute examiner read of PDF pp. 5-6
Passes on structure: statement, significance, status, numbered results with
theorem numbers, literature, outline. Remaining risks: the ten-item list is
long and Jörn has called lists a "typical AI bad-writing pattern" (DS,
14 Sep); the significance paragraph is dense; unverified by Jörn.

### Follow-up: contributions as prose
On coordinator request (Jörn's 14 Sep objection to lists), the ten-item list
became two paragraphs: Theorems 7.1, 7.8, 9.1, 10.1 named first; Theorems
11.1, 12.1, the methods (Sections 2-6) and the data result (Section 8)
grouped after. Claims are the same as checked above, except that the
decagon-scale capacity identity and the triangle example are no longer
mentioned in the introduction. The significance paragraph was cut to
three sentences (least-action sentence removed). Text of the PDF pages:
`review/opening-pass-2026-10-23-intro.txt` (PDF pp. 5-6).
