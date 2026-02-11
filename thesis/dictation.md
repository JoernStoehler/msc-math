# Dictation file for chapter-algorithm.tex

Workflow:
- Jörn writes natural language content under each heading, marks `[ready]`
- Claude reads `[ready]` items, translates to LaTeX, runs QC, updates thesis
- Claude marks items `[question]` if something is unclear or non-mechanical
- Jörn answers questions, re-marks `[ready]`

Statuses: `[draft]` (Jörn working) | `[ready]` (translate now) | `[question]` (Claude needs input) | `[done]` (in thesis, QC passed)

---

## Priority 1: Main result (tells Claude how everything is used)

### thm:main — EHZ capacity algorithm
Status: [done]

Algorithm Pseudocode:
Input: A polytope K \subset R^4 via
  - facet normals n_i \in S^3 and heights h_i > 0, i = 1,...,F
Output: EHZ capacity c_{EHZ}(K)
Process:
  Search and return the minimum value of "A"
  Loop: All subsets S \subset {1,...,F}
    Loop: All permutations \sigma of S, up to cyclic shifts
      Define
        N(i,d) = n_{\sigma(i),d} for i = 1,...,|S|, d = 1,...,4
        H(i,j) = \begin{cases}
          \omega(n_{\sigma(i)}, n_{\sigma(j)}) & i < j \\
          0                                    & i = j
          \omega(n_{\sigma(j)}, n_{\sigma(i)}) & i > j \\
        \end{cases}
      Solve
        [ N^T &  0 ]  * [ \beta  ]  = [ 0 ]
        [ H   & -N ]    [ \lambda]    [ 0 ]
      Filter by
        \beta > 0
      Remark: 
        This is the critical point of
          Q(\beta) = \sum_{1 \leq i < j \leq |S|} \beta_i \beta_j \omega(n_{\sigma(i)}, n_{\sigma(j)})
        subject to
          \sum_i \beta_i n_{\sigma(i)} = 0
          \beta_i > 0
      The value of A for (S, \sigma) is
        A = 2 * Q(\beta)^-1 = (\beta^T H \beta)^-1
  Return the minimum A found

### algorithm — Functional pseudocode
Status: [draft]

---

## Priority 2: Theorems and lemmas (shape the definitions)

### thm:orbit-existence — Existence of Reeb orbits, positive infimum, minimum attained
Status: [draft]

### thm:simple-minimizer — Simple minimizer structure
Status: [draft]

### thm:optimization — Optimization formulation
Status: [draft]

### lem:domain-decomposition — Domain decomposition
Status: [draft]

### lem:global-at-local — Global maximum at local maximum
Status: [draft]

### cor:enumeration — Enumeration computes capacity
Status: [draft]

### lem:local-max — Local maximum characterization
Status: [draft]

### lem:capacity-axioms — Symplectic capacity axioms
Status: [draft]

---

## Priority 3: Definitions (informed by how they're used above)

### def:polytope — Polytope
Status: [draft]

### def:facets — Facets, normals, heights
Status: [draft]

### def:symplectic-form — Standard symplectic form
Status: [draft]

### def:liouville-form — Liouville form
Status: [draft]

### def:convex-body — Convex body
Status: [draft]

### def:normal-cone — Normal cone
Status: [draft]

### def:support-function — Support function
Status: [draft]

### def:gauge-function — Gauge function
Status: [draft]

### def:hamiltonian — Hamiltonian
Status: [draft]

### def:reeb-vectors — Facet Reeb vectors
Status: [draft]

### def:reeb-orbit-smooth — Generalized Reeb orbit (smooth)
Status: [draft]

### def:reeb-orbit-polytope — Generalized Reeb orbit (polytope)
Status: [draft]

### def:action — Action of an orbit
Status: [draft]

### def:ehz-capacity — EHZ capacity
Status: [draft]

### def:dual-variable — Dual variable
Status: [draft]

### def:dual-functional — Dual functional
Status: [draft]

### def:q-function — Q-function
Status: [draft]

### def:constraint-set — Constraint set
Status: [draft]

### def:action-matrix — Action matrix
Status: [draft]