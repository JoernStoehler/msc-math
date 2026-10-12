# Sampling populations, numerical targets and table schemas

The empirical question is whether sampled four-dimensional polytopes approach
or exceed the systolic threshold, and which invariant features describe their
recorded ratios. This is a finite-population computational question. The
retained random/product corpus does not prove Viterbo's conjecture, and its
stored numerical targets are not retroactively certified by the newer
capacity implementation.

## Three distinct data populations

| Population | Actual source and count contract | Interpretation boundary |
| --- | --- | --- |
| Registered generic random | `random.jsonl`, 4,096 accepted rows: 512 for each `F=5..12`, heights `[0.8,1.2]` | June retained numerical values; execution/cache freshness and evaluator certification not recorded |
| Registered Lagrangian products | `random-product.jsonl`, 10,240 accepted rows: 1,024 for each `3 <= k <= m <= 6`, heights `[0.8,1.2]` | Separate product sampling distribution, not an extra generic-random stratum |
| Fresh run-local sources | `sys-datascience-produce`: smoke 8 generic + 10 product; production uses the counts above; explicit bucket plans can override counts/heights | A newly produced population with recorded source and evaluator fields; equal counts do not make it the June snapshot |

[Retained numerical lineage](polytope-datasets/retained-lineage.md)
owns immutable snapshot/file identities and the historical uncertainty.
The recorded maximum `sys=0.86258589584944` and zero recorded `sys>1` support
only the finite stored-table statement. Method summaries do not validate the
underlying numerical computation.

The known HKO pentagon is a separately emitted one-row `known-hko-reference`
source with `role=reference_holdout`. It must not be silently added to the
random sample or used as an independent random exceedance. See the
[producer commands](polytope-datasets/README.md) and
[table reference contract](polytope-invariant-table/README.md#run-local-producer-input).

The standalone producer binaries remain compatibility sources, not aliases
for fresh run-local production. [Generic code](polytope-datasets/random.rs)
has a small default count plan and source-tagged per-attempt generation;
[product code](polytope-datasets/random-product.rs) uses one
shared RNG and default ten rows per bucket. Explicit output/cache flags can
replace canonical files. The [run-local implementation](polytope-datasets/main.rs)
instead derives separate seeds from family, bucket, height interval, sample
index and rejection attempt. Rejection conditions on accepted valid geometry;
the final table is not an unconditional collection of independent uniform
convex bodies. The schema alone does not establish independence between
historical rows.

## Later restoration is separate evidence

The [retrospective revalidation](retrospective-revalidation/README.md) recovered
all historical source joins and rebuilt the full 45-feature schema without
changing old targets or overlapping feature values. Separate current capacity
computations cover 14,335 bodies: 14,289 accepted scalar outputs, 46 wider
intervals and one unresolved body. Those results are later computations with
retained receipts, not recovered execution provenance for June. Their volume
remains numerical, so the account claims no certified `sys` intervals. The
[historical replay account](evidence-closure/README.md) also reestablishes the
original 39-feature association under its historical code/hash contract. None
of these later outputs silently replaces the registered snapshots.

The [product-family theorem mapping](family-theorem-mapping/README.md) checks
separate analytic hypotheses: seven of the ten retained product buckets have a
triangle or quadrilateral factor and are excluded from above-one discovery.
This applies to the underlying accepted coordinate products, not to arbitrary
SO(4)-rotated images or their historical evaluator labels. The remaining three
buckets are not excluded by that checked side-count mapping; this is not a
claim that no other theorem applies.

## Geometry identity is separate from sampling provenance

[Producer schemas](polytope-datasets/rows.rs) distinguish
computed geometry from sampling events. Run-local `computed-polytopes.jsonl`
contains one computed payload per geometry ID; `random-samples.jsonl`,
`random-product-samples.jsonl` and optional `reference-samples.jsonl` associate
sample names and nested source descriptors with those IDs and repeat the
stored target for consistency checking.

`poly_id` hashes ordered binary64 normalized-dual coordinates, with signed
zero normalized. It is defined in
[datascience_cache.rs](sys-landscape/src/datascience_cache.rs).
It is a representation identity: facet permutation, translation or a
symplectic change of representative can change it even when invariant
features agree. It is neither a congruence-class canonicalization nor a hash
of exact rational geometry. `dual_vertices_rational` supplies the associated
exact input coordinates; it does not change that identity convention.

A source descriptor records producer family, height bounds, bucket parameters,
seed, sample index and rejection attempt. Repeated geometry may have multiple
sample provenance rows. `root_group_id` is a grouping identifier, not evidence
of tuning/test disjointness. The compatibility `seed_index` stores the rejection
attempt, not `source.sample_index`; consumers needing the accepted sample's
index must read the source object. A historical absent seed, attempt or
evaluator label remains absent rather than being inferred from a producer's
current defaults.

## Targets and arithmetic routes

Fresh run-local cache misses use certified finite QP minimizers for capacity
and binary64 volume from exact-derived incidence. The payload records capacity
bounds, exact rational capacity, the actual candidate family, tied minimizing
words and method labels. The capacity midpoint has a relative-width guard;
`sys` still combines that capacity with approximate volume. The cache's legacy
volume tolerance and legacy capacity recomputation policy are owned by
[the producer contract](polytope-datasets/README.md#producer-contracts)
and implemented in
[datascience_cache.rs](sys-landscape/src/datascience_cache.rs).

Retained preparation copies stored `capacity`, `volume` and `sys`, computes
features and leaves unrecorded evaluator fields absent. A historical provenance
`backend=ehz_capacity` or `ehz_capacity_billiard` is assigned by the loader and
cannot attest which historical evaluator ran. In fresh payloads the requested
`auto`/`product` backend is likewise distinct from the actual
`general-hk`/`product-closure-vertex` candidate family.

## Preparation and method consumption

The [explicit builder](polytope-invariant-table/builder_cli.rs)
selects `--input-format retained|run-local` before loading. The
[retained adapter](polytope-invariant-table/load_caches.rs)
and [run-local adapter](polytope-invariant-table/run_local.rs)
feed one [feature/provenance/write path](polytope-invariant-table/table_builder.rs).
This consolidates the demonstrated duplicate prepare paths without merging
their different historical metadata policies. The compatibility executable
names `sys-dataset` and `sys-datascience-prepare` select the corresponding
adapter.

The output [schemas](polytope-invariant-table/rows.rs) are:

- `polytope-table.jsonl`: geometry ID, stored `sys`, origin/evaluator metadata
  and current combinatorial and normalized symplectic two-face features.
- `polytope-provenance-table.jsonl`: sampling-event IDs, geometry join, family,
  role, grouping/source information and compatibility access fields.

Raw geometry and absolute capacity/volume stay producer-owned. Methods join
them explicitly when needed and own their matrices, splits, models, summaries
and plots. The builder's invariant-feature statement applies to the formulas;
numerical representative checks do not prove the theorem. The normalized
inequality representation requires an interior origin. The feature list and
ordering/failure counters are owned by
[the builder README](polytope-invariant-table/README.md#prepared-outputs).
Historical 39-feature analysis summaries and current 45-feature selectors
must be interpreted using their own input hashes, as described in
[the methods account](methods.md).

## Validation, cost and retention

[Producer validation](polytope-datasets/validate-datascience-produced.py)
checks source/count/target/payload/stats contracts; an expected plan additionally
checks exact bucket counts and accepted sample-index ranges. It does not
reprove capacity, reconstruct a clean historical run, or check the scientific
sampling distribution. Preparation rejects inconsistent sample/payload targets,
missing or duplicate payload/provenance records, source-family mismatch and
run-local facet/index metadata errors before writing tables. It is not a
replacement for producer validation.

The [frozen builder fixture](polytope-invariant-table/fixtures/builder-smoke/README.md)
contains one five-facet random row and one triangle product. Its injected seed
and evaluator fields are placeholders. `check-builder-fixture.py` checks six
canonical/compatibility routes against frozen bytes, historical unknown fields
and seven malformed inputs; it never evaluates capacity. Run it after building
the binaries:

```bash
cargo build -p exp-polytope-invariant-table --bins
python3 topics/empirical-studies/polytope-invariant-table/check-builder-fixture.py
python3 topics/empirical-studies/polytope-invariant-table/check-retained-lineage.py
```

The [feature cost profiler](polytope-invariant-table/README.md#feature-cost-profiler)
separates component prepare cost from recomputed-volume cost. Its component sum
is not a clean wall-time replay. A prepare run with cached producer volume does
not pay the same cost as a feature-first candidate proposer supplied only dual
vertices. No speedup claim follows from omitting the latter cost.

Large artifacts retain their immutable snapshots through the
[artifact registry](../../artifacts/registry.json) and
[storage contract](../../docs/artifacts.md). Full table rebuilding, canonical
refresh/promotion, `--max-polytopes all` profiling and production sampling can
be expensive or replace data; this consolidation ran neither full producers
nor dataset promotion. The two-face-control bucket plans and LICCA scripts are
[dormant designs](polytope-datasets/plans/README.md), not
execution evidence or a current research assignment.
