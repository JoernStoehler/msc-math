# Handoff memo: experiments for finding a polytope with `sys(K) > 1`

Prepared for the lead development / codex agent. This memo is a knowledge transfer artifact summarizing the current understanding from the session, including corrections to earlier ideas, the experiments that now look most valuable, and the math breadcrumbs needed to implement them.

Date: 2026-04-13

---

## 0. Executive summary

The search problem should be treated as a **finite minimax / exchange-oracle problem**, not as a generic black-box optimization problem.

For fixed facet count `F`, write the polytope in **labeled dual-vertex coordinates**
\[
K^o = \operatorname{conv}(a_1,\dots,a_F), \qquad a_i\in\mathbb R^4,
\]
and denote
\[
sys(K)=\min_{c\in S_F} s_c(K),
\]
where `c` is a permutation/witness and `s_c(K)=sys_c(K)` is the branch value for that witness.

Crucial asymmetry:

- evaluating a **fixed** branch \(s_c(K)\), its gradient, and Hessian is cheap (essentially constant-time once \(c\) is known);
- computing the true \(sys(K)\) is expensive because it requires the **global search over witnesses** \(c\in S_F\), which is the factorial bottleneck.

That pushes the search toward:
1. **local witness reuse along tracked trajectories / lineages**,
2. **cheap partial witness search** before exact witness search,
3. **reduced-model local optimization** on a small active / threatening witness set,
4. **exact separator calls only when the cheap upper model says a point is promising**,
5. **witness-guided continuation and vertex splitting**, not blind random continuation.

The fastest practical wins are likely:
- local witness caches with rigorous upper-bound pruning,
- permutation-neighborhood search around incumbent witnesses,
- warm-starting the exact witness search,
- reduced local optimization via a smoothed soft-min model,
- witness-guided vertex splitting \(F\to F+1\).

---

## 1. Problem model and what is already known

### 1.1 Representation

Work in the labeled dual-vertex model
\[
(a_1,\dots,a_F)\in (\mathbb R^4)^F \cong \mathbb R^{4F},
\]
with the understanding that this is an overparameterization of the polytope space if different labelings have the same convex hull. For continuation / perturbation / vertex splitting experiments, this labeled model is the right one.

### 1.2 Oracle structure

For fixed \(F\),
\[
sys(K)=\min_{c\in S_F} s_c(K),
\]
where:
- \(S_F\) is the set of permutations of \(\{1,\dots,F\}\);
- for fixed \(c\), the function \(s_c(K)\) is cheap to evaluate, along with \(\nabla s_c(K)\) and \(\nabla^2 s_c(K)\);
- the expensive part is the exact search over \(c\in S_F\) to find the minimizing witness.

This is a classic setting for exchange / bundle / gradient-sampling style ideas: the expensive object is the **oracle that discovers threatening branches**, while the branches themselves are cheap once known. Gradient sampling is designed for locally Lipschitz objectives that are differentiable almost everywhere, and approximates generalized subdifferentials by convex hulls of nearby gradients. Bundle methods exploit a similar “model + oracle” structure and use serious/null steps to alternate between moving and enriching the local model. [R1, R2, R3, R4, R5]

### 1.3 Empirical facts from the current session

These are the important empirical constraints already supplied by Jörn:

- Random sampling for low \(F\le 13\) did not find \(sys>1\); observed values reached about \(0.95\).
- Smooth ascent in \(M_F\) from random seeds improves the tail but still tops out near \(0.97\).
- Randomly adding an \((F+1)\)-st dual vertex to a local maximum and then optimizing does not help much; values stay around \(0.971\) or so.
- Exact witnesses are few near interesting points. Empirically, only a small number of witnesses are active or nearly active; generically only one witness is active.
- A raw witness \(c\in S_F\) does **not** transfer geometrically between unrelated polytopes, because the witness is just a permutation of vertex labels. Without a label correspondence, a stored witness is not a meaningful “same feature” across different seeds.

### 1.4 Corrections to earlier thinking

Two points were corrected during the discussion:

1. **Generic simplicity is not a volume argument.**  
   Under an absolutely continuous law on labeled dual vertices in \((\mathbb R^4)^F\), the dual hull is generically simplicial and the primal polytope is generically simple. But this says only that generic iid samples land in simple strata; it does **not** imply that a hypothetical \(sys>1\) region has non-negligible probability mass under the chosen sampling measure.

2. **Continuity alone does not rescue iid random search.**  
   If there exists \(K_*\) with \(sys(K_*)>1\), then indeed some neighborhood around \(K_*\) inside the relevant stratum also satisfies \(sys>1\). But the mass of that neighborhood under a naive raw-coordinate sampling distribution may still be tiny. So “continuity implies positive volume” is true but not operationally useful for iid search.

Conclusion: plain iid + smooth local ascent likely already explored the easy generic-interior regime. The most promising remaining regime is **nonsmooth maximin structure with small active witness sets**, especially along continuation paths.

---

## 2. The key abstraction: exact separator vs reduced witness model

Let \(A\subset S_F\) be any subset of witnesses. Define the reduced model
\[
U_A(K):=\min_{c\in A} s_c(K).
\]

Since \(A\) is only a subset of all witnesses,
\[
sys(K) \le U_A(K).
\]

This is the single most useful inequality in the problem.

Interpretation:
- \(U_A\) is a **rigorous upper bound** on \(sys\);
- if \(U_A(K)<1\), then automatically \(sys(K)<1\), so \(K\) can be **rejected without exact witness search**;
- if \(U_A(K)\) is large, that only means “promising”, not “certified”.

This upper-bound logic is safe for:
- pruning,
- triage,
- optimistic reduced-model local search,
provided that exact verification is used before claiming a new best exact value.

The exact witness search is then best viewed as a **separator / exchange oracle**:
\[
\text{given } K,\ \text{find } c_{\min}(K)\in \arg\min_{c\in S_F} s_c(K).
\]

This is structurally close to bundle methods, proximal bundle methods for composite/max-type structure, and gradient sampling methods for nonsmooth finite-max/min objectives. [R1, R2, R4, R5, R6]

---

## 3. What transfers between polytopes, and what does not

### 3.1 Global witness caches across unrelated seeds are weak

A raw witness \(c\in S_F\) contains no geometric content beyond a label ordering. For an unrelated polytope with unrelated labeling, the same permutation \(c\) is not the “same witness” in any meaningful sense.

So a **global raw-permutation cache** across unrelated seeds is mathematically valid as a source of branches to evaluate, but probably weak in practice.

### 3.2 Local witness transfer along a tracked lineage is strong

Witness reuse is meaningful whenever labels are tracked:

- one gradient/Newton/trust-region step,
- one random perturbation of a labeled seed,
- one continuation step,
- vertex splitting / adding a vertex with inherited labels,
- search inside a parameterized family with fixed labeling.

In those cases, a local witness set \(A(K_0)\) at a parent / nearby point \(K_0\) gives a valid upper model
\[
U_{A(K_0)}(K)=\min_{c\in A(K_0)} s_c(K)
\]
for descendants or nearby points \(K\).

This upper model remains rigorously safe for pruning:
\[
U_{A(K_0)}(K)<1 \implies sys(K)<1.
\]

### 3.3 Store threatening near-active witnesses, not just minimizers

At an exact point \(K_0\), let \(c_*\) be a minimizing witness and define
\[
h_c(K):=s_c(K)-s_{c_*}(K)\ge 0 \quad \text{at } K=K_0.
\]

A simple second-order threat score for radius \(r\) is
\[
T_c(r)
=
h_c(K_0)
-
r \,\|\nabla h_c(K_0)\|
-
\frac12 r^2 \,\|\nabla^2 h_c(K_0)\|_{\mathrm{op}}.
\]

Heuristic meaning:
- if \(T_c(r)\) is large positive, witness \(c\) is unlikely to overtake the minimizer within radius \(r\);
- if \(T_c(r)\) is small or negative, witness \(c\) is threatening and should stay in the local cache.

If this is too much engineering for a first pass, the fallback is simpler:
- keep the exact minimizer,
- keep all witnesses within additive gap \(\Delta\),
- or keep the top \(m\) near-minimizers returned by the exact search.

---

## 4. Prioritized experiments

The list below is prioritized. The first four experiments are the ones to implement first.

---

### Experiment A: instrumentation / API changes

**Goal.** Turn the exact oracle from a single-value endpoint into a useful local-structure provider.

**Required changes.**

1. Extend the exact witness search so that, in addition to the minimizing witness \(c_*\), it can optionally return:
   - the top \(m\) witnesses by branch value,
   - or all witnesses within additive gap \(\Delta\) of the minimum,
   - or at least the best \(m\) incumbents encountered during exact search.

2. Make exact witness search accept one or more **incumbent warm starts** if the internal algorithm can exploit them.

3. Persist, for every exact-evaluated point \(K\):
   - \(sys(K)\),
   - minimizing witness \(c_*\),
   - near-active witness set \(A(K)\),
   - for each \(c\in A(K)\): \(s_c(K)\), \(\nabla s_c(K)\), \(\nabla^2 s_c(K)\),
   - exact search runtime diagnostics.

**Why this matters.** Every later experiment benefits from this. Without it, the cheap branch evaluator is underused.

**Acceptance criteria.**
- The exact oracle can be queried in “top-\(m\)” or “within-gap-\(\Delta\)” mode.
- The exact oracle can optionally use an incumbent witness or a small incumbent set.

---

### Experiment B: witness reuse radius / trust-region calibration

**Goal.** Quantify how long a local witness set remains useful under nearby perturbations.

**Setup.**

Take an exact-evaluated point \(K_0\). Construct several witness sets:
- \(A_{\min} = \{c_*\}\),
- \(A_{\Delta} = \{c: s_c(K_0)-sys(K_0)\le \Delta\}\),
- \(A_{\mathrm{top}m}\),
- \(A_{\mathrm{threat}(r)} = \{c: T_c(r)\le \tau\}\).

For step sizes \(\rho\in\{\rho_1,\rho_2,\dots\}\) and many directions \(u\),
evaluate nearby points
\[
K(\rho,u)=K_0+\rho u
\]
(with whatever projection/normalization is needed to stay in the admissible set).

For each witness set \(A\), measure:
- whether the new exact minimizer belongs to \(A\),
- the upper-bound gap
  \[
  U_A(K(\rho,u)) - sys(K(\rho,u)),
  \]
- how often \(U_A(K(\rho,u))<1\) already certifies rejection.

**Outputs.**
- hit rate of the true minimizer vs. step size,
- average and tail gaps of the upper bound,
- practical trust radius for witness reuse,
- calibration of good values of \(m\), \(\Delta\), and threat threshold \(\tau\).

**Why this matters.** It tells the rest of the pipeline how often local witness caches remain valid and how aggressive the cheap phase can be before exact verification.

---

### Experiment C: cheap partial witness search as a universal prefilter

**Goal.** Reject many points without paying for exact witness search.

Given a candidate \(K\), before exact separation evaluate a modest batch of cheaply chosen witnesses \(B(K)\), then use
\[
U_{B(K)}(K) = \min_{c\in B(K)} s_c(K).
\]
If \(U_{B(K)}(K) < 1\), reject \(K\) immediately.

**Candidate witness sources to compare.**
1. `random`: \(m\) random permutations;
2. `parent-cache`: witness set from the parent / nearby exact point;
3. `perm-local`: local search in permutation space from current incumbent (see Experiment E);
4. `hybrid`: union of the above.

**Benchmark protocol.**
On a test bank of points with known exact \(sys(K)\), compare:
- upper-bound quality \(U_B(K)-sys(K)\),
- rejection rate from the safe rule \(U_B(K)<1\),
- cost per candidate,
- exact-call reduction factor when used as a prefilter.

**Why this matters.** Even if witness transfer between unrelated seeds is weak, **partial witness search itself** may still be an enormous win because fixed-branch evaluation is cheap.

**Important note.**
Pruning on \(U_B(K)<1\) has **zero false negatives by construction**:
\[
sys(K) \le U_B(K) < 1.
\]

---

### Experiment D: reduced local optimization on a small witness set

**Goal.** Spend many cheap steps optimizing a reduced witness model, and call the exact separator only when necessary.

There are two versions worth implementing.

#### D1. Soft-min / log-sum-exp smoothing (recommended first)

For a local witness set \(A\), define
\[
\phi_{A,\tau}(K)
=
-\tau \log \sum_{c\in A} e^{-s_c(K)/\tau}.
\]

Then
\[
\phi_{A,\tau}(K)
\le
U_A(K)
\le
\phi_{A,\tau}(K)+\tau \log |A|.
\]

The derivatives are explicit. Define
\[
w_c(K)=\frac{e^{-s_c(K)/\tau}}{\sum_{d\in A}e^{-s_d(K)/\tau}}.
\]
Then
\[
\nabla \phi_{A,\tau}(K)=\sum_{c\in A} w_c(K)\,\nabla s_c(K),
\]
and
\[
\nabla^2 \phi_{A,\tau}(K)
=
\sum_{c\in A} w_c(K)\,\nabla^2 s_c(K)
-
\frac1\tau
\sum_{c\in A} w_c(K)\,
(\nabla s_c(K)-\bar g)(\nabla s_c(K)-\bar g)^\top,
\]
where
\[
\bar g = \sum_{c\in A} w_c(K)\,\nabla s_c(K).
\]

So \(\phi_{A,\tau}\) can be optimized by trust-region Newton / damped Newton using only cheap fixed-witness evaluations. This is directly in the spirit of smoothing max/min structure to exploit first- and second-order methods. [R7]

**Outer loop: serious step / null step logic.**
1. Optimize \(\phi_{A,\tau}\) cheaply for a few local steps.
2. Evaluate the exact separator at the trial point.
3. If the exact minimizer is already in \(A\), or if the new witness does not violate much, accept a **serious step**.
4. If a new witness \(c_{\text{new}}\) appears with
   \[
   s_{c_{\text{new}}}(K_{\text{trial}})
   <
   U_A(K_{\text{trial}})-\eta,
   \]
   treat this as a **null step**:
   - add \(c_{\text{new}}\) to \(A\),
   - shrink trust region / smoothing temperature as appropriate,
   - retry.

This is bundle-style logic: move when the local model is predictive, enrich the local model when it fails. Serious/null step mechanics are standard in bundle methods. [R4, R8]

**Metrics.**
- exact separator calls per accepted improving move,
- best exact \(sys\) reached,
- wall-clock vs. plain exact-evaluate-every-step baseline.

#### D2. Min-norm convex-hull / stationarity QP (recommended second)

At a point \(K\) with local witness set \(A\), form the gradient matrix
\[
G = [\,g_c\,]_{c\in A},\qquad g_c=\nabla s_c(K).
\]

Solve the tiny QP
\[
\min_{\lambda\ge 0,\ \mathbf 1^\top\lambda=1}
\left\| G\lambda \right\|^2.
\]

This computes the minimum-norm point of the convex hull of the branch gradients. This is exactly the subdifferential-approximation logic behind gradient sampling and active-gradient methods for finite max/min objectives. [R1, R2]

Use cases:
- **stationarity diagnostic**: if \(\|G\lambda^*\|\) is tiny, the current reduced model looks Clarke-stationary;
- **support detection**: the support of \(\lambda^*\) identifies the few witnesses that matter most;
- **initialization for KKT/Newton refinement**.

For an advanced follow-up, solve the tied-active KKT system
\[
s_c(K)-t=0 \quad (c\in S),
\qquad
\sum_{c\in S}\lambda_c \nabla s_c(K)=0,
\qquad
\lambda\in\Delta(S),
\]
for the support \(S=\{c:\lambda_c>0\}\), using the cheap Hessians \(\nabla^2 s_c(K)\). Carathéodory implies only a small support is ever needed in principle.

---

### Experiment E: permutation-neighborhood search around incumbent witnesses

**Goal.** Use the combinatorial nature of the witness space to improve cheap upper bounds and warm-start the exact separator.

For a current incumbent witness \(c\), explore a local neighborhood in permutation space using only cheap branch evaluations.

**Move sets to compare.**
- adjacent swaps,
- insertions,
- small block moves,
- 2-opt / reversal-style moves,
- beam search of width \(b\) and depth \(L\).

**Where to use it.**
- before every exact witness search,
- after each accepted reduced-model step,
- after continuation / vertex splitting,
- inside Experiment C as a cheap witness generator.

**What to measure.**
- best cheap witness value found before exact search,
- whether it equals the exact minimizer,
- runtime reduction in exact witness search when used as an incumbent,
- improvement in safe upper-bound pruning.

**Why this matters.** Even if active witnesses can jump, along a short tracked trajectory the minimizing permutation may often change by only a few local edits. This is a heuristic, not a theorem, but it is a very high-upside one because branch evaluations are cheap.

---

### Experiment F: exact-search warm-start benchmark

**Goal.** Measure whether the exact witness search becomes much cheaper when given good incumbents.

Compare exact witness search runtime under:
1. no warm start,
2. previous exact minimizer,
3. best witness from permutation-neighborhood search,
4. small incumbent set from parent cache,
5. hybrid incumbent set.

Measure:
- wall-clock,
- internal node count / branch-and-bound effort / whatever internal metric exists,
- quality of incumbent at the start,
- frequency of exact minimizer already present in the incumbent set.

This experiment may matter almost as much as the reduced-model pruning itself.

---

### Experiment G: witness-guided vertex splitting \(F\to F+1\)

**Goal.** Replace weak random continuation by a structured continuation that explicitly targets witness disagreement.

Randomly adding a new dual vertex to an elite local maximum has already been tried and appears weak. The next version should be **witness-guided vertex splitting**.

Let \(A\) be the local threatening witness set at an elite exact point \(K\). For each dual vertex \(a_i\), define the per-vertex branch-gradient blocks
\[
g_{c,i}:=\nabla_{a_i} s_c(K)\in\mathbb R^4,\qquad c\in A.
\]

Compute the centered matrix
\[
G_i = \big[\, g_{c,i} - \bar g_i \,\big]_{c\in A},
\qquad
\bar g_i = \frac1{|A|}\sum_{c\in A} g_{c,i}.
\]

Score vertex \(i\) by one of:
- \(\|G_i\|_F\),
- top singular value \(\sigma_1(G_i)\),
- soft-min-weighted variant.

Pick the highest-scoring vertex and split it along the principal disagreement direction:
\[
a_i \mapsto a_i^{\pm} = a_i \pm \varepsilon u_i,
\]
where \(u_i\in\mathbb R^4\) is the top singular vector of \(G_i\).

**Witness lifting to \(F+1\).**
Do not restart witness search from scratch. Lift the parent witnesses:
- if the old label \(i\) is split into \(i',i''\), create child permutations by replacing \(i\) with \(i',i''\) in both local orders,
- optionally run a small local permutation-neighborhood search from these lifted children.

Then run reduced local optimization on the child point before exact verification.

**Variants.**
- direction from softest eigenvector of a reduced Hessian
  \[
  H_{\mathrm{red}} = \sum_{c\in A} w_c \nabla^2 s_c(K),
  \]
  using soft-min weights \(w_c\);
- \(\varepsilon\)-schedule over several split magnitudes;
- one-sided split vs. symmetric split.

**Metrics.**
- best exact \(sys\) found after continuation,
- exact separator calls per successful child,
- comparison to random-addition baseline.

This is likely the best next continuation experiment.

---

### Experiment H: symmetry/orbit-union search

**Goal.** Search low-dimensional structured families that generic iid sampling may never hit efficiently.

Parameterize the dual vertices as unions of group orbits of a small number of seed vectors under subgroups of the signed coordinate-permutation group in \(\mathbb R^4\).

Examples:
- one orbit of one seed vector,
- union of two or three orbits,
- optional small symmetry-breaking perturbation \(\eta\).

For each family:
- sample or optimize in the reduced parameter space,
- use Experiments C/D/E inside the family,
- exact-verify only promising points.

**Why this matters.**
Continuity does not imply that a \(sys>1\) neighborhood has significant mass under naive raw-coordinate sampling. Structured low-dimensional families can be both mathematically special and practically invisible to generic iid proposals.

**Secondary use.**
If this finds nothing, it still gives a principled negative result on an interpretable set of families.

---

### Experiment I: combinatorial type and geometry diagnostics

**Goal.** Log enough structural information to correlate high values with geometry/combinatorics and to support later family design.

For every exact-evaluated elite point, record:

1. **Hull combinatorial type / face structure**
   - canonicalized vertex-facet incidence,
   - or full face lattice if manageable.

   The face lattice encodes the full combinatorial structure of a polytope; vertex-facet incidence is often a more efficient practical representation. [R9, R10]

2. **Order-type / oriented-matroid signature of the dual points**
   - for lifted points \(\tilde a_i=(a_i,1)\in\mathbb R^5\), record
     \[
     \chi(i_1,\dots,i_5)=\operatorname{sign}\det(\tilde a_{i_1},\dots,\tilde a_{i_5})
     \]
     for all 5-subsets, plus near-zero magnitudes.

   Order type / chirotope captures the combinatorial type of the point configuration. [R10]

3. **Basic invariants**
   - \(f\)-vector,
   - facet-size multiset,
   - vertex-degree multiset,
   - count of near-zero \(5\times5\) lifted determinants,
   - approximate symmetry score under signed coordinate permutations.

4. **Witness diagnostics**
   - number of active witnesses,
   - number within gap \(\Delta\),
   - soft-min weight entropy,
   - exact separator runtime and incumbent quality.

Use this to answer questions like:
- do elite points correlate with near-degeneracy?
- do they cluster in a small number of combinatorial types?
- do they exhibit approximate symmetry?
- does exact witness search become harder or easier near elites?

---

### Experiment J: optional branch-and-bound on low-dimensional families

**Goal.** Use witness upper bounds to prune whole regions of a low-dimensional parameter family.

Suppose \(K(\theta)\) is a family parameterized by \(\theta\in\Theta\subset\mathbb R^p\), and \(A\) is a local witness set. Since
\[
sys(K(\theta)) \le U_A(K(\theta)) = \min_{c\in A} s_c(K(\theta)),
\]
for any box \(B\subset\Theta\),
\[
\sup_{\theta\in B} sys(K(\theta))
\le
\sup_{\theta\in B} U_A(K(\theta))
\le
\min_{c\in A} \sup_{\theta\in B} s_c(K(\theta)).
\]

So if one can cheaply upper-bound \(\sup_B s_c\) using Taylor/Hessian bounds, then taking the minimum over \(c\in A\) gives a rigorous box-level upper bound for \(sys\).

This is likely too much for the first implementation pass, but it becomes attractive once a low-dimensional symmetry family is in place.

---

## 5. Recommended implementation order

Implement in this order.

### Phase 1: immediate practical leverage
1. **Experiment A**: instrumentation / top-\(m\) / within-gap return / warm-start support.
2. **Experiment B**: witness reuse radius calibration.
3. **Experiment C**: cheap partial witness search as safe prefilter.
4. **Experiment E**: permutation-neighborhood search.
5. **Experiment F**: warm-start benchmark.

Reason: these are likely to reduce exact witness-search cost fastest.

### Phase 2: local optimization over reduced witness models
6. **Experiment D1**: soft-min reduced local optimization.
7. **Experiment D2**: min-norm convex-hull QP and KKT refinement.

Reason: this exploits the cheap derivative information most aggressively.

### Phase 3: structured exploration
8. **Experiment G**: witness-guided vertex splitting.
9. **Experiment H**: symmetry/orbit-union families.
10. **Experiment I**: combinatorial diagnostics as standard logging.
11. **Experiment J**: branch-and-bound on low-dimensional families if warranted.

---

## 6. What not to prioritize right now

1. **More plain iid random sampling with full exact witness search.**  
   Existing experiments already suggest this is not the regime where the missing examples live.

2. **Global witness caches over unrelated seeds without canonical alignment.**  
   Raw permutations do not transfer much geometric information between unrelated labelings.

3. **Only caching the exact minimizer.**  
   Near-active threatening witnesses are likely much more important for short continuation steps.

4. **Treating reduced-model improvements as real improvements without exact verification.**  
   Reduced models are for pruning / triage / optimistic local search; exact claims must still go through exact separation.

---

## 7. Minimal algorithmic skeleton to build around

The following loop should probably become the standard search primitive.

### Local lineage search loop

Input:
- exact parent point \(K_0\),
- local witness cache \(A_0\),
- trust radius / temperature / step budget.

Loop:
1. Construct a candidate witness set \(A\) from:
   - parent cache,
   - threatening near-actives,
   - permutation-neighborhood search around the current incumbent.
2. Compute the safe upper model
   \[
   U_A(K)=\min_{c\in A} s_c(K).
   \]
3. If \(U_A(K)<1\), prune.
4. Otherwise take several cheap local steps optimizing \(\phi_{A,\tau}\) or another reduced model.
5. When the reduced model becomes interesting, run the exact separator.
6. If a new witness violates the reduced model significantly, enlarge \(A\) (null step) and continue.
7. Otherwise accept the point as the new exact anchor (serious step), refresh the cache, and continue.

This loop should be usable for:
- local ascent in fixed \(F\),
- random perturbation descendants,
- vertex splitting,
- symmetry-family optimization.

---

## 8. References

Use the following as the canonical citations for the next agent.

### Nonsmooth / gradient-sampling / bundle / software

[R1] J. V. Burke, A. S. Lewis, and M. L. Overton,  
**A Robust Gradient Sampling Algorithm for Nonsmooth, Nonconvex Optimization**, SIAM J. Optim. 15(3), 2005.  
PDF: https://cs.nyu.edu/overton/papers/pdffiles/gradsamp.pdf

[R2] J. V. Burke, F. E. Curtis, A. S. Lewis, M. L. Overton, and L. E. A. Simões,  
**Gradient Sampling Methods for Nonsmooth Optimization**, 2018 survey.  
PDF: https://optimization-online.org/wp-content/uploads/2018/04/6597.pdf

[R3] **HANSO** (Hybrid Algorithm for Non-Smooth Optimization), Michael Overton’s page.  
https://cs.nyu.edu/~overton/software/hanso/

[R4] Claudia Sagastizábal,  
**Composite Proximal Bundle Method**, 2009.  
PDF: https://optimization-online.org/wp-content/uploads/2009/07/2356.pdf

[R5] Frank E. Curtis and Lara Zebiane,  
**NonOpt: Nonconvex, Nonsmooth Optimizer**, 2025 technical report / software overview.  
PDF: https://engineering.lehigh.edu/sites/engineering.lehigh.edu/files/_DEPARTMENTS/ise/pdf/tech-papers/25/25T_005.pdf  
Code: https://github.com/frankecurtis/NonOpt

[R6] Frank E. Curtis, Tim Mitchell, and Michael L. Overton,  
**A BFGS-SQP Method for Nonsmooth, Nonconvex, Constrained Optimization and its Evaluation using Relative Minimization Profiles**, 2017.  
PDF: https://pure.mpg.de/rest/items/item_2325867_5/component/file_2487144/content

[R7] Yu. Nesterov,  
**Smooth minimization of non-smooth functions**, 2003.  
PDF: https://webdoc.sub.gwdg.de/ebook/serien/e/CORE/dp2003-12.pdf

[R8] R. Fletcher, S. Leyffer, and Ph. L. Toint,  
**A bundle filter method for nonsmooth nonlinear optimization**, 1999.  
PDF: https://wiki.mcs.anl.gov/leyffer/images/9/9c/NSFilter.pdf

### Combinatorial type / order type / face-lattice logging

[R9] `polymake` tutorial: **Face lattices (of Polytopes)**.  
https://polymake.org/doku.php/user_guide/tutorials/face_lattice_tutorial

[R10] Lars Finschi and Komei Fukuda,  
**Combinatorial Generation of Small Point Configurations and Hyperplane Arrangements**, 2002.  
PDF: https://www.cs.mcgill.ca/~fukuda/download/paper/cgspc020924.pdf

[R11] M. Henk, J. Richter-Gebert, and G. M. Ziegler,  
**Basic Properties of Convex Polytopes**, section discussing the face lattice as a complete encoding of combinatorial structure.  
PDF: https://page.math.tu-berlin.de/~henk/preprints/henk%20richter-gebert%20ziegler%26basic%20properties%20of%20convex%20polytopes.pdf

---

## 9. Concrete first-week checklist

If the next agent wants a shortest path to actionable results, do this first:

1. Modify the exact witness search to return top-\(m\) / within-gap-\(\Delta\) witnesses and accept incumbents.
2. Build a benchmark bank of already exact-evaluated points.
3. On that bank, compare:
   - parent witness cache,
   - permutation-neighborhood search,
   - random partial witness search,
   - hybrids,
   using upper-bound gap and safe rejection rate.
4. Implement the soft-min reduced objective \(\phi_{A,\tau}\) and test a trust-region Newton loop with exact null/serious step verification.
5. Run witness-guided vertex splitting on the current elite \(F\)-level maxima and compare against the old random-addition baseline.

That should tell us very quickly whether the factorial bottleneck can be pushed back enough to change the search frontier.
