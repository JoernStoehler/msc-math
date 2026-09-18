# Full retained-population current-target comparison

2026-09-18. All **14,336 requests accounted for**, in **175.28 seconds** outer
wall time. The frozen selection is the entire historical population, ordered by
poly_id. Each request had a 5-second timeout; the whole run had a 15-minute cap.

**14,245 succeeded. 91 returned typed failures:** 54 primal-norm policy failures
and 37 scalar acceptance failures (`BoundsTooWide` at tolerance 1e-10). No
request timed out; maximum request wall time was 86.97ms. All old values, new
values, errors and identifiers are retained. No tolerance or policy was relaxed.

Among successful bodies, maximum absolute relative deltas are:

| Quantity | Largest relative difference |
| --- | ---: |
| Capacity | 1.06e-14 |
| Volume | 3.14e-14 |
| Systolic ratio | 3.23e-14 |

Maximum new numerical sys is 0.8625858958494405. Current implementation therefore
strongly corroborates the retained targets on these 14,245 matched bodies;
**it does not validate all historical targets**. Failed inputs remain in the
population and must not be silently dropped from scientific claims.

## Central agreement is not interval inclusion

5,583 historical capacity scalars lie outside the new narrow outward intervals.
This is distinct from central-value disagreement: all successful central
relative differences are below 1.06e-14. Narrow exact-product bounds need not
contain an independently rounded historical scalar. `comparison.jsonl.gz`
retains the inclusion flag and exact reported bounds for every successful row.
We do not relabel exclusions as successful certification of the old number.

## Numerical contract and remaining repair

The unchanged evaluator uses exact binary64 geometry and the production capacity
API, retaining exact product values or outward capacity bounds. Volume remains
f64 arithmetic on exact-derived incidence. This is not a certified sys interval.
The 37 `BoundsTooWide` results do not mean capacity search failed: scalar
conversion refused the requested tolerance. The current adapter discards the
otherwise computed bounds on that failure path. Smallest repair: a separate
interval-preserving adapter run on those 37 exact IDs, preserving the original
failure and refusing to call a broad midpoint a 1e-10 scalar. No rerun occurred.

For 54 primal-norm failures, examine exact geometry ranges first. A power-of-two
uniform rescaling may satisfy both primal/dual policy ranges while preserving
exact binary64 geometry up to known scaling; capacity scales quadratically,
volume quartically, and sys is invariant. This is a possible repair, not yet
proved feasible for every failed body. Do not bypass safety bounds or pretend
the original request succeeded. The cheap `norm-diagnostic.json` inspection finds feasible power-of-two scales
for 53 of 54 using retained rational primal vertices; one has no feasible
uniform scale. These vertices were not newly certified complete, so a fresh
geometry reconstruction and normal policy checks remain required. No transformed
request was executed.

The feature-schema migration is separate and does not inherit new certification.
The full population's original statistical analysis remains reproducible; a new
analysis using current targets must expose the unresolved 91 rather than fill
those with legacy values under one undifferentiated contract.

## Retained files and reproduction

`inputs.jsonl.gz` and `results.jsonl.gz` preserve exact full raw JSONL bytes;
`payload-manifest.json` records compressed and uncompressed hashes. Original
uncompressed local files are ignored. `input-manifest.json` records frozen source
hashes; every result embeds build/source/binary identity, actual dirty checkout,
raw input hash, tolerance and timing. `summary.json` retains all failure IDs and
per-stratum coverage; `comparison.jsonl.gz` retains matched scalar comparisons.
The inherited build receipt is in sibling `target-pilot/` (86-second build).

For analysis, decompress to filenames `inputs.jsonl` and `results.jsonl` in this
directory, verify the manifest hashes, then run `analyze.py`. `prepare.py`
reconstructs all inputs from the standard verified source cache, refusing to
overwrite. Full evaluation command from the repository root:

```sh
timeout 900s python3 experiments/sys-datascience/methods/current-body-evaluator/evaluate.py \
 --input docs/ds-retrospective-revalidation/target-full/inputs.jsonl \
 --output /tmp/revalidation-results.jsonl \
 --binary /workspaces/msc-math/target/ds-retrospective-evaluator/release/current-body-evaluator \
 --build-receipt docs/ds-retrospective-revalidation/target-pilot/build-receipt.json \
 --timeout-seconds 5
```

No source mutation during execution. No main merge, thesis edit, or old-target
overwrite. A cap would have left flushed partial results and explicit missing IDs;
this run completed all requests. Resume must select only missing input IDs into
a separately identified follow-up run, not overwrite evidence.
