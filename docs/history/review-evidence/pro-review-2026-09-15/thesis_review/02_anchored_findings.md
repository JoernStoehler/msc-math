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
