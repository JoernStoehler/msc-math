# Thesis layout and summary wording packet

Observed: 2026-10-04 00:36 UTC. Owner: `/root/integration/thesis_layout`.
Receiving integration owner: `/root/integration`.
Base: preservation checkpoint `fe3475b3`; all checkpoint work preserved.
No commit was made by this worker.

## Changes

- `thesis/chapters/01-introduction.tex`: changed the nonsmooth-capable EHZ
  description to “closed generalized characteristic”, matching
  `02-preliminaries-ehz-capacity.tex` and its least-action characterization.
- `thesis/chapters/13-numerics.tex`: shortened the artifact-location sentence,
  made the two packet names breakable paths, and made the path-heavy cube
  footnote locally ragged right. No numerical or reproduction claim changed.
- `thesis/chapters/14-code-data.tex`: reflowed the sentence identifying the
  two external dataset files. Their registered/external status and the plain
  checkout limitation remain explicit.
- `thesis/chapters/16-conclusion.tex`: replaced the ambiguous “the two
  theorems” with the explicit HKO-local and affine-pentagon antecedents.
- `thesis/appendices/data-science.tex`: by integration-owner authorization,
  placed the two long retained-evidence paths following the largest-ratio
  example in a small ragged-right display. The words and paths are unchanged.

The preexisting chapter 13 and 14 overflows were 20.81627 pt and 35.26376 pt;
the integration owner's reproduced baseline also had a 12.36205 pt appendix
overflow. The revised isolated build has none.

## Build and PDF inspection

The PDF skill was read. Its marker succeeded exactly once immediately before
the first edit, using the installed skill's `container_tools/` script:

```sh
node /home/joern/.codex/plugins/cache/openai-primary-runtime/pdf/26.905.11957/skills/pdf/container_tools/mark_artifact_operation_started.mjs --operation-kind edit --expected-output-count 1 --output-format pdf
```

The thesis directory was copied, excluding `build/`, to the isolated snapshot
`/tmp/msc-thesis-layout-20261004/source/`. This avoided the shared final build.
The final build command, run from that snapshot, was:

```sh
TEXINPUTS=.: BIBINPUTS=.: latexmk -norc -pdf -interaction=nonstopmode -halt-on-error -auxdir=../build -outdir=../build main.tex
```

Result: exit 0, 101 pages. The final `main.log` contains zero occurrences of
`Overfull`, `Underfull`, `Warning`, or `undefined`.

PDF: `/tmp/msc-thesis-layout-20261004/build/main.pdf`.
SHA256: `8d1e4562a5668bfd41952ae8a622bea23a7db8520ca31c599c652457ab7267c1`.
The source inventory is `/tmp/msc-thesis-layout-20261004/source-manifest.json`,
SHA256 `b585018c40ad9cc99edf05b1c237271dee9d0a4ac0b12b05d2b3fe04137361bf`.

Pages 6, 78, 79, 80, 84, and 87 were rendered individually using:

```sh
pdftoppm -f PAGE -l PAGE -scale-to 1500 -png -singlefile /tmp/msc-thesis-layout-20261004/build/main.pdf /tmp/msc-thesis-layout-20261004/render/page-PAGE
```

All six complete page images were visually inspected. The changed prose,
path references, equations, footnotes, margins, and page numbers were legible
and fitted cleanly. This is inspection of the affected pages of this exact
snapshot, not whole-PDF independent acceptance.

`git diff --check` passed. At 00:36 UTC all five owned live thesis files
matched the isolated source byte for byte. Their SHA256 values were:

| Thesis-relative source | SHA256 |
| --- | --- |
| `chapters/01-introduction.tex` | `aaa4a884bc9daf7a64fc3c801f3c032f16a4ead34c3459b65a83b472326bd174` |
| `chapters/13-numerics.tex` | `fc9443de07f6c536df010ee8de43a87aecf636034354d5b14706721ab115ad11` |
| `chapters/14-code-data.tex` | `cb7d1dfde12c3a92c0784e71412a415bf0eda369591bd0313472c614bb746c0a` |
| `chapters/16-conclusion.tex` | `eb673aee108c065fba235d2f67dff918724d485eb0297aaf14d5a2f712389bbb` |
| `appendices/data-science.tex` | `fdbb0fb8af8580cf9f17463f6f06e23e5611f432fec0b935e95a094c4cb4f1e7` |

## Evidence boundary and remaining integration

The numerical packet reports and the historical four-test correspondence
receipt were inspected for consistency with the reproduction language. The
general audit's absent original per-system output, the product audit's retained
88 rows, the externally registered population inputs, and the finite scope of
the correspondence tests remain distinguished. Every repository-root path
inside a `\path{}` command in chapters 13/14 and the DS appendix resolved
locally; this does not establish public availability or clean-checkout
materialization. No producer or numerical test was rerun by this worker.

The numerical contract repair is a separate integration packet. If it changes
the correspondence test set or intended action-window account, the integration
owner must reconcile the final chapter 13 description with that result. The
isolated PDF is a QA intermediate: final acceptance must use the assembled
candidate and its exact source/PDF hashes. No theorem acceptance, external
publication, archival release, or human PASS is inferred from this packet.
