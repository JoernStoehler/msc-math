# Assessment of the current thesis PDF

Date: 14 September 2026. Status: **reviewed diagnosis with Jörn's corrections incorporated. He broadly agrees with the main diagnosis, recognizes the style examples as defects without endorsing completeness, identifies the HKO omissions as an incomplete write-up, and judges the AI sign-repair example too trivial to sustain the reflection chapter. Other detailed findings remain attributed to the assessor. This is not a PASS judgment.**

## Purpose and assessed object

This report diagnoses the current manuscript as input to producing a replacement that Jörn judges PASS. It does not prescribe a writing workflow, authorize a rewrite, or assume that workflow research will be necessary. A straightforward writing attempt might succeed. Its purpose here is to identify what such an attempt actually needs to accomplish, including problems that remain after the first obvious defect is fixed.

Assessed artifact: [`thesis/build/main.pdf`](../thesis/build/main.pdf), *Probing Viterbo's Conjecture*, 110 pages. The PDF creation timestamp is 14 September 2026, 12:54:14 Europe/Berlin. Its SHA-256 is:

```text
5f799da24818861cd0aeb54964a65b4fb3df7e87a27c4fc31d8ef80fbfa57ead
```

The checkout HEAD at assessment start was `add4f51bd7024bd7f3826a87445a69d31a9bbd32`. Page numbers below are printed PDF pages, which agree with physical page numbers. A later rebuilt PDF may have different pagination or content; the hash identifies this assessment's object. The manuscript was not edited or rebuilt during the assessment.

Jörn describes the manuscript as currently failing. This report investigates that claim rather than treating it as evidence for any particular defect. The assessor's own recommendation is **revise before presenting this as a finished PASS candidate**, principally because of costly and indirect prose, missing reader-facing mathematical and experimental content, uneven explanation, and unresolved evidence-to-claim connections. This is not an assertion that every mathematician would fail it, or that the central theorems are false.

## Reader and standard

Jörn clarified during this assessment that he and Kai agreed on an imaginary reader who is a mathematically mature master's student, has read standard smooth symplectic geometry lecture notes, and knows basic computational material. Hamiltonians and Reeb vector fields may be assumed; substantial knowledge of Reeb orbits may not. Clarke duality was not covered. Specialized computation and combinatorial optimization must not assume more than Kai knows. This is a materially more specific standard than either “general mathematics student” or “specialist in computational symplectic geometry.”

The report therefore does not demand a beginner's account of all smooth symplectic geometry. It does demand that the thesis teach its nonsmooth duality and optimization machinery, and make its new reasoning accessible from that background.

Jörn also explicitly added **writing style as an independent PASS criterion**. Human readers encounter sentences and paragraphs substantially in sequence; resolving a phrase after reading a whole chapter, or revisiting earlier prose, costs attention. Agents may understand conventional agent phrases with little effort and therefore underestimate that cost. Jörn predicts that the current text would give him a headache after about two pages and reduce his reading speed to perhaps 20% of normal. This is his expectation, not a measured reading test. It makes “an agent could reconstruct the meaning” an inadequate acceptance standard.

The proposed review dimensions are:

| Dimension | What a PASS-level manuscript needs to establish |
| --- | --- |
| Topic and substance | Worthwhile questions and enough mathematical or computational achievement for a master's thesis. A negative experiment can contribute; a new global conjecture is not required. |
| Attribution and contribution | Readers can distinguish imported results, adaptations, and the thesis's contributions, without guessing from whether a theorem has a citation. |
| Mathematical correctness | Statements have appropriate hypotheses; arguments establish their conclusions; conventions and computational reductions agree. |
| Computational support | The finite check actually supports the stated theorem or observation, with the necessary connection to the mathematical objects. |
| Comprehensibility | Sentences and notation have recoverable meanings; unfamiliar machinery is introduced; readers can follow why the arguments work. |
| Writing style and reading effort | Prose communicates directly, with few empty phrases or unnecessary decoding steps. Local references and prerequisite ideas are available when needed. Correctness and eventual recoverability do not excuse a punishing sequential reading experience. |
| Narrative and selection | Questions, methods and answers are connected, and the amount and placement of detail serve that connection. Several related investigations are explicitly permitted. |
| Empirical reporting | Sampling, measurements, features, comparisons and validation are sufficiently specified to understand what was learned. |
| Scholarly practice | Relevant claims are sourced, limitations are accurate, and the promised computational material can be identified and accessed. |
| Visual and document quality | Figures explain something, tables are readable, navigation works, and the document looks finished. |
| Scope and authorship | The agreed content, separate AI disclosure and research reflection, and Jörn's responsibility for the final candidate are handled appropriately. |

These are assessment dimensions, not an additive grading formula. Jörn supplied the reader clarification and the independent writing-style requirement, and reviewed the main diagnosis and selected substantive findings as recorded below. Other detailed findings remain the assessor's judgments. The exact PASS cutoff remains Jörn's judgment.

## Main diagnosis

**The research has credible thesis substance; that does not make the current prose worth salvaging. The assessed PDF does not establish that the topic is failing or that a new main research result is needed.** The ten-facet HKO local-maximality theorem, the rotated-pentagon profile, the finite algorithms and reductions, and the search investigations form a plausible master's-thesis portfolio. The manuscript allows those findings to be identified, but is not thereby a useful prose base for their replacement presentation. Repository context also records Jörn's confirmation that the HKO and pentagon results are established theorems; that is background evidence about the research, not certification of every sentence in this PDF.

**The problem is substantially more than proofreading.** Some sentences need rewriting, but other passages omit the objects, methods or derivations that those sentences are supposed to explain. Smoother language cannot supply an unnamed statistical feature, identify an unspecified sampling distribution, or connect an undisplayed HKO formula to the geometry.

**Missing mathematical content means the write-up is incomplete.** Jörn rejects treating the HKO chapter's deferral to repository files as an acceptable alternative presentation awaiting reviewer preference. Content needed to substantiate the PDF's proof must actually appear in the PDF; its availability elsewhere does not complete the manuscript. Whether the omissions came from interrupted work or an inadequate standard of completeness is not established by this assessment.

**The prose presents a serious independent obstacle, even where its meaning can be reconstructed.** It repeatedly uses indirect descriptions, compressed technical noun phrases and delayed explanations that impose avoidable work on a sequential reader. This assessment initially described the writing as uneven and recoverable; Jörn identified that recoverability misses the human reading-cost criterion. The simple-minimizer operations, closure-versus-placement counterexample, six-facet bilinear reduction and pentagon competitor identify mathematical ideas that can be explained in the replacement. They are not recommendations to preserve their current wording. No useful return from searching this manuscript for reusable phrasing has been demonstrated; Jörn judges that effort essentially worthless. No human reading-speed measurement was made.

**The TOC is a plausible retained structure; the prose does not realize that structure well.** Jörn reports that he and Kai discussed the TOC and found that it fits a good narrative. That is a positive reason to use it as the starting structure for a replacement. The criticism here concerns the manuscript's execution: repeated proof architecture, extensive space for a side result's implementation and exploratory classifications, and too little concrete content for parts of the central result and empirical investigation. It does not establish a need to redesign the agreed chapter structure.

**Content-level work is needed in at least two senses:** recovering and explaining existing mathematical/experimental material, and resolving the strength of selected empirical claims. **New research is not yet shown necessary.** Whether some missing support requires new computation or proof work remains unresolved; this report identifies those points instead of pretending that prose repair alone will close them.

**Some existing material has too little value to merit its current treatment.** Jörn judges the AI chapter's sign-repair example to be a routine, shallow illustration, worth perhaps two lines rather than a chapter. Its problem is not merely narrow coverage or missing caveats. The replacement needs to select content for what it teaches about mathematical research; enlarging a collection of trivial examples would not supply that value.

### What is worth carrying into the replacement

On reviewing the main diagnosis, Jörn said it sounded about right but did not clearly distinguish valuable research from prose that adds problems. His proposed starting point is to discard the current PDF as a writing base, potentially keep the TOC because of the discussion with Kai, and write the thesis afresh from the research findings. This report adopts that distinction. It is input to replacement work, not authorization here to delete files or launch a rewrite.

| Material | Value for a replacement |
| --- | --- |
| Research questions, established results, mathematical constructions and proof ideas | These are the substantive base. Recover their precise statements and support from the research, source papers, derivations and certificates. Their value does not depend on retaining their current sentences. |
| TOC / broad chapter structure | A reasonable starting structure, supported by Jörn's report that he and Kai discussed it and found it narratively suitable. Defects in the present chapter contents do not by themselves argue against that structure. |
| Data, experiment definitions, code and computational witnesses | Potential substantive support, subject to the claim-specific limitations in this report. “Use the research” does not mean trusting every repository summary or historical result without checking its meaning. |
| Current manuscript wording | No demonstrated salvage value that justifies an effort to extract useful phrases. Do not turn favorable comments about a mathematical example into an assignment to mine or polish its prose. |
| Existing figures and tables | Separate assets to consider if they communicate the intended content accurately. A legible figure need not be redrawn merely because the prose is replaced; no blanket reuse recommendation follows from the visual survey. |
| Current PDF as a record | Useful for locating the assessed claims and understanding the diagnosed failures. Retaining that record is compatible with abandoning it as the base for writing. |

The report's proposed repair outcomes specify what the replacement must communicate. They should not be interpreted as instructions to patch the current paragraphs one by one. The case for writing afresh rests on Jörn's assessment of the prose and the diagnosed burdens; no controlled cost comparison of rewriting versus salvage was performed.

## Findings

### F1. The central HKO mathematical write-up is incomplete

**Evidence:** Section 7, pp. 59–66, especially pp. 63–65 and Table 1. The chapter explains that 26 feasible upper functions have derivative rank 25 and a positive convex relation. It says that hand-derived geometry, capacity, volume and volume-derivative formulas establish the base point, and that the surrounding proof supplies the geometric identifications. But it does not actually display the concrete HKO dual-row list, specialized volume derivative, a selected feasible section, or a worked derivative row. The ordered field is specified by a quartic and an isolating interval without identifying its generator as `tan(pi/5)` in this chapter.

The general volume derivative on p. 52 and general feasible-section construction on p. 54 help, but they do not themselves exhibit the specialized formulas that the verifier evaluates. Table 1 tells the reader which obligations exist; it is not the missing calculation.

**Why this matters:** The reader can understand the abstract implication “these rows would prove local maximality” while still being unable to inspect how the actual HKO body yields those rows. The PDF therefore has an incomplete write-up of its central proof. Existing research or code outside the PDF may supply material for completing it, but does not make the present write-up complete. This finding does not establish that the underlying theorem or witness is wrong.

**Recovery evidence:** The theorem packet exists. `experiments/hko-local-maximum/theorem/verify.sage.py` explicitly constructs the ten dual rows and the specialized volume data. Its retained summary reports 26 rows, rank 25, symmetry rank 15 and a positive kernel relation. I inspected those source portions and the summary, but did not independently rerun or certify the entire verifier. This makes recovery from existing work a plausible repair; it is not a reason to relaunch the research.

**Required replacement outcome:** The PDF contains the mathematical content needed to establish the stated result: the specialized base geometry and formulas, the construction of the feasible sections and their derivatives, the mathematical specification of the finite checks, and their connection to the theorem. A representative calculation can help explain the construction; it is not a substitute for completing the argument. Repository pointers and assurances that a hand proof supplies the correspondence do not satisfy this requirement. The author must determine what concrete finite data or specification the argument requires and include an adequate account, rather than presuming that missing content can be deferred.

**Assessment and Jörn's review:** This is an incompleteness finding. The assessor initially presented the division between PDF and repository as a reviewer-preference question. Jörn rejected that framing: something absent from the PDF is absent from the PDF. His response resolves the proposed choice in favor of completing the write-up. It does not independently establish every detailed mathematical claim in this assessment or the historical cause of the omissions.

### F2. Specialized optimization language is assumed before it is taught

**Evidence:** “KKT” first appears on p. 35 without expansion. The manuscript discusses positive KKT solutions, support faces, singular KKT systems and curvature before giving the derivative equation on p. 54 and the explicit block stationary system on p. 94. Section 4 repeatedly explains why stationarity does not imply a global maximum, but does not first derive the elementary constrained stationarity system from the displayed quadratic objective and affine constraints.

Other examples include the abrupt `LDL^T` reference on p. 42 and the transition into exact real-arithmetic algorithms and rational-function classification without a similarly explicit introduction to what operations the reader is being asked to accept.

**Why this matters:** The agreed reader can likely understand this material once explained. Its absence needlessly turns familiarity with optimization vocabulary into a prerequisite. Repeating cautions about a system does not teach the system.

**Minimum repair outcome:** Introduce the relevant Karush–Kuhn–Tucker/Lagrange multiplier equations where first used, explain the role of support restrictions and the tangent Hessian, and carry one small fixed-word example far enough to connect geometry, constraints, stationary solution and value comparison. The name of every standard method need not become a textbook chapter.

**Assessment:** High-confidence gap relative to the reader Jörn specified. It requires explanation and some mathematical content, not just expanding an acronym.

### F3. Background effort is not consistently allocated to the reader's actual difficulties

**Evidence:** Pages 10–16 devote substantial space to smooth action/contact normalization, symmetry invariance, elementary continuity and product coordinates. Section 2.6 then develops Clarke duality through several normalizations, an Euler–Lagrange argument and reconstruction on pp. 16–21. The main duality theorem appears only after the normalization comparisons and auxiliary lemmas.

The introduction to Section 2.6 is useful: it explains that position-free curves allow velocity rearrangement. But the reader must retain several scaling conventions before seeing the full operative theorem. The subsequent simple-minimizer proof is comparatively well organized into explicit operations.

**Why this matters:** There is a mismatch between what can be assumed and what needs teaching. Conventions must still be fixed, even when familiar. But the thesis should spend its teaching effort on the new dual problem, why its constraint is chosen, and how its minimizers return to the boundary.

**Minimum repair outcome:** Present the operative duality result and its use early, then distinguish the proof idea from normalization bookkeeping. Preserve necessary sign and scaling calculations, with longer source-convention comparisons placed where they do not obstruct first understanding.

**Assessment:** Clear structural opportunity; the amount of compression is a judgment call. This finding does not establish that the Clarke argument is mathematically wrong.

### F4. The empirical chapter often omits the experiment while reporting its status

**Evidence:** Section 8.1, p. 67, gives sample counts, seed 42, height interval and rejection sampling, but does not define the distribution of normal directions or the complete acceptance rule. These are part of what “random polytope” means here. Section 8.2, p. 68, names combinatorial and normalized symplectic-area descriptors without giving the actual feature definitions needed for the later conclusions.

Appendix A does not close the main gaps:

- Page 107 reports a strongest absolute Spearman correlation of about 0.938 without identifying the feature or even the direction of the association.
- Pages 107–108 report random-forest `R² ≈ 0.885` versus `≈ 0.009` for metadata, without specifying there whether these are training, test or cross-validation scores, the split, feature preprocessing, or uncertainty.
- Pages 69 and 108 discuss a frozen scalar rule set without stating the rules or their thresholds. They report no values above one and a maximum around 0.868, but do not supply the selected-versus-baseline comparison needed to evaluate the rule's usefulness below that threshold.
- Page 109 says HKO is not far outside the invariant-feature cloud without defining the distance, visualization or diagnostic that makes “not far” interpretable.

**Why this matters:** These are not just missing reproduction commands. The reader lacks enough information to understand the scientific observations. Even perfectly repeatable files would not make an unnamed correlation into a clear mathematical or empirical finding.

**Minimum repair outcome:** Define the sampling model and important descriptors; state the principal selection rules; report the relevant validation design and comparative outcomes. Select a few consequential observations and explain them completely rather than retaining many method names with little information. If a reported result cannot be recovered, identify it as unavailable or make an explicit scope decision.

**Assessment:** High-confidence substantive reporting defect. Existing experiment packets may contain much of the missing content; this assessment did not recover every packet or determine how many results need rerunning.

### F5. The empirical measurements and the mathematical quantity are not consistently connected

**Evidence:** Pages 69–73 explicitly distinguish historical evaluator outputs from certified capacities, including the warning that a common evaluator need not have equal error on the points visited by different methods. This is an important and correct limitation. But the retained random/product table is still repeatedly described as having no systolic ratio above one, including pp. 7, 67–68; only later does the appendix state that its input audit does not validate the capacity or volume calculations. The current certified solver described on pp. 93–96 is a different object from the historical evaluators.

The repository's `docs/capacity-calculation-map.md` confirms that retained `sys` values do not all share one calculation contract, and that capacity and volume provenance are separate. A current certified capacity implementation cannot retroactively certify old derived ratios.

**Why this matters:** The broad empirical question concerns geometric systolic ratios. An observation about an algorithm's stored objective values is a different result unless its measurement reliability is supported. Some negative searches may remain useful with ordinary numerical validation rather than formal certificates, but that argument must actually be made.

**Minimum repair outcome:** Identify the evaluator and volume calculation for each important dataset, give evidence of fitness for the retained claim, and use consistent notation/wording. If only recorded values are supported, state that at the prominent claim sites as well as in limitations. Do not assume that labeling all computations “heuristic” either destroys all their value or repairs every interpretive problem.

**Assessment:** High-confidence need for claim reconciliation; the actual numerical reliability of all 14,336 rows is **not assessed** here. This is one place where further scientific checking, rather than only writing, may be needed.

### F6. The optimizer comparison is more informative than the surrounding diagnostics, but its conclusion remains narrow

**Evidence:** Pages 70–73 and 102–104 give matched starts, a compute-allocation rule, seven implementations, numerical outcomes and controls. They explicitly disclose unestablished held-out independence, varying realized compute, a historical objective, and nonconverged endpoints. Those disclosures are substantive strengths.

Nevertheless, “four-anchor branch history” and related methods are only compactly described, with the outer-step table postponed to p. 103. The bootstrap interval is reported without its resampling details. The comparison does not establish a ranking of the mathematical systolic ratio or of optimally tuned algorithm families. The gap between the same nominal cutoff and unequal realized work matters to the interpretation of a terminal ranking.

**Why this matters:** A useful experiment survives here: implementations behave differently on a fixed recorded objective under an explicit allocation rule. The chapter should explain that finding and its mechanism clearly, rather than imply a stronger method comparison or bury it under repeated disclaimers.

**Minimum repair outcome:** Make the methods and budget intelligible at the point of comparison, specify the statistical comparison sufficiently, and explain the substantive observed lesson. Preserve the endpoint counterexamples and model-failure controls. A new comprehensive nonsmooth-optimizer contest is not implied: Jörn has already recorded that such an extension is not worth delaying the thesis for.

**Assessment:** Reporting and interpretation work is needed; new benchmarking is not established as a prerequisite to PASS.

### F7. Repeated evidence-status prose crowds out the objects being studied

**Evidence:** The following are characteristic passages, not an exhaustive stylistic count:

| Passage | Reader problem |
| --- | --- |
| p. 7: “A stronger collection of these conditions holds on an open dense subset of every fixed real presentation chamber.” | A technically loaded summary arrives before a reader knows what a presentation chamber contributes to the problem. |
| p. 45: discussion of JSON plotting coordinates and raw construction-chart fields | Implementation serialization interrupts the mathematical explanation of the tube figure. |
| p. 64: “The hand-derived field, HKO geometry, capacity, volume, and volume-derivative formulas set the base point and the normalization used for sys.” | It announces the role of formulas instead of showing the missing formulas. |
| p. 69: “a manifest-frozen comparison,” “clean run provenance,” “retained lineage” | Repository-management vocabulary accumulates before the experiment is fully specified. |
| p. 107: strongest absolute correlation ≈ 0.938 | The sentence is easy to parse but scientifically incomplete because its subject feature is unnamed. |
| pp. 98–99: repeated statements that one replay does not establish productivity, model ranking or causal effects | Appropriate limitations occupy much of a short chapter whose positive research-process lesson is narrow. |

The phrase families “retained,” “packet,” “proof-facing,” “contract,” and “boundary” are often meaningful in repository coordination. In publication prose they frequently leave the reader asking what actual data, equation or decision is meant.

**Why this matters:** The text can be grammatically clear while failing to transfer useful understanding. A line editor who merely shortens sentences will miss this problem. Conversely, automatically deleting every qualification would remove important scientific distinctions.

**Minimum repair outcome:** Each paragraph should primarily teach an object, argument, method or observation. Necessary qualifications should be attached to the exact claim they limit, and repeated only where an independent reader-facing use warrants repetition. Replace internal artifact vocabulary with its concrete meaning when the artifact mechanics are not the subject.

**Assessment:** High-confidence pervasive writing pattern. Its severity varies by section; this is not a claim that every sentence is bad.

#### F7a. Reading in order exposes burdens that a chapter-level summary conceals

The following close reading records what a reader has to resolve *at that point*, rather than crediting the sentence because a later passage or repository file permits interpretation.

| Reading location | What arrives before the reader can comfortably use it | Consequence |
| --- | --- | --- |
| p. 6, principal results, continuing onto p. 7 | “labelled polar vertices,” a “25-dimensional slice,” “15 infinitesimal symmetry directions,” “strictly positive feasible choices,” “positive quadratic value,” “upper functions,” “derivative rows,” and a “cotangent space” in one short proof summary | Several are familiar geometric terms separately, but their roles in this problem are not yet grounded. The reader must hold an entire technical construction before having a simple explanation of why upper bounds prove a maximum. |
| p. 7, flow-graph result paragraph | “nondegeneracy conditions on transitions, short row lists, and return maps,” followed by “fixed real presentation chamber,” “rational-input corollary,” “caller contract,” and “Type 1/Type 2 analysis” | These qualifications belong to different levels—mathematical hypotheses, implementation assumptions, comparison with a paper—and arrive before the construction. The reader has to classify them mentally while still trying to learn what the result is. |
| p. 62, opening of Section 7.2 | The first sentences say what the certificate's “role” is and which “statement” it proves; then come “admissible action,” “serialized witness” and “feasible-section chart” | The abstract purpose has already been explained. The new difficulty is the concrete construction, but its introduction is displaced by another layer of descriptions about descriptions. |
| p. 69, method outcomes | “model-based diagnostics,” “in-table predictability,” “provenance-only baselines,” “ridge symplectic-area descriptors” and “candidate-proposer claim” | A reader can decode the intended distinction between fitting known data and predicting new examples. But doing so still does not identify the feature, model evaluation or selection rule. Some delayed meanings never arrive in the PDF. |
| pp. 69–70, transition to optimizer comparison | “manifest-frozen,” “clean run provenance,” “retained lineage,” a later “development” manifest, and “selected-body screen” | The reader repeatedly changes from experiment to recordkeeping to a different experiment before the seven methods are introduced. Much of this detail could be expressed once after the reader knows which comparison it qualifies. |

The problem is not that all technical vocabulary is forbidden. A master's thesis must introduce difficult concepts, and an introduction may preview later results. The defect is **too many unresolved roles at once, without enough immediate payoff or concrete meaning**. Forward references to a proof are often fine; a forward dependency needed to understand the current sentence is much more costly.

Several distinct style failures recur:

1. **Low-information framing.** “The role of the exact certificate is to prove the strict transverse-slice statement left by the chart and quotient reduction” (p. 62) mostly repeats the preceding account. It consumes attention before the actual construction begins.
2. **An indirect label in place of a direct statement.** “The active search surface has two row families” (p. 67) requires translation into “we sampled random polytopes and random polygon products.” The former also makes “active” and “surface” sound like information the reader ought to understand.
3. **Compressed noun phrases.** “A manifest-frozen comparison ... on a random ten-facet start block labelled held out” (p. 67) folds the sample, record, intended validation role and uncertainty into one description. Shorter sentences must actually unpack these relationships, not just split the same phrase across two lines.
4. **Vague achievements.** “recover substantial structure” (p. 7) and “ridge ... descriptors were especially visible” (p. 69) require the reader to infer what measurable observation is being asserted. A correlation, comparison or pattern should be named.
5. **Repeated negative alternatives.** Repeatedly saying what an experiment does not prove makes the reader process claims that were never its plausible result. Some cautions are necessary; others should be consolidated after the positive finding is clear.
6. **An unstable level of discussion.** Mathematical objects, code implementations, provenance records and the thesis's editorial choices share paragraphs without sufficiently clear transitions. The reader must infer which kind of statement each sentence makes.
7. **Explanations supplied too late.** Deferring the fixed-word stationarity system until after repeated KKT discussion is both a content-teaching problem and a sequential reading problem. A later definition does not refund the effort spent reading earlier pages.

These mechanisms can combine within two pages. Their total cost is not captured by counting grammar errors or undefined terms. They also occur in introductory and explanatory prose, where readers should be receiving help rather than additional decoding work.

**Concrete style acceptance outcome:** On an ordinary first reading, Jörn can follow the local argument without repeatedly translating internal phrases, searching ahead for the subject of a claim, or rereading to find what a paragraph contributed. Ordinary mathematical thinking is expected; unnecessary linguistic reconstruction is not. This is separate from whether the theorem is correct, the chapter is logically complete, or an agent can summarize it accurately.

**Limit:** The close reading supports a serious style diagnosis. It does not measure the predicted fivefold slowdown, establish an exact frequency of defects, or prove that a particular rewrite would fix the experience. A short human calibration can discriminate those possibilities if later workflow work needs it, but this report does not require Jörn to perform that test now.

**Jörn's review and disposition:** Jörn agrees that F7a lists defects, but is not convinced that it lists everything wrong with the writing. He does not see a clear benefit in further analyzing GPT-5.6 Sol's writing and proposes switching prose-writing tasks to **GPT-6 Astra**. Accordingly, this assessment does not pursue a more exhaustive style taxonomy. Astra is the selected direction for future prose-writing tasks, not a model whose PASS-level performance has been established here. This report does not claim that Sol authored every assessed passage or that a controlled model comparison was performed.

### F8. The manuscript repeats its architecture without proportionately developing its main result

**Evidence:** The HKO mechanism appears in the abstract, pp. 6–7, pp. 57–58, p. 60, pp. 62–65 and the conclusion. Some repetition is appropriate, but multiple accounts restate the same dimension/rank/positive-relation structure. Meanwhile Section 7 occupies about eight pages, while the pentagon section spans pp. 75–91, including a roughly six-page code exposition and an empirical branch landscape inside the theorem's proof.

Pages 80–82 interrupt the proof's finite classification with a sampled landscape, deduplication rules and numerical status tables; the exact classification resumes on p. 83 and the continuity conclusion comes on p. 84. This is a concrete interruption, not merely an objection to having a long proof.

The conclusion also gives the pentagon result substantial prominence, while the optimizer improvement and its diagnostic lessons receive little comparable synthesis. The recorded scope identifies pentagons as a side result and the search story as a major strand.

**Minimum repair outcome:** Give the central proof concrete mathematical development. Keep a continuous theorem argument for the pentagon profile; put exploratory classifications and substantial implementation details where they can be consulted without interrupting that argument. Make the conclusion's allocation reflect the intended contribution hierarchy. There is no justified numerical page target, and a single tightly unified narrative is not required.

**Assessment:** High-confidence imbalance and interruption; exact relocation and length are editorial decisions for the replacement.

### F9. The attribution of the contribution portfolio is not sufficiently explicit

**Evidence:** Haim–Kislev's main formula and simple-minimizer theorem are clearly attributed. The flow-graph chapter carefully distinguishes its theorem from CH2021. However, the introduction's contribution inventory is less clear about which finite reductions and adaptations are new thesis contributions. The six-facet result receives a theorem and algorithm on pp. 40–41, but its originality and relationship to the rest of the contribution list are not made comparably explicit. The extended presentation of a cited theorem can appear similar in status to a new one.

The bibliography contains nine entries, mainly the mathematical foundations. The data-science chapter names a wide range of methods with little methodological sourcing; the optimizer discussion supplies a general Numerical Optimization citation but does not give a specific reference for every specialized implementation it invokes.

**Why this matters:** The reader should not have to reconstruct novelty from citation absence, or confuse a substantial expository proof with an original result. This affects assessment of substance independently of correctness and style.

**Minimum repair outcome:** State which results are imported, adapted or new, and what the important adaptation achieves. Supply targeted method references and precise definitions where needed. A much larger literature review is not automatically required.

**Assessment:** Contribution presentation needs work. A comprehensive novelty search was not performed, so this report does not certify originality or diagnose plagiarism.

### F10. A few local mathematical and algorithmic interfaces need explicit resolution

These are distinct from a demonstrated counterexample to a central theorem.

1. **Empty tubes and the rejection branch, pp. 44, 47–48.** Definition 5.2 requires a nonsingular return map for surviving words with a *nonempty* closed tube. Algorithm 5.3 does not explicitly discard an empty tube before solving and rejecting a singular fixed-point equation. Its proof says that no long word reaches rejection. Page 44 says empty intermediate intersections exclude extensions, which plausibly supplies the intended rule, but the displayed algorithm and its proof should say so. The regularity hypothesis alone does not make the return map nonsingular for empty domains. This is a control-flow/proof wording gap; I have not constructed a polytope counterexample to the literal algorithm.

2. **Singular stationary families in the pentagon classification, pp. 78–90.** The code handles particular solutions, kernels and forced-zero coordinates. For this reader the argument should explicitly explain why the quadratic objective is constant on an affine family of solutions of one equality-constrained stationary system, and how exceptional parameter ranks are excluded then recovered by continuity. The constant-value fact is elementary: for two stationary feasible solutions differing by `z`, feasibility gives `Cz=0`, subtracting stationarity gives `Hz` in the image of `C^T`, and hence both the linear variation and `z^T H z` vanish. This makes a potentially opaque implementation shortcut mathematically understandable. I found no counterexample to that step.

3. **HKO chart stability, pp. 61–62.** The local-stability argument is plausible and has the correct simple-polytope setting, but its list of “open conditions” compresses the claim that no nonface four-tuple can create a new vertex, including singular tuples. A boundedness/compactness argument would make the persistence of infeasibility explicit. This is a request for an auditable step, not a finding that simple-polytope stability is false.

4. **“Admissible action,” pp. 62–63.** Earlier sections carefully distinguish a feasible dual curve from a boundary orbit. Calling the feasible-section value an “admissible action” without specifying the dual problem risks undoing that distinction. The upper bound follows from the QP even without nearby primal realization; the terminology should preserve that explanation.

5. **Certified numerical route, pp. 93–96.** Lemma 11.1 proves an inverse-defect estimate. It does not by itself prove every claimed outward-rounding bound or every singular exact-fallback case of the whole solver. The chapter summarizes those mechanisms and finite audits. The full implementation correspondence and all fallback cases remain unverified by this assessment. The appropriate response is a targeted check of what the retained numerical claims actually rely on, not automatic formal verification of the software stack.

**Assessment:** Local clarification and selected mathematical checking are warranted. These findings do not support saying “the thesis is full of false theorems.” Their differing evidential status must be preserved.

### F11. The AI reflection devotes a chapter to an example of insufficient value

**Evidence:** Pages 97–99 focus on a replay of four regressions for one known sign defect. The concrete observation is intelligible: one regression passes even after the bad sign is restored. The chapter carefully avoids causal/model/productivity claims that its small, reconstructed record cannot support.

The accepted scope asks for reflection on how mathematicians can use AI in mathematical research, drawing on this project. The current chapter mostly establishes one lesson about mutation-sensitive software tests. It says little about mathematical question selection, proof creation and checking, failed prose, or human judgment, despite the broad AI involvement disclosed on p. 2.

**Jörn's value judgment:** Presented with the current case, Jörn did not regard it as valuable chapter content. He described fixing sign problems as a tiny standard example of AI capability, requiring reading and repeated work without stamina problems rather than deep reasoning, and suggested roughly two lines of discussion. This is a judgment about the example's significance for this thesis. The existence of a replay packet and a carefully qualified observation does not make it an adequate research reflection.

**Required replacement outcome:** Select material with substantive implications for AI use in mathematical research. If the sign example is mentioned, give it proportionate, very brief treatment; do not build the chapter around it or pad it with more routine examples. Recover potentially worthwhile insights from the actual research and assess their significance before writing them up. This assessment does not yet identify and establish the replacement chapter's substantive contents. It does not assign new productivity studies or experiments.

**Correction to the original assessment:** Calling this merely a narrow but useful testing lesson understated the defect. Jörn's response identifies inadequate value relative to the space and role assigned to it. More breadth alone is not the repair criterion.

### F12. Availability is honestly incomplete, but honesty does not complete the promised research artifact

**Evidence:** Pages 96–97 explicitly say that a plain checkout lacks some source datasets and the method-facing invariant table, that no frozen release or DOI is claimed, and that exact reproduction of every run is not available. Pages 74–75 identify a dirty producer state without the patch or hashes needed to reconstruct it. Page 2 states that the current candidate has not received Jörn's completed final review.

These are preferable to false completion claims. They are nevertheless unfinished aspects of the intended outcome where the accepted scope requires durable computational support. The PDF gives repository paths but no specific immutable submission revision or simple overall reproduction route.

**Minimum repair outcome:** Make the retained results and their necessary supporting artifacts reliably identifiable and accessible at the promised level. Resolve unreconstructable evidence by a claim-specific scientific/scope decision. Do not invent a DOI, promise byte-identical timing, or rewrite the personal-review declaration before the review happens.

**Assessment:** Clear availability limitations; their exact status against the final submission requirements is not determined here. No archive publication or administrative action was performed. Current university rules and deadlines were not investigated because the task is manuscript diagnosis, and no current administrative defect is inferred from historical notes.

### F13. Visual quality is mostly serviceable, with specific presentation defects

**Evidence:** All 110 pages were inspected as rendered overview sheets. Pages 12, 33, 46, 64, 72, 81 and 92 were inspected individually at higher resolution. This is a broad layout survey with selected detailed checks, not a full-resolution typography audit of every page.

- Page 9 contains only the final two lines of the introduction above an otherwise nearly empty page. Page 45 similarly leaves substantial white space before the tube figure on p. 46. These are visible pagination problems, not missing mathematical pages.
- The table of contents spills into the introduction page. This is awkward rather than independently disqualifying.
- Figure 6, p. 46, has useful geometry, but very small insets and cramped arrow labels. Its caption spends considerable space on coordinate/export distinctions. Readers need the geometric interpretation more than the serialization explanation.
- Figure 7, p. 72, is legible and compares the seven methods. Its common 0–1 scale makes the small differences among the top three hard to inspect visually; the numerical table partly compensates. An inset or different presentation could help if that close ranking is central, but is not intrinsically required.
- Figure 9, p. 81, shows the competitor landscape successfully, but most of the visually dominant curves are irrelevant to the lower envelope. The figure's position inside the proof is a larger issue than its rendering.
- Figure 10, p. 92, gives an attractive exploratory HKO orbit picture, but its low-contrast cube panel and lack of geometric labels limit what a reader can infer. The thesis's first actual HKO orbit picture also arrives very late.

No gross clipping, missing-figure boxes or widespread typesetting corruption was found in this survey. Some nonsensical glyphs in extracted figure text were extraction artifacts, not visible PDF defects. In particular, a suspected normalization-bracket problem on p. 33 disappeared upon checking the rendered page and TeX; it is **not** a finding.

**Minimum repair outcome:** Repair obvious pagination waste and make consequential figures interpretable and legible. More decoration or a wholesale visual redesign is not shown necessary.

### F14. The conclusion repeats results more than it synthesizes the investigation

**Evidence:** Pages 99–102 restate theorem scope and certificate architecture at length. They do identify meaningful open questions: changing facet counts, singular first-order theory, and other product families. But the conclusion makes less of the concrete optimizer progress/failure lessons, the limited reach of the feature search, and the reason these findings together change the next mathematical question.

**Why this matters:** A reader reaching the end needs a clear view of what the project taught, not another detailed proof synopsis. This affects the perceived substance even when the results are already adequate.

**Minimum repair outcome:** Give a short answer to each retained research question, explain the most consequential learning, and distinguish unresolved questions from tasks that merely remained unfinished. Preserve the multiple-investigation structure rather than inventing a single stronger theorem.

**Assessment:** Expository and selection work; no new result is required by this finding.

## Chapter coverage and what remains uncertain

| PDF part | Positive evidence | Principal issues / assessment boundary |
| --- | --- | --- |
| Abstract and introduction, pp. 1, 5–9 | Real questions and explicit main results; appropriately limited local theorem | Dense technical summary before definitions; repetitive architecture; contribution hierarchy and measurement wording need reconciliation |
| Disclosure, p. 2 | Material AI involvement and incomplete final review stated plainly | Personal and advisor assertions not independently verified; final review still needs to occur |
| Preliminaries, pp. 10–21 | Action convention and reconstruction are developed; duality has an explicit purpose | Allocate detail to unfamiliar duality; improve theorem-first orientation; full foundational proof audit not completed |
| Generalized orbits, pp. 22–31 | Closure/placement counterexample and five-operation simple-minimizer proof are useful | Some repetition; terminology “simple” must retain its local definition; no fatal defect demonstrated in the read argument |
| QP and product reductions, pp. 32–42 | Concrete objective, dual-curve reconstruction, especially clear bilinear six-facet mechanism | Missing initial KKT teaching; attribution of adaptations; two product enumerations need a clear reader-facing reason for coexistence |
| Flow graph, pp. 42–51 | Explicit geometry, algorithm, assumptions, completeness argument and genericity result | Empty-domain control-flow ambiguity; terse genericity witness and substantial vocabulary; full genericity calculation not independently recomputed |
| First-order variation, pp. 51–58 | Feasible upper functions correctly separated from optimizing branches; finite negative-slope criterion explained | Several notions of branch/window need reader orientation; complete nongeneric derivative theory explicitly absent, not secretly established |
| HKO, pp. 59–66 | Strong theorem statement; recognizable slice and upper-function proof | Mathematical write-up incomplete: concrete certificate-to-geometry content is missing from the PDF; no independent full certificate rerun or correspondence certification |
| Search, pp. 67–75 | Several datasets distinguished; optimizer controls expose real failure modes | Incomplete methods/features; measurement support and validation gaps; full search-scope sufficiency unresolved |
| Pentagon, pp. 75–91 | Explicit formula, active word, hand competitor, finite outcomes and continuity argument | Proof interrupted by empirical material; code-heavy side result; full 3340-word run not repeated |
| Visualization, pp. 91–92 | Real illustrative views and modest outcome claim | Limited explanatory labeling; late placement of core geometric example |
| Numerics, pp. 93–96 | Input policy and inverse-defect lemma; counts and limitations of audits | Broad solver claims need their actual supporting derivations/implementation connection; not certified by this reading |
| Availability, pp. 96–97 | Missing artifacts and mutable repository acknowledged | Incomplete durable/reproduction route at the claimed project target |
| AI reflection, pp. 97–99 | The replay supports a specific test-behavior observation | Jörn judges the routine example worth perhaps two lines, not chapter treatment; substantive replacement content remains to be recovered and evaluated |
| Conclusion, pp. 99–102 | Main open mathematical questions visible | Repetition and underdeveloped synthesis of empirical learning |
| Appendix and references, pp. 102–110 | Optimizer step table and numerical details add useful substance | Appendix does not supply several missing methods; limited method sourcing; not a complete bibliography/novelty audit |

This coverage prevents a single repair from being mistaken for completion. Fixing F1 leaves empirical reporting, optimization teaching and narrative issues. Fixing F7's language leaves missing content and unresolved numerical reliability. Completing an archive does not repair the mathematical exposition.

## Mathematical checking performed and its limits

The assessment read the complete extracted manuscript, followed the main logical structure, and inspected selected TeX and supporting sources where a proposed criticism needed verification. It checked the normalization around Theorem 4.1 against the rendered PDF, active TeX and cached Haim–Kislev formula. The apparent bracket error in extracted text was rejected. It inspected the HKO verifier's concrete geometry and volume formulas and retained summary. It inspected the pentagon classifier's handling of singular solutions and considered why stationary objective values are constant.

Two imported inputs were checked against external primary sources. Artstein–Avidan–Ostrover's [paper](https://arxiv.org/pdf/1111.2353) contains the dual attainment, nonsmooth least-action characterization and minimizer criticality results cited as Propositions 2.5/2.7 and Lemma 5.2. This was a check of the available arXiv text, not a complete published-version citation audit. Rudolf's [published Theorem 1](https://link.springer.com/article/10.1007/s10884-022-10228-0) supplies the capacity/minimal-billiard connection with at most `n+1` bounces for convex Lagrangian products, supporting the kind of nonsmooth planar input used in Section 4.3. These checks support the availability of the cited inputs; they do not prove the thesis's subsequent transformations or algorithms.

No complete certificate rerun, full empirical reconstruction, full software correctness audit, comprehensive literature/novelty search, or current university-rule check was performed. There is consequently no basis here for either “all results verified” or “the results are false.” In particular, **the strength and novelty of the entire portfolio cannot be finally graded from this assessment alone**. Enough is visible to reject topic abandonment as an evidenced necessity; not enough has been independently certified to guarantee that every retained claim survives detailed mathematical review.

## What this diagnosis implies for replacement work

The following distinctions are important input to later workflow choices. “Rewriting” here includes fresh composition from research findings; it does not presume that the existing prose is a useful intermediate artifact.

| Kind of work | Established need | What is not established |
| --- | --- | --- |
| Sentence and paragraph rewriting | Yes, as an independent and substantial requirement: direct communication, manageable sequential dependencies, less internal vocabulary and repetition | That agent comprehension certifies human readability, or that style repair alone supplies missing content |
| Mathematical write-up | Yes: teach the optimization interface and complete the HKO argument in the PDF, including its specialization and connection to the finite checks | That repository availability completes an omitted argument, or that the HKO theorem needs to be rediscovered or weakened |
| Recovering empirical content | Yes: sampling distributions, descriptor definitions, rules, validation and comparison results | That every packet needs rerunning or every historical method needs inclusion |
| Selecting AI-reflection content | Yes: identify insights of sufficient research significance; give the routine sign example at most proportionate brief treatment | That more small examples, more caveats or a new productivity study would make the current chapter valuable |
| Scientific verification | Targeted unresolved needs: measurement reliability, actual claim support, selected proof/algorithm interfaces | That a blanket proof/software audit is a prerequisite to every writing attempt |
| Whole-manuscript composition | Yes: realize the contribution hierarchy, continuous proofs, reader progression and synthesis in fresh text | That the TOC discussed with Kai must be redesigned, one tight narrative imposed, or a specific page count met |
| New mathematical research | Not shown necessary by the assessed PDF | That no hidden support problem can require it |
| New experiments | Possibly needed for selected retained claims after evidence recovery | A mandate for a new comprehensive optimizer comparison or broad search campaign |
| Topic replacement | No evidence that it is required | A final guarantee of examiner approval or novelty |
| Workflow design | This report supplies failure modes and observable outcomes for it | That a deliberately designed workflow is needed if straightforward writing succeeds |

For future prose-writing tasks, follow Jörn's stated preference for GPT-6 Astra and fresh composition from the research. Do not turn F7a into an ongoing investigation of Sol's writing or an exhaustive defect checklist. The replacement must still pass human review; model choice alone is not evidence of that outcome.

A passing replacement should let the intended reader explain the main proof mechanisms, understand what was actually sampled and measured, distinguish contributions from prerequisites, and see what the investigations taught. It must also be tolerable to read in sequence, at a reasonable pace for the mathematics, without relying on Jörn to translate repository status language into mathematics. Those outcomes are a more useful success test than fewer caveats, a clean build, or agent agreement that the prose is improved.

## Review outcome and limits of agreement

Jörn has confirmed the intended reader and explicitly required a separate writing-style/reading-effort criterion. He has read the main diagnosis and said it sounds about right, while correcting its failure to distinguish the research's value from the lack of value in salvaging the current prose. He reports that the TOC was discussed with Kai and found to fit a good narrative, and proposes writing afresh from the research with that structure as a possible starting point. He subsequently read F7a, recognized its examples as defects, questioned its completeness, and proposed using GPT-6 Astra for prose-writing rather than analyzing Sol's defects further. Those corrections are incorporated above. **This is not endorsement of every detailed finding or of an exhaustive diagnosis.** The report should not be cited by other agents as “Jörn agrees that these are all the defects.”

Jörn also reviewed the HKO question and rejected the premise that leaving required mathematical content to repository files is a legitimate presentation option. F1 now records an incomplete mathematical write-up, with completion required in the PDF. No further question about accepting those omissions is pending.

Jörn answered the AI-reflection question by challenging the example's value, not merely its breadth. F11 now records that the routine sign-repair case warrants perhaps two lines and does not sustain a chapter on AI in mathematical research. Its replacement needs worthwhile content, not simply more cases of the same kind.

All specifically requested review judgments have now received responses and are incorporated. Jörn has not performed a line-by-line review or endorsed every technical finding. Those findings remain explicitly attributed to the assessor, with evidence and uncertainty. The report can be shared on that basis: it records the broadly accepted diagnosis and Jörn's substantive corrections, not unanimous verification of every detail or an exhaustive style taxonomy. The replacement thesis still requires its own PASS review.
