# Random-Polytope Sys-Datascience

This folder supports the thesis data-science/search slice, currently restricted
to random polytopes and random Lagrangian-product polytopes.

The active thesis question is:

> On the retained and named extension random/product samples and ordinary
> data-science methods, do we see a `sys > 1` example, a credible
> candidate-proposer for finding one, or thesis-useful structure?

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
- `prepare/` owns invariant feature computation, provenance joins, and retained
  prepared tables.
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
or new distributions are separate producer changes. The only currently prepared
producer extension is the smoke-first high-complexity generic/product bucket
packet under `coordination/high-complexity-producer-compute-packet-2026-07-08.md`;
it is not evidence until executed and reviewed.

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

- `agent-memory-and-expansion-plan.md` for the agent-facing interpretation of
  why the current retained wave is a fallback rather than full data-science
  closure, and how to plan broader method/distribution/proposer work.
- `coordination/autonomous-parent-loop.md` for launching or reviewing an
  autonomous parent loop over the full sys-datascience slice.
- `produce/README.md`
- `prepare/README.md`
- `feature-space-coverage-ledger.md`
- `methods/README.md`
- `methods/trusted-random-product-closure-summary.md`
- `methods/trusted-random-product-method-dispositions.md`
- `coordination/README.md` for active cross-session research coordination,
  workflow roles, topic maps, and spawn/rescope/stop state
- relevant `methods/<method>/README.md`

Current closure status: active packets have been rerun under the invariant
feature contract. Retained tables live under `prepare/`, and compact generated
method summaries are tracked under `methods/<method>/artifacts/` when a README
cites current numbers. Keep durable packet conclusions in
`methods/<method>/README.md`, not only in `/tmp`.

Current coordination status: the bounded retained-table source map, P4
generated-candidate closure note, and high-complexity producer compute packet
exist under `coordination/`. The bounded retained story is a fallback, not
automatic full-slice closure. The compute packet is a prepared LICCA handoff,
not a completed run.

Escalate before unrelated cleanup if a method records a trusted `sys > 1` row
or a credible candidate-proposer.
