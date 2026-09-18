# Historical DS restoration: current status

Updated 2026-09-18. **Source geometry and feature schema are restored for all
14,336 historical bodies. Current capacity intervals cover 14,335; one body
remains unresolved.** No main merge or thesis edit has occurred.

| Deliverable | Result | Evidence |
| --- | --- | --- |
| Source recovery and identity | All source hashes, geometry IDs and original target joins verified | `source-verification.json`, `cache-inventory.json` |
| Full feature-schema rebuild | Six missing columns restored; all 645,120 overlapping cells exactly unchanged; provenance bytes unchanged | [Geometry migration](geometry-full/README.md) |
| Unchanged current evaluator | All 14,336 requests accounted for; 14,245 successes and 91 typed refusals | [Full target comparison](target-full/README.md) |
| Bounded failure repair | 37 intervals preserved without scalar conversion; 53 exact rescalings passed fresh geometry/policy checks | [Repair](repair/README.md) |
| Remaining case | `random_F8_s3_45`: no positive uniform scale satisfies numerical-size policy; bounded diagonal-symplectic screen found no candidate | [Norm investigation](repair/norm/README.md) |

Current coverage is **14,289 accepted 1e-10 capacity scalars, 46 wider capacity
intervals and one unresolved body**. Numerical volume is not certified; therefore
no certified sys intervals are claimed. This is new retrospective computation,
not reconstruction of an undocumented historical execution environment.

## Source and historical evidence

The registered snapshot
`f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96`
is now in standard host and sandbox caches. It contains the 53MB generic source,
74MB product source and 126MB shared cache. Registered retrieval sufficed; no
population regeneration was needed. No secrets were read.

- All 14,336 source names uniquely join table/provenance rows.
- BLAKE3 identities of ordered binary64 dual coordinates match every poly_id.
- Rational dual coordinates equal exact binary64 coordinates throughout.
- Stored source sys equals historical table sys exactly; scalar-formula residual
  is at most 2.22e-16.
- Every source body matches shared-cache rational geometry, capacity and volume.
  The cache contains 14,550 unique bodies, including 214 additional bodies.
- Generic cache entries restore recorded master_seed, attempt and facet/height
  parameters. Product cache entries do not restore seed/attempt. These remain
  legacy assertions, not newly certified generation history.

The original 39-feature statistical analysis already reproduces; see the
[DS evidence account](../ds-evidence-closure/README.md). Full schema migration
preserves old sys and old normalization volume while adding six geometry
features. Current targets remain separate, so downstream code cannot accidentally
interpret inherited values as fresh certified outputs.

Historical execution revision, build state and cache freshness are unrecorded.
Retained winning words/orbit scalars are not completeness certificates; historical
zero error fields are not exactness proofs. New computations now supply direct
matched-body evidence instead of requiring those missing historical facts.

## What the new calculations establish

The untouched current scalar evaluator corroborated 14,245 bodies to at most
1.06e-14 relative capacity difference. Some historical scalars fall outside very
narrow current intervals despite tiny central differences; all inclusion flags
and differences remain available. The interval adapter repaired 90 refusals
without relaxing policy or requesting looser accepted scalars. Exact dyadic
rescalings transfer interval endpoints back to the original bodies as rational
numbers. Those rational endpoints are not themselves exact capacity values.

Full feature rebuilding took 586 seconds on four threads. Full original-body
requests took 175 seconds; repair requests took 1.42 seconds plus a 65-second
adapter build. These are measured runs, replacing the earlier pilot estimates.
All receipts retain actual source/binary/input identities, outcomes and timing.

## Remaining actions

1. Integrate the restored feature derivative and its receipts deliberately; point
   consumers at the selected immutable artifact. Do not overwrite old evidence.
2. Preserve separate legacy values, current intervals and refusal/repair status
   in any unified downstream table. Historical association and current target
   corroboration are distinct results.
3. Decide whether to pursue exact symplectic preconditioning for the single
   remaining body. No policy bypass or unbounded rerun is authorized by these
   records. Alternatively retain its precise unresolved status.
4. Volume certification is separate if a mathematical sys interval is needed.
   Later 64/124 panels and optimizer trajectories have separate owners/contracts;
   this baseline restoration does not validate their targets or rankings.

## Reproduction and historical stages

Current full outputs are retained as deterministic gzip files with raw/compressed
hashes. Their local raw forms are gitignored. Follow the linked full-run reports
for restoration and checks. Source retrieval uses the authenticated sandbox:

```sh
ssh codex-msc-math.sbx 'python3 /workspaces/msc-math/scripts/artifacts.py materialize polytope-datasets --no-link'
```

`verify_import.py` rechecks pinned source/table/provenance hashes and complete
joins; `inventory.py` checks cache correspondence. Their original timestamped
receipts record only that stage's checks. They are not a summary of later work.

The [36-body geometry pilot](geometry-pilot/README.md) and
[36-body target pilot](target-pilot/README.md) are **historical pilot records**.
Their proposed full runs have now completed; their old runtime projections and
then-pending next steps are superseded by the full-run reports above. The original
issue is reviewer-trial `historical-ds-provenance.md`, commit `12869c30`.
