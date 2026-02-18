# Code–Thesis Audit Report

**Date:** 2026-02-18
**Branch:** `claude/code-thesis-audit`
**Base commit:** `74a14c7` (local `main`)
**Scope:** All thesis `.tex` files + all core library `.rs` modules

This report inventories every point where the Rust codebase and the thesis diverge. It does **not** fix anything — it is a reference for Jörn to decide what to reconcile, what to accept, and what to annotate.

Action tags used throughout:
- `[add-code-comment]` — add a `///` or `//` Rust comment pointing to the thesis
- `[add-thesis-note]` — add a `% Rust: ...` QC comment in the `.tex` source
- `[thesis-todo]` — missing section/content the thesis should add
- `[accept-divergence]` — intentional divergence, no action needed
- `[jörn-decision]` — requires Jörn to decide which side (or both) to change

---

## §A — Code features NOT described in the thesis

### A1. KKT solver numerical machinery (`kkt.rs`)

The thesis describes the KKT linear system and its solution abstractly (a unique solution exists, extract β). The Rust implementation has substantial numerical infrastructure not mentioned anywhere:

- **LU fast path + SVD fallback.** `solve_kkt` first tries a dense LU solve. If the system appears degenerate (SVD ratio `λ_max/λ_min > SVD_GAP_THRESHOLD`), it falls back to SVD-based rank detection and null-space search.
- **`SVD_GAP_THRESHOLD = 100.0`** — magic constant controlling the LU/SVD switch. The doc comment explains it and documents a known correctness gap: 26 out of 23,650 F=7 (S,σ) pairs get a null space truncated to rank 1 that should be rank 2, causing the β-positivity search to find no valid β. Those orbits are dropped; the correctness risk is low (the orbit is a local saddle, not the minimum). See §D1 for the thesis implication.
- **`find_positive_beta_1d` / `find_positive_beta_nd`.** When the SVD path is used and the null space has rank ≥ 1, a secondary search looks for a direction in null(N) with β > 0. No mention in the thesis.
- **`solve_kkt_svd_only`.** Ablation variant that bypasses the LU path entirely. Test-only, not exported. Not in thesis.
- **Numerical thresholds:** `EPS_BETA_POSITIVE` (β positivity guard), `EPS_Q_POSITIVE` (Q positivity guard), `EPS_KKT_RESIDUAL` (residual validation). None mentioned in thesis.
- **Strict vs lenient β check.** `EhzResult` has both `capacity` (strict: β > +EPS) and `capacity_lenient` (lenient: β > −EPS). The `numerical_gap()` method measures the difference. This quality signal is computed in every run but the thesis never discusses numerical confidence monitoring.

**Action:** `[add-thesis-note]` in `general-case-algorithm-proof.tex` or `correctness.tex`: note that the implementation uses LU/SVD for degenerate cases, with strict/lenient β checks for numerical confidence. The SVD correctness gap (§D1) needs a `[jörn-decision]` on whether to add a limitation note to the thesis.

---

### A2. Two public HK2017 variants in `lib.rs`

Both `ehz_capacity` (unpruned) and `ehz_capacity_pruned` (pruned) are publicly exported from `lib.rs`. The thesis describes the algorithm once (§2.1, `alg:ehz`) and the pruning corollary once (§2.2 / `cor:adjacency-pruning`), but:

- The thesis never says the experiments use the **pruned** variant. The benchmark table labels results as "Pruned" but there is no formal "Algorithm (Pruned)" presentation, and the narrative text never says "we use `ehz_capacity_pruned` in all experiments."
- The unpruned `ehz_capacity` is a real fallback but is never described as such in the thesis.

**Action:** `[add-thesis-note]` in `general-case-algorithm.tex`: note that `alg:ehz` maps to `ehz_capacity` and that the pruned variant (corollary) is what all experiments actually run. `[add-code-comment]` on `ehz_capacity`: note it is the unpruned reference implementation and experiments use `ehz_capacity_pruned`.

---

### A3. Tube algorithm placeholder (`algorithms::tube`)

`algorithms::tube` exists as a module in `lib.rs` and is referenced in the crate module structure. The file (`tube/mod.rs`) is a 5-line placeholder with a `todo!()` panic. The thesis never mentions the tube algorithm at all.

**Action:** `[jörn-decision]` — does the tube algorithm belong in the thesis (even as a future-work note)? If not, the module should stay as-is. No thesis change needed unless Jörn wants to mention it.

---

### A4. Reeb trajectory simulation (`geom/reeb_trajectory.rs`)

`ReebTrajectory`, `ReebSegment`, and `simulate()` implement a forward-simulation of piecewise-linear Reeb orbits given a polytope and a permutation (σ, β). This is used by the visualization experiment. The thesis theory section defines Reeb orbits abstractly; `visualization.tex` uses the simulation output in figures but never describes the simulation algorithm.

**Action:** `[add-thesis-note]` in `visualization.tex`: briefly note that visualization uses a forward-simulation of the orbit given (σ, β). Also see C5 for the `2/h` factor deviation.

---

### A5. Combinatorial skeleton (`geom/skeleton.rs`)

The `Skeleton` struct computes vertex–facet incidence, edges (pairs of facets sharing a ridge), and ridges (codimension-2 faces). This is used internally by the billiard algorithm and visualization. The thesis never describes combinatorial skeleton data.

**Action:** `[accept-divergence]` — infrastructure detail. Not necessary to describe in the thesis.

---

### A6. Volume computation (`geom/volume.rs`)

`volume()` computes polytope volume via qhull triangulation. There is also a `deprecated::volume_divergence()` reference implementation using the divergence theorem. The thesis uses volume in the systolic ratio formula `sys(K) = c_EHZ(K)² / (2 vol(K))` but never describes how volume is computed.

**Action:** `[add-thesis-note]` in `rejection-sampling.tex` or `correctness.tex`: one line noting volume is computed via qhull triangulation. `[add-code-comment]` on `deprecated::volume_divergence`: note it is a reference implementation, not used in production.

---

### A7. Validation algorithms (`geom/validation.rs`)

Two algorithms not described in the thesis:
- `check_bounded()` — O(F³) boundedness check using a 4D cross-product construction.
- `find_redundant_facet()` — affine rank check to detect irredundant facets.

`rejection-sampling.tex` describes *what* polytopes are checked for (bounded, irredundant) but not *how*.

**Action:** `[add-thesis-note]` in `rejection-sampling.tex`: one sentence naming the algorithms used. Or `[accept-divergence]` if implementation details are out of scope.

---

### A8. Polytope construction and vertex enumeration (`geom/polytope.rs`, `geom/vertices.rs`)

Vertices are enumerated by solving all C(F,4) systems of tight inequalities (all 4-face intersections). An irredundancy check runs at construction time. Neither step is described in the thesis.

**Action:** `[accept-divergence]` — standard computational geometry infrastructure, not relevant to the mathematical content.

---

### A9. Supporting infrastructure not in thesis

None of the following are described in the thesis:

| Component | Location | Note |
|---|---|---|
| Named polytope library | `geom/known_polytopes.rs` | `KnownPolytope` struct, `all_known()`, `literature_values()` |
| 2D polygon utilities | `geom/polygon.rs` | `regular_polygon_2d`, `rotate_polygon_2d` |
| 4D cross product | `geom/cross_product.rs` | Used by `check_bounded()` |
| JSONL serialization | `dataset.rs` | Full dataset read/write |
| Random polytope generation | `random.rs` | `generate_random_polytopes` with `ChaCha8Rng` |
| Tolerance constants | `constants.rs` | `EPS_FACET_INCIDENCE` |
| Cyclic permutation enumeration | `algorithms/hk2017/permutations.rs` | Used in `ehz_capacity` and `ehz_capacity_pruned` |

**Action:** `[accept-divergence]` for all — infrastructure details. One exception: `ChaCha8Rng` is the specific PRNG used for rejection sampling reproducibility; `[add-thesis-note]` in `rejection-sampling.tex` if reproducibility is important to document.

---

## §B — Thesis content NOT in the code

### B1. Clarke's Dual Action Principle (`clarkedual-action-principle.tex`)

The thesis develops the full Fenchel duality machinery: primal problem, dual functional `I_K`, dual constraint set `M(K)`, primal-dual equivalence theorem (`thm:primal-dual-equivalence`). The code does not implement any of the dual problem — it operates entirely on the primal (combinatorial search over σ, β). The dual is needed only for the existence proof.

**Action:** `[accept-divergence]` — the dual is theoretical scaffolding for existence proofs, not the computational path.

---

### B2. Existence theorems (cited from literature)

`thm:orbit-existence-smooth` (Rabinowitz 1978), `thm:orbit-existence-polytope` (AAO 2014), and `thm:simple-minimizer` (HK2017) are proved in the thesis (or cited and applied). None have code counterparts — they justify *why* the algorithm works, not how to implement it.

**Action:** `[accept-divergence]` — pure theory.

---

### B3. `SimpleOrbit` struct mentioned in `rem:simple-orbit-data`

The thesis has this QC comment (rendered as a Remark):

> *Rust: `struct SimpleOrbit { sigma: Vec<FacetIdx>, tau: Vec<f64> }`*

No such struct exists in the codebase. The code tracks `sigma` as `Vec<usize>` inside `EhzResult::best_permutation`. The τ values (dwell times) can be recovered from the β vector but are not stored as a named struct.

**Action:** `[jörn-decision]` — either (a) create the `SimpleOrbit` struct in Rust and update the remark to match, or (b) update the remark to accurately describe what the code actually stores (i.e., reference `EhzResult::best_permutation` and note τ is implicit in β).

---

### B4. Billiard trajectory formulation (theory vs code)

`lagrangian-product-algorithm-proof.tex` defines:
- `def:billiard-trajectory` — a closed K_p-billiard trajectory and its K_p°-length
- `thm:billiard-characterization` — EHZ capacity = min billiard length
- `thm:bounce-bound` — at most 3 bounces

The code (`algorithms::billiard`) implements the sigma-structure approach directly (enumerating block-structured permutations `([QQ|Q][PP|P])^k` for k ∈ {2,3}). It does not implement "billiard trajectories" as a concept — the sigma-structure enumeration is the code-level realization of the theoretical billiard characterization.

**Action:** `[add-code-comment]` on `billiard_capacity`: note that the sigma-structure enumeration is the computational implementation of `thm:billiard-characterization` and `thm:bounce-bound`.

---

### B5. Capacity axioms (`lem:capacity-axioms`)

Monotonicity, conformality, symplectic invariance, and normalization are stated and used in the thesis. They are tested empirically in `correctness.tex` (conformality, symplectic invariance, monotonicity) but not encoded as types or formal assertions.

**Action:** `[accept-divergence]` — tested empirically, which is the appropriate approach for a computational implementation.

---

### B6. Smooth setting definitions

`def:closed-characteristic`, `def:reeb-orbit-smooth`, `lem:action-minima-coincide` are purely definitional/theoretical. No code handles smooth bodies.

**Action:** `[accept-divergence]` — code is restricted to polytopes throughout.

---

## §C — Genuine mismatches (thesis says X, code does Y)

### C1. Sign convention in `q_from_beta` vs thesis double sum

**Thesis** (`general-case-algorithm-proof.tex`, `lem:H-quadratic`):
```
Q(β) = Σ_{j < i} β_j β_i ω₀(n_{σ(j)}, n_{σ(i)})
```
Lower index `j` appears first in the ω₀ argument.

**Code** (`kkt.rs`, `q_from_beta`):
```rust
// Σ_{i > j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)})
```
Higher index `i` appears first in the ω₀ argument.

Since ω₀ is antisymmetric, these are negatives of each other. The code maximizes Q (which is positive for reverse traversals), while the thesis minimizes A = 1/(2Q) (positive for forward traversals). Both find the same action value `0.5/|Q(β)|`, so the capacity is correct.

The observable consequence: `EhzResult::best_permutation` is the **cyclic reverse** of the orbit described in the thesis. If Jörn ever reads off a specific orbit from code output and tries to verify it against the theoretical formula, the orientation will be flipped.

`appendix-notation.tex` notes that "HK2017's MATLAB implementation minimizes −Q(σ,β); our Rust code maximizes Q directly. The optima coincide." This captures half the story (optima agree) but does not explain that the returned permutation is reversed relative to the thesis double sum.

**Action:** `[jörn-decision]` — options: (a) flip the sign in `q_from_beta` to match the thesis and update the minimization logic accordingly; (b) add a code comment in `q_from_beta` and `ehz_capacity` explaining that the returned permutation is the cyclic reverse of the thesis orbit; (c) add a note in `appendix-notation.tex` explaining the permutation reversal. Option (b) or (c) is the minimal fix; option (a) is the clean fix but requires care.

---

### C2. Stale code comment in `solve_kkt`

**Code comment (current):**
> "Note: chapter-algorithm.tex `eq:linear-system` omits the ν multiplier, making the system overdetermined. We use the correct KKT system here."

**Thesis (current):** `general-case-algorithm-proof.tex` now includes ν in the (m+5)×(m+5) KKT system, matching the code exactly. The comment was written when the thesis had an older, incorrect version of the system.

**Action:** `[add-code-comment]` — remove or update the stale comment. The thesis and code now agree; no mismatch note is needed. This is a pure cleanup.

---

### C3. No explicit labeling of which variant is "production"

The thesis presents two related algorithms:
1. `alg:ehz` in §2.1 — the full exhaustive search
2. `cor:adjacency-pruning` in §2.2 — the pruning corollary

But the thesis never says: "all experiments in §4 use the pruned variant." A reader implementing the thesis would implement §2.1 and miss the pruning used in practice.

**Action:** `[add-thesis-note]` in `experiments.tex` or `benchmarks.tex`: one sentence stating which function (`ehz_capacity_pruned`) is used in all experiments, with a cross-reference to the pruning corollary.

---

### C4. `rem:simple-orbit-data` references a non-existent Rust struct

See B3 above. The thesis remark contains a code snippet for `struct SimpleOrbit { sigma: Vec<FacetIdx>, tau: Vec<f64> }` which does not exist. Anyone trying to find this struct in the codebase will fail.

**Action:** `[jörn-decision]` — see B3.

---

### C5. Reeb vector drops 2/h factor in `reeb_trajectory.rs`

**Thesis definition** (`basic-definitions.tex`):
```
R_i = (2/h_i) J₀ n_i
```

**Code** (`geom/reeb_trajectory.rs`, `reeb_vector()`):
```rust
// Returns J₀ n (direction only; 2/h scale omitted for visualization)
```
The code has a comment explaining the omission. The capacity computation in `kkt.rs` uses `H = η ⊗ η^T` (which absorbs the `h` factors) so the capacity is correct. But for visualization, the trajectory speed is not scaled by `2/h_i`.

**Action:** `[accept-divergence]` given the code already has a comment explaining the deviation. The visualization is for qualitative insight only; the scale factor doesn't affect the shape. Suggest ensuring the existing comment clearly says "does not affect capacity" to avoid confusion.

---

### C6. Crosspolytope capacity is a placeholder

**Code** (`geom/known_polytopes.rs`): the crosspolytope entry has `capacity: None` (placeholder, capacity unknown).

**Thesis** (`correctness.tex`, `tab:literature-polytopes`): the crosspolytope does not appear in the table of literature values.

The crosspolytope is included in `all_known()` (so it participates in some tests) but excluded from `literature_values()` (so it does not appear in the correctness table). This is intentional but undocumented.

**Action:** `[add-code-comment]` on the crosspolytope entry: add a note explaining why it is excluded from `literature_values()` (capacity not in literature). If Jörn wants to include it in the correctness table once the capacity is known, the code is already structured for it.

---

### C7. Two normalizations not cross-linked to code

`appendix-notation.tex` distinguishes "period normalization" (this thesis) from "HK2017 normalization" (where γ runs on [0,1] instead of [0,T]). The code uses period normalization throughout. No code comment says "this uses period normalization" to help a reader cross-referencing the appendix.

**Action:** `[add-code-comment]` on `q_from_beta` or the KKT system builder: one line noting the code uses period normalization (γ: [0,T]) as defined in `appendix-notation.tex`, consistent with the thesis.

---

## §D — Missing cross-references and open questions

### D1. SVD gap correctness limitation (26/23,650 cases) — not in thesis

The `SVD_GAP_THRESHOLD` doc comment in `kkt.rs` documents a known correctness gap:

> "In a correctness test on 23,650 F=7 (S,σ) pairs, 26 cases had their null space over-truncated to rank 1 by this threshold. Those orbits are dropped rather than found."

This is a real (if small) correctness limitation: for F=7 polytopes with degenerate KKT systems, the algorithm may miss 26 out of ~23,650 candidate orbits. In practice the minimum-action orbit is unlikely to be one of these, but it is not guaranteed.

The thesis does not mention this anywhere — not in the algorithm section, not in the correctness experiment, not as a limitation.

**Action:** `[jörn-decision]` — should the thesis note this as a known limitation of the implementation? The honest answer is "yes, for correctness completeness," but the practical impact is very small. If added, the right place is `correctness.tex` or as a footnote in the algorithm section.

---

### D2. Dataset section commented out in `experiments.tex`

```latex
%% Section 1: The dataset — polytope groups, algorithms, data files
%\input{experiments/dataset}
```

This section stub is commented out. The benchmark and correctness tables implicitly describe the polytopes used, but there is no dedicated polytope dataset section.

**Action:** `[thesis-todo]` (for Jörn) — either write the dataset section or remove the commented-out stub. The correctness table (`tab:literature-polytopes`) and benchmark descriptions cover most of what a dataset section would say; may not be needed.

---

### D3. Stale source path comments in experiment `.tex` headers

Several experiment `.tex` files list source paths in their header comments that use the pre-restructuring path style:

- `pentagon-perturb.tex`: `experiments/data/pentagon-perturb.jsonl`, `experiments/scripts/pentagon_perturb.py`
- `random-product-sweep.tex`: same pattern

The actual paths are `experiments/pentagon-perturb/pentagon-perturb.jsonl` etc. (colocated under the experiment folder).

**Action:** `[add-thesis-note]` — update the header comments in these `.tex` files to reflect the current colocated paths. (Minor comment cleanup, not a functional mismatch.)

---

### D4. `appendix-notation.tex` notation gaps

The notation table omits a few concepts that appear in the code or proofs:

- `β` (dual variables / orbit weights) — appears throughout the algorithm but has no row in the table
- `σ` (cyclic permutation / orbit structure) — same
- `Q(β)` (the bilinear form) — not in the table
- Adjacency graph — listed as `---` / `---` but the concept is defined in `cor:adjacency-pruning`

**Action:** `[jörn-decision]` — is the notation table intended to be a complete glossary or just a "thesis vs HK2017" cross-reference? If the former, β, σ, Q should be added.

---

## Summary checklist

| Item | Tag | Location |
|---|---|---|
| C1: q_from_beta sign / permutation reversal | `[jörn-decision]` | `kkt.rs`, `appendix-notation.tex` |
| C2: Stale `solve_kkt` comment (ν multiplier) | `[add-code-comment]` | `kkt.rs` |
| C3: No "experiments use pruned variant" statement | `[add-thesis-note]` | `experiments.tex` / `benchmarks.tex` |
| C4: `rem:simple-orbit-data` cites non-existent struct | `[jörn-decision]` | `simple-minimizer-existence.tex` |
| C5: Reeb vector drops 2/h in visualization | `[accept-divergence]` | `reeb_trajectory.rs` |
| D1: SVD gap correctness limitation (26/23650) | `[jörn-decision]` | `kkt.rs`, `correctness.tex` |
| A1: KKT numerical machinery not in thesis | `[add-thesis-note]` (brief) | `general-case-algorithm-proof.tex` |
| A2: Two HK2017 variants, production not named | `[add-thesis-note]` | `general-case-algorithm.tex` |
| A3: Tube algorithm placeholder in code | `[jörn-decision]` | `algorithms/tube/mod.rs` |
| A4: Reeb trajectory simulation not described | `[add-thesis-note]` | `visualization.tex` |
| A6: Volume via qhull not described | `[add-thesis-note]` | `correctness.tex` or `rejection-sampling.tex` |
| B3 / C4: SimpleOrbit struct doesn't exist | `[jörn-decision]` | Duplicate of C4 |
| B4: Billiard sigma-structure not linked to theory | `[add-code-comment]` | `algorithms/billiard/mod.rs` |
| C6: Crosspolytope capacity placeholder | `[add-code-comment]` | `geom/known_polytopes.rs` |
| C7: Normalization choice not in code | `[add-code-comment]` | `kkt.rs` |
| D2: Dataset section stub commented out | `[thesis-todo]` | `experiments.tex` |
| D3: Stale path comments in `.tex` headers | `[add-thesis-note]` | experiment `.tex` headers |
| D4: Notation table missing β, σ, Q | `[jörn-decision]` | `appendix-notation.tex` |
