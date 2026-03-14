# Tube Algorithm Spec

Working document. Jörn dictates, Claude Code edits, we iterate until gap-free.

## Voice conventions

- `<j>` Reviewed by Jörn. Likely correct (certainty comes from Rust + tests).
- `<q>` Question from Claude Code. Needs Jörn's attention.
- `<s>` Suggestion from Claude Code. Jörn accepts/rejects/modifies.
- `<dev>` Implementation commentary. Not source of truth, no review needed.

---

## 1. Setting

<j>
K ⊂ R⁴ is a convex polytope in H-representation: K = {x : ⟨n_i, x⟩ ≤ h_i, i = 1..F} where n_i are unit outward normals and h_i > 0. We assume K is full-dimensional (4D), irredundant (removing a halfspace strictly increases the polytope set K), and contains the origin in its interior (equivalent to h_i > 0).

This is the established definition of polytopes in the thesis chapter on the HK2017 algorithm.

The faces of K form a lattice: 3-faces (facets F_i), 2-faces (ridges F_{ij} = F_i ∩ F_j), 1-faces (edges), 0-faces (vertices). A 2-face F_{ij} exists iff F_i and F_j share at least 3 vertices (forming a polygon in R⁴). Every 2-face lies in exactly 2 facets.

A 2-face F_{ij} is **Lagrangian** if ω₀(n_i, n_j) = 0. K is **symplectic** if it has no Lagrangian 2-faces. The tube algorithm requires K to be symplectic.

We in particular can do the trick where we randomly perturb the halfspace data by a tiny amount (e.g. less than fp64 precision) to ensure the result is with probability 1 symplectic.

This is already implemented using rationals (so the probability is due to discreteness only almost 1, but in practice that's good enough to never encounter a non-symplectic polytope).

Furthermore, perturbation also ensures with probability 1 that the polytope is **generic** i.e. every 0-face lies on 4 facets, and every 1-face on 3 facets.
Proof: the intersection of m pairwise different facets gives a LP with m equations (n^T x=h) and F-m inequalities (n^T x <= h) for a 4d variable x. Generically in (n,h) this gives either an empty solution set, or a convex non-degenerate 4-m dimensional solution set (i.e. a 4-m face) on which we also get genericity properties if we like to.

We use, as is done in the code, exact incidence relations to decide such equality questions.

The math writeup of course works with reals, where = is not a problem to handle.
I suggest to immediately do the math writeup for the generic case i.e. simple+symplectic, instead of bothering with potential nuisances from non-generic properties.
Nevertheless, remark when e.g. simple or symplectic is being used, so we can in the future look up those places and think about generalizing the tube algorithm perhaps.
</j>

<q>
Is "symplectic polytope" standard terminology, or is it specific to this thesis? I see it defined in the tex as def:symplectic-polytope.
</q>
<j>
Specific to this thesis.
</j>

---

## 2. Directed edges

<j>
For a symplectic polytope, every 2-face F_{ij} gets a direction from the sign of ω₀(n_i, n_j). We say there is a **directed edge i → j** when there's a Reeb trajectory that has velocity R_i, then velocity R_j and the breakpoint lies on F_i \cap F_j.
This is already computed for HK2017 algorithm. Iirc(!) the equivalent checkable criteria are
1. ω₀(n_i, n_j) > 0
2. there is at least one point x \in F_i \cap F_j s.t. infinitesimally going away -eps*R_i or +eps*R_j stays on the polytope boundary. This is a LP feasability question.

Since ω₀ is antisymmetric, ω₀(n_i, n_j) > 0 iff ω₀(n_j, n_i) < 0. So every 2-face has at most one direction (no bidirectional edges for symplectic polytopes).
I conjecture that for generic polytopes, we have exactly one direction, i.e. if omega_0(ni,nj)>0 then there also is at least one feasable point x.

I conjecture that for generic polytopes, if F_i -> F_j is feasable then F_i\capF_j=F_ij is a non-degenerate 2-face, i.e. we don't have transitions that go through a 1-face instead of a 2-face (?).
I may be wrong about this!
</j>
<investigation>
The two conjectures above are about **generic** polytopes only:

**Conjecture 1** (ω₀ sign suffices for feasibility): Let K be generic and symplectic. If F_{ij} is a 2-face with ω₀(n_i, n_j) > 0, then there exists a point x in the interior of F_{ij} such that the Reeb transition F_i → F_j is feasible at x. In other words: the ω₀ sign check alone is sufficient — the LP feasibility check (criterion 2 above) is automatically satisfied.

**Conjecture 2** (no lower-dimensional transitions): Let K be generic. If a Reeb transition F_i → F_j is feasible, then F_i ∩ F_j is a 2-face (not a 1-face or 0-face). In other words: for generic polytopes, transitions only happen through ridges.

Checked on 19 generic polytopes (7 known + 12 random, F=5..8). Both hold on all 19.

For generic + symplectic: directed edges and ridges are in exact 1-to-1 correspondence.
</investigation>

<s>
For the implementation: since we assume generic + symplectic, directed edges biject with ridges. We can compute the DirectedSkeleton directly from `Skeleton::ridges` + `sign_pattern`, without needing the LP feasibility check. The ω₀ sign determines the direction, and every ridge is guaranteed feasible.

**Successors**: For each facet j, successors = {l : j → l is a directed edge}.
</s>

<q>
The investigation confirms that for generic polytopes, vertex-adjacency and ridge-adjacency coincide. So the distinction I raised (ridge vs vertex adjacency) doesn't matter for generic polytopes. The `build_directed_adjacency_matrix` from HK2017 would give the same answer for generic polytopes (modulo the ω₀ ≥ 0 vs > 0 distinction, which is moot since we're symplectic). Should we reuse it, or build fresh from ridges for clarity?
</q>

---

## 3. Reeb flow on a facet

<draft>
On facet F_i (the hyperplane {x : ⟨n_i, x⟩ = h_i}), the Reeb vector field is constant:

R_i = (2/h_i) J₀ n_i

where J₀ is the standard symplectic matrix in R⁴. A trajectory on F_i starting at point x moves as:

γ(t) = x + t · R_i

until it hits the boundary of F_i (i.e., it reaches a 2-face F_{ij} where ⟨γ(t), n_j⟩ = h_j for some other facet j).

The time to reach F_j from x on F_i is:

t = (h_j − ⟨x, n_j⟩) / ⟨R_i, n_j⟩

The denominator ⟨R_i, n_j⟩ = (2/h_i) ⟨J₀ n_i, n_j⟩ = (2/h_i) ω₀(n_i, n_j). For i → j (directed edge), this is positive, so the trajectory reaches F_j in positive time.

The **action** of a trajectory segment on facet i with duration t is simply t (the elapsed time in the Reeb parametrization).
</draft>

<q>
Is the action literally the time parameter t, or is there a factor? In some formulations, action = integral of the contact form α along γ, and for the Reeb flow dα(R) = 1 so action = time. But the 2/h_i factor in R_i rescales time — does the action pick up a corresponding factor, or is it really just t?
</q>

<q>
`reeb_vector()` in the codebase returns J₀n (without 2/h_i). Should the tube algorithm use the full R_i = (2/h_i) J₀ n_i, or is there a convention where the 2/h_i is absorbed elsewhere?
</q>

---

## 4. Step map Φ_{ijl}

<draft>
The step map describes what happens to a trajectory that enters facet j from facet i, then exits facet j into facet l.

**Domain**: Points x on the 2-face F_i ∩ F_j (i.e., ⟨x, n_i⟩ = h_i and ⟨x, n_j⟩ = h_j).
**Codomain**: Points on the 2-face F_j ∩ F_l (i.e., ⟨Φ(x), n_j⟩ = h_j and ⟨Φ(x), n_l⟩ = h_l).

The trajectory starts at x, flows along R_j for time t(x) until hitting F_l:

Φ_{ijl}(x) = x + t(x) · R_j

where:

t(x) = (h_l − ⟨x, n_l⟩) / ⟨R_j, n_l⟩

The denominator is ⟨R_j, n_l⟩ = (2/h_j) ω₀(n_j, n_l) > 0 (since j → l is a directed edge).

**Key properties**:
- Φ is affine in x (numerator of t is affine, denominator is constant)
- t(x) > 0 for points inside K (since ⟨x, n_l⟩ < h_l in the interior)
- t(x) = action increment (time spent on facet j for this segment)

**As an affine map** Φ(x) = Mx + b:
- M = I₄ − (1/d) R_j n_l^T  (rank-1 update of identity, where d = ⟨R_j, n_l⟩)
- b = (h_l/d) R_j

**Action increment** Δa(x) = t(x) = −(1/d) ⟨n_l, x⟩ + h_l/d:
- gradient = −n_l/d
- constant = h_l/d
</draft>

<q>
The step map uses the FULL Reeb vector R_j = (2/h_j) J₀ n_j. This means the matrix M and offset b depend on h_j (through R_j). Is that right?
</q>

<q>
Verification check I plan to implement: for ridge vertices x of F_i ∩ F_j, verify that Φ(x) satisfies ⟨Φ(x), n_j⟩ = h_j and ⟨Φ(x), n_l⟩ = h_l. This should hold analytically but is a good numerical sanity check. Does this make sense as a test?
</q>

<q>
A subtlety: the domain of Φ_{ijl} is the 2-face F_i ∩ F_j, but not all points there necessarily map to valid points on F_j ∩ F_l that are inside K. The image Φ(F_{ij}) may only partially overlap with F_{jl}. This is handled by the tube extension (intersecting End with the target 2-face polygon). Correct?
</q>

---

## 5. Tube data structure

<draft>
A **tube** T(σ) for a facet sequence σ = (σ₁, σ₂, ..., σ_k) is the set of all pure Reeb trajectories that visit facets in order σ₁ → σ₂ → ... → σ_k, transitioning at the 2-faces F_{σ(i),σ(i+1)}.

Each trajectory in T(σ) is uniquely determined by its first breakpoint γ(t₁) ∈ F_{σ₁} ∩ F_{σ₂}. The key insight is that everything (subsequent breakpoints, total action, etc.) is an affine function of this starting point.

A tube is described by a 5-tuple:

**(Start, End, φ, a, ρ)**

- **Start** ⊂ F_{σ₁} ∩ F_{σ₂}: convex polygon of valid first breakpoints (in 2D local coords of that 2-face)
- **End** ⊂ F_{σ(k−1)} ∩ F_{σ_k}: convex polygon of valid last breakpoints (in 2D local coords)
- **φ: Start → End**: affine map sending each starting point to its corresponding endpoint (composition of step maps along the whole sequence)
- **a: End → R**: affine function giving the total action (total elapsed time across all segments) as a function of the endpoint
- **ρ ∈ R**: accumulated rotation number (sum of rotation increments at each transition)

**Initialization** for k = 2, σ = (i, j):
- Start = End = F_i ∩ F_j (the full 2-face, projected to 2D local coordinates)
- φ = identity
- a ≡ 0  (no segments traversed yet, no elapsed time)
- ρ = 0

**2D local coordinates**: Each 2-face is a 2D convex polygon living in a 2D affine subspace of R⁴. We pick an orthonormal basis (e₁, e₂) for the tangent space and an origin point, then represent points as 2D vectors. The basis is per-2-face.
</draft>

<q>
The tube for k=2 σ = (i, j) has "no segments traversed" — meaning the trajectory hasn't moved yet, it's just sitting at the breakpoint on F_i ∩ F_j. The first actual Reeb flow segment happens when we extend to k=3 by appending some facet l. Is that right?

Alternatively: does the k=2 tube already represent a trajectory that has flowed on F_i until reaching F_i ∩ F_j? If so, the action should already be nonzero for k=2.
</q>

<q>
Why is φ a map from Start to End and not the other way around? Since we parameterize by the starting point, it seems natural to ask "given where you start, where do you end up?" That's Start → End. But for the fixed-point closing condition later, we need φ composed with itself to map Start → Start (after closing). Is there a reason to prefer End → Start?
</q>

<q>
The rotation number ρ — I understand it's the sum of rotation increments Δρ_{σ(i),σ(i+1)} at each transition. The increment Δρ_{jl} comes from the transition matrix ψ_{F_{jl}} ∈ Sp(2), which is positive elliptic for symplectic polytopes, giving Δρ ∈ (0, 1/2). Is this from CH2021? This is the part I'm least confident about.
</q>

---

## 6. Tube extension

<draft>
Given tube data (Start, End, φ, a, ρ) for σ = (σ₁, ..., σ_k) and a new facet l = σ(k+1) with σ_k → l directed:

Let Φ = Φ_{σ(k−1), σ_k, l} be the step map for the last triple.

1. **New End**: End' = Φ(End) ∩ P_{σ_k, l}
   where P_{σ_k, l} is the full 2-face polygon of F_{σ_k} ∩ F_l in its local 2D coords.
   (The image of the old End under the step map, intersected with the target 2-face.)

2. **New φ**: φ' = Φ ∘ φ  (compose step map after old map)

3. **New Start**: Start' = (φ')⁻¹(End')
   (Pull back the new End through the full composed map to get valid starting points.)

4. **New action**: a'(y') = a(Φ⁻¹(y')) + t_{σ(k−1),σ_k,l}(Φ⁻¹(y'))
   (Old action at the previous endpoint, plus the new segment's travel time.)

5. **New rotation**: ρ' = ρ + Δρ_{σ_k, l}

The extended tube data is (Start', End', φ', a', ρ') for σ' = (σ₁, ..., σ_k, l).
</draft>

<q>
For the "new End" computation: I need to apply Φ (a 4D affine map) to the 2D polygon End (which lives in local coords of F_{σ(k-1)} ∩ F_{σ_k}), then intersect with the 2-face polygon P_{σ_k,l} (in local coords of F_{σ_k} ∩ F_l). This means I need to:
- Embed End vertices back to 4D
- Apply Φ in 4D
- Project to the 2D local coords of F_{σ_k} ∩ F_l
- Intersect two 2D convex polygons

Is this the right sequence, or is there a shortcut through 2D?
</q>

<dev> Extension is Step 5 in the plan. This section is here so we can verify the data structures from Steps 1-3 support it. </dev>

---

## 7. Pruning

<draft>
Prune a tube (Start, End, φ, a, ρ) when any of:

1. **Empty**: Start = ∅ or End = ∅ (no valid trajectories remain)
2. **Action bound**: min_{y ∈ End} a(y) > c* (even the cheapest trajectory exceeds current best capacity)
3. **Rotation bound**: ρ ≥ 2 (from CH2021: closed orbits of minimum action have rotation ≤ 2)
4. **Simplicity**: σ has a repeated facet (HK2017 shows minimum-action orbits are simple)
</draft>

<dev> Out of scope for Steps 1-3 implementation. </dev>

---

## 8. Closing and fixed points

<draft>
To check if tube σ = (σ₁, ..., σ_k) can close into a periodic orbit:

1. Check that σ_k → σ₁ is a directed edge (can transition back)
2. Extend twice: append σ₁ then σ₂ to get σ'' = (..., σ_k, σ₁, σ₂)
   This gives (Start'', End'', φ'', a'', ρ'')
   Now Start'' and End'' are both subsets of F_{σ₁} ∩ F_{σ₂}, and φ'' maps Start'' → End''

3. A closed orbit exists iff φ'' has a fixed point in Start'' ∩ End''
   (i.e., a point x where φ''(x) = x)

4. Fixed point: solve (M − I)x = −b where φ''(x) = Mx + b
   The orbit's action is a''(x) at the fixed point.

5. The capacity c_EHZ(K) = min over all such fixed points of a''(x) / 2π... or just a''(x)?
</draft>

<q>
Is the capacity c_EHZ(K) = min action, or min action divided by something? The EHZ capacity is the minimum period of closed characteristics on ∂K. For the Reeb flow with the contact form α = Σ (p_i dq_i - q_i dp_i)/2 restricted to ∂K... I think the action IS the period, so c = min a''(x). But I'm not certain about normalization.
</q>

<dev> Out of scope for Steps 1-3 implementation. </dev>