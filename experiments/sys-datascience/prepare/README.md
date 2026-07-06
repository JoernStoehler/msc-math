# Random-Polytope Datascience Prepare

This directory owns the shared prepare stage for the active
random/product sys-datascience slice: load producer outputs, compute reusable
feature columns, and write prepared tables for method packets.

Current active method-facing covariates are invariant-feature columns. Prepare
does not choose a canonical representative. It computes formulas whose values
are invariant under `Sp(4) x R_+ x R^4 x Perm(F)`, with metadata kept separate
from numeric covariates.

The active prepare path is random/product only. Old ascent and continuation
prepared-table uses are obsolete for this thesis slice.

## Commands

Build the retained random/product table in place:

```bash
experiments/sys-datascience/build-dataset.sh
```

This retained rebuild reads LFS-tracked producer files under `produce/`.
Hydrate those files first with `git lfs checkout`/`git lfs pull` when working in
a no-smudge worktree. Use the run-local producer smoke below when the goal is
to test source/provenance plumbing without retained LFS data.

Build scratch slices for development or evidence runs:

```bash
experiments/sys-datascience/prepare/build-random-only-slice.sh smoke
experiments/sys-datascience/prepare/build-random-only-slice.sh method
experiments/sys-datascience/prepare/build-random-only-slice.sh full
```

The scoped builder calls `sys-dataset --random-only-size <smoke|method|full>`.
Limited presets take deterministic stratified prefixes:

- `smoke`: `8` generic random rows plus `20` product rows;
- `method`: `512` generic random rows plus `1024` product rows;
- `full`: all retained random/product rows.

## Run-Local Producer Input

`sys-datascience-prepare` consumes a run-local producer output directory
containing `computed-polytopes.jsonl` plus producer metadata files:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
  --mode smoke \
  --producers random,random-product \
  --output-dir /tmp/ds-produce-smoke-cold \
  --parallelism 4 \
  --base-cache /tmp/ds-produce-smoke-cache.jsonl

uv run --script experiments/sys-datascience/produce/validate-datascience-produced.py \
  --produce-dir /tmp/ds-produce-smoke-cold \
  --mode smoke \
  --producers random,random-product

cargo run -p exp-sys-landscape --release --bin sys-datascience-prepare -- \
  --produce-dir /tmp/ds-produce-smoke-cold \
  --out-dir /tmp/ds-prepare-smoke

uv run --script experiments/sys-datascience/fingerprint-dataset.py \
  /tmp/ds-prepare-smoke
```

For the known HKO reference/holdout row:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-produce -- \
  --mode smoke \
  --producers known-hko-reference \
  --output-dir /tmp/ds-produce-hko \
  --parallelism 1 \
  --base-cache /tmp/ds-produce-hko-cache.jsonl

cargo run -p exp-sys-landscape --release --bin sys-datascience-prepare -- \
  --produce-dir /tmp/ds-produce-hko \
  --out-dir /tmp/ds-prepare-hko
```

Prepared HKO rows use `capacity_source = known_hko_reference` and provenance
`role = reference_holdout`. Shared trusted-random method filters exclude this
dataset by default; score it only through reference-aware packets.

On LICCA, `licca-datascience-prepare.slurm.sh` runs only the prebuilt
`sys-datascience-prepare` binary. Build before submitting:

```bash
cd "$HOME/msc-math"
export CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
cargo build --release -p exp-sys-landscape --bin sys-datascience-prepare
```

Validate the produce directory before submitting prepare, then fingerprint the
prepare output after the job finishes.

## Prepared Outputs

Current method-facing files:

- `polytope-table.jsonl`: one row per retained random/product polytope, with
  identity/target fields, source metadata needed by methods, and invariant
  feature columns;
- `polytope-provenance-table.jsonl`: one row per retained random/product
  provenance record. Run-local and canonical random/product provenance rows
  include a nested `source` object when source bucket data is available.
  Run-local reference prepares can additionally contain `known_hko_reference`
  provenance rows for holdout scoring.

`polytope-table.jsonl` is the method-facing table. It exports only
identity/target/source fields and invariant feature columns. Raw dual vertices,
capacity, volume, Euclidean representative features, omega magnitudes of
normalized dual rows, and transition features remain in producer artifacts and
are joined by `poly_id` only when source inspection is needed.

The active columns are:

- identity/target/metadata: `poly_id`, `sys`, `capacity_source`;
- combinatorial invariants: facet/vertex/edge/ridge counts, simplicity,
  vertex-incidence summaries, vertex-degree summaries, ridge-size summaries,
  facet-vertex summaries, and facet-neighbor summaries;
- symplectic two-face area invariants: successfully ordered two-face
  symplectic area statistics divided by `volume.sqrt()`, plus dimensionless
  max/top-3 shares.

The mathematical contract is implemented in `invariant_features.rs`. The key
facts are:

- combinatorial columns depend only on the face lattice;
- two-face symplectic area is translation-invariant by telescoping around each
  closed polygonal face and Sp(4)-invariant by definition of a symplectic map;
- under primal scaling `x -> lambda x`, two-face symplectic area scales by
  `lambda^2`, while 4D volume scales by `lambda^4`, so division by
  `volume.sqrt()` removes scale.

In the normalized inequality representation, a translated polytope can be
represented again as `<a_i, x> <= 1` only when the translated origin is
interior, so all facet denominators are positive. The geometric invariance
claim is for the polytope; the f64 table builder assumes such a normalized
input representation.

The v1 active table intentionally excludes Euclidean representative
features, omega magnitudes of normalized dual rows, and cutoff/sign features.
Omega sign and transition features need a tolerant ternary classifier plus
boundary-hit diagnostics before they should be treated as method-facing full
invariants.

Check the invariant feature contract with:

```bash
cargo test -p exp-sys-landscape invariant_features --release
cargo run -p exp-sys-landscape --release --bin sys-datascience-invariant-feature-check
```

The report command uses synthetic polytopes and deterministic representatives
of scale, translation, facet permutation, sampled `Sp(4)` via `exp(JH)`, and a
full-group composition. It is a low-friction guard for new invariant columns;
feature additions should extend this report before method packets consume the
new column.

Check prepared outputs with:

```bash
uv run --script experiments/sys-datascience/fingerprint-dataset.py \
  experiments/sys-datascience/prepare
```

## Feature Cost Profiler

Use `sys-datascience-feature-cost` when a method packet needs per-polytope cost
for the current invariant feature families before adding method-local timing
boilerplate.

Smoke command, independent of retained LFS producer files:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-feature-cost -- \
  --synthetic-smoke \
  --out-dir /tmp/sys-ds-feature-cost-smoke
```

Retained producer sample command, after hydrating `produce/random*.jsonl`:

```bash
cargo run -p exp-sys-landscape --release --bin sys-datascience-feature-cost -- \
  --random-only-size smoke \
  --max-polytopes 10 \
  --out-dir /tmp/sys-ds-feature-cost-smoke
```

Use `--max-polytopes all` only when deliberately spending the larger run. The
command is sequential so that per-row timings remain easy to interpret.
`--retained-produce-dir` overrides the retained cache directory containing
`random.jsonl` and `random-product.jsonl`. This is intentionally different from
`sys-datascience-prepare --produce-dir`, which consumes run-local producer
outputs with `computed-polytopes.jsonl`.

Generated files:

- `feature-cost-per-polytope.jsonl`: one row per profiled polytope, with
  `poly_id`, `capacity_source`, `facet_count`, product bucket fields when
  available, group timings in milliseconds, total timings, cached/recomputed
  volume, and selected invariant feature values for sanity checks.
- `feature-cost-group-summary.tsv`: aggregate timing summary by `bucket` and
  feature group, plus the `all` bucket.
- `feature-cost-run-summary.json`: input mode, row counts, output paths, and
  the timing-boundary note.

Timing boundaries:

- JSONL rows use the direct field names `standard_prepare_feature_time_ms`,
  `volume_recompute_ms`, and
  `feature_first_total_with_volume_recompute_ms`.
- TSV summary rows use shorter group labels:
  `standard_prepare_feature_total`,
  `volume_recompute_from_dual_vertices`, and
  `feature_first_total_with_volume_recompute`.
- `standard_prepare_feature_time_ms` is a component sum for the current prepare
  feature path: decode dual vertices, reconstruct the polytope from dual
  vertices, enumerate the face lattice, compute skeleton summaries, compute
  ridge symplectic-area summaries, and assemble the output row. It uses the
  producer-cached volume for normalization, as the current prepared table does.
  It is not a clean wall-time replay, because the profiler also runs exact
  volume recomputation before the later component timings.
- `volume_recompute_ms` measures exact volume recomputation from the
  reconstructed dual-vertex geometry. This is not currently paid by prepare
  when producer volume is already cached, but it is the relevant extra cost for
  feature-first proposer code that starts from dual vertices only.

## Identity And Provenance Contract

The prepared random/product tables keep polytope identity separate from sample
or state provenance:

- `poly_id` identifies the producer's ordered normalized-dual geometry and is
  stable across producer rows that produce the same polytope.
- provenance rows carry source metadata such as `root_group_id`,
  `lineage_id`, and `source`. `source` is the structured sampling-event
  descriptor; existing flat columns such as `sample_h_min`, `sample_h_max`,
  `product_k`, and `product_m` are compatibility/access columns derived from
  the same source data.
- exact rational geometry payloads such as `dual_vertices_rational` stay with
  the polytope identity rather than with a one-off method-local matrix.
- optimizer or local-system rows may need additional state-level identity, but
  that belongs to a reopened producer design, not to the retained
  random/product prepared schema.

Do not replace this with a single wide table unless there is a current owner
for the provenance loss and migration cost. Add schema fields when possible.

## Code Ownership

- `main.rs` orchestrates retained producer-file loading.
- `prepare.rs` consumes run-local producer directories.
- `load_caches.rs` loads the canonical random/product producer files.
- `invariant_features.rs` computes the active method-facing invariant feature
  columns.
- `features_dual_vertices.rs`, `features_face_symplectic.rs`,
  `features_helpers.rs`, and `features_skeleton.rs` provide the narrow helper
  surfaces used by `invariant_features.rs`.
- `write_database.rs` writes the retained JSONL tables.

Reusable feature columns belong here when several method packets should consume
them. Model-specific matrices, train/test labels, reports, and plots belong in
the relevant method folder.
