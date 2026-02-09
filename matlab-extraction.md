# MATLAB Reference Implementation: EHZ Capacity (Haim-Kislev 2017)

Source: https://github.com/pazithaimkislev/EHZ-capacity (commit `22dd473`)
Author: Pazit Haim-Kislev
Paper: https://link.springer.com/article/10.1007/s00039-019-00486-4

---

## 1. File Structure

| File | Purpose |
|------|---------|
| `EHZ_perms.m` | **Main entry point.** Computes the EHZ capacity of a polytope. Contains the main function plus two inner helper functions (`calculateForPerm`, `calculateForPermSingleSol`) and a tiny `omega` function. |
| `KtimesT.m` | Constructs the Lagrangian product K x T from two lower-dimensional polytopes (vertex-level Cartesian product with coordinate reordering). |
| `barycenter.m` | Computes the barycenter (centroid) of K via Delaunay triangulation, weighting simplex barycenters by volume. |
| `calcNormalsAndFaces.m` | Computes the convex hull, extracts facets (as vertex-index sets), computes outward unit normals. Also handles degenerate simplices. Contains inner function `getNormal`. |
| `calcHeights.m` | Computes the support value (height) of each facet: h_i = dot(v, n_i) for any vertex v on facet i. |
| `prepareAeqAndBeq.m` | Analyzes the linear system for a given facet subset: determines rank, removes redundant constraints, classifies as 0 / 1 / infinitely many solutions. |

---

## 2. Algorithm Flow

### Entry: `EHZ_perms(K)`

**Input:** K is an m x 2n matrix. Each row is a vertex of a polytope in R^{2n}. Coordinate system: (q_1, ..., q_n, p_1, ..., p_n).

**Step 1: Center the polytope**
```matlab
C = barycenter(K);
K = K - repmat(C, size(K,1), 1);
```
Translates K so its barycenter is at the origin. The barycenter is computed via Delaunay triangulation -- each simplex is weighted by |det| (proportional to volume), and the centroid of each simplex is the mean of its vertices.

Note: this is the **barycenter** (volume-weighted centroid), NOT the centroid of vertices. The formula: for each simplex with vertices v_0, ..., v_{2n}, the simplex barycenter is (v_0 + ... + v_{2n}) / (2n+1), weighted by simplex volume. The code computes `C = sum(vol_i * sum(vertices_i)) / ((n+1) * sum(vol_i))` where n = 2n (the dimension), giving the correct volume-weighted centroid.

**Step 2: Compute facets, normals, heights**
```matlab
[normals, inds, K] = calcNormalsAndFaces(K);
heights = calcHeights(K, inds, normals);
```
- Uses MATLAB's `convhulln` with `'QJ'` option (joggle input to handle degeneracies)
- Filters out sub-simplices of rank < n-1 (degenerate faces)
- Merges coplanar simplices via `uniquetol` on normals (tolerance 1e-8)
- Orients normals outward by comparing determinant signs
- Heights: h_i = dot(any_vertex_on_facet_i, normal_i) -- the support value

After this step, K has been re-indexed to include only vertices on the convex hull.

**Step 3: Precompute all pairwise omega values**
```matlab
allOmegas(i,j) = omega(normals([i,j],:), n);
```
where:
```matlab
omega(V, n) = V(1,1:n)*V(2,n+1:2n)' - V(1,n+1:2n)*V(2,1:n)'
```
This is the standard symplectic pairing: omega(u,v) = sum_{i=1}^n (u_{q_i} * v_{p_i} - u_{p_i} * v_{q_i}). The matrix `allOmegas` is antisymmetric: allOmegas(j,i) = -allOmegas(i,j).

**Step 4: Enumerate all facet subsets** (the main loop)

```matlab
for j = 2:k          % j = number of facets in subset
    allChoices = nchoosek(1:k, j);   % all C(k,j) subsets of size j
    parfor i = 1:size(allChoices,1)
        currentPerm = allChoices(i,:);
        ...
    end
end
cap = -1/(2*minCap);
```

For each subset of facets (called `currentPerm` but it is a **subset**, not yet an ordering):
1. Build the equality constraint system
2. Determine feasibility via `prepareAeqAndBeq`
3. If feasible, optimize over all orderings of that subset

The outer loop goes over subset sizes j = 2, 3, ..., k. The inner `parfor` parallelizes across all C(k,j) subsets of that size.

**Final capacity:** `cap = -1/(2*minCap)`. The negative sign and factor of 2 convert the minimum of the quadratic form (which is negative for valid orbits) to the positive capacity value.

---

## 3. Inner Optimization: How beta is solved for a fixed permutation

This is the **core algorithmic insight**. The approach is NOT a QP solver. It is a **direct KKT solve via linear algebra**.

### Setting up the constraint system

For a chosen facet subset S = {i_1, ..., i_j}, the constraint system is:

```
Aeq = [normals(S,:)'; heights(S)]
beq = [0; 0; ...; 0; 1]       (2n zeros, then 1)
```

This encodes two constraints:
- **Closure:** sum_i beta_i * n_i = 0 (the orbit is closed) -- the first 2n rows
- **Normalization:** sum_i beta_i * h_i = 1 (visit-time normalization) -- the last row

So the system is (2n+1) equations in j unknowns (the beta values).

### Feasibility analysis: `prepareAeqAndBeq`

```matlab
function [Aeq, beq, numOfSol] = prepareAeqAndBeq(Aeq, beq)
```

1. Zero out tiny entries: `Aeq(abs(Aeq) < 1e-10) = 0`
2. QR-factorize Aeq' to find its rank
3. QR-factorize [Aeq; beq]' to check if beq is in the column space of Aeq
4. Three outcomes:
   - `numOfSol = 0`: beq is not in span(Aeq) -- no solution exists. **Skip this subset.**
   - `numOfSol = 1`: Aeq is square and full rank -- unique solution. **Solve directly.**
   - `numOfSol = Inf`: underdetermined -- free parameters exist. **Need to optimize.**

Redundant constraints (linearly dependent rows) are removed by selecting only the pivot rows from the QR decomposition.

### Case 1: Unique solution (`numOfSol == 1`)

```matlab
betaArr = Aeq \ beq;
if (all(betaArr > -1e-10))
    allCaps(i) = calculateForPermSingleSol(allOmegas(currentPerm,currentPerm), betaArr);
end
```

The beta vector is uniquely determined. Check that all components are non-negative (with tolerance -1e-10). If so, evaluate the action for every ordering of the facets in the subset, and take the minimum.

`calculateForPermSingleSol` enumerates all (j-1)! orderings (fixing the first element), and for each ordering sigma, computes:
```
action = beta(sigma)' * H(sigma,sigma) * beta(sigma) / 2
```
where H is the lower-triangularized omega matrix (see below).

### Case 2: Infinitely many solutions (`numOfSol == Inf`)

```matlab
allCaps(i) = calculateForPerm(allOmegas(currentPerm,currentPerm), Aeq, beq);
```

This is the more complex case. `calculateForPerm` enumerates all (j-1)! orderings and, for each ordering, solves a **KKT system** to find the optimal beta.

### The KKT system in `calculateForPerm`

For a given ordering (permutation) sigma of the j facets:

**Step 1: Build the action matrix H**
```matlab
H = allOmegas(currentPerm, currentPerm);
H = H - triu(H) + tril(H)';
```
This transforms the antisymmetric omega matrix into a **symmetric** matrix. Specifically:
- `triu(H)` is the strict upper triangle of the antisymmetric matrix
- `tril(H)'` is the transpose of the lower triangle, which equals the upper triangle with signs flipped
- `H - triu(H) + tril(H)'` = lower triangle + lower triangle = **2 * tril(H, -1)** plus the diagonal (which is zero)

Wait, let me trace this more carefully. If A is antisymmetric (A_{ij} = -A_{ji}, A_{ii} = 0):
- triu(A) includes diagonal (0) and upper triangle
- tril(A)' transposes the lower triangle to become an upper triangle
- A - triu(A) = strictly lower triangle of A
- tril(A)' = upper triangle version of lower triangle of A = entries A_{ji} placed at position (i,j) for j>i = -A_{ij} at position (i,j) for j>i

So: H = (strictly lower triangle of A) + (-A_{ij} for j>i in the upper triangle)

Actually: H_{ij} for i>j: = A_{ij} (from the lower triangle part)
H_{ij} for i<j: = tril(A)'_{ij} = A_{ji} = -A_{ij}
H_{ij} for i=j: = 0

So H_{ij} = A_{ij} for i >= j, and H_{ij} = -A_{ij} for i < j. But A_{ij} = -A_{ji}, so for i < j: H_{ij} = -A_{ij} = A_{ji}. And for i > j: H_{ij} = A_{ij} = -A_{ji}.

So H_{ij} = -A_{ji} for all i != j, i.e., H = -A^T = A (since A is antisymmetric). That can't be right.

Let me re-derive. Start with A antisymmetric. Then:
- `triu(A)`: the upper triangle (including diagonal, which is 0). Entries: A_{ij} for i <= j.
- `tril(A)`: the lower triangle (including diagonal). Entries: A_{ij} for i >= j.
- `tril(A)'`: transpose of lower triangle. Entry (i,j) of tril(A)' = entry (j,i) of tril(A) = A_{ji} if j >= i, else 0.

So tril(A)' has entry (i,j) = A_{ji} for i <= j, and 0 for i > j.

Now H = A - triu(A) + tril(A)':
- For i > j (lower triangle): H_{ij} = A_{ij} - 0 + 0 = A_{ij}
- For i < j (upper triangle): H_{ij} = A_{ij} - A_{ij} + A_{ji} = A_{ji} = -A_{ij}
- For i = j: H_{ii} = 0 - 0 + A_{ii} = 0

So H_{ij} = A_{ij} for i > j, and H_{ij} = -A_{ij} = A_{ji} for i < j.

This means **H is symmetric**, with H_{ij} = A_{ij} for i > j (= omega(n_{sigma(i)}, n_{sigma(j)}) for i > j).

The quadratic form beta' * H * beta / 2 equals sum_{i>j} A_{ij} * beta_i * beta_j = sum_{i>j} omega(n_{sigma(i)}, n_{sigma(j)}) * beta_i * beta_j.

This is the **action functional** for the orbit with ordering sigma: the action of a piecewise-linear characteristic on the boundary of K.

**Step 2: Build the KKT matrix**
```matlab
matWithLagrangeCoef = zeros(j + constraints);
matWithLagrangeCoef(1:j, 1:j) = H;
matWithLagrangeCoef(1:j, j+1:end) = Aeq';
matWithLagrangeCoef(j+1:end, 1:j) = Aeq;
```

This is the classic KKT matrix for minimizing beta' * H * beta / 2 subject to Aeq * beta = beq **without inequality constraints**. The system is:

```
[ H    Aeq' ] [ beta   ]   [ 0   ]
[ Aeq  0    ] [ lambda ] = [ beq ]
```

The stationarity condition H*beta + Aeq'*lambda = 0 comes from setting the gradient of the Lagrangian to zero. The constraint Aeq*beta = beq enforces closure and normalization.

**Step 3: Check regularity and solve**
```matlab
if (abs(det(matWithLagrangeCoef)) < 1e-10)
    continue;    % skip singular systems
end
betaArr = matWithLagrangeCoef \ [zeros(j,1); beq];
betaArr = betaArr(1:j);     % extract beta, discard Lagrange multipliers
```

**Step 4: Check non-negativity and compute action**
```matlab
newCap = betaArr' * H * betaArr / 2;
if (all(betaArr > -1e-10))
    allCaps(i) = newCap;
end
```

Key observation: the **non-negativity constraint beta_i >= 0** is NOT enforced in the KKT system. Instead, the code solves the **unconstrained** (equality-constrained only) KKT, then **checks** if the solution happens to satisfy beta_i >= 0. If not, the solution is discarded.

**Why this works:** The paper proves that the minimum-action orbit has all beta_i > 0 for the correct facet subset and ordering. If the unconstrained optimum has a negative beta_i, it means either (a) this facet subset/ordering is not the one achieving the minimum, or (b) the minimum for this ordering lies on the boundary of the feasible region (beta_i = 0), which corresponds to a shorter orbit visiting fewer facets -- and that orbit will be found when the algorithm processes the corresponding smaller subset.

**Step 5: Take minimum over all orderings**
```matlab
minCap = min(allCaps);
```

### Summary of the inner optimization

The approach is:
1. For each facet **subset**, check if the equality system (closure + normalization) is feasible
2. If uniquely determined: compute beta directly, check non-negativity, evaluate action for all orderings
3. If underdetermined: for each **ordering**, solve the KKT system (a single linear solve), check non-negativity, evaluate action
4. Take the global minimum over all subsets and orderings

**There is no QP solver, no iterative optimization, no active-set method.** The optimization is purely algebraic: solve a linear system derived from KKT conditions, check feasibility, evaluate.

---

## 4. Enumeration Strategy

### No graph pruning

The MATLAB code does **NOT** use any adjacency graph or pruning. The README explicitly states:

> "Currently, this implementation is not very efficient and it is very slow when the polytope has a large number of faces. However, one can significantly improve the running time by eliminating many permutations based on the fact that the minimal permutation should correspond to a closed characteristic on the boundary of K (see Remark 3.11 in the aforementioned paper). I will upload an updated version soon."

### Enumeration structure

The enumeration has **two nested levels:**

1. **Outer: all subsets of facets** -- `nchoosek(1:k, j)` for j = 2, ..., k. This generates all C(k,j) unordered subsets of size j.

2. **Inner: all orderings of each subset** -- `perms(2:j)` prefixed with 1. This generates (j-1)! orderings (fixing the first element to avoid counting cyclic rotations).

Total work: sum_{j=2}^{k} C(k,j) * (j-1)! = sum_{j=2}^{k} k! / (j * (k-j)!)

This is **factorial in k** (the number of facets). The README acknowledges this is slow for polytopes with many facets.

### Cyclic symmetry exploitation

Orderings fix the first element to 1:
```matlab
routes = [ones(factorial(size(allOmegas,1)-1), 1), perms(2:size(allOmegas,1))];
```
This reduces by a factor of j (from j! to (j-1)!), exploiting the cyclic nature of orbits. However, it does NOT exploit reflection symmetry (sigma and reverse(sigma) give the same orbit), so there is a factor-of-2 redundancy remaining.

### Parallelism

The outer subset loop uses `parfor`, parallelizing across subsets of the same size. This is MATLAB's parallel for-loop construct.

---

## 5. Adjacency Graph

**There is no adjacency graph in this implementation.** The code enumerates all subsets without regard to whether facets are adjacent. The README says this pruning could be added based on Remark 3.11 of the paper but has not been implemented.

---

## 6. Numerical Details

### Tolerances

| Where | Tolerance | Purpose |
|-------|-----------|---------|
| `prepareAeqAndBeq`: zero clamping | 1e-10 | Set near-zero entries to exact zero before rank computation |
| `prepareAeqAndBeq`: rank detection | 1e-8 | Threshold for diagonal R entries in QR to determine rank |
| `calcNormalsAndFaces`: rank check | 1e-10 | Detecting degenerate faces (rank < n-1) |
| `calcNormalsAndFaces`: normal merging | 1e-8 | `uniquetol` tolerance for identifying coplanar simplices |
| `calculateForPerm`: singularity check | 1e-10 | Skipping singular KKT matrices |
| `calculateForPerm`: non-negativity | -1e-10 | Allowing slightly negative beta (numerical noise) |
| `calculateForPerm`: optimal matching | 1e-8 | Identifying which orderings achieve the minimum |
| Main function: non-negativity (unique case) | -1e-10 | Same as above |

### Edge cases handled

1. **Degenerate faces:** Simplices from `convhulln` with rank < n-1 are filtered out before normal computation.

2. **Coplanar simplices:** Multiple simplices sharing the same normal are merged via `uniquetol`. This handles the fact that `convhulln` triangulates non-simplicial facets into multiple simplices.

3. **Singular KKT systems:** When `det(matWithLagrangeCoef) < 1e-10`, the ordering is skipped. The code comments explain: "If there are infinite solutions, one can find a solution where beta_i = 0 for some i, and we will find this solution using a smaller length permutation."

4. **Infeasible constraint systems:** When `beq` is not in the span of `Aeq` (detected by comparing rank of Aeq vs [Aeq, beq]), the subset is skipped entirely.

5. **Joggled convex hull:** The `'QJ'` option to `convhulln` adds small random perturbations to input points to handle degeneracies in the Qhull algorithm.

---

## 7. Algorithmic Choices Not Obvious from the Paper

### 7a. Centering at the barycenter

The code translates K so its **barycenter** (volume-weighted centroid, via Delaunay triangulation) is at the origin before computing normals and heights. The paper's formula requires the origin to be in the interior of K. The barycenter of a convex body is guaranteed to be in its interior, so this is a safe choice. This is a specific implementation decision -- any interior point would work theoretically, but the barycenter is canonical.

### 7b. The action quadratic form is ordering-dependent

A critical subtlety: the quadratic form beta' * H * beta / 2 depends on the **ordering** of the facets, not just the subset. The matrix H is built from the antisymmetric omega matrix by the transformation `H = A - triu(A) + tril(A)'`, which makes it symmetric but with entries that depend on which index is "earlier" in the ordering. Specifically, H_{ij} = omega(n_i, n_j) for i > j (n_i comes later in the ordering), and H_{ij} = omega(n_j, n_i) = -omega(n_i, n_j) for i < j.

The action sum_{i > j} omega(n_{sigma(i)}, n_{sigma(j)}) * beta_i * beta_j has a sign that depends on the relative order of i and j in the permutation sigma. This is why the algorithm must enumerate all orderings, not just subsets.

### 7c. Subset enumeration, not permutation enumeration

The code's outer loop enumerates **unordered subsets** first, then for each feasible subset enumerates orderings. This is a deliberate structure:
- First check if the equality constraints (closure + normalization) are satisfiable for this subset at all
- Only then invest in the expensive (j-1)! ordering enumeration

This avoids redundant feasibility checks -- all orderings of the same subset share the same constraint system (the constraint depends only on which facets are visited, not the order).

### 7d. Short-circuit for uniquely determined beta

When the constraint system Aeq * beta = beq has a unique solution (square full-rank Aeq), the beta vector is the same for all orderings. The code exploits this: instead of solving a KKT system for each ordering, it solves beta once and then just evaluates the quadratic form for each ordering. This is a significant optimization for subsets where the constraints fully determine beta.

### 7e. No non-negativity in the KKT solve

As noted above, the code does NOT use a QP solver with inequality constraints. It solves the relaxed problem (equality constraints only) and post-hoc checks non-negativity. This is justified by the mathematical structure: if the unconstrained optimum violates non-negativity, the constrained optimum lies on the boundary (some beta_i = 0), which corresponds to a smaller facet subset that will be explored separately.

### 7f. The meaning of the output sign

The minimum over all action values is negative (since the action for a valid orbit is negative in this convention). The final capacity is `cap = -1/(2*minCap)`, which:
- Negates the negative minimum to get a positive number
- Divides by 2 as part of the capacity formula

### 7g. Outward normal orientation

The `getNormal` function computes an outward-pointing normal by:
1. Computing a normal vector to the facet via cofactor expansion (the cross product generalization)
2. Normalizing it to unit length
3. Checking orientation: comparing the determinant of (facet vertices shifted by normal) vs (facet vertices shifted by an exterior vertex). If the signs match, the normal points inward and is negated.

### 7h. KtimesT coordinate ordering

The Lagrangian product function `KtimesT(K,T)` produces vertices in the order (q_K, q_T, p_K, p_T)... actually, examining it more carefully:

```matlab
KXT = [repmat(K, size(T,1), 1), repelem(T, size(K,1), 1)];
```

This creates the Cartesian product of vertex sets, with K's coordinates first and T's coordinates second. If K is in R^{n1} with coordinates (q_1, ..., q_{n1/2}, p_1, ..., p_{n1/2}) and T is in R^{n2} similarly, then KXT has coordinates (q_K, p_K, q_T, p_T). This is **NOT** the (q_1, ..., q_n, p_1, ..., p_n) convention stated in the README.

This means `KtimesT` is only correct if K and T are each given in the q,p coordinate order AND the resulting polytope is treated as having coordinates (q_K, p_K, q_T, p_T). However, the omega function uses the convention that the first n coordinates are q and the last n are p. So there may be a coordinate ordering discrepancy for Lagrangian products -- or else K and T are assumed to be in R^n (pure q-space or pure p-space), not in phase space.

Actually, re-reading the README example:
```
Q = [-1 -1; -1 1; 1 -1; 1 1];    % square in R^2 (q-space)
CP = [eye(2); -eye(2)];            % cross polytope in R^2 (p-space)
QXCP = KtimesT(Q, CP);             % produces vertices in R^4: (q1, q2, p1, p2)
```

This is correct: K provides q-coordinates, T provides p-coordinates, and the concatenation [q1,q2,p1,p2] matches the stated convention. So `KtimesT` is specifically for Lagrangian products where the first factor lives in q-space and the second in p-space.

### 7i. Progress tracking

The code computes a time estimate based on the cumulative fraction of the total number of subset+ordering combinations processed, displayed via `waitbar`. The estimate formula:

```matlab
timeEstimate = factorial(k) ./ ((2:k) .* factorial(k-(2:k)));
timeEstimate = cumsum(timeEstimate) ./ sum(timeEstimate);
```

This computes the fraction of total work completed after processing all subsets of size j, where the work for subsets of size j is C(k,j) * (j-1)! = k! / (j * (k-j)!).

---

## 8. Mathematical Summary

The algorithm computes:

```
c_EHZ(K) = min over all facet subsets S, all cyclic orderings sigma of S:
    -1/2 * (1 / action(S, sigma))
```

where:

```
action(S, sigma) = sum_{i > j in sigma-order} omega(n_{sigma(i)}, n_{sigma(j)}) * beta_i * beta_j
```

and beta is the (unique or optimal) solution to:

```
sum_i beta_i * n_i = 0      (closure)
sum_i beta_i * h_i = 1      (normalization)
beta_i >= 0                  (non-negativity / valid visit times)
```

For the case where beta has free parameters, the KKT conditions for minimizing the action subject to the equality constraints are solved as a single linear system.

---

## 9. Complexity

With k facets, the worst-case complexity is:

```
sum_{j=2}^{k} C(k,j) * (j-1)! = sum_{j=2}^{k} k! / (j * (k-j)!)
```

This is O(k! / k) in the worst case, making the algorithm exponential in the number of facets.

The paper (Remark 3.11) suggests pruning based on the adjacency graph of the polytope (consecutive facets in the orbit must share a codimension-2 face), but this pruning is **NOT implemented** in the MATLAB code.

---

## 10. Correctness-Critical Details for Reimplementation

1. **Normal convention:** Outward-pointing unit normals, computed via cofactor expansion, orientation verified against an exterior vertex.

2. **Omega convention:** omega(u,v) = u_q . v_p - u_p . v_q (standard symplectic form in (q,p) coordinates).

3. **Action matrix construction:** For ordering sigma, the action matrix H is the symmetrized omega matrix with H_{ij} = omega(n_{sigma(i)}, n_{sigma(j)}) for i > j, and H_{ij} = -omega(n_{sigma(i)}, n_{sigma(j)}) for i < j.

4. **Capacity formula:** c = -1 / (2 * min_action). The minimum action is negative for valid orbits.

5. **Non-negativity tolerance:** beta_i > -1e-10 is treated as non-negative.

6. **Centering:** The polytope MUST be translated so the origin is in the interior before computing heights. The MATLAB code uses the barycenter.

7. **Height definition:** h_i = dot(n_i, v) for any vertex v on facet i. With outward normals and origin inside K, all heights are positive.

8. **Constraint system:** The (2n+1) x j system has 2n rows for closure and 1 row for normalization. The normalization constraint is sum_i beta_i * h_i = 1 (not sum_i beta_i = 1).

9. **Cyclic symmetry:** Only (j-1)! orderings are needed (fix one element). Reflection symmetry is NOT exploited.

10. **Singular KKT skip:** When the KKT matrix is singular, the ordering is skipped. The mathematical justification is that singular cases correspond to degenerate orbits where some beta_i = 0, which are covered by smaller subsets.
