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

`polytope-table.jsonl` is the method-facing table. This is a schema-breaking
contract relative to the old volume-one representative table: it no longer
exports raw dual vertices, capacity, volume, Euclidean representative features,
omega magnitudes of normalized dual rows, or transition features. Raw producer
geometry, volume, capacity, and orbit search payloads remain in producer
artifacts and are joined by `poly_id` when source inspection is needed.

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
