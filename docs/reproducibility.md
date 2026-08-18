# Thesis Code And Data

This is the repository entry point for the code and data claims made by the
thesis. Detailed producer commands stay with the producing experiment; the
repository does not maintain a second script that reruns every executable.

## Build the thesis

```bash
cd thesis
latexmk -g
./check-build.sh
```

The checked output is `thesis/build/main.pdf`. It is deliberately Git-ignored
and is added to the final Zenodo ZIP by the release packager.

## Exact certificate packets

The “Published Code And Data” chapter names two theorem-facing packets:

- HKO local result: `experiments/hko-local-maximum/theorem/README.md`, with
  `witness.json`, `verification-summary.json`, and `verify.sage.py`;
- rotated pentagons:
  `experiments/regular-products/pentagon-rotation-formula-proof/README.md`, with
  `executable_proof.sage.py` and `executable_proof.full.stdout.txt`.

Run the commands in those READMEs. The HKO verifier uses explicit failures for
proof-facing checks. The pentagon executable uses ordinary Python assertions,
so do not run it with Python optimization. Its retained stdout contains timing
and is not a byte-identical comparison target.

## Retained empirical results used by the thesis

- The bounded data-science result and its retained 14,336-row random/product
  table have their entry point at `experiments/sys-datascience/README.md`.
- The twelve-start finite first-order experiment has its entry point at
  `experiments/sys-landscape/gradient-ascent-observed-general/README.md`.
- Figure-producing experiments keep their source assets and regeneration
  commands beside the producer. Publication copies under `thesis/` remain
  deliberate because the thesis must build as a self-contained artifact.

These artifacts make the thesis results immediately inspectable. A smoke run
demonstrates plumbing only; it is not a replacement for retained full data or
theorem-facing verification.

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
