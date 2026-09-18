# Examiner’s report
## *Probing Viterbo’s Conjecture* — review of the supplied 86-page candidate

## Overall assessment

**The mathematical core is stronger than the manuscript’s organization, literature account and empirical evidence package.** The HKO local-maximality argument is substantial, and an independent exact reconstruction of its printed 26-bound certificate passed. I found no fatal flaw in Theorem 7.1. Nevertheless, I would ask for major revision before treating this as a finished, high-quality submission: the state of the field contains a definite error, important nearby literature is omitted, the empirical chapter requires a genuine rewrite and stronger reproducibility, and the proof architecture misses opportunities for much simpler and more illuminating mathematics.

This is not principally a copy-editing problem. Correcting a few verbs, adding references and repairing page breaks would leave the largest weaknesses intact. Conversely, the limitations of the historical experiments should not obscure the quality of the exact local theorem or be misreported as defects in that theorem.

The detailed register contains **35 anchored findings**, with factual errors, evidential limitations and editorial judgments labelled separately. Three additional verification/hardening questions are segregated rather than counted as demonstrated theorem failures. These are revision work items, not 35 independent deductions from a grade.

## Review dimensions

The review used the following coverage framework. The institution’s actual marking rubric, submission rules and oral-defense evidence were not supplied.

| Dimension | Questions considered | Assessment of this candidate |
|---|---|---|
| Research purpose and significance | Is the central question worthwhile? Is the thesis a coherent answer rather than a collection of activities? | Strong central question; insufficient hierarchy among the many retained activities. |
| Originality and scholarship | What is imported, adapted, newly proved or merely implemented? Is the relevant literature current? | Plausibly substantial mathematical contribution; materially deficient current-literature account. Priority/originality was not exhaustively established. |
| Mathematical correctness | Definitions, conventions, hypotheses, quantifiers, signs, constants, logical dependencies, existence, degeneracy, equality cases and scope | Main arguments largely withstand scrutiny; definite incorrect contextual claim on p. 5. The local certificate was independently checked exactly. |
| Mathematical explanation | Does the proof reveal the mechanism? Is its complexity justified? Are examples and dependencies well chosen? | Good treatment of dual versus boundary curves; excessive machinery for the pentagon profile and missed connections between results. |
| Computational correctness | Candidate completeness, feasibility, singular cases, exact/approximate boundaries, error bounds, trust assumptions and tests | Strong HKO packet; useful numerical design; important production-arithmetic and fallback obligations are not documented at comparable depth. |
| Empirical design and inference | Sampling laws, controls, validation, duplicate/cached observations, estimands, uncertainty, multiple exploration, confounding, stopping and failures | Many honest qualifications and several sensible controls, but uneven provenance and incomplete specifications limit the conclusions. |
| Algorithmic utility and efficiency | Practical applicability, complexity, common-input benchmarks, costs, failure rates and portability | Reductions are valuable; comparative performance is insufficiently quantified. |
| Writing and narrative | Thesis-level and chapter-level arcs, audience calibration, motivation, pacing, redundancy and synthesis | Locally careful prose; globally fragmented, particularly in the empirical and software-evidence material. |
| Presentation and navigation | Page layout, figures, tables, captions, numbering, cross-references, bibliography and digital usability | Generally legible LaTeX; conspicuous pagination, table-navigation and code-auditability defects. |
| Reproducibility and professional responsibility | Immutable artifacts, data availability, correspondence to the submitted PDF, attribution, final review and defensible mastery | Transparent disclosures, but the archive is incomplete and final author verification of this candidate is not attested. |

## The highest-value corrections

### 1. Correct the mathematical context, not just the bibliography

On p. 5 the thesis says that equality of EHZ and cylindrical capacity for convex four-dimensional domains remains open. It does not: Abbondandolo–Edtmair–Kang’s December 2024 paper gives the coincidence for convex bodies, and the later HKO preprint reports it. The correction actually strengthens the thesis’s relevance, since its EHZ computations also determine cylindrical capacity on its four-dimensional convex inputs. The distinction from Gromov width must remain. **R01** identifies the claim and the necessary replacement. [E1, E2.]

The missing literature also affects how experiments should be interpreted. Seven of the ten product side-count groups have a triangular or quadrilateral factor and satisfy the Viterbo inequality under the results described in **R02**. They can serve as controls or subthreshold geometry experiments, but not as locations for discovering a product with ratio above one. The chapter does not make that distinction. The pertinent quadrilateral result is a March 2026 preprint; this is an update required for the submitted account, not an accusation that every historical design ignored a result available at the time. [E3.]

Finally, the discussion of local maximality should engage directly with Haim–Kislev’s recent work on nonsmooth Zoll-type behavior and cuts of the pentagon product. The current smooth-Zoll comparison is not enough to situate the fixed-facet theorem and its proposed extensions. **R03** gives the specific comparison to add. [E4.]

### 2. Replace the computational profile proof with a structural argument

The five-page theorem discussion and substantial classifier appendix are not needed to prove the rotated-pentagon profile. For fixed feasible weights and a word, the objective has the form

\[
Q(\theta)=A\cos\theta+B\sin\theta,
\]

and the feasible set is independent of the relative rotation. Positive interpolation between the two known HKO angles bounds every competitor; the displayed candidate attains the bound. The full analytic proof is in `03_mathematical_audit.md`.

This matters for grading mathematical judgment and exposition. A correct exhaustive computation is legitimate, but it is not the most illuminating proof when a short argument exposes the governing structure and removes the need for thousands of cases. The computation remains useful as a record of discovery or as an algorithmic regression test. It need not remain the theorem’s logical foundation. **R05** is therefore a major proof-design revision opportunity, not a finding that the theorem is false.

### 3. Complete the missing mathematical arc between Chapters 8 and 9

The thesis asks for a restricted-family relationship between its ridge descriptor and capacity. Its own formulas already yield one. For the regular-pentagon rotation family,

\[
S(K_\theta)=16\sin(\pi/5)\cos d(\theta),
\qquad
\operatorname{sys}(K_\theta)S(K_\theta)^2=16(3+\sqrt5).
\]

I derive this in the mathematical audit. It holds both below and above one. It does **not** validate the pooled random-data correlation outside that family, and it does not contradict the thesis’s correct cube counterexample to universal monotonicity.

This is nevertheless a much stronger narrative conclusion than “the restricted relation remains to be supplied.” It connects statistical discovery, geometric interpretation and an exact theorem, using the objects already central to the thesis. The present separation of those chapters leaves that connection unexplored. **R06** is an important omission in synthesis, not a demand that the candidate solve an unrelated new research problem.

### 4. Rewrite the empirical chapter around questions and evidence, not around the order of runs

The chapter contains useful ideas and several well-designed safeguards: prospective selection before target evaluation, shared-start comparisons, explicit distinctions between numerical misses and proofs, and candid limits on transfer. These should survive revision.

The present chapter is still too close to a research log. It accumulates studies with changing sample laws, evaluators, controls, budgets and statistical units. The reader is asked to keep all of those differences in working memory while processing many reported numbers. Merely adding another caveat to each paragraph would make this worse.

A stronger version would distinguish three aims: explaining a geometric association, selecting promising bodies before expensive evaluation, and optimizing bodies after evaluation. Each needs its own baseline, outcome measure and evidence standard. The central association should be visible in plots, not only in correlation coefficients. The complex prediction models need an S-only baseline on the same split. Missing feature dictionaries, hyperparameters, split membership, normalization conventions and uncertainty definitions must be supplied. **R07, R13–R19** cover these repairs.

Historical evidence with missing producers or inputs should not be allowed to carry the primary validation merely because its limitations are carefully disclosed. A smaller new, reproducible study would contribute more than another extensive interpretation of unreplayable aggregates. This need not mean throwing away all historical work: use it as discovery material, then test the important conclusions on a clean primary panel. **R08** is the evidence-quality issue; **R09** is the distinct release/versioning issue.

### 5. Separate arithmetic theorems, program correspondence and finite tests

The inverse-defect lemma is sound. It does not itself establish that the implemented binary64 routines compute valid outward residual and defect bounds. Those formulas and assumptions, and the exact singular-system feasibility fallback, are part of the proof obligation for the advertised numerical contract.

The chapter should show enough of that correspondence that a mathematically trained reader can follow it to pinned source locations. Finite test panels are valuable for falsification but cannot fill a missing analytic error bound. At the same time, this review did not inspect every Rust path and does not claim that the bounds are absent from the code. The defect is the submission’s under-documentation of the guarantee. See **R10–R11**.

The performance claim needs a separate common-input benchmark. Timings of an adaptive-sampling workflow do not measure the relative merits of the flow-graph and QP capacity solvers. **R18** describes the needed comparison.

## How I would reorganize the thesis

The following is a substantive alternative, not a requirement to adopt these exact headings.

| Main component | Material to retain and reorganize |
|---|---|
| Introduction and contribution map | Current capacity landscape; exact main theorem; known exclusions for sampled families; an explicit distinction among new theorems, adapted proofs, implementations and observations. |
| Finite capacity on polytopes | Necessary notation and dual principle; simple-minimizer argument at the appropriate level; derive the QP rather than reimport it; give the six-facet reduction and its computational consequence. |
| Variation and HKO local maximality | Feasible upper sections, volume derivative, symmetry/slice mechanism, then the exact witness. Explain why value-sparse product minimizers do not settle all transverse derivatives. |
| Rotation geometry and the ridge descriptor | Short analytic pentagon profile; edge-width interpretation; exact restricted-family ridge law; clear counterexample to universal monotonicity. |
| Computational investigation | A small number of designed empirical questions with complete primary evidence, comparisons and costs. Historical pilot inventory moved out of the main argument. |
| Conclusions | What the mathematics establishes, what the experiments genuinely change, and prioritized open directions informed by current literature. |

The alternative flow-graph method can remain a separate optional main chapter if it is being claimed as a central contribution; otherwise it belongs in a substantial appendix. The detailed arithmetic implementation, complete witness data, exploratory branch plots, visualization and research-process reflection can be organized as supporting material. The AI disclosure itself should remain easy to find.

Within the empirical chapter, I would use a sequence such as: **design and source laws → the ridge association → a controlled geometric explanation → prospective selection tests → local-search comparison and failure diagnosis → synthesis**. The point is not to reduce the number of headings arbitrarily; it is to ensure that each experiment answers a question the preceding section has made necessary.

## Writing quality: the main issue is allocation of explanation

The text is not uniformly badly written. The distinction between closure and boundary realizability, the warning that a simple minimizer is an existence result, and the feasible-section mechanism are often explained well. The main theorem’s abstract proof architecture is also clearer than the overall thesis structure.

The recurring weakness is that explanatory space is allocated to repeated status qualifications and software-artifact distinctions while mathematical relations or experimental decisions remain implicit. The intended reader is told that a JSON chart is not a common coordinate chart, that a retained record is not a proof, that a finite miss is not local maximality, and that a witness needs a geometric identification, often more than once. These qualifications have value, but they should not become the narrative itself.

For example, the repeated validation paragraphs in §5.5 could be replaced by a more direct statement of this form:

> On the deterministic rational test bodies with five, six and seven facets, the flow-graph solver agreed with the certified QP aggregate. These tests check implementation correspondence on the listed fixtures. Completeness on the regular class is supplied by Theorem 5.4, not by the test agreement. The fixture inventory should record the words, actions and outcomes of the input and singularity checks.

The detailed inventory would then carry the specifics. This is not simply compression: it restores the order **result → evidence → limitation**, instead of interleaving those layers repeatedly.

The profile rewrite and the ridge-law proposition are more important examples of “better writing through better mathematics.” They change what the chapter explains, rather than merely replacing cumbersome sentences with shorter ones.

## Significance for the mathematical community

The local theorem addresses a natural question around the counterexample and allows arbitrary nearby ten-facet row perturbations, not only deformations of the two factors. The combination of touching feasible upper functions and a transverse positive-spanning certificate is potentially reusable. The six-facet bilinear reduction also deserves greater prominence: it is simple enough to understand independently and changes the character of product-capacity computation. These are reasons to take the work seriously.

The fixed-facet restriction remains a substantial boundary of the result. It does not settle unrestricted local maximality among convex bodies, and new facets are not merely more coordinates in the same chart. The thesis correctly says this. The right community-facing discussion should explain the boundary and connect it to existing cut-based work, rather than inflate the theorem into a stronger one or apologize for not proving it.

The empirical contribution is presently less mature. Useful selection effects can be publishable or mathematically suggestive without producing a new counterexample, but they need interpretable geometry, reproducible evidence and a clear role relative to known family bounds. Without those, the numerical results are difficult for another mathematician to use.

## Presentation and submission readiness

The ordinary mathematical typesetting is generally readable. The major visual failures are localized and fixable: nearly empty continuation pages 47, 64 and 80; an essential but unnumbered/misplaced witness table; other unnumbered result tables; dense, poorly pasteable proof code; and figures whose captions describe rendering conventions more fully than their mathematical data. The bibliography is missing from the table of contents and PDF outline. See **R27–R34**.

The release issue is larger than layout. A living repository without a thesis-matching immutable version, and with missing essential analysis inputs, is not a complete reproducibility package. A DOI is not intrinsically required, but a stable identity and accessible inventory are.

The AI disclosure is transparent and reports advisor approval. I would not penalize the thesis merely for the breadth of disclosed AI assistance. The statement on p. 2 nevertheless does not cover a completed final author review of this candidate. That must be resolved before submission responsibility can be assessed. The defense should test understanding of the dual/primal distinction, singular feasible sections, transverse slope argument and numerical guarantees—not just the ability to repeat a generated proof. Compliance with institution-specific rules cannot be inferred from this PDF alone.

## Grading judgment and order of revision

On mathematical content alone, this is credible and potentially very strong master’s-level work. It does not deserve to be dismissed because the empirical search failed to find a new counterexample. As submitted, however, the literature error, incomplete evidence package and uneven composition prevent an unqualified top assessment. A numerical mark would give false precision without the institution’s rubric and evidence of the candidate’s mastery and permitted contribution model.

The rational revision order is: first correct the mathematical context and establish final responsibility/versioning; next replace the unnecessary profile machinery and connect the ridge law; then rebuild the empirical chapter around reproducible primary evidence; finally perform the prose, figure, table and pagination pass. Doing the layout pass first would polish material that ought to be moved, reduced or replaced.

**Bottom line:** preserve the exact local theorem and the clean six-facet reduction. Spend the largest revision effort on mathematical synthesis, the empirical chapter and the reproducibility boundary—not on expanding the project further.
