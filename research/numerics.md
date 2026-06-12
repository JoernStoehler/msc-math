# Numerics Research Note

## Scope

`experiments/numerics/` is now a replacement numerical error-audit experiment
for KKT-capacity variables and predicates used by retained thesis claims. It is
an evidence surface, not a public certified-solver project.

The active runbook is `experiments/numerics/README.md`. Generated run outputs
normally live under `/tmp`, with raw `events.jsonl`, processed CSV summaries,
and `report.md` produced from the same event stream.

## Current State

The old packet-style tree (`algebraic-exactness`, `error-bounds`, `q-error`,
`kkt-inertia`, `unknown-predicates`, `sage-feasibility`, and the separate
`gradient` package) was deleted in the replacement. Historical artifacts remain
available through git history, but future agents should not treat those removed
paths as active experiment surfaces.

The current experiment asks one question across multiple objects and sigma
contexts:

> What are the empirical errors and predicate disagreements for numerical
> quantities used by the KKT implementations?

Current emitted row families:

- `matrix_assembly`: singular/eigenvalue diagnostics without oracle claims.
- `exact_kkt_oracle`: exact feasibility status for sigma contexts.
- `projection_kkt`: `q`, beta components, margin, residual norm, and the
  `beta_positive` predicate.
- `saddle_kkt`: corrected `q`, beta components, q error bound, inertia
  diagnostics, and the `beta_positive` predicate.

Oracle labels are row-local:

- `exact_rational`: exact rational arithmetic on rational fixture input.
- `exact_binary64_input`: exact rational arithmetic on the rational values
  represented by stored f64 input coordinates.
- `mathematical_identity`: exact-zero reference for residual-style diagnostics.

Input-pair provenance is context-local:

- `rational_source_to_f64`: `P_exact` is the rational source fixture and
  `P_f64` is its f64 conversion.
- `binary64_input_to_exact`: `P_f64` is the stored f64 fixture and `P_exact` is
  the exact rational cast of those binary64 values.

## Latest Local Evidence

On 2026-06-12, the evidence command completed locally:

```bash
cargo run -p exp-numerics --release --bin audit-numerical-errors -- \
  --mode evidence \
  --out-dir /tmp/numerics-replacement-evidence
python3 experiments/numerics/scripts/summarize_observations.py \
  /tmp/numerics-replacement-evidence
```

The generated report covered two exact-rational contexts and two HKO
binary64-input contexts. The simplex and hypercube contexts had no predicate
disagreements; largest exact-rational numeric errors were at ordinary f64
scale, with the largest observed absolute error about `1.7e-15`.

The HKO pentagon rows are valid same-input diagnostics for the stored f64
fixture values: exact arithmetic is run on the rational values represented by
those binary64 coordinates. They are not exact algebraic HKO oracle evidence.
The current run reports `beta_positive` predicate disagreements for both
projection and saddle KKT solvers on the two HKO binary64-input contexts.

## Interpretation

Supported by the active experiment:

- empirical f64-vs-exact error measurements for emitted rational fixture
  contexts;
- explicit predicate disagreement rows for emitted contexts;
- diagnostic context for conditioning and solver behavior.

Not supported by the active experiment:

- a claim that public capacity wrappers are fully certified numerical solvers;
- coverage of all sigma nodes or all polytopes;
- algebraic-exact HKO validation;
- finite-difference-vs-gradient validation.

The generic-case-first proof route in `research/numerics-error-bounds.md` is
still relevant as mathematical context, but the referenced old experiment
packet paths in that note are historical unless refreshed against the new audit
surface.
