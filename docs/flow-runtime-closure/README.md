# Flow theorem/runtime closure

18 September 2026. Focused source audit and fresh tests on branch
`research/flow-runtime-closure`, based on technical plan `60f4fb0b`.
No runtime or active thesis changes. No newly identified mismatch invalidates
the selected chapter's conditional capacity claim.

## Checked sources and conclusion

The selected draft is `/tmp/msc-math-thesis-review-20260917/thesis/candidate/05-flow-graph.tex`.
The integration proposal at
`/workspaces/msc-math/.worktrees/qp-flow-integration-patch/docs/qp-flow-integration/`
changes only its overflowing display; theorem/runtime statements are the same.
Formal owner: `formal/flow-graph-real-algorithm.tex`. Runtime owners:
`crates/symplectic/src/algorithms/flow_graph/{exact_search,exact_tube}.rs`.

| Obligation | Formal/draft meaning | Runtime correspondence |
|---|---|---|
| Valid geometry | Bounded full-dimensional normalized irredundant presentation, origin interior | Caller contract. `exact_tube::validate_exact_input` checks matrix shapes only; it does not prove geometry or supplied incidence/sign correctness. Draft says this explicitly. |
| Transition signs | Nonempty facet pairs have nonzero pairing; a physical transition has positive sign | `exact_search::validate_no_geometric_zero_omega_transition` rejects geometric zero pairings. Shared adjacency builder permits `>=0`, but validation removes zeros first. `primitive_tube` requires both positive signs. No claim of sufficiency for physical passage is made by adjacency. |
| Normalization | R_i=2J_0 a_i, action equals elapsed time | `j_times` uses (-a2,-a3,a0,a1), `primitive_tube` multiplies by two. Time denominator and affine formula agree exactly. |
| Primitive semantics | Both endpoints lie in K; nonnegative time | `primitive_tube` intersects the start face with the affine pullback of end face and the nonnegative-time halfspace. Convexity supplies the segment. |
| Composition/emptiness | Intersect P1 with Phi1 inverse(P2), compose map and add action | `intersect_tubes` and balanced `build_tube` implement this. An empty subtube returns before fixed-point solving. Bounded polygon feasibility retains point/segment domains, not just positive-area polygons. |
| Finite enumeration | Every simple directed cyclic word once | HK enumeration's `SimpleDirectedCyclesCanonical` starts at minimal label, never repeats vertices and checks closure. Finite depth <=F. Search has no CH rotation pruning. |
| Short words | Independence of every <=4 normals excludes positive closure | Runtime enumerates/rank-checks all such subsets, then skips all words <=4 before tube construction. Formal algorithm checks later, but the early skip is justified by the same lemma. |
| Nonsingular fixed point | Solve, check membership and every segment time >0 | `solve_closed_tube` exact inverse, closed membership, positive total action and `all_segment_times_are_positive`. Non-strict boundary fixed points are not recorded as orbits. |
| Singular equations | Idealized theorem excludes nonempty long-word singular return maps | Runtime additionally accepts inconsistent equations or fixed sets missing the tube as empty. Other long-word intersecting singular fixed sets are rejected, including nonpositive singular diagnostic outcomes. Direct length-three resolver's zero-time lemma is not used by exhaustive search because short words are already skipped. |
| Action cutoff | Preserve fixed points below known-best + nonnegative threshold | `restrict_tube_to_action_cutoff` intersects a completed tube with A<=B before solving. B is based only on an already confirmed strict orbit. Formal cutoff lemmas show final-minimum + threshold candidates survive. Final search filter enforces that window. |
| Capacity bridge | Simple minimizer is among these words; all recorded orbits are valid | Same bridge as formal theorem; conditional on stated geometry and regularity. QP scalar comparisons are finite implementation checks, not a proof of this bridge. |

The return object contains word/action pairs, not all trajectory coordinates.
Under finite-orbit regularity each relevant word has at most one fixed point.
A general promise to enumerate all nonsimple or singular orbit families would
be false and is not made by the selected draft.

## Important boundary distinctions

Runtime success is **not a test of finite-orbit regularity**. An empty singular
fixed set can be harmless; a singular domain entirely above the active action
cutoff can be pruned before its singularity is examined. This does not defeat
the cutoff argument for the retained window. It does mean the runtime and
idealized algorithm accept different sets of inputs. The selected chapter
keeps theorem hypotheses separate from runtime behavior and need not weaken
its conditional theorem.

The formal section's final `rem:fg-real-missing-work` mentions a classifier
that accepts nonpositive singular outcomes. That is accurate for the direct
closed-word resolver, but not for exhaustive search: `exact_search.rs` rejects
long-word intersecting singular zero-action outcomes. Do not copy that remark
into the thesis as a description of `capacity_exact`.

Presentation-chamber genericity has the same precise scope in the formal and
selected draft statements: fixed F, ordered normalized genuine facets, open
presentation chambers, and finitely many algebraic nonvanishing conditions.
This audit matched scope; it did not reprove the determinant specialization
argument or import CH2021's different genericity conjecture. Structured polygon
products violate short-word independence and remain outside this search class.

## Fresh verification

`cargo test -p symplectic --release --lib flow_graph`:
**39 passed, 0 failed, 4 ignored**, 34.61s test runtime after 58.55s compilation.
The ignored exhaustive F7 tests were not rerun. Default suite includes F5/F6
scalar QP agreement, retained-word completeness by full flow resolution,
cutoff-enabled versus disabled F6, singular/empty/boundary fixtures, rational
polygon point/segment feasibility, primitive geometry, and balanced versus
left-associative composition. No full producer or artifact replacement ran.

`python3 docs/flow-runtime-closure/check-example.py`: **PASS**. This independent
Fraction calculation verifies the retained six-facet JSON witness:

- every breakpoint lies in K and on its active facet;
- the five positive exact times move along 2J0a_i and close exactly;
- time sum equals the retained rational action;
- the affine return map fixes the construction-chart point;
- its affine action function yields the same action;
- every displayed six-decimal return-map/action/fixed-point/time value agrees
  with rounding the retained rational values.

Action is 39.35654212420112. The chart point is approximately
(0.79185572945, 0.31724345398), **not** the plotting-chart point
(1.58282629513, 0.32878065564). The draft correctly distinguishes the two.
The script checks a retained witness, not a newly generated random fixture or
a proof that this particular word realizes capacity; the example explicitly
requires comparison with other words for capacity.

## What remains

- Integrate the selected chapter and figure with the already prepared QP/flow
  patch after coordination review. Preserve explicit caller assumptions.
- Retain the prepared simple-minimizer dependency and surrounding conventions
  when changing preliminaries; this audit did not reprove that imported result.
- Perform reader-facing/visual review of the integrated pages. Passing tests
  and agreeing decimal values do not establish acceptable exposition.
- No general-input validator, singular-capacity extension, product support,
  algebraic-number backend, benchmark claim, or broader proof audit is required
  by the present chapter or supplied by this work.
