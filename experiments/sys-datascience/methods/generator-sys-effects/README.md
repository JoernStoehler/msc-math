# Generator `sys` effects smoke

This packet summarizes the complete, reviewed one-row target pilot from
`alternative-generator-smoke`. It exists because a four-row-per-arm target run
did not finish inside 120 seconds and left a truncated, law-ordered file. That
partial file is not an analysis input.

Run:

```text
uv run --script experiments/sys-datascience/methods/generator-sys-effects/analyze.py \
  --input experiments/sys-datascience/methods/alternative-generator-smoke/artifacts/target-pilot/smoke-rows.jsonl \
  --manifest experiments/sys-datascience/methods/alternative-generator-smoke/artifacts/target-pilot/batch-report.json \
  --out-dir experiments/sys-datascience/methods/generator-sys-effects/artifacts
```

The companion batch report is mandatory. Before analysis, the command checks
its schema and command contract (including `--rows-per-law 1 --target`), then
checks its row and status counts, seed and law version, attempt cap,
side-count-pair grid, per-arm grid and counts, and runtime-cap classifications
against the JSONL. A truncated file or a valid law-ordered prefix is rejected;
the analyzer never treats a partial producer output as the target pilot.

The report audits row counts, censoring, target cost, all named arm/bucket
witnesses, and the exact paired factorial and antipodal contrasts. The TSV is
sorted within each side-count bucket for inspection. `display_order` is only
the row's position in that display; it is not a population rank.

The only supported use is semantic and hypothesis generation. There is one
deterministic row per arm and bucket, 6x6 targets are predeclared runtime-cap
skips, and one 3x3 Dirichlet-gap row failed bounded generation. These data do
not estimate transfer, tail density, law means, or stable generator order.
Passing the manifest gate establishes artifact completeness and internal
provenance consistency only; it does not strengthen those scientific claims.
