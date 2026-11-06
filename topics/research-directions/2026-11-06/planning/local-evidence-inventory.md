# Evidence inventory for a track-based research proposal

This is a bounded reconstruction from the current research guide and topic accounts, especially `topics/empirical-studies/{discovery,methods,landscape}.md`, `topics/products/README.md`, `topics/variation-and-local-maxima/README.md`, `topics/flow-graphs/README.md`, and `topics/capacity-and-qp/convex-systolic-optimization/README.md`. It is a map for proposal structure and selection, not a novelty survey or assignment to execute every open direction. The previous report had opportunities but did not explain the programme's tracks, motivations, partial progress, shared methods, or why a reader should care about each track.

## Reader's map: what the programme is trying to learn

The broad mathematical motivation is to understand how symplectic capacity and volume interact on convex bodies, after the Haim–Kislev–Ostrover pentagon-product counterexample showed that the proposed bound `c_EHZ(K)^2 <= 2 vol(K)` is false. That single counterexample opens several distinct questions: what makes its ratio large; which deformations can improve it; how shape, symplectic placement and closed characteristics contribute; and whether other geometric inequalities explain or constrain these phenomena. A higher ratio is only one possible output. Structural theorems, classification of equality, reliable capacity methods, and explanations of sampled geometry are separate scientific outcomes.

The report should first give this map in ordinary language. Use “track” for a sustained question and motivation; “approach” for a proof, experiment or algorithm used within it; “exchange” for a concrete result another track can use. Avoid “deformations” as a track label: it conflates local maxima, ascent, shape effects and explanatory geometry.

## Candidate tracks and their actual state

### A. Stability and local structure near distinguished bodies

**Why ask:** The HKO example is a counterexample, but its local rigidity may explain why it is a useful benchmark and what kinds of perturbations can escape it. Local structure also tells us which search directions to try and which finite-dimensional certificates have mathematical meaning.

**Questions:** How does the ratio change under unrestricted perturbations of HKO? How do conclusions change as the number of facets increases? What are other local maximizers and their equality directions? What is the first facet count or mechanism that permits ascent?

**Progress:** A theorem gives local maximality among all convex four-polytopes with at most eleven genuine facets near HKO. A specified family of seven radial cuts yields nearby 17-facet ascent; six positive speeds have negative first derivative only within that specified profile cone. A separate 20-facet body has exact ratio about 1.051914. Up to two characteristic-invariant affine hinges retaining original rows have nonpositive first variation, which does not decide zero-slope higher orders or general 12-facet bodies. A second, inequivalent ten-facet body locally maximizes at ratio one and has two equality parameters.

**Candidate next investigations:** (i) Continue the exact cut family from seven to eight labels as a bounded discriminator of whether the derivative mechanism generalizes. (ii) Extend local analysis from HKO F≤11 to the 12–16 gap, with a new-normal parametrization and explicit branch/scale controls. (iii) Characterize local maxima around the non-HKO body and determine whether its equality parameters persist under larger classes. These are distinct investigations; neither the old finite continuation nor sampled gradients certify a neighborhood theorem.

**Evidence:** [`variation-and-local-maxima`](../../../variation-and-local-maxima/README.md), [`HKO cut-family handoff`](../../../variation-and-local-maxima/hko-cut-family/README.md), [`research-night synthesis`](../../../empirical-studies/discovery.md#october-continuation-mechanisms-restricted-theorems-and-facet-birth), [`r6 optimization`](../../../capacity-and-qp/convex-systolic-optimization/README.md).

### B. Capacity computation and proof-producing methods

**Why ask:** Capacity is defined through closed characteristics, while polytopes are nonsmooth and their minimizing orbit may use several facets. The Haim–Kislev finite-word reduction makes exact computation possible, but global coverage and trustworthy certificates remain difficult. Better methods enable proofs in other tracks and help distinguish real geometric effects from solver artifacts.

**Questions:** Which finite orbit descriptions can be certified complete? Can algorithms handle degeneracy and products robustly? Can the finite QP expose a useful structural theorem rather than only a scalar answer?

**Progress:** HK's formula reduces 4D polytope capacity to a maximum over feasible words and weights. Product support has a ≤3-facets-per-factor reduction. The Rust route has outward-certified numerical enclosures and exact fallbacks under recorded arithmetic conditions. A distinct flow-graph route proves correctness for a specified regular nondegenerate class; products with zero-pairing transitions fall outside its primitive-tube model. Positive feasible Q witnesses only upper-bound capacity; the global maximum needs complete coverage.

**Candidate next investigations:** exact treatment of singular/stationary families for broader geometries; connect the QP word changes to a concise geometric classification; extend flow-graph handling across degenerate product transitions with a precise typed rejection/completion boundary. First compare which gaps block the stability and product questions before choosing a major algorithm extension.

**Evidence:** [`capacity and QP`](../../../capacity-and-qp/README.md), [`flow graphs`](../../../flow-graphs/README.md), [`research guide`](../../../../docs/research-guide.md).

### C. Product geometry and partner inequalities

**Why ask:** Products turn a high-dimensional capacity problem into planar factor data, making them a controlled setting to test whether geometry or factor shape predicts capacity. They can yield exact counterexamples and partner inequalities with interpretable equality cases.

**Questions:** For a fixed planar factor, which partner maximizes the product's systolic ratio? What does relative orientation do? Can containment or width arguments prove sharp bounds? Which results survive nonsymmetric or independently affine changes?

**Progress:** Sparse product QP reduction, the rotated regular pentagon profile, arbitrary GL(4) HKO-orbit rigidity, exact 5×5 and 5×7 affine containment results, all-odd angular width results, and restricted sharp theorems for fixed-factor partner families. A mixed competitor disproves the inference that local translative containment alone gives global capacity minimization. General GL(2) partner cone positivity and arbitrary partners remain open; a broad difference-body formula fails already for two triangles.

**Candidate next investigations:** prove/disprove the reduced seven-variable general-matrix positivity for a fixed pentagon; identify explicit partners outside the restricted normal families and determine whether the bound changes; explain why endpoint interpolation does not give interior sharpness absent one common attaining coefficient. Keep the motivation as understanding partner-dependent capacity and equality, rather than simply maximizing a table.

**Evidence:** [`products`](../../../products/README.md), [`empirical discovery`](../../../empirical-studies/discovery.md#product-results-have-their-own-scientific-owners), [`research guide`](../../../../docs/research-guide.md).

### D. Shape, symplectic placement and geometric explanation

**Why ask:** Euclidean appearance is not the whole symplectic geometry. A fixed Euclidean body can change capacity when its alignment with the symplectic form changes; conversely, scalar features may fail to see important dual geometry. This track seeks explanations and inequalities connecting familiar geometric descriptors to capacity, without assuming they determine it.

**Questions:** Which geometric structures constrain ridge-area concentration? Which measurements separate bodies with the same covariance? What role do shape versus symplectic orientation play? Can descriptor inequalities lead to capacity bounds?

**Progress:** Ridge symplectic areas form a signed graph circulation, yielding sharp lower bounds on effective ridge counts in several classes. A proposed disk-moment inequality was exactly disproved by pure-third-harmonic perturbations. Matched vertex covariance/Williamson spectrum examples still differ in dual-field descriptors. A fixed-shape orientation scan improved one sampled product from 0.862586 to 0.878308; sparse noncompact linear scans found no improvement and stopped the naive random-transform route. These are geometric findings, not capacity laws. The zonotope bound `vol4(Z) <= 4 Phi(Z)^2` holds in specified cases, while the general inequality remains open; Phi is not EHZ capacity.

**Candidate next investigations:** use the exact ridge circulation bound to identify geometries attaining or approaching the sharp infimum and ask whether any capacity consequence exists; formulate a target-free geometric predictor whose test set is fixed before capacity evaluation; prove or find a counterexample to the zonotope inequality in the lowest unresolved generator rank. Avoid returning to scalar moments already disproved or repeating the naive random-transform scan.

**Evidence:** [`geometric discovery`](../../../empirical-studies/discovery.md), [`polytope geometry`](../../../polytope-geometry/README.md).

### E. Landscape, rare events and data-driven discovery

**Why ask:** The retained random generators did not recover a counterexample; rarity outside those sampling laws is unknown. Random tables can expose patterns, but failed searches and feature associations are easy to overread. The track studies how to find useful candidates efficiently and how to separate reproducible discovery from post hoc correlation.

**Questions:** Which prespecified geometric rule enriches candidate search? Can local ascent find witnesses above one? How should finite samples be used to identify new mathematics rather than make unsupported universal claims?

**Progress:** Across 14,336 retained random bodies/products, none has ratio above one (maximum 0.8625858958). Ridge symplectic-area features correlate with the upper tail and improve bounded subthreshold selection; the rule lacks a calibrated above-one discovery claim. All 12 retained F10 ascent traces improved through eight steps and every endpoint still had a finite improving move, so the cap-eight endpoints were not optimized. Historical variable-facet continuation often improved values still below one; raw rows are gone. Fixed-shape orientation found one above-population-sample value; sparse noncompact pilot found no improvement.

**Candidate next investigations:** (i) use a witness-guided continuation with certified scalar checks, comparing it against identical-body F10 and fresh-F11 controls; (ii) test a rule frozen before target computation on new, independently generated bodies, with source lineage and target evaluator verified; (iii) choose new geometries from a theoretical mechanism (circulation, product partner, or cut witness) instead of blind random search. The finite absence of >1 rows motivates the search problem; it does not suggest absence from the class.

**Evidence:** [`methods`](../../../empirical-studies/methods.md), [`landscape`](../../../empirical-studies/landscape.md), [`discovery`](../../../empirical-studies/discovery.md).

### F. Closed-characteristic dynamics and degeneracy

**Why ask:** Capacity records only the least action. It does not explain what an orbit does, how orbit families change with body shape, or what happens when regularity fails. This is an independent scientific objective even if it yields no larger ratio.

**Questions:** How do minimizing orbit words and their action change near a degenerate or symmetry-rich body? Can flow-graph and QP descriptions be related beyond generic regular cases? Which continuation invariants survive products?

**Progress:** HK simple-minimizer theory provides finite word representatives; the flow-graph real algorithm is conditionally correct for a regular class and project implementations explicitly reject unsupported singular positives. Current source shows product degeneracies outside the flow-graph primitive tube model. This is a proved boundary and unresolved extension, not a general dynamics classification.

**Candidate next investigations:** compare word transitions and action along a named product path where a symplectic pairing tends to zero; produce a small exact family tracing when the flow graph rejects and when QP still returns the scalar capacity. Avoid asserting continuity of capacity identifies the minimizing words.

**Evidence:** [`flow graphs`](../../../flow-graphs/README.md), [`capacity and QP`](../../../capacity-and-qp/README.md), [`products`](../../../products/README.md).

## Explicit exchange network

A proposal should show these pathways, because the methods and geometries are connected but the goals are not identical:

1. **Track A → B:** A proposed deformation or local-maximum proof needs capacity branches and exact finite words. B returns a certified action/word family with the exact completeness boundary; that can validate the derivative or show a branch switch.
2. **Track B → C/F:** Product capacities and orbit words provide controlled data for partner inequalities (C) and degeneracy/trajectory questions (F). The transferred object is a named factor pair/path, its exact capacity word(s), and hypotheses on positivity/pairings—not just one scalar table entry.
3. **Track C → E:** A product theorem identifies parameter regions and equality/near-equality candidates; E can freeze those regions as candidate generators and compare them against matched random/product controls. Finite validation tests a prediction, not the theorem.
4. **Track D → E:** A geometric invariant or descriptor fiber defines a preregistered feature or stratification for E. E returns out-of-sample enrichment and counterexamples, with target leakage and source lineage controlled.
5. **Track E → A/C/D:** A newly found high-ratio body returns exact coordinates, a reproducible certified ratio and active characteristic(s). The receiving track asks which geometric mechanism explains it; the sample itself is not a proof of local/global optimality.
6. **Track F ↔ B:** Compare QP word changes with flow-graph tube paths along one exact degenerating family. B supplies capacity/equality checks; F classifies trajectory transitions and identifies where the regular theorem stops applying.

## Terms to define at first use

- **Systolic ratio:** here `c_EHZ(K)^2/(2 vol_4(K))`, a capacity-to-volume comparison; it is not synonymous with capacity.
- **Facet count F:** the number of genuine supporting three-dimensional facets of a 4D polytope. “F≤11” concerns a neighborhood class with at most eleven such facets.
- **Characteristic / minimizing orbit:** a closed trajectory tangent to the kernel of the boundary symplectic form; “least action” gives EHZ capacity on smooth strictly convex bodies and a generalized version for polytopes.
- **Lagrangian product:** a product of planar factors in the q and p coordinate planes; this is different from factors in two symplectic planes.
- **Partner:** the second planar factor when the first is fixed.
- **Orientation/placement:** the relation between a fixed Euclidean body's embedding and the symplectic form; changing it can change capacity without changing Euclidean shape or volume.
- **Cut / facet birth:** a deformation that introduces a new supporting inequality/facet. Distinguish a specified finite cut family from all nearby polytopes with that facet count.
- **Upper bound on capacity:** a feasible dual witness can imply `c_EHZ <= bound`; it need not certify equality or a ratio improvement.
- **Local maximality:** no nearby body in a stated class has larger ratio. A finite ascent/descent sample does not prove this.
- **Effective ridge count / circulation:** a measure of how symplectic two-face area is distributed; structural geometry, not itself a capacity measurement.

## Coverage and limits to state

This inventory recovers mathematical and empirical tracks represented in the consulted current topic accounts. It does not establish that the list is complete across all papers, archived sessions, or the wider symplectic literature. A final proposal should state that bounded scope and invite omitted tracks, then distinguish near-term experiments from theorem programmes and shared enabling methods. The literature search and historical archives were not rerun here.
