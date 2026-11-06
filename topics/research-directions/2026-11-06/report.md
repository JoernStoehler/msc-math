# A research agenda for computational symplectic geometry

**Decision document, 6 November 2026.** Prepared for the project agent through Jörn. Scope: research specification, mathematical reasoning, literature reconnaissance, and proposed investigations. No experiments, certificate replays, repository changes, or independent proof audits were commissioned or performed.

## 0. Overview: seven questions, several ways to answer each

The programme should investigate **extremal laws, local mechanisms, dynamics, geometric information, computational tractability, discovery methods, and distinctions between capacities**. An improved systolic ratio is a possible result of several tracks; it is not the common definition of success.

| Track | Scientific question | Most useful next discriminator | A worthwhile result if the preferred conjecture fails |
|---|---|---|---|
| **T1. Sharp bounds in specified classes** | What is the best capacity–volume inequality in a named class, and what are its equality cases? | For a fixed regular pentagon, attack the unrestricted minimum-area translation-cover problem. | A smaller certified cover; a counterexample to a proposed reduction; a weaker sharp bound for a defensible subclass. |
| **T2. Local extremality and the complexity of ascent** | Which perturbations improve the ratio, and what geometric complexity is required? | Separate the seven-cut threshold inside the known family from the unrestricted 12–16-facet frontier. | A classified obstruction, an exceptional rank-deficient case, or a destabilizing direction for the second local maximum. |
| **T3. Minimizing trajectories and their degenerations** | What is the space of actual action minimizers, and how does it change under perturbation? | Describe one degenerate product's minimizers modulo motions within flat faces, then follow a specified perturbation. | A failure of orbit selection or coding stability with a precise replacement statement. |
| **T4. Geometric information that determines or bounds capacity** | Which measurements retain the information needed for an exact value or useful inequality? | Compare complete facet-pairing data with controlled compressions and certified matched examples. | An impossibility result for a descriptor, even if it remains a useful predictor. |
| **T5. Scalable computation with checkable bounds** | Which reductions, relaxations, and certificates make capacity computation tractable and independently verifiable? | Compare complete-word certificates with a product-specific covering certificate on the same mathematical target. | A rigorous approximation, a complexity obstruction, or an identified class where a method genuinely applies. |
| **T6. Reliable empirical mathematical discovery** | Which search policies find informative examples efficiently, and under what sampling laws? | A prospective comparison on fresh, explicitly defined families, including certification and review costs. | A negative transfer result or a finding that a simpler baseline is better. |
| **T7. Comparing capacities and embedding information** | What do EHZ values reveal, and fail to reveal, about other symplectic measurements? | Certify a cut-additivity defect, or seek a quantitative embedding bound beyond the automatic volume bound. | A precise limitation of scalar/action data or a useful interval for another capacity. |

These tracks are grouped by **the kind of answer they produce**, not by repository ownership, available software, or a particular example. T1 and T2 are separate because a global class inequality and a local perturbation theorem have different quantifiers. T3 and T5 are separate because computing a scalar is much weaker than classifying trajectories. T4 and T6 are separate because information sufficiency, universal inequalities, prediction, and candidate selection require different evidence. Routine computational maintenance is a shared service; T5 becomes research when it produces a new reduction, correctness theorem, complexity result, or demonstrable algorithmic advantage.

**Initial recommendation.** Compare four discriminators first: an unrestricted covering lower bound, a complete perturbation model, one degenerate trajectory family, and one controlled geometric-information test. Develop certification only far enough to make those results trustworthy. These are candidates for a bounded initial portfolio, not assigned jobs; Section 6 gives evidence that should change their order.

The most consequential additions to the supplied seeds are: nonsmooth Zoll/cut-additivity theory; a complete-versus-compressed pairing-data hierarchy; fixed-dimensional complexity as distinct from general NP-hardness; geometric controls obtained by varying symplectic position while preserving Euclidean shape; and a cheap, theorem-backed capacity lower certificate supplied by translation covers.



## T1. Sharp bounds in specified classes

**Question and motivation.** For a precisely defined class $\mathcal C$, determine or improve


$$
C_{\mathcal C}=\sup_{K\in\mathcal C}S(K),
$$


then ask separately about attainment, equality, and stability. Different choices of $\mathcal C$ are separate targets: fixed-factor planar products, all planar products, prescribed symmetry classes, a facet budget, or all convex four-bodies. A theorem about one should not be presented as an approximate resolution of another.

**Progress and boundary.** The project accounts supply sharp results for several polygon rotations, affine factor families, centrally symmetric partners of a fixed pentagon, and restricted asymmetric normal families. Arbitrary asymmetric partners remain open. HKO supplies the external product example. The later twenty-facet project example raises a lower bound for unrestricted four-bodies; it does not automatically raise the best lower bound within planar products. [P0, P3; L1]

The Pro return reports that a current planar covering paper adds useful scope information: quadrilateral and affinely regular hexagon factors satisfy the ratio-one inequality for arbitrary partners; its general product bound is some $\kappa<\sqrt2$. It also identifies a half-symmetric product question. These are meaningful boundaries for allocation and dataset controls. [L3, Theorem 1.2, §5.3, Question 4.4]

**Leading approach.** For fixed regular pentagon $Q$, normalize the partner's capacity to one and minimize its area. The finite triangle-cover equivalence makes this an optimization of the convex hull of translated templates. The ten templates give 18 relative translation variables after anchoring one. The restricted ten-direction problem is already recorded as solved in the repository; it is the unrestricted area lower bound that remains. [P2; L3, Theorem 3.3 and §6.1]

**Competing approaches.** One can seek a geometric area inequality, a complete certificate over hull-contact cases, or a structural compression that moves every cover into a tractable class without increasing area. A published exact translation algorithm exists, but its dependence on the number of templates and real-arithmetic model does not establish practical feasibility here. Parallel sliding provides useful convex subproblems without making the full placement problem convex. [L4, Theorem 3 and Lemma 6]

**Additional branches.** The Pro return identifies a quaternionic result as a baseline theorem, not merely a prospective connection: the relevant right $Q_8$-action forces $S\le1$. The next questions concern weaker symmetry, nonsmooth equality, or quantitative departure from this class; the right $I$-action has a weaker bound and a conditional sharp result. [L7, Theorems 8–11, Conjecture 17] Near centrally symmetric polar products, the capacity identity reduces the polar slice to Mahler geometry, where planar stability already exists. New work should address transverse departures from polarity or a genuinely different class. [L9, L10]

**Why failure is informative.** A cover smaller than the HKO cover refutes fixed-factor optimality with a geometric certificate. Failure of a proposed fan reduction tells us which contact configurations matter. A sharp weaker inequality or a stability theorem within an honest subclass is valuable without deciding the full product constant. Mere failure of a local placement search proves none of these.

## T2. Local extremality and the complexity of ascent

**Question and motivation.** Why can a body resist many perturbations and still admit a coordinated improvement? Which perturbation classes are sufficient to detect its instability? The answers explain search failure, identify the correct local geometry, and turn numerical directions into structural statements.

**Progress and boundary.** On the supplied project account, HKO is locally maximal among at most eleven genuine facets in an existential Hausdorff neighborhood, and maximal over its full $GL(4)$-orbit. Yet specified simultaneous radial cuts give ascent. A different ten-facet hexagon–quadrilateral product $P^*$ has a non-strict local maximum at ratio one within the established fixed-ten-facet model, with two equality parameters after quotienting symmetries; its general eleven-facet extension is unresolved. These examples should both appear in the proposal. They separate stability, equality moduli, and combinatorial changes. [P0, P3, P6]

For the supplied cut profiles,


$$
D^+\log S=A\sum_i w_i-B\max_i w_i,\qquad 6A<B<7A.
$$


This proves a seven-direction support threshold **within those profiles**. Define instead the unrestricted local complexity threshold


$$
m_* =\min\{m:\text{arbitrarily Hausdorff-close to HKO there is a body with at most }m
\text{ genuine facets and larger }S\}.
$$


The supplied results imply $12\le m_*\le17$: the upper endpoint uses the seven added cuts, while twelve through sixteen remain unsettled. The support threshold of a chosen cut model is not a lower bound for $m_*$. Nor is the two-hinge first-variation statement a twelve-facet neighborhood theorem. [P0; deduction in §7.2]

**Leading approach.** Use the existing simultaneous-splitting derivative theorem where its rank condition holds for every maximizing word. Build the complete linear response constraints, quotient translation/dilation/symplectic directions, and ask for the smallest support on which all genuine bottlenecks improve after accounting for volume. This is a structured sparsity problem, potentially an exact finite feasibility problem.

**Competing approaches.** For rank-deficient points, use feasible action sections and stationary families; alternatively stratify the finite parameter space semialgebraically and analyze the discriminant where ranks, signs, or active words change. A fixed-facet capacity value is semialgebraic under algebraic parametrization, without any need to assert globally smooth KKT branches (§7.6). Support-function and Minkowski-sum variations provide another chart; the established Brunn–Minkowski inequality supplies constraints, not an automatic ratio improvement. [L11]

**Missing step.** A neighborhood theorem must cover moving old rows, all relevant limiting new normals, and different rates of vanishing cut depths. Finite directional tests and one-sided derivatives do not control zero-slope directions, higher orders, or non-uniform neighborhoods. For $P^*$, first decide whether the known equality family accounts for all neutral directions before importing HKO's proof pattern.

**Why failure is informative.** A lower-complexity ascent changes the geometric picture. A complete obstruction for a named birth type is also useful. If a response model fails, the failed rank or uniformity hypothesis identifies a new stratum to study. A continuation that only increases the known record is lower priority unless it exposes a new active-word transition or reusable certificate.

## T3. Minimizing trajectories and their degenerations

**Question and motivation.** Scalar capacity hides multiplicity, placement, topology, and limits of minimizers. Understanding these objects can explain active-set changes, produce geometric obstructions to perturbations, and supply information needed by capacities beyond the first action value.

**Progress and boundary.** The project's QP method handles scalar capacities of degenerate products, while its physical flow-graph theorem requires nonzero relevant pairings, independence of small row sets, and finite-orbit regularity. Products violate these conditions. Tied QP words are not an inventory of physical trajectories, and agreement on scalar values does not validate word-by-word correspondence. [P0, P4, P5]

Rudolf's general billiard characterization provides a nonsmooth scalar bridge and existence of a minimizer with at most three bounces in the planar setting. It carefully distinguishes strong and weak billiards and does not identify all minimizers from a finite optimizing list. [L2, Theorems 1 and 4] Chaidez–Hutchings gives a smoothing correspondence for its specified symplectic polytopes and orbit types; its hypotheses must remain separate from the project's implementation contract. [L5, Definition 1.4, Theorems 1.11–1.12]

**Leading approach.** Start with an exact product and describe actual minimizers via the normal-cone billiard equations, recording physical endpoint sets and zero-length degenerations. Compare that description with QP optima after fixing action, sign, cyclic-order, and time conventions. Then specify a one-parameter perturbation and ask which components persist, merge, or disappear.

**Promising alternative.** Nonsmooth Zoll theory supplies a quotient that forgets internal motions along flat coisotropic faces; a coisotropic tangent space contains its symplectic orthogonal. Haim-Kislev's Proposition 1.6 and Theorem D give a reason to consider this quotient for polytopes and products. [L6] A proposed finite description of that quotient is a more focused target than “extend the flow graph to all degeneracies.” It is still a new obligation: compactness of a quotient does not prove finite representability or convergence for every moving body.

**Further questions.** Distinguish minimum-action dynamics from the dynamics of typical orbits. The HKO literature's special periodic behavior makes perturbations of return maps worth considering, but return-map construction and regularity must precede claims about entropy, ergodicity, or genericity. Orbit topology and index information are a selective extension once supported smoothing models exist, not something the scalar solver already returns. [L1, §3; L5]

**Why failure is informative.** A product can exhibit constant capacity while its minimizing trajectory space changes. A counterexample to a proposed selection theorem would specify the topology or equivalence relation that needs weakening. A complete analysis of one singular family, including its failures of coding, is a worthwhile mathematical result even without a general algorithm.

## T4. Geometric information that determines or bounds capacity

**Question and motivation.** The phrase “geometry explains capacity” conceals several questions. Is a representation sufficient to determine capacity? Does a quantity give a universal bound? Does it describe a dynamical mechanism? These differ from predicting typical values under a chosen generator, which belongs to T6.

**Progress and boundary.** The project has adverse examples and partial inequalities: signed ridge-area circulation constraints, a failed moment inequality, geometries sharing covariance summaries but differing in higher or dual measurements, and restricted zonotope pairing-area inequalities. None alone establishes an EHZ inequality. In particular, different higher descriptors do not prove different capacities; that requires another argument. [P0]

**A rigorous starting hierarchy.** Write a bounded full-dimensional polytope as $K_A=\{x:Ax\le\mathbf1\}$ with the origin interior and normalized facet rows. Its complete labeled matrix


$$
\Omega=AJA^T,\qquad \Omega_{ij}=\omega(a_i,a_j)
$$


determines the body up to linear symplectic equivalence, provided the rows span the ambient space. Section 7.3 proves this elementary assertion. It is not a new low-dimensional descriptor: it is a complete representation, and changing the chosen origin changes its normalization. It nevertheless gives a precise upper end of an information hierarchy. Compressions such as pairing maxima, spectra, histograms, signed cycles, and ridge summaries can now be assessed for what they lose.

There is also a meaningful analytic baseline. For centrally symmetric bodies, the reciprocal of the largest polar symplectic pairing controls EHZ capacity within a factor of four. [L8, Theorem 1.6] The mathematical opportunity is to identify which additional closure or ordering information sharpens that coarse bound in a chosen class. A list of pairings without its incidences need not retain the cyclic order constraints in the capacity formula.

**Competing approaches.** Construct matched bodies that agree on the proposed descriptor but have certified disjoint capacity intervals; prove inequalities for a restricted class and characterize the equality obstruction; or vary symplectic position while preserving the entire Euclidean shape. For orientation-preserving rotations in four dimensions, the effective choice of orthogonal symplectic structure is a two-dimensional sphere (§7.4). This provides a controlled alternative to comparing unrelated random bodies.

For planar products, use support-function perturbations whose odd part changes while width stays fixed. Remove first harmonics, which merely translate the body, and enforce convexity. Two-bounce lengths cancel odd support data; three-bounce candidates need not. This isolates a mechanism worth testing rather than treating “asymmetry” as an undifferentiated scalar.

**Missing step.** Choose an exact target: sufficiency, a bound with a stated constant, or a perturbation mechanism. Then demand the corresponding evidence. A universal inequality needs a proof connecting the geometric quantity to EHZ, not only a positivity statement about that quantity. A feature collision needs a capacity separation, not only a geometric separation. Symplectic invariance must be preserved or its failure explicitly modeled; arbitrary affine whitening can change capacity.

**Why failure is informative.** An impossibility theorem identifies what every successful descriptor must remember. A feature can fail as an exact representation while succeeding as a predictor. Conversely, a provable bound may be too loose for candidate selection. These outcomes should be reported separately.

## T5. Scalable computation with checkable bounds

**Question and motivation.** The aim is to understand and reduce the mathematical cost of a trustworthy result. Faster arithmetic, a larger candidate list, and a correctness theorem are different advances.

**Progress and boundary.** The project already has outward enclosures, exact fallback, singular-family treatment, and a separate rational product route. The six-facet product reduction is a structural achievement. These facts do not make every candidate stream complete. The inspected production account additionally limits its scalar input contract to at most sixteen facets and a bounded cycle budget; the twenty-facet certificate belongs to a separate route. Certification of the exact dyadic input is not a rounding-error bound for a different algebraic body. [P0, P4]

Haim-Kislev's formula and simple minimizing representation give the finite mathematical starting point. [L12, Theorems 1.1 and 1.5] General capacity computation is NP-hard with dimension allowed to grow; that result does not settle fixed four-dimensional complexity. Linear-ordering and nonsmooth simplex optimization are already represented in the literature. [L13, Theorem 1.1; L14] New work should target a specific structural parameter, a degeneracy class, certificate size, approximation quality, or a measured advantage over those alternatives.

**Leading approach.** Separate an untrusted searcher from a smaller checker. A complete certificate must cover the input geometry, the candidate family, justified exclusions, singular stationary systems, an attainer, and any required volume. A list produced by a floating search is not a completeness certificate. Section 7.1 fixes the directions of all bounds.

**Competing approaches.** For products, use closure-polytope vertices, strong-billiard face systems, or translation-cover certificates. The last can prove a capacity lower bound without computing the exact value. For general bodies, investigate branch-and-bound with rigorous quadratic bounds; exact or reconstructed dual certificates from semidefinite/copositive relaxations; and order-variable formulations that retain continuous weights. An ILP for fixed simplex weights does not directly solve the general weighted problem.

**Structural opportunities.** Ask whether low rank of $\Omega$, symmetry, near-product structure, or limited transition complexity controls runtime or certificate size. In dimension four, $\operatorname{rank}\Omega=4$ for every full-dimensional input: the opportunity is to exploit a fixed small rank, not to select unusually low-rank members of this class. Ask for the smallest necessary support of *some* optimizing word. Carathéodory's theorem applied to closure alone does not preserve the quadratic objective, so it cannot simply impose a universal short-word bound. A certificate can establish scalar optimality without classifying every optimal trajectory.

**Why failure is informative.** A rigorously delimited approximation or a hard subfamily can change what the rest of the programme attempts. If coverage dominates arithmetic cost, refactoring the numerical kernel is the wrong intervention. If a special geometry supplies lower certificates directly, exact full capacity computation may be unnecessary for that scientific question.

## T6. Reliable empirical mathematical discovery

**Question and motivation.** Determine which policies turn a fixed resource budget into useful, verified mathematical information, and identify the distributions on which those claims hold. The object of study is a discovery method, not a hypothetical uniform population of convex bodies.

**Progress and boundary.** The supplied historical samples comprise 4,096 generic polytopes and 10,240 products from conditioned generators. Ridge concentration supplied sub-threshold signals, not calibrated discovery of counterexamples. Most old capacities received retrospective checks, but uncertified volumes prevent promoting the whole ratio table. Previous selection on target associations also prevents calling a later scout independent or target-free. [P0]

**Leading approach.** Define an explicit family, normalization, sampling law, novelty criterion, verification standard, and budget. Compare a simple random baseline, a theorem-informed baseline, a geometric selector, and a structural search such as cuts or covers. Count useful verified outputs per total cost, including failed certifications and review. Record unresolved solver outcomes separately from negative examples.

**Competing approaches.** Use prediction when scalar accuracy is the target; use ranking or best-achieved certified value when allocation is the target; use an adversarial matched-family design when a proposed mechanism is the target. Do not score all three by a single held-out correlation. Fresh families, held-out generators, and splits by base body are needed to distinguish transfer from nearly duplicated samples.

**Missing step.** Construct a small trustworthy panel before a large campaign: known forbidden classes, known counterexamples, singular cases, smooth controls, and deliberate feature collisions. Use triangle/quadrilateral/affinely regular hexagon exclusions as controls, not as discovery lanes for $S>1$. A theorem-based upper bound can reject a search region; a heuristic predictor cannot confer that status.

**Why failure is informative.** If enrichment disappears on a new generator, that is a limitation of the selector, not evidence against the mathematical conjecture. If a complicated method fails to beat a geometry-based baseline after certification costs, retire it. If observed tails depend strongly on conditioning, publish the dependence rather than fitting a universal tail law. No observed endpoint stopped by a budget cap should be relabeled a maximum.

## T7. Comparing capacities and embedding information

**Question and motivation.** EHZ measures least action. Symplectic embeddings and other capacities can distinguish bodies with the same least action or ratio. This track broadens the scientific target while requiring a different evidence contract.

**A baseline that removes a false research goal.** Volume preservation immediately gives


$$
\frac{c_G(K)}{c_{\mathrm{EHZ}}(K)}\le S(K)^{-1/2}.
$$


Thus a certified $S>1$ already separates Gromov width from EHZ. Merely proving noncoincidence again is not a new target. A worthwhile embedding project would improve that quantitative bound, prove a lower bound by an actual embedding, or compute a capacity on a specific body. A linear ellipsoid fit is a lower bound on Gromov width, not an exact computation. [L1; deduction §7.7]

**A route using existing scalar capabilities.** Haim-Kislev's nonsmooth Zoll paper gives an implication from equality of EHZ with the spectral-norm diameter $c_\gamma$ to cut additivity. [L6, Theorem 1.16] Consequently, a certified strict cut-additivity defect can rule out that equality. The paper's HKO discussion includes numerical evidence; this packet does not promote that plot to an exact certificate. For one cut it would suffice to establish


$$
c^-_{\mathrm{EHZ}}(K_+)+c^-_{\mathrm{EHZ}}(K_-)>
c^+_{\mathrm{EHZ}}(K).
$$


This target uses capacities of three convex bodies and needs no volume calculation. Import the precise definition and hypotheses of $c_\gamma$ before drawing the conclusion. A numerical size of the cut defect is not automatically the size of the capacity gap.

**Other approaches.** Study higher capacities through supported orbit/index or Floer-theoretic tools, or ask which ratio-one equality families are actually symplectic balls. Neither a sorted action list nor a list of QP ties computes a higher capacity. Start with one example and a concrete imported theorem; do not initiate a general embedding solver as incidental infrastructure.

**Why failure is informative.** A tight interval for another capacity can resolve whether a proposed equality mechanism is plausible. An obstruction to extracting it from scalar data identifies missing dynamical/topological information and gives T3 a concrete target. If no such bridge is available, keep this as a small exploratory allocation.

## 3. Concrete candidate investigations


| Supplier → receiver | Object supplied | What the receiver can do with it | Required boundary |
|---|---|---|---|
| T1 → T5 | Complete normalized triangle list and covering equivalence | Certify a capacity lower bound by verifying containment and area | Completeness and conventions of the template list must be proved. |
| T5 → T1 | A global lower bound on cover area, or an exact smaller placement | Prove a class upper bound, or refute its proposed optimizer | A good placement alone cannot prove the lower bound. |
| T5 → T2 | Complete active scalar family, including singular solutions and rigorous exclusions | Build valid first-variation constraints and detect rank failures | A truncated tied-word list is not the required family. |
| T3 → T2 | Minimizer incidence and face-motion data | Identify perturbations that affect every relevant carrier; interpret response coefficients | Carrier incidence is not automatically the complete derivative formula. |
| T2 → T5 | Certified non-ascent cones, exact rank strata, or continuation breakpoints | Prune searches or cache certificates on a justified parameter region | A pointwise numerical observation does not license regional pruning. |
| T4 → T6 | Matched examples and theorem-based bounds | Test transfer, reject impossible lanes, and distinguish shape effects from orientation effects | Preserve the sampling and feature definitions used to make the comparison. |
| T6 → T4 | Prospectively replicated counterexamples to a specific proposed relation | Choose an inequality to disprove or a missing geometric variable to investigate | Empirical success does not prove the relation. |
| T1 → T3 | Exact equality families and nontrivial plateau examples | Compare topology and multiplicity of carriers along equality loci | Constant scalar action does not imply constant orbit space. |
| T3/T5 → T7 | Physical orbit data or three rigorous capacity intervals for a cut | Apply a supported higher-capacity theorem or certify a cut defect | Action ordering alone is not a higher-capacity computation. |
| T7 → T3 | A capacity equality/non-equality that a proposed orbit model must explain | Select the next dynamical invariant to compute | State the actual imported implication, not just a thematic connection. |

These exchanges are deliverables, not organizational arrows. They also prevent circular validation: an empirical feature trained on QP outputs cannot independently validate the QP theorem, and agreement of two implementations sharing the same incomplete enumeration cannot establish global coverage.

## 5. Coverage, objections, and deliberate omissions


| Perspective | Questions retained | Alternative that guards against an overly narrow programme |
|---|---|---|
| Symplectic geometry | Sharp class bounds, equality, capacity comparisons, higher-dimensional transfer | Ask about actual embeddings or higher capacities only when an appropriate theorem/tool is available. |
| Convex geometry and billiards | Translation covers, asymmetry, support functions, polarity, area inequalities | Use geometric lower certificates instead of treating four-dimensional enumeration as obligatory. |
| Dynamics | Physical minimizing families, singular limits, return maps, cut additivity | Treat a space of minimizers, or its justified quotient, as the object of study. |
| Optimization | Nonsmooth active constraints, facet births, support sparsity, complete global bounds | Compare geometric inequalities, semialgebraic strata, and mixed discrete/continuous formulations. |
| Scientific computation | Exact input semantics, enclosures, global coverage, portable checking, complexity | Evaluate a simpler checker and specialized certificates before expanding a general solver. |
| Empirical discovery | Sampling laws, held-out transfer, rare-event selection, certification cost | Use controlled counterexamples and theorem-informed baselines as well as random populations. |

The coverage is intentionally uneven: planar geometry, local mechanisms, and the existing computational investment justify more near-term attention. It is a decision-oriented map, not a claim to classify all promising research in symplectic geometry.

### 5.2 Objections that should materially change the proposal

**The programme may still overvalue the HKO neighborhood.** Its local obstructions and explicit cuts make it unusually legible, but this can create a selection trap. Keep $P^*$, symmetric classes, and shape-preserving orientation controls as independent comparisons. Reserve some effort for general product bounds rather than assigning every investigation a route back to HKO.

**“More geometry” can become a moving target.** Once a descriptor fails, adding arbitrarily many measurements can eventually reconstruct the body. T4 therefore needs a complexity budget and a specified goal. The full pairing matrix is a baseline for information sufficiency, not evidence that compression is easy or that a statistical explanation has been found.

**A global optimum might require degenerations outside current charts.** Finite facet families and a compact translation model are useful restrictions. They do not show that the global four-dimensional extremizer lies in them. Any claimed global compactness or reduction modulo transformations must use transformations under which the relevant objective is controlled. General affine normalization is not a symmetry of EHZ.

**The symmetric result changes what counts as an open question.** Do not propose proving the quaternionic inequality again. Use its precise action as a control, then examine the weaker-symmetry and low-regularity questions it leaves. Likewise, check the current polygon-factor theorem map before searching for violations in triangle, quadrilateral, or affinely regular hexagon buckets.

**Smooth approximation is inexpensive at the scalar level.** Section 7.8 already gives a quantitative ratio-preservation bound from a multiplicative inclusion. A worthwhile smoothing investigation must add curvature control, effective orbit/action/index correspondence, or a precise stability statement. Repeating continuity alone is a small lemma, not a major track.

**Existing general methods may consume the apparent novelty.** Integer ordering, nonsmooth optimization, planar billiards, and exact convex-hull placement algorithms all have relevant prior art. The project can still contribute a stronger domain of validity, exactness at degeneracies, a new reduction, or a useful benchmark. These are claims to investigate, not priority assertions.

**An unsuccessful certificate may reveal geometry rather than a software defect.** Singular families, exploding contact cases, and non-uniform limits can be the mathematical obstruction. Conversely, an input-boundary or volume failure is not evidence against the conjecture. Report the failure at the layer where it occurred.

### 5.3 Significant omissions and why

- **Nonconvex and arbitrary star-shaped domains:** their geometry and capacity behavior would broaden the task too far and break several central tools. Reopen only for a specific comparison or boundary example.
- **A general Floer/ECH computation platform:** potentially important, but it has substantial prerequisites not supplied by the current scalar solver. T7 begins with imported theorems and one example.
- **High-dimensional random optimization:** retain the analytic transfer benchmark and one coupling question, but defer broad search until the expected result exceeds known product consequences.
- **A full classification of global Reeb dynamics:** minimum-action dynamics is closer to existing proof and computational capabilities. Typical-orbit dynamics is retained as an exploratory branch with a defined return-map prerequisite.
- **A complete formalization of every old proof or full recertification of historical data:** first check the evidence needed by the selected investigations. Neither is a universal prerequisite for research planning.
- **Optimizing only the twenty-facet continuation:** use the unreviewed continuation to select a mathematical transition worth checking. A slightly larger value by itself gets little initial allocation.

There are also lower-priority opportunities inside the retained tracks: minimal facet counts for any global violation (distinct from $m_*$); stability away from the exact polar-product slice; extensions of the restricted zonotope inequality with an explicit capacity bridge; and topological distinctions among ratio-one bodies. Their current frontier must be checked before describing a specific instance as open.

## 6. Initial allocation and evidence that would change it


**First, make four comparisons concrete.** Give the fixed-pentagon lower-bound question an exact unrestricted formulation; give the local-ascent question a complete stated tangent model; give the dynamics question one exact degenerate family; give the explanation question one feature-matched pair or family. These can be pursued largely by mathematical reasoning and source analysis before expensive computation.

**Then choose the computational bottleneck.** A cover certificate, a fixed-word singular-family check, a global word-coverage proof, and a physical trajectory classifier have different requirements. Build or extend only the route selected by the first comparisons. The limited Codex budget should not be spent on a generalized platform before this choice. The generous Pro resource is well suited to developing alternative inequalities, checking counterexamples to reductions, and preparing proof obligations with explicit assumptions.

**Require a scientific discriminator before expanding a campaign.** Examples are a contact stratum that decides an area-bound approach, a rank-defective active family that defeats the current derivative theorem, or a fresh generator on which a selector either transfers or fails. Avoid purchasing more iterations when the next result would have the same interpretation as the last one.

### 6.3 Reallocation triggers

- If the unrestricted cover admits a short analytic lower bound, move effort from general search toward the proof, equality, and stability cases. If a certified smaller cover appears, prioritize understanding its geometry and whether the method transfers to other factors.
- If the 12–16-facet frontier requires an uncontrolled continuum of birth types, reduce claims to a useful restricted theorem and compare its value with $P^*$ or the planar problem. Do not silently replace the neighborhood question by a finite search.
- If low-order features fail on clean matched examples, stop pursuing universal bounds from those features alone. Keep them as selectors only if they pass the separate prospective test.
- If independent certification is inexpensive for the selected objects, reduce T5's share. If proof coverage, rather than arithmetic, is the bottleneck, favor structural reductions and smaller checkers.
- If a strict cut defect is easy to certify, T7 deserves more than its initial share because it produces a different symplectic conclusion. If it requires a new general capacity theory, preserve it as a lead.
- If the local theorem or certificate accounts fail source review, revise the relevant baseline immediately. Do not transfer their claimed status to conclusions built on top of them.

The next research selection should be made from these concrete discriminators. Jörn's relay and reading time need not be consumed by a second round of abstract reorganization.



## Further reading

The [technical agenda](technical-notes.md) retains the full candidate investigations, detailed mathematical deductions, source citations and integration checks. The [source ledger](source-ledger.md) states what Pro read and what remains unchecked. These materials support the proposal; they do not turn project accounts into independent proof verification.
