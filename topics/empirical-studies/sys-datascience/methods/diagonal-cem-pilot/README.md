# Bounded diagonal CEM versus IID pilot

This packet retains a completed numerical `5x5` product search comparison,
separate from the unexecuted full IID/local/CEM programme in
[the July prototype](../equal-budget-product-search/README.md). The
[supported-v2 summary](artifacts-supported-v2/analysis.json) records 1,344
successful unique evaluations and 1,536 charged arm memberships across three
seed blocks. Diagonal CEM improved both the best ratio and the distinct top-eight
median in each block; no target exceeded one. Three finite blocks do not
establish population superiority or a threshold-directed proposer.

| Seed | IID best `sys` | CEM best `sys` | CEM minus IID best | CEM minus IID top-eight median |
| --- | ---: | ---: | ---: | ---: |
| 202609140301 | 0.759574 | 0.798513 | 0.038938 | 0.053459 |
| 202609140302 | 0.810904 | 0.831348 | 0.020444 | 0.022546 |
| 202609140303 | 0.760068 | 0.855655 | 0.095587 | 0.091432 |

## Design and numerical boundary

The [frozen config](artifacts-supported-v2/frozen-config.json) specifies a
population of 64, 16 elites, four generations including the shared initial
population, smoothing 0.5 and variance floor fraction 0.05. Each arm is charged
256 evaluations per seed. The shared initial 64 bodies are evaluated once and
charged to both arms: 448 unique requests versus 512 memberships per seed.
CEM fits a diagonal Gaussian in the product chart's 17 continuous coordinates;
its phase coordinate uses circular wrapping. The IID arm supplies independent
base-law generations; adaptive generations depend on preceding targets.

The [producer](src/main.rs) reuses the fixed `5x5` chart and CEM code from the
prototype. Each generation has a 640-construction-attempt cap and target-free
admission checks matched to the evaluator's actual geometry/size policies.
A failed generation or target row stops the run without an update. Failed
construction attempts are recorded, not replaced invisibly. The supported run
records 4,407 construction attempts and 3,063 rejections.

[current-body-evaluator](../current-body-evaluator/README.md) evaluates the
stored binary64 dual coordinates through the structural product capacity route.
Capacity bounds and exact product capacities are retained, but volume is
numerical; hence `sys` is numerical, not a certified ratio enclosure. All
successful stored results in this summary use the same evaluator identity.

The earlier v1 run stopped after 256 requests. The v2 config explicitly records
that prior target exposure, mechanically revised admission and retained seeds
and constants. Treat supported-v2 as a bounded engineering/search comparison,
not a pristine first-exposure confirmatory experiment. `artifacts/` and
`source-v1/` preserve the original attempt; `artifacts-supported-v2/` owns the
completed result. Do not combine their calls as an independent replication.

## Retention, cost, and reproduction

The supported packet retains every input, target result, generation request,
construction log, update state and per-batch cost, plus frozen source/binary
hashes, `run-status.json`, the compact summary and best-so-far curves. Observed
whole-run wall time was about 215.8 seconds; generation-process time about 182.2
seconds and unique request wall time about 26.8 seconds. These clocks differ;
charged per-arm clocks include the shared initialization twice and are not
unique physical cost. No concurrent throughput claim follows.

[analyze.py](analyze.py) makes no target calls but checks the frozen current
source **and executable** hashes before writing the retained summary. Its
no-argument execution is therefore not a lightweight portable rerun: the
historic `.git/codex/...` binaries and exact source bytes must exist and match,
and it overwrites summary/curve files. [run.py](run.py) launches target calls
and writes directly to `artifacts-supported-v2/` with exclusive file creation;
it is a historical runner, not a safe fresh-output CLI. Do not launch either as
part of documentation or migration verification. A deliberate fresh experiment
needs new paths, an actual evaluator receipt, an agreed budget and its own
frozen identity. Inspect retained JSONL/summary bytes for ordinary reading.
