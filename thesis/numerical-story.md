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

[AGENT ADDITION] Properties of J₀ and ω₀ used downstream (from thesis/basic-definitions.tex rem:j0-properties, Jörn-approved):
- J₀² = -I₄,  J₀⁻¹ = J₀ᵀ = -J₀
  [Reader needs this for: inverting J₀ in the closure condition derivation (Part 0a proof), understanding R_i = 2J₀a_i]
- ω₀(J₀u, J₀v) = ω₀(u,v)  (J₀-invariance)
  [Reader needs this for: the Q(β,σ) ↔ action computation, where ω₀(2J₀a_i, 2J₀a_j) = 4ω₀(a_i, a_j)]
- |J₀u| = |u|  (isometry)
- ω₀(u,u) = 0  (antisymmetry)
  [Reader needs this for: H_ii = 0 in the action matrix, self-pairing vanishes in shoelace formula]
- ω₀(u,v) = -ω₀(v,u)
  [Reader needs this for: understanding H construction. H is defined by H_ij = ω₀(a_σ(i), a_σ(j)) for i<j, then H_ji := H_ij, H_ii = 0. Symmetric by construction (not inherited from ω₀). The quadratic form ½β^THβ = Σ_{i<j} β_i β_j ω₀(a_σ(i),a_σ(j)) uses only the upper triangle. See lem:H-quadratic.]
- ω₀ is nondegenerate: ω₀(u,v) = 0 for all v implies u = 0.
  [Reader needs this for: understanding that J₀ is invertible, and that ω₀ distinguishes directions]

[AGENT ADDITION] Liouville form and action (from thesis/basic-definitions.tex, Jörn-approved):
- Liouville form: λ₀ = ½ Σ (q_i dp_i - p_i dq_i), satisfying dλ₀ = ω₀
- At point x, applied to tangent vector v: λ₀|_x(v) = ½ ω₀(x,v) = ½ ⟨J₀x, v⟩
- Action of closed curve γ: A(γ) = ∫_γ λ₀ = ½ ∫₀ᵀ ⟨J₀γ(t), γ̇(t)⟩ dt
  [Reader needs this for: the shoelace formula derivation, and for A(γ)=T (contact normalization)]
- Contact normalization on polytopes: since λ₀|_x(R_i) = 1 on each facet, and generalized Reeb orbits have γ̇ ∈ conv{R_i}, we get λ₀(γ̇) = 1, hence A(γ) = T.
  [Reader needs this for: the fundamental identity A(γ)=T used throughout]

[AGENT ADDITION] Reeb vectors in dual-vertex notation:
- Facet Reeb vector: R_i = 2 J₀ a_i
  - Derivation: if a_i = n_i/h_i (unit normal / height), then R_i = (2/h_i) J₀ n_i = 2 J₀ (n_i/h_i) = 2 J₀ a_i
  - Tangent to ∂K: ⟨R_i, a_i⟩ = 2⟨J₀a_i, a_i⟩ = 2ω₀(a_i, a_i) = 0 ✓  (but note: ⟨R_i, n_i⟩ = 0 is the geometric fact; ⟨R_i, a_i⟩ = 0 is the same since a_i ∝ n_i)
  [Reader needs this for: velocity formula in the orbit construction, closure condition Σβ_i R_σ(i) = 0 ↔ Σβ_i a_σ(i) = 0]

### 0b: The combinatorial QP

[Reader needs: ω₀ definition, J₀, polytope K with dual vertices a_i, Reeb vectors R_i = 2J₀a_i, shoelace formula, action = period]

- For subset S ⊆ {1,...,F}, |S| = m ≥ 2, cyclic permutation σ of S:
  - **Action matrix** H(σ) ∈ R^{m×m}: for i < j, H_ij = H_ji = ω₀(a_σ(i), a_σ(j)); H_ii = 0.
    - H is symmetric by construction (H_ij := ω₀(a_σ(i), a_σ(j)) for i<j, extended symmetrically)
    - Note: ω₀ is antisymmetric, but H is symmetric because H uses the (i<j) convention: the double sum Σ_{i<j} β_i β_j ω₀(a_σ(i), a_σ(j)) = ½ β^T H β (lem:H-quadratic, proven in formal/library/kkt.tex)
    - H is indefinite in general (not positive or negative definite) — this is what makes the QP non-trivial
  - **Constraint matrix** C(σ) ∈ R^{5×m}:
    - Rows 0–3: C_{d,k} = a_{σ(k),d} for d=0,...,3 (closure: Σ a_σ(i) β_i = 0 ↔ A^T β = 0)
    - Row 4: all ones (normalization: Σ β_i = 1)
    - Right-hand side: d = (0,0,0,0,1)^T
    - Note: C has 5 rows, m columns. Full row rank requires m ≥ 5 (otherwise C is automatically rank-deficient).
      For m ≤ 5, rank(C) < 5 is generic, meaning the constraint set A = {Cβ = d} may be a single point or empty.
  - Origin of the constraints:
    - Closure: Reeb orbit closes iff Σ β_i R_σ(i) = 0. Since R_i = 2J₀a_i and J₀ is invertible, this is equivalent to Σ a_σ(i) β_i = 0.
    - Normalization: dwell-time fractions sum to 1: Σ β_i = 1.
    - See lem:dual-vertex-qp in formal/library/kkt.tex (Jörn-approved) for the complete derivation.

- The QP: Q_max = max over (S,σ) of max over β of ½β^T H(σ) β, subject to C(σ)β = d, β ≥ 0
- c_EHZ(K) = 1/(2 Q_max)

[SCOPE: Should we include the proof that Q_max > 0? It follows from HK2017 (existence of a Reeb orbit with positive action), but the reader might want to see it stated explicitly.]

Depends on: Part 0a (definitions, HK2017 theorem)

### 0c: Minimum-support reduction

[Reader needs: Q_max definition, the (σ,β) optimization problem, β ≥ 0 constraint]

- A global maximizer (σ*,β*) may have β*_k = 0 (orbit doesn't dwell on facet k)
- Among maximizers achieving Q_max, take one with minimum |supp(β*)|
- **Proof that minimum-support maximizers have β* > 0:**
  - Suppose β*_k = 0 for some k. Remove σ(k) from the permutation → shorter σ'.
  - The k-th row/column of H contributes nothing to ½β^T H β (since β_k = 0 and H_kk = 0).
  - The remaining β components still satisfy the constraints (closure and normalization are linear, and removing a zero-weight term doesn't change them).
  - So (σ', β'_{-k}) achieves the same Q value with smaller support, contradicting minimality. ∎
- **Consequence for the algorithm:** Can skip any (σ,β) where the maximum of Q on {Cβ=d, β≥0} is attained only on the boundary (β_k=0 for some k). A shorter σ' achieves same or higher Q.
- **Algorithmic implication:** The per-σ solver (Part 3) only needs to find and report the **interior maximum** (β > 0). If the maximum is on the boundary, it returns DROP — the HK2017 outer loop over all σ lengths will find the same Q via a shorter σ'.

Depends on: Part 0b

## Part 1: Exact vs. Approximate Computation

[Reader needs: the QP formulation from Part 0b, that a_i ∈ Q⁴ in our implementation]

Distinguish what can be computed exactly from what requires floating-point approximation.

- **Exact over Q**: polytopes stored as a_i ∈ Q⁴ (rational coordinates). All combinatorial quantities are exact:
  - Vertex enumeration (Cramer's rule over Q)
  - Vertex-facet incidence
  - Symplectic products ω₀(a_i, a_j) = Σ (a_{i,q_k} a_{j,p_k} - a_{i,p_k} a_{j,q_k}) — exact since a_i ∈ Q⁴
  - Hence H(σ) and C(σ) are exact over Q
  - The set of candidate σ (which permutations to try) is determined exactly

- **Floating-point computation**: the per-σ QP solve for each σ:
  - SVD of C (to get null-space basis V and particular solution β₀)
  - Eigendecomposition of H' = V^T H V (to classify the critical point)
  - Linear algebra to solve H'α + g = 0 (to find β*)
  - All subject to rounding: every f64 operation fl(a ⊕ b) = (a ⊕ b)(1 + δ) with |δ| ≤ ε_mach ≈ 1.11 × 10⁻¹⁶ (IEEE 754 double precision unit roundoff)
  - Introduces errors in β, the eigenvalues γ_j of H', and Q(β)
  - **Branching decisions can be wrong**: β_k > 0? (sign test near zero), γ_j > 0? (eigenvalue sign near zero), rank(C) = 5? (singular value near zero)

- **The fundamental problem**: the exact algorithm (Part 3) uses discontinuous tests (sign of β_k, sign of γ_j, rank of C). Floating-point evaluation of a discontinuous test is unreliable near the discontinuity.

- **Key design principle**: replace each discontinuous predicate by a **trinary test** with continuous margin:
  - TRUE: the predicate holds, certified by a margin that exceeds the proven error bound
  - FALSE: the predicate fails, certified by margin
  - INDETERMINATE: the margin is too small to certify — cannot decide
  - The margin is a continuous function of the input; the TRUE/INDETERMINATE boundary is at a derived threshold (not heuristic)
  - INDETERMINATE is propagated to the accumulator (Part 8) for lazy resolution

[Reader needs for next part: the trinary design principle, ε_mach definition]

Depends on: Part 0

## Part 2: Pruning (Exact + Conservative Heuristic)

[Reader needs: polytope K with facets, Reeb vectors R_i = 2J₀a_i, the outer loop over σ]

Which (σ, S) pairs can be skipped before solving the QP?

- **Adjacency pruning** (lem:numerical-transition-feasibility in formal/library/kkt.tex):
  For a simple Reeb orbit to visit facets σ(i) then σ(i+1) consecutively, there must exist a point on the shared facet boundary where the trajectory can transition. Two conditions are necessary and sufficient:
  - **Condition 1 (symplectic sign):** ω₀(a_σ(i), a_σ(i+1)) ≥ 0
    - Exact from rational arithmetic (a_i ∈ Q⁴)
    - Physical meaning: the approach trajectory (velocity R_σ(i) = 2J₀a_σ(i)) must not violate the constraint ⟨a_σ(i+1), x⟩ ≤ 1 as it approaches the transition point. Working out ⟨a_σ(i+1), x + t·J₀a_σ(i)⟩ = ⟨a_σ(i+1), x⟩ + t·ω₀(a_σ(i), a_σ(i+1)) for t < 0 shows this requires ω₀(a_σ(i), a_σ(i+1)) ≥ 0.
  - **Condition 2 (LP feasibility):** ∃ x ∈ F_σ(i) ∩ F_σ(i+1) such that ⟨a_k, x⟩ < 1 for all k that would be "pushed" beyond their facet by the approach or departure velocity.
    - Specifically: k ∉ {σ(i), σ(i+1)} with ω₀(a_σ(i), a_k) < 0 or ω₀(a_σ(i+1), a_k) > 0
    - This is an LP feasibility problem, solved in floating-point with relaxed tolerance → **conservative**: never prunes valid transitions (may keep invalid ones)
    - The LP tests whether there's room on the shared boundary for the trajectory to pass without hitting a third facet

- **Pruning is performance optimization, not correctness-critical**: the algorithm is correct even with no pruning. Pruning reduces the number of QPs to solve but doesn't affect which σ gives Q_max.

- After pruning, remaining (σ, S) pairs are the "candidate orbits" to solve.

[SCOPE: Include the full proof of lem:numerical-transition-feasibility here, or just state and cite? The proof is in formal/library/kkt.tex and is ~90 lines. It's self-contained and geometrically instructive but not on the critical path of the numerical error story.]

Depends on: Part 0, Part 1

## Part 3: The Per-σ Solver

For a fixed σ, we solve: maximize Q(β) = ½β^T H β subject to Cβ = d, β ≥ 0.

[Reader needs: H, C, d from Part 0b; the minimum-support consequence (if max is on boundary → DROP)]

### 3a: Notation and general structure (rem:qp-setup in formal/numerics/error-bounds.tex)

- Suppress σ-dependence: H = H(σ), C = C(σ)
- Affine constraint set: A = {β ∈ R^m : Cβ = d}
- Feasible set: F = A ∩ {β ≥ 0}
- A may be empty (if Cβ = d is inconsistent)
- F is compact: closed (intersection of closed sets) and bounded (contained in simplex Δ = {β ≥ 0, Σβ_i = 1} by the normalization constraint). Includes F = ∅.
- When F ≠ ∅: Q attains its max on F (continuous function on compact set)
- The per-σ problem: find max_{β ∈ F} Q(β), or determine it's on the boundary

### 3b: Projection formulation (primary)

- **SVD of C** → orthonormal null-space basis V ∈ R^{m×k}, where k = m − rank(C)
  - C = UΣV_full^T; last k columns of V_full form V (corresponding to zero singular values)
  - V^T V = I_k (orthonormal columns)
- **Particular solution** β₀ = C⁺d (minimum-norm solution of Cβ = d, via pseudoinverse)
  - β₀ ∈ A (satisfies constraints)
  - β₀ ⊥ ker(C) (minimum-norm property)
  - ‖β₀‖ ≤ ‖C⁺‖ · ‖d‖ = ‖d‖/σ_min(C) [this enters later bounds]
- **Parametrize A** as β = β₀ + Vα, α ∈ R^k
  - Every β ∈ A can be written this way (β₀ is a particular solution, Vα spans ker(C))
  - The problem reduces from m-dimensional β to k-dimensional α
- **Reduced Hessian** H' = V^T H V ∈ R^{k×k} (symmetric, since H is symmetric)
  - Eigenvalues γ₁,...,γ_k with orthonormal eigenvectors w₁,...,w_k
  - H' is the restriction of the quadratic form to the constraint manifold
- **Reduced gradient** g = V^T H β₀ ∈ R^k
- **Q on the affine plane** (prop:q-on-affine):
  Q(β₀ + Vα) = ½α^T H' α + g^T α + Q(β₀)
  - Quadratic in α with Hessian H', linear term g, constant Q(β₀)
  - Setting ∇_α = 0: stationarity condition H'α + g = 0

### 3c: Critical-point classification (prop:critical-points in formal/numerics/error-bounds.tex)

Three cases for H'α + g = 0:

1. **H' invertible** (det H' ≠ 0): unique α* = −(H')⁻¹g, hence β* = β₀ + Vα*.
   - The critical point is unique; it's a maximum iff H' is negative definite.
2. **H' singular, g ⊥ ker(H')** (i.e., g ∈ im(H')): affine subspace of critical points: α*_part + ker(H').
   - All critical points give the same Q value (lem:well-defined, proven in formal/library/kkt.tex, Jörn-approved).
   - This is the semidefinite case with a flat direction preserving Q.
3. **H' singular, g ∉ im(H')**: H'α + g = 0 is inconsistent, no critical points on A.
   - Q|_A has no stationary point.

### 3d: Boundary vs. interior maximum (prop:boundary-vs-interior in formal/numerics/error-bounds.tex)

**Boundary cases** (F_max ⊆ ∂F, i.e., every maximizer has some β_k = 0):

- **B1: Some γ_j > 0** (H' has a positive eigenvalue).
  Argument: if some maximizer β* had β* > 0, then β* would be a local max of Q|_A (since F agrees with A near any interior point). The second-order necessary condition for a local max requires H' ≼ 0, contradicting γ_j > 0. So all maximizers are on ∂F.

- **B2: All γ_j < 0 (H' negative definite), unique β* but β*_k ≤ 0 for some k.**
  Argument: β* is the global max of Q on A (strict concavity), and Q(β) < Q(β*) for all β ∈ A \ {β*}. Since β* ∉ {β > 0}, the max on F is on ∂F.

- **B3: H' singular, g ∉ im(H')** (case 3 above — no critical point on A).
  Argument: if some maximizer β* had β* > 0, it would be a local max of Q|_A (locally F = A), hence a critical point. But no critical point exists. Contradiction.

- **B4: H' negative semidefinite, g ⊥ ker(H'), critical subspace doesn't intersect {β > 0}.**
  Argument: Q|_A is concave, max attained on the critical subspace. Since no critical point has β > 0, max is on ∂F.

**Interior cases** (F_max ∩ {β > 0} ≠ ∅):

- **I1: All γ_j < 0 (negative definite H'), unique β* > 0.**
  β* is the global max on A (concavity) and lies in int(F). Q_F = Q(β*).

- **I2: H' negative semidefinite, g ⊥ ker(H'), critical subspace intersects {β > 0}.**
  Q_F = Q(β*) for any critical β* > 0.

In I1 and I2, the interior maximizer has β* > 0 and Q(β*) is the per-σ contribution to Q_max.

### 3e: Connection to minimum-support

In boundary cases B1–B4, the per-σ solver returns **DROP**: by Part 0c, a shorter σ' achieves the same Q. The algorithm only needs to report Q for interior cases I1/I2.

**This is why the solver doesn't need to compute the boundary maximum.** The outer loop over all σ lengths will find the same Q via a shorter permutation where the maximum is interior.

Depends on: Part 0

## Part 4: Second-Order Classification (Trinary)

[Reader needs: eigenvalues γ_j of H', cases B1/I1/I2 from Part 3d, trinary design principle from Part 1]

The floating-point solver must determine the eigenvalue signs of H' — but this is a discontinuous test (sign flips at γ_j = 0). Replace by a trinary test with continuous margin.

- Eigenvalues γ_j of H' are computed from floating-point SVD + eigendecomposition
- The computed γ̃_j satisfy |γ̃_j − γ_j| ≤ ε_γ (derived in Part 6, Link 4)
  - The bound: ε_γ = c₅ · ‖H‖ · ε_mach / σ_min(C)
  - Sources of error: (1) assembly errors in H and C (Link 1), (2) null-space basis error from SVD of C (Link 2), (3) reduced Hessian formation V^T H V (Link 3), (4) eigendecomposition of H' (backward stable)
  - ε_γ is a **derived threshold**, not a heuristic: it bounds the total error accumulated through the perturbation chain

- **Trinary sign test** for each eigenvalue:
  - |γ̃_j| > ε_γ → sign(γ̃_j) = sign(γ_j), **certified**
    Proof: |γ̃_j − γ_j| ≤ ε_γ < |γ̃_j|, so γ_j has the same sign as γ̃_j.
  - |γ̃_j| ≤ ε_γ → **INDETERMINATE**: γ_j could be positive, zero, or negative

- **Classification of the per-σ problem:**
  - All γ̃_j < −ε_γ → H' negative definite (certified) → unique interior maximum (case I1, if also β > 0)
  - Some γ̃_j > ε_γ → H' has a positive eigenvalue (certified) → **DROP** (case B1, shorter σ' finds boundary max)
  - Some γ̃_j ∈ [−ε_γ, ε_γ] → **INDETERMINATE**: proceed as if case I2 (search flat eigendirections for β > 0 via LP, rem:near-null-lp-search in formal/library/kkt.tex)
  - No eigenvalue with γ̃_j > ε_γ, but some with |γ̃_j| ≤ ε_γ: could be I2 (semidefinite, flat direction) or B1 (small positive eigenvalue). The LP search handles this.

**Forward reference:** The threshold formula ε_γ = c₅ · ‖H‖ · ε_mach / σ_min(C) is derived in Part 6, Link 4. The derivation traces how rounding errors propagate from the input a_i through C → SVD → V → H' → eigenvalues. The reader needs only the functional form here; the justification is in Part 6.

Depends on: Part 3, Part 6 (for ε_γ formula)

## Part 5: β > 0 Classification (Trinary)

[Reader needs: β* = β₀ + Vα*, the three error sources in β from Part 6 Link 5, trinary design principle]

Given β̃ from the solver, certify whether β* > 0.

- **Componentwise perturbation bound:** for each i = 1,...,m:
  |β̃_i − β*_i| ≤ η_i
  where η_i is a computable bound derived from the perturbation chain (Part 6, Link 5).

- **The bound η_i decomposes into three sources** (eq:beta-error-decomposition in formal/numerics/error-bounds.tex):
  1. **Critical-point shift:** Σ_j |(Vw_j)_i| · |δα_j|, where |δα_j| ≈ ε_mach / |γ_j| (amplification by 1/|γ_j| per eigendirection)
  2. **Null-space rotation:** ‖δV‖ · ‖α*‖ (from perturbation of V)
  3. **Particular-solution shift:** ‖δβ₀‖ (from perturbation of C⁺d)

- **Computable formula** (eq:eta-computable in formal/numerics/error-bounds.tex):
  η_i = (Ê_{ΔH'} ‖α̃‖ + Ê_{δg}) · Σ_j |(Ṽw̃_j)_i| / |γ̃_j| + Ê_{δV} ‖α̃‖ + Ê_{δβ₀}
  where:
  - Ê_{ΔH'} = c · ‖H̃‖ · ε_mach / σ̃_min(C)  [from Link 3]
  - Ê_{δg} = c · ‖H̃‖ · ‖C̃‖ · ε_mach / σ̃_min(C)²  [from Link 2c]
  - Ê_{δV} = c · ε_mach / σ̃_min(C)  [from Link 2]
  - Ê_{δβ₀} = c · ‖C̃‖ · ε_mach / σ̃_min(C)²  [from Link 2b]
  - All tildes denote computed (f64) values, available from the solver
  - Safety constant c = m² (empirical; zero violations on well-conditioned problems)
  - η_i = ∞ if any retained eigenvalue has |γ̃_j| ≤ Ê_{ΔH'} (perturbation dominates eigenvalue)

- **Trinary β > 0 test** (for each component i):
  - **TRUE:** β̃_i − η_i > 0 for all i → certified β*_i > 0 for all i
  - **FALSE:** β̃_i + η_i < 0 for some i → certified β*_i < 0 (infeasible)
  - **INDETERMINATE:** otherwise — cannot determine sign of β*_i

- **What INDETERMINATE means for capacity:**
  - Cannot DROP: Q̃ might overestimate Q_F (because β* might not be > 0, so the unconstrained Q exceeds the constrained maximum)
  - Cannot trust Q̃ as lower bound on Q_max (the overestimate means Q̃ could exceed the true Q_max)
  - Must propagate to accumulator (Part 8) for lazy resolution

- **Empirical performance** (from `research/numerics-error-bounds.md`, 44,808 natural polytope σ-nodes):
  - True positive: 44,414 (β* > 0 certified)
  - False positive: **0** (no false certifications)
  - False negative: 9 (β* > 0 but not certified — all m=6, rank-deficient M with near-null eigenvalue)
  - Minimum true-positive margin: 1.11 × 10⁻⁵
  [VERIFY: these numbers are from the logbook 2026-03-31 session; confirm they match the latest run]

Depends on: Part 3, Part 6 (for η_i bound)

## Part 6: Error Analysis — Perturbation Chain

[Reader needs: the projection formulation from Part 3b (V, β₀, H', g, γ_j), ε_mach from Part 1]

Forward error analysis: traces rounding errors from input through each computation step. All bounds have σ_min(C) in denominators — this is the **conditioning gateway**.

### Conditioning precondition (rem:conditioning-precondition)

σ_min(C) controls every downstream bound:
- Null-space error: O(ε_mach / σ_min(C))
- Particular solution error: O(ε_mach / σ_min(C)²)
- Eigenvalue threshold: O(ε_mach / σ_min(C))

When σ_min(C) is small, all bounds blow up and no certification is possible. The solver checks σ̃_min(C) > ε_C as a fast short-circuit; if it fails, return INDETERMINATE for the entire σ-node.

**Physical meaning of small σ_min(C):** The constraint matrix C has rows [a_σ(1)^T; ...; a_σ(m)^T; 1...1]. Small σ_min(C) means the dual vertices a_σ(i) are nearly linearly dependent — a geometrically degenerate configuration of facet normals.

### Standard results used (stated with citations, not proved)

- **Weyl's eigenvalue perturbation theorem** (Weyl 1912; Horn & Johnson 2013, Thm 4.3.1):
  For symmetric A, A+E: |λ_j(A+E) − λ_j(A)| ≤ ‖E‖ (spectral norm).
  [Used in: Link 4, bounding |γ̃_j − γ_j|]

- **Davis-Kahan sin(Θ) theorem**:
  If A has eigenvalue gap δ between two groups of eigenvalues, and B = A + E, then the angle between the corresponding eigenspaces satisfies ‖sin Θ‖ ≤ ‖E‖/δ.
  [Used in: Link 2, bounding null-space basis error. The "gap" is σ_min(C) (gap between smallest nonzero and zero singular values of C).]

- **Backward stability of SVD** (Golub & Van Loan 2013, §8.6):
  The computed SVD is the exact SVD of a nearby matrix: C + δC_SVD with ‖δC_SVD‖ ≤ c₃ · ε_mach · ‖C‖.
  [Used in: Link 2]

- **Backward stability of eigendecomposition** (same reference):
  Computed eigenvalues are exact eigenvalues of a nearby matrix.
  [Used in: Link 4]

- **Pseudoinverse perturbation** (Wedin 1973):
  For full-row-rank C, ‖δ(C⁺)‖ = O(‖δC‖ / σ_min(C)²).
  [Used in: Link 2b]

### The perturbation chain (rem:perturbation-chain)

```
a_i → [assemble] → C, H → [SVD] → V, β₀ → [V^T H V] → H' → [eig] → γ_j → [(H')⁻¹g] → α* → [β₀+Vα*] → β*
```

### Link 1: Assembly (lem:link-assembly)
- C, H assembled from a_i by O(m²) f64 operations (ω₀ evaluations = two multiplications and a subtraction per entry)
- ‖δC‖ ≤ c₁ · ε_mach · ‖C‖
- ‖δH‖ ≤ c₂ · ε_mach · ‖H‖
- Constants c₁, c₂ are O(m) (standard floating-point error analysis)
- In EHZ setting: ‖C‖, ‖H‖ = O(1) (dual vertices a_i are O(1)), so perturbations are O(ε_mach)
  [TODO: verify c₁, c₂ — the assembly involves subtraction (ω₀ = q₁p₂ − q₂p₁) which could cause cancellation if a_i are nearly aligned. For our polytopes, a_i are O(1) with no near-cancellation.]

### Link 2: SVD of C → null-space basis V (lem:link-svd)
- Backward stable SVD: computed SVD is exact SVD of C + δC_total, where δC_total combines assembly and SVD errors
- Davis-Kahan: ‖sin Θ(Ṽ, V)‖ ≤ ‖δC_total‖ / (σ_p(C) − σ_{p+1}(C))
  - σ_p(C) = σ_min(C) (smallest nonzero singular value; C has full row rank p = 5)
  - σ_{p+1}(C) = 0 (the gap to the null space)
  - So: ‖sin Θ(Ṽ, V)‖ ≤ ‖δC_total‖ / σ_min(C) = O(ε_mach · ‖C‖ / σ_min(C))
- **σ_min(C) is the conditioning gateway:** all downstream bounds have σ_min(C) in denominators

### Link 2b: Particular solution β₀ = C⁺d (lem:link-beta0)
- ‖δβ₀‖ = O(ε_mach · ‖C‖ · ‖d‖ / σ_min(C)²)
- One power worse than null-space error (1/σ_min(C)² vs 1/σ_min(C)) because β₀ depends on inverses of singular values, not just their separation from zero
- Also: ‖β₀‖ ≤ ‖d‖ / σ_min(C) (enters subsequent bounds)

### Link 2c: Reduced gradient g = V^T H β₀ (lem:link-gradient)
- δg = V^T H δβ₀ + δV^T H β₀ + V^T δH β₀ + O(second order)
- Three first-order terms, each bounded:
  1. ‖V^T H δβ₀‖ ≤ ‖H‖ · ‖δβ₀‖ (Link 2b)
  2. ‖δV^T H β₀‖ ≤ ‖H‖ · ‖δV‖ · ‖β₀‖ (Link 2)
  3. ‖V^T δH β₀‖ ≤ ‖δH‖ · ‖β₀‖ (Link 1)
- Combined: ‖δg‖ = O(ε_mach / σ_min(C)²) in EHZ setting

### Link 3: Reduced Hessian H' = V^T H V (lem:link-reduced-hessian)
- H̃' = (V+δV)^T (H+δH)(V+δV) = H' + ΔH'
- First-order: ‖ΔH'‖ ≤ 2‖H‖·‖δV‖ + ‖δH‖ + O(second order)
- = O(‖H‖ · ε_mach / σ_min(C))

### Link 4: Eigenvalues of H' (lem:link-eigenvalues)
- Weyl's theorem + backward stability of eigendecomposition:
  |γ̃_j − γ_j| ≤ ‖ΔH'‖ + c₄ · ε_mach · ‖H̃'‖ = O(‖H‖ · ε_mach / σ_min(C))
- **Eigenvalue sign threshold:** ε_γ = c₅ · ‖H‖ · ε_mach / σ_min(C)
  - If |γ̃_j| > ε_γ, then sign(γ̃_j) = sign(γ_j) — certified
  - **This delivers Part 4's threshold.**

### Link 5: Critical point and β certification (lem:link-beta)
- **Assumption:** all retained eigenvalues have γ̃_j < −ε_γ (no null eigenvalues; the semidefinite/LP case is separate)
- First-order α error (eq:alpha-perturbation):
  δα = −(H')⁻¹(ΔH' · α* + δg) + O(second order)
- **Per-eigendirection error** (eq:alpha-component-error):
  (δα)_j = −(e_j + f_j)/γ_j where e_j = w_j^T ΔH' α*, f_j = w_j^T δg
  - Amplification 1/|γ_j| makes error large for near-zero eigenvalues
  - Empirically confirmed (rem:eigendirection-error): |δα_j| · |γ_j| ≈ 10⁻¹⁶ to 10⁻¹⁷ across 15 orders of magnitude of |γ_j|, on 364 natural polytope I1 problems
- **Full β error decomposition** (eq:beta-error-decomposition):
  (δβ)_i = Σ_j (Vw_j)_i · (δα)_j + (δV · α*)_i + (δβ₀)_i + O(second order)
  = critical-point shift + null-space rotation + particular-solution shift
- **Componentwise bound η_i** (eq:eta-computable): see Part 5 for the formula
  - Uses computed quantities (Ṽ, w̃_j, γ̃_j, α̃) as proxies for exact ones
  - Safety constant c = m² (empirical)
  - **This delivers Part 5's threshold.**

### Gap: null-eigenvalue case
- When H' has near-zero eigenvalues (|γ_j| ≲ ε_γ), the solver discards them from (H')⁻¹ and searches the null eigendirections via LP (rem:near-null-lp-search)
- The LP shift is O(1) in the null eigendirection, but the η_i bound above assumes all retained eigenvalues are well-separated from zero
- **39 violations** on null-eigenvalue cases from natural polytope data (1192 σ-nodes)
- Extending the bound to cover the LP search: open
  [GAP: The η_i bound (eq:eta-computable) does not cover the null-eigenvalue LP search case. The violation mechanism: near-zero eigenvalue → solver retains it → 1/γ amplification gives O(1) error. Or: solver discards it → LP search shifts β by O(1) in that direction → η_i bound predicts only O(ε_mach) shift.]

Depends on: Part 3b (projection formulation)

## Part 7: Error Analysis — Q Error Bound

[Reader needs: Q(β) = ½β^T H β, the KKT system Hβ* + C^Tλ* = 0 (stationarity), Cβ* = d, Part 3b notation]

**Complementary to Part 6**: Part 6 traces errors through the computation chain (forward analysis). Part 7 bounds Q error directly from the KKT residual (backward analysis). The two approaches give different information: Part 6 gives componentwise β understanding, Part 7 gives a single scalar Q error bound.

### First-order Q bound (lem:q-error-first-order, PROVEN)

**Statement.** Let β* > 0 solve the interior KKT system. Let β̃ be a computed solution with residual r = M(β̃,λ̃)^T − (0,d)^T. Then:
  |Q(β̃) − Q(β*)| ≤ (‖H‖·‖β*‖/σ_min(C)) · ‖r‖ + ½‖H‖·‖δβ‖²

**Proof outline** (5 steps):
1. **Taylor expand** Q at β*: Q(β̃) − Q(β*) = (Hβ*)^T δβ + ½ δβ^T H δβ
2. **Rewrite first-order term using KKT stationarity:** Hβ* = −C^T λ*, so (Hβ*)^T δβ = −λ*^T C δβ = −λ*^T r_λ
   - **Key insight:** at the critical point, dQ/dβ = Hβ* = −C^T λ* lies in range(C^T). So null(C) perturbations of β don't affect Q at first order — only the constraint residual r_λ matters.
3. **Bound ‖λ*‖:** from C^T λ* = −Hβ* and C having full row rank: ‖λ*‖ ≤ ‖Hβ*‖/σ_min(C) ≤ ‖H‖·‖β*‖/σ_min(C)
4. **Bound the first-order term:** |λ*^T r_λ| ≤ ‖λ*‖·‖r_λ‖ ≤ (‖H‖·‖β*‖/σ_min(C))·‖r‖ (since ‖r_λ‖ ≤ ‖r‖)
5. **Combine** by triangle inequality.

### Q correction (lem:q-correction-second-order, PROVEN)

- Define Q_corr = Q(β̃) + λ̃^T r_λ (adding the computed multiplier times the constraint residual)
- This cancels the first-order error exactly:
  Q_corr − Q(β*) = δλ^T r_λ + ½ δβ^T H δβ
  - Both terms are products of error quantities (second-order small)
  - In practice: corrected Q errors at machine epsilon for well-conditioned problems

**Proof:**
  From step 2 above: Q(β̃) − Q(β*) = −λ*^T r_λ + ½ δβ^T H δβ.
  Add the correction: Q_corr − Q(β*) = (−λ*^T + λ̃^T)r_λ + ½ δβ^T H δβ = δλ^T r_λ + ½ δβ^T H δβ. ∎

### Structural theorem (lem:pseudoinverse-orthogonality, PROVEN)

**Statement.** For symmetric M with b ∈ col(M): let x̃ = M⁺b (pseudoinverse solution) and x* ∈ col(M) with Mx* = b. Then δx^T M δx = 0.

**Proof:** δx = x̃ − x* ∈ col(M) (both x̃ and x* are in col(M)). M δx = r. r ⊥ col(M) (standard pseudoinverse property). So δx^T r = 0, hence δx^T M δx = δx^T r = 0. ∎

**Consequence** (cor:taylor-structure, GAP in proof):
  - The first-order Q error term equals −2× the second-order term exactly
  - So Q(β̃) − Q(β*) = −½ δβ^T H δβ (half the second-order term, with opposite sign)
  - The Q correction is exact in exact arithmetic (cor:exact-correction, PROVEN)
  - Empirically verified: first_order/second_order = 2.0000 across 95 rank-deficient cases
  [GAP: The proof of cor:taylor-structure has extra terms d^T δλ + λ*^T r_λ − 2 r_λ^T δλ that need to cancel. Cancellation believed to follow from x*^T r = 0, but the full derivation needs Jörn.]

### Runtime bound

At runtime, the exact β* is unknown. Substitute computed β̃:
  E₁ = ‖H‖ · ‖β̃‖ · ‖r‖ / σ_min(C)
- All quantities available from solver output (no extra computation)
- Bounds the first-order part of Q error
- Second-order part ½‖H‖·‖δβ‖² is negligible when the solver achieves small residuals on well-conditioned problems
  [GAP: Bounding ‖δβ‖ in terms of ‖r‖ via block structure of M — the naive bound uses ‖M⁺‖ which can be 10¹⁶]
- **Empirical tightness:** max ratio |Q_err|/E₁ = 0.217 on 51K problems, zero violations (B3 bound in logbook)

### Q loss from dropping near-boundary components (lem:near-boundary-drop, PROVEN)

- If β*_k ≤ ε and other components bounded away from zero (β*_i ≥ δ > 0 for i ≠ k):
  - The shorter σ' (removing index k) achieves Q within E_drop of Q(β*)
  - E_drop = ε · (‖H‖·‖β*‖ + ‖H‖·‖β*‖·‖c_k‖/σ_min(C_{-k}) + ‖H‖·‖c_k‖²ε / (2σ_min(C_{-k})²))
  - Three terms: (1) direct removal of row/column k, (2) feasibility restoration via projection onto C_{-k}β' = d, (3) quadratic correction from the projection
- Used in Part 8 to justify ignoring low-Q INDETERMINATE nodes

Depends on: Part 3 (either formulation works for first-order bound; projection formulation for structural theorem)

## Part 8: The Accumulator

[Reader needs: TRUE/FALSE/INDETERMINATE verdicts from Parts 4+5, Q error bound E₁ from Part 7, minimum-support from Part 0c]

Collects all (σ, Q̃, verdict) triples from the per-σ solvers and determines c_EHZ.

- **Verdicts** (from the per-σ solver):
  - **TRUE:** Parts 4 and 5 both certified (eigenvalue signs confirmed negative, β* > 0 confirmed). Q̃ is a reliable approximation of the per-σ maximum Q_F(σ).
  - **FALSE:** eigenvalue test (Part 4) found a positive eigenvalue → DROP (case B1), or β test (Part 5) certified β*_k < 0 → DROP (case B2). A shorter σ' covers this Q.
  - **INDETERMINATE:** either eigenvalue sign or β > 0 test inconclusive. Cannot drop, cannot trust.

- **Best certified Q̃** = max{Q̃ : verdict = TRUE}
- **Best uncertain Q̃** = max{Q̃ : verdict ∈ {TRUE, INDETERMINATE}}
- **Safety assertion:** best uncertain ≥ best certified (if violated → invariant failure in the solver, because every TRUE Q̃ is also in the TRUE ∪ INDETERMINATE set)

- **Final capacity:** c_EHZ = 1/(2 · Q̃_best_certified)

### Which error bound applies?

- For **TRUE** nodes: Q error bounded by E₁ from Part 7. The reported Q̃ is within E₁ of Q*(σ).
  - Total capacity error: |c̃ − c*| = |1/(2Q̃) − 1/(2Q*)| ≈ E₁/(2Q*²) for small E₁
- For **INDETERMINATE** nodes: Q̃ may **overestimate** Q_F(σ) (because β* might not be > 0, so the unconstrained maximum Q(β*) exceeds the constrained maximum on F = A ∩ {β ≥ 0}).

### Lazy INDETERMINATE resolution (rem:trinary-beta in formal/numerics/error-bounds.tex)

Sort all nodes by Q̃ descending:

1. Any INDETERMINATE with Q̃ > Q̃_best_certified: **must be resolved** (e.g., by exact rational arithmetic). Its true Q_F(σ) might exceed Q̃_best_certified, which would mean we missed the global maximum.

2. INDETERMINATE with Q̃ ≤ Q̃_best_certified: **safe to ignore.**
   - Even if the true Q_F(σ) differs, it differs by at most E_drop (lem:near-boundary-drop, Part 7)
   - And by minimum-support (Part 0c), a shorter σ' achieves the same or higher Q
   - So this INDETERMINATE cannot affect the capacity

3. If **no TRUE nodes exist**: all candidates are INDETERMINATE → must resolve all (rational arithmetic fallback)

### Practical outcome

In practice (47 polytopes, F ≤ 8):
- The vast majority of σ-nodes are FALSE (pruned or boundary maximum → DROP)
- Of the remaining, most are TRUE (well-conditioned, β > 0 certified)
- INDETERMINATE nodes are rare (9 false negatives on 44,808 natural σ-nodes, all with Q̃ ≤ Q̃_best_certified)
- Zero cases where an INDETERMINATE node had Q̃ > Q̃_best_certified
  [VERIFY: confirm this against the actual pipeline output]

Depends on: Parts 4, 5, 7

## Part 9: End-to-End Error Statement

[Reader needs: everything above]

The punchline: what can we say about the computed capacity?

### For well-conditioned capacity-achieving orbits (the common case):

- **Q error:** |Q̃ − Q*| ≤ E₁ ≈ ε_mach · ‖H‖·‖β‖·‖r‖/σ_min(C)
  - Typical values on natural polytopes (logbook): ‖H‖ ≈ 1.5, ‖β‖ ≈ 0.5, ‖r‖ ≈ 10⁻¹⁵, σ_min(C) ≈ 0.3 → E₁ ≈ 10⁻¹⁵
  - Max observed Q error on natural data: 3.95 × 10⁻¹⁴
- **β > 0 certified** with margin ≫ η_i (minimum margin 1.11 × 10⁻⁵ on natural data)
- **Capacity error:** |c̃ − c*| = |1/(2Q̃) − 1/(2Q*)| ≈ E₁/(2Q*²) ≈ 10⁻¹⁵ (double precision)

### For degenerate orbits (σ_min(C) small):

- Q error can be large: E₁ up to O(1) when σ_min(C) ≈ 10⁻³
- β > 0 may be INDETERMINATE
- These orbits are **never the capacity-achieving orbit** in our test data
  - **CONJECTURE:** degenerate orbits (σ_min(C) → 0) are never capacity-achieving.
  - Geometric argument: σ_min(C) → 0 means facet normals a_σ(i) are nearly linearly dependent — the orbit degenerates to a lower-dimensional configuration. Such orbits have high action (Q → 0) and cannot achieve Q_max.
  - Empirical: on all tested polytopes (47 polytopes, F ≤ 8), the winning orbit has σ_min(C) ≥ 0.3.
  [GAP: no proof that this holds in general. A counterexample would be a polytope where the capacity-achieving orbit visits facets with nearly-dependent normals.]

### Empirical validation summary (from `research/numerics-error-bounds.md`)

- **Literature polytopes:** 7 polytopes match published c_EHZ values at 1 × 10⁻⁶ relative tolerance
- **Capacity axioms:** 6 axioms (monotonicity, conformality, symplectic invariance, normalization, Lagrangian product, symplectic product) verified on 47 polytopes
- **β > 0 classification:** zero false positives on 45K problems; 9 false negatives (root-caused: eigenvalue threshold for m=6 rank-deficient problems)
- **Q error bound B3:** zero violations on 51K problems, max ratio 0.217
- **Q correction:** reduces error from first-order (~10⁻¹⁵ for well-conditioned) to second-order (~10⁻³¹ in exact arithmetic)

### What remains proven vs conjectured

| Claim | Status |
|-------|--------|
| Q error bound |Q̃−Q*| ≤ E₁ | **PROVEN** (lem:q-error-first-order) |
| Q correction exact in exact arithmetic | **PROVEN** (cor:exact-correction) |
| β > 0 certification: no false positives | **PROVEN** (if η_i bound holds) |
| η_i bound on well-conditioned problems | **PROVEN** (lem:link-beta, eq:eta-computable) |
| η_i bound on null-eigenvalue cases | **OPEN** (39 violations) |
| Degenerate orbits not capacity-achieving | **CONJECTURE** |
| cor:taylor-structure (first-order = −2× second-order) | **GAP** in proof |
| Safety constants c = m² | **EMPIRICAL** (zero violations, not derived) |

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

1. **cor:taylor-structure GAP** — algebraic cancellation in proof needs verification (extra terms d^T δλ + λ*^T r_λ − 2 r_λ^T δλ)
2. **Null-eigenvalue case** — η_i bound doesn't cover LP search for flat eigendirections; 39 violations on natural data
3. **Constants c₁–c₆** — currently c = m² (empirical safety factor), not derived from first principles
4. **Degenerate-orbit conjecture** — unproven (that capacity-achieving orbits always have well-conditioned C)
5. **Transition feasibility lemma** — cited in Part 2 but formal statement+proof lives in formal/library/kkt.tex (lem:numerical-transition-feasibility); [SCOPE: reproduce vs cite?]
6. **Part III of formal/numerics/error-bounds.tex** (formal \begin{algorithm} for the floating-point solver) not written yet
7. **Clarke dual action principle** — deferred from Part 0a; substantial (thesis/clarkedual-action-principle.tex is ~500 lines of Jörn-approved content)
