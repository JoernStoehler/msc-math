# Moderate parameter certificate: proof obligations and coverage

29October2026. Parameter t=3/10. **Work in progress: no improved ratio is asserted until all finite checks and independent audit pass.** Discovery arithmetic is not a proof premise.

## Exact body and conventions

Let tau be the root in(0,1) of tau^4−10tau²+5=0, s=(5−tau²)/2=√5, and let the ten original HKO rowsN0 be exactly the matrix in `exact_certify.py` (first row(1,tau,0,0)). DefineD by that file and N=[N0;N0+(3/10)D]. This is the same family K_t=H∩L_t⁻¹H as the infinitesimal theorem. All fields and sign decisions use SageQ(tau), with its explicit exact embedding intoAA.

The objective convention is Q=sum_{i<j}β_iβ_j a_iᵀJ a_j, J=[[0,I],[-I,0]]. The characteristic velocity uses−J. Capacity is1/(2Qmax); ratio is1/(8VQmax²).

## Geometry and the directed graph

`exact_geometry.py` checks every one of20choose4 normal subsets. For each invertible matrixB it solvesv=B⁻¹1 and tests all20 inequalities exactly. The body is full-dimensional and bounded because it contains a small ball and is contained inH. Thus every vertex occurs in this list. There are75 distinct active index sets, each consisting of exactlyfour facets; hence all vertices are simple. In particular every facet intersection is nonempty iff some listed vertex lies in it.

The graph retains i→j iff two facets meet and a_iᵀJ a_j≥0. This is a necessary condition for a physical transition: if the characteristic arrives along velocity−Ja_i at a point of facetj from insideK, the derivative a_j·(−Ja_i)=a_iᵀJ a_j is nonnegative. Haim–Kislev's Theorem1.5 (as stated in `thesis/chapters/03-generalized-reeb-orbits-polytopes.tex`, theorem `thm:generalized-reeb-simple-minimizer`) supplies a capacity-minimizing polygonal characteristic whose nonzero normal segments have distinct facet labels. Consequently at least one global maximizing word is a simple directed cycle of this graph. This step relies on the published simple-characteristic capacity theorem, not on claiming that every graph edge is dynamically realizable.

The exact graph has120 edges and equals the discovery graph byte-for-entry. Numerical vertex classification therefore does not supply acceptance evidence.

## Symmetry quotient

The label set consists of two copies of the original ten rows. On each copy write a label as(f,i), f∈{0,1},i∈Z/5. Simultaneously shiftingi byk and optionally swappingf give ten symmetries. Reflectingi→4−i gives an antisymplectic transformation; combine it with reversing the word. Canonicalize by these twenty actions and cyclic rotation to the least label.

The exact checker constructs the corresponding linear transformations from four independent original rows, verifies their action on **all twenty** rows, and verifies that they preserve the symplectic pairing matrix, or negate it for reflection. Thus the constraint systems, objective values, inertia and feasibility are preserved by the word quotient. Cyclic rotation preservesQ on closure weights and preserves its tangent quadratic form on kerC.

## Certified prefix pruning and complete cycle coverage

For a wordw of lengthm, letC have the four normal-coordinate rows and a last row of ones. LetH be symmetric with H_ij=ω(a_i,a_j) fori<j, soQ=βᵀHβ/2. If there isz∈kerC withzᵀHz>0, no strictly positive feasibleβ on this word can be a local maximum. Extendz by zero to any larger word containingw in cyclic order. It remains in the larger constraint kernel and has the same positive quadratic value. Thus every cyclic extension is also excluded as an interior maximum.

`exact_certify.py` reconstructsC,H for each of77,927 canonical excluded prefixes, computes an exact nullspace basisZ, and proves thatZHZᵀ has a positive direction. Its exact congruence algorithm is elementary: a positive diagonal already supplies a direction; otherwise eliminate a strictly negative diagonal by a Schur complement. If all diagonals are zero but an off-diagonal is nonzero, the corresponding2×2 principal form has a positive direction. If the matrix reduces to zero or negative pivots only, it has no positive direction. All advertised exclusions must pass; an ambiguous/failed predicate is not accepted.

`coverage_replay.cpp` is the acceptance traversal. It uses only the exact verified graph and the certified exclusion list. It performs canonical simple-cycle DFS, starting at each possible least label and visiting only larger unused labels thereafter. At each prefix it stops only when its canonical word belongs to the certified exclusion list. Every other closed cycle must occur in the separately checked leaf list. No floating-point threshold or optimizer enters this traversal. The replay reports43,110 surviving cycles, maximum length12, represented by2326 canonical leaf words, and **zero unknown cycles**. No20! stream is materialized.

A maximizing word cannot be pruned: its positive weights give a global maximum on its active support, and every certified prefix obstruction would contradict the necessary tangent-curvature condition. Therefore a scalar capacity maximum is represented among the checked leaves.

## Complete leaf optimization, including singular families

For each leaf, exact rank first decides whetherCβ=e is consistent. If not, the word is infeasible. Otherwise retain independent rows ofC and corresponding entries ofe; this changes no constraint solution. Let this full-row-rank reduced matrix still be calledC. An interior maximum satisfies

Hβ+Cᵀλ=0,  Cβ=e.

Every positive active support has some KKT multiplier because these equality constraints have independent rows. We do not require a negative-definite Hessian, nor do we claim every retained stationary point is a maximum.

If the augmented KKT matrix is invertible, there is a single stationary point. A negative weight excludes an interior maximum. A nonnegative solution is a feasible HK word, so its exactQ is a valid candidate; saddles cause no problem because all feasibleQ are bounded byQmax.

If the KKT matrix is singular, exact rank decides consistency. For a consistent affine solution family(β0,λ0)+kerKKT, the objective has one constant value: each kernel element(z,η) satisfiesCz=0, Hz+Cᵀη=0, hence e·η=0 (multiply byβ0 and use the first stationarity equation), and Q=−e·λ/2. The checker verifies these kernel identities exactly.

A particular stationary solution having negative weights does **not** imply the family is infeasible. The final checker is `exact_refined_leaves.py`. Once the separately checked positive witness establishes the targetQ=(205/1019)tau, singular families with constantQ≤target cannot raise the maximum and need no positivity test. For any remaining family, writeβ=β0+Zy, where the columns ofZ are the β components of a KKT-nullspace basis. The projection from the KKT kernel toβ is injective: ifz=0, thenCᵀη=0, and full row rank ofC forcesη=0. ThereforeZ has full column rank.

The feasible parameter set {y:β0+Zy≥0} is bounded, becauseCβ=e includes normalizationΣβ=1, soβ lies in the compact simplex, and the injective affine map has bounded inverse on its image. If nonempty, this bounded polyhedron has a vertex, where d=dim(y) linearly independent inequalitiesβ_i=0 are active. This remains true if the polyhedron is lower-dimensional. The checker enumerates everyd-element index set, solves its exact full-rank system, and tests all resulting weights. Finding one nonnegative vector establishes feasibility; exhausting all bases establishes emptiness. Thus every potentially larger singular objective is exactly resolved, with no assumption on the signs of a particular solve_right solution.

An earlier Fourier–Motzkin implementation is mathematically sound but was terminated because intermediate inequalities used excessive memory. It has no accepted result and is not part of the final execution route.

A cheap exact rejection precedes this elimination: H/(28/25)⊂K_(3/10), since each moving row has H-support1+(2/5)t=28/25. Capacity monotonicity yieldsQmax≤Q_H(28/25)². Any singular stationary family with constantQ above this bound has no feasible nonnegative member. This implication uses a proved global inclusion and the HK formula, not the sign of one particular solution.

The diagnostic `exact-all-leaves-summary.json` intentionally omitted singular-family feasibility and has a loose upper bound≈.4474. It does **not** certify improvement. Only `exact-refined-leaves-summary.json`, after exact positivity resolution, can support the desired scalar bound.

## Volume

For a simple bounded polytope{Nx≤1}, a generic linear functionell gives Lawrence's exact formula

V=(1/4!) Σ_vertices v (ell·v)^4 / (|detN_I| Π_j(ell N_I⁻¹)_j),

whereI is its four active rows. The checker verifies every denominator is nonzero for each of two generic vectorsell=(1,2,4,8),(1,3,9,27), and obtains the same exact result

V=300748771/52178240−(99664289/260891200)tau²≈5.562221570778812.

Primary source: Jim Lawrence, *Polytope Volume Computation*, NISTIR89-4123, §2 theorem, PDFpage8(zero-basedindex7)/printedp4: https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir89-4123.pdf . The corresponding 1991 Mathematics of Computation paper is DOI10.1090/S0025-5718-1991-1079024-2. The report theorem was directly read; the AMS PDF returned403 in this session. Two identical evaluations are arithmetic cross-checks; the formula itself is the cited mathematical input.

## Acceptance boundary

Required: all exact curvature exclusions pass; exact graph coverage replay passes; every leaf stationary family is exactly handled, including positivity of singular families; a feasible attaining word matches the computed global upper bound; exact volume and ratio sign pass; independent assembled audit finds no gap. A float probe or feasible orbit alone supplies no capacity lower bound.
