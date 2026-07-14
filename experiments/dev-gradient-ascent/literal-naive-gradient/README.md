# Literal Naive Branch-Gradient Ascent

This owner preserves the literal baseline

```text
sigma = deterministic currently minimizing admissible branch
da = eta * grad_a sys_sigma(a)
a = a + da
```

The update is unconditional. The producer does not normalize the gradient,
project the state, construct a near-active set, use a maximin or negative
direction, line-search, reject a decrease, or stop for small improvement. It
records full exact `sys` after every update and tracks best-so-far only for
analysis. A trajectory stops early only if the raw updated dual vertices do not
define a valid state or the exact state/branch-gradient computation fails.

## Discussion entry point

Start with
[`artifacts/evaluation/DISCUSSION.md`](artifacts/evaluation/DISCUSSION.md), then
use the generated
[`artifacts/evaluation/analysis.json`](artifacts/evaluation/analysis.json) for
exact denominators and per-trajectory values. The discussion report is
generated from validated raw trajectories by `analyze_multistart.py`; it is not
a second hand-maintained metric table.

The evaluation sample is the first six `F=6` rows in canonical
`experiments/sys-datascience/produce/random.jsonl` source order after excluding
the already-observed `random_F6_s0_1`. The generator uses seed `42` and height
interval `[0.8,1.2]`. Selection uses neither initial `sys` nor optimizer
outcomes. Every selected start receives every retained learning rate. This is a
small descriptive sample, not a population-wide estimate.

The human-facing investigation figures are:

- [`figures/evaluation-paired-outcomes.png`](figures/evaluation-paired-outcomes.png):
  paired start-by-rate best gain, final regret, and invalidity;
- [`figures/evaluation-prefix-retention.png`](figures/evaluation-prefix-retention.png):
  8/20-iteration class disagreement, invalidity, and best-state retention;
- [`figures/evaluation-selected-trajectories.png`](figures/evaluation-selected-trajectories.png):
  the motivating diagnostic and post-hoc labeled evaluation examples exposing
  late recovery, final regret, and invalidity.

PDF versions sit beside each PNG. The motivating `random_F6_s0_1` start is
shown only as a diagnostic; it is not counted as new evaluation evidence.

## Motivating single-start diagnostic

The retained packet runs 100 updates from the ordinary six-facet
`3daddfde...104e` start at learning rates
`1e-5,1e-4,1e-3,1e-2,1e-1,1`. Inspect
[`artifacts/summary.json`](artifacts/summary.json) for the compact trajectory
comparison and the JSONL files for every raw update. In this single-start
packet, every learning rate below `1` remained valid for all 100 updates and
found a new best after iteration 20; `eta=1` made the first updated geometry
invalid. The viable trajectories include frequent decreases and branch
switches, so final-state and best-so-far behavior must not be conflated.

[`figures/iteration-vs-sys.png`](figures/iteration-vs-sys.png) plots both raw
full-`sys` and best-so-far trajectories. Regenerate it with
`uv run --script experiments/dev-gradient-ascent/literal-naive-gradient/plot.py`.

This establishes that eight-step prefixes cannot evaluate the literal baseline
on this start. It does not select a general learning rate, establish population
behavior, or support a local-maximality claim. The multi-start evaluation above
tests whether its qualitative behavior transfers to an unselected source
prefix.

## Regeneration

From the repository root:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-literal-naive-gradient -- \
  --polytope-table experiments/dev-sys-prediction/facet-scale-baseline-error/polytope-panel.jsonl \
  --out-dir experiments/dev-gradient-ascent/literal-naive-gradient/artifacts \
  --etas 1e-5,1e-4,1e-3,1e-2,1e-1,1 \
  --updates 100
```

Expected complete trajectories contain 101 rows: the initial state plus 100
updates. The `eta=1` failure trajectory contains the initial row and its first
failed update. `run-provenance.json` records source, input, implementation, and
command identity.

Regenerate the paired evaluation from the repository root after checking out
the canonical LFS source:

```bash
git lfs pull --include='experiments/sys-datascience/produce/random.jsonl,experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/trajectory-*.jsonl'

cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-literal-naive-gradient -- \
  --polytope-table experiments/sys-datascience/produce/random.jsonl \
  --facet-count 6 \
  --start-count 6 \
  --exclude-start-ids random_F6_s0_1 \
  --out-dir experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/evaluation \
  --etas 1e-5,1e-4,1e-3,1e-2,1e-1,1 \
  --updates 100 \
  --parallelism 8

uv run --script \
  experiments/dev-gradient-ascent/literal-naive-gradient/analyze_multistart.py
```

Parallelism schedules independent start/rate trajectories only. Each trajectory
remains deterministic and applies the same literal update. The analyzer fails
if paired coverage, raw row counts, source identity, update identity, summary
agreement, or the fixed six-rate/100-update contract disagrees.

The 8/20-prefix practical classification uses a fixed operational threshold: a
best-so-far gain of at least `1%` of initial `sys`. It was chosen before the full
producer execution but was not independently preregistered; the resulting
counts are descriptive. `analysis.json` separately records whether any later
improvement occurred, so the threshold does not hide small late gains. Invalid
trajectories are censored from final-regret
denominators and remain visible in the raw artifacts and figures. For an
invalid trajectory, the producer's legacy `summary.json` field `final_sys`
means the last valid pre-failure state, not a valid 100-update endpoint. The
generated `analysis.json` therefore sets evaluative `final_sys` to null and
preserves the producer value as `last_valid_sys`.

## Near-term optimizer suite

`optimizer_suite.rs` is the shared exact-state harness for the retained
literal proposal and four small controls: invalidity-only dyadic safeguarding,
strict monotone dyadic backtracking, near-active maximin ascent, and a
derivative-free positive-spanning poll on the arbitrary first-dual-vertex
four-coordinate slice (not the full ambient or quotient space). Every target proposal is logged and counted, including invalid,
rejected, and poll candidates. Safeguarding retries begin at the nominal rate
on every update; monotone backtracking has a declared twenty-halving safety
bound and reports a method stall when it is exhausted. Maximin and polling use
an explicit adaptive radius and stop on a shrunken radius, never a local
maximum claim. The maximin `1e-3` near-active window selects branches only; it
is not an acceptance or stopping threshold, and its sensitivity is untested.

The cheap common smoke is regenerated with:

```bash
cargo run --release -p exp-dev-gradient-ascent --bin dev-gradient-ascent-optimizer-suite -- \
  --smoke --out-dir experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/smoke \
  --policies invalidity_only,monotone,maximin,poll \
  --facet-count 6 --start-count 1 --exclude-start-ids random_F6_s0_1
```

The frozen six-start/six-rate safeguard runs and the smaller maximin/poll
panel are stored under `artifacts/suite-*`. Generate the compact comparison
and investigation figures with `uv run --script analyze_suite.py`. These are
descriptive paired evidence only: no policy run establishes convergence,
local maximality, or population-wide behavior. A basic nearby-gradient bundle
is intentionally omitted; the shared state/target interface is sufficient for
adding one later, but this packet does not pay for an unvalidated fifth policy.
