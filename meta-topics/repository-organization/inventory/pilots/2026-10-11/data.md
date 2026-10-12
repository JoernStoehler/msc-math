# Data builder early unit — 11 October 2026

## Result and boundary

`sys-table-builder --input-format retained|run-local` now dispatches two explicit input adapters into one feature, provenance, and write path. `sys-dataset` and `sys-datascience-prepare` remain compatibility commands. The retained adapter still leaves historically unrecorded `capacity_method`, `capacity_candidate_family`, and `volume_method` absent; the run-local adapter reads these labels from its computed payload. No capacity evaluator, sampler, cache, or retained dataset was run or rewritten.

Owned changes: `topics/empirical-studies/polytope-invariant-table/{Cargo.toml,builder_cli.rs,table_builder.rs,run_local.rs,load_caches.rs,main.rs,prepare.rs,README.md,build-retained-table.sh,build-random-only-slice.sh,licca-build-retained-table.slurm.sh,licca-datascience-prepare.slurm.sh}` and `fixtures/builder-smoke/`. The local scripts now call the explicit-format entrypoint; the dormant Slurm run-local wrapper expects its prebuilt binary. The feature-cost target was not edited. Root owns the coordination/inventory synthesis.

## Frozen inputs and comparison

The [builder fixture](../../../../../topics/empirical-studies/polytope-invariant-table/fixtures/builder-smoke) contains one generic five-facet polytope and one 3×3 triangle product. The rows originated in the registered `polytope-datasets` cache. The seeded command fixture uses `seed=20261011` and `attempt=0`; those values and the run-local evaluator labels are **fixture placeholders**, not historical run facts. The original rows, with absent seed and evaluator metadata, are also retained. Pre-edit baseline outputs were frozen before implementation changes; their output JSONL files were copied into the fixture. The run-local `prepare-stats.json` was excluded from exact-byte baselines because it includes paths and timings.

The seeded input SHA256 values are: retained `random.jsonl` `2d9937048163c5c3022271854f15a2f4cbd7bbe8803b9e960dbd25d51b009ae7`, retained `random-product.jsonl` `de09967684a293cd1d10cca0e448d46a4b6a0e0534637db1b372b44e6881680b`, run-local `computed-polytopes.jsonl` `b299aac3636629163d4104c3910b4de4ae38ac237b643f2445d745d84458077c`, `random-samples.jsonl` `af304d64f962c147d737c7640e0d9eb761e05736e8606b1d8537f0aa29941807`, and `random-product-samples.jsonl` `ecbd62ea311fff42fde1d89bb3ede6d2938c45ba1db06680f8bc4d571e3a6b2f`.

From repository root, the actual supported commands used for post-edit comparisons were:

```bash
cargo build -p exp-polytope-invariant-table --bins
RAYON_NUM_THREADS=1 target/debug/sys-table-builder --input-format retained \
  --random topics/empirical-studies/polytope-invariant-table/fixtures/builder-smoke/retained-input/random.jsonl \
  --random-product topics/empirical-studies/polytope-invariant-table/fixtures/builder-smoke/retained-input/random-product.jsonl \
  --out-dir /tmp/msc-math/data-pilot/new/retained-canonical
RAYON_NUM_THREADS=1 target/debug/sys-table-builder --input-format run-local \
  --produce-dir topics/empirical-studies/polytope-invariant-table/fixtures/builder-smoke/run-local-input \
  --out-dir /tmp/msc-math/data-pilot/new/runlocal-canonical
```

Both compatibility commands were run on the same frozen input bytes. The canonical retained command also accepted the script's `--produce-dir ... --random-only-size smoke` arguments on this fixture. Each route produced two polytope and two provenance rows, with byte-identical JSONL to its own pre-edit baseline:

| Format | `polytope-table.jsonl` SHA256 | `polytope-provenance-table.jsonl` SHA256 |
| --- | --- | --- |
| Retained | `bbcf761bada93d1e4719f6f762a1b95bfd9acdbb357a7141b116b9dd2ec99673` | `35b3a716f25cbff661deb6295094e99ac8a553c38ab5c07c2c7dec83e0a92281` |
| Run-local | `e4442327fe58001e697a15886f1ca6df10c9ca69ddaa28c32100efc9ba35f5d3` | `de49c81295dcf534416efb6926752d4a9694b130dac5a5ebab2f350e12686107` |

This equality covers geometry IDs, targets, invariant values, capacity/volume labels, source descriptors, provenance joins, and serialization on these rows. The historical-unknown input also produced byte-identical JSONL to its pre-edit baseline; all three evaluator keys remained absent. Run-local stats retained the same keys, row counts, `max_sys`, and `sys_gt_one`; only path and timing values varied as expected.

The three copied malformed run-local fixtures each failed before output creation, with the same baseline and post-edit exit status `101` and diagnostic: missing computed payload, sample/payload `sys` mismatch, and duplicate provenance ID. The diagnostic files from the baseline run remain under `/tmp/msc-math/data-pilot/baseline/failure-probes/`; the malformed input bytes are retained with the builder fixture.

## Checks and review

`cargo check -p exp-polytope-invariant-table --bins`, `cargo build -p exp-polytope-invariant-table --bins`, `cargo fmt --all -- --check`, focused `cargo test -p exp-polytope-invariant-table invariant_features`, four local shell syntax checks with `bash -n`, both canonical fingerprint commands, byte comparisons, and the malformed-input probes passed. The feature test ran two focused cases in each of five binary test targets. A separate fresh Sol reviewer inspected the adapter extraction, exact outputs, stats, metadata omission, failures, and scripts and approved the change. Its one nonblocking observation was the lost retained diagnostics; the builder now restores stage timings and capacity-source counts, which the reviewer rechecked and approved.

The fixture is a two-row plumbing and contract check, not validation of the mathematical formulas across the full retained corpus or proof of numerical target correctness. The updated shell wrappers and dormant Slurm command were syntax-checked; no retained full-table or LICCA job was launched.
