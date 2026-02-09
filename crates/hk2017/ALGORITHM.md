# The HK2017 Algorithm for EHZ Capacity of Convex Polytopes in R^4

This document gives an implementation-ready description of the algorithm from Haim-Kislev 2017 for computing the Ekeland-Hofer-Zehnder (EHZ) capacity of a convex polytope in R^4. It covers all definitions, two theorems with proofs, the algorithm with pseudocode, and a graph-pruned variant.

**Conventions.** We use the standard symplectic convention: J(q,p) = (-p,q) and omega(u,v) = <Ju, v>. See Section A6 for the precise definitions.

---

# A. Definitions and Prerequisites

We work in R^4 with coordinates (q_1, q_2, p_1, p_2).

---

## A1. Convex Polytope in H-Representation

A **convex polytope** K in R^4 is the intersection of finitely many closed half-spaces:

K = { x in R^4 : for all i in {1,...,F}, <x, n_i> <= h_i }

where:

- F is the number of **facets** (3-dimensional faces) of K,
- n_i in S^3 is the **outward unit normal** to facet F_i,
- h_i > 0 is the **oriented height** of facet F_i from the origin.

We require that 0 lies in the interior of K (written 0 in int(K)), which guarantees all heights h_i are strictly positive.

---

## A2. Support Function

The **support function** of K is h_K : R^4 -> R defined by

h_K(y) = sup over x in K of <x, y>

**Properties:**

1. **Positively 1-homogeneous:** h_K(t y) = t h_K(y) for all t > 0.
2. **Convex:** h_K is a convex function on R^4.
3. **Value at normals:** h_K(n_i) = h_i, since the supremum of <x, n_i> over x in K is attained at facet F_i.

---

## A3. Gauge Function (Minkowski Functional)

The **gauge function** of K is

g_K(x) = inf { r > 0 : x/r in K }

Geometrically, g_K(x) measures how far x is from the origin in units of K: the ray from 0 through x hits the boundary of K at the point x/g_K(x).

**Properties:**

1. **Positively 1-homogeneous:** g_K(t x) = t g_K(x) for all t > 0.
2. **Convex:** g_K is a convex function on R^4.
3. **Level sets:** g_K(x) = 1 on the boundary of K, g_K(x) < 1 in the interior, g_K(x) > 1 outside K.
4. **Formula for H-representation polytopes:** g_K(x) = max_{i=1,...,F} <x, n_i> / h_i. The ray from 0 through x exits K at the facet F_i for which <x, n_i> / h_i is largest.

---

## A4. Fenchel Duality Between g_K^2 and (1/4) h_K^2

The functions g_K^2 and (1/4) h_K^2 are **Legendre-Fenchel duals** (convex conjugates) of each other:

(1/4) h_K^2(y) = sup over x of ( <x, y> - g_K^2(x) )

g_K^2(x) = sup over y of ( <x, y> - (1/4) h_K^2(y) )

**Derivation.** We compute the conjugate of g_K^2 directly. Parametrize x = r theta where g_K(theta) = 1 (theta on the boundary of K) and r >= 0. Then g_K^2(x) = r^2 and <x, y> = r <theta, y>. The conjugate is:

(g_K^2)*(y) = sup_{r >= 0, g_K(theta)=1} ( r <theta, y> - r^2 )

For fixed theta with <theta, y> > 0, the function f(r) = r <theta, y> - r^2 is a downward parabola in r with f'(r) = <theta, y> - 2r = 0 at r = <theta, y> / 2, giving maximum value <theta, y>^2 / 4. For <theta, y> <= 0, f(r) <= 0 for all r >= 0, so the supremum is 0 (at r = 0). Therefore:

(g_K^2)*(y) = sup_{g_K(theta)=1} <theta, y>^2 / 4 = [ sup_{g_K(theta)=1} <theta, y> ]^2 / 4 = h_K(y)^2 / 4

The last step uses h_K(y) = sup_{x in K} <x, y> = sup_{g_K(theta) <= 1} <theta, y>, and since <theta, y> is linear, the supremum over the convex set {g_K(theta) <= 1} is attained on the boundary {g_K(theta) = 1}.

**Fenchel inequality (pointwise).** From the conjugate definition:

g_K^2(x) + (1/4) h_K^2(y) >= <x, y>    for all x, y in R^4

**Equality condition.** The inequality becomes an equality if and only if the subdifferential condition holds:

g_K^2(x) + (1/4) h_K^2(y) = <x, y>  iff  y in subdiff(g_K^2)(x)  iff  x in subdiff((1/4) h_K^2)(y)

Here the **subdifferential** of a convex function f at x is the set of all vectors y such that f(z) >= f(x) + <y, z - x> for all z. For smooth f, subdiff(f)(x) = {gradient of f at x}.

This Fenchel duality pair is the key tool for Clarke's dual action principle in Section B.

---

## A5. Outward Normal Cone

For a point x on the boundary of K, the **outward normal cone** N_K(x) collects all outward-pointing directions that are normal to K at x:

N_K(x) = R_+ * conv{ n_i : x in F_i }

where R_+ = [0, infinity) and conv denotes the convex hull.

The structure of N_K(x) depends on where x sits on the boundary:

- **Interior of a facet:** If x lies in the interior of facet F_i, then N_K(x) = R_+ * n_i, a single ray.
- **Edge (2-face):** If x lies on the intersection of two facets F_i and F_j, then N_K(x) = R_+ * conv{n_i, n_j}.
- **Vertex:** If x is a vertex where multiple facets meet, N_K(x) is the cone spanned by all adjacent facet normals.

---

## A6. The Symplectic Form and the Standard Complex Structure

The **standard complex structure** J : R^4 -> R^4 is the linear map

J(q_1, q_2, p_1, p_2) = (-p_1, -p_2, q_1, q_2)

As a matrix in the (q_1, q_2, p_1, p_2) basis:

```
J = [ 0   0  -1   0 ]
    [ 0   0   0  -1 ]
    [ 1   0   0   0 ]
    [ 0   1   0   0 ]
```

That is, J = [0, -I; I, 0] in 2x2 block form.

**Properties of J:**

1. **J^2 = -I:** Applying J twice negates every vector.
2. **J^T = -J:** J is skew-symmetric.

The **standard symplectic form** on R^4 is the skew-symmetric bilinear form

omega(u, v) = <Ju, v> = sum over i of (u_{q_i} v_{p_i} - u_{p_i} v_{q_i})

Equivalently, in differential form notation, omega = dq_1 ^ dp_1 + dq_2 ^ dp_2.

---

## A7. Key Identity: omega(Ju, Jv) = omega(u, v)

The symplectic form is invariant under J: rotating both arguments by J does not change the symplectic pairing.

**Proof.** We compute:

omega(Ju, Jv) = <J(Ju), Jv>           (definition of omega)
              = <J^2 u, Jv>            (associativity)
              = <-u, Jv>               (J^2 = -I)
              = -<u, Jv>               (linearity)
              = -<J^T u, v>            (definition of transpose: <Ax, y> = <x, A^T y>)
              = -<-Ju, v>              (J^T = -J)
              = <Ju, v>                (double negation)
              = omega(u, v)            (definition of omega)

---

## A8. The Liouville 1-Form

The **Liouville 1-form** on R^4 is

lambda_0 = (1/2) <Jx, dx> = (1/2) sum over i of (p_i dq_i - q_i dp_i)

Its exterior derivative recovers the symplectic form: d(lambda_0) = omega.

---

## A9. Facet Reeb Vectors

**Definition (Reeb vector field).** Let (Sigma, alpha) be a (2n-1)-dimensional contact manifold with contact form alpha. The **Reeb vector field** R on Sigma is the unique vector field satisfying:

1. alpha(R) = 1
2. d alpha(R, v) = 0 for all v in T Sigma

**Setup.** The facet F_i lies in the hyperplane H_i = {x in R^4 : <x, n_i> = h_i}, which has tangent space T_x H_i = {v : <v, n_i> = 0} = n_i^perp. The restriction of the Liouville form alpha = lambda_0|_{H_i} is a contact form on H_i, with d alpha = omega|_{H_i}.

**Lemma.** The Reeb vector field of (H_i, lambda_0|_{H_i}) is the constant vector field

R_i = (2 / h_i) J n_i

**Proof.** We verify the three requirements: R_i in n_i^perp, d alpha(R_i, ·) = 0, and alpha(R_i) = 1.

**(a) Tangent to H_i:** <R_i, n_i> = (2/h_i) <Jn_i, n_i> = (2/h_i) omega(n_i, n_i) = 0, since omega is skew-symmetric. So R_i in n_i^perp.

**(b) d alpha(R_i, v) = 0 for all v in n_i^perp:** We have d alpha(R_i, v) = omega(R_i, v) = <JR_i, v> = (2/h_i) <J^2 n_i, v> = -(2/h_i) <n_i, v> = 0 for all v in n_i^perp.

**(c) alpha(R_i) = 1:** On H_i, alpha(R_i) = (1/2) <Jx, R_i> = (1/h_i) <Jx, Jn_i> = (1/h_i) <x, n_i> = (1/h_i) h_i = 1. Here we used <Jx, Jn_i> = <x, n_i>, which holds because J is orthogonal: J^T J = I.

**Notation.** We write p_i = R_i = (2/h_i) J n_i for the Reeb vector of facet i.

**Properties:**

1. **Magnitude:** |p_i| = 2 / h_i, since J preserves lengths. Facets closer to the origin have faster Reeb flow.
2. **Constant on facet:** R_i = (2/h_i) J n_i depends only on n_i and h_i, not on the point x in F_i.

---

## A10. Generalized Closed Characteristic on the Boundary of K

A **generalized closed characteristic** on the boundary of K is a closed loop gamma in the Sobolev space W^{1,2}([0,T], R^4) satisfying:

1. **Boundary constraint:** gamma(t) lies on the boundary of K for all t in [0, T].
2. **Velocity constraint (differential inclusion):** gamma'(t) in J N_K(gamma(t)) a.e.
3. **Closure:** gamma(0) = gamma(T).

At a point in the interior of facet F_i, the velocity is a positive multiple of the Reeb vector p_i. At edges or vertices, the velocity can be any convex combination of adjacent Reeb directions.

---

## A11. Symplectic Action

The **symplectic action** of a closed loop gamma : [0,T] -> R^4 is

A(gamma) = (1/2) integral from 0 to T of <J gamma(t), gamma'(t)> dt

By the algebraic identity <Ju, v> = <-Jv, u> (from J^T = -J), the integrand satisfies <J gamma, gamma'> = <-J gamma', gamma> pointwise, so

A(gamma) = (1/2) integral from 0 to T of <-J gamma'(t), gamma(t)> dt

Geometrically, A(gamma) measures the symplectic area enclosed by the loop. With the standard J, the differential inclusion gamma' in J N_K(gamma) produces counterclockwise orbits, and A is positive for these.

**Key identity for piecewise-constant velocity.** If gamma' is piecewise constant, taking values w_1, ..., w_m on consecutive time intervals I_1, ..., I_m, then for a centered loop (integral of gamma dt = 0):

2 A(gamma) = sum over j < i of |I_i| |I_j| omega(w_j, w_i)

where j < i means segment j is earlier than segment i in the time ordering, and omega(w_j, w_i) has the earlier velocity as the first argument. This is HK2017, Proposition 3.4.

**Proof for two segments.** Consider a closed loop with velocity w_1 on [0, T_1] and velocity w_2 on [T_1, T], where T = T_1 + T_2. The position is gamma(t) = gamma(0) + t w_1 for t in [0, T_1] and gamma(t) = gamma(0) + T_1 w_1 + (t - T_1) w_2 for t in [T_1, T]. Computing the action integrand on each segment and integrating:

integral of <J gamma, gamma'> dt = sum over k of integral over segment k of <J gamma(t), w_k> dt

= sum_k [ T_k <J gamma(0), w_k> + T_k <J(displacement before k), w_k> + (T_k^2 / 2) <J w_k, w_k> ]

The first terms sum to <J gamma(0), T_1 w_1 + T_2 w_2> = 0 by closure. The self-pairing terms vanish: <J w_k, w_k> = omega(w_k, w_k) = 0. The only surviving contribution is from segment 2, where the accumulated displacement T_1 w_1 appears:

integral = T_1 T_2 <J w_1, w_2> = T_1 T_2 omega(w_1, w_2)

Result: 2A = T_1 T_2 omega(w_1, w_2), with the earlier velocity first.

**General case (m segments).** For m segments with velocities w_1, ..., w_m on intervals of duration T_1, ..., T_m, the position on segment k is gamma(t) = gamma(0) + (sum_{l < k} T_l w_l) + (t - t_k) w_k, where t_k is the start time of segment k. The integral of <J gamma, gamma'> splits into three types of terms:

1. **Starting-point terms:** sum_k T_k <J gamma(0), w_k> = <J gamma(0), sum_k T_k w_k> = 0 by closure.
2. **Self-pairing terms:** sum_k (T_k^2 / 2) <J w_k, w_k> = 0 since omega(w_k, w_k) = 0.
3. **Cross-terms:** For each segment k, the accumulated displacement D_k = sum_{l < k} T_l w_l from all earlier segments contributes T_k <J D_k, w_k>. Expanding D_k:

sum_k T_k <J (sum_{l < k} T_l w_l), w_k> = sum_{l < k} T_l T_k <J w_l, w_k> = sum_{j < i} T_j T_i omega(w_j, w_i)

Therefore 2A = sum_{j < i} T_j T_i omega(w_j, w_i), with the earlier velocity first in omega.

---

## A12. EHZ Capacity

The **Ekeland-Hofer-Zehnder (EHZ) capacity** of the convex polytope K is

c_EHZ(K) = min { A(gamma) : gamma is a generalized closed characteristic on the boundary of K }

**Properties:**

1. **Symplectic invariance:** c_EHZ is unchanged under symplectomorphisms.
2. **Translation invariance:** c_EHZ(K + v) = c_EHZ(K).
3. **2-homogeneity:** c_EHZ(lambda K) = lambda^2 c_EHZ(K) for lambda > 0.

---

## A13. Dual Functional

The **dual functional** is

I_K(z) = (1/4) integral from 0 to T of h_K^2(-J z'(t)) dt

This depends only on the velocity z', not on the position z.

---

## A14. Normalizations

Two normalizations appear:

- **Talk normalization (used in proofs):** Curves on [0, T], with A(z) = T. At minimizers, I_K(z) = T.
- **HK2017 normalization (used in the capacity formula):** Curves on [0, 1], with sum beta_i h_i = 1.

We prove the results in talk normalization, then convert to HK2017 normalization in Section D.

---

# B. Clarke's Dual Action Principle

This section proves Clarke's dual action principle, which transforms the problem of finding minimum-action orbits on the boundary of K (the primal problem) into a minimization of the dual functional I_K over a space of loops with no boundary constraint (the dual problem).

---

## B1. The Primal Problem

**Primal problem.** Among all generalized closed characteristics gamma on the boundary of K, find one that minimizes the symplectic action A(gamma). The minimum value is:

c_EHZ(K) = min { A(gamma) : gamma is a generalized closed characteristic on boundary of K }

This is hard to work with directly because of the boundary constraint: gamma must remain on boundary(K) and satisfy gamma'(t) in J N_K(gamma(t)).

---

## B2. The Dual Problem

**Dual problem (talk normalization).** Minimize

I_K(z) = (1/4) integral from 0 to T of h_K^2(-J z'(t)) dt

over all z in W^{1,2}([0,T], R^4) satisfying:

1. **Closure:** z(0) = z(T).
2. **Zero mean velocity:** integral of z'(t) dt = 0 (equivalent to closure).
3. **Centering:** integral of z(t) dt = 0.
4. **Action constraint:** A(z) = T. This fixes a representative in the homothety class: A scales as lambda^2 under z -> lambda z, so every non-trivial loop can be rescaled to satisfy A = T for any T > 0.

---

## B3. Theorem

**Theorem (Clarke's dual action principle).** The primal and dual minimizers correspond one-to-one via z = gamma - center(gamma), where center(gamma) = (1/T) integral of gamma(t) dt. At corresponding minimizers:

I_K(z*) = T = A(gamma*) = c_EHZ(K)

---

## B4. Proof

### Step 1: Fenchel inequality

By the Fenchel duality of Section A4, for any x, y in R^4:

g_K^2(x) + (1/4) h_K^2(y) >= <x, y>

### Step 2: Promote to equality along Hamiltonian orbits

Let gamma be a generalized closed characteristic. The differential inclusion gamma'(t) in J N_K(gamma(t)) implies:

-J gamma'(t) in subdiff(g_K^2)(gamma(t))    a.e.

To see this: gamma'(t) = J eta for eta in N_K(gamma(t)), so -J gamma'(t) = -J^2 eta = eta (using J^2 = -I). On the boundary of K, the normal cone N_K(x) equals the subdifferential subdiff(g_K^2)(x). This is a standard result in convex analysis: for a convex function f, the normal cone to the sublevel set {f <= c} at a boundary point equals subdiff(f) at that point. Here f = g_K^2 and c = 1.

Setting x = gamma(t) and y = -J gamma'(t), the subdifferential condition is exactly the Fenchel equality condition (Section A4):

g_K^2(gamma(t)) + (1/4) h_K^2(-J gamma'(t)) = <gamma(t), -J gamma'(t)>    a.e.

### Step 3: Integrate over time

Integrating the pointwise equality from Step 2 over [0, T]:

integral of g_K^2(gamma) dt  +  I_K(gamma)  =  integral of <gamma, -J gamma'> dt

The right side equals integral of <-J gamma', gamma> dt. By the pointwise algebraic identity <u, -Jv> = <Ju, v> (since J^T = -J), the integrand <gamma, -J gamma'> = <J gamma, gamma'>, so this integral equals 2 A(gamma) by the definition of the action (Section A11).

### Step 4: Use g_K = 1 on boundary(K)

Since gamma lies on boundary(K), g_K^2(gamma(t)) = 1 for all t, so:

T + I_K(gamma) = 2 A(gamma)

In talk normalization, A(gamma) = T, so:

I_K(gamma) = T = A(gamma)

### Step 5: Critical point correspondence

The primal and dual critical points are in one-to-one correspondence via z = gamma - center(gamma). The key:

- The Fenchel equality condition y in subdiff(g_K^2)(x) is equivalent to x in subdiff((1/4) h_K^2)(y) (Section A4).
- Setting x = gamma(t) = z(t) + center(gamma) and y = -J gamma'(t) = -J z'(t), the primal inclusion becomes z(t) + c in subdiff((1/4) h_K^2)(-J z'(t)), matching the dual critical point condition with c = center(gamma).
- Constraints match: closure, centering (by construction), and A(z) = A(gamma) = T (since gamma' = z').

---

## B5. Significance of the Dual Formulation

The dual problem is easier to work with:

1. **No position constraint.** I_K depends only on z', not on z.
2. **No differential inclusion.** The velocity z' is unconstrained beyond integral constraints.
3. **Amenable to rearrangement.** Reordering velocity segments does not change I_K, but does change A(z). This is the key mechanism for the simple orbit theorem (Section C).

---

# C. Simple Orbit Structure (Theorem 1)

## Theorem (HK2017 Thm 1.2)

For every convex polytope K in R^4, there exists a minimum-action generalized closed characteristic gamma* such that:

1. gamma* is piecewise affine,
2. gamma*'(t) is a pure facet Reeb vector (not a convex combination) on each piece,
3. for each facet i, the set {t : gamma*'(t) = c * p_i for some c > 0} is a contiguous interval or empty.

We call such an orbit a **simple orbit**. It visits each facet at most once, in some cyclic order.

## Proof

The proof transforms an arbitrary dual minimizer into a simple orbit through five steps. We work in talk normalization, where A(z) = T and I_K(z) = T at minimizers. We use two functionals:

- **Symplectic action:** A(z) = (1/2) integral of <Jz, z'> dt
- **Dual functional:** I_K(z) = (1/4) integral of h_K^2(-Jz') dt

### Step 1: Approximate

**What happens.** Start with a dual minimizer z. Approximate z in W^{1,2} by a sequence of piecewise affine loops z_N, each with finitely many affine segments.

**Why it works.** We use the following approximation lemma (HK2017, Lemma 4.2):

> **Lemma (Piecewise affine approximation).** Let z be a W^{1,2} loop with z'(t) in conv{v_1, ..., v_N} a.e. Then there exist piecewise affine closed loops z_n -> z in W^{1,2} such that each z_n has finitely many affine segments, each with velocity in {v_1, ..., v_N}.

The velocity of the dual minimizer satisfies z'(t) in conv{p_i} a.e., where p_i = (2/h_i) Jn_i are the facet Reeb vectors, so the lemma applies. The lemma follows from standard convex approximation: on each small time interval, the averaged convex combination of velocities is redistributed into a concatenation of pure-velocity segments, maintaining the total displacement.

**Effect on functionals.** By W^{1,2} continuity: A(z_N) -> A(z) = T and I_K(z_N) -> I_K(z) = T.

### Step 2: Split

**What happens.** Replace each segment where the velocity is a convex combination of Reeb vectors by a concatenation of segments with pure Reeb velocities.

**Splitting mechanism.** Suppose on some time interval, the velocity is v(t) = sum a_i(t) X_i with sum a_i = 1, where X_i are Reeb vectors. Replace this interval by pure-velocity segments: velocity X_i for duration A_i = integral of a_i(t) dt.

**Effect on I_K: unchanged at T.** For a pure Reeb velocity p_i = (2/h_i) Jn_i:

1. Apply -J: -J p_i = (2/h_i)(-J^2)n_i = (2/h_i) n_i.
2. Apply h_K: h_K((2/h_i) n_i) = (2/h_i) h_K(n_i) = (2/h_i) h_i = 2.
3. Square: h_K^2(-Jp_i) = 4.

This holds for every pure Reeb velocity, so h_K^2(-Jz') = 4 at every time, and I_K = (1/4) integral of 4 dt = T. Splitting preserves total duration (the durations of the pure segments sum to the original interval length), so the integral remains T.

**Effect on A: changes by +/- epsilon.** The time ordering of pure segments within a split interval affects the action. We choose the ordering that does not decrease A.

### Step 3: Rearrange (coalescing)

**What happens.** After splitting, the same Reeb vector may appear in multiple disjoint time intervals. We merge all intervals carrying the same Reeb vector into a single contiguous interval.

**Sign-flip argument.** Consider a velocity sequence ABAD with durations T_A1, T_B, T_A2, T_D. We want to merge the two A-blocks (which carry the same velocity w_A). There are two options:

- Move the first A past B: ABAD -> BAAD (swap adjacent A₁ and B)
- Move the second A past B: ABAD -> AABD (swap adjacent B and A₂)

By the piecewise-constant velocity identity (A11), swapping two adjacent segments changes the action by flipping the sign of their pairwise omega contribution. Concretely:

- ABAD -> BAAD: the pair (A₁, B) changes from "A₁ before B" to "B before A₁", so Delta_1 = -T_A1 T_B omega(w_A, w_B).
- ABAD -> AABD: the pair (B, A₂) changes from "B before A₂" to "A₂ before B", so Delta_2 = +T_A2 T_B omega(w_A, w_B).

The two deltas are proportional to omega(w_A, w_B) with opposite signs (since T_A1, T_A2, T_B > 0). Therefore they cannot both be negative: at least one rearrangement does not decrease the action.

**Effect on I_K: unchanged.** I_K depends only on velocity magnitudes and durations, not ordering.

**Effect on A: does not decrease.**

### Step 4: Renormalize

**What happens.** After steps 2-3, we have a simple loop but A may have changed: A(z_N'') = A(z_N) + delta_N. We restore talk normalization by time rescaling.

**Rescaling.** Set beta = T / A(z_N''). Multiply each time interval by beta.

- **A scales quadratically** (each term involves a product T_i T_j): A' = beta^2 A(z_N'') = T^2 / A(z_N'') = T' (new total time). So A' = T'.
- **I_K scales linearly** (single integral over time): I_K' = beta T = T^2 / A(z_N'') = T'.

As N -> infinity: A(z_N) -> T (by Step 1), so the perturbations delta_N from Steps 2-3 satisfy delta_N / A(z_N) -> 0. Therefore A(z_N'') -> T, beta = T / A(z_N'') -> 1, and T' = T^2 / A(z_N'') -> T.

### Step 5: Compactness

**What happens.** Extract a convergent subsequence of simple loops.

A simple loop is determined by finite data: a cyclic ordering sigma of visited facets, and segment durations |I_i| >= 0 with sum |I_i| = T'. The set of orderings is finite (at most F! options). For each ordering, the durations lie in a compact set (bounded closed subset of R^k). By Bolzano-Weierstrass, there is a convergent subsequence with I_K(z*) = T, making z* a simple minimizer.

---

# D. Combinatorial Capacity Formula (Theorem 2)

## Theorem (HK2017 Thm 1.1)

**Definition.** The Q-function is:

Q(sigma, beta) = sum_{j < i} beta_{sigma(i)} beta_{sigma(j)} omega(n_{sigma(j)}, n_{sigma(i)})

where sigma is a permutation encoding the cyclic order of facets visited, beta = (beta_1, ..., beta_F) are non-negative weights, and the sum is over all pairs j < i (j = earlier, i = later in the ordering). The earlier normal n_{sigma(j)} appears as the first argument of omega.

**Definition.** The constraint set is:

M(K) = { beta in R^F : beta_i >= 0, sum beta_i h_i = 1, sum beta_i n_i = 0 }

The three constraints encode:

1. **Non-negativity:** beta_i >= 0.
2. **Height normalization:** sum beta_i h_i = 1 (HK2017 convention).
3. **Closure:** sum beta_i n_i = 0 (the orbit closes as a loop).

**Theorem.** For any convex polytope K in R^4 with 0 in int(K):

c_EHZ(K) = (1/2) [max_{sigma in S_F, beta in M(K)} Q(sigma, beta)]^{-1}

## Derivation

By Theorem 1 (Section C), the minimum-action orbit is a simple orbit: it visits facets in some cyclic order sigma, spending time T_i on facet sigma(i), with pure Reeb velocity p_{sigma(i)} = (2/h_{sigma(i)}) Jn_{sigma(i)}.

We derive the formula in four steps. Steps A-C work in talk normalization (curves on [0, T], with A(z) = T). Step D converts to HK2017 normalization.

### Step A: Action of a simple orbit (talk normalization)

The piecewise-constant velocity identity (Section A11) gives:

2A(z) = sum_{j < i} T_i T_j omega(w_j, w_i)

where w_k is the velocity on the k-th segment and the earlier velocity w_j appears first in omega.

For a simple orbit, T_k is the duration on facet sigma(k) and w_k = p_{sigma(k)} = (2/h_{sigma(k)}) Jn_{sigma(k)}.

We compute omega between two Reeb velocities. By bilinearity:

omega(w_j, w_i) = (4 / (h_{sigma(j)} h_{sigma(i)})) omega(Jn_{sigma(j)}, Jn_{sigma(i)})

By the identity omega(Ju, Jv) = omega(u, v) (Section A7):

= (4 / (h_{sigma(j)} h_{sigma(i)})) omega(n_{sigma(j)}, n_{sigma(i)})

Substituting:

2A(z) = sum_{j < i} T_i T_j (4 / (h_{sigma(i)} h_{sigma(j)})) omega(n_{sigma(j)}, n_{sigma(i)})

### Step B: Variable substitution

Define beta_{sigma(k)} = T_k / h_{sigma(k)}, so that T_k = beta_{sigma(k)} h_{sigma(k)}.

**Closure.** The orbit closes when sum T_k p_{sigma(k)} = 0:

sum T_k (2/h_{sigma(k)}) Jn_{sigma(k)} = 2J sum beta_{sigma(k)} n_{sigma(k)} = 0

Since J is invertible: sum beta_i n_i = 0.

**Normalization.** The total time is:

sum T_k = sum beta_{sigma(k)} h_{sigma(k)} = sum beta_i h_i = T

### Step C: Action equals 2Q

Substitute T_k = beta_{sigma(k)} h_{sigma(k)} into the action formula:

2A(z) = sum_{j < i} (beta_{sigma(i)} h_{sigma(i)})(beta_{sigma(j)} h_{sigma(j)}) (4 / (h_{sigma(i)} h_{sigma(j)})) omega(n_{sigma(j)}, n_{sigma(i)})

The heights cancel:

2A(z) = 4 sum_{j < i} beta_{sigma(i)} beta_{sigma(j)} omega(n_{sigma(j)}, n_{sigma(i)}) = 4 Q(sigma, beta)

Therefore: A(z) = 2 Q(sigma, beta).

In talk normalization, c_EHZ = T = A(z), so: c_EHZ = 2 Q(sigma, beta) with sum beta_i h_i = T.

### Step D: Normalization bridge (talk -> HK2017)

Define beta_tilde_i = beta_i / T, so sum beta_tilde_i h_i = 1 (HK2017 normalization).

Since Q is quadratic in beta: Q(sigma, beta) = T^2 Q(sigma, beta_tilde).

From Step C: c_EHZ = T = 2 Q = 2 T^2 Q(sigma, beta_tilde), so:

1 = 2T Q(sigma, beta_tilde)

c_EHZ = T = 1 / (2 Q(sigma, beta_tilde))

**Optimization.** The capacity is achieved by the (sigma, beta_tilde) that maximizes Q: larger Q means smaller capacity, and the minimum-action orbit has the smallest capacity. Therefore:

c_EHZ(K) = (1/2) [max_{sigma in S_F, beta_tilde in M(K)} Q(sigma, beta_tilde)]^{-1}

---

# E. The Algorithm

We translate the capacity formula (Theorem 2) into a concrete algorithm.

## E1. Input and Output

**Input:** A convex polytope K with F facets, given by outward unit normals n_1, ..., n_F and heights h_1, ..., h_F, with 0 in int(K).

**Output:** c_EHZ(K) = (1/2) [max_{sigma, beta} Q(sigma, beta)]^{-1}.

## E2. Preprocessing: Centering

If the polytope is not centered at the origin, translate it: replace K by K - c for some interior point c. After translation, normals n_i are unchanged — they describe facet orientation, which is translation-invariant — and heights become h_i = h_K(n_i) > 0.

## E3. The Two-Level Enumeration

Computing c_EHZ(K) reduces to maximizing Q(sigma, beta) over all orderings sigma and all beta in M(K):

**Outer loop (subsets):** Enumerate all subsets S of {1, ..., F} with |S| >= 2. Facets not in S receive weight beta_i = 0.

**Inner loop (orderings):** For each S, enumerate all cyclic orderings sigma of S. Each ordering determines a different Q because omega is antisymmetric.

For each (S, sigma) pair, solve for optimal beta and evaluate Q. The global maximum gives the capacity.

## E4. Solving for beta: Two Cases

For a given subset S, the weight vector beta must satisfy:

- Closure: sum beta_i n_i = 0 (four equations)
- Normalization: sum beta_i h_i = 1 (one equation)

This gives a 5-by-|S| matrix A_eq with normals as the first four rows and heights as the fifth row, and right-hand side b_eq = (0, 0, 0, 0, 1)^T.

### Case 1: beta uniquely determined

When |S| = rank(A_eq) (typically |S| = 5), the system has a unique solution. Solve for beta directly. If all beta_i > 0, evaluate Q(sigma, beta) for every cyclic ordering sigma and update the running maximum. The ordering sigma does not affect beta — only the value of Q.

### Case 2: beta underdetermined (KKT optimization)

When |S| > rank(A_eq), different choices of beta yield different Q values. For each ordering sigma, maximize Q(sigma, beta) subject to the equality constraints using the KKT system (Section E6).

## E5. The Symmetrized Action Matrix H

Q(sigma, beta) is a quadratic form in beta. To express it as (1/2) beta^T H beta, we construct the symmetric matrix H.

Define the omega matrix W with entries W_{ij} = omega(n_{sigma(i)}, n_{sigma(j)}). This matrix is antisymmetric (W = -W^T) since omega is antisymmetric.

The symmetrized action matrix H is defined by:

- For i < j: H_{ij} = omega(n_{sigma(i)}, n_{sigma(j)})    [= W_{ij}]
- For i > j: H_{ij} = omega(n_{sigma(j)}, n_{sigma(i)})    [= -W_{ij} = H_{ji}]
- H_{ii} = 0

Equivalently: H = U + U^T where U is the strictly upper triangular part of W.

**Why H is symmetric.** For i < j: H_{ij} = omega(n_{sigma(i)}, n_{sigma(j)}). For j > i (same pair): H_{ji} = omega(n_{sigma(i)}, n_{sigma(j)}) by the i > j rule. So H_{ij} = H_{ji}.

**Why Q = (1/2) beta^T H beta.** We have:

(1/2) beta^T H beta = beta^T U beta = sum_{i < j} beta_{sigma(i)} beta_{sigma(j)} omega(n_{sigma(i)}, n_{sigma(j)})

This sum runs over pairs i < j in the ordering (upper triangle of H). Re-labeling these pairs as (a, b) = (i, j) with a < b, and noting that the Q-function sums over j < i (where j is earlier, i is later), we identify a = j and b = i:

= sum_{j < i} beta_{sigma(j)} beta_{sigma(i)} omega(n_{sigma(j)}, n_{sigma(i)}) = Q(sigma, beta)

The key point: the ordering sigma determines which entries of W appear in the upper triangle of H, and therefore determines H. Different orderings produce different H matrices from the same omega values.

## E6. The KKT System

To maximize Q = (1/2) beta^T H beta subject to A_eq beta = b_eq:

```
[ H     A_eq^T ] [ beta   ]   [ 0     ]
[ A_eq  0      ] [ lambda ] = [ b_eq  ]
```

lambda is the Lagrange multiplier vector (discarded after solving). If the KKT matrix is nonsingular, solve the system and extract beta. If all beta_i > 0, compute Q = (1/2) beta^T H beta and update the running maximum. If the KKT matrix is singular, skip this (S, sigma) pair.

## E7. Why KKT Without Non-negativity is Correct

The non-negativity constraints beta_i >= 0 are not enforced in the KKT system — they are checked post-hoc. This is correct because: the non-negativity-constrained maximum of Q over a convex polytope {A_eq beta = b_eq, beta >= 0} is attained either in the interior (all beta_i > 0) or on a face (some beta_i = 0). In the interior case, the equality-constrained KKT optimum coincides with the constrained one. In the face case, beta_j = 0 means facet j is not visited, which corresponds to the smaller subset S' = S \ {j} — explored in a separate iteration.

Since the outer loop enumerates all subsets exhaustively, every case is covered.

## E8. Cyclic Symmetry

Q(sigma, beta) is invariant under cyclic rotations of sigma. For |S| = k, we enumerate only (k-1)! orderings by fixing sigma(1) to a canonical element (say, the smallest index in S).

## E9. Complexity

Total (subset, ordering) pairs: sum_{k=2}^{F} C(F, k) * (k-1)! = sum_{k=2}^{F} F! / (k (F-k)!) = O(F! / F).

For each pair, solving the KKT system costs O(|S|^3), negligible compared to the combinatorial explosion.

## E10. Pseudocode

```
function compute_ehz_capacity(normals n[], heights h[], num_facets F):
    // Preprocessing: verify 0 in int(K), i.e., all h_i > 0

    best_q = 0

    for each subset S of {1, ..., F} with |S| >= 2:

        // Build equality constraint matrix A_eq (5 rows by |S| columns)
        // Rows 1-4: normals n_i for i in S (closure)
        // Row 5:    heights h_i for i in S (normalization)
        // RHS: b_eq = (0, 0, 0, 0, 1)

        if system A_eq * beta = b_eq is infeasible:
            continue

        if |S| = rank(A_eq):    // beta uniquely determined
            beta = solve(A_eq, b_eq)
            if any beta_i <= 0:
                continue

            // Enumerate cyclic orderings: fix first element, permute rest
            for each cyclic ordering sigma of S:
                q = sum over j < i of:
                    beta[sigma(i)] * beta[sigma(j)]
                    * omega(n[sigma(j)], n[sigma(i)])
                best_q = max(best_q, q)

        else:                   // beta underdetermined
            for each cyclic ordering sigma of S:

                // Build symmetrized action matrix H (|S| by |S|):
                //   H[i][j] = omega(n[sigma(i)], n[sigma(j)])  if i < j
                //   H[i][j] = omega(n[sigma(j)], n[sigma(i)])  if i > j
                //   H[i][i] = 0

                // Assemble KKT system:
                //   [ H      A_eq^T ] [ beta   ]   [ 0     ]
                //   [ A_eq   0      ] [ lambda ] = [ b_eq  ]

                if KKT matrix is singular:
                    continue

                (beta, lambda) = solve(KKT_matrix, KKT_rhs)

                if any beta_i <= 0:
                    continue

                q = (1/2) * beta^T * H * beta
                best_q = max(best_q, q)

    return 1 / (2 * best_q)
```

---

# F. Graph-Pruned Enumeration Variant

The algorithm in Section E enumerates all subsets and orderings. Most do not correspond to geometrically realizable orbits. The graph-pruned variant replaces blind enumeration with a structured search over the facet adjacency graph.

## F1. The Facet Adjacency Graph

Define a directed graph G = (V, E):

- **Vertices:** The facets {1, ..., F}.
- **Directed edges:** i -> j if and only if there exists a point x on facet F_i and a scalar c > 0 such that x + c * p_i lies on facet F_j, where p_i = (2/h_i) J n_i is the Reeb vector of facet i.

The edge i -> j exists precisely when the Reeb flow on facet i can reach facet j.

**Concrete test.** Since the Reeb vector p_i is tangent to facet F_i (<p_i, n_i> = 0), the flow x + c p_i stays on the hyperplane {<x, n_i> = h_i}. The edge i -> j exists if and only if there exists x in R^4 satisfying:

1. x on facet F_i: <x, n_i> = h_i and <x, n_k> <= h_k for all k.
2. Endpoint on facet F_j: <x + c p_i, n_j> = h_j, i.e., c = (h_j - <x, n_j>) / <p_i, n_j>.
3. Endpoint in K: <x + c p_i, n_k> <= h_k for all k.
4. Positive travel: c > 0.

This is a linear feasibility problem in x (since c is determined by x once <p_i, n_j> != 0). A necessary condition is <p_i, n_j> = (2/h_i) omega(n_i, n_j) != 0; if the Reeb vector is parallel to F_j's hyperplane, the flow never reaches it.

## F2. Pruning Strategy

Replace the two-level enumeration by: **enumerate all simple directed cycles in G**.

A simple directed cycle (i_1, i_2, ..., i_k) specifies both a subset S = {i_1, ..., i_k} and an ordering sigma = (i_1, ..., i_k). The rest of the algorithm (solving for beta, evaluating Q, checking non-negativity) is identical to Section E.

## F3. Correctness

By Theorem 1, the minimum-action orbit visits facets in a definite cyclic order with transitions from sigma(i) to sigma(i+1) requiring the Reeb flow on sigma(i) to reach sigma(i+1) — which is exactly the edge condition in G. Therefore every realizable simple orbit corresponds to a directed cycle in G.

## F4. Computational Significance

The number of simple directed cycles in G can be vastly smaller than the total number of subset-ordering pairs. For polytopes with sparse facet adjacency, where most facets are not reachable from each other in one Reeb step, the pruning yields a substantial speedup. The worst case remains exponential.
