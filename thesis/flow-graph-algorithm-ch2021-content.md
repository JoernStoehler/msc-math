# Flow-Graph Algorithm Thesis Content Notes

Status: section-local support ledger for
`thesis/flow-graph-algorithm-ch2021.tex`. Not source truth.

Live algorithm/control source:
`crates/symplectic/src/algorithms/flow_graph/README.md`.

Use this file to check that the thesis section matches implementation support.
If this file and the live flow-graph README disagree, refresh this file from
the README, code, tests, and validation results.

## Thesis Role

The flow-graph section should present FG as a second serious capacity algorithm,
not as a failed route or a minor boundary note. Its thesis role is:

- present the CH2021-derived flow-graph/tube model enough that the reader
  understands what was implemented and why CH2021 is relevant;
- provide an independent behavioral cross-check by comparing eligible FG
  outputs with certified HK/QP scalar capacities;
- explain why QP remains the practical thesis workhorse: the implemented FG
  route has narrower supported inputs, explicit degeneracy rejections, and no
  demonstrated performance advantage.

The section should not frame QP as chosen because FG is unserious. A smarter FG
search could in principle change the performance story, but this thesis does
not rely on that possibility and does not use FG as the datascience or retained
experiment workhorse.

## Thesis Claim Strength

Current thesis-facing claims:

- FG is included as a second serious implemented approach to scalar capacity,
  not as a failed attempt.
- For the stated restricted class of rational 4D inputs, exact FG enumerates
  closed Chaidez--Hutchings flow-graph words, solves tube fixed-point equations
  exactly, filters accepted positive words by strict segment times, and returns
  a scalar flow-graph action.
- On eligible generated examples, FG's scalar capacities agree with certified
  HK/QP scalar capacities.
- The method is partial: it rejects or excludes zero-`omega_0`, singular
  fixed-set, HKO, and Lagrangian-product degeneracies.
- f64 computations are used for exploratory searches, debugging, and proposing
  candidate closed words; they do not certify thesis capacity values.
- QP remains the practical workhorse because FG currently has narrower
  applicability and no demonstrated performance advantage.

Details that can appear locally when needed, but should not be headline claims:

- The exact implementation input class uses exact rational dual facet normals,
  matching facet-intersection and exact `omega_0`-sign matrices,
  bounded-irredundant halfspace data established before the flow-graph search
  is run, nonzero exact `omega_0` on every transition whose two-face is marked
  nonempty, nonnegative exact action threshold, and no positive-action singular
  fixed set during exact closed-word resolution.
- Exact search returns retained exact flow-graph words up to an action
  threshold after exact closed-tube resolution and the strict segment-time
  filter.
- HK/QP is a scalar comparison check only; it is not a retained-word oracle.

The section does not claim a universal certified solver, CH2021 rotation
pruning, full CH2021 smoothing-theorem proof, arbitrary raw-input validation, or
support for HKO/Lagrangian-product degeneracies.

Correctness stronger than the implementation/checks statement above depends on
recovering and reviewing the May tube proof against the current `flow_graph`
code. The active thesis section should not silently promote that proof-recovery
target into a theorem claim.

## Support Ledger

| thesis surface | claim | support source | status |
| --- | --- | --- | --- |
| Opening paragraph | The implementation uses the CH2021 flow-graph picture as a restricted exact search model and records a checked computational route and exclusions, not a replacement for HK/QP or a full CH2021 proof. | `thesis/generalized-reeb-orbits-polytopes.tex` CH2021 background; flow-graph README scope and decisions. | Supported as exposition/caveat. |
| Implemented model | States are oriented two-faces, transitions flow along a facet, primitive tubes store start set, affine map, and action, and closed words are fixed-point problems. | `crates/symplectic/src/algorithms/flow_graph/README.md` Algorithm Contract; `exact_tube.rs`; `f64_tube_search.rs`. | Supported by implementation contract. |
| Supported exact inputs | Exact rational dual normals, matching matrices, pre-established bounded-irredundant data, nonzero exact `omega_0` on nonempty two-face transitions, nonnegative action threshold, no positive singular fixed set. | `ExactFlatTubeInput`; `validate_exact_input`; `search_closed_orbits_exact`; README target input and path-specific outcome contract. | Implementation-support claim with explicit caveat that arbitrary raw bounded-irredundant validation is not complete. |
| HKO/product exclusions | HKO and Lagrangian-product zero-`omega_0` fixtures are rejected or excluded. | Exact rejection tests in `exact_search.rs`; f64 rejection tests in `tests_e2e_prediction.rs`. | Supported by tests after this packet. |
| Exact search behavior | Search enumerates transition-pruned simple closed words, resolves each word exactly, filters exact positive output by strict segment times, reports the minimum positive action, and retains words up to threshold. | `search_closed_orbits_exact`; `resolve_closed_word_exact_with_action_cutoff`; `all_segment_times_are_positive`; proposition-style tests in `exact_tube.rs`. | Supported as implementation behavior, not as a proof of the full capacity theorem. |
| Retained-word convention | Retained words are accepted exact flow-graph words after the strict segment-time filter and are not identified with QP retained words. | README role of HK2017/QP comparison; `assert_exact_search_supports_p1_p2` compares retained words with full exact flow-graph resolution. | Supported and caveated. |
| Checks paragraph | F5/F6 generated examples compare exact flow-graph scalar capacity with certified HK/QP scalar capacity; F7 checks are slower ignored regression checks. | `exact_accepted_f5_polytope_supports_p1_p2`; `exact_accepted_f6_polytope_supports_p1_p2`; ignored F7 tests in `exact_tube.rs`; README Evidence Ledger. | Supported by implementation checks. |
| f64 boundary | f64 diagnostics and `capacity_f64` are not exact certificates by themselves; direct f64 positive words remain f64 outputs, while f64 error words receive exact closed-word status only when exact closed-tube resolution succeeds. | `f64_tube_search.rs` module docs; README path-specific outcome contract; tests resolving f64 closed-word errors exactly. | Supported and caveated. |
| Limitations paragraph | No universal solver, no CH2021 rotation pruning, no full CH2021 proof, no HKO/product degeneracy support. | README explicit non-goals, decisions, evidence limits, dependency ledger. | Supported as caveat. |

## Review Gates

- Thesis prose must read as publication text, not as a README, task note, or
  implementation log.
- Each implementation-support claim in the thesis section must appear in the
  support ledger above.
- The section must not use HK/QP as a retained-word oracle.
- The section must not imply f64 output proves exact capacity values.
- The section must not claim support for HKO or Lagrangian products.
- The section must not hide the bounded-irredundant validation caveat.

## Source Pointers

- Algorithm/control: `crates/symplectic/src/algorithms/flow_graph/README.md`
- Exact search: `crates/symplectic/src/algorithms/flow_graph/exact_search.rs`
- Exact closed words/tubes: `crates/symplectic/src/algorithms/flow_graph/exact_tube.rs`
- f64 diagnostics/fallback: `crates/symplectic/src/algorithms/flow_graph/f64_tube_search.rs`
- f64/rejection tests: `crates/symplectic/src/algorithms/flow_graph/tests_e2e_prediction.rs`
- Experiments: `experiments/dev-flow-graph/README.md`
- CH2021 background in thesis: `thesis/generalized-reeb-orbits-polytopes.tex`
- Paper source: `papers/ch2021/`

## Legacy Proof Surface Pointers

The May 2026 tube/flow-graph proof material is not lost, but it is not active
source truth. `research/tube-algorithm-raw-jorn-2026-05-04.md` preserves the
raw Jörn note: tube definition, affine primitive maps, tube composition, action
cutoff, closed fixed-point solving, and finite simple-word enumeration.

Deleted history contains a larger formal surface:

- latest inspected formal version:
  `git show 25dd8b9acb8aeaaa6aa3abd80fc6d95db00c4747:formal/tube-algorithm.tex`;
- related implementation predecessor:
  `git show 0ef7ab86f4685e574929c27777ab3030d12a3ba0:crates/symplectic/src/algorithms/tube/mod.rs`;
- cleanup/import commit:
  `69b3a50afa148c12bab18db2503b511b79ae4977`.

That formal file was explicitly marked agent-written and unverified. It states
conditional results such as `prop:tube-search-correctness-finite-orbit-regular`
and `cor:tube-capacity-conditional`, with hypotheses including pairwise
nonzero `omega_0`, dual-vertex general position, and finite-orbit regularity.
Use it as recovery material to audit and migrate proof arguments, not as a
direct thesis source.

## Proof Recovery Audit Snapshot

The recovered proof distinguishes the closed polygonal search domain from the
strict tube/orbit object:

- `def:tube-data` uses `tau_r >= 0` for the closed search domain;
- the same definition says a point belongs to the represented tube exactly when
  all segment times `tau_r` are positive;
- `alg:tube-exhaustive-simple-word-search` records a fixed point only after
  reconstructing segment times and checking that they are all positive;
- `prop:tube-search-correctness-finite-orbit-regular` relies on that positive
  segment-time filter.

Current exact code matches the closed-domain side: `primitive_tube` adds
nonnegative-time halfspaces, `ExactPolygon::contains` uses closed membership,
and `solve_closed_tube` solves exact fixed points. The 2026-06-21 repair also
matches the strict-output side for nonsingular fixed points: before returning
`PositiveOrbit`, `exact_tube.rs` reconstructs exact segment times from the
fixed start point and cyclic word and requires every segment time to be
strictly positive. Positive total action alone is no longer enough for exact
positive-orbit output.

Remaining proof-recovery work is the hypothesis map, not this strict-time
filter: compare pairwise nonzero `omega_0` in the recovered proof with the
current local/geometric nonzero-`omega_0` condition, dual-vertex general
position with the current fixture/data-generation assumptions,
finite-orbit-regular singular handling with `UnsupportedPositiveSingular`,
positive transition signs with the current transition-matrix orientation, and
simple-word enumeration with `for_each_sigma_pruned_by_transition`.
