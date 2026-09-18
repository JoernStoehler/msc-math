# Paired tangentialization: completed small numerical experiment

On this frozen 16-pair conditional design, replacing both factors' support
heights by one increased mean systolic ratio by **0.0152685**. Eleven paired
effects were positive and five negative. The increase was concentrated in the
4×6 bucket; this is a heterogeneous finite-panel effect, not universal
monotonicity or a population/significance claim.

| Contrast | 4×4 mean (positive/8) | 4×6 mean (positive/8) | Equal-bucket mean | Overall range |
|---|---:|---:|---:|---:|
| Both minus baseline (primary) | .0039814 (4) | .0265556 (7) | .0152685 | [−.0291101,.0734920] |
| q only minus baseline | −.0052409 (3) | .0077422 (6) | .0012506 | [−.0331839,.0301866] |
| p only minus baseline | .0021108 (4) | .0187760 (6) | .0104434 | [−.0470656,.0688863] |
| Interaction: 11−10−01+00 | .0071115 (5) | .0000374 (4) | .0035745 | [−.0039892,.0584241] |

All 64 requests succeeded, with 64 distinct geometries, one evaluator identity,
and all 16 four-arm grids complete. Maximum numerical sys was .7414698962;
none exceeded one. Every paired value, sign count, bucket range and mean is in
`artifacts/analysis.json`. No post-hoc significance test or selected-arm verdict
was introduced. The results motivate attention to bucket dependence of the
operation; they do not establish that tangentialization always helps.

## Design and implementation

Source `ds-tangentialization-v1`, seed 202609140101; eight latent draws each in
4×4 and 4×6. For each latent attempt, ChaCha8 receives the BLAKE3 digest of
`ds-tangentialization-v1|seed=202609140101|bucket=4x4|row=0|attempt=0`, substituting
bucket/row/attempt with their actual decimal values, without a trailing newline.
The existing `random_polygon_2d` draws q then p with heights in [.8,1.2].

The generator reuses the alternative-source packet's `active` and `normalize`
logic. It retains baseline factors Bq,Bp and same-normal unit-height factors
Tq,Tp; it normalizes all four factors once, then forms arms 00=Bq×Bp,
10=Tq×Bp, 01=Bq×Tp, 11=Tq×Tp. Separate area normalization is a common scaling
composed with a symplectic scaling, so it preserves the mathematical ratio;
setting unequal heights to one is the shape intervention. Numerical normalized
areas differed from one by at most 6.67e−16. Actual reconstructed volumes,
which were used by the evaluator, ranged from .9999999999999978 to
1.000000000000001. These are numerical normalization checks, not exact identities
for rounded source geometry.

Every arm must pass target-free exact-binary64 boundedness/nonredundancy
reconstruction before its latent attempt is accepted. Attempts are capped at
128 per latent row. There were 63 attempts: 16 accepted and 47 rejected
(37 at baseline q active/area validation, 10 at baseline p). The complete
construction log includes failed attempt indices and reasons. This jointly
admissible conditional law is explicitly different from the historical
factorial-both packet. No target-based rejection or replacement occurred.

All 64 input rows were written and hashed before evaluating any target.
Order interleaves bucket 4×4 then 4×6 within row index; each latent row uses
00,10,01,11. Input rows retain source/latent identity, normals, pre/post heights,
pre/post areas and actual dual coordinates. Input SHA-256:
`9cd70daadc64e47baee0bedb91b219182d14626bfcfdbf6eb0d16aeb46982fa5`.
`artifacts/freeze.json` records the pre-evaluation time, source HEAD, generator
source/binary, generator-library source, lock and input hashes. They were all
rechecked unchanged after evaluation.

## Evaluation, cost and validation

The unchanged `current-body-evaluator` consumes actual stored binary64 dual
geometry. All rows dispatched to the production structural-product capacity
route with outward bounds, exact rational capacity and midpoint tolerance
1e-10; volume uses f64 vertices from exact-derived incidence. Thus sys is
numerical, not a certified ratio enclosure. Each result retains complete input,
error/status, source/binary receipt, bounds, volume method and timing.

Execution held `/workspaces/msc-math/.git/codex/ds-evaluator.lock` via `flock`,
used one serial evaluator with two-thread environment caps, a five-second
per-input deadline and 120-second total execution cap. There were exactly
64 scientific requests and no additional target controls in this packet.
The adapter's already-retained analytic cube/scaling/U(2)/volume/dispatch and
failure controls provide its plumbing checks; this experiment additionally
checks area normalization and the full paired grid.

Cached package compilation took 2.14s. Full construction, including rejected
attempts and all four-arm geometry checks, took 1.91s wall (0.32s user + 1.51s
system CPU, 7616 KiB peak RSS). Complete evaluator execution took 1.41s wall
(1.06s user + .29s system CPU, 35760 KiB peak RSS). Summed per-request wall time
was 1.130s; it excludes supervisor bookkeeping and must not be called total
experiment cost. Construction plus evaluation was 3.32s, or approximately
5.46s including build; editing/review elapsed time is separate. GNU-time logs
are retained. No concurrency benchmark or worst-case runtime claim is implied.

`analyze.py` requires the exact 16×4 grid, unique IDs, matching input geometry,
one evaluator identity/input hash, successful rows, formula consistency,
bounds and product routing. An incomplete/error grid produces only an
operational summary, without mean-effect conclusions. Independently reviewed
producer logic confirmed latent pairing, normalization, pre-target acceptance
and paired analysis; review did not rerun targets.

Commands are retained in `commands.sh` (historical output paths; do not overwrite
retained artifacts). To recompute only the summary, without target calls:

```bash
python3 experiments/sys-datascience/methods/paired-tangentialization/analyze.py
```

No historical data were rebuilt, evaluator modified, additional method fitted,
or scientific pilot expanded during this experiment.
