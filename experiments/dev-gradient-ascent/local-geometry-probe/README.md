# Local Geometry Probe

Status: active gradient-ascent development producer and its reporting tools.
The directory contains no canonical retained run.

## Question

For fixtures selected by the
[branch degeneracy diagnostic](../branch-diagnostic/README.md), how do finite
branch-derived perturbations behave, which proposed ascent steps are actually
improving after exact recomputation, and what endpoint condition does the
implemented finite scan support?

The probe serves method development in [`../CHARTER.md`](../CHARTER.md).
[`../METHOD-CANDIDATE.md`](../METHOD-CANDIDATE.md) records the current
candidate algorithm, and
[`../PROMOTION-READINESS.md`](../PROMOTION-READINESS.md) records what must be
regenerated before a promotion decision.

## Producer

Run a branch diagnostic first, then pass its output directory:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-local-geometry-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --out-dir /tmp/dev-gradient-ascent-local-geometry-check \
  --steps 1e-3,1e-4 \
  --endpoint-steps 1e-3,1e-4,1e-5,1e-6 \
  --max-fixtures-per-label 1 \
  --trace-iterations 8 \
  --min-observed-relative-delta 1e-3
```

The normal mode writes:

- `fixture-selection.jsonl` and `local-geometry-probe.jsonl`;
- `run-trace.jsonl`;
- `endpoint-diagnostic.jsonl` and `endpoint-direction-scan.jsonl`;
- `run-provenance.json`, `compute-budget-report.json`, and `summary.json`.

Optional modes add candidate-window direction comparisons, exhaustive
direction/step ranking audits, selected-iteration state exports, or iterative
step-policy ablations. Read `--help` and the relevant retained packet README
before using those modes:

- [`../adaptive-direction-ablation/`](../adaptive-direction-ablation/README.md);
- [`../iterative-policy-ablation/`](../iterative-policy-ablation/README.md);
- [`../optimizer-score-comparison/`](../optimizer-score-comparison/README.md).

## Modes and interpretation

The stop threshold at each state is

```text
max(min_observed_delta, min_observed_relative_delta * abs(base_sys)).
```

The normal trace orders directions by their local prediction, then accepts the
first finite step whose recomputed improvement clears that threshold. Set
`--endpoint-steps` when the post-stop scan should use a different finite grid
from the trace.

Direction models:

- `near-active` scores directions from branches inside the selection
  threshold.
- `candidate-window` scores a finite step by the lower envelope over the wider
  candidate branch window, including each branch's base action gap.
- `--include-candidate-window-directions` additionally generates
  step-indexed candidate-window maximin directions. It is a direction-set
  experiment, not merely another scoring rule.

Audit modes:

- `--write-step-ranking-audit` exhaustively recomputes every generated
  direction/step pair at each traced state. It writes
  `step-ranking-audit.jsonl` and can be expensive.
- `--audit-iterations 0,4,8` audits named trace bases instead of producing the
  normal probe and endpoint files. It also writes `states.jsonl`, including
  intervening lineage states, and `audit-state-status.jsonl`, including
  requested states the trace did not reach.
- `--audit-step-policies fixed,geometric,boundary-scaled` compares schedulers
  at those frozen states without changing the trace that reached them. Exact
  target evaluations are shared in memory and rows record whether an
  evaluation was reused.
- `--audit-direction-limit` and `--audit-policy-proposal-limit` bound a
  state-local audit; they do not change the normal direction generator or
  trace policy.
- `--iterative-ablation-policies` runs bounded trajectories rather than
  state-local scheduler audits. It requires `--direction-model
  candidate-window` and `--iterative-exact-evaluation-budget`; its retained
  experiment is documented in
  [`../iterative-policy-ablation/`](../iterative-policy-ablation/README.md).

The producer removes its own known output files before reusing an output
directory, so files from normal, selective-audit, and iterative modes are not
silently mixed.

For split expensive runs, `--skip-fixtures-per-label` skips eligible fixtures
after filtering by label and threshold and sorting by label, descending input
`sys`, then `poly_id`. `fixture-selection.jsonl` records the resulting
`selection_rank_within_label`.

## Reporting tools

These nested executables consume one or more completed producer directories;
they do not recompute `sys(a)`:

- `trace-policy-sweep/` relabels existing trace and endpoint observations
  under alternative absolute and relative stop thresholds.
- `aggregate-summaries/` combines run summaries and compute budgets without
  inspecting trace rows.
- `endpoint-scan-report/` summarizes finite endpoint scans, including
  positive-but-below-threshold observations.
- `run-trace-report/` summarizes accepted/rejected directions and cases where
  recomputation disagreed with the local prediction.

Example report commands:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-trace-policy-sweep -- \
  --geometry-dir /tmp/dev-gradient-ascent-local-geometry-check

cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-aggregate-summaries -- \
  --out-dir /tmp/dev-gradient-ascent-panel \
  /tmp/dev-gradient-ascent-local-geometry-check

cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-endpoint-scan-report -- \
  /tmp/dev-gradient-ascent-local-geometry-check

cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-run-trace-report -- \
  /tmp/dev-gradient-ascent-local-geometry-check
```

## Claim and cost boundaries

The producer tests a finite direction and step set. Its endpoint scan does not
certify local maximality, branch/germ completeness, or behavior from all
starts. Reporting tools only summarize the observations present in their
inputs.

Exact recomputation and endpoint scans can take minutes for a single fixture.
Use small `/tmp` runs for plumbing checks. Put outputs under a retained packet
only when the question, panel, decision rule, and downstream claim warrant
preserving them.

`run-provenance.json` records CLI parameters, input and source identities, and
the observed paths. These identities document a run; they are not staleness
gates. Regenerate and inspect retained outputs before quoting fixture counts,
trace behavior, endpoint status, or compute cost.
