# Theory Authoring Map

Status: cross-chapter authoring map for the shared theory used from the
preliminaries through the finite algorithms. It is not mathematical source
truth and does not prescribe final paragraph order.

Purpose: map the reader's generating questions to the mathematical
understanding they require, the passage that should own that explanation, and
the sources an author must inspect. This is the entry point for deciding what
the early theory covers before drafting prose.

Overruled by: accepted Jörn/Kai decisions, active theorem statements,
`FACTSHEET.md`, `formal/`, source papers, and later integrated reader review.

## Root Questions

The introduction should leave the reader with four generating questions.

1. Why probe Viterbo's conjecture after the HKO counterexample?
   The reader needs to know what the EHZ systolic ratio measures, why values
   above one matter, and which structural questions remain about the
   counterexample, local optimality, computational discovery, and exact
   families.
2. Why can a nonsmooth polytope answer a symplectic-dynamics question?
   The reader needs a route from smooth Reeb action to generalized polytope
   orbits and then to one capacity-minimizing finite facet word.
3. Why do the finite methods compute the capacity rather than merely generate
   candidates?
   The reader needs to see separately the Haim--Kislev global finite formula
   and the exact flow-graph completeness argument, including their hypotheses
   and different proof routes.
4. What do the later local-maximality, first-order, search, and polygon results
   actually add?
   The reader needs the common objective and coordinate language first; local
   charts, derivatives, certificates, evidence boundaries, and family-specific
   symmetries should arrive with the result that uses them.

These questions generate coverage. Existing preliminary prose does not
generate a requirement merely by already existing.

## Reader-State Progression

### After the preliminaries

The reader should know the common language used by more than one later story:

- the project coordinate order and the conventions for \(J_0\), \(\omega_0\),
  \(\lambda_0\), action, closed characteristics, and contact-normalized Reeb
  orbits, including \(A=T\);
- full-dimensional convex bodies containing the origin, support and gauge
  functions, normalized facet rows \(a_i=n_i/h_i\), polarity, boundedness,
  irredundancy, and the distinction between labelled presentations and
  unlabelled bodies;
- \(c_{\mathrm{EHZ}}\), the four-dimensional systolic ratio, Viterbo's
  threshold, Euclidean four-volume, and why EHZ is a symplectic capacity: its
  monotonicity, conformality, symplectic invariance, and normalization, with
  the translation/scaling consequences used by the thesis;
- the geometric definition of a Lagrangian product and the distinction between
  its \(q/p\) factors and a symplectic product.
- Clarke's dual variational principle for convex bodies in the thesis's
  free-period, uncentered convention, including how ordinary smooth closed
  characteristics and nonsmooth generalized characteristics enter the same
  correspondence.

The reader should not yet be carrying a finite-geometry implementation,
generalized facet words, the polytope reduction of Clarke's dual problem to a
quadratic program, or an algorithm-specific local chart.

### After the generalized-orbit material

The reader should additionally know:

- the generalized Reeb differential inclusion on a polytope and the pure facet
  directions \(R_i=2J_0a_i\);
- why the same normalization still gives action equal to elapsed time;
- active words, positive dwell times, closure, cyclic redundancy, and the
  separate base-point realizability condition;
- how the general nonsmooth Clarke correspondence specializes to the polytope
  inclusion just defined;
- why at least one capacity minimizer can be chosen simple, together with the
  splitting, merging, rescaling, compactness, and piecewise-affine action
  ingredients that prove it.

This is the conceptual finite-reduction theorem. It does not yet tell the
reader how either finite algorithm is organized.

### After the Haim--Kislev material

The reader should additionally know how normalized dwell times give the QP
constraints, how the action identity gives the objective and factor, and why
the global maximum gives the scalar EHZ capacity. The text must distinguish
arbitrary feasible points, KKT candidates, fixed-word maxima, and global
maximizers. Enumeration, pruning, solver correctness, and exact-versus-f64
evidence receive only the bridge required by their later consumers.

### After the flow-graph material

The reader should additionally understand affine facet-pair tubes, return-map
closure, the exact regularity assumptions, why exhaustive simple-word search
is complete, and why this is a project theorem rather than a restatement of
CH2021. CH2021's Type 1/2/3 and smoothing material belongs here as comparison
and motivation, not on the Haim--Kislev dependency spine.

## Explanatory Ownership

| Material | Reader purpose | Proposed owner | Main support route |
|---|---|---|---|
| Coordinates, symplectic form, primitive, action, smooth Reeb normalization | Fix conventions used everywhere and explain the optimized quantity | Preliminaries | Active convex/symplectic notation; standard background; revalidated legacy definitions |
| Normalized halfspaces, polar rows, support/gauge, boundedness, irredundancy | Give a stable mathematical input model shared by both algorithms and later row charts | Preliminaries | Active polytope definitions; `formal/random-polytope-boundedness.tex`; standard convex duality |
| EHZ capacity, compact capacity axioms, systolic ratio, Viterbo threshold | State the thesis target, why it is symplectic, and what comparisons mean | Preliminaries | `02-preliminaries-ehz-capacity.tex`; authoritative capacity sources still need citation placement review |
| Lagrangian product definition and \(q/p\) factor placement | Make the central HKO example, search family, and polygon family intelligible | Preliminaries | Active definition; HKO2024 and later family owners |
| Generalized polytope inclusion and \(R_i\) | Explain what replaces the smooth Reeb field | Generalized orbits | Active definition; HK2017 convention translation |
| Clarke dual principle in free-period, uncentered form | Present the general smooth/nonsmooth convex duality that later makes finite reduction possible | Preliminaries | Keep `02-preliminaries-clarke-dual-action-principle.tex`; `PROJECT_COMPLETION.md` records the settled proof boundary; AAO2014 analytic inputs and HK2017 Lemma 2.1 are the external route |
| Piecewise-affine action/shoelace identity | Make splitting/merging and the later QP normalization check transparent | Generalized orbits near first use; QP cites it | Current preliminary lemma, to be moved intact unless writing shows a better bridge |
| Simple-minimizer theorem and proof | Establish that the capacity search contains a finite pure-facet word | Generalized orbits | Active five-operation proof; HK2017 simple-loop theorem |
| Words, dwell times, closure, base-point recovery | Define the interface passed to the finite methods and the realizability boundary | Generalized orbits | Active word/base-point material; `formal/reeb-orbit-recovery.tex` |
| Haim--Kislev objective, word orientation, factor, global formula | Prove the first finite scalar capacity formula | Haim--Kislev material | HK2017 formula proof; `formal/hk2017-qp-conventions.tex`; active QP text |
| KKT solving, pruning, enumeration, exact/f64 contracts | Explain the project's computational realization, without changing the theorem's input | Haim--Kislev computation/algorithm owner or numerics, according to consumer | `quadratic-program-algorithm-hk2019-content.md`, `formal/ehz-kkt-system.tex`, `formal/capacity-algorithms.tex`, and `crates/symplectic/src/algorithms/hk2017/` |
| CH2021 types, smoothing limits, and source comparison | Situate the second algorithm and delimit what is imported from the literature | Flow-graph material | Published CH2021 result; `formal/flow-graph-ch2021-comparison.tex` |
| Flow tubes, return maps, regularity, genericity, exact correctness | Prove the project-original second finite algorithm | Flow-graph material | `formal/flow-graph-real-algorithm.tex` and implementation-evidence owners |
| Fixed-facet Hausdorff charts and labelled-row perturbations | Define the local spaces used for HKO and first-order work | First local consumer, with short shared reminders | `formal/hko-feasible-section-upper-branches.tex`, `hko-local-maximum-content.md`, `first-order-perturbations-content.md`, and the preliminary polarity vocabulary |
| Systolic symmetry group, quotient slice, infinitesimal action | Explain what local optimality means modulo equality directions | HKO local-maximality material | HKO theorem/certificate owners |
| Capacity and systolic continuity | Close a limiting argument where it is actually used | Rotated-polygon material unless another general use appears | Move the proposition from `02-preliminaries-ehz-capacity.tex` near `09-rotated-regular-polygons-exact-certificate.tex`; verify its standard source |
| Product-volume identity and family-specific rotation invariance | Convert exact capacity branches into the polygon systolic formula | Rotated-polygon material | `rotated-regular-polygons-content.md` and `09-rotated-regular-polygons-pentagon-profile-theorem.tex`; ordinary Fubini proof |
| Vertex enumeration, incidence extraction, boundedness kernels, face tests | State the input/producer contract of algorithms that actually consume this data | Algorithm or data owner; appendix only if a reader-facing audit needs it | `crates/symplectic/src/exact/polytope.rs`, `crates/euclidean-polytopes/src/faces.rs`, `formal/random-polytope-boundedness.tex`, and the consuming flow-graph/HKO/first-order companions |

## Material To Remove From The Shared Preliminary Burden

- The four-hyperplane vertex-enumeration procedure, incidence-only edge and
  two-face rules, and triple-kernel boundedness test are not common conceptual
  prerequisites. Their exact hypotheses should be assessed only after their
  algorithm/data owner is chosen.
- The symplectic-product capacity formula has no active downstream consumer and
  is easily confused with the Lagrangian products central to the thesis. Cut it
  unless the introduction or a retained result creates a concrete use.
- Simplex-volume formulas, product-volume constancy, detailed \(q/p\) label
  order, and fixed-facet chart conditions should move to their first real
  consumer.
- Retain a compact explanation of the capacity axioms because the introduction
  uses EHZ to connect the project to symplectic capacities. Do not expand this
  into a survey of constructions or capacities that the thesis never uses.

## Recovered Legacy Candidates

The source scan found three candidates that the former preliminaries companion
had deliberately left for later disposition:

- Retain a compact smooth existence/minimum-action statement with an
  authoritative citation. The generalized polytope existence statement belongs
  with the Clarke/simple-minimizer route. Whether either needs a separately
  displayed theorem remains an authoring decision, not missing mathematics.
- The longer characteristic-line-field and reparametrization explanation in
  `legacy/basic-definitions.tex` is optional explanatory source. The active
  characteristic/Reeb distinction and \(A=T\) derivation currently cover the
  downstream need; reconsider the longer explanation only if a cold reader
  cannot follow that transition.
- Clarke's symmetry-class uniqueness remark has no identified downstream
  consumer and is deliberately excluded. Reopen it only if the dual-principle
  proof or a later quotient argument creates a concrete use.

## Source And Statement Risks

- Check published AAO2014 numbering and hypotheses for the analytic inputs to
  the Clarke derivation. The current repo has citation metadata but no cached
  source.
- Preserve the settled Clarke proof boundary recorded in
  `PROJECT_COMPLETION.md`: cite existence and the nonsmooth multiplier input;
  derive the free-period uncentered multiplier equation, its coefficient, the
  Reeb reconstruction, and \(I_K=T=A\) locally. The candidate is internally
  reviewed; only integrated reader review remains. Moving the material must not
  reopen that citation-versus-proof decision or compress the derivation.
- Place authoritative citations where the thesis first claims that EHZ is the
  least characteristic action and that the nonsmooth generalized-orbit minimum
  gives the same capacity; the present Rabinowitz citation supports only part
  of that paragraph.
- Keep HK2017's normals/heights, fixed-\([0,1]\), sign, and word order distinct
  from the thesis's normalized dual rows, free period, \(J_0\), and active
  traversal order.
- Keep existential simple-minimizer claims distinct from classifications of all
  minimizers.
- Do not decide the incidence-rule statement until it has an owner and an
  explicit input class. It may disappear from publication prose, become a
  restricted algorithm contract, or require an exact face-dimension test.
- Every global QP maximizer does realize a simple minimum-action generalized
  orbit after zero weights are deleted, but this universal quantifier is a
  consequence of HK2017's constructed dual loop, the scalar capacity equality,
  and Lemma 2.1 rather than an explicitly stated source theorem. Retain the
  explicit loop/action/dual-functional normalization in the formal owner and do
  not attribute the stronger sentence verbatim to HK2017.

## Questions The Thesis May Acknowledge Without Settling

- local optimality of HKO when facet count changes or among all nearby convex
  bodies;
- classification of all capacity minimizers rather than existence of a simple
  one;
- extension of the flow graph beyond its exact regular class and across the
  full CH2021 Type 2/3 boundary;
- a complete first-order theory for arbitrary non-generic polytopes;
- structural conclusions beyond the bounded search designs and data retained
  in the thesis;
- extensions of the exact rotated-polygon formula to other polygon pairs.

## Reopen Conditions

Reconsider this ownership map if authoring reveals a repeated explanation, a
forward dependency, or a later theorem that genuinely needs a supposedly
local fact earlier. For Clarke, repair the current forward reference by
introducing the general nonsmooth characteristic/Hamiltonian-inclusion notion
before the theorem; do not move the general dual principle merely because its
first substantial application is the simple-minimizer proof.
