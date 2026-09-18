# Open literature map for probing Viterbo's conjecture

First synthesis: 18 September 2026. This search started from the thesis's results and tools, then followed neighboring mathematics and computational research. It is a research resource, not an exhaustive bibliography or a certificate of novelty. No thesis prose or computational producers were changed.

## What changes decisions

1. **There is substantially more neighboring algorithm work than the thesis's immediate formula citations reveal.** Krupp's thesis develops nonconvex-QP/QAP formulations and relaxations; Krupp–Rudolf explicitly treats polygonal factors; Leipold's June 2026 thesis develops exact ILP evaluation and nonsmooth optimization for simplices. Compare our actual contributions with these before claiming a new algorithmic paradigm. This does not invalidate a different implementation, product reduction, or proof. See [algorithms](algorithms.md) and [products](products.md).
2. **The fixed-ten-facet HKO result remains distinct from checked nonsmooth Zoll/cuts results.** Capacity loss under a cut is not a volume-normalized local maximum, and cuts can leave the ten-facet class. The statement-level comparison is now done; use [extrema](extrema.md) rather than repeating the search. All-convex local optimality and generalized-Zoll status remain different questions.
3. **The P5 × arbitrary K programme has direct mathematical and experimental predecessors.** BMP's covering reformulation supplies its endpoint, while related product-billiard algorithms and actual optimization of other polygon-cover problems supply useful context. Coordinate with the empirical desk, which already owns this class. See [products](products.md).
4. **A structured alternative dataset could have known capacities and new volume/geometry questions.** Symplectically self-polar constructions and a recent self-polarity-preserving interpolation offer more than another distribution of generic random polytopes. The interpolation need not retain polyhedral boundaries. Mean-width bounds give another theoretical reason to use geometric features that are not invariants. These are optional scientific opportunities, not scope additions. See [geometry landscape](geometry-landscape.md).
5. **Adaptive data/feature/mathematics loops have concrete precedents.** The knot study of Davies et al. moved through a new feature, deliberately constructed counterexamples, then a repaired theorem. Polytope-learning work explains why a representation succeeds; automated conjecturing retains bounds with informative witnesses. These support the empirical process Jörn proposed, without establishing that it succeeds autonomously here. See [discovery](discovery.md).
6. **A superficially relevant polytope systolic-bound preprint is withdrawn.** Do not rely on arXiv:2509.19083's main bound. Separately, known NP-completeness is a variable-dimension theorem, not a fixed-R4 impossibility result. See [algorithms](algorithms.md).

## Find checked sources without another broad search

| Question or search terms | Start here | Actual coverage |
|---|---|---|
| Lagrangian product, polygon, Minkowski billiard, Q-cover, worm, odd pentagon, duality, Toda | [products](products.md) | Relevant theorem scopes, neighboring representations, open questions and proof-overlap boundaries |
| HK finite formula, QP, QAP, ILP, SDP, Clarke, simplex, flow graph, numerical certification | [algorithms](algorithms.md) | Actual computational prior work and bounded interfaces to optimization; not all combinatorial optimization |
| HKO, cuts, local maximum, fixed facet, Zoll, systoles, capacity equality | [extrema](extrema.md) | Direct project-to-literature comparison plus adjacent dynamics; reading depth recorded |
| Self-polar, Mahler, interpolation, mean width, geometric controls, toric | [geometry landscape](geometry-landscape.md) | One structured alternative family and operations, plus broader capacity context |
| Mathematical discovery, features, invariants, conjectures, counterexamples, symbolic regression | [discovery](discovery.md) | Transferable primary-source case studies with limitations |

Each topic note contains source links, exact editions where checked, theorem/section pinpoints, what was read, the connection to this project, and search/reopening records. An abstract-only lead is not ready to support a detailed theorem claim. “Read statement” is not “independently proved.”

## Relation to existing project work

This map extends [literature closure](../history/literature-closure-plan/README.md), whose introduction correction is already committed in this branch's ancestry. It does not replace the technical audits, [project facts](../project-facts.md), or accepted proof sources. The currently developed affine-pentagon theorem is at the sibling worktree `pentagon-affine-integration/formal/pentagon-affine-products/`; its convention audit and proof ownership remain there. The empirical research desk owns active experiments, including P5 × arbitrary polygon K. Paths in older planning notes may identify other active worktrees rather than sources included in this checkout.

The inherited family-to-generator mapping and capacity normalization audit should be reused. Current readers should not infer that all old planning follow-ups remain undone merely because the older plan says so.

## Incorporation and remaining work

**Ready for writing:** the HKO cuts/fixed-facet distinction; corrected R4 EHZ/cylindrical status; careful mathematical-discovery motivation; accurate indication that product billiard algorithms and nonsmooth capacity optimization have existing literature.

**Requires a bounded comparison before stronger claims:** our code/algorithm versus the precise Krupp/Leipold formulation; proof economy or prior-art overlap of the new affine result; any claim that a sampled family satisfies an imported theorem's hypotheses. These are theorem-to-project comparisons, not reasons to restart the open search.

**Requires a scientific decision before implementation:** self-polar dataset, mean-width bound optimization, new learned representations, active symbolic acquisition, or a new billiard algorithm. Literature existence is not an assignment to build everything.

**Unsettled:** global priority of the new project statements; complete forward-citation coverage; final numbered ABE smooth theorem (full-text access issue); any claim involving arbitrary convex local maximality or generalized-Zoll status of HKO. No conclusion here depends on resolving all of these.

## When another search is warranted

Use this map and primary sources for ordinary writing now. Reopen a bounded search when a genuinely new representation/algorithm is selected; a claimed implication fails its hypotheses; a new result changes the question; an unread lead becomes consequential; or a later substantial time gap makes a current-status claim stale. Lack of a search hit does not establish novelty. Do not interpret this first synthesis as closing the literature indefinitely.

The search deliberately did not survey all Floer theory, symplectic embedding classifications, Mahler theory, general global optimization, or all ML. Specialist queries and backward/forward trails are recorded in the notes; most effort followed concrete mathematical connections rather than fixed citation quotas. Web primary sources were used; no third-party PDF archive is redistributed here.
