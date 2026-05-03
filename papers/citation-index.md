# Citation Index

Verified theorem/section numbers for results cited in `crates/**/math.tex`.
Agents: read this file instead of re-searching books/papers.

Last updated: 2026-05-03.

## Source availability

| Key | Source | Location | Status |
|-----|--------|----------|--------|
| HK2017 | Haim-Kislev (2017), EHZ-polytopes | `papers/hk2017/EHZ-polytopes.tex` | available, LaTeX source |
| CH2021 | Cieliebak-Haim-Kislev (2021) | `papers/ch2021/systolic_paper.tex` | available, LaTeX source |
| HKO2024 | Haim-Kislev-Ostrover (2024) | `papers/hko2024/counterexample.tex` | available, LaTeX source |
| BBLM2023 | Baracco-Bernardi-Lerario-Mondino | external PDF only; not stored in repo | not available as repo source |
| Higham2002 | Higham, "Accuracy and Stability", 2nd ed., SIAM | `papers/Higham_2002.pdf` | local only (.gitignore) |
| GVL2013 | Golub & Van Loan, "Matrix Computations", 4th ed. | `papers/GVL4_2013.pdf` | local only (.gitignore) |
| BGL2005 | Benzi-Golub-Liesen, "Saddle point problems" | external PDF only; not stored in repo | available externally (author-hosted) |
| CHLS2007 | Cieliebak-Hofer-Latschev-Schlenk, "Quantitative symplectic geometry" | external PDF only; not stored in repo | available externally (MSRI) |
| HoferZehnder1994 | Hofer-Zehnder, "Symplectic Invariants and Hamiltonian Dynamics" | NOT available (Springer paywall) | cite Ch. 2 axioms |

## Numerical linear algebra

### Higham (2002) — "Accuracy and Stability of Numerical Algorithms", 2nd ed.

Chapter structure (verified from PDF ToC):
- Ch. 1: Principles of Finite Precision Computation
- Ch. 2: Floating Point Arithmetic
- Ch. 3: Basics (§3.1 Inner and Outer Products)
- Ch. 4: Summation
- Ch. 5: Polynomials
- Ch. 6: Norms (§6.4 SVD — mathematical tool only)
- Ch. 7: Perturbation Theory for Linear Systems (§7.1 Normwise, §7.2 Componentwise, §7.4 Matrix Inverse)
- Ch. 8: Triangular Systems (§8.1 Backward Error)
- Ch. 19: QR Factorization
- Ch. 20: Least Squares Problem

| Result | Location | Page | Statement snippet |
|--------|----------|------|-------------------|
| Dot product rounding (γₙ bound) | **§3.1, Lemma 3.1** | ~50 | `|x^T y - ŝ| ≤ γ_n |x|^T |y|` where `γ_n = nu/(1-nu)` |
| Triangular solve backward error | **Theorem 8.5** | 171 | `(T + ΔT)x̂ = b` with `|ΔT| ≤ γ_n |T|` for substitution in any order |
| Normwise perturbation bound | **Theorem 7.2** | ~149 | Forward error bound `‖x-y‖/‖x‖ ≤ ...` assuming `ε‖A⁻¹‖‖E‖ < 1` |
| Banach perturbation lemma | **Not a standalone lemma** | — | Used inline in Ch. 7 proofs. The result `‖(B+E)⁻¹‖ ≤ ‖B⁻¹‖/(1 - ‖B⁻¹‖‖E‖)` is a standard Neumann series bound; Higham doesn't give it a separate number. Best cited as "standard; see e.g. Higham (2002), proof of Thm. 7.2" or Horn-Johnson (2013). |
| SVD backward stability | **Not in Ch. 5** (Ch. 5 = Polynomials). In **Ch. 19** (QR). | ~430+ | No separately numbered theorem found in Higham for SVD backward stability specifically. |

### Golub & Van Loan (2013) — "Matrix Computations", 4th ed.

Chapter structure (verified from PDF):
- Ch. 2: Matrix Analysis (§2.4 SVD, §2.6 Sensitivity of Square Systems)
- Ch. 3: General Linear Systems (§3.1 Triangular Systems)
- Ch. 8: Symmetric Eigenvalue Problems (§8.1 Properties, §8.6 Computing the SVD)

| Result | Location | Page | Statement snippet |
|--------|----------|------|-------------------|
| Weyl eigenvalue perturbation | **Corollary 8.1.6** | 442 | `|λ_k(A+E) - λ_k(A)| ≤ ‖E‖_2` for symmetric A, A+E |
| Weyl singular value perturbation | **Corollary 8.6.2** | 487 | `|σ_k(A+E) - σ_k(A)| ≤ σ_1(E) = ‖E‖_2` for A, A+E ∈ ℝ^{m×n} |
| SVD backward stability | **Algorithm 8.6.2** preamble | 492 | `U^T A V = D + E` with `‖E‖_2 ≈ u‖A‖_2`. Not a numbered theorem. |
| Sensitivity of square systems | **§2.6** | — | First-order perturbation identity `(A+δA)⁻¹(b+δb) - A⁻¹b = -A⁻¹(δA·x + δb) + O(‖δA‖²)` |

## Perturbation theory (original papers)

| Result | Source | Year | Journal | Key statement |
|--------|--------|------|---------|---------------|
| Pseudoinverse perturbation | **Wedin** | 1973 | BIT 13, 217–232 | `‖(C+δC)⁺ - C⁺‖ ≤ √2 ‖C⁺‖² ‖δC‖ + O(‖δC‖²)` for rank-preserving perturbations. Central result of the paper. |
| Pseudoinverse perturbation (survey) | **Stewart** | 1977 | SIAM Review 19, 634–662 | Survey covering Wedin's results and extensions. |
| Weyl eigenvalue inequality | **Weyl** | 1912 | Math. Annalen 71, 441–479 | Original paper. Modern ref: Horn-Johnson (2013) Thm 4.3.1. |

## Symplectic geometry

### HK2017 — Haim-Kislev, "On the EHZ capacity of polytopes"

Numbering uses a shared counter (Theorem, Corollary, Remark all in one sequence).

| Number | Type | Label in source | Statement |
|--------|------|-----------------|-----------|
| 1.1 | Theorem | `formula_theorem` | EHZ capacity of a polytope equals the maximum of a quadratic program |
| 1.2 | Corollary | — | — |
| 1.3 | Remark | — | — |
| 1.4 | **Remark** | `abbondandolo_remark` | Clarke's dual action principle connection. **NOT a Theorem.** |
| 1.5 | Theorem | `simple_loop_theorem` | Minimum-action orbit is simple: visits each facet at most once on a connected time interval |
| 1.6 | Remark | — | — |
| 1.7 | Conjecture | — | — |
| 1.8 | Theorem | — | — |

### Billiard characterization

| Result | Source | Theorem # |
|--------|--------|-----------|
| c_EHZ(K_q × K_p) = min billiard length | Artstein-Avidan & Ostrover (2014) | **Theorem 2.13** |
| Same, extended to Minkowski billiards | Rudolf (2022) | **Theorem 1** |
| Bounce bound (at most n+1 vertices) | Bezdek & Bezdek (2009) | **Theorem 1.1, Lemma 2.4** |

### Continuity of c_EHZ

| Claim | Source | Location |
|-------|--------|----------|
| c_EHZ continuous on convex bodies w.r.t. Hausdorff | Hofer-Zehnder (1994) | Ch. 2, monotonicity axiom (A1). No standalone theorem. |
| Explicit discussion | CHLS (2007) | MSRI Publ. vol. 54, pp. 1–44 |
| Polytopes covered directly | (axioms) | No approximation argument needed |

### Benzi-Golub-Liesen (2005) — "Numerical solution of saddle point problems"

| Result | Location | Statement |
|--------|----------|-----------|
| Theorem 3.2 | p. 16 | **Nonsingularity criterion**: A SPD + B full rank + C=0 ⟹ saddle-point matrix nonsingular iff ker(A)∩ker(B)={0}. **NOT the inertia result.** |
| Inertia result | §3.3, body text after eq. (3.9), p. 21 | "A has n positive and m negative eigenvalues" via Sylvester's Law of Inertia. Not a numbered theorem. |
