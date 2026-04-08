# Numerical Story: Argumentative Chain

Target: a single self-contained .tex file (thesis appendix or chapter) that tells the full story of how we compute c_EHZ numerically, what's proven correct, and what the error bounds are. Later carved into main-body vs appendix pieces.

Filepath: `thesis/numerical-story.tex` (replaces existing draft).

Reader: thesis advisors (Kai Cieliebak, Elizabeth Gaar) — mathematicians familiar with symplectic geometry and optimization, not with our codebase.

## Part 0: Problem Statement

### 0a: Standard Symplectic Geometry Setup
- Define ω₀ (standard symplectic form on R⁴), J₀
- State EHZ capacity definition for smooth convex bodies
  - Reeb vector field on \partial K
  - Reeb orbits: closed trajectories of the Reeb flow
  - Action of an orbit = ∫_{orbit} λ, where λ is the Liouville 1-form
  - c_EHZ(K) = minimum of actions of Reeb orbits on \partial K
  - Theorem: there is at least one Reeb orbit, the minimum is achieved
- State Viterbo Conjecture: sys(K) := c_EHZ(K)^2 / (2 vol(K)) \leq 1
  - made in 2000 [TODO: check]
  - disproven in HKO2024 via example for a polytope (which has smoothings, so smooth counterexamples also exist)

- Define polytope K via dual vertex set a_1,...,a_F ∈ R⁴
  - star-shaped with respect to 0, i.e. 0 \in int(K), automatically satisfied in this definition
  - bounded, which is equivalent to 0 \in int(K^o); 0 \in boundary(K^o) would be unbounded
  - no a_i is redundant, i.e. the a_i are the vertices of the dual polytope K^o
  - K = {x : a_i^T x ≤ 1} \subset R^4
- Define dual polytope K^o = conv{a_1,...,a_F}
  - vertices <-> dual vertices
- Define 0,1,2,3-faces
- Define gauge function g_K(x) = min{λ ≥ 0 : x ∈ λK} = max_i=1^F a_i^T x
- Define support function h_K(v) = max_{x∈K} v^T x = g_{K^o}(v)

- State EHZ capacity definition for polytopes
  - constant Reeb vectors on polytope facets, multiple vectors on 0,1,2-faces where multiple 3-facets meet, use the convex hull as Reeb vector set
  - generalized Reeb trajectories/orbits: take velocities from the Reeb vector set available at each point in time, curves are now absolutely continuous in W^{1,2}, not necessarily smooth, not necessarily piecewise linear
  - action = ∫ λ as before, well-defined
  - c_EHZ(K) = minimum action of a generalized Reeb orbit on \partial K
  - Theorem (implied by another, later theorem): there is a generalized Reeb orbit, the minimum is taken

- HK2017 theorem (in our notation):
  A_min = 1/2 * 1/ max_{σ} max_{β≥0, \sum_i a_i β_i = 0 (closed), \sum_i β_i = 1 (normalized)} Q(\beta,\sigma)
  Q(\beta,\sigma) = \sum_{i < j} β_σ(i) β_σ(j) \omega_0(a_{σ(i)}, a_{σ(j)}) (action)
- Theorem (ours): Any argmax can be turned into a minimum action Reeb orbit:
  \dot\gamma(t \in [t_i, t_{i+1}]) = 2 J_0 a_{σ(i)} for i=1,...,F (constant velocity = Reeb vector of facet σ(i))
  t_{i+1} - t_i = β_σ(i) (time increment)
  gamma(0) = ... [TODO: look up formula we derived (new result)]

- Proof of the HK2017 theorem: reproduced and simplified/written up more nicely
  - clarke dual action principle -> surprising theorem: there is a minimizer \gamma which is piecewise linear, with pure Reeb vectors as velocities instead of convex combinations, and each Reeb vector is used for a single contiguous time interval or for zero time.
  - define: simple Reeb orbit = ...
  - Lemma: action(Reeb orbit) = period(Reeb orbit)
  - Lemma: shoelace formula: action(piecewise linear curve) $= sum_{i<j} 1/2 (t_{i+1}-t_i)(t_{j+1}-t_j) ω₀(velocity_i, velocity_j)$
  - Our Theorem: given simple Reeb orbit \gamma, define (\sigma,\beta):
    - wlog t=0 is a breakpoint
    - call breakpoints t_1 \leq t_2 \leq ... ≤ t_F = T ; insert zero-length intervals to reach the full count F for convenience in notation
    - σ(i) = Reeb vector index on (t_i, t_{i+1})
    - β_σ(i) = (t_{i+1} - t_i) / T
    - thus: \sum_i \beta_i = 1 (normalization)
    - thus: \sum_i a_i β_i = 1/T \sum_i a_i (t_{i+1} - t_i) = 1/T \int_0^T 1/2 J_0^{-1} \dot\gamma(t) dt = 0 (closed)
    - then A(gamma)=T [lemma]
    - $Q(β,σ) = 1/T^2 sum_{i<j} (t_{i+1}-t_i)(t_{j+1}-t_j) 1/4 ω₀(2 J_0 a_{σ(i)}, 2 J_0 a_{σ(j)}) = 1/T^2 1/2 A(gamma) = 1/2 / A(gamma)$
    - Thus: $A(gamma) = 1/2 * 1/Q(β,σ)$
  - Our Theorem: reverse: given (σ,β), with Q(β,σ) > 0, define \gamma:
    - T = 1/2 * 1/Q(β,σ)
    - t_{i+1} - t_i = T β_σ(i)
    - \dot\gamma(t ∈ [t_i, t_{i+1}]) = 2 J_0 a_{σ(i)}
    - we get closedness: \sum_i a_i β_i = 0 → \int_0^T 1/2 J_0^{-1} \dot\gamma(t) dt = 0 → gamma(T) = gamma(0)
    - we get per shoelace formula the action: A(gamma) = 1/2 * 1/Q(β,σ) = T
    - tricky part: picking gamma(0), and showing \gamma(t) \in \partial(K) ; requires that (σ,β) is a maximizer, not just any (σ,β) with Q(β,σ) > 0
    - Clarke dual action principle -> 
    


- Our algorithm

- State HK2017 theorem (with citation, without proof): the minimum-action simple Reeb orbit achieves c_EHZ, and corresponds to a maximizer of the combinatorial QP below

### 0b: The combinatorial QP
- For subset S ⊆ {1,...,F}, |S| = m ≥ 2, cyclic permutation σ of S:
  - Action matrix H(σ): H_ij = ω₀(a_σ(i), a_σ(j)), H_ii = 0
  - Constraint matrix C(σ) ∈ R^{5×m}: rows 0–3 are a_σ(k),d (closure), row 4 is all ones (normalization)
  - d = (0,0,0,0,1)^T
- Q_max = max over (S,σ) of max over β of ½β^T H(σ) β, subject to C(σ)β = d, β ≥ 0
- c_EHZ(K) = 1/(2 Q_max)

### 0c: Minimum-support reduction
- A global maximizer (σ*,β*) may have β*_k = 0 (orbit doesn't dwell on facet k)
- Among maximizers achieving Q_max, take one with minimum |supp(β*)|
- This minimum-support maximizer has β* > 0: if β*_k = 0, removing index k from σ gives shorter σ' with same Q, contradicting minimality
- Consequence: can skip any (σ,β) where max of Q on {Cβ=d, β≥0} is attained only on boundary (β_k=0 for some k). A shorter σ' achieves same or higher Q.

Depends on: nothing (self-contained definitions + one cited theorem)

## Part 1: Exact vs. Approximate Computation

Distinguish what can be computed exactly from what requires floating-point approximation.

- **Exact over Q**: polytopes stored as a_i ∈ Q⁴. All combinatorial quantities exact:
  - Vertex enumeration (Cramer's rule)
  - Vertex-facet incidence
  - Symplectic signs sign(ω₀(a_i, a_j))
  - Non-degeneracy: after generic perturbation of a_i, all ω₀(a_i, a_j) ≠ 0

- **Floating-point computation**: the per-σ QP solve (SVD, eigendecomposition, linear algebra)
  - Define ε_mach ≈ 1.11 × 10⁻¹⁶ (IEEE 754 double precision unit roundoff)
  - Introduces rounding errors in β, λ, Q
  - Branching decisions (β > 0?, eigenvalue signs) can be wrong near thresholds

- **Key design principle**: every numerical decision is trinary (TRUE / FALSE / INDETERMINATE), with a continuous margin separating TRUE from INDETERMINATE. Discontinuous predicates (rank, sign) are replaced by continuous quantities (singular values, margins) + thresholds.

Depends on: Part 0

## Part 2: Pruning (Exact + Conservative Heuristic)

Which (σ, S) pairs can be skipped before solving the QP?

- Adjacency pruning: for consecutive pair (σ(i), σ(i+1)), check whether a Reeb trajectory from facet σ(i) to σ(i+1) exists. Uses directed transition feasibility test on the facet normals.
  - Condition (1): sign of ω₀(a_σ(i), a_σ(i+1)) — exact from rational arithmetic
  - Condition (2): existence of x in the intersection of the two facets satisfying halfspace conditions — LP feasibility in floating-point with relaxed tolerance → conservative (never prunes valid transitions, may keep invalid ones)
- Pruning is performance optimization, not correctness-critical: algorithm is correct even with no pruning
- After pruning, remaining (σ, S) pairs are the "candidate orbits" to solve

Depends on: Part 1

## Part 3: The Per-σ Solver

For a fixed σ, we solve: maximize Q(β) = ½β^T H β subject to Cβ = d, β ≥ 0.

### 3a: Notation and general structure
- Suppress σ-dependence: H = H(σ), C = C(σ)
- Affine set A = {β : Cβ = d}, feasible set F = A ∩ {β ≥ 0}
- F is compact (contained in simplex Δ = {β ≥ 0, Σβ_i = 1}), possibly empty
- Q attains its max on F (continuous on compact, when F ≠ ∅)

### 3b: Projection formulation (primary)
- SVD of C → orthonormal null-space basis V ∈ R^{m×k}, k = m − rank(C)
- Particular solution β₀ = C⁺d (minimum-norm)
- Parametrize A as β = β₀ + Vα, α ∈ R^k
- Reduced Hessian H' = V^T H V ∈ R^{k×k}
- Reduced gradient g = V^T H β₀ ∈ R^k
- Q(β₀ + Vα) = ½α^T H' α + g^T α + Q(β₀)
- Stationarity: H'α + g = 0

### 3c: Critical-point classification
Three cases for H'α + g = 0:
1. H' invertible: unique α* = −(H')⁻¹g, β* = β₀ + Vα*
2. H' singular, g ⊥ ker(H'): affine subspace of critical points, all with same Q
3. H' singular, g ∉ im(H'): no critical point on A

### 3d: Boundary vs. interior maximum
Which cases lead to max on boundary ∂F (some β_k = 0) vs. interior (β > 0)?
- Boundary cases (B1–B4):
  - B1: some γ_j > 0 → saddle → second-order necessary condition for max violated
  - B2: H' neg. def., unique β* but β*_k ≤ 0 → max at boundary
  - B3: H' singular, g ∉ im(H') → no critical point → max at boundary
  - B4: H' neg. semidef., g ⊥ ker(H'), critical subspace misses {β > 0}
- Interior cases (I1–I2):
  - I1: H' neg. def., unique β* > 0 → interior maximum
  - I2: H' neg. semidef., g ⊥ ker(H'), critical subspace intersects {β > 0}
- In I1 and I2, Q(β*) is the per-σ contribution to Q_max

### 3e: Connection to minimum-support
In boundary cases B1–B4, the per-σ solver returns DROP: a shorter σ' achieves same Q (Part 0c). The algorithm only needs to report Q for interior cases I1/I2.

Depends on: Part 0

## Part 4: Second-Order Classification (Trinary)

The floating-point solver must determine the eigenvalue signs of H' — but this is a discontinuous test. Replace by a trinary test with continuous margin.

- Eigenvalues γ_j of H' are computed from floating-point SVD + eigendecomposition
- The computed γ̃_j satisfy |γ̃_j − γ_j| ≤ ε_γ where ε_γ is derived in Part 6 (Link 4)
- Trinary sign test:
  - |γ̃_j| > ε_γ → sign(γ̃_j) = sign(γ_j), certified
  - |γ̃_j| ≤ ε_γ → INDETERMINATE
- Classification:
  - All γ̃_j < −ε_γ → H' negative definite → unique interior maximum (I1, if also β > 0)
  - Some γ̃_j > ε_γ → saddle point → DROP (shorter σ' finds the boundary max)
  - Some γ̃_j ∈ [−ε_γ, ε_γ] → INDETERMINATE: proceed as if case I2 (flat direction preserving Q)

**Note:** ε_γ = c · ‖H‖ · ε_mach / σ_min(C) is a derived threshold (Part 6, Link 4), not heuristic. The reader needs only the functional form ε_γ here; the derivation is in Part 6. [This is a forward reference by necessity: the threshold formula cannot be stated without the perturbation chain, but the classification logic must be described before the chain.]

Depends on: Part 3, Part 6 (for ε_γ formula)

## Part 5: β > 0 Classification (Trinary)

Given β̃ from the solver, certify whether β* > 0.

- Componentwise perturbation bound: η_i ≥ |β̃_i − β*_i| for each i (derived in Part 6, Link 5)
- Trinary test:
  - TRUE: β̃_i − η_i > 0 for all i → certified β* > 0
  - FALSE: β̃_i + η_i < 0 for some i → certified β*_i < 0
  - INDETERMINATE: otherwise

**What INDETERMINATE means for capacity:** Cannot drop (Q̃ might overestimate Q_F), cannot trust Q̃ as lower bound on Q_max. Must propagate to accumulator (Part 8).

**Note:** η_i derivation in Part 6, Link 5. Same forward-reference situation as Part 4.

Depends on: Part 3, Part 6 (for η_i bound)

## Part 6: Error Analysis — Perturbation Chain

Forward error analysis: traces rounding errors from input through each computation step. All bounds have σ_min(C) in denominators — this is the conditioning gateway.

### Standard results used (stated with citations, not proved)
- Weyl's eigenvalue perturbation theorem: |λ̃_j − λ_j| ≤ ‖E‖ for symmetric A + E
- Davis-Kahan sin(Θ) theorem: subspace perturbation bound
- Backward stability of SVD (Higham 2002): computed SVD is exact SVD of C + δC
- Backward stability of eigendecomposition (Golub & Van Loan 2013)
- Pseudoinverse perturbation (Wedin 1973 / Stewart 1977): rank-preserving case

### Link 1: Assembly
- C, H assembled from a_i by O(m²) floating-point ops
- ‖δC‖ ≤ c₁ · ε_mach · ‖C‖, ‖δH‖ ≤ c₂ · ε_mach · ‖H‖
- In EHZ setting, ‖C‖, ‖H‖ = O(1), so perturbations are O(ε_mach)

### Link 2: SVD of C → null-space basis V
- Backward stable SVD: computed SVD is exact SVD of C + δC_SVD
- Davis-Kahan: ‖sin Θ(Ṽ, V)‖ ≤ ‖δC_total‖ / σ_min(C)
- σ_min(C) is the conditioning gateway: all downstream bounds have σ_min(C) in denominators

### Link 2b: Particular solution β₀ = C⁺d
- ‖δβ₀‖ = O(ε_mach · ‖C‖ / σ_min(C)²)
- One power worse than null-space error because β₀ depends on inverses of singular values

### Link 2c: Reduced gradient g = V^T H β₀
- Three first-order terms (from δV, δH, δβ₀), all bounded
- ‖δg‖ = O(ε_mach / σ_min(C)²) in EHZ setting

### Link 3: Reduced Hessian H' = V^T H V
- ‖ΔH'‖ = O(‖H‖ · ε_mach / σ_min(C))

### Link 4: Eigenvalues of H'
- Weyl + backward stability of eigendecomposition
- |γ̃_j − γ_j| = O(‖H‖ · ε_mach / σ_min(C))
- → eigenvalue sign threshold ε_γ = c₅ · ‖H‖ · ε_mach / σ_min(C) (delivers Part 4's threshold)

### Link 5: Critical point and β certification
- Per-eigendirection error: |δα_j| = O(ε_mach / |γ_j|)
  - Amplification 1/|γ_j| makes error large for near-zero eigenvalues
- Full β error decomposition: three sources (critical-point shift, null-space rotation, particular-solution shift)
- Computable componentwise bound η_i (delivers Part 5's threshold)
  - η_i uses computed quantities (Ṽ, w̃_j, γ̃_j, α̃) as proxies for exact ones
  - Safety constant c = m² (empirical; zero violations on well-conditioned problems)

### Gap: null-eigenvalue case
- When H' has near-zero eigenvalues, the solver searches null eigendirections
- The η_i bound above assumes all retained eigenvalues are well-separated from zero
- Extending the bound to cover the null-eigenvalue search: open

Depends on: Part 3b (projection formulation)

## Part 7: Error Analysis — Q Error Bound

Backward error analysis: complementary to Part 6. Uses KKT residual directly, doesn't go through the perturbation chain.

### First-order Q bound (lem:q-error-first-order, PROVEN)
- |Q(β̃) − Q(β*)| ≤ ‖H‖·‖β*‖·‖r‖/σ_min(C) + ½‖H‖·‖δβ‖²
- Proof idea: Taylor expand Q at β*. First-order term (Hβ*)^T δβ = −λ*^T r_λ by KKT stationarity Hβ* = −C^T λ*. Bound ‖λ*‖ via σ_min(C).
- Key insight: at critical point, dQ/dβ = Hβ* ∈ range(C^T), so null(C) perturbations don't affect Q at first order — only constraint residual r_λ matters.

### Q correction (lem:q-correction-second-order, PROVEN)
- Q_corr = Q(β̃) + λ̃^T r_λ cancels first-order error exactly
- Corrected error = δλ^T r_λ + ½ δβ^T H δβ (both second-order)
- In practice: corrected Q errors at machine epsilon for well-conditioned problems

### Structural theorem (lem:pseudoinverse-orthogonality, PROVEN; cor:taylor-structure, GAP)
- For pseudoinverse solution: δx^T M δx = 0 (orthogonality of δx and residual in col(M))
- Consequence: first-order term = −2 × second-order term exactly
- → Q_corr is exact in exact arithmetic (cor:exact-correction, PROVEN)
- cor:taylor-structure proof has GAP (extra terms need algebraic cancellation argument)

### Runtime bound
- E₁ = ‖H‖·‖β̃‖·‖r‖/σ_min(C) — all quantities available from solver at runtime
- Bounds first-order part of Q error; second-order part negligible for well-conditioned problems

### Q loss from dropping near-boundary components (lem:near-boundary-drop, PROVEN)
- If β*_k ≤ ε and other components bounded away from zero, the shorter σ' (with index k removed) achieves Q within E_drop of Q(β*), where E_drop = O(ε · ‖H‖ · ‖β*‖)
- Used in Part 8 to justify ignoring low-Q INDETERMINATE nodes

Depends on: Part 3 (either formulation works for first-order bound; projection formulation for structural theorem)

## Part 8: The Accumulator

Collects all (σ, Q̃, verdict) triples and determines c_EHZ.

- Verdicts: TRUE (Parts 4+5 both certified), FALSE (eigenvalue or β test fails), INDETERMINATE (either test inconclusive)
- Best certified Q̃: highest Q̃ among TRUE nodes
- Best uncertain Q̃: highest Q̃ among TRUE ∪ INDETERMINATE nodes
- Safety assertion: best uncertain ≥ best certified (if violated → invariant failure)
- Final capacity: c_EHZ = 1/(2 · Q̃_best_certified)

### Which error bound applies?
- For TRUE nodes: Q error bounded by E₁ from Part 7. The reported Q̃ is within E₁ of Q*.
- For INDETERMINATE nodes: Q̃ may overestimate Q_F (because β* might not be > 0, so the unconstrained Q exceeds the constrained maximum).

### Lazy INDETERMINATE resolution
- Sort by Q̃ descending
- Any INDETERMINATE with Q̃ > Q̃_best_certified: must be resolved (e.g. by exact arithmetic)
- INDETERMINATE with Q̃ ≤ Q̃_best_certified: safe to ignore
  - Even if the true Q_F differs, shorter σ' covers it via minimum-support (Part 0c)
  - Q loss from dropping bounded by lem:near-boundary-drop (Part 7)
- If no TRUE nodes exist: all candidates INDETERMINATE → must resolve

Depends on: Parts 4, 5, 7

## Part 9: End-to-End Error Statement

The punchline: what can we say about the computed capacity?

### For well-conditioned capacity-achieving orbits (the common case):
- Q error: |Q̃ − Q*| ≤ E₁ ≈ ε_mach · ‖H‖·‖β‖/σ_min(C) · ‖r‖
- β > 0 certified with margin ≫ η_i
- Capacity error: |c̃ − c*| = |1/(2Q̃) − 1/(2Q*)| ≈ E₁/(2Q*²)

### For degenerate orbits:
- Q error can be large (E₁ up to O(1) when σ_min(C) small)
- β > 0 may be INDETERMINATE
- CONJECTURE: degenerate orbits are never capacity-achieving
  - Empirical: on all tested polytopes, the winning orbit is well-conditioned
  - Geometric argument: degenerate orbits have σ_min(C) → 0, meaning facet normals are nearly linearly dependent — geometrically degenerate configuration
  - GAP: no proof that this holds in general

### Empirical validation summary
- 7 literature polytopes match published values at 1e-6 relative tolerance
- 6 capacity axioms verified on 47 polytopes
- Zero false positives in β > 0 classification (45K problems, 0 violations)
- 9 false negatives (root-caused: eigenvalue threshold for m=6 rank-deficient problems)
- Q error bound (B3): zero violations on 51K problems, max ratio 0.217

Depends on: everything above

## Dependency Graph

```
Part 0 (problem, definitions, HK2017, minimum-support)
  ├→ Part 1 (exact vs approximate, ε_mach, trinary design principle)
  │    └→ Part 2 (pruning: adjacency test, LP feasibility)
  ├→ Part 3 (per-σ solver: projection, H', critical points, B1-B4/I1-I2)
  │    ├→ Part 4 (eigenvalue classification, trinary) ←── Part 6 Link 4 (ε_γ)
  │    ├→ Part 5 (β > 0 classification, trinary) ←── Part 6 Link 5 (η_i)
  │    ├→ Part 6 (perturbation chain: Links 1–5, needs projection formulation)
  │    └→ Part 7 (Q error bound: first-order, correction, near-boundary drop)
  └→ Part 8 (accumulator: verdicts, lazy resolution) ← Parts 4, 5, 7
       └→ Part 9 (end-to-end error + validation) ← everything
```

Forward references: Parts 4,5 use thresholds from Part 6. Unavoidable: the classification logic must be described before its justification (the perturbation chain). The .tex handles this by stating the threshold formulas in Parts 4,5 with forward citations to Part 6 where the derivation lives.

## Open Items

1. **cor:taylor-structure GAP** — algebraic cancellation in proof needs verification
2. **Null-eigenvalue case** — η_i bound doesn't cover LP search for flat eigendirections; open extension
3. **Constants c₁–c₆** — currently c = m² (empirical safety factor), not derived from first principles
4. **Degenerate-orbit conjecture** — unproven
5. **Transition feasibility lemma** — cited in Part 2 but formal statement+proof lives in library math.tex (lem:numerical-transition-feasibility), needs to be either reproduced or cited
6. **Part III of math.tex** (formal \begin{algorithm} for the floating-point solver) not written yet
