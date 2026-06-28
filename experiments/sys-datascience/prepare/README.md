# Random-Polytope Datascience Prepare

This directory owns the shared prepare stage for the active
random/product sys-datascience slice: load producer outputs, canonize
representatives, compute reusable geometry features, and write prepared tables
for method packets.

The active prepare path is random/product only. Old ascent and continuation
prepared-table uses are obsolete for this thesis slice.

## Commands

Build the retained random/product table in place:

```bash
experiments/sys-datascience/build-dataset.sh
```

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
cargo run -p exp-sys-landscape --release --bin sys-datascience-prepare -- \
  --produce-dir /tmp/ds-produce-smoke-cold \
  --out-dir /tmp/ds-prepare-smoke
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
  volume-one dual vertices, volume-one capacity/action values, unchanged `sys`,
  and reusable features;
- `polytope-provenance-table.jsonl`: one row per retained random/product
  provenance record.

`polytope-table.jsonl` is the method-facing table. It does not carry raw
producer geometry. The prepare stage reconstructs the producer polytope,
normalizes to the volume-one representative, and computes feature groups from
that representative. Consequently `volume` is `1.0`, `capacity` and
`sys` are the volume-one evaluation values. Raw producer geometry and orbit
search payloads remain in producer artifacts and are joined by `poly_id` when
source inspection is needed.

Check prepared outputs with:

```bash
uv run --script experiments/sys-datascience/fingerprint-dataset.py \
  experiments/sys-datascience/prepare
```

## Identity And Provenance Contract

The prepared random/product tables keep polytope identity separate from sample
or state provenance:

- `poly_id` identifies the canonicalized ordered dual-vertex geometry and is
  stable across producer rows that produce the same polytope.
- provenance rows carry source metadata such as `root_group_id` and
  `lineage_id`.
- exact rational geometry payloads such as `dual_vertices_rational` stay with
  the polytope identity rather than with a one-off method-local matrix.
- optimizer or local-system rows may need additional state-level identity, but
  that belongs to a reopened producer design, not to the retained
  random/product prepared schema.

Do not replace this with a single wide table unless there is a current owner
for the provenance loss and migration cost. Add schema fields when possible.

## Code Ownership

- `main.rs` orchestrates canonical producer-file loading for retained tables.
- `prepare.rs` consumes run-local producer directories.
- `load_caches.rs` loads the canonical random/product producer files.
- `features*.rs` computes reusable feature columns.
- `write_database.rs` writes the retained JSONL tables.

Reusable feature columns belong here when several method packets should consume
them. Model-specific matrices, train/test labels, reports, and plots belong in
the relevant method folder.
