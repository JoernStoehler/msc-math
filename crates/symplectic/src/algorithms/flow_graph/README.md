# Flow-Graph Algorithm

Status: live control surface for the flow-graph capacity algorithm work in
`crates/symplectic/src/algorithms/flow_graph/`.

This file is the first file to read before changing the flow-graph algorithm,
tests, experiments, or thesis-facing flow-graph claims.

This file is not thesis prose and not a proof file. Source code, tests,
experiment artifacts, CH2021, accepted Jörn/Kai decisions, and future formal
proof text can overrule it. When they do, update this file or mark the mismatch.

## Current State

- Exact exhaustive search is supported as limited implementation evidence for
  deterministic exact-admissible four-dimensional rational polytopes satisfying
  the input predicates stated below.
- The f64 path is the development path for larger flow-graph searches. It is
  not an exact certificate by itself.
- `exact_tube.rs` contains exact rational closed-word/tube resolution for one
  selected word. `exact_search.rs` contains exact exhaustive search. The exact
  search has an explicit action-cutoff policy: disabled for baseline checks,
  enabled to use action cutoffs after an exact action `> 0` is known.
- The old `research/tube-algorithm*.md` files are legacy/imported source
  material, not the live control surface.
- The current experiment package is `experiments/dev-flow-graph/`.
- The thesis section is `thesis/flow-graph-algorithm-ch2021.tex`.

## Scope and Caveats

The implemented flow-graph path is not meant to work on every geometrically
interesting input. This section records the implemented CH2021-derived
flow-graph/tube route, its checked scope, and its caveats so future proof and
thesis work can start from explicit code behavior instead of reconstructed
development history.

Here "CH2021-derived flow-graph/tube route" means the implemented finite
four-dimensional polytope model whose states are oriented two-faces
`F_i cap F_j`, whose transition `(i,j) -> (j,k)` represents flowing along the
Reeb direction of facet `j`, whose closed candidates are cyclic facet words
expanded as `[s_0, ..., s_m, s_0, s_1]`, and whose candidate orbits are fixed
points of the composed affine tube map. This phrase does not mean the
implementation proves the full CH2021 smoothing theorem, implements CH2021
rotation pruning, handles Type 3/bad-face behavior, or supports all
symplectic polytopes.

Exact implementation scope currently documented here:

- four-dimensional rational polytope data with exact rational dual facet
  normals;
- matching facet-intersection and exact `omega_0`-sign matrices;
- bounded irredundant halfspace data supplied by the existing trusted
  fixture/data-generation path;
- nonzero exact `omega_0` on every geometrically possible transition;
- nonnegative exact action threshold;
- no positive-action singular fixed set encountered by exact closed-word
  resolution;
- exact exhaustive search and retained-word semantics for that input class;
- scalar capacity comparison against certified HK2017/QP as implementation
  evidence, not as a word-level oracle;
- typed rejection or caveat language for unsupported inputs and unresolved
  numerical cases.

Explicit non-goals unless thesis review asks for them:

- supporting Lagrangian products or HKO fixtures with geometrically possible
  `omega_0 = 0` transitions;
- implementing CH2021 rotation pruning;
- turning f64 diagnostics into exact certificates without exact fallback;
- proving every dependency inside code or experiment documentation instead of
  putting mathematical proof in formal/thesis text.

## Algorithm Contract

Target input:

- a 4-dimensional convex polytope represented by facet normals;
- exact rational dual facet normals for the exact path;
- the trusted fixture/data-generation path supplies bounded irredundant
  halfspace data; the exact flow-graph path does not fully validate arbitrary
  raw bounded-irredundant input;
- facet-intersection data matching the facet count;
- exact `omega_0` signs for facet pairs;
- nonzero exact `omega_0` on every geometrically possible transition;
- nonnegative exact action threshold;
- no positive-action singular fixed set during exact closed-word resolution;
- a numerical rejection policy for nearly zero `omega_0` on geometric
  transitions in the f64 path.

Target output shape:

- the reported flow-graph action;
- retained cyclic facet words with action at most `reported action + threshold`;
- the action for each retained word;
- explicit rejection or undecided status when the input or numerical path is
  outside the supported case.

Core objects:

- A facet word records the ordered facets used by a partial trajectory.
- A primitive tube uses a word `[i, j, k]` and represents trajectories entering
  the two-face `F_i cap F_j`, flowing along `R_j`, and leaving through
  `F_j cap F_k`.
- A composite tube is built by intersecting compatible tube data.
- A closed tube has word `[s_0, s_1, ..., s_m, s_0, s_1]`.
- A closed tube candidate is solved by a fixed-point equation on the starting
  two-face.
- Action cutoffs restrict the endpoint polygons by a halfspace.

Search contract:

- Enumerate only simple words: before closure, a facet may appear at most once.
- Reject words containing a geometrically impossible directed transition.
- Cache or build partial tubes only when that changes current computation or
  evidence.
- Return a reported flow-graph action only after the search is exhaustive for
  the supported input class and action threshold.

Path-specific outcome contract:

- Exact accepted output contains the exact reported action, retained cyclic
  facet words, and exact action for each retained word. Exact positive closed
  words are filtered by reconstructing the segment times and requiring every
  segment time to be strictly positive; zero-time boundary fixed points are not
  accepted as positive orbits for the displayed word.
- Exact rejection reasons currently known are: invalid input shapes, a
  geometrically possible `omega_0 = 0` transition, and a positive-action
  singular fixed set. Full bounded-irredundant validation is not part of this
  rejection boundary.
- f64 accepted output has approximate actions and may include words accepted by
  f64 predicates directly. It is not covered by the exact strict segment-time
  contract unless the specific word is also resolved by exact closed-tube
  arithmetic.
- `capacity_f64` is the current f64 wrapper used by tests and experiments. It
  computes a diagnostic f64 closed-word search and resolves every f64
  closed-word error with exact flow-graph closed-word arithmetic. Direct f64
  positive words remain f64 outputs; the exact resolution boundary applies only
  to f64 error words that are reclassified exactly. The wrapper returns a value
  only if exact resolution resolves all such error words without an exact
  construction error or unsupported positive singular outcome. Its returned
  action is the minimum of direct f64 positive words and exact-resolution
  positive words.
- `capacity_exact` is the current exact rational search wrapper used by tests
  and experiments.
- `diagnose_f64_closed_words` is the development/experiment function.
  It may return a candidate action together with per-word errors. Its output is
  not accepted capacity output.
- f64 does not decide `omega_0 = 0`. It rejects when a geometrically possible
  transition has `omega_0` too close to `0` for the f64 policy.
- f64 also rejects near-singular fixed-point problems unless earlier cheap
  checks make the word irrelevant.
- Counts and timing are not API output. Use `tracing`, profiling, benchmarks,
  or experiment-local output for them.

## Dependency Ledger

Status labels:

- `source-backed`: supported by current code, tests, paper source, or an
  accepted durable note.
- `recovered-proof-unreviewed`: appears in the recovered May formal proof
  surface, but that file is agent-written and not accepted thesis proof.
- `Jörn review needed`: accepted in chat or plausible, but not yet durable
  enough for thesis/code reliance.
- `unproved`: known dependency without a checked proof in this repo.
- `implementation evidence`: current code/tests exercise it, but do not prove
  the mathematical claim.
- `future`: not needed for the first supported path.

| statement | status | current use | next check |
| --- | --- | --- | --- |
| Minimum-action generalized Reeb orbits may be chosen simple. | source-backed by HK2017 Theorem 1.5 / `simple_loop_theorem`; thesis legacy restates as `thm:simple-minimizer` | reduces the capacity search to simple Reeb orbits | cite HK2017 in thesis prose; do not re-prove as an FG theorem |
| A simple minimizer visits each facet direction at most once. | source-backed by HK2017 Theorem 1.5 / `simple_loop_theorem` | justifies searching facet words without repeated facet directions | prove only the FG correspondence between simple Reeb orbits and searched FG words |
| Nonzero `omega_0` is required only for geometrically possible transitions, not for all facet pairs. | source-backed by `research/tube-algorithm.md` accepted clarification; implemented by exact and f64 validators | keeps zero `omega_0` on empty two-faces out of the rejection condition | adapt the recovered pairwise-nonzero proof if thesis theorem wording needs the weaker condition |
| Empty tube implies every containing tube is empty. | source-backed by `research/tube-algorithm.md`; recovered-proof-unreviewed via `lem:tube-intersection` | pruning and cache interpretation | migrate the invariant into active proof text before theorem-strength thesis use |
| Empty unclosed subtube makes the closed tube empty. | source-backed by `research/tube-algorithm.md`; recovered-proof-unreviewed via `lem:tube-intersection` | closed-word pruning | migrate the invariant into active proof text before theorem-strength thesis use |
| Action equals elapsed flow time for the stored Reeb segments. | source-backed by `research/tube-algorithm-raw-jorn-2026-05-04.md`; recovered-proof-unreviewed via `def:tube-data` and `lem:tube-intersection` | action computation | migrate the normalization calculation into active proof text before theorem-strength thesis use |
| Restricting a valid tube by a smaller action cutoff is an affine halfspace restriction. | recovered-proof-unreviewed via `lem:tube-action-restriction` | action cutoff pruning | migrate the lemma into active proof text before theorem-strength thesis use |
| Rejection for empty facet intersection is sound. | source-backed | transition matrix | keep tests around transition construction |
| Rejection for forbidden `omega_0` direction is sound. | source-backed for code path; recovered-proof-unreviewed via `def:tube-positive-transition-signs` and `prop:tube-search-correctness-finite-orbit-regular` | transition matrix | reconcile sign convention before theorem-strength thesis use |
| f64 rejection near small geometric `omega_0` is a numerical policy, not a theorem. | source-backed | input validation | test rejection behavior |
| Singular closed-tube fixed-point cases are not silently capacity values. | implementation evidence | exact and f64 closed-word resolution | reject positive-action singular fixed sets unless start/end polygons are disjoint |
| CH2021 rotation pruning is optional future work. | source-backed from legacy note | not in first implementation path | add only behind a flag after formula review |

## Proposition Ledger

The proposition ledger states what tests and experiments are meant to support.
It does not replace proofs.

Status labels:

- `implementation evidence`: supported by existing code or tests, but not a
  proof of the mathematical theorem.
- `future`: useful later, not required for the limited thesis packet.

| id | proposition | evidence type | current status | next check |
| --- | --- | --- | --- | --- |
| P1 | On selected generated exact-admissible polytopes, exact exhaustive flow-graph search returns the same scalar capacity as certified QP. | exact implementation evidence plus QP comparison evidence | implementation evidence for F5/F6 default tests and ignored F7 checks | broaden only if thesis asks for stronger empirical coverage |
| P2 | On selected generated exact-admissible polytopes and chosen action thresholds, exact exhaustive flow-graph search returns the cyclic flow-graph words with action `<= capacity + threshold` that pass exact closed-tube resolution and the strict segment-time filter. | exact implementation evidence | implementation evidence from exact retained-word checks against full flow-graph resolution | keep retained-word checks flow-graph-internal unless an overlap convention with QP is defined |
| P3 | On the overlap where both conventions apply, exact flow-graph retained words and certified QP gap-window orbits agree. | cross-algorithm sanity check | future | compare only after defining the overlap convention |
| P4 | Expected-rejected polytopes are rejected for the expected concrete reason. | exact/f64 implementation evidence | implementation evidence for invalid input shapes, exact zero-`omega_0`, f64 near-zero `omega_0`, and selected unsupported cases | maintain separate buckets for future rejection cases |
| P5 | f64 accepted output agrees with exact or QP after ordinary indeterminate predicates that could affect output are resolved exactly. | f64 implementation evidence plus exact/QP comparison evidence | future | define ordinary predicate fallback scope before adding broad f64 e2e tests |

## First Exact E2E Suite

Purpose:

- support P1 and P2;
- test exact exhaustive search on selected deterministic exact-admissible
  polytopes;
- avoid f64 behavior and rejection behavior in this suite.

For each polytope and action threshold:

- exact flow-graph search returns accepted output;
- exact flow-graph capacity equals certified QP capacity;
- every retained flow-graph word has action `<= capacity + threshold`;
- every exact flow-graph checked word with action `<= capacity + threshold` is
  retained;
- retained words are compared up to cyclic rotation.

QP use:

- QP is used for the scalar exact capacity comparison.
- QP is not used as the retained-word oracle.

Initial buckets:

- tiny deterministic accepted polytopes for default fast e2e smoke;
- deterministic accepted polytopes with nonzero action threshold to test
  retained words above capacity and below `capacity + threshold`;
- ignored slower exact accepted polytopes for occasional stronger evidence.

Separate rejection suite:

- invalid input shapes;
- exact geometrically possible `omega_0 = 0` transition;
- exact positive-action singular fixed set.

Out of scope for this exact e2e suite:

- f64 near-zero `omega_0` rejection;
- f64 near-singular fixed-point rejection;
- f64 exact resolution behavior;
- visualization;
- profiling;
- QP retained-word equality outside the convention overlap.

## Evidence Ledger

Code map:

- `mod.rs` owns the public module surface and re-exports;
- `words.rs` owns combinatorial word enumeration and half-cache helpers;
- `f64_tube_search.rs` owns f64 input validation, tube geometry, f64 search,
  exact fallback wrapper, and visualization snapshots;
- `exact_tube.rs` owns exact rational closed-word/tube resolution for one word;
- `exact_search.rs` owns exact exhaustive word search, action cutoffs, retained
  orbit aggregation, and exact capacity output.

Current evidence:

- f64 tests live in `tests.rs` and `tests_e2e_prediction.rs` as children of
  `f64_tube_search.rs`, so they can inspect f64 implementation details without
  making those fields public.
- `experiments/dev-flow-graph/frontier` measures word-frontier and f64 tube counts.
- `experiments/dev-flow-graph/endpoint-spike` is an exact endpoint-set spike, not a
  supported exact implementation.
- `exact_tube.rs` has exact rational tests for polygon intersection and selected
  closed words. Current checked cases include a positive F7 word, a zero-action
  F7 word, and an F6 word where same-sigma QP has a critical point/action while
  exact flow-graph tube arithmetic returns an empty tube. Exact positive
  closed-word output now reconstructs segment times and rejects non-strict
  boundary fixed points before returning a positive orbit.
- `exact_search.rs` has baseline rejection tests for exact exhaustive search.
  The proposition-style exact exhaustive tests currently live in the
  `exact_tube.rs` test module because they reuse closed-word fixtures and
  assertions from that module.
- The proposition-style exact accepted-polytopes tests support P1 and P2. The
  default F5/F6 tests check certified QP capacity equality and retained-word
  completeness for zero and positive action gaps. The Rust `#[ignore]` F7
  gap-zero and positive-gap tests passed in release on 2026-06-06 in about two
  minutes each.
- The cutoff-enabled exact search policy is tested against the disabled policy
  on an F6 polytope. A separate exact single-word test checks that adding an action
  cutoff can make a known higher-action F7 word empty.
- Exact exhaustive search rejects invalid matrix shapes and inputs with a
  geometrically possible zero-omega transition before enumeration. Full
  bounded-irredundant validation is still a remaining implementation task.
- `tests_e2e_prediction.rs` now checks deterministic generated polytopes where
  diagnostic f64 search has closed-word errors: the diagnostic f64 best word
  exact-resolves to the QP capacity, and every f64 error word exact-resolves to
  no lower positive action. It also asserts the exact-resolution composition of
  those diagnostic f64 error words.
  The current F5/F7 smoke cases have no exact positive diagnostic f64 error word
  below QP capacity; the F7 case has one exact positive diagnostic f64 error word
  above capacity. The first two F10 cases also have no exact positive diagnostic
  f64 error word below QP capacity. This prediction bucket now includes F10
  polytopes. `capacity_f64` matches QP on the F5/F7 smoke cases and on the first
  two F10 deliberate cases.
  The default release flow-graph suite passed on 2026-06-06 in 53.92 seconds.
- f64 near-singular fixed-point solving now keeps the existing inconsistent
  linear-system skip and additionally skips the word when
  `start_polygon cap end_polygon` is definitely empty. On the first two F10
  discovery attempts this did not reduce the closed-word error count; the
  remaining F10 errors are still near-singular fixed-point maps and
  numerically indeterminate polygons.
- The deliberate Rust `#[ignore]` verification suite contains exact F7 checks,
  F10 exact closed-word resolution checks, and F10 capacity_f64
  FG == QP checks. It passed on 2026-06-06 in 141.98 seconds.
- `experiments/dev-flow-graph/unresolved-diagnostic` emits both per-word records
  and an aggregate diagnostic f64 error-word summary. The summary separates f64
  near-singular fixed-point errors, f64 polygon-indeterminate errors, exact
  empty tubes, exact zero-action no-orbits, exact positive orbits, exact
  unsupported singular outcomes, and exact positives below or at/above the QP
  capacity.
- On 2026-06-21, exact rejection tests were added for the Lagrangian triangle
  product and Lagrangian triangle-square zero-`omega_0` fixtures.
  `cargo test -p symplectic --release --lib flow_graph` passed with 40 tests
  passed, 6 ignored, and 0 failed. The same checkpoint also passed
  `cargo fmt --check`, `cd thesis && latexmk && ./check-build.sh`, and
  `git diff --check`.

Current analysis inventory:

| Surface | Current analysis/evidence | Current home | Promotion target |
| --- | --- | --- | --- |
| Exact closed-word/tube arithmetic | polygon intersection tests, selected positive/zero/empty-tube word cases, exact action/fixed-point behavior | `exact_tube.rs` tests | crate regression tests; only promote artifact-backed slower suites to `experiments/verification/` |
| Exact exhaustive search | exact rejection tests, P1/P2 accepted-polytopes checks, action-cutoff comparison against disabled policy | `exact_search.rs` tests and proposition-style tests in `exact_tube.rs` | crate tests while cheap; `experiments/verification/` for slower fixed suites |
| f64 tube search and exact resolution | primitive/intersection/fixed-point unit checks, generated f64-error cases exact-resolved against QP scalar capacity | `tests.rs`, `tests_e2e_prediction.rs`, `f64_tube_search.rs` | `experiments/numerics/` only when the question becomes reusable f64/exact methodology |
| Word frontier and f64 tube counts | transition-pruned frontier sizes, f64 tube live/empty/unsupported counts, polygon-operation counters | `experiments/dev-flow-graph/frontier` | `experiments/performance/` once the measured algorithm path is stable |
| Endpoint and closed-word spikes | exploratory exact endpoint-set and selected closed-word representation experiments | `experiments/dev-flow-graph/endpoint-spike`, `experiments/dev-flow-graph/closed-word-spike` | retire to git history or keep dev-local notes once the crate exact path supersedes them |
| Case discovery | bucketed search for high-value FG examples and expected labels | `experiments/dev-flow-graph/discover-e2e` | crate tests for cheap reviewed rows; `experiments/verification/` for slower artifact-backed suites |
| Unresolved f64 diagnostics | f64 failure taxonomy, exact tube resolution, exact one-sigma QP summaries, geometric recovery references | `experiments/dev-flow-graph/unresolved-diagnostic` | `experiments/numerics/` for reusable f64/exact behavior; `experiments/verification/` for stable error-path evidence |
| Tube visualization | JSON snapshots and rendered tube geometry for selected words | `experiments/dev-flow-graph/visualize-tube` | thesis/topic asset packet only for selected exposition figures |

Role of HK2017/QP comparison:

- HK2017/QP is not used inside the flow-graph algorithm.
- For a specific flow-graph word, HK2017/QP is not used to decide whether the
  tube is empty, whether the closed tube has a fixed point, whether the action
  is positive, whether the word can be pruned, or whether an f64-indeterminate
  word is resolved.
- Those word-level decisions belong to flow-graph-specific f64 tube arithmetic,
  exact tube arithmetic, and flow-graph pruning/stopping lemmas.
- HK2017/QP is used in the verification layer: compare the final scalar
  capacity returned by flow-graph with the final scalar capacity returned by
  HK2017/QP on the same exact-admissible polytope.
- HK2017/QP is not a literal retained-word oracle for flow-graph. QP uses its
  own candidate convention. The exact flow-graph output reports words accepted
  by exact closed-tube resolution and the strict segment-time filter.
- QP has certified exact gap-window support, but retained-word comparisons must
  be restricted to the overlap where both algorithms' conventions apply.
- Passing that comparison on diverse exact-admissible polytopes is a useful
  implementation check, but not a proof of the flow-graph theorem, stopping
  rule, or exact tube arithmetic.

Evidence limits for future thesis wording:

- the exact path is implementation evidence for the stated input class, not a
  standalone proof of the CH2021 capacity theorem, stopping rule, or tube
  arithmetic;
- arbitrary raw bounded-irredundant input is not fully validated by the
  flow-graph exact path;
- f64 diagnostics and `capacity_f64` are not exact certificates; only
  individual f64 error words that pass the exact closed-word resolution boundary
  receive exact closed-word status;
- performance, rotation pruning, and broader F7/F8 action-cutoff profiling are
  future work unless a later task needs stronger claims.

## Decisions

- Use the name `flow_graph` for code and package paths.
- Keep "tube" as a mathematical object name only when useful.
- Keep CH2021 rotation pruning out of the first supported implementation path.
- Keep exact and f64 paths conceptually separate. Do not imply that the f64 path
  proves exact capacity values by itself.
- Keep experiments in `experiments/dev-flow-graph/`, not under
  `experiments/combinatorial-cells/`.

## Failure Log

- A previous planning pass treated source containers as adequate planning. That
  was wrong; this README is meant to make the result state explicit.
- A previous planning pass introduced a new top-level `flow_graph/` directory.
  That added repo clutter and was rejected.
- A previous code/planning pass appears to have drifted from Jörn's algorithm
  definition. Future changes must update this README when code semantics differ
  from the contract above.
- Claims imported only from chat are not durable until recorded here or in a
  proof/source file with status.
- As of 2026-06-05,
  `cargo test -p symplectic --release --lib flow_graph` passes after replacing
  the old random agreement test with an e2e prediction smoke suite. The suite
  currently has rejection cases, deterministic generated near-equality polytopes
  with f64 closed-cycle errors, exact closed-word resolution for those f64 error
  words, and a deterministic generated mismatch polytope.

## Source Ledger

Project sources:

- `AGENTS.md`
- `tasks/definition-of-success.md`
- `tasks/current-state.md`
- `tasks/planning-notes.md`
- `research/tube-algorithm.md`
- `research/tube-algorithm-raw-jorn-2026-05-04.md`
- `thesis/flow-graph-algorithm-ch2021.tex`
- `papers/ch2021/`

## Maintenance Rule

Update this file when:

- the algorithm contract changes;
- a dependency changes status;
- code implements a different semantic path;
- experiments add evidence used by thesis text;
- a failure changes future safeguards;
- thesis wording relies on a stronger flow-graph claim.
