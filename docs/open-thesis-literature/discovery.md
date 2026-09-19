# Empirical mathematical discovery: transferable literature

Search and source inspection: 2026-09-18. This note supports research design and thesis context; it reports no project experiment and does not recommend importing an entire ML system. Read with the empirical desk's `mathematical-discovery.md` and `adaptive-discovery.md`.

## What changes the programme

The strongest precedent is not “train many predictors and pick the best.” Existing primary research supports several distinct scientific outputs: a geometrically interpreted feature and theorem, a conjectured sharp bound, a small counterexample, or a concise generative construction. These precedents establish that such processes have succeeded elsewhere, not that they will succeed on symplectic capacities within our budget.

For the present programme, the highest-value additions seem to be: (1) explicit candidate inequalities with witnesses showing their added information; (2) selecting new geometries where competing explanations disagree; (3) interpreting dimension reduction and learned features mathematically; and (4) reducing numerical constructions to simple generative rules. These are design inferences, not conclusions of the cited papers about this project.

## Primary sources and transfer boundaries

### 1. Davies et al.: features, counterexamples, then a repaired theorem

[Advancing mathematics by guiding human intuition with AI](https://www.nature.com/articles/s41586-021-04086-x), Nature 600, 70–74 (2021), published 1 December 2021. Full main text inspected, especially Topology and Methods.

The knot study used prediction and attribution to focus on cusp quantities, then introduced the geometrically meaningful natural slope. An initial slope–signature bound involving volume survived several sampled distributions but failed on deliberately constructed braids. Adding injectivity-radius dependence yielded a theorem. This is unusually close to the proposed evolving-data/evolving-feature process: prediction identifies a promising relationship; mathematics supplies a new quantity; adverse constructions expose missing hypotheses.

Transfer: a ridge/capacity association can be useful even if its first universal interpretation fails. The next output should be an explicit corrected statement or explanatory feature, rather than another aggregate accuracy number. Attribution is evidence about the fitted predictor, not proof that a quantity controls capacity. Multiple successful sampling distributions did not remove the need for adversarial examples. The paper's human mathematical intervention was substantial; it does not establish autonomous discovery.

### 2. Coates, Hofscheier and Kasprzyk: interpret a representation, not just a score

[Machine Learning the Dimension of a Polytope](https://arxiv.org/html/2207.07717), arXiv:2207.07717v1, 15 July 2022. Sections 2–3 inspected.

The study generates lattice/rational polytopes and predicts dimension, volume and quasi-period from Ehrhart-series data. Representation matters sharply: two principal components of logarithmic coefficients work very well for dimension, while compressing raw coefficients to two components performs poorly. Section 2.4 explains the log representation through the leading Ehrhart asymptotics; other sections interpret the observed structure mathematically. Code/data provenance is supplied.

Transfer: inspect what a low-dimensional representation actually encodes before celebrating discovered clusters or high prediction accuracy. For our incidence/pairing representations, a useful success may be identifying an already-understood leading effect, then studying departures from it. This paper concerns lattice-point invariants, not symplectic capacities; Ehrhart data may be expensive and is not an intrinsic replacement feature for our nonlattice geometry. It supplies a practice and comparison, not a relevant new capacity theorem.

### 3. Davila / Graffiti: useful bounds rather than correlation rankings

[Automated conjecturing in mathematics with TxGraffiti](https://arxiv.org/html/2409.19379v1), arXiv:2409.19379v1, 28 September 2024. Sections 2 and 4 inspected.

The classical Dalmatian idea admits a conjectured bound if it survives the current examples and improves on retained bounds at some example. The paper also describes a static filter using sets of equality instances. These are finite-data heuristics, not mathematical validity or independence certificates. In particular, the static implementation retains some candidates with identical equality sets; its prose about eliminating redundancy should not be adopted as a guarantee.

Transfer: maintain a small frontier of interpretable bounds with explicit improvement witnesses, deliberately search for violations, and retain superseded candidates in case their replacements fail. This can discover conditional inequalities without treating systolic ratio as the only target. Floating-point near-equality and numerically uncertain capacities need explicit treatment; graph-integer “touch” counts cannot be transplanted verbatim. A finite family label is not automatically a meaningful mathematical hypothesis.

### 4. Wagner: search for falsifying constructions and simplify them

[Constructions in combinatorics via neural networks](https://arxiv.org/html/2104.14516v1), arXiv:2104.14516v1, 29 April 2021. Sections 2.1–2.2 inspected.

The deep cross-entropy procedure generates constructions, evaluates a reward, and learns from successful samples. It produces counterexamples to combinatorial conjectures. One illustrative spectral/matching conjecture had already been disproved; the paper explicitly acknowledges this and finds a smaller example. Thus a successful numerical search and a novel mathematical disproof are separate claims.

Transfer: once a concrete inequality or sufficient-descriptor hypothesis exists, reward its violation and seek a small interpretable witness. The search objective need not be maximizing systolic ratio. For continuous polygons, legality-preserving parameterizations and costly capacity evaluation are additional obstacles; the graph experiments do not establish that training a neural proposer beats direct optimization here. Simple existing search machinery should be the initial comparator.

### 5. Medina and White: request the next datum from competing equations

[Active Learning in Symbolic Regression with Physical Constraints](https://arxiv.org/abs/2305.10379v3), arXiv:2305.10379v3, 9 August 2024. Abstract/version record inspected; detailed experimental claims not audited.

The method selects new observations by disagreement among a Pareto frontier of symbolic equations and uses physical constraints. Reported evidence concerns data-efficient rediscovery of known equations, not new theorems in convex geometry.

Transfer hypothesis: if several simple expressions explain retained polygon data, choose a legal polygon or deformation parameter at which their predictions differ, rather than collecting another indiscriminate batch. Constraints available here include homogeneity, verified symmetries, positivity and exact family values. Committee agreement does not establish truth, and a common missing feature can make all candidate equations fail together. This is a concrete acquisition option only after competing expressions exist; it is not a reason to build active-learning infrastructure first.

### 6. Udrescu and Tegmark: structural reduction before symbolic fitting

[AI Feynman: a Physics-Inspired Method for Symbolic Regression](https://arxiv.org/abs/1905.11481v2), arXiv:1905.11481v2, 15 April 2020; Science Advances 6, eaay2631. Abstract/version record inspected.

The algorithm exploits symmetries, separability and composition while recovering known formulae from data. Its benchmark success concerns synthetic formula recovery; it is not evidence that an unrestricted capacity function has a short expression in our current features.

Transfer hypothesis: remove proven scaling/symmetry degrees of freedom and separate active-branch regimes before fitting formulas. A lower envelope of simple capacity branches can look complicated globally. This argues for giving symbolic fitting the right mathematical object, not merely substituting it for another regressor. Verify inferred formulas exactly on a proposed domain; a fit across finite samples cannot establish branch completeness.

### 7. Romera-Paredes et al.: discover a construction rule

[Mathematical discoveries from program search with large language models](https://www.nature.com/articles/s41586-023-06924-6), Nature 625, 468–475 (2024), online 14 December 2023. Main description and cap-set result inspected. [Authors' implementation](https://github.com/google-deepmind/funsearch).

FunSearch searches programs paired with an evaluator, producing cap-set constructions and bin-packing heuristics. A programme generating a family can be more interpretable and transferable than one isolated numerical example. The cap-set problem was advanced, not solved.

Transfer hypothesis: if numerical optimization repeatedly finds related polygons, seek a short polygon-generation or deformation rule and test that rule on fresh parameters. Do not introduce a full evolutionary LLM harness merely because this precedent exists: correctness of our geometric evaluator, admissibility and cost per evaluation remain decisive. This is most useful after an interesting repeated construction appears, not as a replacement for defining the scientific question.

## Concrete uses without launching another large campaign

These are proposals for the empirical owner, not new assignments:

1. **Conjecture table rather than method table.** For each live geometric relation retain its domain, expression, supporting cases, falsifying case, known-theorem explanation, and the next geometry where alternatives disagree. This makes adaptive follow-up legible and gives the eventual thesis a scientific account.
2. **One inequality frontier.** On an already-authorized representation dataset, ask whether a small set of dimensionally consistent expressions yields complementary empirical bounds. Compare to known inequalities first. Stop if all candidates merely repackage established information.
3. **One disagreement acquisition.** Given two actual explanations of a phenomenon, design one admissible path on which they diverge. Check whether its geometry is computable before recommending a larger active-learning method.
4. **Recover a construction rule.** For a meaningful extremal or exceptional output, seek a simple parameterization and exact coordinates. This can turn a search result into mathematics even without a better global maximum.

The fixed-pentagon/arbitrary-partner programme is already owned elsewhere. These suggestions neither duplicate its capacity-interface work nor assume its eventual scientific objective must be HKO optimization. Likewise, do not revive the rho–ridge scout as independent evidence: the empirical desk has identified substantial information already forced by prior correlations.

## Search coverage and reopening triggers

Queries covered ML-guided pure-mathematical discovery, polytope/Ehrhart learning, invariant inequality conjecturing, reinforcement-learning counterexamples, symbolic active learning and program-generated constructions. Primary sources above were followed rather than relying on news or model-generated summaries. A targeted repository search for these authors/method names in `thesis/`, `papers/` and `experiments/sys-datascience/` Markdown/BibTeX/TeX found no hits in this worktree; that is not a complete repository-wide absence claim.

This is a selective practice-oriented map, not an exhaustive history or tool benchmark. Original AutoGraphiX, the original Dalmatian papers, and broader active experimental design are follow-up leads if implementation depends on them. No claim about the latest strongest method is made. Reopen targeted search when a concrete expression grammar, acquisition objective, geometry encoding or counterexample-search bottleneck appears. Ordinary thesis motivation can already cite the directly inspected precedents above without repeating the broad web search.
