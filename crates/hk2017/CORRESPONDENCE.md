# Notation and Definition Correspondence: Our Thesis vs HK2017 Paper

**Purpose:** Prevent notation mix-ups by documenting every correspondence between our conventions (from `knowledge-dump.md`) and the HK2017 paper (`papers/hk2017/EHZ-polytopes.tex`). Reviewed by Jörn before any further writing.

**Sources:**
- "Ours" = `knowledge-dump.md` (the single source of truth for the thesis chapter)
- "HK2017" = the paper `EHZ-polytopes.tex` by Pazit Haim-Kislev
- "MATLAB" = the reference implementation documented in `matlab-extraction.md`

---

## 1. Notation Table

| Concept | Our notation | HK2017 notation | Relationship / Notes |
|---------|-------------|-----------------|---------------------|
| **Ambient space** | R^4 (we work in n=2 only) | R^{2n} (general dimension) | We specialize to n=2. The paper's results hold for all n. |
| **Coordinates** | (q₁, q₂, p₁, p₂) | Not explicitly named; works with abstract R^{2n} | Our coordinate choice matches the MATLAB implementation. The paper never names coordinates. |
| **Standard complex structure** | J, with matrix `[[0,0,-1,0],[0,0,0,-1],[1,0,0,0],[0,1,0,0]]` | J, called "the standard complex structure" | SAME symbol. The paper never writes J as a matrix — it only says "standard complex structure in R^{2n}." Our explicit matrix is the standard convention for (q,p) coordinates: J(q,p) = (-p,q). |
| **Standard symplectic form** | ω(u,v) = ⟨Ju, v⟩ = Σᵢ(u_qᵢ v_pᵢ − u_pᵢ v_qᵢ) | ω, called "the standard symplectic structure" | SAME. The paper writes ω without a coordinate formula. Our formula ω(u,v) = ⟨Ju,v⟩ is the standard definition. |
| **Number of facets** | F | F_K (macro `\kF`, rendered as bold **F**_K) | Different symbol. The paper uses a bold-face F_K to emphasize dependence on K. We use plain F. |
| **Facets** | Fᵢ, i = 1,...,F | F_i, i = 1,...,F_K | SAME convention, different cap on index. |
| **Outward unit normals** | nᵢ ∈ S³ | n_i, unit outer normal to F_i | SAME. We explicitly say S³ (the unit sphere in R⁴); the paper says "unit outer normal" without naming the sphere. |
| **Oriented heights** | hᵢ = h_K(nᵢ) > 0 | h_i = h_K(n_i), with h_i > 0 when 0 ∈ int(K) | SAME. |
| **Support function** | h_K(y) = sup_{x∈K} ⟨x, y⟩ | h_K(x) = sup { ⟨y, x⟩ : y ∈ K } | SAME definition, different dummy variable names. Note: HK2017 Section 2.2 writes h_K(x) = sup { ⟨y,x⟩ ; y ∈ K }, swapping the role of x and y relative to our convention. The inner product is symmetric, so the definitions are identical. |
| **Gauge function** | g_K(x) = inf { r > 0 : x/r ∈ K } | g_K(x) = inf { λ : x/λ ∈ K } | SAME. Different dummy variable name (r vs λ). |
| **Hamiltonian** | H = g_K² (implicit, used in derivation of Reeb vectors) | g_K² (explicit, Section 2.1) | SAME. |
| **Outward normal cone** | N_K(x) = R₊ · conv{ nᵢ : x ∈ Fᵢ } | N_K(x) := R₊ conv{ n_i : x ∈ F_i } | SAME. |
| **Facet Reeb vectors** | pᵢ = (2/hᵢ) J nᵢ | p_i = J ∇(g_K²)(x) for x ∈ int(F_i), which equals (2/h_i) J n_i | SAME formula. We state the explicit formula directly; HK2017 defines p_i first as J∇(g_K²) and then derives p_i = (2/h_i) J n_i in Proposition 3.1. |
| **Closed characteristic (smooth case)** | Not explicitly defined (we go straight to the polytope case) | Embedded circle γ on ∂Σ with γ'(t) ∈ ker(ω\|_{∂Σ}), equivalently γ'(t) ∥ Jn | — |
| **Generalized closed characteristic** | γ ∈ W^{1,2}([0,T], R⁴), γ(0)=γ(T), Im(γ) ⊂ ∂K, γ̇(t) ∈ J N_K(γ(t)) a.e. | γ ∈ W^{1,2}([0,1], R^{2n}), Im(γ) ⊂ ∂K, γ̇(t) ∈ J N_K(γ(t)) a.e. (Definition 2.1) | **DIFFERENT DOMAIN.** We use [0,T]; HK2017 uses [0,1]. See conventions summary. |
| **Symplectic action** | A(γ) = ½ ∫₀ᵀ ⟨Jγ(t), γ̇(t)⟩ dt | A(γ) := ½ ∫₀ᵀ ⟨J γ(t), γ̇(t)⟩ dt | SAME formula. Note: HK2017 writes the action with domain [0,T] even though closed characteristics are defined on [0,1] — the paper uses T as the period. Our knowledge dump also writes ½⟨Jγ, γ̇⟩ and equivalently ½⟨−Jγ̇, γ⟩, both integrated over [0,T]. These are equal by integration by parts for closed loops. |
| **Liouville 1-form** | λ₀ = ½⟨Jx, dx⟩ = ½Σᵢ(pᵢ dqᵢ − qᵢ dpᵢ), with dλ₀ = ω | Not explicitly introduced | We introduce λ₀ to give A(γ) = ∫_γ λ₀ a geometric meaning. HK2017 does not use the Liouville form. |
| **EHZ capacity** | c_EHZ(K) = min { A(γ) : γ gen. closed char. on ∂K } | c_EHZ(K) = min { A(γ) : γ closed char. on ∂K } | SAME. |
| **Dual functional** | I_K(z) = ¼ ∫₀ᵀ h_K²(−Jż(t)) dt | I_K(z) = ¼ ∫₀¹ h_K²(−Jż(t)) dt | **DIFFERENT DOMAIN.** We integrate over [0,T]; HK2017 integrates over [0,1]. See normalization discussion. |
| **Dual constraint set E** | z ∈ W^{1,2}([0,T], R⁴), z(0)=z(T), ∫ż=0, ∫z=0, A(z)=T | z ∈ W^{1,2}([0,1], R^{2n}), ∫ż=0, ∫⟨−Jż,z⟩=1 | **DIFFERENT.** See normalization discussion. Our "talk normalization" has A(z)=T and ∫z=0; HK2017 has ∫⟨−Jż,z⟩=1 and no centering constraint. |
| **Piecewise-constant velocity identity** | 2A(z) = ∫⟨−Jż,z⟩ dt = Σ_{j<i} \|Iᵢ\| \|Iⱼ\| ω(wᵢ,wⱼ) | ∫⟨−Jż,z⟩ dt = Σ_{i=1}^m Σ_{j=1}^{i-1} \|I_j\| \|I_i\| ω(w_i,w_j) (Prop 3.4) | SAME identity, different index notation. We write Σ_{j<i}; HK2017 writes Σᵢ Σ_{j<i}. Note the asymmetry: the paper's sum has the LATER velocity (w_i) first in ω(w_i,w_j). |
| **Q-function** | Q(σ,β) = Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)}) | Same expression appears inside the capacity formula (Thm 1.1) | HK2017 does NOT name this function "Q." We introduced the name Q for convenience. |
| **Constraint set M(K)** | M(K) = { β ∈ R^F : βᵢ ≥ 0, Σβᵢhᵢ = 1, Σβᵢnᵢ = 0 } | M(K) = { (βᵢ)_{i=1}^{F_K} : βᵢ ≥ 0, Σβᵢhᵢ = 1, Σβᵢnᵢ = 0 } | SAME. |
| **Permutation group** | S_F (not explicitly named) | S_{F_K} | SAME concept, different subscript (F vs F_K). |
| **Capacity formula** | c_EHZ(K) = ½ [max_{σ,β∈M(K)} Q(σ,β)]⁻¹ | c_EHZ(K) = ½ [max_{σ∈S_{F_K}, β∈M(K)} Σ_{j<i} β_{σ(i)}β_{σ(j)}ω(n_{σ(i)},n_{σ(j)})]⁻¹ | SAME. We just abbreviate the sum as Q(σ,β). |
| **Variable substitution** | β_{σ(i)} = Tᵢ / h_{σ(i)} | Same: β_{σ(i)} = T_i / h_{σ(i)} (proof of Thm 1.1) | SAME. |
| **Symmetrized action matrix H** | H_{ij} = ω(n_{σ(i)}, n_{σ(j)}) for i>j, H_{ij} = −ω(n_{σ(i)}, n_{σ(j)}) for i<j, H_{ii}=0 | Not introduced as a separate object | We introduced H for the algorithm; HK2017 does not use matrix notation. The MATLAB constructs H via `H = A - triu(A) + tril(A)'`. |
| **Adjacency graph G** | Directed graph: vertex per facet, edge i→j iff ∃ x∈Fᵢ, c>0 with x+c·pᵢ∈Fⱼ | Same definition (Remark 3.11) | SAME. |
| **Fenchel duality** | g_K²(x) + ¼h_K²(y) ≥ ⟨x,y⟩, with equality iff y ∈ ∂g_K²(x) | 4⁻¹g_K² is the Legendre transform of h_K² (Section 2.2) | SAME relationship, different presentation. We state the pointwise inequality explicitly; HK2017 invokes Legendre duality by reference. |

---

## 2. Definitions Table

| Concept | Our definition | HK2017 definition | Same? | If different, how |
|---------|---------------|-------------------|-------|-------------------|
| **Convex polytope** | K ⊂ R⁴, 0 ∈ int(K), K = ∩ᵢ { x : ⟨x,nᵢ⟩ ≤ hᵢ } | K ⊂ R^{2n}, convex polytope with non-empty interior. Origin assumed in K for technical arguments. | Essentially same. We require 0 ∈ int(K) from the start; HK2017 initially only assumes non-empty interior (Thm 1.1) and adds 0 ∈ K in Section 2.1. The paper notes (Remark 3.8) that the formula is translation-invariant. |
| **Support function** | h_K(y) = sup_{x∈K} ⟨x,y⟩ | h_K(x) = sup { ⟨y,x⟩ ; y ∈ K } | YES | Same definition. Only dummy variable names differ (we use y for the argument, they use x). |
| **Gauge function** | g_K(x) = inf { r > 0 : x/r ∈ K } | g_K(x) = inf { λ : x/λ ∈ K } | YES | Same definition. Different dummy variable (r vs λ). |
| **Symplectic form** | ω(u,v) = ⟨Ju,v⟩ = Σᵢ(u_qᵢ v_pᵢ − u_pᵢ v_qᵢ) | ω is "the standard symplectic structure" on R^{2n} | YES | Same. HK2017 never writes the coordinate formula. |
| **Action** | A(γ) = ½ ∫₀ᵀ ⟨Jγ, γ̇⟩ dt = ∫_γ λ₀ | A(γ) := ½ ∫₀ᵀ ⟨Jγ(t), γ̇(t)⟩ dt | YES | Same formula. We also give the Liouville form interpretation, which HK2017 does not. |
| **Dual functional I_K** | I_K(z) = ¼ ∫₀ᵀ h_K²(−Jż) dt | I_K(z) = ¼ ∫₀¹ h_K²(−Jż) dt | **DIFFERENT domain** | We integrate over [0,T] (talk normalization); HK2017 integrates over [0,1]. When restricted to the respective constraint sets, both give c_EHZ = 2·I_K(z*) at minimizers. See normalization discussion. |
| **Dual constraint set** | z ∈ W^{1,2}([0,T], R⁴), z(0)=z(T), ∫₀ᵀ ż dt = 0, ∫₀ᵀ z dt = 0, A(z) = T | E = { z ∈ W^{1,2}([0,1], R^{2n}) : ∫₀¹ ż dt = 0, ∫₀¹ ⟨−Jż,z⟩ dt = 1 } | **DIFFERENT** | Three differences: (1) domain [0,T] vs [0,1], (2) we include centering ∫z=0, HK2017 does not, (3) our action constraint is A(z)=T, HK2017's is ∫⟨−Jż,z⟩=1 (which equals 2A=1 in their normalization, NOT the same as our A=T). |
| **EHZ capacity** | c_EHZ(K) = min { A(γ) : γ gen. closed char. on ∂K } | c_EHZ(K) = min { A(γ) : γ closed char. on ∂K }. Also: c_EHZ(K) = min_{z∈E} 2I_K(z) | YES | Same primal definition. The dual characterization differs in normalization (see above), but gives the same numerical value. |
| **Generalized closed characteristic** | γ ∈ W^{1,2}([0,T], R⁴), γ(0)=γ(T), Im(γ)⊂∂K, γ̇ ∈ J N_K(γ) a.e. | γ ∈ W^{1,2}([0,1], R^{2n}), Im(γ)⊂∂K, γ̇ ∈ J N_K(γ) a.e. (Def. 2.1) | **DIFFERENT domain** | Domain [0,T] vs [0,1]. Any such characteristic on [0,T] can be reparametrized to [0,1] and vice versa. |
| **Facet Reeb vector** | pᵢ = (2/hᵢ) J nᵢ | p_i = J∇(g_K²)\|_{int(F_i)} = (2/h_i) J n_i (Prop 3.1) | YES | Same final formula. HK2017 defines it via the Hamiltonian gradient and derives the formula. We state the formula directly and derive it in a note. |
| **Q-function** | Q(σ,β) = Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)}) | (unnamed) Σ_{j<i} β_{σ(i)} β_{σ(j)} ω(n_{σ(i)}, n_{σ(j)}) | YES | Same formula. We name it Q; HK2017 does not. |
| **Constraint set M(K)** | { β ∈ R^F : βᵢ ≥ 0, Σβᵢhᵢ = 1, Σβᵢnᵢ = 0 } | { (βᵢ) : βᵢ ≥ 0, Σβᵢhᵢ = 1, Σβᵢnᵢ = 0 } | YES | Identical. |
| **Symmetrized action matrix H** | Symmetric matrix with H_{ij} = ω(n_{σ(i)}, n_{σ(j)}) for i>j; Q = ½β^T H β | (not defined) | N/A | We introduced H for the algorithm description. HK2017 does not use a matrix form. The MATLAB constructs H via `A - triu(A) + tril(A)'` on the antisymmetric ω matrix. |
| **Normal cone** | N_K(x) = R₊ · conv{ nᵢ : x ∈ Fᵢ } | N_K(x) := R₊ conv{ n_i : x ∈ F_i } | YES | Identical. |
| **Duality correspondence** | z = γ − center(γ), I_K(z) = T = A(γ) at minimizers (talk normalization) | z = λγ + b, A(γ) = 2I_K(z) (Lemma 2.2) | **DIFFERENT form** | HK2017 allows scaling (λ) and translation (b), and gets A(γ) = 2I_K(z). Our talk normalization fixes this by centering (∫z=0) and using A(z)=T, getting I_K(z)=T=A(γ). These are equivalent: our z is the HK2017 z rescaled so that ∫⟨−Jż,z⟩=2T and then the relation A(γ) = 2I_K(z) becomes T = 2·(T/2) = T, while I_K(z)=c²=T. **UNCLEAR — needs Jörn's verification** of whether the talk normalization's duality statement is correctly recorded in the knowledge dump as "I_K(z) = T = A(γ)" vs HK2017's "A(γ) = 2I_K(z)." |
| **Capacity via Q** | c_EHZ = ½ [max Q]⁻¹ (HK2017 normalization: Σβᵢhᵢ=1) | c_EHZ = ½ [max Σ_{j<i} β_{σ(i)}β_{σ(j)}ω(n_{σ(i)},n_{σ(j)})]⁻¹ with β∈M(K) | YES | Same. |

---

## 3. Conventions Summary

### Convention 1: Curve domain — [0,T] vs [0,1]

**Our choice:** Generalized closed characteristics live on [0,T]; the dual functional integrates over [0,T].

**HK2017:** Everything lives on [0,1]. The "speed" of the characteristic absorbs the period.

**Why we differ:** The knowledge dump introduces a "talk normalization" (from the January talk notes) where curves live on [0,T] with A(z) = T. This makes the constraint more transparent: the action equals the period. The HK2017 normalization ∫⟨−Jż,z⟩ = 1 is less intuitive. We prove in talk normalization and then bridge to HK2017 normalization at the end (knowledge dump Section D4).

**Impact on formulas:** The final capacity formula (Thm 1.1) is the same in both normalizations. The intermediate objects (I_K, constraint set E, duality lemma) have different forms. All differences cancel in the normalization bridge (Section D4).

### Convention 2: Two normalizations used

**Our choice:** We use BOTH normalizations:
- **Talk normalization:** curves on [0,T], constraint A(z)=T, at minimizers I_K(z)=T.
- **HK2017 normalization:** curves on [0,1], constraint ∫⟨−Jż,z⟩=1, at minimizers I_K(z)=c².

**HK2017:** Only uses the [0,1] normalization.

**Why:** Talk normalization is easier for proofs (the action constraint is more natural). HK2017 normalization is needed for the final formula derivation (M(K) has Σβᵢhᵢ=1).

### Convention 3: Naming — Q-function and H-matrix

**Our choice:** We name the quadratic form Q(σ,β) and introduce a symmetrized matrix H.

**HK2017:** Neither Q nor H is named. The quadratic form appears inline in formulas.

**Why:** The algorithm description needs to refer to these objects repeatedly. Naming them reduces ambiguity and makes the algorithm pseudocode cleaner.

### Convention 4: Dimension — R⁴ vs R^{2n}

**Our choice:** We work exclusively in R⁴ (n=2).

**HK2017:** General R^{2n}.

**Why:** The thesis focuses on 4-dimensional polytopes. All formulas specialize to n=2. The algorithms work for general n but our implementation and experiments are in R⁴.

### Convention 5: Facet count symbol — F vs F_K

**Our choice:** F (plain).

**HK2017:** F_K (bold, with K subscript).

**Why:** We work with one polytope at a time in most contexts; the subscript K is unnecessary.

### Convention 6: Explicit vs implicit J matrix

**Our choice:** We write J as an explicit 4×4 matrix.

**HK2017:** J is "the standard complex structure" — never given as a matrix.

**Why:** Our code and algorithm description need the explicit matrix. The MATLAB code also uses (q,p) coordinates with J implicit in the omega function.

### Convention 7: Centering constraint in the dual problem

**Our choice (talk normalization):** The dual constraint set includes ∫₀ᵀ z dt = 0 (centering).

**HK2017:** The constraint set E does NOT include a centering constraint. The correspondence lemma (Lemma 2.2) allows z = λγ + b with arbitrary translation b.

**Why we differ:** The centering constraint simplifies the duality: it pins down the translation ambiguity, making z = γ − center(γ) without a free parameter b. **UNCLEAR — needs Jörn's verification** of whether adding the centering constraint actually restricts the minimizer set or just fixes a gauge freedom.

### Convention 8: MATLAB sign convention for the action

**MATLAB:** Computes `beta' * H * beta / 2` which can be negative, then takes `cap = -1/(2*minCap)`.

**Our formula:** c_EHZ = ½ [max Q]⁻¹ where Q > 0 at the optimum.

**Relationship:** The MATLAB's `minCap` is the minimum of the action over all orderings. For valid orbits, `minCap < 0`. Our `max Q = -minCap > 0`. So `-1/(2*minCap) = 1/(2*max Q)`. The two are equivalent. The sign difference arises because the MATLAB evaluates the quadratic form for ALL orderings (including those where the form is negative), while we define Q with the ordering that makes it positive.

---

## 4. Items Requiring Jörn's Verification

1. **Duality correspondence (Definition table, "Duality correspondence" row):** The knowledge dump says "I_K(z) = T = A(γ)" at minimizers in talk normalization, while HK2017 says "A(γ) = 2I_K(z)." These ARE consistent because the two I_K definitions use different domains of integration. Our I_K = ¼∫₀ᵀ h_K²(−Jż)dt integrates over [0,T]; HK2017's I_K = ¼∫₀¹ h_K²(−Jż)dt integrates over [0,1]. For a simple Reeb orbit where h_K²(−Jż) = 4 (constant), our I_K = ¼·4·T = T while HK2017's I_K = ¼·4·1 = 1, and HK2017 gets c_EHZ = 2I_K = 2·1 = 2... which is only correct if the orbit has been rescaled to live on [0,1] with the right action constraint. **Needs Jörn's verification** that the two normalizations give consistent final capacity values. The derivation in the knowledge dump (B2, Steps 3-4) appears self-consistent for the talk normalization, but the correspondence with HK2017's Lemma 2.2 should be verified.

2. **Centering constraint (Convention 7):** Does adding ∫z dt = 0 to the constraint set restrict the space of minimizers, or just fix the translation gauge freedom? If it restricts, the dual minimum could be different.

3. **The identity 2A(z) = ∫⟨−Jż,z⟩ dt:** The knowledge dump writes "2A(z) = the left side for a centered loop" (Section A8). Is the identity 2A = ∫⟨−Jż,z⟩ exact for all closed loops, or does it require centering (∫z=0)? By integration by parts, for closed loops (z(0)=z(T)): ∫⟨Jz,ż⟩ = −∫⟨Jż,z⟩ (since J^T = −J and boundary terms vanish). So A = ½∫⟨Jz,ż⟩ = −½∫⟨Jż,z⟩ = ½∫⟨−Jż,z⟩. Hence 2A = ∫⟨−Jż,z⟩. This holds for ALL closed loops, not just centered ones. The knowledge dump's parenthetical "(for a centered loop)" may be misleading. **Needs Jörn's verification.**
