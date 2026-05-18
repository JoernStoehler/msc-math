# Tube Algorithm Source Note

## Status

Epistemic status: fillable source note for Jörn's current tube-algorithm
formalization. This file is intended to become the source of truth before we
write any formal TeX, thesis prose, or Rust implementation.

Current state, as of 2026-05-04: the algorithm mostly exists in Jörn's head and
on paper. Old repo material was deleted from the active tree because it was not
trusted as a specification. Use git history only if comparison with the stale
drafts becomes necessary.

Refresh or invalidate this note when Jörn fills the algorithm contract below,
when a formal TeX file is written from that contract, or when the Rust tube
implementation becomes supported.

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
- [accepted 2026-05-04] The implementation should use a functional-programming
  style with modular primitives. Define what a tube is, how to intersect tubes,
  how to build primitive three-facet tubes `(a_1,a_2,a_3)` describing flow from
  `a_1 cap a_2` to `a_2 cap a_3` along `R_2 = 2J a_2`, how to detect empty
  tubes, and how to solve fixed points of closed tubes. The orchestrator is a
  separate layer that chooses the tube-build order, first to get a good action
  bound quickly and then to exhaust the full tube set. An empty sub-segment
  implies every containing tube is empty.
- [accepted 2026-05-04] A closed tube has combinatorics
  `(a_1,a_2,...,a_k,a_1,a_2)`. The start and end both live on
  `a_1 cap a_2`; fixed points are solved on that two-face.
- [accepted 2026-05-04] For thesis/numerics wording, it is acceptable to state
  a stronger input condition than exact mathematics needs.
- [accepted 2026-05-04] Thesis scope: implementation and empirical validation
  are still part of the desired complete tube story. Theory-only is not enough
  for the ideal outcome, but including clean theory without empirics is better
  than dropping the tube algorithm entirely.

## Deleted Repo Noise

These stale surfaces were removed from the active tree on 2026-05-04. They were
not trusted as specifications; use git history only if comparison material is
needed.

- `thesis/tube-algorithm.tex`: long agent-written thesis draft. It contained
  many Jörn TODOs and unreviewed claims about rotation, closing, and Type 2
  orbits.
- `formal/tube-algorithm.tex`: compressed formal copy sourced from the thesis
  draft, wrapped in `unverified` blocks. It was not independent proof-bearing
  source.
- `crates/symplectic/src/algorithms/tube/mod.rs`: blocked test-only Rust module.
  Its header said the rotation-increment formula was incorrect and
  `tube_capacity` was not re-exported.
- `thesis/tube-algorithm-notes.md`: stale migration task note. It pointed to
  deleted files and old `library/` paths.
- `thesis/legacy/migration-findings.md`: legacy-era mismatch inventory. Rows 1
  and 11-14 may still be useful as a checklist for conflicts between old thesis
  prose and old code, but need revalidation before driving current work.

Potential downstream work after this source note is filled:

1. Write a new formal tube-algorithm file from this note, preserving only
   verified statements.
2. Write a new thesis tube-algorithm section from the formal contract before
   adding it to the thesis build.
3. Build a new Rust tube implementation from the current mathematical source,
   instead of patching around the known-bad deleted module.

## Intended Role

Fill this section first.

- Thesis role:
   - The Tube Algorithm was developed and discussed with Kai, and it promises to be vastly faster for some polytopes, under the restriction that it uses the generic condition "omega_0(a_i,a_j)!=0" which in particular excludes lagrangian products, and it's unlikely we can get rid of that condition (beyond like, slightly weakening it or sth).
   - So it's just for completeness sake to define it and not loose it, and also it may be great for getting vastly more data (like 10x or sth), plus get additional confidence in correctness [since then rather different approaches yield the same results]
   - Theory without empirical validation is not enough for the ideal tube
     story, but clean theory without empirics is still worth including over
     dropping the algorithm.
- Experiment/code role:
   - It's a search algorithm that finds the minimum action + all simple Reeb orbits below some action threshold (optionally: it rejects reeb orbits that have too high CZ index to be minimal, but that can be disabled) ; so we have a nice high-level comparable output between Tube and HK2017
   - Conceptually, HK2017 makes use of pruning on a "adjacent" level; Tube prunes on arbitrary order, i.e. it uses the same "Reeb orbit => combinatorics sigma" map, but does not just check whether pairs (sigma_i, sigma_i+1) have any Reeb trajectory at all, but checks arbitrary segments. i.e. it can for example check whether any trajectory goes thorugh a triplet (a,b,c) ; it does so via an intersection-like algorithm i.e. for segments (a_1,...,a_k, a_k+1) and (a_k, a_k+1, ..., a_m) it obtains for each side the set of trajectories that go through the segment, then intersects the two sets to obtain (a_1, ..., a_m) ; closed loops can be detected by taking (a_1, ..., a_k, a_1) and looking for fixed points of the start-end affine map.
   - We encode the set of trajectories that have compatible combinatorics as the convex sets of points on the intersection F_1 \cap F_2 which is part the polytope boundary ; the intersection is for non-redundant representations always something like an empty set, or a 0/1/2-face. So we can just pick any base point and basis of the affine hyperplane H_1 \cap H_2 = { <x, a_1> = 1, <x, a_2> = 1 } [genericity is needed for this to be a 2-dim affine space]. We also encode the affine map from F_1 \cap F_2 to F_k-1 \cap F_k. This graph however is only a function if we exclude polytopes that allow free-moving breakpoints which happens when there's some finite segment along R_1=2Ja_1 on the boundary where both a_1,a_2 are active, which is exactly the condition omega_0(a_1,a_2) = 0. If we assume genericity here, we get that we can flow from 2-hyperplane to 2-hyperplane via an affine map, and we can compose those.
   - Intersection is then simply to compose the two maps, and update/intersect the start&end sets by pushforward/pullback
   - Closing is then to append `(a_1,a_2)` to obtain a closed tube
     `(a_1,a_2,...,a_k,a_1,a_2)` and solve fixed points of the resulting map
     from `F_1 cap F_2` to itself.
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
- Edge cases: non-transition facet pairs may have `omega_0(a_i,a_j) = 0`;
  only actual transition pairs need the nondegeneracy condition.

### Tube

- Facet sequence:
- Start set:
- End set:
- Parameterization:
- Empty-tube condition:
- Convexity or shape invariant:
- Primitive constructor: build three-facet tubes `(a_1,a_2,a_3)` encoding flow
  from `a_1 cap a_2` to `a_2 cap a_3` along `R_2 = 2J a_2`.
- Intersection operation: combine compatible tubes by intersecting the induced
  trajectory sets and updating start/end sets through pushforward/pullback.

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

- When closing is attempted: after appending `(a_1,a_2)` to a tube with initial
  facets `(a_1,a_2)`.
- Required closing edges: the closed combinatorics are
  `(a_1,a_2,...,a_k,a_1,a_2)`.
- Fixed-point equation: solve the start-end map on `a_1 cap a_2`.
- Candidate orbit validation:
- Action computation:

## Algorithm Contract

Write the algorithm as precise steps, not implementation details.

1. Precompute:
2. Build primitive tubes:
3. Intersect/build composite tubes:
4. Detect empty tubes:
5. Close and solve fixed points:
6. Orchestrate search order:
7. Select output:

## Pruning Claims

For each pruning rule, record the exact statement and the proof dependency.

### Empty Tube

- Rule: if a tube segment is empty, then every larger tube containing that
  segment is empty.
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
- Data structures: prefer modular, functional primitives for tube construction,
  tube intersection, empty detection, and closed-tube fixed-point solving; keep
  search heuristics in a separate orchestrator layer.
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
