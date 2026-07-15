# Flow-Graph Proof-Risk Rows

This packet is an executable regression surface for
`formal/flow-graph-proof-risk.tex`. It writes JSONL rows keyed by `claim_id`
and `check_id`; passing rows are falsifier evidence for the current code
boundary, not proofs of the flow-graph theorem.

Run the smoke packet:

```bash
cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk
```

Run the full packet:

```bash
cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk -- --full
```

Outputs are written under this directory:

- `smoke-flow-graph-proof-risk.jsonl`
- `flow-graph-proof-risk.jsonl`

## Row Groups

- `exact_fg_capacity_matches_certified_hk_qp`: compares exact flow-graph
  capacity against current certified HK/QP aggregation on deterministic
  generated polytopes.
- `retained_words_match_direct_exact_resolver`: compares search-retained words
  against direct exact closed-word resolution under the current flow-graph
  convention.
- `cutoff_enabled_matches_disabled`: checks that exact action-cutoff execution
  preserves the disabled-cutoff output on selected rows. The counters record
  how many later words received an active cutoff and how many nonempty tubes
  were actually intersected with it. The selected F5/F6 fixtures may reach the
  policy without requiring an intersection; neither counter is a lower-bound
  certificate.
- `zero_omega_fixture_rejected`: checks that known zero-omega fixtures are
  rejected before exact capacity output.
- `positive_singular_word_is_typed_unsupported`: replays the structural
  hypercube word `[0,4,1,5]`. The exact resolver must return
  `unsupported_positive_singular` with `singular_all_points` and action range
  `4..4`, rather than accepting the word as a capacity output.
- `length_three_word_is_zero_time_no_orbit`: replays the deterministic
  generated F5 word `[0,2,4]`. The exact resolver must return a zero-action
  no-orbit outcome with `length_three_zero_time`, rather than treating this
  common structural singularity as an input-level rejection.
- `zero_action_word_is_typed_no_orbit` and
  `known_positive_f7_word_resolves_directly`: pin representative direct
  closed-word outcomes on the deterministic F7 attempt 31 case.

## Boundary

This packet does not prove primitive tube semantics, tube concatenation,
finite-domain completeness, or cutoff lower-bound soundness. It also does not
justify rejecting every singular fixed map: generated polytopes routinely have
length-three zero-time singular fixed lines. Positive-action singular
fixed-set rejection is tracked separately by the structural hypercube row.
An inconsistent singular fixed equation, or a singular fixed set disjoint from
the searched tube, is an exact empty-tube/no-orbit outcome. Other intersecting
singular no-orbit statuses are not theorem-facing accepted outputs unless a
future lemma covers them.
