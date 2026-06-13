# Numerics Audit

This experiment asks one question across multiple objects and sigma contexts:

> What are the empirical errors and predicate disagreements for numerical
> quantities used by the KKT implementations?

Use this package for numerical analysis whose methodology should improve
together across algorithms or contexts. Algorithm-local numerical diagnostics
can stay in `experiments/dev-<algo>/` while they are still deciding the
algorithm design, supported cases, or failure taxonomy. For example,
flow-graph f64/exact checks belong in `experiments/dev-flow-graph/` while they
are flow-graph design triage, and belong here once they become a reusable
f64/exact audit surface.

The durable evidence artifact is `events.jsonl`. Each row is a structured event.
The central row types are:

- `observation`: one numeric variable at one algorithm/context site.
- `predicate_observation`: one trinary f64 predicate compared with a binary
  oracle when an oracle is available.
- `context_started`, `context_finished`, `run_started`, `run_finished`: run
  boundaries and provenance. Context rows include `input_pair_kind`, the
  provenance of the audited `(P_exact, P_f64)` input pair.

Generated outputs should normally go under `/tmp`; they are review artifacts,
not durable source state.

## Commands

Run a smoke audit:

```bash
cargo run -p exp-numerics --release --bin audit-numerical-errors -- \
  --mode smoke \
  --out-dir /tmp/numerics-audit-smoke
```

Run the evidence audit:

```bash
cargo run -p exp-numerics --release --bin audit-numerical-errors -- \
  --mode evidence \
  --out-dir /tmp/numerics-audit-evidence
```

Summarize either run:

```bash
python3 experiments/numerics/scripts/summarize_observations.py \
  /tmp/numerics-audit-evidence
```

The binary prints the output directory. It writes `events.jsonl`. The summary
script writes:

- `processed/numeric_summary.csv`
- `processed/predicate_summary.csv`
- `report.md`

## Modes

- `smoke`: one simplex sigma context chosen to exercise exact-oracle numeric
  observations and predicate comparisons quickly.
- `evidence`: the smoke context plus a hypercube context and two HKO pentagon
  binary64-input diagnostics. The HKO rows audit exact arithmetic on the
  rational values represented by the stored f64 fixture, not the algebraic HKO
  object.

Changing the object bank, sigma bank, or sampling policy is a change to the
experiment. Do it in code so the evidence-producing command remains
reproducible.

## Object And Context Bank

The current bank uses `symplectic::geom::known_polytopes`.

- `simplex`, sigma `[0, 2, 1, 3, 4]`, policy `smoke_known_winner`
- `hypercube`, sigma `[0, 4, 1, 5]`, policy `known_winner`
- `hko_pentagon`, sigma `[1, 8, 7, 3, 4, 5, 9]`, policy
  `hko_selected_winner`
- `hko_pentagon`, sigma `[1, 7, 2, 8, 4, 6, 5]`, policy
  `hko_rank_deficient_diagnostic`

Each context is audited once in a deterministic single-threaded run.

Input-pair provenance is explicit:

- `rational_source_to_f64`: `P_exact` is the rational source fixture and
  `P_f64` is its f64 conversion.
- `binary64_input_to_exact`: `P_f64` is the stored f64 fixture and `P_exact` is
  the exact rational cast of those binary64 values.

## Algorithms And Variables

The current audit covers:

- `matrix_assembly`: singular values of the equality-constraint matrix and
  eigenvalues of the Hessian block.
- `exact_kkt_oracle`: exact feasibility status for the sigma context.
- `projection_kkt`: `q`, beta components, margin, residual norm, and the
  `beta_positive` predicate.
- `saddle_kkt`: corrected `q`, beta components, a-posteriori q bound, inertia
  diagnostics, and the `beta_positive` predicate.

Rows with `oracle_kind` compare f64 output with an oracle. Rows without
`oracle_kind` are diagnostics retained for conditioning and solver-context
interpretation.

## Oracle Policy

- `exact_rational`: exact rational arithmetic over rational fixture input.
- `exact_binary64_input`: exact rational arithmetic on the rational values
  represented by the stored f64 input coordinates.
- `mathematical_identity`: a zero reference for implementation residuals that
  should vanish in exact arithmetic.

The experiment records the oracle kind per row. Do not collapse those kinds when
interpreting a report.

## Interpretation Boundary

This experiment supports empirical error-audit claims for the emitted context
bank. It does not certify all possible sigma nodes or all possible polytopes. It
also does not validate finite-difference gradients against analytic gradients;
that question belongs outside this replacement unless it is reframed as
exact-or-high-precision versus f64 for a retained numerical variable.

The HKO pentagon rows are `exact_binary64_input` diagnostics. They are valid for
the same-input question "what does exact arithmetic say about the stored f64
fixture values?", but they are not `exact_algebraic_hko` evidence. A true
algebraic HKO oracle is not implemented here.

The old packet-style numerics tree was intentionally deleted in this
replacement. Historical artifacts remain available through git history.
