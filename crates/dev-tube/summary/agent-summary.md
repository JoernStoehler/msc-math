# Tube Algorithm — Agent Summary

Agent-written reference material (2026-04-04). Summarizes what is in the repo
and in the CH2021 paper. NOT a source of truth — Jörn writes the authoritative
spec in `algorithm.md`.


## 1. CH2021 Mathematical Foundation

Source: `papers/ch2021/`, citation key `CH2021` in `thesis/bibliography.bib`.
Full title: "Computing Reeb dynamics on 4d convex polytopes" (Chaidez & Hutchings, 2021).

### 1.1 Symplectic Flow Graph (CH2021 §2, Def. 2.10)

CH2021 defines a **linear flow graph** G = (Γ, A, Φ):
- **Vertices**: linear domains (open subsets of affine spaces)
- **Edges**: each edge e from u to v carries a **linear flow** Φ_e = (D_e, φ_e, f_e)
  - D_e ⊂ A_u — domain of definition (linear domain)
  - φ_e: D_e → A_v — affine map (the "flow map")
  - f_e: D_e → R — affine function (the "action function")
- **Composition** (Def. 2.5): Ψ∘Φ = (φ⁻¹(E), ψ∘φ, f + g∘φ)
- **Trajectory**: pair (p, x) where p is a path in Γ and x ∈ D_p
- **Periodic orbit**: trajectory (p, x) where p is a cycle and φ_p(x) = x
- **Action**: f_p(x) for periodic orbit (p, x)

A **symplectic flow graph** (Def. 2.12) additionally equips each vertex with a
symplectic form ω_v on TA_v, and requires φ_e to be symplectic.

### 1.2 The Flow Graph of a Polytope (CH2021 Def. 2.13)

For a symplectic polytope X ⊂ R⁴, CH2021 defines G(X):
- **Vertices** = 2-faces of X (each a 2D linear domain with ω₀ restricted)
- **Edges**: from 2-face F₁ to 2-face F₂ when there is a 3-face E adjacent to
  both, and the Reeb flow on E carries points from F₁ to F₂
  - φ_e(x) = y where y is the endpoint of the Reeb trajectory on E starting at x
  - f_e(x) = transit time (= λ₀-integral along segment)
  - Both are affine because the Reeb vector R_E is constant on E

**Key bijection** (Prop. 2.14): periodic orbits of G(X) ↔ Type 1 combinatorial
Reeb orbits of X, with matching actions.

### 1.3 Quaternionic Trivialization (CH2021 Def. 2.22)

Defines canonical trivializations τ_F: TF → R² for each 2-face F:
- Fix quaternionic matrices **i**, **j**, **k** ∈ SO(4) with **i** = J₀
- For 2-face F: find the unique adjacent 3-face E whose Reeb vector points
  INTO E from F. Let ν = outward unit normal to E.
- τ_F(V) = (⟨V, **j**ν⟩, ⟨V, **k**ν⟩)

In coordinates (x₁, x₂, y₁, y₂) = our (q₁, q₂, p₁, p₂):

```
i = J₀ = [[0,0,-1,0],[0,0,0,-1],[1,0,0,0],[0,1,0,0]]
j = [[0,-1,0,0],[1,0,0,0],[0,0,0,1],[0,0,-1,0]]
k = [[0,0,0,-1],[0,0,1,0],[0,-1,0,0],[1,0,0,0]]
```

### 1.4 Transition Matrix (CH2021 Def. 2.20, Lem. 2.21)

For 2-face F between 3-faces E (with outward normal ν) and E' (with outward normal ν'):

**Transition matrix**: ψ_F = τ_F ∘ (τ'_F)⁻¹ ∈ Sp(2)

where τ'_F uses the OTHER adjacent 3-face E' and its normal ν'.

**Explicit formula** (CH2021 Lem. 2.21): with a₁ = ⟨ν',ν⟩, a₂ = ⟨**i**ν',ν⟩,
a₃ = ⟨**j**ν',ν⟩, a₄ = ⟨**k**ν',ν⟩:

```
ψ_F = (1/a₂) * [[a₁a₂ - a₃a₄, -a₂² - a₄²],
                  [a₂² + a₃²,    a₁a₂ + a₃a₄]]
```

**Key properties**:
- Tr(ψ_F) = 2⟨ν',ν⟩ ∈ (-2, 2) → ψ_F is elliptic
- a₂ > 0 (by Lem. 2.24, the "EinEout" lemma) → ψ_F is **positive** elliptic
- By Cor. 2.22: rotation number ∈ (0, 1/2)

### 1.5 Rotation Numbers (CH2021 §2, Appendix A)

**Lift convention** (Def. 2.22): for each edge e from F₁ to F₂, lift
τ_{F₂} ∘ Tφ_e ∘ τ_{F₁}⁻¹ ∈ Sp(2) to φ̃_{e,τ} ∈ Sp̃(2) with rotation in (-1/2, 1/2].

**Result** (Cor. 2.22): each lift has rotation number in **(0, 1/2)** (open interval).

**Combinatorial rotation number** (Def. 2.25): for a cycle p = e₁...eₖ,
ρ_comb = ρ(φ̃_{eₖ,τ} ∘ ... ∘ φ̃_{e₁,τ}).

**Computing products** (Prop. A.7): if ρ(Ã) ∈ (0, 1/2), then
ρ(B̃) ≤ ρ(ÃB̃) ≤ ρ(B̃) + 1/2.
This bounds the accumulated rotation without computing the exact Sp̃(2) product.

**Mod Z formula** (Lem. A.5): for positive elliptic A with eigenvalues e^{±2πiθ}:
ρ̄(A) = θ ∈ (0, 1/2). So: Δρ = arccos(Tr(ψ_F)/2) / (2π).

### 1.6 CH2021 Result for Computing c_EHZ

**Corollary 1.15**: c_EHZ(X) = min A_comb(γ) over combinatorial Reeb orbits γ
with ρ_comb(γ) ≤ 2 that are either Type 1 or Type 2.

This bound ρ ≤ 2 makes the search finite: each step adds rotation > 0, so
the maximum cycle length is bounded.


## 2. Repo's Algorithm End-to-End

### 2.1 Notation Translation

| CH2021 | Repo (thesis & code) | Notes |
|--------|---------------------|-------|
| Polytope X | K = {x : aᵢᵀx ≤ 1} | Dual vertex parameterization |
| 3-face normal ν | nᵢ/‖nᵢ‖ (unit) or nᵢ (not unit) | Code uses aᵢ = nᵢ/hᵢ as dual vertices |
| Reeb vector R_E | Rᵢ = 2J₀aᵢ | Code: `reeb_direction(a) = J₀a` (half) |
| 2-face as vertex of Γ | Directed edge Fᵢ → Fⱼ | Repo indexes by facet pairs, not 2-faces |
| Linear flow (D, φ, f) | Step map Φ_{ijl}, action Δa_{ijl} | Triple-indexed in repo |
| Transition matrix ψ_F | Δρ_{jl} | Code computes scalar only, not matrix |

**Coordinate convention**: (q₁, q₂, p₁, p₂) with indices [0,1] = q, [2,3] = p.
ω₀(u,v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁.

### 2.2 Types

**Input**: `Polytope4D` — H-rep K = {x : aᵢᵀx ≤ 1} with:
- `dual_vertices: Vec<[BigRational; 4]>` — exact aᵢ
- `dual_vertices_f64: Vec<Vector4<f64>>` — f64 copies
- `omega_signs: DMatrix<i8>` — precomputed sign(ω₀(aᵢ, aⱼ))
- `incidence: DMatrix<bool>` — vertex-facet incidence

**Precondition**: symplectic polytope — ω₀(aᵢ, aⱼ) ≠ 0 for all 2-face pairs.
Checked by `check_symplectic()`, returns `TubeError::HasLagrangian2Face` on failure.

**TubePrecomputation** (built once):
- `directed_edges[i]`: list of j with Fᵢ → Fⱼ (ω₀(aᵢ,aⱼ) > 0 and shared 2-face)
- `ridge_vertices[(i,j)]`: vertex coordinates of the 2-face Fᵢ ∩ Fⱼ (in R⁴)
- `reeb_vectors[i]`: Rᵢ = 2J₀aᵢ
- `dual_vertices[i]`: aᵢ
- `rotation_increments[j][idx]`: Δρ_{j,l} for each directed edge j → l

**TubeData** (per tube during DFS):
- `sequence: Vec<usize>` — facet sequence σ
- `start_vertices_4d: Vec<Vector4<f64>>` — Start polygon vertices (in R⁴)
- `end_vertices_4d: Vec<Vector4<f64>>` — End polygon vertices (in R⁴)
- `phi_matrix: Matrix4<f64>` — affine map φ: Start → End (4×4 matrix part)
- `phi_offset: Vector4<f64>` — affine map offset
- `action_gradient: Vector4<f64>` — action function gradient (in R⁴)
- `action_constant: f64` — action function constant
- `rotation: f64` — accumulated rotation number

**TubeResult** (output):
- `capacity: f64` — min action = c_EHZ
- `best_sequence: Vec<usize>` — facet sequence of optimal orbit
- `fixed_point: Vector4<f64>` — starting point of optimal orbit
- `tubes_explored`, `tubes_pruned` — diagnostics

### 2.3 Initialization

For each directed edge (i, j) with Fᵢ → Fⱼ:
- `sequence = [i, j]`
- `Start = End = ridge_vertices[(i,j)]` (the full 2-face polygon)
- `phi_matrix = I₄`, `phi_offset = 0`
- `action_gradient = 0`, `action_constant = 0`
- `rotation = 0`

### 2.4 Step Map (thesis Def. step-data, eq:step-map)

For triple (i, j, l) with Fᵢ → Fⱼ and Fⱼ → Fₗ:

```
Φ_{ijl}(x) = x + t_{ijl}(x) · Rⱼ
t_{ijl}(x) = (hₗ - ⟨x, nₗ⟩) / ⟨Rⱼ, nₗ⟩
```

Denominator: ⟨Rⱼ, nₗ⟩ = (2/hⱼ)·ω₀(nⱼ, nₗ) > 0 (since Fⱼ → Fₗ).

Action increment: Δa_{ijl}(x) = t_{ijl}(x) (transit time).

Note: CH2021 defines the flow map φ_e implicitly (flow along R_E from F₁ to F₂).
The repo's formula makes the step map explicit and triple-indexed (i,j,l)
because the step depends on which 2-face you arrived FROM (determines
the hyperplane constraint ⟨x, nₗ⟩ = hₗ at the destination).

### 2.5 Extension (thesis Def. tube-extension)

Given tube T(σ₁,...,σₖ), extend by appending σ(k+1) = l:
- Φ = Φ_{σ(k-1), σ(k), l} (step map)
- End' = Φ(End) ∩ (Fσ(k) ∩ Fₗ) — polygon intersection
- φ' = Φ ∘ φ — composed affine map
- Start' = (φ')⁻¹(End') — pulled back
- a'(y') = a(Φ⁻¹(y')) + Δa(Φ⁻¹(y')) — accumulated action
- ρ' = ρ + Δρ_{σ(k), l} — accumulated rotation

### 2.6 Pruning

1. **Empty** (lem:prune-empty): Start = ∅ or End = ∅
2. **Action** (lem:prune-action): min_{y ∈ End} a(y) > c* (current best)
3. **Rotation** (lem:prune-rotation): ρ ≥ 2 (CH2021 Cor. 1.15: min orbit has ρ ≤ 2)
4. **Simple** (lem:prune-simple): σ(i) = σ(j) for some i ≠ j (min orbit is simple)

### 2.7 Closing (thesis Def. tube-close)

Append σ(k+1) = σ(1) and σ(k+2) = σ(2) — two extension steps.
Result: φ'' maps Start' ⊂ F_{σ(1)} ∩ F_{σ(2)} → F_{σ(1)} ∩ F_{σ(2)}.

Closed orbits ↔ fixed points of φ'' in Start' (lem:fixed-point).
Action = a''(x) at fixed point x.

**Preconditions for closing** (not always satisfiable):
- Need F_{σ(k)} → F_{σ(1)} (directed edge must exist)
- Need F_{σ(1)} → F_{σ(2)} (always exists — it's the tube's first edge)

### 2.8 Search

DFS over facet sequences. For each tube:
1. Prune (§2.6)
2. Try closing (§2.7) — update c* if fixed point found
3. Extend (§2.5) — for each l ∉ {σ(1),...,σ(k)} with F_{σ(k)} → Fₗ, recurse


## 3. Known Divergences Between CH2021 and the Repo

### 3.1 Rotation Increment (CRITICAL)

**CH2021**: Δρ_{jl} = rotation number of transition matrix ψ_{F_{jl}}, computed via
the quaternionic trivialization (§1.4 above). Exact formula:
Δρ = arccos(Tr(ψ_F)/2) / (2π) where Tr(ψ_F) = 2⟨ν', ν⟩.

**Code** (`tube/mod.rs:325-358`): uses angle between Reeb vectors as heuristic:
```
cos_angle = R_j · R_l / (|R_j| |R_l|)
rho = arccos(cos_angle) / (2π)
rho = clamp(rho, 0.01, 0.49)
```

This is NOT the CH2021 formula. The code comment (line 339-345) acknowledges this:
"The psi_{jl} computation was abandoned (Sherman-Morrison singular)."

**Impact**: pruning bounds may be incorrect — the heuristic angle may over- or
underestimate the true rotation, leading to either missed orbits or insufficient
pruning. The algorithm still produces correct results on test cases because the
pruning is conservative (clamped to [0.01, 0.49]), but this is not proven.

### 3.2 Working in R⁴ vs Local 2D Coordinates

**Thesis math** (def:tube-data): "closed convex polygon (in local 2-dimensional
coordinates of the 2-face)" and "φ stored as 2×2 matrix + offset in local coords."

**Code**: works entirely in R⁴:
- `start_vertices_4d: Vec<Vector4<f64>>` — polygon vertices in R⁴
- `phi_matrix: Matrix4<f64>` — 4×4 affine map
- Polygon intersections done by projecting to 2D, clipping, lifting back

The 4D representation avoids coordinate system construction per 2-face but
introduces numerical issues (4×4 matrix inversions, degenerate projections).
CH2021's flow graph uses each 2-face's intrinsic 2D coordinates.

### 3.3 Step Map Parameterization

**CH2021**: edge from 2-face F₁ to 2-face F₂ through 3-face E.
The flow map φ_e: F₁ → F₂ is parameterized by the 3-face.

**Repo**: triple (i, j, l) — from Fᵢ through Fⱼ to Fₗ. The step map Φ_{ijl}
maps points in Fᵢ ∩ Fⱼ to points in Fⱼ ∩ Fₗ. This is parameterized by facets,
not by 2-faces or 3-faces.

The repo's formulation is equivalent but reorganized: one CH2021 edge
(from 2-face Fᵢⱼ to 2-face Fⱼₗ through 3-face Fⱼ) corresponds to
one repo step map Φ_{ijl}.

### 3.4 Type 2 Orbits

**CH2021**: Cor. 1.15 requires minimizing over both Type 1 AND Type 2 orbits.
Conj. 1.26: for generic symplectic polytopes, no Type 2 minimum-action orbit exists.

**Code**: searches Type 1 only. Relies on Conj. 1.26 (unproven).

### 3.5 Action Function Representation

**Thesis math**: action function a: End → R is exactly affine (gradient + constant).

**Code** (`tube/mod.rs:~500-530`): uses `fit_affine_action()` — least-squares fit.
This introduces numerical error. The thesis/math.tex proves a is exactly affine
(composition of affine maps), so the fit should be unnecessary — the affine
structure could be maintained analytically through composition.


## 4. Open Questions

From `thesis/tube-algorithm.tex` TODO/GAP markers:

### JÖRN Q1 (line 462): Transition matrix formula translation

CH2021 Lem. 2.21 gives ψ_F in quaternionic notation (ν, ν', **i**, **j**, **k**).
Translation to repo notation (nᵢ, hᵢ, J₀) is needed for implementation.
Should the thesis state the translated formula, or just cite CH2021?

Key: ν = nᵢ/‖nᵢ‖ (unit outward normal to 3-face). The a₁...a₄ in CH2021's
formula are inner products of the two unit normals under quaternionic rotations.

### JÖRN Q2 (line 129): Lagrangian 2-face equivalence

Is ω₀|_{TF_{ij}} = 0 equivalent to ω₀(nᵢ, nⱼ) = 0?
CH2021 Def. 1.4 uses the first form; the repo uses the second.
The thesis claims equivalence but doesn't prove it.

### JÖRN Q3 (line 416): Rotation number definition

The thesis defines rotation abstractly (path in Sp(2), lift to universal cover).
CH2021 defines it concretely via the quaternionic trivialization.
Question: which definition should the thesis use? The algorithm needs:
(1) each Δρ_{jl} ∈ (0, 1/2), and (2) total ρ = sum of Δρ.

### JÖRN Q4 (line 593): Closing edge check

Closing requires F_{σ(k)} → F_{σ(1)} to be a directed edge. This is NOT
guaranteed — depends on the polytope. The algorithm box doesn't state this check
explicitly. If the edge doesn't exist, the step map is undefined.

### JÖRN Q5 (line 716): Correctness proof

The proof sketch claims: (1) simple min-action orbit exists (HK2017), (2) it's
in some T(σ), (3) the search is exhaustive, (4) pruning is sound. Is a sketch
sufficient for the thesis?

### GAP: Step map formula (line 317)

The explicit step map formula (eq:step-map) is agent-derived from the
dictation notes. Jörn should verify.

### GAP: 2-face uniqueness proof (line 87)

The normal-cone proof of Lem. 2-face-structure is agent-written.
Jörn should verify or simplify.


## 5. What's NOT in the Repo

Based on Jörn's statement that "past efforts were made to expand the algorithm
way past what CH2021 did, and most of the knowledge isn't really in the repo":

- **Exact rotation increment computation** — attempted via Sherman-Morrison but
  abandoned. The quaternionic trivialization approach from CH2021 was never
  implemented. The knowledge of what went wrong is in Jörn's head, not written up.

- **Deleted spec files** — `tube-spec.md` and `tube-algorithm-plan.md` (referenced
  in `handoffs/tube-algorithm.md`) were deleted. They contained detailed specs
  with open questions (`<q>` markers).

- **Deleted dictation notes** — `tube-notes.md` (Jörn's raw dictation, 2026-02-18)
  was removed in post-migration cleanup. The thesis section was agent-written FROM
  these notes.

- **Archaeology code** — `archaeology/raw/code/archive__tube.rs` (39KB) and
  `reverted__tube.rs` (20KB) contain prior implementation attempts with known bugs
  (trivialization issues, orbit validation gaps). These are untrusted reference only.

- **Extensions beyond CH2021** — Jörn mentioned "expand the algorithm way past
  what CH2021 did." What these extensions are is not documented.
