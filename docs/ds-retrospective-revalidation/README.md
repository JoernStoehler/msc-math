# Historical DS restoration: source recovery completed

2026-09-18. Zero capacity calls; zero feature rebuilds. Continues the
[DS evidence account](../ds-evidence-closure/README.md) and reviewer-trial issue
`docs/reviewer-trial/issue-reports/historical-ds-provenance.md` (12869c30).

## Result

Registered source snapshot `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96`
was downloaded using the standard artifact tool in `codex-msc-math.sbx`, then
copied to the host standard cache. Source files (53MB generic, 74MB product,
126MB shared cache) are recovered; no population regeneration is necessary.
No repository links were changed or secret contents read.

`source-verification.json` records new retrospective verification:

- Both source hashes match the lineage owner.
- All 14,336 source names join uniquely to table/provenance.
- BLAKE3 identities of ordered binary64 dual coordinates match every poly_id.
- Stored source sys equals table sys exactly; scalar formula residual at most 2.22e-16.
- Rational dual coordinates equal exact binary64 coordinates throughout.
- All source bodies match shared cache by rational geometry; capacity and volume agree exactly. Cache has 14,550 distinct bodies, no duplicate keys.

Full hash/geometry-ID/target verification took 2.06 seconds; rational/cache
inventory about 9 seconds. These are verification times, not evaluator timings.
Scripts and machine-readable receipts are retained here. Payloads are in the
standard host cache under `/home/joern/.cache/msc-math/artifacts/polytope-datasets/`.

## Restore without recomputation versus remaining work

| Obligation | Current evidence and next action |
| --- | --- |
| Exact input body and target row | Fully recovered for all 14,336; no recomputation needed for identity |
| Generation selection | Generic cache recovers master_seed, attempt, requested facets/heights. Product cache retains family/size placeholders, no seed/attempt; source datasets have no seed/attempt. Import these as legacy assertions, not reconstructed execution |
| Historic execution | Actual revision/build/cache freshness not recorded. Re-running creates new reproducible evidence, never a historic receipt |
| Capacity certification | Cache has nonempty sigmas/orbit_scalars for 14,528 bodies, zero error/cutoff fields, no current method/revision/bound/exact-capacity fields. Winning words can establish upper bounds but not completeness; no demonstrated no-minimization certification route |
| Certified sys | Capacity and volume must both be justified. Current scalar adapter uses f64 volume; certified capacity alone does not certify ratio |
| Current 45-feature schema | Six missing fields need ridge areas and volume: three threshold fractions plus entropy/effective count/normalized entropy. Compute from geometry, not capacity. Existing quantiles do not determine these fields |
| Original statistical result | Historical 39-feature analyzer already reproduces; no reason to rerun it for this import |
| Later panels/optimizer | Separate contracts and populations; baseline restoration does not validate their targets or optimizer rankings |

Zero-valued historical error fields do not prove exactness. Current cache
migration performs fresh minimization; its mismatch panic should not be used
for an audit that must preserve discrepancies.

## Concrete next actions

1. Create a run-local imported derivative retaining snapshots/hashes, source
   names, poly_id, geometry, original targets, recovered generation assertions,
   and the new verification receipt. Do not fabricate the fields required by
   the current producer's certified payload schema. These scripts validate an
   import; they do not themselves implement a production migration.
2. Geometry-only schema repair: use existing face-descriptor code on a frozen
   pilot of two smallest-poly_id bodies per bucket (36 bodies across 18 buckets).
   Compare overlapping features and measure costs before full migration.
   Retained rational primal vertices may avoid enumeration only after their
   incidence and completeness are verified.
3. Current-target pilot on those same frozen 36 bodies: current scalar evaluator,
   five-second per-request timeout, four-minute outer evaluation deadline, build
   separate. Maximum waiting is 180 seconds plus startup/receipt overhead, not a
   prediction of successful runtime. Retain old/new capacity, volume, sys,
   bounds, timing, failures and discrepancies; do not panic or erase old targets.
   Benchmark all strata before estimating a full rerun.

No pilot was run. Historical capacity timing sums to 732.57 seconds; generic-12
median 88ms and product-6x6 median 407ms. These are recorded legacy values, not
current-code estimates. Prior current controls range from milliseconds on tiny
products to 0.5s on a rotated square product, not representative of generic
12-facet bodies. A defensible full-run estimate requires the stratified pilot.

A pilot supports only its matched-body comparison. To replace full-table targets,
reevaluate all relevant rows, or narrow the claimed population and address
selection. If targets differ, rerun association with the new column and retain
both results. Fresh sampling addresses transfer rather than legacy accuracy.

## Reproduction

Retrieval, using authenticated rclone already available in the sandbox:

```sh
ssh codex-msc-math.sbx 'python3 /workspaces/msc-math/scripts/artifacts.py materialize polytope-datasets --no-link'
```

Copy the verified snapshot to host if needed. Set `source_dir` to its `files`
directory and `table` to the recovered invariant table:

```sh
uv run docs/ds-retrospective-revalidation/verify_import.py \
  --table "$table" \
  --provenance experiments/polytope-invariant-table/polytope-provenance-table.jsonl \
  --source-dir "$source_dir" --output /tmp/new-source-verification.json
python3 docs/ds-retrospective-revalidation/inventory.py \
  "$source_dir" /tmp/new-cache-inventory.json
```

Verifier pins original hashes, refuses overwrite, and records its own timestamp
and script hash. Inventory must be run on the hash-verified source directory.
Neither script evaluates capacities. No manuscript or live producer was edited.
