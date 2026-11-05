# Reproduction metadata for the selected thesis

Inventory date: 16 October 2026. This document collects recoverable execution
metadata for the selected manuscript's producers. It supplements the
[reproduction contract](reproducibility.md) and
[thirteen-figure runbook](figure-reproduction.md), including its publication-copy
and isolated PDF steps. It does not select new empirical runs or record human
acceptance of remaining reproduction limits.

The inspection covered those contracts, the selected code/data chapter, the
linked October build/figure/exact-check receipts, the retained random/product
lineage, September restoration and historical-association packets, P2, the
twelve-start panel, generated-candidate and numerical runbooks, and the named
LICCA helpers. It was a source inventory: no producer, installation, download,
cluster command or external service was run. No current machine specification
was measured. A machine or version observed in a dated receipt below describes
that run, not this checkout's present execution environment.

## Recorded environments and timings

Times are measured observations rounded for readability, not future runtime
estimates. Compilation, dependency retrieval and analysis are included only
where the receipt says so. CPU model and installed/allocated RAM are unknown
for the host rows unless explicitly stated.

| Work and dated evidence | Recorded environment | Recorded time and scope |
| --- | --- | --- |
| [Original non-HKO exact run and clean-extraction replay](../topics/variation-and-local-maxima/hko-local-maximum/non-hko/README.md#historical-evidence), 22 September | Nested packet members `evidence/exact/RUN.json` and `evidence/reproduction/RUN.json`: Python 3.13.5, SymPy 1.14.0, Linux 6.18.44 x86_64, glibc 2.41 | 6.91 s and 7.75 s for the exact verifier. Core source/witness hashes and timestamps are retained; no discovery rerun is needed. |
| [Non-HKO exact rerun](history/thesis-restart-2026-10-04/non-hko-acceptance/exact/RUN.json), 4 October | Python 3.13.3, SymPy 1.14.0, Linux 6.8.0-146 x86_64, glibc 2.39 | 12.53 s. Exact rational and rational-function checks; not the analytic neighborhood argument. |
| [HKO appendix and full verifier](history/thesis-continuation-2026-10-04/hko-execution/receipt.json), 4 October | Existing Conda Sage environment: Sage 10.9, Python 3.13.15, GCC 15.3.0; optimization disabled | 7.11 s appendix, 6.77 s full verifier, including their `conda run` invocations. Full summary bytes matched the retained summary. |
| [Six empirical graphics](history/thesis-continuation-2026-10-04/figure-reproduction/receipt.json), 4 October | Driver Python 3.12.3, uv 0.12.11, Poppler 24.02.0, Pillow 10.2.0. Association PDF reports Matplotlib 3.10.8; conditional/calibration/derivative PDFs report 3.11.2; fresh rotation PNG reports 3.11.2 | Association 12.21 s; conditional 3.62 s; HKO calibration 4.29 s; derivative 2.84 s; rotation 1.94 s. These are plot/analysis invocations on retained inputs, not dataset generation. |
| [Seven structural graphics and three data exports](history/thesis-continuation-2026-10-04/figure-reproduction/structural/receipt.json), 4 October | Python 3.13.3, Matplotlib 3.11.2, NumPy 2.5.3, uv 0.12.11, rustc/Cargo 1.94.0, Node 22.23.2, Playwright 1.61.1, Three 0.128.0, Chromium headless shell 149.0.7827.55 revision 1228 | Commands, versions, source hashes and successful comparisons retained; per-step elapsed times are **not recorded** in this receipt. The screenshot route uses explicit SwiftShader and lost-context/blank-canvas checks. |
| [Source-only PDF rebuild](history/thesis-restart-2026-10-04/source-only-rebuild.json), 4 October | Installed host TeX environment; 61 manifest-listed thesis files, fresh source directory | Build 11.18 s; complete check 33.79 s; 100 pages. This is an older candidate. |
| [Regenerated-graphics PDF rebuild](history/thesis-continuation-2026-10-04/regenerated-assets-rebuild.json), 4 October | Host route, Latexmk 4.83, Poppler 24.02.0; fresh source copy and all thirteen regenerated graphics | Build 11.34 s; complete check 33.49 s; 101 pages. It does not cover subsequent manuscript revisions. |
| [Second TeX environment after dependency repair](history/thesis-continuation-2026-10-04/cross-environment-repaired.json), 4 October | Image SHA and package changes retained; 2 CPUs, 2 GiB, 256 PID cap; TeX Live 2025/Debian, pdfTeX 1.40.28, Biber 2.21; network disconnected before build | Complete container/setup/build 22.64 s, of which build 10.84 s. `lmodern` plus three dependencies were installed after the first attempt failed. Existing image, not a full fresh-machine installation. Host candidate pdfTeX was 1.40.25. |
| [Current-schema P2 input rebuild](../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/README.md#current-status), 5 September | Existing host/cache; about 11 host CPU cores observed; hardware/tool versions not recorded there | 364.92 s for 14,336 rows in each derived table, expected hashes matched; no analyzer ran. The load was objectionable. Current helper defaults of one Rayon worker and one Cargo build job are a later concurrency contract, not the old run's configuration. |
| [Retrospective geometry/target restoration](../topics/empirical-studies/retrospective-revalidation/README.md), 18 September | Separate current-source/binary/input receipts linked beside each stage | Feature rebuild 586 s on four threads; 14,336 original-body requests 175.28 s; repair 1.42 s plus about 65 s adapter build. These are later corroboration runs, not recovered June execution metadata. |
| [Historical first-order panel](../topics/empirical-studies/sys-landscape/gradient-ascent-observed-general/README.md#direct-results) | Historical schema v1, twelve seeds 42–53, retained commit and producer/analyzer hashes in runbook; hardware and compiler unknown there | Aggregate recorded compute 400.889 s, 204 finite-step evaluations, 405,772 capacity-orbit iterations. This is panel compute, not a measured clean build plus end-to-end rerun walltime. |

The [historical association replay](../topics/empirical-studies/evidence-closure/README.md#what-was-actually-checked)
records exact matching headline values and Spearman coefficients, with 31
numeric leaves differing by at most 6.83e-13. Its
[receipt](../topics/empirical-studies/evidence-closure/replay-receipt.json)
explicitly records no original replay timing instrumentation; no walltime is
recoverable from that receipt.

## LICCA route and missing original-run facts

The [retained numerical lineage](../topics/empirical-studies/polytope-datasets/retained-lineage.md)
binds the June random/product payloads and derived plotting table to immutable
hashes and Git history. It does **not** bind those bytes to a particular LICCA
job, clean executable revision, toolchain, fresh computation or cache state.
Provenance labels identify preparation/source roles and cannot supply that
missing execution receipt.

Recoverable job *designs* are:

- [Historical post-feature table rebuild handoff](../topics/empirical-studies/sys-datascience/legacy/licca-post-feature-rebuild.md):
  `sbatch --parsable experiments/polytope-invariant-table/licca-build-retained-table.slurm.sh`,
  a single Rayon process, requested `epyc`, 32 CPUs, 32G, two hours;
  `sacct` fields for elapsed time, allocation, CPU time, MaxRSS and exit status.
  These old paths/checkout/materialization instructions are historical.
- [Current-location retained-table helper](../topics/empirical-studies/polytope-invariant-table/licca-build-retained-table.slurm.sh)
  preserves that request. The
  [random refresh helper](../topics/empirical-studies/polytope-datasets/licca-refresh-random.slurm.sh)
  requests `epyc`, 32 CPUs, 16G and two hours, builds two release binaries,
  copies an existing cache when present, and writes separate review targets.
- [Run-local producer](../topics/empirical-studies/polytope-datasets/licca-datascience-produce.slurm.sh)
  and [table preparation](../topics/empirical-studies/polytope-invariant-table/licca-datascience-prepare.slurm.sh)
  preserve alternative explicit-output wrappers. The
  [LICCA status owner](../topics/empirical-studies/sys-datascience/LICCA.md)
  classifies all these helpers as dormant.

Requests and comments about a node's core count are not observed allocations,
actual CPU models, installed memory, job durations or proof of execution.
No LICCA access is required for the local plot/exact-check routes below.

**TODO — original execution metadata:** locate a retained job ID/log or run
manifest tying the June source payload hashes to its actual command, source and
lockfile identity, compiler/library versions, cache inputs, node CPU model,
allocated CPUs/RAM and per-step elapsed time. Those facts are absent from the
inspected lineage and handoff sources; their recoverability is unknown. Record
an unresolved historical limit if they cannot be recovered. Current workstation
specifications or later restoration timings cannot fill this gap.

## Comparison classes and local entry points

An observed match is scoped to its source, inputs and environment. The table
specifies useful comparison targets already supported by retained contracts;
it does not record Jörn's acceptance of them as the final two-year package.

| Artifact class | Supported comparison and execution entry |
| --- | --- |
| Registered inputs | Exact manifest inventory, sizes and SHA-256 hashes: `python3 scripts/artifacts.py verify-cache polytope-invariant-table` (offline) and the corresponding `polytope-datasets` entry. Empty-cache authenticated retrieval remains a separate unchecked operation; see [artifact contract](artifacts.md). |
| HKO finite certificate | Exact predicate pass and expected counts/ranks; October full summary was byte-identical. Copy verifier plus witness to scratch before executing, as shown below. Candidate discovery need not be repeated. |
| Non-HKO finite certificate | Exact rational/rational-function predicates and independent audit; compare mathematical fields, excluding execution timing/environment. [Packet commands](../topics/variation-and-local-maxima/hko-local-maximum/non-hko/README.md#reproduction) use SymPy 1.14.0, assertions enabled and fresh output. |
| Three flow/viewer JSON exports and two viewer PNGs | Byte identity observed in October with pinned sources/tool versions. Full export and screenshot commands are in the [figure runbook](figure-reproduction.md#structural-figures-and-visualization). The screenshot producer also rejects lost contexts and blank canvases. This is not a cross-browser guarantee. |
| Other selected figures | All six empirical plot pairs had identical rendered pixels in the recorded comparison, despite differing file bytes. Five structural PDFs had matching text and 144-dpi rasters. Use the receipt's renderer/resolution and inspect labels/crops; PDF metadata and PNG software metadata can change bytes. The [complete figure map](figure-reproduction.md#complete-figure-map) supplies every producer/input/copy name. |
| Final PDF | Same-host fresh-copy builds matched layout text and all 96-dpi page rasters for the dated candidates, with differing PDF bytes. The second TeX environment built 101 pages without overfull boxes or undefined references, but [comparison](history/thesis-continuation-2026-10-04/cross-environment-comparison.json) found different rasters on 91 pages and layout text on eight. Cross-version layout equivalence is **not established**. Current candidate: `sh thesis/build.sh`; force/check with `bash thesis/check-build.sh`. |
| Historical 39-feature associations | [Replay wrapper](../topics/empirical-studies/evidence-closure/replay-historical.sh) pins historical code/input hashes. Exact headline/rank comparison plus explicitly listed floating-point differences is the observed contract; no universal numeric tolerance is approved. |
| P2 45-feature input/models | [P2 runbook](../topics/empirical-studies/sys-datascience/methods/standard-baseline-p2/README.md#command) records exact expected derived table hashes and single-thread BLAS analysis command. Historical 39-feature plotting input is unsuitable. Table rebuild/model execution requires a scoped resource budget; model output equivalence across library versions is unverified. |
| Twelve-start historical panel | [Historical source-to-raw procedure](../topics/empirical-studies/sys-landscape/gradient-ascent-observed-general/README.md#fresh-session-reproduction-contract) requires the recorded commit, all twelve validated seeds and exact *within-run* raw concatenation. Elapsed time and absolute input-directory fields prevent historical byte comparison. Current schema v2 is a different experiment and cannot replace v1 evidence. |
| Generated-candidate compact packet | [Runbook](../topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md) retains frozen config, 1,675 pre-target/evaluated rows and reports. Current evaluator emits v3 while historical rows are v2; geometry/feature caches were omitted. A current full run is not a byte-reproduction claim for the historical packet. |
| Numerical QP/product audits | [General report](../topics/capacity-and-qp/development/tools/general_algorithm_ablation/RESULTS.md) and [runner](../topics/capacity-and-qp/development/tools/general_algorithm_ablation/run.sh) retain the 249-system and 88/1,271-word contracts; original per-system transcripts are absent. [Product runbook](../topics/capacity-and-qp/development/tools/product_closure_route/README.md#reproduce) retains 88 cases row by row. Compare nonempty cohorts, exact agreement/interval/sign failure counters and scope; timings vary. Neither report supplies original CPU model or an approved cross-machine timing threshold. |

From the repository root, these local routes preserve selected evidence:

```bash
mkdir -p /tmp/msc-math/reproduction-metadata
metadata_run=$(mktemp -d /tmp/msc-math/reproduction-metadata/run.XXXXXX)
mkdir "$metadata_run/hko"
cp topics/variation-and-local-maxima/hko-local-maximum/theorem/verify.sage.py \
   topics/variation-and-local-maxima/hko-local-maximum/theorem/witness.json \
   "$metadata_run/hko/"
PYTHONOPTIMIZE=0 conda run --name sage python "$metadata_run/hko/verify.sage.py"
cmp "$metadata_run/hko/verification-summary.json" \
  topics/variation-and-local-maxima/hko-local-maximum/theorem/verification-summary.json
PYTHONOPTIMIZE=0 conda run --name sage python thesis/appendices/hko_core.py
sh topics/empirical-studies/evidence-closure/replay-historical.sh \
  "$metadata_run/historical-association"
```

These commands are transcribed from retained successful routes and checked
against current paths/source interfaces; they were not executed for this
inventory. The historical replay needs cached registered input and Git objects
for its pinned historical code. Conda/Sage setup is described in the
[installation workflow](../meta-topics/development-environment/installation-workflow.md).

**TODO — final comparison decision:** identify the agreed remaining workloads,
resource budget and acceptable historical gaps; state the required non-byte
comparison for each selected floating-point/model/PDF result. Existing observed
matches alone do not settle that acceptance. Fresh environment installation,
empty-cache retrieval, original empirical producer replay and cross-version
model equivalence remain separately scoped checks in
[distance to submission](../thesis/DISTANCE-TO-SUBMISSION.md#reproduction-and-evidence-closure).
