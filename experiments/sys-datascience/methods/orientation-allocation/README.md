# Fixed orientation allocation versus fresh IID shapes

Completed 2026-09-14: all 124 unique target requests succeeded, representing
128 charged arm memberships. In all four frozen blocks, 16 IID product draws
achieved a higher maximum numerical systolic ratio than the fixed 16-request
orientation allocation on their shared first body. This is a completed finite
negative for this allocation tactic, not a claim that orientation cannot help.

| Block | IID best | Orientation best | Difference | IID charged seconds | Orientation charged seconds |
| --- | ---: | ---: | ---: | ---: | ---: |
| 3x3, 0 | .507761 | .477545 | −.030215 | .249 | 1.202 |
| 3x3, 1 | .610268 | .563753 | −.046515 | .243 | 1.099 |
| 4x4, 0 | .773536 | .529285 | −.244251 | .296 | 4.738 |
| 4x4, 1 | .636202 | .377470 | −.258732 | .310 | 5.409 |

The equal-bucket mean difference of block maxima is −.144928. Orientation
improved three of its four starting bodies, but spending the same number of
requests on fresh shapes did better in every block. The operation was also
more expensive on this evaluator. Four blocks do not support a population or
significance claim. No evaluated value exceeded one; maximum .773536.

## Frozen design and actual geometry

`src/main.rs` uses master seed 202609140201, buckets 3x3 and 4x4, two blocks
each, and 16 target-blind IID product draws per block. Every per-draw seed is
ChaCha8 seeded with BLAKE3 of the retained explicit domain/bucket/block/draw/
attempt string. The existing `random_polygon_2d` draws q then p normals and
heights [.8,1.2); exact-binary64 reconstruction checks boundedness and all facets.
Up to 128 failed construction attempts are allowed, with reasons retained.

Each orientation arm uses IID draw0, followed by a U(2) control with c=.6,s=.8,
then 14 maps from the existing compact orientation formula with theta=pi/8,pi/4
and phi=2*pi*j/7, interleaved by j. All actual matrices and transformed dual
arrays are retained. Source incidence signatures and reconstructed volumes are
checked before any targets. The entire panel is emitted only after all checks.
`artifacts/freeze.json` binds input, producer, lock and analysis identities before
evaluation. Input SHA-256 is
`823ac528d47459c5fee79b55a2d163d3207ecad2f5349876116f810037701aa2`.

General SO(4) rotations preserve Euclidean shape but can leave the Lagrangian
product class. Evaluation therefore uses the actual transformed full dual body
through the immutable `current-body-evaluator`. Dispatch was 72 product and
52 general requests. The phi=0 maps still preserve the coordinate product form,
so a product route for those maps is expected. No source label forces dispatch.

All four U(2) target controls pass |delta sys|<=1e-7, with maximum observed
3.89e-16. All orthogonality/determinant/symplectic controls and incidence checks
pass; transformed reconstructed volumes agree to the declared relative 1e-8.
Capacity midpoint acceptance uses certified relative error <=1e-10. Volume is
f64 from the same exact-binary64-derived incidence, so the reported ratio remains
numerical and is not certified solely by the capacity bounds.

## Calls and time

The shared start is evaluated once but belongs to both scientific budgets.
There are 31 unique requests per block and 32 memberships, hence 124 and 128.
Requests alternate arms within a block, with the first arm alternating across
blocks. One evaluator subprocess runs at a time under the shared `flock`; each
request has a five-second timeout and the complete command a 300-second ceiling.
No request failed, timed out or was replaced, and the full grid is complete.

Charged time includes each request's supervisor wall time plus generation/
rejection or transformation and exact reconstruction validation time. Shared
start generation/evaluation is charged to both arms. Thus table totals exceed
unique physical compute by the shared costs. Proposal timing includes the
target-free geometry validation that the adapter subsequently repeats; it is
actual work performed, not a hypothetical optimized implementation. Compilation,
panel serialization, output serialization and analysis are excluded from these
per-arm clocks; whole-process observations are recorded separately.

Measured release build: 3.19 seconds, reusing readiness's target directory with
two build jobs. Panel generation: .57 seconds GNU-time wall. Evaluation: 13.29
seconds wall, 12.57 user + .59 system seconds, 35,808 KiB peak RSS. Unique
supervisor request time sums to 12.925 seconds; unique measured proposal time
sums to .548 seconds. The adapter pins numerical thread limits to two; this is
serial request execution, not a demonstrated single-CPU contract.

`artifacts/curves.jsonl` contains every arm's charged-call/time best-so-far curve.
`time-checkpoints.json` reports the predeclared .01,.03,.1,.3,1,3,... second
checkpoints only where both arms actually ran that long. Null means no completed
candidate at the checkpoint. Stopped IID curves are not extended into fictitious
continued searches. `details.json` records orientation improvements, route mix
and numerical control residuals. The descriptive formatter `report.py` was added
after targets; it introduces no selections, thresholds or scientific criteria.

## Reproduction

Retained results must not be overwritten. New runs require fresh output paths
and their actual evaluator build receipt. The executed commands were:

```sh
cargo build --offline --release --manifest-path experiments/sys-datascience/methods/orientation-allocation/Cargo.toml --target-dir .git/codex/ds-first-wave/readiness/target -j2
/usr/bin/time -o experiments/sys-datascience/methods/orientation-allocation/artifacts/generation.time .git/codex/ds-first-wave/readiness/target/release/orientation-allocation > experiments/sys-datascience/methods/orientation-allocation/artifacts/inputs.jsonl
# The input/source/lock/analyzer hashes and timestamp were then frozen in artifacts/freeze.json.
flock /workspaces/msc-math/.git/codex/ds-evaluator.lock timeout --kill-after=2s 300s /usr/bin/time -o experiments/sys-datascience/methods/orientation-allocation/artifacts/evaluation.time python3 experiments/sys-datascience/methods/current-body-evaluator/evaluate.py --input experiments/sys-datascience/methods/orientation-allocation/artifacts/inputs.jsonl --output experiments/sys-datascience/methods/orientation-allocation/artifacts/results.jsonl --binary .git/codex/ds-first-wave/readiness/target/release/current-body-evaluator --build-receipt experiments/sys-datascience/methods/current-body-evaluator/controls/build-receipt.json --timeout-seconds 5
python3 experiments/sys-datascience/methods/orientation-allocation/analyze.py
python3 experiments/sys-datascience/methods/orientation-allocation/report.py
```

`analyze.py` checks exact membership coverage, retained input identity, every
result's payload/hash, formula/bounds/volume consistency and U(2) controls. Its
complete flag requires all 124 targets valid. It does not infer statistical
significance from this four-block exploratory experiment. The negative allocation
result can enter the AI-assisted exploration account as an actually executed
geometric search bet alongside the earlier witness-level positive orientation
effects, keeping the distinction between changing a body and allocating a budget.
