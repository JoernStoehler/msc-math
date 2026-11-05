# Complete thesis review

The four component reports are collected here for single-file reading. The annotated PDF and independent verification scripts are supplied separately in the review package.

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


---

# Anchored findings

**Document:** *Probing Viterbo’s Conjecture*, 86-page supplied PDF. Printed page numbers equal PDF page numbers.

“Major” means a thesis-level, chapter-level or primary-evidence repair; “Substantial” means a section, method specification or important presentation repair; “Local” means a bounded correction. These are priorities, not a mechanical mark allocation. **Confirmed errors, methodological limitations and editorial judgments are labelled separately.** Q01–Q03 are verification/hardening questions and are not counted as demonstrated mathematical errors. Several findings concern different consequences of the same root problem; do not add their grading penalties as if independent. External sources E1–E6 are identified in `04_sources_and_scope.md`.

## Index

| ID | Priority | Location | Finding |
|---|---|---|---|
| R01 | Major | p. 5, §1.1; references pp. 85–86 | An advertised open problem had already been resolved |
| R02 | Major | pp. 49–57, especially §§8.1, 8.6–8.8 | The search families are not separated into those that can and cannot beat one |
| R03 | Substantial | pp. 6, 39–47, 70 | The closest nonsmooth-maximizer literature is missing |
| R04 | Major | pp. 3–7; overall organization | The manuscript does not give its strongest contribution the structural priority it deserves |
| R05 | Major | pp. 58–62; Appendix B, pp. 75–82; §§4.3, 4.5 | The rotated-pentagon proof carries avoidable computational machinery |
| R06 | Major | pp. 51–52, 57, 59–62, 70 | The thesis already contains ingredients for an exact ridge–capacity law across sys=1 |
| R07 | Major | pp. 48–57 and Appendix A | The empirical chapter reads as a sequence of retained runs rather than a cumulative argument |
| R08 | Major | pp. 49, 56, 67–68, 70–71 | The strongest empirical aggregates cannot presently be independently reconstructed from their production inputs |
| R09 | Major | pp. 67–68, §12; proof packets referenced on pp. 61–62 | The submission has no immutable, complete research artifact |
| R10 | Substantial | pp. 65–67, §§11.2–11.4 | The inverse-defect lemma is not the complete floating-point certificate |
| R11 | Substantial | pp. 24, 65–66 | The production treatment of singular stationary systems is not specified |
| R12 | Major | p. 2; pp. 68–69 | The current submission is not covered by the author’s stated final verification |
| R13 | Substantial | pp. 50–51, §§8.2–8.3 | The predictive experiments are not specified well enough to repeat |
| R14 | Substantial | pp. 50–51 | The central statistical relationship is never shown, and the complex predictors lack a simple signal baseline |
| R15 | Substantial | pp. 52–53, 56 | The confidence statements need an explicit estimand and sampling unit |
| R16 | Substantial | pp. 49–57 | Descriptor association, prediction and high-ratio discovery are not evaluated as distinct objectives |
| R17 | Substantial | pp. 54–55, §§8.8–8.9 | The adaptive/IID comparison changes representation-dependent admission as well as the proposal law |
| R18 | Substantial | p. 6, §1.3; pp. 35, 65–67 | The algorithm performance claims lack a relevant benchmark |
| R19 | Substantial | pp. 38–39, 55–57, 71–74 | The local-search descriptions omit the geometry of their step sizes and distances |
| R20 | Substantial | pp. 18–24, especially proof of Theorem 4.1 | The promised derivation of the finite formula ends by importing the formula again |
| R21 | Substantial | pp. 27–28, 42–44, 68 | The relation between six-facet value minimizers and seven-facet local witnesses is insufficiently explained |
| R22 | Substantial | pp. 28, 30, 35, 38–39, 56–57, 61–62, 67–69 | Repeated audit language interrupts the mathematical and experimental narrative |
| R23 | Local | p. 6, §1.2 | The informal proof sketch forgets to exclude symmetry directions |
| R24 | Local | p. 75, Appendix B.1 | The constraint rows are described incorrectly |
| R25 | Local | pp. 18 and 22 | A single author is given plural verbs |
| R26 | Local | pp. 15, 52, 65 | Several local conventions need a more explicit bridge |
| R27 | Substantial | pp. 47, 64 and 80 | Float/page-break handling leaves nearly empty pages |
| R28 | Substantial | pp. 44 and 48; tables pp. 49, 54, 56, 62 | Important tables lack captions, numbers and reliable cross-references |
| R29 | Substantial | pp. 83–85, Appendix C | The proof-facing program is printed in a difficult-to-audit form |
| R30 | Substantial | pp. 30–31, Figure 6 | The flow-graph illustration lacks the data needed to function as a worked example |
| R31 | Local | pp. 62–64, Figure 10 | The visualization section has little mathematical payoff and a low-information figure |
| R32 | Local | pp. 3–4, 85–86; PDF metadata | The bibliography is missing from the contents and PDF outline |
| R33 | Local | pp. 49–57; pp. 73–74 and 81–82 | Precision and figure captions are not consistently matched to the evidential claim |
| R34 | Local | p. 1; p. 2 | The title page does not identify the submission or its examination context |
| R35 | Substantial | p. 1 abstract; p. 70 conclusion | The conclusion treats unresolved empirical transfer as too narrow a choice of remedies |

## R01 — An advertised open problem had already been resolved

**Priority:** Major. **Status:** Confirmed factual error. **Anchor:** p. 5, §1.1; references pp. 85–86.

> The equality of EHZ and cylindrical capacity on general convex domains in four dimensions is still listed as open in [HO26, Remark 1.4(v)].

This is not the current mathematical situation. Abbondandolo, Edtmair and Kang, arXiv:2412.01777 (December 2024), prove the capacity coincidence for convex bodies in R⁴. The introduction of Haim–Kislev–Ostrover v3 itself reports this result in its discussion of different capacities. Thus the thesis both gives an outdated state of the field and attaches a stale pinpoint reference to a 2026 bibliography entry. The next sentence, “Edtmair instead…”, consequently presents the developments in a misleading relationship. The uniformly convex hypothesis of the stronger dynamical theorem must not be confused with the convex-body scope of its capacity corollary. [External sources E1, E2.]

**Repair:** Replace the open-problem claim with the correct coincidence result and distinguish it from the earlier surface-of-section theorem. Update the cited version and pinpoint. Explain the useful consequence: on the four-dimensional convex bodies studied here, the computed EHZ value also determines cylindrical capacity. This correction does not make Gromov width equal to either capacity.


## R02 — The search families are not separated into those that can and cannot beat one

**Priority:** Major. **Status:** Substantive literature/design omission. **Anchor:** pp. 49–57, especially §§8.1, 8.6–8.8.

> We sought values above one.

The ten product groups include seven with a triangle or quadrilateral factor: 3×3, 3×4, 3×5, 3×6, 4×4, 4×5 and 4×6. The triangular case satisfies the Viterbo inequality; Balitskiy–Mitrofanov–Polyanskii’s March 2026 preprint proves it for an arbitrary quadrilateral factor and any other planar convex factor. Consequently these seven groups cannot produce a product with sys>1. The height-equalization pilot on 4×4 and 4×6 also stays inside excluded families. This does not invalidate their use as controls or as subthreshold geometry experiments, but it changes the interpretation of unsuccessful counterexample search. Of the ten product groups, only 5×5, 5×6 and 6×6 are not excluded by these particular results. General orthogonal rotations that destroy the Lagrangian-product structure are a different matter. [E3.]

**Repair:** Add a family-by-family table of known upper bounds and intended role: calibration, descriptor study, or potentially above-one discovery. Reinterpret the old runs in that light and allocate new product-search effort accordingly. The March 2026 result need not have been available when a historical run was designed; the submitted thesis nevertheless needs the updated interpretation.


## R03 — The closest nonsmooth-maximizer literature is missing

**Priority:** Substantial. **Status:** Substantive literature omission. **Anchor:** pp. 6, 39–47, 70.

> Allowing facets to appear or disappear is a different local problem, which the theorem leaves open.

The introduction connects the result to smooth Zoll optimality, but omits Haim–Kislev’s November 2025 *Dynamical extensions of Zoll to nonsmooth convex bodies*. That work discusses the same pentagon product, hyperplane cuts, nonsmooth systoles and explicit families of HKO minimizers; Example 1.12 and Proposition 1.13 are particularly pertinent. This is not an assertion that it proves Theorem 7.1. It is missing context for the theorem’s novelty, the meaning of the fixed-facet restriction and the proposed next step of adding facets. [E4.]

**Repair:** Add a focused comparison explaining what is known for cuts, what your arbitrary ten-row perturbations add, and what remains unproved when facets are created. Keep numerical observations in that paper distinct from its propositions. An indiscriminate expansion of the bibliography is unnecessary.


## R04 — The manuscript does not give its strongest contribution the structural priority it deserves

**Priority:** Major. **Status:** Editorial judgment: thesis-level organization. **Anchor:** pp. 3–7; overall organization.

> This led to several separate results, from a local maximality theorem to algorithms and statistical observations.

There are fourteen top-level sections, two lengthy algorithm routes, a first-order theory chapter, a large empirical chapter, a separate profile proof, visualization, numerics, availability and AI reflection. The main theorem arrives on p. 40; after it, the closely related pentagon profile is separated from it by ten pages of heterogeneous experiments. The introduction supplies a roadmap, so navigation is not wholly absent. The problem is hierarchy: main mathematical results, optional methods and historical project records receive similar structural weight. A reader must continually decide what belongs to the main argument and what is merely retained work. This is a chapter-reorganization problem, not something repaired by adding transition sentences.

**Repair:** Build the main path around finite capacity → feasible upper bounds → local maximality, with the profile and its geometric consequences adjacent. Present the flow-graph route as a genuinely optional second method. Consolidate empirical methods/results into a designed study, and move most software-contract detail, exploratory branch plots and research-process reflection to supporting material. A concrete replacement outline is in the examiner report.


## R05 — The rotated-pentagon proof carries avoidable computational machinery

**Priority:** Major. **Status:** Editorial judgment, supported by reviewer derivation. **Anchor:** pp. 58–62; Appendix B, pp. 75–82; §§4.3, 4.5.

> The proof uses an exhaustive exact computation

The profile can be proved analytically from the already cited HKO endpoint value and the displayed feasible word. For every fixed word and feasible weight vector, feasibility is independent of relative rotation and Q(θ)=A cosθ+B sinθ. Positive trigonometric interpolation between the two HKO angles ±π/10 bounds every candidate simultaneously; the displayed word attains the bound. The complete proof is supplied in the mathematical audit. It requires neither the 3,340-word classification nor root isolation nor singular-KKT case handling. The existing computation can still be a correct discovery or regression-test artifact. The criticism is not that a computer-assisted proof is intrinsically inferior, but that here it obscures a short structural argument and enlarges the trusted implementation unnecessarily.

**Repair:** Use the analytic proof as the theorem proof. Retain a compact account of computational discovery and, only where useful, archive the classifier as supplementary validation. Reassess the space allocated to the twelve-facet block reduction once it is no longer needed for this theorem. Update the abstract and conclusion accordingly.


## R06 — The thesis already contains ingredients for an exact ridge–capacity law across sys=1

**Priority:** Major. **Status:** Missed mathematical synthesis; reviewer supplies derivation. **Anchor:** pp. 51–52, 57, 59–62, 70.

> The current evidence does not supply that relation.

Combining the ridge formula in §8.4 with the pentagon profile gives, for the entire regular-pentagon rotation family, S(Kθ)=16 sin(π/5) cos d(θ) and sys(Kθ) S(Kθ)²=16(3+√5). This is a precise inverse-square relation on a specified family, including values above one. A proof is supplied in the mathematical audit. It does not establish that the empirical correlation transfers to arbitrary high-ratio bodies, nor contradict the cube counterexample to universal monotonicity. But it is exactly the sort of controlled-family mathematical explanation the empirical chapter and conclusion say remains to be found. Leaving §§8.4 and 9 disconnected misses a stronger narrative payoff than another selection pilot.

**Repair:** Add the restricted-family proposition and explicitly distinguish it from a universal law. Use the regular rotation family, followed by controlled deformations of it, as an above-one diagnostic set. Revise the conclusion from “no restricted relation obtained” to a precise account of what the relation explains and what it does not.


## R07 — The empirical chapter reads as a sequence of retained runs rather than a cumulative argument

**Priority:** Major. **Status:** Editorial judgment: chapter-level writing. **Anchor:** pp. 48–57 and Appendix A.

> These are distinct experimental results, with different sources and evaluation contracts.

The chapter moves through correlations, two different predictors, thirty selection rules, concentration, covariance, generic transfer, tangential sampling, paired height changes, orthogonal rotations, adaptive sampling, seven optimizers and several endpoint diagnostics. Most paragraphs are locally understandable, and many caveats are good. Globally the reader must remember changing populations, controls, labels, budgets and statistical units while extracting the scientific message from numerous six-digit results. The most revealing negative diagnostics are separated from the corresponding methods. The final synthesis remains largely a recap rather than an explanation of why the combined evidence changes a mathematical belief or a search decision. The current chapter needs substantial rewriting and selection.

**Repair:** Choose a small number of questions and group studies beneath them. Give each question an estimand, comparison, result and consequence. Put a single study inventory and evidence-status table before the results; retain only numbers needed for the argument in prose. Integrate endpoint failures with the optimizer comparison and demote exploratory runs that do not alter the conclusion. See the proposed chapter outline in the examiner report.


## R08 — The strongest empirical aggregates cannot presently be independently reconstructed from their production inputs

**Priority:** Major. **Status:** Confirmed evidential limitation. **Anchor:** pp. 49, 56, 67–68, 70–71.

> The original producer tree and the individual proposal/evaluation trace tables are not locally available

The thesis itself discloses reused historical capacity evaluations, incompletely certified source information, missing source datasets and feature tables, unavailable optimizer producer trees/traces, and unresolved held-out history. These are not allegations of fabricated results. They mean that the reported correlations, prediction scores and algorithm rankings have weaker evidential standing than a submission-ready reproducible study. Repeatedly calling them “recorded numerical ratios” is honest but does not supply label validation, explain duplicate dependence, or let another researcher recompute the central results. The main exact theorem is in a different evidential category and should not be tarred with this limitation.

**Repair:** Promote a smaller, freshly reproducible dataset and comparison to primary evidence, using the current evaluator and a documented volume/error treatment. Re-evaluate a representative and extreme-value sample of historical labels where inputs survive. Otherwise retain historical aggregates as explicitly secondary discovery records, not the central empirical validation.


## R09 — The submission has no immutable, complete research artifact

**Priority:** Major. **Status:** Confirmed reproducibility defect. **Anchor:** pp. 67–68, §12; proof packets referenced on pp. 61–62.

> This is a living Git repository rather than a frozen archival release

The PDF supplies a repository URL but no thesis-matching commit or release identifier. It also states that twelve external snapshots and essential analysis inputs are absent from a plain checkout. A later reader cannot reliably know which changing code/data version supports this PDF, and not every figure or analysis can be regenerated. The printed HKO witness substantially reduces this problem for Theorem 7.1, whereas the cleaned pentagon excerpts and empirical analyses still depend on external material. The public repository is accessible; the problem is not a broken URL but missing versioned closure of the submission.

**Repair:** Pin a commit and archive the PDF, source, proof packets, relevant logs, data, feature definitions, environment/lockfiles and reproduction commands with a manifest and hashes. Provide explicit locations for intentionally separate large artifacts. A DOI is useful but not mandatory; an immutable, accessible and complete version is the essential requirement. List any intentionally unreproducible claims rather than implying a clone is sufficient.


## R10 — The inverse-defect lemma is not the complete floating-point certificate

**Priority:** Substantial. **Status:** Under-documented correctness claim. **Anchor:** pp. 65–67, §§11.2–11.4.

> The implementation computes the residual and inverse-defect bounds with explicit roundoff and subnormal-error terms.

Lemma 11.1 is correct, but its hypotheses require actual outward enclosures for the exact dyadic K, residual, defect and induced norms. The PDF does not give the roundoff formulas, constants, summation-order assumptions or how errors in assembling K are included. Testing 249 systems cannot substitute for these bounds. The blanket implementation claim at the start of §11 is therefore harder to audit than the short proof suggests. This review did not perform a full Rust arithmetic audit, so it does not assert that the code lacks the stated bounds or that any returned interval is wrong.

**Repair:** Give a compact derivation of the implemented enclosures, including input-matrix rounding, overflow/subnormal handling and the arithmetic contract, and point to exact code locations at the pinned revision. Separate the mathematical inverse lemma, the implementation correspondence argument and empirical tests. Keep the exact fallback and applicability limits explicit.


## R11 — The production treatment of singular stationary systems is not specified

**Priority:** Substantial. **Status:** Under-documented algorithmic obligation. **Anchor:** pp. 24, 65–66.

> with exact fallback for every indeterminate case

Section 4 correctly explains that a singular KKT system is neither automatically infeasible nor a single optimizer: the objective is constant on its affine solution set, but strictly positive weight feasibility must still be decided. Section 11’s exact-fallback description does not say how the general implementation makes this decision, how rank-deficient closure constraints are handled, or how this outcome enters the global interval aggregate. These are necessary cases for the claimed general input contract, not just implementation details of an invertible linear solve.

**Repair:** Document the exact affine-feasibility procedure (for example, a rational feasibility problem with a common positivity slack), its outcomes, and how lower-dimensional supports and zero objective values are treated. Name targeted tests for consistent singular, inconsistent singular, forced-zero and positive-affine-family cases. Do not reject singularity merely because the verified inverse test is unavailable.


## R12 — The current submission is not covered by the author’s stated final verification

**Priority:** Major. **Status:** Submission-readiness/assessment issue, not misconduct finding. **Anchor:** p. 2; pp. 68–69.

> This statement therefore does not assert that I have completed a final review of the current candidate.

The disclosure records a review of the 13 July version and substantial subsequent revision, then expressly declines to attest a completed final review of this candidate. It is not evidence that every current claim is unverified; it is a gap in the submission’s accountability statement. Broad AI involvement is disclosed and reported as advisor-approved, so it should not be treated as concealed assistance or automatic academic misconduct. A master’s assessment still needs the candidate’s demonstrated understanding, responsibility for the final claims and compliance with the applicable examination rules. Advisor approval alone does not tell a reviewer what those rules are.

**Repair:** Complete and document review of the exact submitted version, with a dated version identifier and clear responsibility for its claims. Establish the applicable institutional policy separately, and use the defense to test understanding of the core mathematical and computational arguments. Do not invent individual idea-origin claims that cannot be reconstructed.


## R13 — The predictive experiments are not specified well enough to repeat

**Priority:** Substantial. **Status:** Missing empirical specification. **Anchor:** pp. 50–51, §§8.2–8.3.

> used 39 descriptors: 27 describing face counts and incidences, and 12 summarizing ridge areas

The feature counts do not define the feature matrix. The exact 39 columns, transformations, handling of undefined/degenerate measurements, model hyperparameters, software versions, random seeds, tuning history and train/test group membership are not supplied here. “Capacity-source label and facet count” also needs an operational definition, particularly for product groups and historical cached labels. The second feature-family comparison has another split and feature list. The text correctly warns against directly ranking the two learners; that warning does not make either experiment reproducible.

**Repair:** Provide a feature dictionary and a compact, versioned experiment specification with exact split identifiers, estimator configuration and model-selection history. Link each quoted metric to a particular specification and dataset digest. Include these as essential archived inputs rather than only a narrative feature count.


## R14 — The central statistical relationship is never shown, and the complex predictors lack a simple signal baseline

**Priority:** Substantial. **Status:** Missing diagnostic/baseline. **Anchor:** pp. 50–51.

> The Spearman correlation ... was −0.938.

The large difference between rank and linear correlation makes the shape of the relationship scientifically important. Yet the empirical chapter has no plot of S versus the ratio, no groupwise display, residual diagnostic or view of the high-ratio tail. Numerous pages are allocated to toy mathematical illustrations while the principal data pattern is invisible. A 39-feature forest’s R² is also difficult to interpret without a simple predictor using S alone or a small geometrically motivated baseline. The ridge-only boosting result compares feature families, not the incremental value over a one-dimensional monotone rule.

**Repair:** Add pooled and group-aware views of the actual data, identify controls/outliers and show where HKO and the regular rotation family lie. Evaluate an S-only monotone or comparably simple baseline on the same held-out split as the main model. Show whether added features improve the scientifically relevant tail, not just the pooled score.


## R15 — The confidence statements need an explicit estimand and sampling unit

**Priority:** Substantial. **Status:** Statistical interpretation insufficiently specified. **Anchor:** pp. 52–53, 56.

> These intervals use the twenty seed-by-group differences and a Student-t calculation with nineteen degrees of freedom.

The intervals are described more carefully than is common, but their nominal coverage level is not stated in the relevant prose. For the t interval, the reader needs to know whether the target is an average over ten fixed groups, a hypothetical population of groups, or repeated candidate-pool generation. Twenty seed-by-group effects are not automatically interchangeable observations from one distribution. Shared controls and overlaps also matter for resampling a contrast. The thesis often acknowledges these dependencies, so this is not a finding of proven pseudoreplication or a claim that reusing a seed itself makes disjoint samples dependent.

**Repair:** State the estimand, weighting, confidence level and resampling unit for each interval. Justify the t model or use a design-respecting stratified/paired resampling scheme. With two or three independent pools, distinguish conditional panel uncertainty from between-run uncertainty; report descriptive variation when broader coverage cannot be justified.


## R16 — Descriptor association, prediction and high-ratio discovery are not evaluated as distinct objectives

**Priority:** Substantial. **Status:** Evaluation mismatch. **Anchor:** pp. 49–57.

> The pattern also served a complementary aim: finding bodies with large ratios.

An ordering correlation, pooled R², a mean selected-versus-control difference and a best-so-far search ratio answer different questions. The chapter switches among them without a single explicit account of which result supports geometric explanation and which improves discovery per unit cost. A filter can raise the mean while excluding the rare geometry that crosses one; a model can predict the bulk while failing on that tail. This does not negate the reported gains, but weakens the inference from them to the stated discovery objective.

**Repair:** Declare two outputs: restricted geometric statements and useful search policies. For the latter, report a prespecified tail statistic, valid-candidate/evaluator failure rates, best-so-far versus cost and success on structured above-one controls. Do not turn a lack of a new counterexample into a failure of the whole thesis.


## R17 — The adaptive/IID comparison changes representation-dependent admission as well as the proposal law

**Priority:** Substantial. **Status:** Limited comparability, already partly acknowledged. **Anchor:** pp. 54–55, §§8.8–8.9.

> It is not a symmetry-invariant condition on all pentagon products.

The admission criterion acts on raw IID representations but on normalized adaptive reconstructions. Thus accepted shape distributions are affected by two changes: adaptation and a representation-dependent rejection mechanism. The thesis candidly says so. This is a limitation of what the experiment identifies, rather than a hidden-confound accusation. The three paired wins are valid observations of the implemented comparison, but do not isolate the effect of the cross-entropy update alone.

**Repair:** For a primary adaptation comparison, canonicalize both arms before the same admission test and record acceptance/rejection counts and costs. Alternatively make the narrower system-level comparison the declared target and refrain from attributing the difference solely to adaptation.


## R18 — The algorithm performance claims lack a relevant benchmark

**Priority:** Substantial. **Status:** Missing quantitative support for a contribution claim. **Anchor:** p. 6, §1.3; pp. 35, 65–67.

> Its performance in our tests made it less useful for large searches

The manuscript provides correctness checks and some timings for complete experimental workflows, but not a common-input benchmark supporting the QP/flow-graph performance comparison or quantifying the advertised optimization of the capacity routines. Selected F5–F7 agreement cases establish correspondence on examples, not scalability. The applicability comparison also matters: condition 2 of Definition 5.2 excludes planar Lagrangian polygon products, since any three q-factor rows lie in a two-dimensional subspace. Thus the regular flow-graph theorem does not cover the central product examples, independently of timing. A mathematical reader cannot tell which facet counts are practical, how often certification falls back to exact arithmetic, how exclusions affect coverage, or what the six-facet route actually saves.

**Repair:** Add a compact reproducible benchmark with common input sets, machine/compiler/dependency versions, word counts, runtimes, fallback rates and rejected inputs. Separate algorithmic reductions from engineering speedups. Include candidate-count analysis: the product closure-vertex route has at most cubic many supports per planar factor and at most 120 orders per support pair; bit complexity is a separate issue.


## R19 — The local-search descriptions omit the geometry of their step sizes and distances

**Priority:** Substantial. **Status:** Missing operational definitions. **Anchor:** pp. 38–39, 55–57, 71–74.

> normalized beta allowance 0.3

“Relative step length,” “normalized internal move,” “source distance from HKO,” “distance removed,” the beta allowance and construction of the transverse basis are not fully defined as reproducible quantities. These matter because Euclidean distances in row coordinates are not invariant under the symmetry group, and different quotient slices can make the same nominal radius mean different shape changes. The thesis correctly distinguishes initial and moving slices; the missing normalization details prevent a reader from reproducing or geometrically interpreting the figures and optimization steps.

**Repair:** Define the norm, normalization scale, tangent projection, basis construction, distance-to-HKO convention and any alignment before measuring distance. Give precise beta admission inequalities and trust-region initialization/stopping rules. Put a compact algorithm specification next to the corresponding results.


## R20 — The promised derivation of the finite formula ends by importing the formula again

**Priority:** Substantial. **Status:** Mathematical exposition/organization. **Anchor:** pp. 18–24, especially proof of Theorem 4.1.

> This is Haim–Kislev’s finite capacity formula ... rewritten

After a detailed proof of the simple-minimizer theorem, Theorem 4.1 is justified mainly by translating Haim–Kislev’s theorem, rather than explicitly deriving it from the work just done. Citing a correct theorem is legitimate, so this is not circular invalidity. It weakens the claimed self-contained passage from dynamics to the QP and makes part of the long preceding chapter seem dispensable. The following proposition already contains almost all the needed derivation.

**Repair:** Move the dual-curve construction before the final formula. Any feasible q>0 gives I=1/(2q), hence q≤1/(2c); a simple minimizing orbit supplies equality. This proves positivity and the formula directly, after which the comparison with Haim–Kislev’s conventions can be a remark. The mathematical audit supplies the short argument.


## R21 — The relation between six-facet value minimizers and seven-facet local witnesses is insufficiently explained

**Priority:** Substantial. **Status:** Mathematical exposition. **Anchor:** pp. 27–28, 42–44, 68.

> The five-by-five closure minor is nevertheless invertible

The thesis proves that six facets suffice to compute a product’s capacity, then makes seven-facet singular data central to the local theorem. The statements are compatible, and the worked example is correct. The conceptual distinction deserves an explicit explanation in the mathematics chapter rather than leaving the reader to infer it: a sparse candidate family sufficient for a value at a product need not supply all derivatives needed under arbitrary non-product perturbations; a flat stationary family can provide useful feasible sections. The reason that twenty-six bounds are natural in twenty-five transverse dimensions is likewise more illuminating than the table alone.

**Repair:** Explain this distinction before the worked seven-facet example. Describe what the witness search selected and which limitation of the initial six-facet/nonsingular data motivated the extra sections, without claiming necessity unless it was proved. Explain positive spanning and why a full-dimensional positive dependence needs at least 26 vectors in dimension 25. A pair of labelled planar pentagons with the worked orbit projections would connect the facet words to the geometry more effectively than the later unlabelled three-dimensional skeleton.


## R22 — Repeated audit language interrupts the mathematical and experimental narrative

**Priority:** Substantial. **Status:** Editorial judgment: writing quality. **Anchor:** pp. 28, 30, 35, 38–39, 56–57, 61–62, 67–69.

> These checks are falsification and correspondence evidence on selected examples

The manuscript repeatedly restates what a check does not prove, identifies raw versus rendered representations, and reports retained/producer/witness/source-contract status. Much of this is important somewhere. Its repetition inside the main exposition makes readers repeatedly stop following the mathematical object and instead parse the history or trust status of a software artifact. Section 5.5 contains several near-duplicate limitations within two paragraphs; §5.1’s JSON/rendering discussion precedes the main correctness argument. Dense technical prose is not made readable merely by being cautious.

**Repair:** Centralize evidence classes and reproducibility contracts. At each result give only the local qualification needed to avoid a wrong inference, with a precise cross-reference to the evidence table. Replace artifact-management nouns by the relevant mathematical object or experimental unit whenever possible. The examiner report includes a sample rewrite.


## R23 — The informal proof sketch forgets to exclude symmetry directions

**Priority:** Local. **Status:** Confirmed overbroad wording. **Anchor:** p. 6, §1.2.

> For each nearby direction, at least one of these bounds decreases.

As written this includes translations, dilations and linear symplectic tangent directions. The later certificate explicitly makes every derivative row vanish on the symmetry tangent space. The theorem and detailed proof correctly use transverse directions, so this is an introductory overstatement rather than a defect in the theorem.

**Repair:** Say “in every direction transverse to the symmetries, at least one touching upper bound has strictly negative first derivative,” then mention the local symmetry decomposition.


## R24 — The constraint rows are described incorrectly

**Priority:** Local. **Status:** Confirmed description error. **Anchor:** p. 75, Appendix B.1.

> The last four rows impose ... while the final row imposes ...

The KKT matrix has five constraint rows: four closure rows followed by one normalization row. The last four rows include normalization and omit one closure coordinate. The code itself fills the correct five rows.

**Repair:** Replace with “The first four of the final five rows impose closure; the last row imposes normalization.”


## R25 — A single author is given plural verbs

**Priority:** Local. **Status:** Confirmed grammar error. **Anchor:** pp. 18 and 22.

> Haim–Kislev prove / Haim–Kislev state

[Hai19] has one author, Pazit Haim-Kislev. The surname is not a pair of authors. The plural verbs on these pages are therefore wrong.

**Repair:** Use “Haim–Kislev proves” and “Haim–Kislev states,” or make the paper the grammatical subject.


## R26 — Several local conventions need a more explicit bridge

**Priority:** Local. **Status:** Notation/terminology clarity. **Anchor:** pp. 15, 52, 65.

> A generalized Reeb orbit is simple if ...

The thesis’s “simple” condition means a pure facet word with no repeated direction, rather than merely not being a multiple cover. The explicit definition prevents formal ambiguity, but a warning would avoid importing the usual dynamical meaning. The covariance discussion introduces J=−J0 without explaining that the paired Williamson eigenvalues are unchanged. In §11, “H is twice the quadratic form” confuses a matrix with a scalar-valued form, and the right-hand-side b in Lemma 11.1 should be identified explicitly. None of these observations refutes a formula.

**Repair:** Name the facet-simple convention explicitly; use J0 consistently or explain the harmless sign change; define H by βᵀHβ=2Q and b=(0,d). Group these small convention repairs into one notation pass.


## R27 — Float/page-break handling leaves nearly empty pages

**Priority:** Substantial. **Status:** Confirmed layout defect. **Anchor:** pp. 47, 64 and 80.

Page 47 contains only the short final proof paragraph, p. 64 only the visualization section’s final two lines, and p. 80 only the short paragraph before a full-page plot. These are visually confirmed, not inferred from sparse text extraction. They interrupt reading and inflate an otherwise flowing article-style layout. Ordinary margins and some full-page figures are not themselves defects; these orphaned continuations are.

**Repair:** Rebalance float sizes/placement and remove unnecessary forced page breaks or float barriers. Keep each concluding paragraph with its preceding argument or following figure discussion, then rerun the complete pagination check.


## R28 — Important tables lack captions, numbers and reliable cross-references

**Priority:** Substantial. **Status:** Confirmed navigation/layout defect. **Anchor:** pp. 44 and 48; tables pp. 49, 54, 56, 62.

> The accompanying table gives the exact fixed values for every section

The essential twenty-six-choice witness table appears on p. 48, after the proof has ended, although it is introduced on p. 44 as “accompanying.” It has no displayed table number/caption suitable for a precise cross-reference. Several empirical tables are also unnumbered, while the classification table on p. 62 is labelled Table 2. The reader cannot reliably cite or navigate these important objects by a consistent scheme.

**Repair:** Number and caption every substantive table, with mathematical role or experimental population in the caption. Cross-reference the HKO witness table explicitly and place it with the certificate construction or in its appendix. Move the long exact literals out of the main proof when they distract from its mechanism.


## R29 — The proof-facing program is printed in a difficult-to-audit form

**Priority:** Substantial. **Status:** Confirmed auditability/layout problem. **Anchor:** pp. 83–85, Appendix C.

> The listings below form a complete SageMath program.

The HKO listing uses approximately nine-point monospace, long single-line functions, semicolon-packed statements and huge literal rows. Wrapping occurs inside expressions and strings; extracted quotation marks are typographic apostrophes. A reader can recover the program, but not reliably paste the PDF verbatim into Python. Completeness of the mathematical data is valuable and was enough for the independent reconstruction in this review. It should not be confused with executable usability or clear code exposition.

**Repair:** Provide an exact plain-text attachment or pinned source file, use ordinary code quotes, logical line breaks, descriptive names, one statement per line and short comments identifying each mathematical check. Print the core verifier readably and place bulky assignment data in a separate data listing/file. Do not claim a typography defect invalidates the underlying certificate.


## R30 — The flow-graph illustration lacks the data needed to function as a worked example

**Priority:** Substantial. **Status:** Figure/exposition weakness. **Anchor:** pp. 30–31, Figure 6.

> The figure is regenerated from exact rational tube geometry

The chart conventions are carefully described, but the body’s facet rows, the numerical/exact action and a concrete affine passage or return matrix are not presented with the example. The black cross returning to its location illustrates a fixed point, but does not let the reader connect the drawn polygons to the formulas or independently reconstruct the example. The JSON and coordinate-frame caveats receive more attention than those mathematical data. Independent chart origins are not themselves an error and are correctly disclosed.

**Repair:** Identify the example body and provide at least one explicit passage and the final fixed-point/action data, in the caption or a compact worked example. Move rendering-contract detail to the reproduction notes.


## R31 — The visualization section has little mathematical payoff and a low-information figure

**Priority:** Local. **Status:** Figure effectiveness/layout judgment. **Anchor:** pp. 62–64, Figure 10.

> We found no visual pattern that gave a reliable hypothesis

The HKO skeleton is very faint, the cube panel is much smaller than the neighboring orbit panel, and neither panel labels the facets/segments needed to connect it to earlier orbit data. The text correctly warns about clipping and metric distortion, so it should not be accused of claiming a symplectic invariant. As a stand-alone top-level section spread over three pages, however, a qualitative view plus a negative anecdote contributes little relative to its space.

**Repair:** Either make it a labelled explanatory example tied to a word and dwell times, or reduce it to a short figure/sidebar or appendix note. Increase contrast and balance the panels. Remove the two-line continuation page.


## R32 — The bibliography is missing from the contents and PDF outline

**Priority:** Local. **Status:** Confirmed document navigation defect. **Anchor:** pp. 3–4, 85–86; PDF metadata.

“References” appears at the end but is absent from the contents and from the PDF bookmarks. The PDF title/author metadata fields are also empty. The latter is an accessibility/discoverability improvement rather than a mathematical grading issue.

**Repair:** Add the bibliography to the contents and outline and populate document metadata. Check the final internal links after repagination. Do not infer a requirement for a list of figures/tables unless the institutional template calls for one.


## R33 — Precision and figure captions are not consistently matched to the evidential claim

**Priority:** Local. **Status:** Presentation precision. **Anchor:** pp. 49–57; pp. 73–74 and 81–82.

> 0.984783

Six-decimal historical means/medians sit beside substantial between-start variation and incomplete evaluator provenance. Such digits are useful in archived records but dominate the readable comparisons. The captions of Figures 11–12 identify qualitative diagnostics without defining the plotted distance normalization, and the branch-landscape figure relies on a remote explanation of admissibility and grouping. These are mostly presentation defects, not evidence that the underlying rounded values are wrong.

**Repair:** Use a few meaningful digits in the main discussion, preserve full values in machine-readable tables, and define statistical units/normalizations locally in captions. Retain the existing distinction between across-start ranges and confidence intervals.


## R34 — The title page does not identify the submission or its examination context

**Priority:** Local. **Status:** Conditional formal-completeness concern. **Anchor:** p. 1; p. 2.

The cover gives the title, author and year, but no institution, degree/program, supervisor/examiner or exact submission date. Whether these and a particular declaration are mandatory depends on the institution’s regulations, which were not supplied. This is therefore a submission-package check, not a finding of rule violation.

**Repair:** Compare the final front matter with the actual required template, add the required identifiers/declarations, and ensure the version/date is consistent with the final-review statement and archived artifact.


## R35 — The conclusion treats unresolved empirical transfer as too narrow a choice of remedies

**Priority:** Substantial. **Status:** Conclusion/claim-framing weakness. **Anchor:** p. 1 abstract; p. 70 conclusion.

> whether further progress requires different statistical methods or new geometrically motivated measurements

The abstract’s final alternatives overlook sampling-family choice, known exclusions, missing above-one controlled tests, label quality, optimization budget and the mathematical relation derivable from the thesis’s own formulas. The conclusion repeats the experimental limitations without deciding which obstacle the evidence actually identifies. This is not a logically exclusive “either/or” assertion, but it is a poor prioritization of next steps and weakens the narrative closure.

**Repair:** End with a ranked account of what is established, what failed under which source laws, and which next experiment or theorem would distinguish plausible explanations. Incorporate the exact regular-pentagon ridge law and the relevant cutting literature. Keep “new counterexample not found” separate from “no useful mathematics found.”


# Verification questions and optional hardening — not established theorem errors

## Q01 — The singular-classifier safety claim is broader than the visible control flow

**Anchor:** pp. 78–79, classify_sigma; same logic in the inspected repository source

The nonzero-Q, nonzero-gap path checks positivity of the particular solution returned by solve_right. If this solution is feasible on at least one sign cell, the final success path does not first establish that the KKT kernel is trivial. For a genuinely singular system with varying beta, another affine representative could be feasible on other cells. Constancy of Q on the stationary affine space does not by itself make those positivity domains equal. Consequently the source does not establish its advertised general “every unresolved singular case” guarantee merely from the shown branching logic. However, an independent numerical reconstruction of all 3,340 words at an interior angle found 470 consistent singular systems, all with numerically zero stationary Q; this matches the forced-zero count and gives no evidence of an actually misclassified positive-Q word. I therefore do **not** count this as a demonstrated failure of Theorem 9.1. Add an explicit nonsingularity check on that path, or an exact assertion/log showing that it is never entered with beta-changing kernel, or certify the gap on all potentially feasible affine families. The analytic proof in the mathematical audit removes the theorem’s dependence on this classifier altogether.

## Q02 — Assertion-state hardening of the printed HKO verifier

**Anchor:** pp. 83–85, Appendix C

The printed program places its checks in Python assert statements but does not contain the explicit __debug__ guard described for the pentagon executable. Python optimization can remove assertions while leaving the final success print. The documented ordinary sage -python invocation normally keeps assertions enabled, so this is not a failure of the stated normal run. Add a guard or explicit checked exceptions and record assertion state in the log as defense against misleading accidental execution.

## Q03 — Pinpoint references need version reconciliation, not speculative renumbering

**Anchor:** p. 12, Theorem 2.6 citation; bibliography

The consulted arXiv version of Haim–Kislev’s polytope paper labels the dual correspondence Lemma 2.2, whereas the thesis cites Lemma 2.1. I did not establish the numbering in the final typeset article, so I do not mark this as a confirmed miscitation. Conversely, Rudolf’s final journal article really does call the relevant billiard theorem Theorem 1; an arXiv numbering difference is not an error in the thesis. Reconcile all pinpoint references against the exact cited editions during the bibliography pass.


---

# Mathematical audit and replacement arguments

This file distinguishes the thesis’s arguments from calculations and proofs supplied during this review. Locations refer to the supplied 86-page PDF. External sources E1–E6 are identified in `04_sources_and_scope.md`.

## 1. Main result: the exact HKO witness survives independent reconstruction

**I found no mathematical refutation of Theorem 7.1.** More positively, I independently reconstructed the printed 26-section certificate using exact arithmetic in SymPy’s algebraic number field

\[
\mathbb Q(t),\qquad t^4-10t^2+5=0,\quad t=\sqrt{5-2\sqrt5}\in(0,1).
\]

This did not execute the thesis’s Sage program. The assignment data were recovered from Appendix C, and the derivative calculation was independently organized through the stationary-value identity rather than copied from the printed feasible-section differentiation code.

### What was checked exactly

For every one of the 26 assignments, the selected five-column closure minor is invertible, the reconstructed weights satisfy closure and normalization, every weight is strictly positive, and the quadratic value equals the known HKO value. The base weights also satisfy the equality-constrained stationarity equations. Using independently constructed symplectic Lie algebra generators, the resulting derivative matrix annihilates all fifteen symmetry columns. Its rank is 25; the symmetry-column rank is 15; and its one-dimensional left kernel contains a vector with all 26 entries strictly positive. Normalizing that vector gives an exact relation with sum one.

The important distinction is that **the ranks, signs and identities were checked exactly**. The following decimals are only readable diagnostics:

| Diagnostic | Reconstructed value |
|---|---:|
| Smallest weight among the 26 assignments | approximately 0.01790666645 |
| Smallest normalized positive-relation coefficient | approximately 0.002186171686 |
| Largest normalized positive-relation coefficient | approximately 0.08029271605 |
| Worked derivative in §7.3, entry 2, facet 2, momentum coordinate 1 | approximately −0.18017073246472 |

The last number agrees with the printed −0.1801707325. The full normalized coefficient vector and verification output are in `verification/`.

### Why the independent derivative route is valid

At the base point, write

\[
H_\sigma\beta=C_\sigma^T\lambda,
\qquad C_\sigma\beta=e.
\]

For any differentiable feasible section through that point, differentiating feasibility gives

\[
C_\sigma D\beta=-(DC_\sigma)\beta.
\]

Consequently

\[
Dq[h]=D_aQ[h]-\langle\lambda,(D_aC[h])\beta\rangle.
\]

This identity uses stationarity **at the base point** and feasibility of the chosen section. It does not require an invertible KKT matrix or a smoothly continuing optimizer. It is therefore valid for the singular seven-facet sections. With the independently reconstructed volume derivative, differentiation of

\[
U=\frac{1}{2V}\left(\frac{1}{2q}\right)^2
\]

gives the required 26 rows.

### What arithmetic alone does not prove

These computations verify the finite content of Lemma 7.4, conditional on the known HKO capacity and the identification of the displayed rows with the specified body. They do not replace the geometric argument. I also checked the following logical links in the text:

* A sufficiently close ten-facet body can be labelled by ten polar vertices close to the original ones. The vertex-count argument is essential; it would not cover arbitrary facet creation.
* Simplicity and strict slack give the stable local face structure and smooth volume used by the proof.
* A positive feasible weight section gives an upper bound, even when it is not a nearby optimizing branch.
* Rank 25, annihilation of the 15-dimensional symmetry space and a strictly positive relation imply a negative lower-envelope slope in every nonzero transverse direction.
* Compactness of the transverse unit sphere and a common first-order remainder estimate give a genuine neighborhood, not merely separately chosen radii along rays.
* The inverse function theorem for the group action and a complementary slice transports the strict slice statement to nearby bodies and yields the equality characterization.

These links are adequate as written. The absence of an explicit numerical neighborhood radius is **not** a gap in an existential local-maximality theorem. Nor is a Hessian test needed: the proof uses a nonsmooth, linearly decreasing envelope of upper bounds.

There remains ordinary reliance on the exact-arithmetic implementation used for this review, as there is on Sage in the thesis. This is independent corroboration, not a formally verified proof assistant development or an exhaustive audit of the whole software repository.

## 2. An analytic replacement for the rotated-pentagon certificate

**Reviewer-supplied proof.** This replaces the enumeration-dependent part of Theorem 9.1; it does not assume that theorem’s conclusion. It uses the independently known HKO capacity, the Haim–Kislev formula and the explicit feasible word already displayed on pp. 59–60.

Set

\[
a=\frac{\pi}{10},\qquad b=\frac{\pi}{5},\qquad
D=(1+\cos b)^2,
\qquad K_\theta=P_5\times_L R(\theta)P_5.
\]

We prove

\[
c_{\rm EHZ}(K_\theta)=D\sec\theta
\quad\text{for }-a\leq\theta\leq a.
\]

The symmetries already established in the thesis then give the complete profile.

### Step 1: the feasible set does not depend on the relative angle

Write the factor rows as \((u_i,0)\) and \((0,R(\theta)v_j)\). For a fixed word, closure is equivalent to

\[
\sum_i\beta_i^q u_i=0,
\qquad
R(\theta)\sum_j\beta_j^p v_j=0.
\]

Since the rotation is invertible, these constraints, together with nonnegativity and total weight one, are independent of \(\theta\). We may therefore compare **the same feasible weight vector and word** at different angles. This is the key point that makes the following interpolation legitimate.

### Step 2: each candidate is a first harmonic

Same-factor symplectic pairings vanish. Each mixed pairing is a scalar product involving \(R(\theta)\), hence is linear in \(\cos\theta\) and \(\sin\theta\). Thus every fixed feasible candidate has

\[
Q_{\sigma,\beta}(\theta)=A_{\sigma,\beta}\cos\theta
+B_{\sigma,\beta}\sin\theta.
\]

There is no stationarity or nondegeneracy assumption here.

### Step 3: both endpoint maxima are already known

The angle \(-\pi/2\) of the HKO body is congruent to \(-a\) modulo the pentagon’s \(2\pi/5\) rotational symmetry. Simultaneous reflection of both factors identifies the capacities at \(-a\) and \(+a\). Therefore the independently known HKO capacity gives

\[
Q_{\max}(-a)=Q_{\max}(a)=q_*:=\frac1{2c_{\rm HKO}}.
\]

Every feasible candidate is consequently at most \(q_*\) at both endpoints.

### Step 4: positive interpolation bounds every candidate at once

For \(-a\leq\theta\leq a\), the elementary identity

\[
Q_{\sigma,\beta}(\theta)
=
\frac{\sin(a-\theta)}{\sin(2a)}Q_{\sigma,\beta}(-a)
+
\frac{\sin(a+\theta)}{\sin(2a)}Q_{\sigma,\beta}(a)
\]

has nonnegative coefficients. Their sum is \(\cos\theta/\cos a\). Hence

\[
Q_{\sigma,\beta}(\theta)
\leq q_*\frac{\cos\theta}{\cos a}.
\]

Taking the maximum over all words and all feasible weights preserves this bound. Using the HKO value printed on p. 40,

\[
c_{\rm HKO}=2\cos a(1+\cos b),
\]

and \(2\cos^2a=1+\cos b\), we obtain

\[
c_{\rm HKO}\cos a=D,
\qquad
Q_{\max}(\theta)\leq\frac{\cos\theta}{2D}.
\]

### Step 5: the thesis’s displayed word attains the bound

For the constant positive weights associated with \(\sigma_{\rm act}=(3,8,1,0,5,6)\), the calculation on p. 59 gives

\[
Q_{\rm act}(\theta)=\frac{\cos\theta}{2D}.
\]

This candidate is feasible throughout the interval. Thus it supplies the reverse inequality for \(Q_{\max}\), and

\[
Q_{\max}(\theta)=\frac{\cos\theta}{2D},
\qquad
c_{\rm EHZ}(K_\theta)=D\sec\theta.
\]

Since the volume is constant, the thesis’s elementary area calculation yields

\[
\operatorname{sys}(K_\theta)
=\frac{5+2\sqrt5}{10}\sec^2\theta
\quad(-a\leq\theta\leq a).
\]

The previously established reflection, factor-exchange and pentagon symmetries yield the formula with \(d(\theta)\) for every real \(\theta\). This proof includes the endpoints directly. There are no exceptional interior parameters to recover by continuity, and no singular stationary systems to classify. ∎

### What this changes in the thesis

This is not merely a faster implementation of the same certificate. It changes the mathematical explanation: the angle-independent feasible set and its harmonic objectives control all competitors simultaneously. The long computer-assisted proof can become a short structural argument, with the computational work retained as discovery history or a test of the algorithms.

A related optional observation follows from Theorem 4.6. For any two fixed planar polygons, the closure-vertex weights are independent of relative rotation, and there are finitely many support/order candidates. Thus \(Q_{\max}(\theta)\) is the support function of a finite coefficient polygon evaluated at \((\cos\theta,\sin\theta)\). It is a finite upper envelope of first harmonics. This could provide a clearer mathematical explanation of rotation profiles than a collection of rational KKT branches. Developing this general observation further is an optional enrichment, not a requirement for passing a master’s thesis.

## 3. The missing bridge between the empirical descriptor and the exact profile

**Reviewer-supplied proposition.** For the regular-pentagon family of §9, with \(S\) defined in §8.2,

\[
\boxed{S(K_\theta)=16\sin(\pi/5)\cos d(\theta)}
\]

and therefore

\[
\boxed{\operatorname{sys}(K_\theta)\,S(K_\theta)^2
=16(3+\sqrt5).}
\]

This is an exact restricted-family relation on both sides of \(\operatorname{sys}=1\). It is not a universal law for polytopes.

### Proof

Let \(b=\pi/5\). The edge length of a circumradius-one regular pentagon is \(\ell=2\sin b\), and its area is

\[
B=5\sin b\cos b.
\]

The edge directions are equally spaced by \(2\pi/5\). Among the 25 pairs of edges from the two factors, each difference of edge-direction indices occurs five times. The exact product-ridge formula on p. 51 therefore gives

\[
S(K_\theta)
=\frac{5\ell^2}{B}\sum_{k=0}^4
\left|\cos\left(\theta+\frac{2\pi k}{5}\right)\right|
=4\tan b\sum_{k=0}^4
\left|\cos\left(\theta+\frac{2\pi k}{5}\right)\right|.
\]

On \(-\pi/10\leq\theta\leq\pi/10\), the signs of the five cosine terms are \((+,+,-,-,+)\), allowing zeros at endpoints. Pairing the symmetric terms gives

\[
\sum_{k=0}^4\left|\cos\left(\theta+\frac{2\pi k}{5}\right)\right|
=
\left(1+2\cos\frac{2\pi}{5}-2\cos\frac{4\pi}{5}\right)\cos\theta
=(1+\sqrt5)\cos\theta.
\]

Since \(1+\sqrt5=4\cos b\), this is precisely

\[
S(K_\theta)=16\sin b\cos\theta.
\]

The absolute-cosine sum is even and has period \(\pi/5\), so the formula extends with \(\theta\) replaced by \(d(\theta)\). Multiplying its square by the proven systolic profile and using

\[
\sin^2(\pi/5)=\frac{5-\sqrt5}{8}
\]

gives

\[
256\frac{5-\sqrt5}{8}\frac{5+2\sqrt5}{10}
=16(3+\sqrt5).
\]

This proves both identities. ∎

At HKO, the formula gives \(S=4\sqrt5\), agreeing with the reported 8.944272. At aligned pentagons it gives approximately 9.404564. The systolic ratio exceeds one exactly when

\[
d(\theta)>\arccos\sqrt{\frac{5+2\sqrt5}{10}}
\approx13.2825256^\circ,
\]

within a half-period ending at \(18^\circ\). Thus an above-one structured test set is readily available; it is not necessary to wait for random sampling to discover a previously unknown counterexample before examining the descriptor there.

The scientific conclusion must remain narrow: this proves an inverse-square law for one rotation family. It neither explains the pooled rank correlation for the original random distribution nor rescues universal monotonicity, which the thesis correctly disproves using the cube.

## 4. The finite capacity formula can be derived directly from the preceding chapters

This is the bounded repair for R20, not a correction to the value of the formula.

Let \(c=c_{\rm EHZ}(K)\). For a feasible word/weight pair with \(q=Q_\sigma(\beta)>0\), construct the pure-velocity closed loop with period \(T=1/(2q)\), as in Proposition 4.2. The closure equation, shoelace identity and pure-velocity support values give

\[
\mathcal A(z)=I_K(z)=T.
\]

It is dual feasible, so \(c\leq T\), or \(q\leq1/(2c)\). This establishes an upper bound for every positive candidate Q-value.

Conversely, Theorem 3.4 provides a simple minimizing generalized Reeb orbit of period \(c\). Normalize its dwell times by \(c\). The resulting feasible weights have

\[
Q_\sigma(\beta)=\frac1{2c}>0.
\]

The global maximum is therefore positive and equal to \(1/(2c)\). This proves Theorem 4.1 from the dual principle and simple-minimizer theorem already developed, instead of relying on a second import of the whole finite formula.

## 5. Other mathematical checks and their limits

| Portion of thesis | Review outcome |
|---|---|
| §2.1–2.3: symplectic signs, primitive, action and contact normalization | The convention is internally consistent. In particular, \(R_i=2J_0a_i\) has \(\lambda_0(R_i)=1\) on its facet. No sign/factor-of-two error identified. |
| Theorem 2.6 and reconstruction | The conjugate \(H_K^*=h_K^2/4\), multiplier identity and simultaneous space/time rescaling are consistent. Existence and the nonsmooth multiplier theorem remain imported results, as acknowledged. |
| Lemmas 3.1–3.9 and Theorem 3.4 | The action formula, base-point recovery and dual splitting/merging/rescaling argument are coherent. The proof is an existence result for a simple minimizer, not a classification of all minimizing orbits. |
| §4.2: singular stationarity systems | The proof that Q is constant on an affine stationary solution set is correct. Positive feasibility must still be decided; this distinction matters for implementation. |
| Theorem 4.4 | The cited final Rudolf theorem covers nonsmooth convex factors and at most three strong-billiard bounce points in the planar case. The thesis’s reversal of convention and recovery after dual surgery are appropriate. No missing strict-convexity hypothesis was found. |
| Theorem 4.6 | The bilinear argument and vertex support bound are sound. This is one of the clearest mathematical pieces in the thesis. It does not claim every minimizer has at most six facets. |
| Theorem 5.4 and Corollary 5.5 | Completeness follows on the expressly stated regular class from a simple minimizer and tube semantics. It is not a theorem for arbitrary nongeneric input or the full CH Type 1/2/3 model. |
| Proposition 5.6 | Independently computed exact return-map determinants \(-10643/600\) for ABCDE and \(1/168\) for ACDE, and checked several AB-repeat specializations. The polynomial nonvanishing argument does not require those repeated-row specializations to be irredundant geometric inputs. |
| §6 and volume calculations in §7 | The row-volume derivative, feasible-section bounds, envelope derivative and conditional finite-minimum derivative are consistent. The finite coverage assumption in §6.3 is important and is explicitly stated. |
| Theorem 7.1 | Geometric proof reviewed; finite witness independently reconstructed exactly as above. No counterexample or fatal proof gap identified. |
| §8.4: ridge formula and cube example | The mixed ridge area, edge-width identity and cube comparison are correct. The missed issue is their integration with §9, not a wrong formula. |
| Theorem 9.1 | Analytic proof supplied above. The full Sage classification was not rerun. The generic singular-classifier concern is isolated as Q01, not asserted to invalidate the theorem. |
| Lemma 11.1 | The inverse-defect estimate is correct with its stated outward bounds. Turning it into certified machine arithmetic requires the additional implementation details identified in R10. |
| §11.3 curvature pruning | A positive tangent-curvature witness extends by zeros to an order-preserving superword and excludes an interior fixed-word maximum. I found no flaw in this mathematical pruning argument. I did not audit every corresponding Rust implementation path. |

### Classifier inspection: why the uncertainty is specifically bounded

The inspected `classify_sigma` implementation contains the control-flow issue described in Q01. It should not be promoted into a finding that the thesis’s 410 positive-gap classifications are wrong. An independent numerical enumeration reproduced the 50,400 raw representatives and 3,340 distinct pruned words at the interior angle \(\pi/20\). Among them, 470 systems were consistent and numerically singular; all had numerically zero stationary quadratic value. Twenty-five systems were inconsistent at that angle. This is consistent with the reported special outcomes and suggests that the problematic positive-Q singular success path may never be used here.

That diagnostic is not an exact generic-rank/identity proof. A rigorous guard or exact run-level assertion is the right repair if the classifier is retained. The analytic argument above avoids relying on the question at all.

## 6. Criticisms deliberately not made

I have **not** treated any of the following as mistakes: using exact rational choices that happen to resemble rounded decimals; using singular KKT data to construct smooth feasible sections; proving local maximality only with exactly ten facets; using 26 upper bounds rather than classifying all minimizers; leaving the neighborhood radius existential; using finite computation in a mathematical proof; failing to discover a new above-one body by random search; distinguishing dyadic input from unrounded source geometry; or reporting negative experiments with appropriately limited conclusions.

I also have not inferred plagiarism, fabricated data, prohibited AI use, or lack of personal understanding from the disclosure. Those would require evidence not available here. The review’s concerns about final verification and examination responsibility are narrower and explicit.


---

# Sources, review scope and evidence boundaries

## Submitted document

*Probing Viterbo’s Conjecture*, Jörn Stöhler, 2026. The supplied PDF has 86 pages; its printed numbering agrees with PDF page numbering.

Original file SHA-256:

```text
d7dae9a78dffc87fe89f3005bed9b6b51d28728fa90e1a5c811bc09b236e1695
```

All 86 pages were read. All pages were rendered and visually inspected in contact sheets, with individual-page inspection of the key mathematical figures, tables and code listings. Findings about nearly empty pages, table placement and code readability refer to the actual rendered PDF, not merely to text extraction.

Page, section, theorem and equation anchors are used throughout the standalone files so that they remain useful outside the chat. The PDF annotations use the same R01–R35 and Q01–Q03 identifiers. A statement labelled a reviewer derivation is not being attributed to the original thesis.

## External primary sources used for substantive corrections

**E1. Alberto Abbondandolo, Oliver Edtmair and Jungsoo Kang.** *On closed characteristics of minimal action on a convex three-sphere*. arXiv:2412.01777v1, submitted 2 December 2024. See the abstract and **Corollary 1**, in the introduction’s “Consequences concerning symplectic capacities.” The stronger dynamical theorem concerns uniformly convex domains; the stated capacity coincidence extends to every convex body in R⁴. This is the direct source for R01.

**E2. Pazit Haim-Kislev and Yaron Ostrover.** *A counterexample to Viterbo’s conjecture*. *Annals of Mathematics* 203(2) (2026), 603–622, DOI: 10.4007/annals.2026.203.2.5. The final-publication metadata was checked on the journal site; the readable mathematical text consulted was **arXiv:2405.16513v3**. Proposition 1.4 supplies the HKO value; §1.1(iii) reports EHZ/cylindrical coincidence and cites E1. The thesis’s p. 5 pinpoint is not reconciled to that version. The analytic replacement proof uses the HKO value, not the thesis’s rotation theorem.

**E3. Alexey Balitskiy, Ivan Mitrofanov and Alexander Polyanskii.** *Triangle covering problems and the Viterbo inequality in the plane*. **arXiv:2603.12495v1**, 12 March 2026. Theorem 1.2 covers arbitrary quadrilateral factors and affinely regular hexagonal factors; §4 treats triangular factors. R02’s identification of seven excluded side-count groups is a deduction from those results and the thesis’s own sample-group list. This is a preprint; it is not described here as a refereed 2026 journal article.

**E4. Pazit Haim-Kislev.** *Dynamical extensions of Zoll to nonsmooth convex bodies*. **arXiv:2511.16644v1**, 20 November 2025. The introduction, Example 1.12 and Proposition 1.13 give the directly relevant nonsmooth/cutting context. R03 does not claim this paper already proves the thesis’s local-maximality theorem. Statements reported there as numerical evidence are not upgraded to theorems in this review.

**E5. Daniel Rudolf.** *The Minkowski Billiard Characterization of the EHZ-Capacity of Convex Lagrangian Products*. *Journal of Dynamics and Differential Equations* 36 (2024), 2773–2791, DOI: 10.1007/s10884-022-10228-0. **Definition 2 and Theorem 1 in the final journal text** were checked. The theorem supports the nonsmooth strong-billiard input used in §4.3. Different arXiv numbering is not a reason to call the thesis’s Theorem 1 citation incorrect.

**E6. Foundational variational sources.** Pazit Haim-Kislev, *On the symplectic size of convex polytopes*, DOI: 10.1007/s00039-019-00486-4, with arXiv:1712.03494 consulted; Shiri Artstein-Avidan and Yaron Ostrover, *Bounds for Minkowski billiard trajectories in convex bodies*, DOI: 10.1093/imrn/rns216, with arXiv:1111.2353 consulted. These were used to check the scope and normalization of the imported finite and nonsmooth variational results. The possible lemma-number discrepancy is left as Q03 because final-edition numbering was not established for that pinpoint.

## Repository inspection

The GitHub connector successfully accessed the repository named in the thesis, `JoernStoehler/msc-math`. The inspected material included the pentagon proof directory and the branch classifier source, the HKO packet inventory, and repository HEAD metadata.

Observed HEAD:

```text
1bec30637070effe7ab8b0a022ae62e83f6bbcab
```

Inspected pentagon classifier blob:

```text
01bc62dc0691574efaf86149bcc22cabe4593aa2
```

Path:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

These identifiers pin what was inspected during the review. They are **not** claimed to be the revision that generated the supplied PDF. That correspondence is one of the missing submission details. The repository is accessible; R09 concerns immutable identification and complete dependencies, not an invented availability failure.

## What was executed, and what was not

Executed checks are included in `verification/`: an independent exact reconstruction of the HKO witness, rational checks of the flow-genericity determinants, and a numerical enumeration/rank diagnostic for the 3,340 pentagon words. The two analytic arguments in the mathematical audit were derived during the review and checked algebraically; they are presented with their proofs.

The thesis’s full Sage pentagon classifier was **not** rerun. The complete Rust implementation, all arithmetic/compiler paths, all historical data, the full experimental pipeline and every figure producer were **not** independently reproduced. In particular, the historical correlations, prediction metrics and optimizer summaries were checked as reported claims, not re-estimated from unavailable source data. No claim of exhaustive plagiarism checking, publication-priority determination or compliance with a particular university’s AI rules is made.

This distinction explains the language of the findings: an observed textual error is called an error; missing evidence is called missing evidence; an unexecuted or unresolved implementation obligation is not called a proved computational failure. Editorial judgments identify their concrete reading cost and a repair, rather than pretending to be mathematical counterexamples.
