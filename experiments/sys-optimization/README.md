# Systematic polytope improvement for sys > 1

**Status:** Ideation

**Motivation:** Random polytope sampling (random-sweep, pentagon-perturb) has not found sys > 1. Instead of hoping to get lucky, we try to *construct* a polytope P' with higher sys than a starting polytope P, by making targeted changes to P's half-space description.

**Prior art in this repo:** `pentagon-perturb/` applied random perturbations to the HK-O counterexample. PCA showed some perturbation directions correlate with sys, but the approach was undirected — no attempt to follow the gradient.

## Core idea

sys(P) = c_EHZ(P)^2 / (2 vol(P)).

To increase sys, we want to decrease vol without decreasing capacity, or increase capacity without increasing vol, or both. The half-space description P = {x : n_k . x <= h_k} gives us knobs: the heights h_k and (more expensively) the normals n_k.

## Phase 1: Infinitesimal sensitivity analysis

### What we compute

For a polytope P with F facets, compute the sensitivity of sys to each h_k:

- **d(vol)/d(h_k):** The volume derivative w.r.t. h_k equals the 3-volume of facet k (standard result for convex bodies in H-representation). Computable from qhull or from our vertex data.

- **d(c_EHZ)/d(h_k):** Since c_EHZ = min_{S,sigma} A(S,sigma), this derivative depends on the structure of the minimum:
  - **Non-degenerate case** (unique minimizer, well-separated from runners-up): The capacity is locally smooth in h_k. The derivative comes from the winning orbit only. Specifically, A(S,sigma) = 1/(2 Q(beta)), and Q and beta depend on h_k through the KKT system. Only orbits (S,sigma) that include facet k in S are affected by h_k at all.
  - **Degenerate case** (multiple near-optimal orbits): The min has a kink. The "derivative" is directional: decreasing h_k may switch the winning orbit. Need to track the top few orbits, not just the best.

- **d(sys)/d(h_k):** Chain rule from the above. The sign tells us whether increasing h_k improves sys.

### Phase 1a: Sparse changes

For each facet k independently, determine the sign and magnitude of d(sys)/d(h_k). This gives a gradient vector in R^F. If any component is positive (increasing h_k improves sys) or negative (decreasing h_k improves sys), we have a direction of improvement.

Questions to answer:
- For typical random polytopes, how many facets have a favorable gradient direction?
- Is the gradient ever zero everywhere (local optimum of sys)?
- How does the gradient structure depend on facet count F?

### Phase 1b: Dense changes

Use the full gradient (d(sys)/d(h_1), ..., d(sys)/d(h_F)) to take a step in the steepest-ascent direction. This is classical gradient ascent on sys(h_1, ..., h_F).

Complication: the feasibility region (P must be bounded, full-dimensional, origin-interior) constrains the step. We need to stay in the feasible region.

### Phase 1c: Adding facets (lower priority, higher cost)

Instead of changing existing h_k, introduce a new half-space that clips a vertex or edge. This changes the combinatorial type of P (more facets, different adjacency). The cost is higher because:
- F increases, so HK2017 gets exponentially more expensive
- The space of possible new facets is continuous (normal + height)
- Analyzing the effect requires a full HK2017 recomputation

Defer this until Phases 1a/1b show whether h_k changes alone can push sys past 1.

## Phase 2: Finite step sizes

Phase 1 gives directions. Phase 2 answers: how far can we go?

**Degeneracy bound for h_k:** Changing h_k moves facet k. The polytope degenerates when:
- A vertex hits a new facet (vertex becomes incident to more facets — combinatorial type change)
- A vertex disappears (facet k sweeps past it — the vertex is no longer extremal)
- The polytope becomes unbounded or loses full-dimensionality

These are computable: for each vertex v and facet j, the critical h_k value where v lands on facet j is determined by the linear system. This gives us a maximum step size Delta_h_k in each direction before the combinatorial type changes.

**What to compute:** For the gradient direction from Phase 1, find the largest step that stays within the same combinatorial type. Evaluate sys at the new polytope. This gives a concrete sys improvement Delta_sys from P to P'.

## Phase 3: Greedy / random optimization

Given Phases 1-2, we have a local improvement oracle: given P, compute gradient, take a step, get P'. Iterate.

Questions to answer:
- Does greedy gradient ascent converge? To what?
- Does the combinatorial type change at some point? If so, re-instrument HK2017 and continue.
- Random restarts: start from many random polytopes, run greedy ascent on each, compare final sys values.
- Alternative: random step directions (not just gradient), random step sizes within the feasibility bound.
- What's the best algorithm? Pure gradient ascent, simulated annealing, CMA-ES, ...?

## Phase 4: Does it work?

- Do we find sys > 1?
- If not, what's the highest sys achieved? Compare to the random-sweep distribution.
- What do the optimized polytopes look like? (facet count, combinatorial type, resemblance to HK-O pentagon)
- Distribution of optimized sys across many starting polytopes vs. the starting distribution.

## Prerequisites / instrumented HK2017

The current `EhzResult` stores only the single best (S, sigma, beta). Phases 1-2 need richer data:

1. **All valid orbits table:** For every (S, sigma) with beta > 0, store (action, S, sigma, beta). Needed to compute d(c_EHZ)/d(h_k) correctly near degeneracies and to identify which orbits are runners-up.

2. **Per-facet index:** For each facet k, which (S, sigma) include k? What is the best action among those? This is derivable from (1) but worth precomputing for the gradient calculation.

3. **Top-N orbits:** At minimum, the best orbit plus all orbits within some tolerance of optimal (e.g., action < 1.01 * best_action). These are the orbits that matter for sensitivity.

This instrumented version lives in the experiment binary (per repo convention: library stays stable, experiment binaries carry variants). It can copy and modify `ehz_capacity` to collect the extra data.

## Estimated cost

- **Phase 1a:** One HK2017 run per polytope (instrumented), plus volume derivatives. For F=8, HK2017 takes ~seconds. Gradient computation is post-processing. Cheap.
- **Phase 1b:** Same cost as 1a per step, but iterated. ~10-100 steps per polytope.
- **Phase 2:** Linear algebra per step (vertex-facet incidence). Cheap.
- **Phase 3:** Many iterations of Phase 1+2. Parallelizable across starting polytopes.
- **Phase 1c:** Expensive (F increases). Defer.

## Starting polytopes

Both random families from this repo:
- **random-sweep:** Random polytopes with F=5..12 facets. Broad coverage of combinatorial types.
- **random-product-sweep:** Random Lagrangian products. These have billiard algorithm as a fast cross-check.

## Open questions

- Is sys(h_1, ..., h_F) locally concave? (Would make gradient ascent well-behaved.)
- Can we compute d(c_EHZ)/d(h_k) analytically from the KKT system, or do we need finite differences?
- What facet count range is tractable? F=8 is fine, F=10 (like HK-O) might be the sweet spot, F=12+ gets expensive for repeated HK2017.
- **Normal sensitivity (d(sys)/d(n_k)):** Normals live on S^3, not R, so the derivative is on the tangent space of the sphere. Harder than height sensitivity, but could matter — a rotation of a facet normal changes the polytope shape in ways that height changes alone cannot reach. Worth investigating after h_k-only optimization shows its limits.
