# Thesis Code And Data

This is the repository entry point for the code and data claims made by the
thesis. Detailed producer commands stay with the producing experiment; the
repository does not maintain a second script that reruns every executable.

Start with [INSTALL.md](../INSTALL.md) for environment setup and the initial
reproduction checks. This file maps thesis results to their producing sources
and describes the release bundle.

## Build the thesis

```bash
sh thesis/build.sh
```

The selected manuscript output is `thesis/build/main.pdf` and is
Git-ignored. This command compiles the selected candidate; it does not establish
mathematical correctness or prose acceptance. The release packager uses
`thesis/check-build.sh`, which forces a rebuild and checks the PDF signature,
overfull boxes and undefined references. Publication is a separate decision;
neither a successful build nor this document records a released thesis.

On 4 October 2026, a [source-only rebuild check](history/thesis-restart-2026-10-04/source-only-rebuild.json)
copied only the candidate manifest's 61 tracked thesis files into a fresh empty
directory and ran this command using the installed host TeX environment. The
100-page result matched the retained candidate's extracted layout text and
every page raster at 96 dpi; PDF bytes differed. This checks the selected
source-to-PDF build without relying on existing build files. It does not test
a new tool environment, cross-version byte identity, or regeneration of the
figures and empirical data.

## Mathematical results and exact checks

The selected “Availability of Code and Data” chapter distinguishes the current
proof routes from earlier computational certificates:

- [HKO local result](../experiments/hko-local-maximum/theorem/README.md):
  `witness.json`, `verification-summary.json`, and `verify.sage.py`. The finite
  predicate supports the theorem together with the analytic upper-function
  and symmetry-slice argument. The separate appendix presentation is
  [hko_core.py](../thesis/appendices/hko_core.py).
- [Second ten-facet local maximum](../experiments/hko-local-maximum/non-hko/README.md):
  the rational witness, Python/SymPy exact verifier and independent audit.
  The README routes both the retained rerun and original nested archive.
  Exact predicates do not replace the analytic uniform-neighborhood argument.
- [Pentagon rotation and independent linear factors](../formal/pentagon-affine-products/README.md):
  maintained analytic proofs and their primary-source convention check.
  The selected rotation and affine chapters do not depend on the older
  [Sage enumeration packet](../experiments/regular-products/pentagon-rotation-formula-proof/README.md),
  whose classifier and full stdout remain available as the earlier proof route.

Run the commands in the packet READMEs, inspecting their output paths before
execution: the HKO verifier writes its retained summary and the displayed full
pentagon command replaces its transcript. Use a temporary copy or fresh output
path when checking without replacing evidence. The HKO verifier uses explicit
failures; the non-HKO audit and pentagon executable use Python assertions, so
keep optimization disabled. Timing and environment fields are not byte-identical
comparison targets. These are mathematical arguments with executable checks,
not proof-assistant formalizations.

## Retained empirical results used by the thesis

- The bounded data-science result and its retained 14,336-row random/product
  table have their entry point at [sys-datascience](../experiments/sys-datascience/README.md).
  [Retrospective revalidation](ds-retrospective-revalidation/README.md) separates
  recovered source geometries, restored features and current capacity intervals
  for 14,335 bodies from the historical numerical ratios. One body remains
  outside the numerical-size policy; capacity intervals do not certify the
  floating-point volumes or ratios.
- The [generated-candidate experiment](../experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md)
  retains 1,675 selected/control bodies before ratio evaluation and their
  evaluated results. The larger generated pool is not a count of capacity
  evaluations.
- The twelve-start finite first-order experiment has its entry point at
  [gradient-ascent-observed-general](../experiments/sys-landscape/gradient-ascent-observed-general/README.md).
- Numerical comparison reports and their distinct retention limits are mapped
  in [the numerical chapter](../thesis/chapters/13-numerics.tex) and
  [the code/data chapter](../thesis/chapters/14-code-data.tex). A retained summary
  does not imply that every original per-system transcript is included.
- Figure-producing experiments keep their source assets and regeneration
  commands beside the producer. Publication copies under `thesis/` remain
  deliberate because the thesis must build as a self-contained artifact.

These artifacts make the thesis results immediately inspectable. A smoke run
demonstrates plumbing only; it is not a replacement for retained full data or
theorem-facing verification. A plain checkout builds the thesis from its retained
figure inputs, but it does not supply every input needed to regenerate all
figures and empirical analyses. Registered bulk datasets require authenticated
R2 materialization as described in [INSTALL.md](../INSTALL.md).

## Shared data policy

At closure, commit small data useful for immediate interpretation, validation,
or continuation. Put bulk generated data and expensive producer caches in
immutable R2 snapshots registered by `artifacts/registry.json`. Remove
disposable smoke output, superseded intermediates, and cheap caches without a
consumer. See `docs/artifacts.md` for explicit materialization and publication.

Selection happens while curating the final reviewed commit and artifact
registry. After required third-party cleanup, the packager includes every path
tracked by that commit, materializes registry entries marked for release at
their established repository paths, and adds the checked thesis PDF. The
Zenodo ZIP therefore records both the final source tree and the reviewed bulk
data selection rather than depending on Git LFS.

See `submit/archive-closure-checklist.md` for cleanup and publication gates.

## Build the Zenodo ZIP

After final cleanup and review of the exact commit and artifact registry:

```bash
REVIEWED_COMMIT=0000000000000000000000000000000000000000  # literal closure record
python3 scripts/build-release.py \
  --expected-commit "$REVIEWED_COMMIT" \
  --output /tmp/joern/msc-math-release.zip
```

The packager uses committed contents, materializes and hash-checks every
registered release snapshot, forces a checked thesis build, adds the PDF, and
verifies the ZIP inventory and hashes. It does not decide which data are
valuable or whether third-party material may be redistributed; those are
final-tree and registry cleanup decisions.
