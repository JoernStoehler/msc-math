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

## Rust package boundary

The repository root `Cargo.toml` is the shared workspace for active packages.
This directory's `Cargo.toml` registers the `produce/`, `prepare/`, and small
method binaries that use the same dependency surface. Eight method packets
already have standalone `Cargo.toml` and `Cargo.lock` files and remain
independently runnable; their READMEs use `--manifest-path` or explicitly run
Cargo from the method directory. The remaining Rust method executables are
targets of this directory's package rather than one package per small binary.

These packages depend one-way on the shared `exp-sys-landscape` library for
capacity, cache, and polytope helpers. `exp-sys-landscape` does not depend on
the data-science or method packages. Dataset flow is recorded by paths and
commands, not by a reverse Cargo dependency.

## Current Random Distributions

Current retained producer contract:

- generic random rows: facet counts `F=5..12`, `512` accepted samples per `F`;
- random Lagrangian-product rows: polygon-pair buckets `3 <= k <= m <= 6`,
  `1024` accepted samples per bucket;
- both use seed `42`, height interval `[0.8, 1.2]`, and rejection until a
  valid polytope is produced.

This is a finite sample from these distributions, not a universal random-model
claim. A separate method-local `factorial-both` source under
`methods/alternative-source-transfer/` supplied one frozen prospective
ridge/rho transfer test; it does not extend or replace these retained tables.
Broader height intervals, independent seeds, other facet-count ranges, or new
distributions remain separate producer changes. No further extension is
selected. Later demonstration data must be chosen for an explicit consumer;
reopened exploration still requires an exact claim, cost, review gate, and
stopping rule before compute begins.

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

- `coordination/research-closeout-2026-07-22.md` for current routing after the
  known-seed cycle;
- `coordination/final-research-account-2026-07-12.md` and
  `coordination/next-session-candidates.md` for current routing and status;
- `coordination/research-direction-review-2026-07-11.md` for the closed-cycle
  exhaustive inventory only, not current portfolio or launch authority;
- `coordination/exploration-result.md` for the audit of the recovery agent's
  premature exploration closure;
- `produce/README.md`
- `prepare/README.md`
- `feature-space-coverage-ledger.md`
- `methods/README.md`
- `methods/trusted-random-product-closure-summary.md`
- `methods/trusted-random-product-method-dispositions.md`
- `coordination/README.md` for the coordination ownership boundary
- relevant `methods/<method>/README.md`

Current closure status: active packets have been rerun under the invariant
feature contract. Retained tables live under `prepare/`, and compact generated
method summaries are tracked under `methods/<method>/artifacts/` when a README
cites current numbers. Keep durable packet conclusions in
`methods/<method>/README.md`, not only in `/tmp`.

Current coordination status: the corrected exploration/research slice is
feature-complete under the July 15 refresh of
`coordination/final-research-account-2026-07-12.md`. Existing evidence supports
a controlled negative hostile-search benchmark; bounded same-source and
alternative-source operational sub-threshold ridge/rho proposers; coarse
generic ridge transfer with failed harder conditioning; exact triangle and
generator-local bounce structure under their separate review boundaries;
witness-level orientation relevance with a failed ridge mediator; and bounded
gradient/optimizer results. Bespoke adaptive search produced no scientific
comparison. Parked routes have named reopen conditions rather than forming a
queue. FACTSHEET 34.1 still requires later consumer-driven thesis-level data.
`LICCA.md` classifies retained dormant scripts; none is selected by default.

The July 22 addendum records the later five-case local-maxima screen, exact
Chaidez--Hutchings fixture, promoted bounded HKO panel, retained
orientation/equality pilots, exploratory conditional-tail figures,
first-order theorem review gate, and the current cross-line idea shortlist.
Its stop/defer decisions compare expected total project cost and state what
evidence would reverse them; “deferred” does not itself mean desirable.

Escalate before unrelated work only when an active method produces a trusted
new `sys > 1` candidate or source beyond the already-known HKO/rotated-pentagon
family and declared reference/control rows, or produces a credible new
threshold-directed proposer. A known positive reference is expected comparison or
plumbing evidence unless its value, provenance, or relation to the active
question is newly surprising.
