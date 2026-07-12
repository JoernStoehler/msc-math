# Independent Review: Iterative Step-Policy Ablation

Reviewed after retention commit `c416ca44`, against the pre-run packet at
`83710107` and the packet-history range `83710107..c416ca44`. This is a
provenance/artifact review, not a capacity rerun.

## Acceptance checks

- **Frozen question and rule — satisfied.** The predeclared two-start strict
  gate is in [`README.md`](README.md) at `83710107`: select dyadic only when it
  improves exact-evaluation efficiency on both starts and introduces no new
  correctness or numerical failure. The retained README preserves the same
  rule, full command, policy order, candidate-window direction model, eight
  evaluation cap, and four-proposal-per-iteration cap. Producer enforcement is
  in [`main.rs`](../local-geometry-probe/main.rs).
- **Source and input identity — satisfied.**
  [`run-provenance.json`](artifacts/run-provenance.json) records source revision
  `ce8c9799`, a clean worktree-diff hash, and BLAKE3 hashes for the producer,
  branch diagnostic, and polytope panel. Fresh BLAKE3 checks match all four
  recorded values; the producer and both inputs are unchanged between that
  revision and `c416ca44`. The provenance file's BLAKE3 is
  `1e4be439ea8daf26c3f14a845bdf614852b13f1cced90ef452f0ed2eb0f00533`, matching
  both [`compute-budget-report.json`](artifacts/compute-budget-report.json) and
  [`summary.json`](artifacts/summary.json).
- **Retained-file identity — satisfied.** The three LFS JSONL artifacts match
  their `c416ca44` pointers: fixture selection
  `1ef621ea02a200fa3312b83377175f8a2eca39e8cde85c3c0f509832cd7824cc`, outcomes
  `73658fafa349ac643a17643ba4eacaef262370a7f2659d093c7c87c2988d89d8`, and
  proposals `8d8b92fdb0b105e218ffb68bf72b139aaf9ec6eb75acc3bee011b6a23d2aa981`.
- **Outcomes, budget, and costs — satisfied.**
  [`iterative-policy-outcomes.jsonl`](artifacts/iterative-policy-outcomes.jsonl)
  contains exactly six outcomes: three policies on each of the two selected
  `narrow_gap` starts. Every start-policy receives the same eight-evaluation
  cap. Five consume eight; the rank-0 boundary-scaled trajectory validly stops
  after four with `no_observed_improving_move`. The six outcome counts sum to
  44, equal to the 44 proposal rows and the report total. Their base and target
  orbit costs sum respectively to 151397 and 241219, matching the compute
  report.
- **Proposal accounting and failures — satisfied.**
  [`iterative-policy-proposals.jsonl`](artifacts/iterative-policy-proposals.jsonl)
  has 44 rows, all `status: "ok"`, with finite observed deltas. The producer
  increments the exact-evaluation counter immediately after every proposal, so
  each retained row is charged. Per outcome, proposal-row counts equal recorded
  exact evaluations; selected-row counts equal accepted moves; selected final
  targets equal recorded final `sys`. No iteration exceeds four proposals.
- **Arithmetic and gate — satisfied.** For all six outcomes,
  `initial_sys + observed_gain_sys = final_sys` to a tolerance of `1e-12`.
  Dyadic exceeds fixed on both starts, but boundary-scaled has no improving move
  on rank 0 and exceeds dyadic on rank 1. Thus the strict selection gate fails
  exactly as the retained README states; no correctness or numerical failure is
  recorded.

## Bounded disposition

Accepted for the stated optimizer-development use: retain dyadic as a robust
improvement over fixed on this two-start panel, while retaining the observed
regime split. It does **not** support selecting a default policy, claiming
dominance over boundary scaling, or inferring endpoints or local maxima. The
README's reopen boundary is adequate: reopen only for a downstream need to
choose one default scheduler or a concrete trajectory failure that requires
distinguishing the two regimes; then use a larger frozen panel.
