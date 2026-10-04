# Current thesis work remaining

Reconciled against the selected source and the 4 October 2026 restart
continuation candidate. This remains the detailed obligation owner for `thesis/main.tex`;
[current.json](current.json) owns live assignments. The preserved checkpoint
`fe3475b3` supplied most of the source repairs below, now assembled with the
restart corrections. Exact source/PDF identity and bounded review outcomes are
in [the continuation integration record](../history/thesis-continuation-2026-10-04/integration.md)
and [candidate manifest](../history/thesis-continuation-2026-10-04/candidate-manifest.json).
The first 100-page acceptance bundle at `2704f054` remains immutable; the
101-page continuation adds missing model-feature/split details and clarifies
exact rational outputs from dyadic inputs.
A clean build or agent review does not establish human PASS. No submission or
public release is implied. Human acceptance, the overall data-science scope
classification and long-horizon reproduction completeness remain unknown.

Status terms used below:

- **Confirmed** means the issue is present in the selected source or is an
  explicit unresolved fact in the retained evidence.
- **Agent-suggested** means a bounded reader recommended the change, but no
  human verdict established that it is required.
- **Unknown** means acceptance or adequacy has not been established. Unknown is
  not itself a manuscript defect.

## Source obligations and their current disposition

### 1. Foundations and the quadratic-program reduction

**Status: source repairs completed.** The four named clarifications are now
present: normalized rows, the usable nonsmooth billiard theorem, explicit word
surgery, and the positivity/vertex-replacement argument. Independent restart
review found no high/medium defect in this scope. The original suggestions are
from `docs/history/coordination/whole-exposition-assessment.md`; they were
exposition findings, not reports that the mathematics was false.

**Selected sources:**

- `thesis/chapters/02-preliminaries-lagrangian-products.tex`
- `thesis/chapters/04-quadratic-program.tex`

**Checked source obligations:**

1. In Chapter 2, replace or clarify the premature phrase “the Reeb direction
   associated later with a support row” by naming the normalized facet row, or
   defer the observation until support rows have been introduced.
2. In Chapter 4's Lagrangian-product reduction, give a compact local definition
   of the minimizing Minkowski billiard/dual polygon and state the exact usable
   input and output of Rudolf's theorem.
3. Before invoking the splitting--merging--rescaling portion of Theorem 3.4,
   state the cyclic-word operation in one sentence.
4. In the dependence/vertex-replacement argument, explicitly connect positive
   objective value with nonzero mass in both factors, state why a sufficiently
   small dependence perturbation preserves nonnegativity, and state why both
   vertex replacements preserve the positive global maximum.

**Done means:** each named object and implication is available at the point of
use, without changing the theorem statements or making a stronger claim about
the imported billiard theorem.

### 2. Variation, HKO certificate, and local-maximality navigation

**Status: source navigation repairs completed.** The KKT block matrix,
symmetry slice, seven-facet derivative, 26-versus-25 explanation, four predicate
groups and appendix trust boundary are explicit. The assignment table has a
caption and the executable listing has line numbers. These changes explain
an existing proof; they do not treat program output as its analytic implication.

**Selected sources:**

- `thesis/chapters/06-variation.tex`
- `thesis/chapters/07-hko.tex`
- `thesis/appendices/hko-certificate.tex`
- `thesis/appendices/hko_core.py`

**Checked source obligations:**

1. Identify the KKT block matrix used for the implicit-function argument,
   connect the slice `S` to the later symmetry complement, and show the
   derivative check in the worked seven-facet quadratic.
2. Orient the reader to the 26-section certificate: explain why 26 bounds are
   natural for the 25-dimensional transverse quotient and why singular
   seven-facet feasible sections still yield smooth bounds when a stationary
   optimizer branch is unavailable.
3. Map the four groups of finite predicates to the touching, independence/rank,
   symmetry-annihilation, and positive-relation clauses used by Lemma 7.4.
4. Give the assignment listing a stable caption/continuation marker or line
   numbering, and state plainly which rank/kernel/symmetry facts are checked by
   the program and which geometric implications are supplied by Lemmas 7.2--7.7.

**Done means:** a reader can move from the main lemma to the exact assignments,
the verifier, and the geometric conclusion without treating the program as a
substitute for an unstated implication. Recomputing the certificate is not part
of this repair.

### 3. Exact product-family proofs

**Status: source proof expansions completed.** Ordered pairings, endpoint
interpolation, product closure reduction, convex combinations, cyclic cuts,
polar-decagon/equality steps, the mass bound and Hausdorff passage are now
locally traceable. Independent bounded product/foundation review found no
high/medium defect; this is not an unrestricted theorem or implementation audit.

**Selected sources:**

- `thesis/chapters/09-rotated-regular-polygons.tex`
- `thesis/chapters/10-affine-pentagons.tex`
- `thesis/chapters/11-product-position.tex`

**Checked source obligations:**

1. For rotated regular polygons, display the four ordered symplectic pairings
   behind the `2u\mathbin\cdot v` sum, replace “determines the value” by the
   endpoint interpolation formula and coefficient sign, and spell out the
   closure-polytope vertex maximization, the 2/4/6 alternating-group count, the
   two zero groups, and the `q`-prefix calculation.
2. Justify the positive-triple angular-gap assertion by the
   origin-in-triangle argument. In the four-group matrix argument, add the
   explicit bilinear convex-combination sentence; the two set inclusions are
   already present.
3. Define the cyclic cut and the steps represented by `v,z,w` before the
   six-case table. Give the polar-decagon identity and slow the
   automorphism/equality classification by the one or two missing connective
   sentences; do not restate the already-present symplectic map and angle
   congruences as a new proof gap.
4. In the product-position proof, identify the reversed planar order, derive
   `4t(1-t)`, map the three terms of the displayed bound to the symplectic-form
   blocks, and state the capacity squeeze explicitly.
5. Replace the single-sentence passage from polygons to arbitrary planar convex
   bodies by the actual Hausdorff approximation argument, convergence of the
   quantities used in the bound, and a cited or stated capacity-continuity
   result.

**Done means:** every finite reduction and limiting step named above is locally
traceable, while the theorem scopes and the retained exact/numerical distinction
remain unchanged.

### 4. Visualization, numerical example, and artifact mapping

**Status: source and layout repairs completed.** Panel interpretations,
exact cube example, separate audit-packet owners, external-data status and
appendix/public-version distinction are present. The integrated 101-page build
has no overfull boxes or undefined-reference warnings. Independent continuation
inspection covers its changed pages and binds the unchanged pages to the
earlier full inspection. All thirteen selected figures have been regenerated
from named inputs and staged into an isolated source copy; the resulting PDF
matches all 101 candidate page rasters at 96 dpi and its extracted layout text.
The blank-success WebGL screenshot failure found by that run is repaired. The
commands, copy names and limits are in [figure reproduction](../figure-reproduction.md).

**Selected sources:**

- `thesis/chapters/12-visualization.tex`
- `thesis/chapters/13-numerics.tex`
- `thesis/chapters/14-code-data.tex`
- `thesis/appendices/hko-certificate.tex`

**Checked source obligations:**

1. State what each visualization panel illustrates and what inference the
   reader must not draw from it.
2. Add one compact certified numerical example to Chapter 13: concrete input,
   returned interval/action decision, and capacity output.
3. Attach the reported audit counts (249 KKT systems, 88/1271 comparisons, 88
   and 10,240 products) to their canonical repository packets and identify
   whether each packet is tracked or an external registered artifact.
4. Map the appendix's annotated exposition program to the public verifier named
   in the availability chapter. Do not imply that the appendix file was present
   in the cited public commit if it was not.

**Done means:** every displayed figure/example/count has a reader-facing purpose
and a resolvable evidence owner, and public versus working-repository
availability is stated literally.

### 5. Data-science prose and evidence disposition

**Status: source concerns dispositioned; human acceptance unknown.** The selected
chapter and appendix are scientifically scoped and contain the completed
14,335-of-14,336 retrospective capacity coverage. The [current-source comment
disposition](../history/thesis-restart-2026-10-04/ds-comment-disposition.md)
records all twelve original annotations and later qualified/withdrawn comments.
The restart removed a surviving trivial scaling explanation, made specific
quantity references explicit, and preserved the accepted correlation and
absolute-area passages.
Human acceptance of the selected chapter is **unknown**. Jörn's explicit
“Needs revision before acceptable” verdict applied to the different ridge-only
sample retained at
`experiments/writing-quality/human-review/responses/20260918-root-contemporary-ds-reader.md`,
not to the complete selected chapter. Jörn also rejected pages 7–9 of an
  earlier complete DS draft; that version-level verdict is retained in
  `docs/history/coordination-map-review/ds-review-20260918.md`. Later revisions
  supersede that draft without supplying a new human acceptance verdict.

**Selected sources:**

- `thesis/chapters/08-data-science.tex`
- `thesis/appendices/data-science.tex`

**Evidence to use when editing:**

- `docs/history/ds-revision-reconciliation/README.md`
- `docs/history/reviewer-trial/human/microbatch-1.md` through
  `microbatch-4.md`
- `docs/ds-retrospective-revalidation/README.md`
- `docs/history/whole-review/thesis-v3-writing-changes.md`

**Completed source-disposition scope:** source-aware comparison, not wholesale
import, of the retained human concerns: result-oriented title/topic sentences;
phenomenon before population mechanics; explicit “systolic ratio” rather than
indirect labels; clear separation of empirical observation, mathematical
explanation, and selection experiments; and a mathematically useful closing.
Preserve the explicitly accepted full correlation paragraph and the accepted
absolute-symplectic-area clarification. Preserve qualifications and withdrawn
objections recorded in the microbatches. Reproduction details moved out of
reader prose must remain available in the appendix or a canonical methods
packet.

The linked disposition records applied, already-absent, contextually accepted
and alternate-draft concerns. The selected chapter preserves the
historical/current evaluator distinction. Human acceptance remains a separate
judgment; a source-level disposition does not invent it. The broader
roughly-100-method expectation remains uncertain in Jörn's intended sense,
without treating that historical approximate count as a numeric gate or
scheduling more methods.

The single body `random_F8_s3_45` may remain explicitly unresolved. Solving it,
relaxing numerical policy, or certifying volume is additional scientific work,
not a prerequisite while the thesis states the limitation accurately.

### 6. AI reflection

**Status:** no confirmed source defect remains from the ten human annotations.
The v3 source implemented them and an independent comparison found no required
correction. Chapter 15 is unchanged by the checkpoint and restart patches. Human acceptance of the additions and of the selected chapter as a
whole is unknown.

**Selected source:** `thesis/chapters/15-ai-reflection.tex`.

**Evidence:** `docs/history/ai-reflection-review/ai-reflection-v3-changes.md` and
`docs/history/ai-reflection-review/ai-reflection-v3-agent-review.md`.

**Done means:** preserve the mapped annotation repairs and verify that the
selected source still matches that map after other integration edits. Earlier
optional v2 suggestions about inherited detail or repeated examples are not
current requirements.

### 7. Selected ten-facet local-maximum claims

**Status:** L10-N and L10-N+ are promoted in the 24 September claim
disposition. The selected reader-facing statement is the subsection “A second
ten-facet local maximum” in `thesis/chapters/07-hko.tex`, included in the
selected manuscript identified by `thesis/README.md`.

**Exact evidence owner:** `experiments/hko-local-maximum/non-hko/`. Its
`README.md` identifies the rational witness and verifier and separates exact
finite checks from the analytic neighborhood argument in `PROOF.md`. For
L10-N+, preserve the explicit caveat that `-13/432` is curvature of the
selected feasible upper sections, not a Hessian assertion about the actual
capacity.

**Completed PDF/evidence check (4 October):** pages 51–53, subsection 7.7
and Theorem 7.8 of the retained 100-page candidate, match
`docs/consolidation/2026-09-24/claim-disposition.md` and the packet's trust
boundary. The theorem includes the non-strict fixed-ten-facet local maximum
and two-parameter equality family modulo the stated symmetries. The proof
identifies `-13/432` as upper-section curvature, continues the constant-rank
family, and supplies the mixed-direction and chart argument. Exact predicates
remain attributed to the rational packet; no explicit radius, added-facet or
global claim is introduced. Fresh exact and independent checks passed, and
historical evidence now resolves through the nested-archive route.

[Independent PDF acceptance](../history/thesis-restart-2026-10-04/acceptance-pdf.md)
records the exact artifact identity, all-eleven-area coverage and review depth;
[source acceptance](../history/thesis-restart-2026-10-04/acceptance-source-review.md)
records the analytic/predicate comparison. No mismatch remains in this bounded
claim check. This does not establish human acceptance or exhaustive proof
verification.

## Scientific scope disposition

- **Selected and already integrated:** the product-position initial-regime
  theorem, the exact separating endpoint/profile observation, and their
  combined interior-maximum remark in
  `thesis/chapters/11-product-position.tex`. Their research basis is
  `docs/empirical-viterbo-design/desk/product-position-research/README.md`.
  The later lift-containment counterfamily is reviewed research context, not a
  separately selected thesis theorem.
- **Selection unresolved; currently omitted:** the fixed regular
  pentagon/symmetric-partner bound. Jörn reacted positively to the class and
  result, but no retained final instruction selects it for the manuscript. Its
  exact source is
  `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/symmetric-partner-lemma.md`;
  absence from the manuscript is neither a known rejection nor a mandatory
  addition.
- **Reviewed but unselected optional results:** harmonic blind-direction
  families, ridge circulation bounds, the moment counterexample, and
  joint-representation separations under
  `docs/empirical-viterbo-design/desk/`. Their validity or interest does not
  create an integration obligation, and the retained source checks do not
  establish publication novelty.
- **Conjectural and unselected:** the general zonotope volume/Phi bound. The
  reviewed disposition is to pause it; it must not enter the thesis as a
  theorem.
- **Optional future work:** further profile catalogues, new random collections,
  more scalar-matching experiments, specialist outreach, publication-level
  novelty review, and new review-workflow experiments. None is needed to close
  the source repairs above.

No retained record conclusively selects another empirical-desk result for
thesis integration. If a later decision selects one, it must name its exact
statement and evidence packet before prose is added.

## Already repaired: do not reopen as current work

The following findings are closed in the selected source unless a later edit
regresses them:

- The abstract defines HKO, identifies the local equality statement and the
  flow algorithm's purpose, names the empirical outcomes, establishes a result
  hierarchy, and foregrounds the negative search outcome. The introduction's
  false deictic theorem reference and empirical-scope sentences are repaired;
  the conclusion's overgeneralized method claim and “mathematical questions”
  label are repaired. Evidence:
  `docs/history/coordination/repair-framing.md`, commit `44372a61`.
- Segment-duration notation in Chapter 3 and the normalized/bare `\beta`
  convention in Chapter 4 are repaired. Commit `ac8da690`.
- The Chapter 4 boundary and normal-cone bridge that realizes the dual curve as
  a generalized characteristic is repaired. Commit `4b3a8af1`. This does not
  close the separate question whether the imported billiard theorem needs a
  fuller local statement. The displayed length definition and explicit support-
  function pairing are present; do not retain them as missing notation.
- The Chapter 5 tube closure, zero-time omission, artifact reference, named
  pruning arguments, repeated-section contraction, algebraic-witness status,
  parameter space, denominator domain, pairing table, and genericity proof are
  repaired. Evidence: `docs/history/coordination/flow-exposition-repair.md`,
  commit `54143c6d`.
- The four older consistency defects (self-contained-proof promise,
  product-position matrix naming, false flow availability destination, and the
  conclusion's ten-facet quantifier) and the HKO nonsmooth-cutting literature
  comparison were repaired before the latest whole-exposition assessment.
- The DS chapter's three scoped v3 issues, the two optimizer-definition gaps in
  its appendix, and the ten mapped AI-reflection annotations were repaired in
  the selected sources. These repairs are not whole-chapter human acceptance.
- The apparent missing proofs for the product block reduction, HKO finite
  certificate, affine equality argument, and numerical certification were
  withdrawn after direct source verification. They may receive the specific
  orientation edits listed above, but must not be restated as absent proofs.

Optional conclusion compression, relocating the HKO assignment table, and
rewriting unflagged AI v1 prose are not required repairs.

## Integration and completion checks after source edits

1. Build only the selected manuscript with `sh thesis/build.sh`.
2. Resolve every manuscript path after the layout migration using
   `docs/resume/layout-migration.json`; remove no retained evidence merely
   because a historical report still names an old path.
3. Inspect changed pages and adjacent floats, references, tables, and appendix
   continuations. Record the exact source revision and PDF hash.
4. Keep `docs/resume/` frozen as historical evidence. The new
   `docs/history/thesis-continuation-2026-10-04/candidate-manifest.json` identifies
   the current retained candidate and all tracked thesis-source hashes. Recheck those
   hashes after integration; a later source edit requires a new candidate and
   explicit review disposition.
5. Freeze one exact candidate for any eventual final human judgment. The
   judgment remains unknown until it occurs; diagnostic agent reviews and a
   warning-free build do not establish PASS. No university submission or public
   release is authorized by this work list.

## Remaining acceptance and reproduction limits

- Historical human PASS remains unknown: the working threshold is Kai finding
  only minor issues Jörn could fix in about two hours. Neither agent review nor
  an inspected build supplies that judgment.
- The retained tube theorem is conditional on explicit regularity hypotheses;
  chamber-generic validity and bounded rational implementation checks do not
  certify the optimized production flow search on every input.
- The wide general-QP action-window contract received a separate targeted
  repair and regression review in the restart. This does not imply a wrong
  scalar capacity result or a complete audit of every consumer.
- The reproduction inventory distinguishes current analytic and exact routes,
  historical evidence and authenticated bulk-data materialization. The selected
  thirteen-figure producer/copy/PDF chain now passes on the installed host;
  three selected structural JSON exports were recomputed and empirical plots
  used verified retained inputs. This does not rerun historical capacity data
  or verify R2 access. The current 45-feature P2 modeling table requires its
  separate reconstruction; the historical plotting table lacks six columns.
  A fresh independent environment has not reproduced every empirical producer,
  and original timing/machine metadata is incomplete.
- Submission status/current institutional rules and Jörn/Kai review remain
  externally unestablished; no administrative action is selected by this file.
