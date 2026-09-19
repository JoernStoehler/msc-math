# Current arbitrary-body scalar evaluator

A small serial JSONL adapter for fresh support-height and orientation pilots.
This packet enables evaluation; it contains no pilot population or search result.

## Input and execution

Each line is an object with a unique nonempty string `id` and `dual_vertices`,
a list of four-element finite binary64 coordinate arrays. Additional metadata is
retained. The body is exactly `{x: dot(dual_vertices[i], x) <= 1}` for the stored
binary64 numbers. Source rational/algebraic coordinates are not substituted.

```json
{"id":"body-001","dual_vertices":[[1,0,0,0],[-1,0,0,0],[0,1,0,0],[0,-1,0,0],[0,0,1,0],[0,0,-1,0],[0,0,0,1],[0,0,0,-1]]}
```

From repository root, use the existing isolated target directory or another
chosen build cache. Commands build only this standalone package, offline, with
two build jobs. The lockfile is retained.

```bash
python3 experiments/sys-datascience/methods/current-body-evaluator/build.py \
  --target-dir .git/codex/ds-first-wave/readiness/target \
  --receipt /tmp/current-body-build-receipt.json
python3 experiments/sys-datascience/methods/current-body-evaluator/evaluate.py \
  --input /tmp/frozen-inputs.jsonl --output /tmp/new-results.jsonl \
  --binary .git/codex/ds-first-wave/readiness/target/release/current-body-evaluator \
  --build-receipt /tmp/current-body-build-receipt.json \
  --timeout-seconds 30
```

The output must not already exist. Each input line, including malformed JSON,
gets one flushed result row. Duplicate IDs are errors, not silent cache hits.
The supervisor stores the raw input line, its SHA-256, parsed metadata where
possible, full input-file SHA-256, source/binary identity and elapsed wall time.
Every request must count against a pilot's charged budget; errors are never
silently replaced. This adapter has no cache, resume, selection or retry policy.
A completed supervisor exits 0 even when individual rows are errors; the caller
must check exact expected unique IDs, statuses and complete paired grids before
analysis. Charge `wall_ms` on every row and add proposal/construction cost.
A caller terminating the supervisor early must account for unfinished lines;
flush provides process-crash visibility, not fsync durability against host loss.

Each input starts one Rust subprocess; Python enforces a per-input deadline,
kills and waits for an expired child, then emits a `Timeout` result and continues.
No solver grandchildren are spawned. Timeout covers subprocess communication;
OS process creation is not itself interruptible by Python's timeout. Only one
child runs at a time. Child environment caps Rayon, OpenMP and OpenBLAS at two
threads. A pilot should additionally wrap the whole command with its selected
wall deadline. There is no automatic scientific interpretation of failed rows.

`build.py` snapshots Rust sources, relevant workspace/path-dependency manifests
and the standalone lock before/after a successful Cargo build, then writes their
hashes and the executable digest to a receipt. The supervisor checks receipt
consistency against current source and executable bytes before opening output.
Every result includes that receipt, supervisor SHA-256, actual HEAD and dirty
status. This is recorded local build provenance; a dirty checkout is disclosed,
not called a clean revision. Source mutation during a run is outside the
supported coordination contract: freeze evaluator source throughout a batch.

## Mathematical/numerical boundary

The worker explicitly applies the production facet-count and finite/dual-norm
checks, reconstructs `exact_binary64_polytope_geometry`, then checks primal norms.
The current library limits are at most 16 facets and primal/dual vertex infinity
norms in [1e-3,1e3]. Invalid geometry, size policy and exact reconstruction errors
retain their Rust error variants and stage.

Volume is `volume_from_incidence_f64` on rounded exact-derived primal vertices
and exact vertex/facet incidence. It is recorded as
`f64-from-exact-binary64-derived-incidence-v1`; it must be finite and positive.

Capacity uses the production scalar `capacity(&geometry)` API after its required
checks. Dispatch uses actual exact binary64 geometry, not source labels:
structural q/p products select `product`, otherwise `general`. Capacity bounds
and route are retained; midpoint acceptance uses `capacity_value(...,1e-10)`.
Product results also carry their exact rational capacity. General results
carry certified outward bounds; this scalar route does not enumerate/return all
tied minimizers. The general candidate family is the complete transition-pruned
simple HK words within the library's candidate limit; product capacity uses the
closure-vertex reduction. See `docs/capacity-calculation-map.md` for the public
limits and mathematical dependencies.

`sys = capacity²/(2 volume)` remains numerical because volume is binary64.
Bounds on capacity alone do not enclose sys. These outputs do not automatically
certify a threshold crossing, algebraic source object, or exact transformation
invariance. Error rows carry the input, status, stage and detailed error type;
process panics/exits carry exit status/stderr. Successful rows additionally retain
geometry, volume and capacity component timings. All rows retain supervisor wall
time, including failed/timed-out rows. Failed calls do not claim component timing
or route information that was never returned.

## Retained controls and cost

`controls/make_panel.py` freezes six valid bodies plus two invalid inputs:
analytic [-1,1]^4, its factor-two dilation, triangle×triangle, one U(2) rotation
and one non-symplectic SO(4) rotation of that product, a rotated square×square,
an unbounded presentation and malformed coordinate schema. Rotations use the
matrix entries .6/.8; identities below are numerical tolerance checks for the
stored binary64 coordinates. Coordinates are ordered (q0,q1,p0,p1). U(2) rotates
(q0,p0); the non-symplectic SO(4) map rotates (q0,p1).

`controls/results.jsonl` records all eight outcomes. `controls/check.py` checks
analytic c=4 and V=16 for the cube, c scaling by 4 and V by 16, U(2) capacity
invariance, rotated volume invariance, actual product/general dispatch, scalar
formula/bounds and typed invalid-input outcomes. These are substantive controls,
not performance or population claims. `failure-inputs.jsonl` and
`failure-results.jsonl` additionally retain a deliberately 1-microsecond timeout,
duplicate ID, overflow-to-infinity JSON number and malformed JSON. Twelve input
lines in total; nine subprocess launches including invalid and timeout requests;
six valid completed target evaluations. No further target evaluations were run.

Run the retained checks without evaluating anything:

```bash
python3 experiments/sys-datascience/methods/current-body-evaluator/controls/check.py
```

Observed on 2026-09-14, one child at a time:

| Control | Actual route | Capacity ms | Whole request ms |
|---|---|---:|---:|
| Cube | product | .194 | 2.54 |
| Cube dilated 2 | product | .379 | 5.25 |
| Triangle×triangle | product | .763 | 4.55 |
| U(2) triangle product | general | 81.04 | 85.60 |
| SO(4) triangle product | general | 27.39 | 33.16 |
| SO(4) square product | general | 504.44 | 509.88 |

All valid controls succeeded. U(2) capacity relative difference was about 3.4e-14;
volume controls passed 1e-12 relative tolerance. Complete eight-line execution
used .80s wall, .77s user+system CPU, and 35836 KiB GNU-time peak RSS. Component
and wall timings are distinct; fresh geometry and process overhead are included
only in whole-request time. Initial isolated-package release build took 66s,
reusing the earlier target directory but recompiling differing dependency/profile
features. No simultaneous evaluator throughput was tested.

This supports a small, capped rotated-product pilot, not a worst-case estimate:
near-degeneracy, facet count and exact fallback can change cost dramatically.
Preserve timeouts and reduce a design before target exposure if the planned
budget needs adjustment. The six-body controls are not new evidence about the
research pilots' scientific questions.
