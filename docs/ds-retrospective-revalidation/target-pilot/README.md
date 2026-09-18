# Current-target pilot: all 36 bodies completed

2026-09-18. Frozen selection: two lexicographically smallest poly_id in each of
18 strata (eight generic facet counts, ten product buckets), before new target
exposure. Inputs preserve source geometry and old capacity/volume/sys. No cache.

All 36 requests completed successfully, no errors/timeouts. Identity coverage is
checked against the frozen input. Request wall time summed to 0.4984 seconds;
outer process 0.63 seconds; fresh release build 86 seconds. Maximum request 68.6 ms.

Maximum absolute relative deltas from retained numerical values:

- Capacity: 9.36e-16.
- Volume: 1.12e-15.
- Systolic ratio: 2.10e-15.

Thirteen retained capacities are outside new narrow outward intervals despite
sub-femtoscale relative differences. This is expected to be possible with
historical rounded scalars and extremely narrow exact-product intervals; it is
**not** reported as interval inclusion. Exact old/new values and the inclusion
flag are retained per row. No mismatches were suppressed or converted to panic.

The current code supplies certified capacity bounds for the exact binary64
body. Volume uses f64 arithmetic on exact-derived incidence, so neither volume
nor sys is claimed as certified. Rational/binary64 source identity was already
verified for the entire source population. This pilot validates only 36 bodies.

## Measured full-population cost projection

Weighting each two-body stratum's mean request time by its 512 generic or 1,024 product
population gives **192.1 seconds serial request time**, plus orchestration/output
and final comparison. This is a point projection, not a bound: two observations
per stratum do not measure rare exact fallback or timeout tails. Compilation is
already done. The next useful action is a full immutable-input reevaluation with
per-request 5 s limits and an outer 15 min cap, flushing all statuses and preserving
old/new values. Do not silently drop timeouts or claim completed coverage from
partial output. A full run was deliberately not started in this assignment.

## Reproduce

The imported current-body-evaluator packet is copied unchanged from main's
previously untracked packet. Its source/binary receipt accompanies every result.
Build source hashes are frozen; later receipt checks reject changed sources.

```sh
python3 experiments/sys-datascience/methods/current-body-evaluator/build.py \
 --target-dir /workspaces/msc-math/target/ds-retrospective-evaluator \
 --receipt /tmp/new-ds-evaluator-build.json
timeout 240s python3 experiments/sys-datascience/methods/current-body-evaluator/evaluate.py \
 --input docs/ds-retrospective-revalidation/pilot-inputs.jsonl \
 --output /tmp/new-ds-target-pilot.jsonl \
 --binary /workspaces/msc-math/target/ds-retrospective-evaluator/release/current-body-evaluator \
 --build-receipt /tmp/new-ds-evaluator-build.json --timeout-seconds 5
python3 docs/ds-retrospective-revalidation/compare_targets.py \
 /tmp/new-ds-target-pilot.jsonl /tmp/new-ds-target-comparison.json
```

See sibling `geometry-pilot/` for separate feature-schema restoration, with no
capacity calls. Together these pilots remove the reason to assume restoration
requires a costly new population or wholesale research rerun.
