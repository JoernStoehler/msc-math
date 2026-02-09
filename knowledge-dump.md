# Knowledge Dump: HK2017 Algorithm Writeup

This file is the single source of truth for writing ALGORITHM.md sections.
Section writers: use ONLY this file for mathematical content. Do not invent formulas or claims.

---

## Conventions

**Coordinates:** (q₁, q₂, p₁, p₂) in R⁴, matching HK2017 and the MATLAB implementation.

**Symplectic form:** ω(u,v) = ⟨Ju, v⟩ = Σᵢ (u_qᵢ v_pᵢ - u_pᵢ v_qᵢ), equivalently ω = dq₁∧dp₁ + dq₂∧dp₂.

**Complex structure:** J: R⁴ → R⁴, J(q₁,q₂,p₁,p₂) = (-p₁,-p₂,q₁,q₂). Matrix:
```
J = [ 0   0  -1   0 ]
    [ 0   0   0  -1 ]
    [ 1   0   0   0 ]
    [ 0   1   0   0 ]
```
Properties: J² = -I, Jᵀ = -J, ω(u,v) = ⟨Ju, v⟩.

**Liouville 1-form:** λ₀ = ½⟨Jx, dx⟩ = ½Σᵢ(pᵢ dqᵢ - qᵢ dpᵢ), with dλ₀ = ω.

**Two normalizations used:**
- **Talk normalization** (used in proofs): curves on [0,T], constraint A(z)=T, I_K(z)=T at minimizers.
- **HK2017 normalization** (used in formula/algorithm): curves on [0,1], constraint ∫⟨-Jż,z⟩=1, I_K = 1 at pure Reeb velocities.

We prove everything in talk normalization, then derive the HK2017 formula via a normalization bridge at the end.

---

## Section A: Definitions

### A1. Convex polytope in H-representation

K ⊂ R⁴ is a convex polytope with 0 ∈ int(K):

K = ∩ᵢ₌₁ᶠ { x ∈ R⁴ : ⟨x, nᵢ⟩ ≤ hᵢ }

- F = number of facets (3-dimensional faces)
- nᵢ ∈ S³ = outward unit normal to facet Fᵢ
- hᵢ = h_K(nᵢ) > 0 = oriented height (positive since 0 ∈ int K)

### A2. Support function

h_K(y) = sup_{x∈K} ⟨x, y⟩

Properties: positively 1-homogeneous, convex. h_K(nᵢ) = hᵢ.

### A3. Gauge function (Minkowski functional)

g_K(x) = inf { r > 0 : x/r ∈ K }

Properties: positively 1-homogeneous, convex. g_K(x) = 1 on ∂K, g_K(x) < 1 in int(K).

### A4. Fenchel duality (g_K² and ¼h_K²)

**Theorem (Legendre-Fenchel duality).**
g_K²(x) = sup_y ( ⟨x,y⟩ - ¼h_K²(y) )
¼h_K²(y) = sup_x ( ⟨x,y⟩ - g_K²(x) )

**Fenchel inequality (pointwise):**
g_K²(x) + ¼h_K²(y) ≥ ⟨x, y⟩

**Equality condition:**
g_K²(x) + ¼h_K²(y) = ⟨x, y⟩  ⟺  y ∈ ∂g_K²(x)  ⟺  x ∈ ∂(¼h_K²)(y)

This is the key tool for Clarke's dual action principle.

### A5. Outward normal cone

For x ∈ ∂K:
N_K(x) = R₊ · conv{ nᵢ : x ∈ Fᵢ }

At interior of a facet: N_K(x) = R₊ · nᵢ (single ray).
At edges/vertices: a cone spanned by the normals of all adjacent facets.

### A6. Facet Reeb vectors

pᵢ = (2/hᵢ) J nᵢ

Derivation: The Hamiltonian H = g_K² has gradient ∇H = (2/hᵢ)nᵢ on int(Fᵢ), so the Hamiltonian vector field is J∇H = (2/hᵢ)Jnᵢ = pᵢ.

Properties:
- pᵢ is tangent to Fᵢ (since ⟨Jnᵢ, nᵢ⟩ = ω(nᵢ, nᵢ) = 0)
- |pᵢ| = 2/hᵢ (inversely proportional to distance from origin)

### A7. Generalized closed characteristic on ∂K

A closed loop γ ∈ W^{1,2}([0,T], R⁴) with γ(0)=γ(T) such that:
1. Im(γ) ⊂ ∂K
2. γ̇(t) ∈ J N_K(γ(t)) = conv{ pᵢ : γ(t) ∈ Fᵢ } a.e.

This is a differential inclusion (not equation) because ∂K is not smooth.

### A8. Symplectic action

A(γ) = ½ ∫₀ᵀ ⟨Jγ(t), γ̇(t)⟩ dt = ½ ∫₀ᵀ ⟨-Jγ̇(t), γ(t)⟩ dt = ∫_γ λ₀

Geometric meaning: symplectic area enclosed by γ.

**Key identity for piecewise-constant velocity (HK2017 Prop 3.4):**
If ż is piecewise constant with velocity wᵢ on interval Iᵢ = (τᵢ₋₁, τᵢ), then:

∫₀ᵀ ⟨-Jż, z⟩ dt = Σ_{j<i} |Iᵢ| |Iⱼ| ω(wᵢ, wⱼ)

(Note: 2A(z) = the left side for a centered loop.)

### A9. EHZ capacity

c_EHZ(K) = min { A(γ) : γ generalized closed characteristic on ∂K }

Properties: symplectically invariant, translation invariant, 2-homogeneous (c_EHZ(λK) = λ²c_EHZ(K)).

### A10. Dual functional

I_K(z) = ¼ ∫₀ᵀ h_K²(-Jż(t)) dt

Depends only on the velocity ż, not the position z. Translation invariant.

### A11. Identity: ω(Ju, Jv) = ω(u, v)

Proof via ω(a,b) = ⟨Ja, b⟩:
ω(Ju, Jv) = ⟨J²u, Jv⟩ = ⟨-u, Jv⟩ = -⟨Jᵀu, v⟩ = -⟨-Ju, v⟩ = ⟨Ju, v⟩ = ω(u, v)

Uses: J² = -I, Jᵀ = -J.

---

## Section B: Clarke's Dual Action Principle

### B1. Statement

**Primal problem:** Minimize A(γ) over generalized closed characteristics γ on ∂K.

**Dual problem (talk normalization):** Minimize I_K(z) over:
- z ∈ W^{1,2}([0,T], R⁴), z(0)=z(T)
- ∫₀ᵀ ż dt = 0 (closed loop)
- ∫₀ᵀ z dt = 0 (centered)
- A(z) = T (action = period)

**Theorem (Clarke's dual action principle).**
The primal and dual minimizers correspond 1:1, with z = γ - center(γ) and I_K(z) = T = A(γ).

In particular: c_EHZ(K) = T = I_K(z*) = A(γ*) where z* is the dual minimizer and γ* is the corresponding primal minimizer.

### B2. Proof (following the talk, with Fenchel duality calculations shown)

**Step 1: Fenchel inequality.**
From A4, for any x, y ∈ R⁴:
g_K²(x) + ¼h_K²(y) ≥ ⟨x, y⟩

**Step 2: Apply to Hamiltonian orbit.**
For a generalized closed characteristic γ on ∂K with H = g_K², the differential inclusion gives:
-Jγ̇(t) ∈ ∂g_K²(γ(t)) a.e.

Set x = γ(t), y = -Jγ̇(t). By the equality condition of Fenchel duality:
g_K²(γ(t)) + ¼h_K²(-Jγ̇(t)) = ⟨γ(t), -Jγ̇(t)⟩

**Step 3: Integrate over time.**
∫₀ᵀ g_K²(γ(t)) dt + ¼∫₀ᵀ h_K²(-Jγ̇(t)) dt = ∫₀ᵀ ⟨γ(t), -Jγ̇(t)⟩ dt

The right side is 2A(γ). The second term on the left is I_K(γ). So:
∫₀ᵀ g_K²(γ) dt + I_K(γ) = 2A(γ)

**Step 4: Use g_K ≡ 1 on ∂K.**
Since Im(γ) ⊂ ∂K, we have g_K(γ(t)) = 1, so g_K²(γ(t)) = 1. Therefore ∫g_K² dt = T.
T + I_K(γ) = 2T  ⟹  I_K(γ) = T = A(γ)

**Step 5: Critical point correspondence.**
From variational calculus, critical points of A subject to Im(γ)⊂∂K satisfy:
-Jγ̇(t) ∈ ∂g_K²(γ(t)) a.e., g_K²(γ) ≡ 1

Critical points of I_K subject to the dual constraints satisfy:
z(t) + const ∈ ∂(¼h_K²)(-Jż(t)) a.e., ∫⟨-Jż, z⟩ dt = 2T

By the Fenchel equality condition (y ∈ ∂g_K²(x) ⟺ x ∈ ∂(¼h_K²)(y)), these two inclusions are equivalent under z = γ - center(γ). The constraint correspondences also match.

### B3. Significance

The dual problem is easier because:
- I_K depends only on velocity ż, not position z
- No position constraint (no Im(z) ⊂ ∂K)
- No differential inclusion constraint
- The constraint set is more amenable to approximation and rearrangement arguments

---

## Section C: Simple Orbit Structure (Theorem 1)

### C1. Statement

**Theorem (HK2017 Thm 1.2).** For every convex polytope K ⊂ R⁴, there exists a minimum-action generalized closed characteristic γ* such that:
- γ* is piecewise affine
- γ̇*(t) is a pure facet Reeb vector (not a convex combination) on each piece
- For each facet i, the set {t : γ̇*(t) = c·Jnᵢ for some c>0} is a contiguous interval or empty

We call such an orbit a "simple orbit." It visits each facet at most once, in some cyclic order.

### C2. Proof: 5-step structure

**Step 1: Approximate.**
Start with a minimizer z of I_K in the dual problem (talk normalization: A(z)=T, I_K(z)=T).
Approximate z in W^{1,2} by piecewise affine loops z_N with:
- ż_N(t) ∈ conv{pᵢ} a.e. (velocities in convex hull of Reeb vectors)
- A(z_N) → A(z) = T and I_K(z_N) → I_K(z) = T

Uses: HK2017 Lemma 4.2 (piecewise affine approximation inside a convex hull).

**Step 2: Split.**
Replace each mixed-velocity segment (convex combination of Reeb vectors) by a concatenation of segments with pure velocities.

Splitting mechanism: If v(t) = Σ aᵢ(t)·Xᵢ with Σaᵢ(t)=1, replace by pure segments of duration Aᵢ = ∫aᵢ(t)dt for velocity Xᵢ. Total time preserved: ΣAᵢ = ∫Σaᵢ(t)dt = ∫1 dt = T.

Key mechanism (action): The time ordering of the pure segments affects the action. Reversing the order reverses the change in action (sign-flip). Choose the order that changes A by +ε:
A(z_N') = A(z_N) ± ε (we pick the + sign)

I_K = T exactly after splitting: For pure Reeb velocity pᵢ = (2/hᵢ)Jnᵢ:
- -Jż = (2/hᵢ)(-J²)nᵢ = (2/hᵢ)nᵢ
- h_K(-Jż) = (2/hᵢ)·h_K(nᵢ) = (2/hᵢ)·hᵢ = 2
- h_K²(-Jż) = 4 everywhere on every segment
- I_K = (1/4)·∫₀ᵀ 4 dt = T

**Step 3: Rearrange (grow+shrink).**
After splitting, ż takes values in a finite set of pure Reeb vectors. If the same Reeb vector Rᵢ appears in disjoint intervals, merge them.

Key mechanism: ABAD can become AABD or BAAD (moving one A-block to be adjacent to the other). The area differences are negatives of each other:
area(AABD) - area(ABAD) = -(area(BAAD) - area(ABAD))

So one option has A ≥ original (or both equal). Choose that one.

I_K is unchanged (depends only on velocity magnitudes and total time per velocity, not ordering).

**Step 4: Renormalize.**
After steps 2-3, action changed: A(z_N'') = A(z_N) ± ε.

Time rescaling by β = T / A(z_N''):
- Velocities unchanged (still pure Reeb vectors)
- Each time interval: ΔTₖ → β·ΔTₖ
- New total time: T' = β·T = T²/A(z_N'')

Consequences:
- A' = β²·A(z_N'') = T²/A(z_N'') = T', so talk normalization A'=T' restored
- I_K' = β·I_K(z_N'') = β·T = T', so I_K = T' as well

In the limit (N→∞): A(z_N) → T, so β → 1, T' → T. Vanishing perturbation.

**Step 5: Compactness.**
A simple loop is encoded by finite data: (σ, |Iᵢ|), where σ is the facet ordering and |Iᵢ| are segment lengths.
- The set of orderings is finite (≤ F! options)
- The set of segment lengths satisfying the constraints is compact (bounded and closed)

So the sequence {z_N'''} lives in a compact space. Extract a convergent subsequence.

In the limit: I_K(z*) = T* = T (since I_K(z_N''') = T_N''' → T), and I_K(z*) = I_K(z) (since z was a minimizer). So z* is a simple minimizer.

---

## Section D: Combinatorial Capacity Formula (Theorem 2)

### D1. The Q-function

Q(σ, β) = Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)})

where σ is a permutation of the facets and β = (β₁,...,β_F) are weights.

### D2. The constraint set M(K)

M(K) = { β ∈ R^F : βᵢ ≥ 0, Σβᵢhᵢ = 1, Σβᵢnᵢ = 0 }

Three constraints:
1. Non-negativity: each facet has non-negative weight
2. Height normalization: Σβᵢhᵢ = 1 (HK2017 normalization)
3. Closure: Σβᵢnᵢ = 0 (orbit is closed)

### D3. Statement

**Theorem (HK2017 Thm 1.1).**
c_EHZ(K) = (1/2) [max_{σ∈S_F, β∈M(K)} Q(σ,β)]⁻¹

### D4. Derivation

By Theorem 1, we can restrict to simple orbits: γ visits facets in order σ, spending time Tᵢ on facet σ(i), with velocity p_{σ(i)} = (2/h_{σ(i)}) J n_{σ(i)}.

**Step A: Action of a simple orbit (talk normalization).**

From the piecewise-constant velocity identity:
2A(z) = ∫₀ᵀ ⟨-Jż, z⟩ dt = Σ_{j<i} |Iᵢ| |Iⱼ| ω(wᵢ, wⱼ)

With wᵢ = p_{σ(i)} = (2/h_{σ(i)}) J n_{σ(i)}:

ω(wᵢ, wⱼ) = (4/(h_{σ(i)} h_{σ(j)})) · ω(Jn_{σ(i)}, Jn_{σ(j)})
            = (4/(h_{σ(i)} h_{σ(j)})) · ω(n_{σ(i)}, n_{σ(j)})     [using ω(Ju,Jv) = ω(u,v)]

So: 2A(z) = Σ_{j<i} Tᵢ Tⱼ · (4/(h_{σ(i)} h_{σ(j)})) · ω(n_{σ(i)}, n_{σ(j)})

**Step B: Variable substitution β_{σ(i)} = Tᵢ / h_{σ(i)}.**

Then Tᵢ = β_{σ(i)} · h_{σ(i)}, and:
- Closure: Σ Tᵢ p_{σ(i)} = 0 ⟹ Σ Tᵢ (2/h_{σ(i)}) Jn_{σ(i)} = 0 ⟹ Σ β_{σ(i)} n_{σ(i)} = 0
- Normalization: Σ β_{σ(i)} h_{σ(i)} = Σ Tᵢ = T

So in talk normalization: Σβᵢhᵢ = T.

**Step C: Action in terms of Q.**

2A(z) = Σ_{j<i} (β_{σ(i)} h_{σ(i)})(β_{σ(j)} h_{σ(j)}) · (4/(h_{σ(i)} h_{σ(j)})) · ω(n_{σ(i)}, n_{σ(j)})
       = 4 Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)})
       = 4 Q(σ, β)

So A(z) = 2Q(σ, β), and c_EHZ = T = A(z) = 2Q(σ, β) (in talk normalization with Σβᵢhᵢ = T).

**Step D: Normalization bridge to HK2017.**

In talk normalization: Σβᵢhᵢ = T, and c_EHZ = 2Q(σ,β).

Rescale: set β̃ᵢ = βᵢ/T, so Σβ̃ᵢhᵢ = 1 (HK2017 normalization).

Then Q(σ, β) = T² · Q(σ, β̃) (since Q is quadratic in β).

So c_EHZ = 2Q(σ,β) = 2T² · Q(σ,β̃).

Since c_EHZ = T: T = 2T² · Q(σ,β̃), hence Q(σ,β̃) = 1/(2T) = 1/(2·c_EHZ).

Therefore: c_EHZ = 1/(2·Q(σ,β̃)).

Maximizing Q over all σ and β̃ ∈ M(K) (with Σβ̃ᵢhᵢ=1) gives the capacity formula:

c_EHZ(K) = (1/2) [max Q(σ, β̃)]⁻¹

---

## Section E: Algorithm

### E1. Main algorithm

```
Input: polytope K in H-representation (normals nᵢ, heights hᵢ), with 0 ∈ int(K)
Output: c_EHZ(K)

best_q = 0

for each subset S ⊆ {1,...,F} with |S| ≥ 2:
    Build equality constraints: Aeq·β = beq
      Aeq = [ n_{i₁} ... n_{i_|S|} ]  (4 rows: closure in R⁴)
             [ h_{i₁} ... h_{i_|S|} ]  (1 row: normalization)
      beq = [0, 0, 0, 0, 1]ᵀ

    Check feasibility: if rank(Aeq) < rank([Aeq|beq]), skip S

    If Aeq is square and full-rank (β uniquely determined):
        Solve β = Aeq \ beq
        If all βᵢ > 0:
            For each cyclic ordering σ of S:
                Compute Q(σ,β) = Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)})
                Update best_q = max(best_q, Q(σ,β))

    If Aeq is underdetermined (free parameters in β):
        For each cyclic ordering σ of S:
            Build KKT system:
                [H   Aeq'] [β     ]   [0  ]
                [Aeq  0  ] [lambda] = [beq]
            where H is the symmetrized action matrix:
                H_{ij} = ω(n_{σ(i)}, n_{σ(j)}) for i > j
                H_{ij} = -ω(n_{σ(i)}, n_{σ(j)}) for i < j
                H_{ii} = 0

            If det(KKT matrix) < tolerance: skip (singular)
            Solve for β (discard lambda)
            If all βᵢ > 0:
                Compute Q(σ,β) = βᵀ H β / 2
                Update best_q = max(best_q, Q(σ,β))

return c_EHZ(K) = 1/(2 · best_q)
```

### E2. Why the KKT approach works

The KKT system solves: maximize Q(σ,β) subject to Aeq·β = beq (equality constraints only).

Non-negativity βᵢ ≥ 0 is NOT enforced in the KKT system — it is checked post-hoc.

Why this is correct: If the equality-constrained optimum has β_j < 0, the non-negativity-constrained optimum has β_j = 0, which corresponds to a smaller subset S' = S \ {j}. That subset will be explored in a separate iteration of the outer loop.

### E3. Cyclic symmetry

Q(σ,β) is invariant under cyclic rotations of σ. Fix σ(1) = first element of S, enumerate only (|S|-1)! orderings instead of |S|!.

### E4. The symmetrized action matrix H

The matrix H encodes the quadratic form Q for a given ordering σ:

Q(σ,β) = Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)}) = ½ βᵀ H β

where H is symmetric with H_{ij} = ω(n_{σ(i)}, n_{σ(j)}) for i > j (and H_{ij} = -ω(n_{σ(i)}, n_{σ(j)}) for i < j, since ω is skew). The factor ½ arises from the symmetrization.

### E5. Preprocessing: centering

The polytope must have 0 ∈ int(K) for heights to be positive. The MATLAB implementation centers K at its barycenter (volume-weighted centroid via Delaunay triangulation). Any interior point works; the barycenter is canonical.

### E6. Complexity

Outer loop: Σ_{j=2}^{F} C(F,j) subsets.
Inner loop: (j-1)! cyclic orderings per subset.
Total: Σ_{j=2}^{F} C(F,j) · (j-1)! = Σ_{j=2}^{F} F!/(j·(F-j)!) = O(F!/F).

Exponential in F (number of facets).

---

## Section F: Algorithm Variant — Graph-Pruned Enumeration

### F1. Facet adjacency graph

Build a directed graph G:
- Vertices: facets {1,...,F}
- Directed edge i→j iff ∃ x ∈ Fᵢ, c>0 such that x + c·pᵢ ∈ Fⱼ

In words: edge i→j means the Reeb flow starting somewhere on facet i can reach facet j.

### F2. Pruning

Replace the enumeration of all subsets × permutations by: enumerate directed cycles in G.

Only cycles in G correspond to geometrically realizable orbits.

### F3. Correctness

By Theorem 1, the minimum-action orbit visits facets in sequence. The transition from facet i to facet j requires the Reeb flow on facet i to reach facet j, which is exactly the edge condition in G.

### F4. Computational significance

The number of simple cycles in G can be much smaller than F!, especially for polytopes with sparse adjacency. (Though still exponential in the worst case.)

The MATLAB reference implementation does NOT implement this pruning.

---

## Style guidance for section writers

**Target audience:** Master thesis readers. Assume linear algebra, analysis, basic combinatorics (permutations). Do NOT assume familiarity with Fenchel duality, symplectic geometry, or convex analysis.

**Proof style:** Every step explicit. Cite the specific identity/property used. No "it is easy to see" or "one can show." If a step is non-trivial, say so and give the calculation.

**Format:** Markdown with Unicode math (ω, ∈, ≤, etc.). Use ```code blocks``` for the algorithm pseudocode only.

**Level of detail for each section:**
- Section A (Definitions): State each definition precisely. Include geometric meaning in 1-2 sentences. Include the derivation of facet Reeb vectors. Include the key identity ω(Ju,Jv) = ω(u,v) with full proof.
- Section B (Clarke duality): Full Fenchel duality calculations shown. State all hypotheses. This is the most detailed proof section.
- Section C (Simple orbit): 5-step proof. Each step: what happens, why it works, what changes to A and I_K.
- Section D (Capacity formula): Derivation from simple orbit. The normalization bridge is the key new content.
- Section E (Algorithm): Pseudocode + correctness argument for KKT approach.
- Section F (Graph variant): Brief. State the graph, the pruning, the correctness.

**Do NOT:**
- Add content not in this knowledge dump
- Change notation or conventions
- Add "remarks" or "notes" beyond what's specified
- Use LaTeX syntax (use Unicode instead)
- Include the systolic ratio (it's not part of the algorithm)
