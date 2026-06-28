# Random-Polytope Sys-Datascience

This folder supports the thesis data-science/search slice, currently restricted
to random polytopes and random Lagrangian-product polytopes.

The active thesis question is:

> On the retained random/product samples and ordinary data-science methods, do
> we see a `sys > 1` example or a credible candidate-proposer for finding one?

The old fixed-F ascent and continuation runs are not part of this active slice.
They were confusing for this chapter because endpoint diagnostics showed that
the retained ascent endpoints should not be treated as local maxima. Use git
history if an old ascent artifact is needed for archaeology; do not route new
datascience work through those files.

Legacy bounded random/ascent/continuation observations that are still worth
preserving live in `experiments/sys-landscape/legacy-ascent-continuation-debt.md`.
They are context only. They are not active method-table rows and do not support
local-maximality, exhaustive-search, or candidate-proposer claims.

## Data Flow

```text
produce/  ->  prepare/  ->  methods/
```

- `produce/` owns random/product row production and cached expensive
  polytope/capacity payloads.
- `prepare/` owns canonization, reusable geometry features, provenance joins,
  and retained prepared tables.
- `methods/` owns method packets over the prepared random/product tables.

## Current Random Distributions

Current retained producer contract:

- generic random rows: facet counts `F=5..12`, `512` accepted samples per `F`;
- random Lagrangian-product rows: polygon-pair buckets `3 <= k <= m <= 6`,
  `1024` accepted samples per bucket;
- both use seed `42`, height interval `[0.8, 1.2]`, and rejection until a
  valid polytope is produced.

This is a finite sample from these distributions, not a universal random-model
claim. Broader height intervals, independent seeds, other facet-count ranges,
or new distributions are future producer changes.

## Retained Tables

Prepared table output path:

```text
experiments/sys-datascience/prepare/
```

Active table files:

- `polytope-table.jsonl`: one row per retained random/product polytope;
- `polytope-provenance-table.jsonl`: producer/source metadata for those rows.

Build or refresh the retained random/product tables from canonical producer
files:

```bash
experiments/sys-datascience/build-dataset.sh
```

Build scoped scratch tables:

```bash
experiments/sys-datascience/prepare/build-random-only-slice.sh smoke
experiments/sys-datascience/prepare/build-random-only-slice.sh method
experiments/sys-datascience/prepare/build-random-only-slice.sh full
```

Check a prepared table fingerprint:

```bash
uv run --script experiments/sys-datascience/fingerprint-dataset.py \
  experiments/sys-datascience/prepare
```

## Method Surface

Read first:

- `produce/README.md`
- `prepare/README.md`
- `feature-space-coverage-ledger.md`
- `methods/README.md`
- `methods/random-only-closure-summary.md`
- `methods/random-only-method-dispositions.md`
- relevant `methods/<method>/README.md`

Current closure status: pending review. The current-schema random/product
prepare rerun and active method-packet reruns have been performed on the
retained full table. The prepared full table is retained because regeneration is
multi-minute work.

Escalate before unrelated cleanup if a method records a trusted `sys > 1` row
or a credible candidate-proposer.
