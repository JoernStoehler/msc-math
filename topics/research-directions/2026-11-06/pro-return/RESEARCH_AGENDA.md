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

**Recommended initial allocation:** T1 25%, T2 20%, T3 15%, T4 15%, T5 15%, T6 5%, T7 5%. These are proposed shares of an initial bounded research portfolio, not timing estimates or authorized jobs. Concentrate the first tranche on four discriminators: an unrestricted covering lower bound, a complete perturbation model, one degenerate trajectory family, and one controlled geometric-information test. Develop certification only far enough to make those results trustworthy. Section 6 specifies the evidence that should change this allocation.

The most consequential additions to the supplied seeds are: nonsmooth Zoll/cut-additivity theory; a complete-versus-compressed pairing-data hierarchy; fixed-dimensional complexity as distinct from general NP-hardness; geometric controls obtained by varying symplectic position while preserving Euclidean shape; and a cheap, theorem-backed capacity lower certificate supplied by translation covers.

## 1. Corrected specification and vocabulary

### 1.1 The actual commission

Produce a **comparative research map** for convex symplectic geometry, initially concentrated in dimension four. It must let a reader distinguish:

1. A **scientific question**: for example, whether a stated class satisfies a sharp inequality.
2. An **approach**: for example, translating that inequality into a planar covering problem.
3. A **shared tool**: for example, an exact quadratic-program verifier.
4. An **investigation**: for example, checking whether a particular family of area inequalities can certify the unrestricted pentagon cover.

Each question needs a domain, quantifiers, a useful output, a current evidence boundary, at least one competing approach, and an informative failure outcome. The proposal should make choices visible without describing every plausible idea as an active project. Its first page should support allocation; technical material should justify the choices.

The agenda is not a thesis-completion plan. It is not a promise to resolve the unrestricted sharp constant. It should not treat another numerical capacity record, a polished exposition, a refactor, or a larger dataset as a sufficient scientific outcome by itself.

### 1.2 Conventions

For a full-dimensional convex body in standard symplectic four-space,

\[
S(K)=\frac{c_{\mathrm{EHZ}}(K)^2}{2\operatorname{vol}_4(K)}.
\]

The ball has ratio one. The **EHZ capacity** is the least action of a generalized closed characteristic. A **capacity carrier** means an actual generalized closed characteristic attaining that least action; a discrete word or stationary weight vector is not automatically a carrier in a specified physical placement.

A **Lagrangian product** \(P\times_L Q\) places the planar factors in the position and momentum planes. A product in two symplectic planes is a different construction. The **support function** is \(h_Q(u)=\max_{x\in Q}\langle x,u\rangle\); the associated billiard length of a closed polygon is \(\sum_i h_Q(q_{i+1}-q_i)\). It may be asymmetric. A **translation cover** for a collection of triangles is a convex body into which each triangle fits by its own translation; the copies may overlap and their orientations are fixed.

A **genuine facet** is a codimension-one boundary face, not merely an inequality in a redundant presentation. A **facet birth** is a perturbation in which an inequality that was inactive or coincident becomes a genuine facet. A **word** is an ordered list of facet labels. An **active word** attains the global capacity-formula maximum at the body under discussion; this is a scalar optimization term, not a claim of unique orbit coding. A **chamber** is a parameter region on which specified combinatorial or optimizing data remain unchanged. A **singular stationary family** is the affine solution set of a rank-deficient fixed-word stationarity system, intersected with the required weight constraints.

**Zoll**, in the smooth setting used here, means that the boundary is foliated by action-minimizing closed characteristics. Nonsmooth replacements must specify their definition: equal higher capacities, cut additivity, and coverage by minimizers are not interchangeable without hypotheses. **Cut additivity** means \(c(K)=c(K_+)+c(K_-)\) for every hyperplane decomposition into convex pieces. It is distinct from the response of \(S\) to removing a small cap.

### 1.3 What counts as progress

This document uses four evidence levels:

- **Project account:** supplied evidence or a repository account read at the pinned baseline; proofs and runs have not thereby been independently checked.
- **External result:** a primary-source statement inspected here. This does not mean its entire proof was audited. Read depth is recorded in `SOURCE_LEDGER.md`.
- **Deduction:** an argument given explicitly in Section 7 from stated hypotheses. No novelty is claimed.
- **Proposal:** a question, method, experiment design, or allocation recommendation.

The commit `f0c4e1862d8922ccb6cf6be10faa56b87bef55b0` was accessible through read-only GitHub resources. Selected accounts were read; there was no local checkout, executable inspection, certificate replay, or exhaustive repository search. In particular, the imported 29 October continuation remains a lead rather than an accepted improvement. The supplied distinction between agent review and external refereeing is preserved.

## 2. The research tracks

### T1. Sharp bounds in specified classes

**Question and motivation.** For a precisely defined class \(\mathcal C\), determine or improve

\[
C_{\mathcal C}=\sup_{K\in\mathcal C}S(K),
\]

then ask separately about attainment, equality, and stability. Different choices of \(\mathcal C\) are separate targets: fixed-factor planar products, all planar products, prescribed symmetry classes, a facet budget, or all convex four-bodies. A theorem about one should not be presented as an approximate resolution of another.

**Progress and boundary.** The project accounts supply sharp results for several polygon rotations, affine factor families, centrally symmetric partners of a fixed pentagon, and restricted asymmetric normal families. Arbitrary asymmetric partners remain open. HKO supplies the external product example. The later twenty-facet project example raises a lower bound for unrestricted four-bodies; it does not automatically raise the best lower bound within planar products. [P0, P3; L1]

The current planar covering paper adds useful scope information: quadrilateral and affinely regular hexagon factors satisfy the ratio-one inequality for arbitrary partners; its general product bound is some \(\kappa<\sqrt2\). It also identifies a half-symmetric product question. These are meaningful boundaries for allocation and dataset controls. [L3, Theorem 1.2, §5.3, Question 4.4]

**Leading approach.** For fixed regular pentagon \(Q\), normalize the partner's capacity to one and minimize its area. The finite triangle-cover equivalence makes this an optimization of the convex hull of translated templates. The ten templates give 18 relative translation variables after anchoring one. The restricted ten-direction problem is already recorded as solved in the repository; it is the unrestricted area lower bound that remains. [P2; L3, Theorem 3.3 and §6.1]

**Competing approaches.** One can seek a geometric area inequality, a complete certificate over hull-contact cases, or a structural compression that moves every cover into a tractable class without increasing area. A published exact translation algorithm exists, but its dependence on the number of templates and real-arithmetic model does not establish practical feasibility here. Parallel sliding provides useful convex subproblems without making the full placement problem convex. [L4, Theorem 3 and Lemma 6]

**Additional branches.** The quaternionic result is now a baseline theorem, not merely a prospective connection: the relevant right \(Q_8\)-action forces \(S\le1\). The next questions concern weaker symmetry, nonsmooth equality, or quantitative departure from this class; the right \(I\)-action has a weaker bound and a conditional sharp result. [L7, Theorems 8–11, Conjecture 17] Near centrally symmetric polar products, the capacity identity reduces the polar slice to Mahler geometry, where planar stability already exists. New work should address transverse departures from polarity or a genuinely different class. [L9, L10]

**Why failure is informative.** A cover smaller than the HKO cover refutes fixed-factor optimality with a geometric certificate. Failure of a proposed fan reduction tells us which contact configurations matter. A sharp weaker inequality or a stability theorem within an honest subclass is valuable without deciding the full product constant. Mere failure of a local placement search proves none of these.

### T2. Local extremality and the complexity of ascent

**Question and motivation.** Why can a body resist many perturbations and still admit a coordinated improvement? Which perturbation classes are sufficient to detect its instability? The answers explain search failure, identify the correct local geometry, and turn numerical directions into structural statements.

**Progress and boundary.** On the supplied project account, HKO is locally maximal among at most eleven genuine facets in an existential Hausdorff neighborhood, and maximal over its full \(GL(4)\)-orbit. Yet specified simultaneous radial cuts give ascent. A different ten-facet hexagon–quadrilateral product \(P^*\) has a non-strict local maximum at ratio one within the established fixed-ten-facet model, with two equality parameters after quotienting symmetries; its general eleven-facet extension is unresolved. These examples should both appear in the proposal. They separate stability, equality moduli, and combinatorial changes. [P0, P3, P6]

For the supplied cut profiles,

\[
D^+\log S=A\sum_i w_i-B\max_i w_i,\qquad 6A<B<7A.
\]

This proves a seven-direction support threshold **within those profiles**. Define instead the unrestricted local complexity threshold

\[
m_* =\min\{m:\text{arbitrarily Hausdorff-close to HKO there is a body with at most }m
\text{ genuine facets and larger }S\}.
\]

The supplied results imply \(12\le m_*\le17\): the upper endpoint uses the seven added cuts, while twelve through sixteen remain unsettled. The support threshold of a chosen cut model is not a lower bound for \(m_*\). Nor is the two-hinge first-variation statement a twelve-facet neighborhood theorem. [P0; deduction in §7.2]

**Leading approach.** Use the existing simultaneous-splitting derivative theorem where its rank condition holds for every maximizing word. Build the complete linear response constraints, quotient translation/dilation/symplectic directions, and ask for the smallest support on which all genuine bottlenecks improve after accounting for volume. This is a structured sparsity problem, potentially an exact finite feasibility problem.

**Competing approaches.** For rank-deficient points, use feasible action sections and stationary families; alternatively stratify the finite parameter space semialgebraically and analyze the discriminant where ranks, signs, or active words change. A fixed-facet capacity value is semialgebraic under algebraic parametrization, without any need to assert globally smooth KKT branches (§7.6). Support-function and Minkowski-sum variations provide another chart; the established Brunn–Minkowski inequality supplies constraints, not an automatic ratio improvement. [L11]

**Missing step.** A neighborhood theorem must cover moving old rows, all relevant limiting new normals, and different rates of vanishing cut depths. Finite directional tests and one-sided derivatives do not control zero-slope directions, higher orders, or non-uniform neighborhoods. For \(P^*\), first decide whether the known equality family accounts for all neutral directions before importing HKO's proof pattern.

**Why failure is informative.** A lower-complexity ascent changes the geometric picture. A complete obstruction for a named birth type is also useful. If a response model fails, the failed rank or uniformity hypothesis identifies a new stratum to study. A continuation that only increases the known record is lower priority unless it exposes a new active-word transition or reusable certificate.

### T3. Minimizing trajectories and their degenerations

**Question and motivation.** Scalar capacity hides multiplicity, placement, topology, and limits of minimizers. Understanding these objects can explain active-set changes, produce geometric obstructions to perturbations, and supply information needed by capacities beyond the first action value.

**Progress and boundary.** The project's QP method handles scalar capacities of degenerate products, while its physical flow-graph theorem requires nonzero relevant pairings, independence of small row sets, and finite-orbit regularity. Products violate these conditions. Tied QP words are not an inventory of physical trajectories, and agreement on scalar values does not validate word-by-word correspondence. [P0, P4, P5]

Rudolf's general billiard characterization provides a nonsmooth scalar bridge and existence of a minimizer with at most three bounces in the planar setting. It carefully distinguishes strong and weak billiards and does not identify all minimizers from a finite optimizing list. [L2, Theorems 1 and 4] Chaidez–Hutchings gives a smoothing correspondence for its specified symplectic polytopes and orbit types; its hypotheses must remain separate from the project's implementation contract. [L5, Definition 1.4, Theorems 1.11–1.12]

**Leading approach.** Start with an exact product and describe actual minimizers via the normal-cone billiard equations, recording physical endpoint sets and zero-length degenerations. Compare that description with QP optima after fixing action, sign, cyclic-order, and time conventions. Then specify a one-parameter perturbation and ask which components persist, merge, or disappear.

**Promising alternative.** Nonsmooth Zoll theory supplies a quotient that forgets internal motions along flat coisotropic faces; a coisotropic tangent space contains its symplectic orthogonal. Haim-Kislev's Proposition 1.6 and Theorem D give a reason to consider this quotient for polytopes and products. [L6] A proposed finite description of that quotient is a more focused target than “extend the flow graph to all degeneracies.” It is still a new obligation: compactness of a quotient does not prove finite representability or convergence for every moving body.

**Further questions.** Distinguish minimum-action dynamics from the dynamics of typical orbits. The HKO literature's special periodic behavior makes perturbations of return maps worth considering, but return-map construction and regularity must precede claims about entropy, ergodicity, or genericity. Orbit topology and index information are a selective extension once supported smoothing models exist, not something the scalar solver already returns. [L1, §3; L5]

**Why failure is informative.** A product can exhibit constant capacity while its minimizing trajectory space changes. A counterexample to a proposed selection theorem would specify the topology or equivalence relation that needs weakening. A complete analysis of one singular family, including its failures of coding, is a worthwhile mathematical result even without a general algorithm.

### T4. Geometric information that determines or bounds capacity

**Question and motivation.** The phrase “geometry explains capacity” conceals several questions. Is a representation sufficient to determine capacity? Does a quantity give a universal bound? Does it describe a dynamical mechanism? These differ from predicting typical values under a chosen generator, which belongs to T6.

**Progress and boundary.** The project has adverse examples and partial inequalities: signed ridge-area circulation constraints, a failed moment inequality, geometries sharing covariance summaries but differing in higher or dual measurements, and restricted zonotope pairing-area inequalities. None alone establishes an EHZ inequality. In particular, different higher descriptors do not prove different capacities; that requires another argument. [P0]

**A rigorous starting hierarchy.** Write a bounded full-dimensional polytope as \(K_A=\{x:Ax\le\mathbf1\}\) with the origin interior and normalized facet rows. Its complete labeled matrix

\[
\Omega=AJA^T,\qquad \Omega_{ij}=\omega(a_i,a_j)
\]

determines the body up to linear symplectic equivalence, provided the rows span the ambient space. Section 7.3 proves this elementary assertion. It is not a new low-dimensional descriptor: it is a complete representation, and changing the chosen origin changes its normalization. It nevertheless gives a precise upper end of an information hierarchy. Compressions such as pairing maxima, spectra, histograms, signed cycles, and ridge summaries can now be assessed for what they lose.

There is also a meaningful analytic baseline. For centrally symmetric bodies, the reciprocal of the largest polar symplectic pairing controls EHZ capacity within a factor of four. [L8, Theorem 1.6] The mathematical opportunity is to identify which additional closure or ordering information sharpens that coarse bound in a chosen class. A list of pairings without its incidences need not retain the cyclic order constraints in the capacity formula.

**Competing approaches.** Construct matched bodies that agree on the proposed descriptor but have certified disjoint capacity intervals; prove inequalities for a restricted class and characterize the equality obstruction; or vary symplectic position while preserving the entire Euclidean shape. For orientation-preserving rotations in four dimensions, the effective choice of orthogonal symplectic structure is a two-dimensional sphere (§7.4). This provides a controlled alternative to comparing unrelated random bodies.

For planar products, use support-function perturbations whose odd part changes while width stays fixed. Remove first harmonics, which merely translate the body, and enforce convexity. Two-bounce lengths cancel odd support data; three-bounce candidates need not. This isolates a mechanism worth testing rather than treating “asymmetry” as an undifferentiated scalar.

**Missing step.** Choose an exact target: sufficiency, a bound with a stated constant, or a perturbation mechanism. Then demand the corresponding evidence. A universal inequality needs a proof connecting the geometric quantity to EHZ, not only a positivity statement about that quantity. A feature collision needs a capacity separation, not only a geometric separation. Symplectic invariance must be preserved or its failure explicitly modeled; arbitrary affine whitening can change capacity.

**Why failure is informative.** An impossibility theorem identifies what every successful descriptor must remember. A feature can fail as an exact representation while succeeding as a predictor. Conversely, a provable bound may be too loose for candidate selection. These outcomes should be reported separately.

### T5. Scalable computation with checkable bounds

**Question and motivation.** The aim is to understand and reduce the mathematical cost of a trustworthy result. Faster arithmetic, a larger candidate list, and a correctness theorem are different advances.

**Progress and boundary.** The project already has outward enclosures, exact fallback, singular-family treatment, and a separate rational product route. The six-facet product reduction is a structural achievement. These facts do not make every candidate stream complete. The inspected production account additionally limits its scalar input contract to at most sixteen facets and a bounded cycle budget; the twenty-facet certificate belongs to a separate route. Certification of the exact dyadic input is not a rounding-error bound for a different algebraic body. [P0, P4]

Haim-Kislev's formula and simple minimizing representation give the finite mathematical starting point. [L12, Theorems 1.1 and 1.5] General capacity computation is NP-hard with dimension allowed to grow; that result does not settle fixed four-dimensional complexity. Linear-ordering and nonsmooth simplex optimization are already represented in the literature. [L13, Theorem 1.1; L14] New work should target a specific structural parameter, a degeneracy class, certificate size, approximation quality, or a measured advantage over those alternatives.

**Leading approach.** Separate an untrusted searcher from a smaller checker. A complete certificate must cover the input geometry, the candidate family, justified exclusions, singular stationary systems, an attainer, and any required volume. A list produced by a floating search is not a completeness certificate. Section 7.1 fixes the directions of all bounds.

**Competing approaches.** For products, use closure-polytope vertices, strong-billiard face systems, or translation-cover certificates. The last can prove a capacity lower bound without computing the exact value. For general bodies, investigate branch-and-bound with rigorous quadratic bounds; exact or reconstructed dual certificates from semidefinite/copositive relaxations; and order-variable formulations that retain continuous weights. An ILP for fixed simplex weights does not directly solve the general weighted problem.

**Structural opportunities.** Ask whether low rank of \(\Omega\), symmetry, near-product structure, or limited transition complexity controls runtime or certificate size. In dimension four, \(\operatorname{rank}\Omega=4\) for every full-dimensional input: the opportunity is to exploit a fixed small rank, not to select unusually low-rank members of this class. Ask for the smallest necessary support of *some* optimizing word. Carathéodory's theorem applied to closure alone does not preserve the quadratic objective, so it cannot simply impose a universal short-word bound. A certificate can establish scalar optimality without classifying every optimal trajectory.

**Why failure is informative.** A rigorously delimited approximation or a hard subfamily can change what the rest of the programme attempts. If coverage dominates arithmetic cost, refactoring the numerical kernel is the wrong intervention. If a special geometry supplies lower certificates directly, exact full capacity computation may be unnecessary for that scientific question.

### T6. Reliable empirical mathematical discovery

**Question and motivation.** Determine which policies turn a fixed resource budget into useful, verified mathematical information, and identify the distributions on which those claims hold. The object of study is a discovery method, not a hypothetical uniform population of convex bodies.

**Progress and boundary.** The supplied historical samples comprise 4,096 generic polytopes and 10,240 products from conditioned generators. Ridge concentration supplied sub-threshold signals, not calibrated discovery of counterexamples. Most old capacities received retrospective checks, but uncertified volumes prevent promoting the whole ratio table. Previous selection on target associations also prevents calling a later scout independent or target-free. [P0]

**Leading approach.** Define an explicit family, normalization, sampling law, novelty criterion, verification standard, and budget. Compare a simple random baseline, a theorem-informed baseline, a geometric selector, and a structural search such as cuts or covers. Count useful verified outputs per total cost, including failed certifications and review. Record unresolved solver outcomes separately from negative examples.

**Competing approaches.** Use prediction when scalar accuracy is the target; use ranking or best-achieved certified value when allocation is the target; use an adversarial matched-family design when a proposed mechanism is the target. Do not score all three by a single held-out correlation. Fresh families, held-out generators, and splits by base body are needed to distinguish transfer from nearly duplicated samples.

**Missing step.** Construct a small trustworthy panel before a large campaign: known forbidden classes, known counterexamples, singular cases, smooth controls, and deliberate feature collisions. Use triangle/quadrilateral/affinely regular hexagon exclusions as controls, not as discovery lanes for \(S>1\). A theorem-based upper bound can reject a search region; a heuristic predictor cannot confer that status.

**Why failure is informative.** If enrichment disappears on a new generator, that is a limitation of the selector, not evidence against the mathematical conjecture. If a complicated method fails to beat a geometry-based baseline after certification costs, retire it. If observed tails depend strongly on conditioning, publish the dependence rather than fitting a universal tail law. No observed endpoint stopped by a budget cap should be relabeled a maximum.

### T7. Comparing capacities and embedding information

**Question and motivation.** EHZ measures least action. Symplectic embeddings and other capacities can distinguish bodies with the same least action or ratio. This track broadens the scientific target while requiring a different evidence contract.

**A baseline that removes a false research goal.** Volume preservation immediately gives

\[
\frac{c_G(K)}{c_{\mathrm{EHZ}}(K)}\le S(K)^{-1/2}.
\]

Thus a certified \(S>1\) already separates Gromov width from EHZ. Merely proving noncoincidence again is not a new target. A worthwhile embedding project would improve that quantitative bound, prove a lower bound by an actual embedding, or compute a capacity on a specific body. A linear ellipsoid fit is a lower bound on Gromov width, not an exact computation. [L1; deduction §7.7]

**A route using existing scalar capabilities.** Haim-Kislev's nonsmooth Zoll paper gives an implication from equality of EHZ with the spectral-norm diameter \(c_\gamma\) to cut additivity. [L6, Theorem 1.16] Consequently, a certified strict cut-additivity defect can rule out that equality. The paper's HKO discussion includes numerical evidence; this packet does not promote that plot to an exact certificate. For one cut it would suffice to establish

\[
c^-_{\mathrm{EHZ}}(K_+)+c^-_{\mathrm{EHZ}}(K_-)>
c^+_{\mathrm{EHZ}}(K).
\]

This target uses capacities of three convex bodies and needs no volume calculation. Import the precise definition and hypotheses of \(c_\gamma\) before drawing the conclusion. A numerical size of the cut defect is not automatically the size of the capacity gap.

**Other approaches.** Study higher capacities through supported orbit/index or Floer-theoretic tools, or ask which ratio-one equality families are actually symplectic balls. Neither a sorted action list nor a list of QP ties computes a higher capacity. Start with one example and a concrete imported theorem; do not initiate a general embedding solver as incidental infrastructure.

**Why failure is informative.** A tight interval for another capacity can resolve whether a proposed equality mechanism is plausible. An obstruction to extracting it from scalar data identifies missing dynamical/topological information and gives T3 a concrete target. If no such bridge is available, keep this as a small exploratory allocation.

## 3. Concrete candidate investigations

These are alternatives and discriminators, not a commissioned execution schedule. An experiment here can be a bounded proof attempt or a source comparison as well as a numerical calculation.

### I1. A global lower certificate for the pentagon cover — T1/T5

**Target:** prove that every placement of the ten fixed normal triangles has hull area at least the HKO cover area, or exhibit a smaller cover. Begin with the exact unrestricted objective and the already solved restricted fan. Try a family of signed polygonal-area inequalities whose nonnegative combination cancels dependence on translations. Every individual area estimate needs a winding/multiplicity justification; an arbitrary loop inside a convex body can wind repeatedly and has no such bound.

**Alternative:** branch over complete hull-contact strata and certify lower bounds on a compact competitive domain. Common-direction sliding can propose good placements but cannot close the global argument. **Informative failure:** identify a concrete contact stratum where the proposed inequality or fan-compression step fails. That is more useful than additional unexplained local minima. No new claim of novelty attaches to the finite formulation. [P2; L3–L4]

### I2. Which perturbations can beat the seven-cut threshold? — T2/T5

**Target:** after checking the existing derivative theorem's rank and coverage assumptions, form the complete constraints for a named enlargement of the cut family. Search for strict ascent with fewer newly genuine facets. Include old-row motion only if the response model includes it; specify which normal cones the new rows may approach.

**Outputs:** an exact direction with a proved positive one-sided ratio derivative, or a dual/separation certificate excluding ascent within the stated tangent cone. Preserve the distinction between a finite direction model and a Hausdorff neighborhood theorem. **Informative failure:** a rank-deficient maximizing word or a zero-slope family that forces second-order analysis. Do not spend the tranche reproving the derivative theorem that the project already has. [P0, P3]

### I3. Transverse stability of the ratio-one equality family — T2/T3

**Target:** at \(P^*\), compare the tangent space of the two-parameter equality family with the neutral directions of the complete local action model after quotienting actual symmetries. Then examine one unresolved facet-birth type.

**Outputs:** a transverse stability statement with an explicit topology and scope, a new equality direction, or a certified destabilizing deformation. A useful negative result identifies the first order at which the ratio changes and the minimizing family responsible. This is an independent comparison case for the HKO mechanism; it need not produce a ratio exceeding the current counterexamples. [P0]

### I4. One complete singular orbit model and its perturbation — T3/T5

**Target:** select a product with a nontrivial minimizing family and solve its strong normal-cone billiard equations for the relevant face patterns. Record endpoint sets, normal-cone choices, and collapsed segments. Construct the map from those physical trajectories to discrete optimizing representations, with its failures of injectivity and surjectivity stated explicitly.

**Alternative:** classify the quotient by coisotropic-face motion first. Follow one precisely defined perturbation and ask whether each limiting component is realized. **Outputs:** a correspondence theorem or a counterexample with the correct weaker statement. A scalar convergence plot is insufficient. This investigation supplies actual orbit data for I2 and I9. [L2, L5–L6]

### I5. An information-loss ladder for geometric descriptors — T4/T6

**Target:** choose one proposed feature family and compare it against the full labeled pairing matrix. Use either fixed Euclidean shape with varied symplectic position, or constant-width planar support perturbations with genuine odd harmonics. Keep scale and volume conventions explicit.

**Outputs:** two certified capacities separated despite identical proposed descriptors, a proof of sufficiency on a stated class, or a quantitative bound on the unresolved variation. Require exact equality or a proved tolerance for the matched feature. Search failure is not evidence of sufficiency. Existing covariance-matched constructions are useful starting inputs only after checking that their capacity labels answer this question. [P0; §7.3–7.4]

### I6. A geometric upper bound from a symmetric envelope — T1/T4/T6

**Target:** use the specified quaternionic action to build \(H_Q(K)=\operatorname{conv}\bigcup_{g\in Q_8}gK\). The theorem and monotonicity imply \(S(K)\le\operatorname{vol}(H_Q(K))/\operatorname{vol}(K)\), as shown in §7.5.

**Outputs:** certified envelope bounds on a small heterogeneous panel, and an assessment of whether they can exclude candidate regions or quantify necessary symmetry breaking. The inequality itself is an elementary corollary, not a research breakthrough. The research question is its sharpness, efficient optimization over admissible frames, or improved class-specific envelopes. **Informative failure:** a uniformly loose envelope sends effort toward pairing/closure information rather than a more elaborate predictor of the same defect. [L7]

### I7. A complete, portable certificate on a difficult small example — T5

**Target:** pick an instance that exercises the scientific bottleneck: a singular stationary family, an algebraic input, or a nontrivial coverage exclusion. Compare two methods with genuinely different global justification, such as product closure enumeration and a covering proof. Require certified input/volume where the conclusion uses \(S\).

**Outputs:** a checker, its mathematical contract, and one accepted certificate; or a precise obstruction to coverage or portability. This is not an instruction to rewrite the production solver. Use the same instance and output guarantee for comparisons. A method that certifies only the optimum value and one that classifies trajectories solve different tasks. [P4; L12–L14]

### I8. A prospective, cost-aware discovery comparison — T6

**Target:** compare a theorem-informed baseline with the best existing geometric selector on a fresh declared generator, and then on a second generator chosen for a different geometry. Freeze the candidate policy before revealing the new target values. Count capacity and volume uncertainty, infeasibility, unsupported cases, and certification failures.

**Outputs:** an explicit claim about the tested generators and budgets, not a claim about all convex bodies. Evaluate verified discoveries, informative counterexamples to descriptors, or certified best-achieved ratio according to the named goal. **Informative failure:** no transfer, no gain after checking costs, or concentration in a known forbidden family. A retrospective association can nominate a policy but cannot serve as its prospective validation. [P0]

### I9. An exact cut-additivity defect — T7/T3/T5

**Target:** recover a promising hyperplane cut of HKO or another body and enclose the three capacities tightly enough to decide strict subadditivity. If the source does not supply exact cut coordinates, selecting an explicit rational or algebraic cut is part of the investigation. Check the imported implication to \(c_\gamma\) independently before using it.

**Outputs:** an exact non-equality result for capacities, or a proved additive cut explained by a gluable action minimizer. A failed numerical candidate can still identify where orbit reconstruction or capacity bounds are too weak. This is attractive because it can produce a new kind of symplectic information without a general embedding computation or certified volume. It does not quantify a spectral-capacity gap from the cut defect alone. [L6, Proposition 1.4 and Theorem 1.16]

### I10. Test a transfer principle beyond its automatic consequences — T1/T2

**Target:** if higher dimensions are selected, first use equal-capacity symplectic 2-products as the exact benchmark. Their normalized systolic ratios multiply; extending a four-dimensional violation by this construction is already known. [L1, proof of Theorem 1.3; L15]

The new discriminator is a specified coupling between the blocks that is not absorbed by the known product formula, with a proved perturbation model. Alternatively, select departures from the polar-product slice and compare capacity loss with known planar Mahler stability. **Outputs:** a coupling obstruction, a new effect beyond transfer, or a decision that the added dimensional/analytic cost buys too little information. Do not launch a broad high-dimensional search merely because tensoring preserves violations.

## 4. Concrete exchanges between tracks

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

### 5.1 Coverage by mathematical perspective

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

**The programme may still overvalue the HKO neighborhood.** Its local obstructions and explicit cuts make it unusually legible, but this can create a selection trap. Keep \(P^*\), symmetric classes, and shape-preserving orientation controls as independent comparisons. Reserve some effort for general product bounds rather than assigning every investigation a route back to HKO.

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

There are also lower-priority opportunities inside the retained tracks: minimal facet counts for any global violation (distinct from \(m_*\)); stability away from the exact polar-product slice; extensions of the restricted zonotope inequality with an explicit capacity bridge; and topological distinctions among ratio-one bodies. Their current frontier must be checked before describing a specific instance as open.

## 6. Initial allocation and evidence that would change it

### 6.1 Proposed shares

| Track | Share | Reason for the initial share | Evidence that earns more effort |
|---|---:|---|---|
| T1 | 25% | A precise unrestricted planar problem, competing analytic/computational routes, and meaningful class theorems | A lower-bound mechanism covering new contact cases, or a smaller exact cover. |
| T2 | 20% | Strong inherited local evidence with a sharply stated gap and a second comparison example | A complete response model with tractable support constraints, or a previously missed rank/birth mechanism. |
| T3 | 15% | Dynamics is scientifically independent and supplies missing meaning for degenerate scalar outputs | A finite useful quotient model or a perturbation correspondence on one nontrivial family. |
| T4 | 15% | Existing adverse examples support tests that can decisively settle proposed explanations | A new certified feature collision, or a sharp bound that predicts a mechanism across classes. |
| T5 | 15% | Trustworthy evidence is essential, but infrastructure should follow a selected scientific bottleneck | Measured coverage cost dominates, or a specialized certificate substantially reduces checking cost. |
| T6 | 5% | Valuable method question, currently limited by labels and weak tail transfer evidence | Prospective improvement over simple baselines on different generators after all costs. |
| T7 | 5% | A distinct scientific target with a promising scalar cut-defect route | A verified imported implication plus a tractable exact candidate or useful embedding construction. |

This is a **recommendation under uncertainty**, not an optimal allocation inferred from measured utilities. No execution time, theorem probability, novelty probability, or machine cost has been fabricated.

### 6.2 Sequence by information gained

**First, make four comparisons concrete.** Give the fixed-pentagon lower-bound question an exact unrestricted formulation; give the local-ascent question a complete stated tangent model; give the dynamics question one exact degenerate family; give the explanation question one feature-matched pair or family. These can be pursued largely by mathematical reasoning and source analysis before expensive computation.

**Then choose the computational bottleneck.** A cover certificate, a fixed-word singular-family check, a global word-coverage proof, and a physical trajectory classifier have different requirements. Build or extend only the route selected by the first comparisons. The limited Codex budget should not be spent on a generalized platform before this choice. The generous Pro resource is well suited to developing alternative inequalities, checking counterexamples to reductions, and preparing proof obligations with explicit assumptions.

**Require a scientific discriminator before expanding a campaign.** Examples are a contact stratum that decides an area-bound approach, a rank-defective active family that defeats the current derivative theorem, or a fresh generator on which a selector either transfers or fails. Avoid purchasing more iterations when the next result would have the same interpretation as the last one.

### 6.3 Reallocation triggers

- If the unrestricted cover admits a short analytic lower bound, move effort from general search toward the proof, equality, and stability cases. If a certified smaller cover appears, prioritize understanding its geometry and whether the method transfers to other factors.
- If the 12–16-facet frontier requires an uncontrolled continuum of birth types, reduce claims to a useful restricted theorem and compare its value with \(P^*\) or the planar problem. Do not silently replace the neighborhood question by a finite search.
- If low-order features fail on clean matched examples, stop pursuing universal bounds from those features alone. Keep them as selectors only if they pass the separate prospective test.
- If independent certification is inexpensive for the selected objects, reduce T5's share. If proof coverage, rather than arithmetic, is the bottleneck, favor structural reductions and smaller checkers.
- If a strict cut defect is easy to certify, T7 deserves more than its initial share because it produces a different symplectic conclusion. If it requires a new general capacity theory, preserve it as a lead.
- If the local theorem or certificate accounts fail source review, revise the relevant baseline immediately. Do not transfer their claimed status to conclusions built on top of them.

The next research selection should be made from these concrete discriminators. Jörn's relay and reading time need not be consumed by a second round of abstract reorganization.

## 7. Explicit deductions and mathematical checks

These arguments make the proposed investigations reviewable. They are deductions from the stated conventions and imported results, not claims of novel theorems or independently verified project results.

### 7.1 Bound directions: QP certificates and translation covers

Let the complete facet-word problem have optimum \(Q_{\max}>0\). If rigorous bounds give

\[
0<q_-\le Q_{\max}\le q_+,\qquad
0<v_-\le\operatorname{vol}_4(K)\le v_+,
\]

then the supplied capacity formula gives

\[
\frac1{2q_+}\le c_{\mathrm{EHZ}}(K)\le\frac1{2q_-},
\qquad
\frac1{8v_+q_+^2}\le S(K)\le\frac1{8v_-q_-^2}.
\]

A feasible word supplies a lower bound on \(Q_{\max}\), hence an **upper** capacity bound. Certifying a ratio improvement requires the opposite capacity bound together with a volume upper bound. Singular stationary families and exact arithmetic do not change this direction. [P0, P4; L12]

For a fixed planar factor \(Q\), define

\[
m_Q=\inf\{\operatorname{area}(P):c_{\mathrm{EHZ}}(P\times_LQ)\ge1\},
\qquad C_Q=\sup_P S(P\times_LQ).
\]

Scaling only the position factor by \(\lambda>0\) scales capacity by \(\lambda\). Indeed, the map \((q,p)\mapsto(\lambda q,p)\) is a dilation by \(\sqrt\lambda\) composed with the symplectic map \((q,p)\mapsto(\sqrt\lambda q,p/\sqrt\lambda)\). Planar area scales by \(\lambda^2\). Normalizing each partner to capacity one therefore gives

\[
C_Q=\frac1{2\operatorname{area}(Q)m_Q}.
\]

Using the complete normalized triangle-cover equivalence [L3], a verified placement of all templates with hull area \(U\) gives

\[
m_Q\le U,\qquad
S(P\times_LQ)\ge\frac1{2\operatorname{area}(Q)U}.
\]

Thus a sufficiently small cover certifies a counterexample to a proposed class bound without exact capacity computation. Conversely, a universal lower bound \(m_Q\ge L\) proves \(C_Q\le1/(2\operatorname{area}(Q)L)\). This universal quantifier is the hard part of proving optimality.

There is an elementary compactness reduction for the finite placement problem. Anchor one nondegenerate template with vertices \(v_1,v_2,v_3\). If a hull containing it has area at most a known competitor area \(M\), every other hull point \(x\) satisfies

\[
|\det(v_2-v_1,x-v_1)|\le2M,
\qquad |\det(v_3-v_1,x-v_1)|\le2M.
\]

These independent strip bounds put all vertices, hence all relative translations, in a bounded set. Hull area is continuous, so the anchored competitive sublevel is compact and the finite problem attains its minimum. This supplies a bounded domain for a global proof; it does not make that proof efficient. It also does not establish compactness of an unrestricted four-dimensional extremal problem. The repository records this placement reduction already. [P2]

### 7.2 What the seven-cut formula proves

For nonnegative, nonzero cut speeds, homogeneity allows normalization \(\max_iw_i=1\). If precisely \(k\) speeds are nonzero, then

\[
D^+\log S=A\sum_iw_i-B\le Ak-B.
\]

The given inequalities imply \(A>0\). For \(k\le6\), the derivative is strictly negative. For seven unit speeds it is \(7A-B>0\). This proves the restricted support threshold.

The project account also gives a genuine-facet neighborhood theorem through eleven facets and an arbitrarily small seven-cut ascent retaining the original ten facets. Together they imply \(12\le m_*\le17\). No step rules out other twelve- through sixteen-facet perturbations. A zero first derivative in a different family would require further analysis rather than establishing local optimality. [P0, P3, P6]

### 7.3 Complete labeled pairings determine a spanning configuration

Use the convention \(\omega(u,v)=u^TJv\), with the standard symplectic matrix \(J\), so \(J^T=-J\) and \(J^2=-I\). Let \(A,B\) be \(m\times2n\) matrices of full column rank, with column normal vectors \(a_i,b_i\) as their transposed rows. Suppose

\[
AJA^T=BJB^T=\Omega.
\]

Because \(A\) is injective as a map from \(\mathbb R^{2n}\),

\[
\ker\Omega=\ker A^T=\ker B^T.
\]

Consequently the rule

\[
T\Big(\sum_i u_i a_i\Big)=\sum_i u_i b_i
\]

is well defined and invertible. Equality of the pairing matrices implies, for all coefficient vectors \(u,v\),

\[
\omega\!\left(T\sum_i u_i a_i,T\sum_j v_j a_j\right)
=u^T\Omega v
=\omega\!\left(\sum_i u_i a_i,\sum_j v_j a_j\right).
\]

Thus \(T\) is symplectic. Since \(B=AT^T\), the normalized bodies satisfy

\[
K_B=\{x:Bx\le\mathbf1\}=T^{-T}K_A.
\]

The inverse transpose of a symplectic map is symplectic. Hence the labeled matrix determines the normalized body up to linear symplectic equivalence. It also recovers the closure relations through \(\Omega\beta=0\). Scale, support-height normalization, labeling, and the origin choice matter. Neither the spectrum nor the multiset of matrix entries is this complete datum; nothing here classifies nonlinear symplectic equivalence.

### 7.4 Varying symplectic position while fixing Euclidean shape

Fix a body and rotate it by \(O\in SO(4)\). The rotated normals have pairings

\[
\omega(Oa_i,Oa_j)=a_i^T(O^TJO)a_j.
\]

Orthogonal symplectic rotations form \(U(2)\), so rotations that differ by such a left factor have the same effective symplectic structure. The quotient is a two-sphere: equivalently, the orthogonal complex structures compatible with the chosen orientation can be written

\[
J(u)=u_1J_1+u_2J_2+u_3J_3,\qquad u\in S^2,
\]

where an orthogonal quaternionic triple satisfies \(J_r^2=-I\) and \(J_rJ_s=-J_sJ_r\) for \(r\ne s\). These relations give \(J(u)^2=-I\). The associated pairing matrix is linear in \(u\).

This is a controlled family with unchanged volume, distances, and Euclidean congruence class. Capacity need not remain fixed under these rotations. Descriptors that are themselves orientation-sensitive must still be matched or tracked explicitly. The sphere may be further identified by symmetries of the chosen body; this does not invalidate it as a parameter space for the experiment.

### 7.5 A quaternionic envelope gives a coarse upper bound

Work in the precise quaternionic coordinates and symplectic convention of [L7]. Set

\[
H_Q(K)=\operatorname{conv}\bigcup_{g\in Q_8}gK,
\]

where the group acts by the specified right multiplication. This hull contains \(K\), is convex and \(Q_8\)-invariant, and therefore satisfies \(c_{\mathrm{EHZ}}(H_Q)^2\le2\operatorname{vol}(H_Q)\). Monotonicity gives

\[
S(K)\le\frac{\operatorname{vol}(H_Q(K))}{\operatorname{vol}(K)}.
\]

One may apply the same construction after a symplectic change of frame or a translation and take the best proved upper bound. Finding the globally best frame is a separate optimization problem. At a fixed frame, the corollary is immediate; its usefulness depends on the envelope volume excess. In particular, it may bound the possible size of a violation while failing to exclude any violation at all. It is not assumed to be sharp.

### 7.6 Semialgebraic structure does not require smooth stationary branches

Consider a fixed finite number of normalized facet rows depending polynomially or rationally on finitely many real parameters. Restrict to a semialgebraic parameter domain where denominators are nonzero and the body is bounded, full dimensional, and contains the origin in its interior. For every ordering, the weight simplex with closure constraints is a compact semialgebraic set, and its action objective is polynomial in the weights and rational in the parameters.

The graph of the global maximum can be expressed by existence of an attainer and the assertion that every feasible point in every ordering has no larger objective. Quantifier elimination over the reals therefore makes this graph semialgebraic. Inverting its positive value gives semialgebraic capacity. Volume is piecewise rational after partitioning by vertex and face incidences and choosing finite triangulations, so the ratio is semialgebraic too.

This supplies a legitimate structural alternative at singular KKT loci. It does not imply manageable quantifier elimination, smoothness across strata, completeness of a selected branch list, or a uniform neighborhood theorem when the number and limiting positions of new facets are not controlled. The deduction uses the complete finite formula [L12], not a truncated numerical model.

### 7.7 The automatic Gromov-width separation

In four dimensions, a ball of capacity \(a\) has volume \(a^2/2\). Every symplectic embedding of that ball into \(K\) preserves volume. Taking the supremum over such embeddings gives

\[
c_G(K)\le\sqrt{2\operatorname{vol}(K)},\qquad
\frac{c_G(K)}{c_{\mathrm{EHZ}}(K)}\le\frac1{\sqrt{S(K)}}.
\]

Consequently \(S(K)>1\) already implies strict separation. This argument supplies an upper bound for an embedding capacity, not an embedding or its exact value. It explains why a proposed separation project needs a stronger target. The same elementary observation is consistent with the HKO counterexample discussion. [L1]

### 7.8 A scalar smoothing estimate

Suppose a smooth approximation, or any comparison body, satisfies

\[
(1-\delta)K\subset K_\delta\subset(1+\delta)K,
\qquad 0<\delta<1,
\]

about a common interior origin. Monotonicity and quadratic homogeneity of capacity, and fourth-power scaling of volume, imply

\[
\left(\frac{1-\delta}{1+\delta}\right)^4S(K)
\le S(K_\delta)\le
\left(\frac{1+\delta}{1-\delta}\right)^4S(K).
\]

For \(S(K)>1\), the strict violation survives whenever

\[
\delta<\frac{S(K)^{1/4}-1}{S(K)^{1/4}+1}.
\]

The estimate is conditional on the stated inclusions. It gives no curvature bound, smooth construction with prescribed derivatives, or correspondence of individual trajectories. Those are possible research targets; the scalar estimate alone is not one.

## Sources and integration

`SOURCE_LEDGER.md` gives the precise primary sources, versions, selected passages read, and pinned project accounts behind the labels used here. `INTEGRATION_CHECKS.md` identifies the claims whose scope is most likely to be lost during integration. These references preserve the distinction between planning, explicit deductions, imported theorem statements, and unverified project proof/certificate accounts.
