# Tube Algorithm Source Note

## Status

Epistemic status: fillable source note for Jörn's current tube-algorithm
formalization. This file is intended to become the source of truth before we
rewrite `formal/tube-algorithm.tex`, `thesis/tube-algorithm.tex`, or
`crates/symplectic/src/algorithms/tube/mod.rs`.

Current state, as of 2026-05-04: the algorithm mostly exists in Jörn's head and
on paper. Existing repo material is useful for finding old terminology and
possible implementation hooks, but it is not trusted as a specification.

Refresh or invalidate this note when Jörn fills the algorithm contract below,
when the formal TeX file is rewritten from that contract, or when the Rust tube
module becomes a supported implementation.

## Accepted Clarifications

- [accepted 2026-05-04] The nondegeneracy condition is local to trajectory
  transitions. The algorithm does not need `omega_0(a_i,a_j) != 0` for every
  pair of facets. Facet pairs that never occur as a nonempty trajectory
  transition may have `omega_0(a_i,a_j) = 0`. The intended weaker condition is:
  whenever the point-intersection relevant to a trajectory transition is
  nonempty, the corresponding `omega_0(a_i,a_j)` is nonzero.
- [accepted 2026-05-04] Rotation pruning is not part of the first implementation
  milestone. The first milestone should write a TODO for rotation and keep the
  algorithm correct without it. Later implementation can add the
  Conley-Zehnder/rotation cutoff behind an easy-to-disable flag, because it is
  control-flow pruning by a scalar number rather than part of the affine tube
  data.
- [accepted 2026-05-04] The target output is `capacity` and all simple Reeb
  orbits below `capacity + threshold`. The action pruning rule is therefore
  based on `segment_action <= best_action_so_far + threshold`.

## Existing Repo Noise

Treat these files as downstream or stale until this note says otherwise:

- `thesis/tube-algorithm.tex`: long agent-written thesis draft. It is currently
  included by `thesis/main.tex`, but it contains many Jörn TODOs and unreviewed
  claims about rotation, closing, and Type 2 orbits.
- `formal/tube-algorithm.tex`: compressed formal copy sourced from the thesis
  draft, wrapped in `unverified` blocks. It is currently included by
  `formal/main.tex`, but it should not be treated as independent proof-bearing
  source.
- `crates/symplectic/src/algorithms/tube/mod.rs`: blocked test-only Rust module.
  Its header says the rotation-increment formula is incorrect and
  `tube_capacity` is not re-exported.
- `thesis/tube-algorithm-notes.md`: stale migration task note. It points to
  deleted files and old `library/` paths.
- `thesis/migration-findings.md`: mismatch inventory. Rows 1 and 11-14 are a
  useful checklist for conflicts between old thesis prose and old code.

Potential cleanup after this source note is filled:

1. Delete or replace `thesis/tube-algorithm-notes.md`.
2. Rewrite `formal/tube-algorithm.tex` from this note, preserving only verified
   statements.
3. Either remove `thesis/tube-algorithm.tex` from the thesis build while the
   algorithm is not thesis-ready, or rewrite it from the formal contract.
4. Delete, quarantine, or rebuild the blocked Rust module rather than patching
   around its known-bad rotation formula.

## Intended Role

Fill this section first.

- Thesis role:
   - The Tube Algorithm was developed and discussed with Kai, and it promises to be vastly faster for some polytopes, under the restriction that it uses the generic condition "omega_0(a_i,a_j)!=0" which in particular excludes lagrangian products, and it's unlikely we can get rid of that condition (beyond like, slightly weakening it or sth).
   - So it's just for completeness sake to define it and not loose it, and also it may be great for getting vastly more data (like 10x or sth), plus get additional confidence in correctness [since then rather different approaches yield the same results]
- Experiment/code role:
   - It's a search algorithm that finds the minimum action + all simple Reeb orbits below some action threshold (optionally: it rejects reeb orbits that have too high CZ index to be minimal, but that can be disabled) ; so we have a nice high-level comparable output between Tube and HK2017
   - Conceptually, HK2017 makes use of pruning on a "adjacent" level; Tube prunes on arbitrary order, i.e. it uses the same "Reeb orbit => combinatorics sigma" map, but does not just check whether pairs (sigma_i, sigma_i+1) have any Reeb trajectory at all, but checks arbitrary segments. i.e. it can for example check whether any trajectory goes thorugh a triplet (a,b,c) ; it does so via an intersection-like algorithm i.e. for segments (a_1,...,a_k, a_k+1) and (a_k, a_k+1, ..., a_m) it obtains for each side the set of trajectories that go through the segment, then intersects the two sets to obtain (a_1, ..., a_m) ; closed loops can be detected by taking (a_1, ..., a_k, a_1) and looking for fixed points of the start-end affine map.
   - We encode the set of trajectories that have compatible combinatorics as the convex sets of points on the intersection F_1 \cap F_2 which is part the polytope boundary ; the intersection is for non-redundant representations always something like an empty set, or a 0/1/2-face. So we can just pick any base point and basis of the affine hyperplane H_1 \cap H_2 = { <x, a_1> = 1, <x, a_2> = 1 } [genericity is needed for this to be a 2-dim affine space]. We also encode the affine map from F_1 \cap F_2 to F_k-1 \cap F_k. This graph however is only a function if we exclude polytopes that allow free-moving breakpoints which happens when there's some finite segment along R_1=2Ja_1 on the boundary where both a_1,a_2 are active, which is exactly the condition omega_0(a_1,a_2) = 0. If we assume genericity here, we get that we can flow from 2-hyperplane to 2-hyperplane via an affine map, and we can compose those.
   - Intersection is then simply to compose the two maps, and update/intersect the start&end sets by pushforward/pullback
   - Closing is then to look for fixed points of the map from F_1 \cap F_k to itself
   - We can additionally prune by adding an action upper bound (that for example moves with the current best known closed Reeb orbit); the partial action from the point on a_1,a_2 to a_k-1,a_k can be computed and is an affine map in the start/end point again. so intersection with {action(x) <= bound} is again a 2-dim polygon; actions are additive and >=0 for Reeb trajectories.
   - Finally there's also a rotation cutoff from CH2021 we can use , which prunes entire combinatorics [if i understand how the rotation (which then defines the CZ index) is computed] ; basically every 2-face transition has a rotation increment (>=0)
- Future-work role:
   - MAYBE switch to the faster algorithm for large datasets
   - VERIFY HK2017+Tube against each other
- What success would look like:
   - The tube algorithm is implemented
   - Its numerical behavior has been empirically analyzed
   - An exact math path is available for numerical analysis
   - Its numerical behavior has been proven
   - It is empirically compared to HK2017
   - It is profiled + hotspots have been optimized
   - Thesis contains a chapter where the algorithm is defined and proven correct [this makes sense as sth do to after, or perhaps during, HK2017 pruning discussion, since we just 'prune more smartly' and instead of using the dual problem we actually trace the trajectories (which are "tubes" that fray only at the start & end 2-face and stay tight in the middle)]
   - Visualization of tubes in the 3d viz tool
   - Recap the proof of CH2021 about the rotation definition+bound
   - One main thing to ""profile"" is what heuristic for the search is best - we after all have a dynamic bound, so lowering it as early as possible matters a lot
- What is explicitly out of scope:
   - Fixing the "genericity" assumption
   - Perhaps it's even okay to constraint to "|omega(a_i,a_j)| > eps" if that matters for numerics
   - proving that some heuristic is best vs just comparing different attempts

## Input Object

Describe the precise input class.

- Ambient space:
- Polytope representation:
- Normalization:
- Genericity or nondegeneracy assumptions: required only for actual trajectory
  transitions. It is acceptable for non-transition facet pairs to have
  `omega_0(a_i,a_j) = 0`.
- Lagrangian 2-face assumptions:
- Type 1 / Type 2 boundary:
- Required exactness or numerical tolerance:

## Output Object

Describe what the algorithm returns and what the return value certifies.

- Returned value: capacity.
- Returned orbit or certificate: all simple Reeb orbits with action at most
  `capacity + threshold`.
- Failure / undecided modes:
- Relationship to `c_EHZ(K)`:
- Relationship to HK2017:

## Core Definitions

Use this section to fix notation before the algorithm steps.

### Directed Face Graph

- Vertices:
- Directed edges:
- Edge labels:
- Source theorem/lemma:
- Edge cases:

### Tube

- Facet sequence:
- Start set:
- End set:
- Parameterization:
- Empty-tube condition:
- Convexity or shape invariant:

### Step Map

- Input data:
- Formula:
- Domain:
- Image:
- Action increment:
- Invertibility or singular cases:

### Rotation Increment

- Object whose rotation is measured:
- Formula:
- Range:
- Additivity statement:
- CH2021 reference:
- What remains conjectural or unproved: first implementation milestone should
  skip rotation pruning and leave this as a TODO. Later implementation can add a
  scalar cutoff behind a flag.

### Closing

- When closing is attempted:
- Required closing edges:
- Fixed-point equation:
- Candidate orbit validation:
- Action computation:

## Algorithm Contract

Write the algorithm as precise steps, not implementation details.

1. Precompute:
2. Initialize:
3. Extend:
4. Prune:
5. Close:
6. Select output:

## Pruning Claims

For each pruning rule, record the exact statement and the proof dependency.

### Empty Tube

- Rule:
- Why sound:
- Required invariant:

### Action Lower Bound

- Rule: prune a segment only when its partial action lower bound is greater
  than `best_action_so_far + threshold`.
- Why sound:
- Required invariant:

### Rotation Bound

- Rule:
- Why sound:
- Required invariant:

### Simplicity / Repeated Facets

- Rule:
- Why sound:
- Required invariant:

## Correctness Claim

State the strongest currently intended theorem.

- Theorem statement:
- Preconditions:
- Completeness argument:
- Soundness argument:
- Known missing cases:
- What Jörn needs to review:

## Implementation Notes

Only fill this after the mathematical contract is stable enough to guide code.

- Target Rust module:
- Reusable existing code:
- Data structures:
- Exact versus f64 split:
- Unit tests:
- Comparison tests against HK2017:
- Known old-code traps to avoid:

## Questions For Jörn

Use this as the running queue of decisions that block downstream rewriting.

1. Which parts of the old TeX draft should be kept as terminology only, if any?
2. Is the algorithm meant to prove exact `c_EHZ(K)` for a named class, or to be
   a generic-case search with explicit failure modes?
3. What is the exact rotation-increment formula and where does CH2021 supply
   the needed statement?
4. Are Type 2 orbits excluded by an assumption, handled later, or treated as an
   undecided/failure mode?
5. Is the thesis payoff high enough to keep a tube section in the May 2026
   thesis path?
