# Gradient-Ascent and Branch-Behavior Development

Status: active development package for `sys(a)` gradient ascent and the HK
branch behavior that an optimizer must handle. The Cargo package name
`exp-dev-gradient-ascent` predates the broader branch-behavior scope.

Some retained packet prose and schema fields call a target evaluation
“exact.” In those evaluators, candidate admissibility/action aggregation uses
exact binary64-rational geometry and volume is computed by exact rational
arithmetic, but both capacity and volume are ultimately rounded to f64 before
forming `sys`. The resulting `sys` is a high-confidence reference value, not
an exact real or rational output. Current source makes this boundary visible
through `exp_sys_landscape::reference::exact_volume_as_f64`; ordinary
production-style `sys` computation uses f64 volume instead.

This package is not thesis evidence by itself. Individual retained packets
state the question, evidence, result, and claim boundary they own.

## Start here

Read according to the task:

- [`CHARTER.md`](CHARTER.md) defines the package objective, research question
  model, artifact roles, and readiness conditions.
- [`branch-cartography/`](branch-cartography/README.md) is the entry point for
  local, semi-local, and effectively global perturbation behavior.
- [`local-geometry-probe/`](local-geometry-probe/README.md) is the entry point
  for finite probes, ascent traces, endpoint scans, and their reporting tools.
- [`METHOD-CANDIDATE.md`](METHOD-CANDIDATE.md) records the current ascent
  candidate. [`PROMOTION-READINESS.md`](PROMOTION-READINESS.md) records its
  evidence gaps and the decision that would be needed before promotion.
- [`optimizer-runs/`](optimizer-runs/README.md) is the clean traced runner for
  matched local-optimizer development.
  [`optimizer-comparison/`](optimizer-comparison/README.md) strictly validates
  and compares its datasets.
- For a completed comparison or diagnostic, start with that directory's
  README rather than reconstructing its purpose from the executable.

Source files and retained artifacts overrule these navigation summaries.

## Directory map

This is the exhaustive set of immediate child directories:

| Directory | Role |
| --- | --- |
| `src/` | package library code: artifact schemas, synthetic smoke support, and branch-diagnostic implementation |
| `smoke/` | synthetic package/schema check; it does not compute real `sys` values |
| [`branch-diagnostic/`](branch-diagnostic/README.md) | producer that classifies real input rows by near-active branch counts |
| [`branch-cartography/`](branch-cartography/README.md) | producer for finite perturbation and branch-visibility records |
| [`local-geometry-probe/`](local-geometry-probe/README.md) | producer for finite probes, traces, endpoint scans, audits, and its four reporting executables |
| [`adaptive-direction-ablation/`](adaptive-direction-ablation/README.md) | retained adaptive-direction comparison and screening analysis |
| [`iterative-policy-ablation/`](iterative-policy-ablation/README.md) | retained bounded step-policy comparison |
| [`literal-naive-gradient/`](literal-naive-gradient/README.md) | literal branch-gradient baseline and multi-start optimizer comparison |
| [`optimizer-score-comparison/`](optimizer-score-comparison/README.md) | selected-case ranking smoke for near-active and candidate-window scores |
| [`quotient-endpoint-diagnostic/`](quotient-endpoint-diagnostic/README.md) | retained quotient-aware derivative-free endpoint diagnostic |
| [`optimizer-runs/`](optimizer-runs/README.md) | manifest-driven full-`sys` optimizer runner and trace schemas; the clean foundation contains no retained production output |
| [`optimizer-comparison/`](optimizer-comparison/README.md) | strict runner-dataset validation and matched trajectory comparison |

The local-geometry directory owns these consumers of its output:

```text
local-geometry-probe/
|-- trace-policy-sweep/
|-- aggregate-summaries/
|-- endpoint-scan-report/
`-- run-trace-report/
```

They are reporting components, not independent evidence packets.

## Package smoke

This command checks the package shape and synthetic artifact schemas only:

```bash
cargo run -p exp-dev-gradient-ascent --bin dev-gradient-ascent-smoke -- \
  --out-dir /tmp/dev-gradient-ascent-smoke
```

It must not be cited as a method result. Real-data producers and their input
requirements are documented in their local READMEs.

The optimizer-runner plumbing smoke is documented separately in
[`optimizer-runs/README.md`](optimizer-runs/README.md). It exercises the
registered algorithm families for two charged calls on a checked-in fixture;
it is likewise not method evidence.

## Evidence and downstream boundaries

- Generic endpoint checks here are finite and heuristic. HKO theorem-strength
  local maximality belongs to `experiments/hko-local-maximum/`.
- Reusable algorithm code should leave this development package only after the
  retained method and its required interfaces are understood.
- Fixed-`F` datascience conclusions require rerunning the relevant producer
  with the fixed method; development traces are not a substitute.
- Runtime-only measurements belong in `experiments/performance/`, and
  correctness/regression checks belong in `experiments/verification/`, when
  those concerns should move independently of method development.
