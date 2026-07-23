# Combinatorial Cells

Status: inactive retained exploration with bounded negative and diagnostic
results. Revive only for a named thesis or method question.

Original question: can combinatorial-boundary events explain or constrain
changes in `sys` and the selected minimizing characteristic as a polytope
crosses cell boundaries?

Current use: retained boundary-transition evidence and counterpressure against
unsupported continuity, single-boundary, convexity, and monotonicity models.

This package is the physical home of the combinatorial-boundary producers,
retained outputs, and local interpretation for boundary events and transitions
across four-dimensional polytopes.

It does not establish global convexity, continuity from sampling, a
single-boundary model, or monotonicity of repeated `sys` improvements.

Changes to shared boundary-stepping code or producer-selected sigma semantics
require reassessing this packet. Numerical-method lessons may inform other
packets through copied methods or explicit links; they do not make those
packets dependencies of this evidence.

## Start here

1. Read `Interpretation And Constraints` below for the usable conclusions.
2. Read the relevant child packet README before its source or artifacts:
   `omega-hypothesis/` produces the common polytope sample;
   `boundary-characterization/`, `cell-widths/`, `convexity/`, and
   `multiple-crossings/` produce separate evidence; and
   `gradient-discontinuity/` joins two producer packets.
3. Use `boundary-characterization/first-boundary-transition-report.md` for the
   current joined report.
4. Inspect producer code or JSON summaries for a concrete quantitative or
   reproducibility question.
5. Do not run a producer merely to test its command: the Rust binaries refresh
   tracked evidence and currently lack smoke modes.

## Rust Command Contract

- `cell-omega` is the upstream producer. It reads and updates the package-local
  canonical cache `polytopes.jsonl` and writes
  `omega-hypothesis/omega-obstacle.jsonl`.
- `cell-widths`, `cell-boundary-characterization`, `cell-convexity`, and
  `cell-multiple-crossings` read `polytopes.jsonl` and write tracked evidence
  JSONL files beside each binary.
- `gradient-discontinuity/analyze.py` is an analyzer, not a Rust producer
  binary. It reads boundary-characterization and cell-width outputs and writes
  interpretive figures under `gradient-discontinuity/`.
- `boundary-characterization/analyze_transition_atlas.py` joins the retained
  anatomy, crossing, and gradient artifacts on their unique direction key. It
  writes a compact generated summary, exception ledger, and readable report in
  the same folder. It does not consume the repeated-crossing stress artifact.
  Regenerate it with
  `python3 experiments/combinatorial-cells/boundary-characterization/analyze_transition_atlas.py`.
- These binaries do not currently have smoke modes. Do not run them as quick
  command checks unless intentionally refreshing the tracked artifacts.
- For compile-only checks, use `cargo test -p exp-combinatorial-cells
  --all-targets` or `cargo clippy -p exp-combinatorial-cells --all-targets`.

## Current Artifact Snapshot

- `polytopes.jsonl`: 953 rows (950 random + 3 known polytopes).
- `boundary-characterization/combinatorial-boundaries-anatomy.jsonl`: 6,671
  rows.
- `boundary-characterization/combinatorial-boundaries-crossing.jsonl`: 5,633
  rows; `construction_ok_after` succeeds for 5,631.
- `multiple-crossings/combinatorial-boundaries-sweep.jsonl`: 3,812 rows.
- `cell-widths/combinatorial-boundaries-profiling.jsonl`: 66,230 rows.
- `convexity/combinatorial-boundaries-convexity.jsonl`: 19,060 rows.
- `omega-hypothesis/omega-obstacle.jsonl`: 953 rows.

The current first-boundary join and its interpretation are generated in
`boundary-characterization/first-boundary-transition-report.md`; input paths,
row and byte counts, key-coverage checks, selected-sigma equivalence
sensitivity, and detailed counts remain in
`first-boundary-transition-summary.json`. The companion exception artifact
preserves the incidence selected-best-sigma witness, symmetric gradient
anomalies, epsilon-floor/fallback checks, crossing failures, and
threshold-discordant rows. The analyzer prints a staleness warning with its
timestamp and working directory, then relies on semantic validation rather
than retained input hashes.

Older counts and conclusions from early notes must be revalidated against
current artifacts before reuse as numeric claims.

## Interpretation And Constraints

- `cell-omega` bootstrap-writes `polytopes.jsonl` and remains the upstream
  dependency for `cell-widths` and `cell-boundary-characterization`.
- `compute_step_bound_detailed` is the shared core for crossing logic across
  multiple binaries. Downstream behavior is dominated by boundary-crossing
  robustness.
- `multiple-crossings` stress-tests the stepping kernel by repeatedly consuming
  `compute_step_bound_detailed`; repeated construction failures are evidence
  about numerical robustness, not a separate theorem.
- `convexity` uses the same perturbation strategy as `cell-widths`, so contrast
  between incidence-only and transition-aware checks is a methodological
  difference, not artifact drift by itself.
- The near-Lagrangian-ridge hypothesis is retained as falsified negative
  evidence from existing runs.
- Do not assume global convexity in dual-vertex space.
- Do not infer continuity of first-boundary `sys` from samples alone.
- Do not assume a single-boundary model for multi-step behavior.
- Do not assume monotonicity for repeated `sys` improvements.
- The first-boundary atlas supports hypothesis generation about omega-sign
  crossings, changes of the cyclically canonicalized producer-selected best
  sigma, and gradient kinks. It is not mechanism evidence: the selected sigma
  does not enumerate tied minima, most omega flips do not change it, the
  incidence exception remains a required witness, and symmetric points can
  have unstable gradient representatives.
- Selected-sigma strings are audited under raw, cyclic, and
  reversal-inclusive sensitivities in the generated summary. Reversal is not
  asserted as an equivalence of oriented characteristics. The cached action gap
  belongs to the starting polytope and returned branch set, not the immediate
  pre-boundary point or every possible branch.
- Do not infer repeated-transition rates from `multiple-crossings`; its
  construction failures are informative censoring and it lacks per-step
  producer-selected best-sigma identities.

## Revive Conditions

Treat this package as inactive unless a thesis section or method row needs it.
If revived:

- refresh the quantitative baseline from current artifacts;
- compare against the historical 140-polytope values only after rechecking the
  current artifact schema;
- decide whether sweep robustness is safe before increasing run scale for
  larger `MAX_FACET_COUNT`;
- close the two blockers before using this as strong thesis evidence: formal
  continuity of `sys` at boundaries, and systematic handling of
  `multiple-crossings` failures.
