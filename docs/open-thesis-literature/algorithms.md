# Algorithms, optimization and computational prior work

Checked 18 September 2026. This is a literature discovery and comparison note, not an audit of the project's proofs or implementation. Read depth is stated below; a bibliographic connection is not an imported theorem contract.

## Findings that change the landscape

The most consequential new source is **Karla Leipold's 2026 dissertation**: it combines simplex capacity evaluation by integer programming with active-branch nonsmooth optimization and symmetry testing. Thus the relevant prior work is already considerably closer than a generic combinatorial-optimization survey. **Krupp's earlier dissertation** is the place to look before independently inventing conic relaxations or product billiard algorithms. A search hit advertising a universal simplex bound is **withdrawn**, and must not be used as a theorem.

These findings do not establish that the project's implementation, derivative results, or HKO local theorem are subsumed. The appropriate comparison is by domain, hypotheses, numerical guarantees, and exact output—not by shared algorithm keywords.

## Source cards

### Leipold 2026: simplex ILP and nonsmooth optimization

[Repository record](https://kups.ub.uni-koeln.de/80566/), [accepted dissertation PDF](https://kups.ub.uni-koeln.de/80566/1/Dissertation_karla_leipold_.pdf), oral exam 8 June 2026. Read contents, §9.2, §11.3, §12.5, appendix; no full proof audit.

§9.2 encodes simplex capacity as a linear-ordering ILP: binary pair-order variables and triangle inequalities prohibit directed three-cycles. §11.3 gives active-branch Clarke formulas on SL(2n); §12 develops retraction/line-search optimization. The appendix identifies [author code](https://github.com/kleipold/CombinatorialApproachestoehzcapacityofsimplex), with capacity, gradient-search and symplectomorphism routines. §12.5 explicitly treats the proposed nonsmooth second-order manifold model as conceptual, not a full theorem. Its systolic normalization is the nth root of this project's ratio; numerical values cannot be copied directly.

**Project use:** compare variation and optimizer chapters before making novelty statements. The exact simplex specialization does not immediately supply a general-polytope solver: simplex balance weights are fixed, unlike the project's continuous weight search. A selective code/statement comparison is useful; a new large implementation project is not forced by this discovery.

### Krupp 2020/2021: conic and spectral alternatives

[University record and code ZIP](https://kups.ub.uni-koeln.de/36196/), [dissertation](https://kups.ub.uni-koeln.de/36196/1/DissertationKrupp.pdf). Title page 2020; defense 18 February 2021. Read abstract, Chapter 5 derivation passages, especially Theorem 5.3.5 and (5.20)–(5.23); no full proof audit.

Chapter 5 treats finite capacity optimization through quadratic assignment/nonconvex quadratic programming. It develops spectral bounds and completely-positive formulations, relaxed by positive-semidefinite plus entrywise-nonnegative constraints. Equation (5.21) bounds **1/(4c_EHZ)** from above, hence bounds capacity from below after positive reciprocation. This direction matters for pruning or claimed certificates. The repository also supplies [author implementation](https://github.com/S-Krupp/EHZ-Capacity-of-polytopes).

**Project use:** this is relevant algorithmic prior art and a possible alternative bound/oracle, not automatic certified numerics. Comparing tightness and total cost to the existing exact support search would precede adoption. This pass did not establish floating-point certificate guarantees or benchmark equivalence.

### Leipold–Vallentin 2024: complexity boundary and graph interpretation

[arXiv:2402.09914v3](https://arxiv.org/html/2402.09914v3), 5 October 2024. Read Theorem 1.1, reduction outline, §3. The theorem states NP-completeness of deciding whether a rational H-described polytope has EHZ capacity at most a rational threshold; hardness already occurs for simplices. The reduction increases dimension. It does **not** establish hardness for fixed dimension four.

The fixed-weight simplex expression becomes weighted ordering/maximum-acyclic-subgraph optimization. This gives a concrete interface to discrete optimization, rather than a reason to survey all combinatorics.

**Project use:** explain why a finite formula alone does not promise scalable general computation. For any fixed-beta permutation subproblem, weighted ordering suggests an algorithmic direction, but solving it does not jointly optimize beta. Do not claim an efficient fixed-R4 method is ruled out, or transfer simplex ILP performance without checking that distinction.

### Haim–Kislev 2019: finite formula and simple minimizer

[arXiv:1712.03494v3](https://arxiv.org/pdf/1712.03494v3), [journal DOI](https://doi.org/10.1007/s00039-019-00486-4). Read Theorems 1.1 and 1.5, Remarks 1.4 and 1.6; existing project convention/proof audits remain owners.

Theorem 1.1 optimizes facet permutations and nonnegative balanced, support-normalized weights. Theorem 1.5 supplies an action minimizer using each facet-normal direction at most once. Remark 1.6 warns that other minimizing characteristics need not have this structure. Corollary 1.2 simplifies centrally symmetric inputs. Remark 1.4 relates the construction to dual-action discretization.

**Project use:** distinguish finding a capacity-realizing simple representative from enumerating every minimizer. Central symmetry is a possible separate optimized route if relevant datasets justify it, not a reason to change the current general kernel. [The author's Matlab repository](https://github.com/pazithaimkislev/EHZ-capacity) is a baseline implementation; its README explicitly describes current scaling limitations and possible orbit-based permutation elimination.

### Chaidez–Hutchings 2021: graph dynamics and higher-orbit questions

[arXiv:2008.10111v2](https://arxiv.org/pdf/2008.10111v2), [journal record](https://www.aimsciences.org/article/doi/10.3934/jcd.2021016), J. Computational Dynamics 8(4), 403–445. Read Theorems 1.11–1.12, Corollary 1.15, Remarks 1.13 and 1.19, §1.5–1.6 and flow-graph boundary footnote.

For symplectic four-polytopes, smoothing correspondence retains action/index information. Positive face constants and rotation bounds control relevant word complexity; action bounds alone can permit arbitrarily many segments near bad edges. Corollary 1.15 includes Type 2 orbits, not only interior Type 1 graph cycles. The paper discusses experiments beyond minimum action, including index-selected action ratios.

**Project use:** the existing exact rational route intentionally has a narrower stated input contract and does not implement rotation pruning. Compare those scopes explicitly. Higher-orbit statistics offer a distinct future empirical direction if reliable index and boundary handling are available; current scalar QP output alone is insufficient. Beware edition numbering: the arXiv v2 capacity corollary is 1.15, while one internal footnote still says 1.13.

### Krupp–Rudolf 2022: planar Minkowski billiard algorithms

[arXiv:2203.01802v1 PDF](https://arxiv.org/pdf/2203.01802v1). Read Definition 1.2/Theorem 1.3 and §7.3; no complete algorithm audit.

Strong and weak nonsmooth billiard notions differ; Theorem 1.3 needs strict convexity for the reverse implication. §7 treats planar algorithms, with separate two- and three-bounce searches and small LPs. §7.3 reports independent face-choice calculations suitable for parallelization; it also discusses perturbation or sampled normals at ambiguous dual vertices. These details prevent casually identifying all implementations with an exact nonsmooth capacity oracle.

**Project use:** use as neighboring algorithmic prior work and benchmark/design lead. For the product-capacity equality itself, route to Rudolf's separate [arXiv:2203.01718](https://arxiv.org/abs/2203.01718) and its already-assigned imported-theorem contract. The new fixed-pentagon partner campaign has its own BMP interface owner; do not launch a duplicate producer here.

### Zediker 2023: rotated four-cubes and a statistical computational approach

[University MSc record](https://egrove.olemiss.edu/etd/2602/), 2023, advisor Samuel Lisi. **Abstract-level only:** PDF endpoint failed through the browser tool. It reports an upper bound on normalized capacities of rotated hypercubes, a linearized cylindrical-capacity formulation, and a distribution result for random rotations.

**Project use:** directly relevant prior context for rotation/orientation and feature-discovery experiments; its target is not automatically EHZ. Retrieve the full text before adopting a theorem, distribution law or estimator. This is a concrete follow-up lead, not a verified exclusion for current families.

### Withdrawn source: Zediker 2025

[arXiv:2509.19083v2 status](https://arxiv.org/abs/2509.19083v2) records withdrawal on 30 September 2025. The author reports a possible R6 counterexample to Theorem 1.1 and says the remaining new results depend on it. Do not import the abstract's advertised sharp simplex bound. This withdrawal does not invalidate the separate 2023 MSc thesis.

## Bounded follow-ups and limits

1. **Necessary literature integration:** compare Leipold's simplex active-branch analysis to our variation discussion and cite Krupp's optimization alternatives. Preserve the project's different general-polytope and certificate scope.
2. **Potentially useful short technical comparison:** inspect the simplex ILP as a regression oracle or fixed-weight ordering solver. First check whether any live bottleneck calls for it; fixed-R4 five-facet simplices may be too cheap already for an ILP to help.
3. **Certification research only if triggered:** a conic relaxation is not a validated bound merely because its mathematical optimum has the right inequality. Need verified feasible primal/dual witnesses and a mapped normalization before claiming certification. No directly reusable certified-EHZ package was established by this search.
4. **Empirical lead:** full-text Zediker retrieval, then decide whether his orientation distribution supplies a useful baseline. Leave campaign decisions with the empirical desk.

This pass did not survey general SDP, graph optimization, interval arithmetic, or nonsmooth optimization textbooks exhaustively. The directly relevant primary sources now provide precise entry points; further searches should follow a concrete mathematical/algorithmic question.

## Search/reuse record

Repository seeds: `formal/README.md`, `docs/literature-closure-plan/README.md`, `crates/symplectic/src/algorithms/flow_graph/README.md`, `experiments/dev-quadratic-program/README.md`. These show existing proof-audit owners, rational FG scope, and selected-route correspondence; no source or producer was modified.

Queries (18 September 2026): `Haim Kislev symplectic capacities convex polytopes combinatorial formula algorithm`; `Chaidez Hutchings computing Reeb dynamics four dimensional polytopes 2021`; `symplectic capacity computation convex bodies algorithm numerical optimization billiards Rudolf`; exact titles `Computing the EHZ capacity is NP-hard`, `Calculating the EHZ Capacity of Polytopes`, `Combinatorial Approaches to the EHZ Capacity of Simplices`; `symplectic capacity numerical computation convex optimization Abbondandolo Majer`; `EHZ certified computation capacity` (mostly irrelevant electrical-meter hits). Followed source links to university dissertations and author code, not aggregator interpretations. Parent supplied the Zediker titles. No author contacted, code executed, or dataset generated.
