# Geometry-only historical schema pilot

2026-09-18: **36 rows completed; every one of 1,620 overlapping table cells agrees exactly with the historical table.** Six new columns were computed. This supports cheap schema restoration on these bodies, not capacity certification or full-population feature equivalence.

## Inputs and guarantees

Parent `pilot-inputs.jsonl` and `pilot-source/{random.jsonl,random-product.jsonl}` freeze two smallest polytope IDs in each of 18 buckets: 16 generic bodies and 20 products. Source coordinates and historical evidence were unchanged.

The existing `sys-dataset` standard path reconstructs geometry and computes skeleton/ridge descriptors. Inspection of main.rs, load_caches.rs, invariant_features.rs and the geometry constructor confirms **no capacity evaluation or volume recomputation**. Normalization uses historical volume; sys remains historical. Polar vertices are reconstructed from exact binary64 rational coordinates; ridge descriptors include floating-point arithmetic. This is a new retrospective feature build, not fabricated historic execution or numerical certification.

## Results and timing

- Release build: 59.76 seconds, separately capped at 300 seconds.
- Execution: 2.7 seconds internal table time; 4.786 seconds including SSH/startup. Four Rayon threads, 240-second cap.
- 36 unique output geometries and 36 provenance rows; IDs equal the frozen selection.
- 45 common columns including metadata/target (39 historical feature columns); 1,620 cells match exactly.
- Six additions: normalized ridge-area threshold fractions at 1e-3, 1e-2 and 1e-1, entropy, effective face count and normalized entropy.
- No removed columns or comparison discrepancies.

Equal-cost extrapolation gives `2.7 * 14336 / 36 = 1075 seconds`, about 18 minutes on four threads, excluding full-input loading. This is only a planning estimate: two deterministic bodies per bucket do not measure runtime tails, and full-population bucket weights differ. The full build was not run. Migration must compare all overlapping rows and preserve discrepancies.

## Retained evidence

`run-receipt.json` records input/source hashes, Git revision, command, elapsed time and output hashes. `run.stdout` records the executed binary SHA-256. `run.stderr` and `build.log` retain execution/build timing. `comparison.json` contains the schema and all discrepancies (none). Source revision and hashes plus Cargo.lock identify the source; compiler/environment differences can change a rebuilt binary.

From the worktree root:

```sh
ssh codex-msc-math.sbx 'cd /workspaces/msc-math/.worktrees/ds-retrospective-revalidation && timeout 300 cargo build --release --bin sys-dataset'
python3 docs/ds-retrospective-revalidation/geometry-pilot/run.py
python3 docs/ds-retrospective-revalidation/geometry-pilot/compare.py /path/to/historical/polytope-table.jsonl
```

Runner refuses to overwrite its receipt. For reruns use a separate output/receipt location; keep these retained artifacts. Comparator verifies the historical table's pinned SHA-256 and exact selected IDs. Historical capacity-source labels and missing certification metadata remain; no provenance upgrade is inferred.
