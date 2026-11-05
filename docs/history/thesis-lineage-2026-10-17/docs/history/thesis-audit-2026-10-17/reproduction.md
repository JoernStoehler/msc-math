# Bounded reproduction receipt — 17 October 2026

**Option A passed:** the exact fixed candidate sources compile in a new source/build directory inside a disposable second TeX environment, and all release-marked registered inputs download into an initially absent cache and pass complete integrity checks. No empirical producers were rerun.

## Candidate and isolated build

The source freeze at 2026-10-17T08:06:55.021614+00:00 contains 64 current tracked thesis files, copied from working bytes after the chapter fixes and affiliation correction. The base commit is `51ae8e2b6c7d4c6a791ce958067874d6a449f1e6`; the audit edits were still uncommitted at freeze, so the exact file list and hashes in [final-source-inventory.json](final-source-inventory.json) identify the tested source. Its inventory SHA-256 is `76e6d4fe86cdf9b35565bed62784c6f83c43397829e86229aca9b7f256319ae9`. The source copy had no `thesis/build/` directory before execution.

`timeout 180s bash thesis/check-build.sh` passed in **23.95 seconds**, producing **105 pages**, no undefined references/citations, and no overfull horizontal boxes exceeding 1 pt. The host candidate remains unchanged by this check:

| PDF | SHA-256 | Bytes |
| --- | --- | ---: |
| Final host candidate, `thesis/build/main.pdf` | `bfbe9d25547b0262d62df26e33bdd33a94887a5c0d0bc470138af97e2ce8cff8` | 1,784,097 |
| Independent isolated build | `4513109ce7bf021583604226a849b7d47244bc99fd7e72df5e962b87e1b7867c` | 1,769,205 |

The existing image is `msc-math-sbx:local`, immutable image ID `sha256:f61a94c452ec20c17a9ff338601a9ade32326be8928fd13239fae03e7367f295`. A new container received only the already documented `lmodern` dependency and its three package-manager dependencies; the image and host installation were not changed. Networking was detached before building, with 2 CPUs, 2 GiB memory and a 256-PID limit. The container was removed and its absence verified. This is an isolated build in a second existing tool environment after a bounded dependency repair, **not a fresh-machine installation**.

| Tool | Host candidate | Isolated build |
| --- | --- | --- |
| TeX Live / pdfTeX | 2023/Debian / 1.40.25 | 2025/Debian / 1.40.28 |
| Biber | 2.19 | 2.21 |
| latexmk | 4.83 | 4.87 |

The PDFs and extracted layout text are not byte-identical. After Unicode NFKC, soft-hyphen removal and whitespace collapsing, there are six token differences on pages 15, 28, 49 and 52. Paired renders of all four pages confirm fraction/glyph extraction order and matrix spacing differences; the displayed fractions, not-equal signs and entries agree. The table of contents is byte-identical; all **174 label key/displayed-number/page tuples match**. Hyperref's internal equation destination names differ by version, with no undefined-reference or duplicate-destination warning. This comparison supports structural and inspected-content agreement, not complete raster identity or mathematical correctness.

The [machine receipt](reproduction-receipt.json) records commands, constraints, versions and comparison details; [selected build output](reproduction-build-check.log) retains the command result, tool versions and full-log hashes. Full temporary logs, PDFs and page renders are under `/tmp/msc-math/thesis3/20261017/reproduction`.

## Empty-cache registered-input check

All **12 `release: true` snapshots**, comprising **21 payload files / 748,982,368 bytes**, were retrieved from authenticated R2 access into a previously absent `/tmp/msc-math/thesis3/20261017/reproduction/empty-cache`. Manifest-only inspection established the transfer size before downloading. The read-only materializations and repeated offline checks all passed in **60.80 seconds**:

```bash
python3 scripts/artifacts.py materialize ARTIFACT \
  --cache-root /tmp/msc-math/thesis3/20261017/reproduction/empty-cache --no-link
python3 scripts/artifacts.py verify-cache ARTIFACT \
  --cache-root /tmp/msc-math/thesis3/20261017/reproduction/empty-cache
```

For each release-marked artifact, materialization checked the completion manifest's content-addressed snapshot identity, exact file inventory, sizes and SHA-256 payload hashes. `verify-cache` then repeated the local checks without network access. [registered-input-receipt.json](registered-input-receipt.json) retains all snapshot IDs, file manifests and per-artifact results. The only registered non-release snapshot, `covariance-rho-frozen-packet`, was outside this release-input check. Existing caches and repository consumer links were untouched (`--no-link` throughout). Credentials remain outside the receipt.

## What this closes

The current candidate no longer lacks an isolated second-environment source/build check or an authenticated empty-cache integrity check of release-selected registered input objects. Jörn has **no reproduction execution time remaining for this selected bounded option**. Scientific reruns, complete fresh-machine provisioning and eventual release publication remain separate scopes; this receipt makes no promise that they occurred. The retained figure inputs were compiled, not regenerated, and successful compilation does not validate proofs or prose.
