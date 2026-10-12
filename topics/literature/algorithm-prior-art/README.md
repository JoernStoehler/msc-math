# Algorithm prior-art comparison

Checked 18 September 2026. This closes the bounded comparison requested after the open literature map. It does not certify priority or independently audit the cited proofs. No manuscript or computational producer was changed.

## Integration decision

Add two short related-work passages: one distinguishing the product closure-vertex enumeration from existing polygon billiard algorithms; one locating the variation/search chapter alongside Leipold's simplex optimization. The inspected chapters already attribute the general formula to Haim–Kislev and the flow geometry to Chaidez–Hutchings. I found no explicit first-in-literature claim that needs retraction and no theorem invalidated by this comparison. The omission is relevant attribution and context, not a reason to discard the project's derivations.

The proposed wording and bibliography entries are in [inserts.tex](inserts.tex) and [references.bib](references.bib). These are editorial proposals, not human-approved prose. Do not insert every detail of this audit into the thesis.

## Primary sources and read depth

* Karla Leipold, *Combinatorial Approaches to the EHZ Capacity of Simplices*, 2026 dissertation. [University record](https://kups.ub.uni-koeln.de/80566/), [PDF](https://kups.ub.uni-koeln.de/80566/1/Dissertation_karla_leipold_.pdf). Read Lemma 3.2, Part III setup, §9.2, §11.3, Algorithm 4 and §12.3–12.5. Printed pages 31–32, 73–74, 79–81, 106–107, 117, 119–125. PDF SHA256 `fe687876b73b4c189a01f2bbc376a057248a347654119b80649e94aa69ac83ad`.
* Stefan Krupp and Daniel Rudolf, *Shortest Minkowski billiard trajectories on convex bodies*, arXiv:2203.01802v1 (2022). [PDF](https://arxiv.org/pdf/2203.01802v1). Read §7.3, especially Proposition 7.1 and the implementation discussion on printed pages 54–64. PDF SHA256 `c9dcde86d18b551c079afd04678817c41eadfc0d4ab19dabd1689e59a9ecbeaf`.

Browser retrieval failed; ordinary HTTPS download succeeded. PDFs were inspected as locally extracted text. Hashes identify the editions; PDFs are not redistributed here. Existing Rudolf/AAO imported-theorem and HKO comparison audits were deliberately not repeated.

## Leipold: exact overlap and limits

**Capacity evaluation.** Lemma 3.2 assumes the simplex's support-one rows sum to zero, obtainable by centring as described immediately before it. Its balance weights are all `1/(2n+1)`. Section 9.2, equations (23)–(24), turns the remaining permutation problem into a binary linear-ordering ILP. Pair variables plus two inequalities per triple exclude directed three-cycles. The skew-symmetric objective permits the maximum to be obtained from the negative minimum. This is a mathematically exact finite reformulation; §12.3 uses Gurobi. Do not describe it as a proved interval/rational certificate implementation: that guarantee was not established by the passages inspected.

Our QP chapter optimizes weights as well as order on general four-polytopes. Except for a simplex or another fixed-weight subproblem, the ILP does not remove that continuous optimization. Conversely, generic integer programming for capacity and avoiding explicit permutation enumeration are already represented in the literature. The thesis can explain its stationarity-system solver without presenting the finite-optimization paradigm as new.

**Variation.** Section 11.3 equations (44)–(46) writes the capacity of `X S_(2n)` as the reciprocal of a finite maximum of smooth permutation functions, derives active-gradient convex-hull expressions, and projects onto `SL(2n,R)`. These are volume-preserving linear deformations of a simplex. Our chapter permits independently moving facet rows in a stable simple-polytope chart: the feasibility matrix and optimizing weights move, so its regular-branch derivative includes the constraint-multiplier term. Its feasible-section upper functions also apply where the optimizing KKT system is singular. These distinctions explain the project's extra derivation; they are not a novelty proof.

**Optimization.** Algorithm 4 uses a minimum-norm generalized gradient and Armijo backtracking. Section 12.3's implementation additionally uses phased fixed steps, tolerated objective increases and a solution-pool cap (typically 5000), explicitly acknowledging truncation of the active set. Thus active-branch optimization with repeated capacity evaluation is prior art, while a numerical active list is not automatically complete. This matches the project's explicit coverage caveat. Section 12.5 treats a broader second-order model as a proposal; it does not supply a theorem that settles the project's general nongeneric first-order boundary or HKO local maximum.

**Do not conflate derivatives.** Our finite-minimum one-sided directional derivative is the minimum of active slopes under its stated coverage assumptions. It is not generally the Clarke generalized directional derivative. A literature citation is not permission to rename it. Leipold's `sys_n = c/(n! vol)^(1/n)` is the nth root of `c^n/(n! vol)`; in dimension four her ratio is the square root of ours.

## Krupp–Rudolf: exact overlap and limits

Section 7.3 explicitly treats two polygonal factors, although the preceding construction begins with a smooth strictly convex factor. It considers two- and three-bounce trajectories and finite choices of faces. Proposition 7.1 (also attributed there to Krupp's dissertation Theorem 4.3.6) states: two admissible primal/dual polygonal pairs satisfying their normal-cone system (2), with corresponding vertices in the relative interiors of the same ordered faces, have the same Minkowski length. Vertices are allowed as zero-dimensional faces. This already establishes fixed-face action constancy in this setting; our thesis should not claim that observation as new.

The two-bounce routine searches face pairs in both factors. The three-bounce routine uses a homothetic-triangle fitting LP, Proposition 7.3, followed by a normal-direction construction. At nonsmooth dual vertices, page 63 expressly discusses perturbing the polygon or sampling finitely many normal directions. The reported implementation uses CVXOPT `conelp`; independent face choices are parallelized. These passages do not establish the exact rational, all-degeneracies numerical guarantee claimed for our closure-vertex route.

Our product proof maximizes the HK bilinear objective over the two normalized closure polytopes. An optimum can be taken at vertices of both, each supported on at most three facets. Enumerating their support unions requires at most six facet directions and 120 cyclic orders per pair. It does not solve the billiard-position LP or identify all minimizing trajectories. This is a concrete representation/implementation difference; neither a complexity advantage nor novelty follows without another argument. In particular, the earlier twelve-facet block family is an exhaustive candidate family, not a claim that older product algorithms required twelve bounces.

## Manuscript scope checked

Actual interrupted candidate at `/tmp/msc-math-thesis-review-20260917/thesis/candidate/`: `04-quadratic-program.tex`, `05-flow-graph.tex`, and `recovered/thesis-candidate/{01-introduction,06-variation}.tex`. These contain the relevant general QP, twelve-facet and six-facet product algorithms, conditional rational flow search, facet-row variation and optimizer interpretations. Assembly is independently changing these files; proposals intentionally use section anchors rather than a brittle patch against that evolving tree.

The flow chapter already says it uses Chaidez–Hutchings local passage geometry and separates its simple-word completeness argument and regularity hypotheses. Neither source compared here supplies a general four-dimensional flow-graph algorithm replacing that discussion. Existing flow/runtime correspondence remains the owner of correctness and certification claims.

## Remaining uncertainty and stop condition

This comparison is sufficient for modest attribution and accurate description of the method differences. It does not establish priority of the six-facet closure-vertex proof, comparative speed, completeness of another author's implementation, or correctness of every nonsmooth optimization theorem in Leipold. Do not claim those outcomes. No new literature search or benchmark is needed merely to add the proposed context. Reopen a narrower comparison if the thesis later asserts one of those stronger claims.
