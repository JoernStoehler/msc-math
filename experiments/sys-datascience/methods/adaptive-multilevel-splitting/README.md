# Adaptive multilevel splitting readiness smoke

## Status and decision

This is a target-free-tested, pre-run packet. No production target evaluation
has been run and no real-target artifact is tracked. Its next consumer is an
independent technical review deciding whether the exact frozen command below
may spend the 64-call readiness budget.

The smoke asks only whether this AMS-style candidate-finder policy is
operationally healthy: honest target accounting, valid chart mutations,
inspectable ancestry, enough diversity, and bounded runtime. It is not evidence
that adaptive search beats IID, a scientific negative if it fails, an invariant
MCMC kernel, or an estimate of conditional mass or rare-event probability.

## Frozen policy

`resolved-config.json` is the tracked contract. The adaptive arm constructs 16
independent valid `5 x 5` products, then runs two levels. At each level the
deterministic order is descending exact `sys` and ascending candidate ID; the
top eight survive. A seeded uniform-with-replacement assignment gives each of
eight clones a survivor parent. Each clone receives two sequential charged
valid Gaussian chart proposals. A successful proposal becomes the clone state
exactly when its `sys` is at least the level's frozen threshold. This closes at
`16 + 2 * 8 * 2 = 48` adaptive target requests. The control arm makes 16
independent IID valid target requests.

The fixed mutation standard deviations are `0.08` for each independent gap
logit, `0.04` for each independent centered log radius, and `0.08` radians for
relative phase. They are conservative pre-target choices and were not tuned on
smoke target values. The chart removes continuous gauges but deliberately does
not quotient q/p factor exchange; named q and p chart fields remain in every
target row.

Construction failures are uncharged, reasoned, and retried up to 64 times.
Every valid target request is charged before arm-private cache lookup, so a
duplicate, cache hit, failed computation, or fallback consumes budget.
Execution aborts after crossing the frozen 600-second wall-time gate.
Candidate IDs bind packet/config/source identity, parent, seed, replicate/arm,
level/clone/step or base index, and construction attempt.

## Target-free smoke

This command builds the binary it immediately runs, writes only disposable
synthetic artifacts, and then invokes the independent fail-closed verifier:

```bash
cd experiments/sys-datascience/methods/adaptive-multilevel-splitting
rm -rf /tmp/ams-readiness-synthetic
cargo build --release --locked && ./target/release/adaptive-multilevel-splitting synthetic --config resolved-config.json --artifacts /tmp/ams-readiness-synthetic
python3 analyze.py /tmp/ams-readiness-synthetic
```

The synthetic path uses cheap near-regular valid chart states and a
deterministic scalar oracle. It still enters the production adaptive/IID
resampling, Gaussian mutation, chart-construction, charging, cache, genealogy,
transition, artifact, and stop paths. It intentionally skips the expensive
exact polytope constructor and production target evaluator, so its geometry and
scores are plumbing fixtures, not research evidence.

The positive stop path is exercised without a real target by adding
`--force-synthetic-hit`; verification must then find one flushed target row, a
`stop-event.json`, and no later request.

## Reserved production smoke

Do not run this until the packet and dependencies are committed cleanly and an
independent pre-run review returns `GO`. The producer refuses production mode
when `git status --porcelain` is nonempty. The exact run path builds the current
binary before execution and requires a new artifact directory:

```bash
cd experiments/sys-datascience/methods/adaptive-multilevel-splitting
test ! -e /tmp/ams-readiness-production
cargo build --release --locked && ./target/release/adaptive-multilevel-splitting production --config resolved-config.json --artifacts /tmp/ams-readiness-production
python3 analyze.py /tmp/ams-readiness-production
```

Before any call, the manifest binds the exact config, clean Git revision,
current executable SHA-256, and packet `Cargo.lock` SHA-256. A trusted
`sys > 1` result synchronously flushes its cache and target rows, writes the
stop event, and returns without starting another request. Such an event requires
independent geometry/target validation and stopping unrelated search.

## Artifacts and gates

- `manifest.json`: exact config, packet kind, source/build identity, fixed
  budgets, explicit absence of a probability estimate, and unquotiented factor
  exchange.
- `target-evaluations.jsonl`: every charged request, candidate identity, exact
  geometry key and stable identity, cache/evaluation status, capacity, volume,
  `sys`, chart, ancestry, threshold, and target wall time.
- `cache.jsonl`: every successful miss with exact geometry, facet count, and
  returned scalars. Hits and failed misses remain visible in target rows.
- `construction-rejections.jsonl`: every uncharged rejected attempt and reason.
- `levels.jsonl`: frozen thresholds, ordered survivors, roots, and clone-parent
  assignments.
- `mutation-transitions.jsonl`: before/proposal/after state, threshold,
  acceptance, and root for every sequential proposal.
- `arm-runs.jsonl`: per-arm attempts, rejection/cache counts, distinct
  successful keys, completeness, and wall time.
- `stop-event.json`: present only after `sys > 1` flush-and-stop.

An unstopped smoke passes readiness only with exactly 48 adaptive and 16 IID
charged rows, four distinct successful keys per arm, at least two surviving
roots at every level, at least one accepted valid mutation, contiguous attempt
indices, exact cache/geometry reconciliation, and complete accounting. These
are operational gates only. The analyzer produces no arm-quality comparison or
probability claim.

## Local verification

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
python3 -m unittest -v test_analyze.py
python3 -m py_compile analyze.py test_analyze.py
git diff --check -- experiments/sys-datascience/methods/adaptive-multilevel-splitting
```
