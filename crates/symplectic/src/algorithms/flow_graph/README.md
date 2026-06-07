# Flow-Graph Algorithm

Status: live control surface for the flow-graph capacity algorithm work in
`crates/symplectic/src/algorithms/flow_graph/`.

This file is the first file to read before changing the flow-graph algorithm,
tests, experiments, or thesis-facing flow-graph claims.

This file is not thesis prose and not a proof file. Source code, tests,
experiment artifacts, CH2021, accepted Jörn/Kai decisions, and future formal
proof text can overrule it. When they do, update this file or mark the mismatch.

## Current State

- Exact exhaustive search is a thesis artifact target for exact-admissible
  polytopes. It is not yet thesis-ready.
- The f64 path is the intended production path for larger polytopes. It is not
  an exact certificate by itself.
- `exact.rs` contains the first exact rational closed-word resolver and an exact
  exhaustive search. The exact search has an explicit action-cutoff policy:
  disabled for baseline checks, enabled to use action cutoffs after an exact
  action `> 0` is known.
- The old `research/tube-algorithm*.md` files are legacy/imported source
  material, not the live control surface.
- The current experiment package is `experiments/flow-graph/`.
- The thesis section is `thesis/flow-graph-algorithm-ch2021.tex`.

## Algorithm Contract

Target input:

- a 4-dimensional convex polytope represented by facet normals;
- the dual vertices must define a bounded irredundant intersection of
  halfspaces, as in the QP path;
- facet-intersection data;
- `omega_0` signs for facet pairs;
- a numerical rejection policy for nearly zero `omega_0` on geometric
  transitions in the f64 path.

Target output:

- the capacity action;
- retained cyclic facet words with action at most `capacity + threshold`;
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
- Return a capacity only after the search is exhaustive for the supported input
  class and action threshold.

Path-specific outcome contract:

- Exact accepted output contains the exact capacity action, retained cyclic
  facet words, and exact action for each retained word.
- Exact rejection reasons currently known are: invalid bounded irredundant
  halfspace input, a geometrically possible `omega_0 = 0` transition, and a
  positive-action singular fixed set.
- f64 accepted output has the same ordinary shape as exact accepted output, but
  with approximate actions and a different error type.
- `capacity_f64` is the current f64 wrapper used by tests and experiments. It
  computes a diagnostic f64 closed-word search and resolves every f64
  closed-word error with exact flow-graph closed-word arithmetic. It returns a
  value only if exact resolution resolves all such words without an exact
  construction error or unsupported positive singular outcome. Its returned
  action is the minimum of f64 positive words and exact-resolution positive
  words.
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
- `Jörn review needed`: accepted in chat or plausible, but not yet durable
  enough for thesis/code reliance.
- `unproved`: known dependency without a checked proof in this repo.
- `implementation evidence`: current code/tests exercise it, but do not prove
  the mathematical claim.
- `future`: not needed for the first supported path.

| statement | status | current use | next check |
| --- | --- | --- | --- |
| Simple Reeb orbits suffice for the retained capacity search target. | Jörn review needed | justifies simple-word enumeration | import chat/source proof into this file or formal note |
| A simple orbit visits each facet at most once before closure. | Jörn review needed | pruning repeated facets | record proof or accepted reference |
| Empty tube implies every containing tube is empty. | Jörn review needed | pruning and cache interpretation | write invariant precisely |
| Empty unclosed subtube makes the closed tube empty. | Jörn review needed | closed-word pruning | write invariant precisely |
| Action equals flow time for Reeb segments. | Jörn review needed | action computation | record proof using `lambda_0(R_j)=1` |
| Extending a valid tube does not decrease action. | Jörn review needed | action cutoff pruning | state assumptions and proof |
| Rejection for empty facet intersection is sound. | source-backed | transition matrix | keep tests around transition construction |
| Rejection for forbidden `omega_0` direction is sound. | source-backed for code path; proof needs review | transition matrix | link proof/reference before thesis claim |
| f64 rejection near small geometric `omega_0` is a numerical policy, not a theorem. | source-backed | input validation | test rejection behavior |
| Singular closed-tube fixed-point cases are not silently capacity values. | implementation evidence | exact and f64 closed-word resolution | reject positive-action singular fixed sets unless start/end polygons are disjoint |
| CH2021 rotation pruning is optional future work. | source-backed from legacy note | not in first implementation path | add only behind a flag after formula review |

## Proposition Ledger

The proposition ledger states what tests and experiments are meant to support.
It does not replace proofs.

Status labels:

- `target`: intended thesis-supporting proposition, not fully established yet.
- `current evidence`: supported by existing code or tests, but not enough for
  thesis reliance.
- `future`: useful later, not required for the next exact e2e suite.

| id | proposition | evidence type | current status | next check |
| --- | --- | --- | --- | --- |
| P1 | For deterministic exact-admissible polytopes, exact exhaustive flow-graph search returns the same exact capacity as certified QP. | exact implementation evidence plus QP comparison evidence | target | build exact e2e suite using certified QP capacity |
| P2 | For deterministic exact-admissible polytopes and a chosen action threshold, exact exhaustive flow-graph search returns exactly the cyclic flow-graph words with action `<= capacity + threshold` under the flow-graph convention, including zero-time segments. | exact implementation evidence | target | test retained-word completeness against the exact flow-graph enumeration, not against QP |
| P3 | On the overlap where both conventions apply, exact flow-graph retained words and certified QP gap-window orbits agree. | cross-algorithm sanity check | future | compare only after defining the overlap convention |
| P4 | Expected-rejected polytopes are rejected for the expected concrete reason. | exact/f64 implementation evidence | target | maintain separate buckets for invalid halfspace input, exact `omega_0 = 0`, f64 near-zero `omega_0`, and singular fixed-point cases |
| P5 | f64 accepted output agrees with exact or QP after ordinary indeterminate predicates that could affect output are resolved exactly. | f64 implementation evidence plus exact/QP comparison evidence | future | define ordinary predicate fallback scope before adding broad f64 e2e tests |

## First Exact E2E Suite

Purpose:

- support P1 and P2;
- test exact exhaustive search on deterministic exact-admissible polytopes;
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

- invalid bounded irredundant halfspace input;
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

Current evidence:

- Unit tests live in `tests.rs` while the module remains compact.
- `experiments/flow-graph/frontier` measures word-frontier and f64 tube counts.
- `experiments/flow-graph/endpoint-spike` is an exact endpoint-set spike, not a
  supported exact implementation.
- `exact.rs` has exact rational tests for polygon intersection and selected
  closed words. Current checked cases include a positive F7 word, a zero-action
  F7 word, and an F6 word where same-sigma QP has a critical point/action while
  exact flow-graph tube arithmetic returns an empty tube.
- `exact.rs` also has a baseline exhaustive exact search over transition-pruned
  simple closed words. Default tests include an F6 polytope. Two F7 exhaustive tests
  are ignored because they take about 60 seconds in release; run them before
  changing exact search semantics.
- `exact.rs` now has proposition-style exact accepted-polytopes tests for P1
  and P2. The default F5/F6 tests check certified QP capacity equality and
  retained-word completeness for zero and positive action gaps. The Rust
  `#[ignore]` F7 gap-zero and positive-gap tests passed in release on
  2026-06-06 in about two minutes each.
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
- `experiments/flow-graph/unresolved-diagnostic` emits both per-word records
  and an aggregate diagnostic f64 error-word summary. The summary separates f64
  near-singular fixed-point errors, f64 polygon-indeterminate errors, exact
  empty tubes, exact zero-action no-orbits, exact positive orbits, exact
  unsupported singular outcomes, and exact positives below or at/above the QP
  capacity.

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
  own candidate convention, including positive dwell-time filters, while
  flow-graph allows zero-time segments in its tube convention.
- QP has certified exact gap-window support, but retained-word comparisons must
  be restricted to the overlap where both algorithms' conventions apply.
- Passing that comparison on diverse exact-admissible polytopes is strong
  implementation evidence, but not a proof of the flow-graph theorem, stopping
  rule, or exact tube arithmetic.

Evidence still needed before thesis writeup can treat this as a finished
non-writing result:

- focused unit tests for primitive tubes;
- focused unit tests for tube intersection;
- focused unit tests for action cutoff restriction;
- broader fixed-point tests for closed tubes, including singular/rejected cases;
- f64 rejection tests for nearly zero geometric `omega_0`;
- exact-vs-f64 comparison on exact-admissible examples;
- comparison to HK2017 on the same exact-admissible polytopes;
- release-mode profiling with counts for polygon operations and word counts.
- broader action-cutoff tests on F7/F8 polytopes and profiling for how much exact
  cutoff pruning helps;

## Decisions

- Use the name `flow_graph` for code and package paths.
- Keep "tube" as a mathematical object name only when useful.
- Keep CH2021 rotation pruning out of the first supported implementation path.
- Keep exact and f64 paths conceptually separate. Do not imply that the f64 path
  proves exact capacity values by itself.
- Keep experiments in `experiments/flow-graph/`, not under
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
