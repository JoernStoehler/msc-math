# Selected manuscript

`main.tex` is the sole thesis entry point. It selects:

- `chapters/`: the abstract, main chapters and their mathematical fragments;
- `appendices/`: DS details and the printed HKO verification program;
- `figures/`: the figures actually used by this manuscript;
- `references/`: the four bibliography sources;
- `ai-use-disclosure.tex`: the short disclosure, distinct from the reflection chapter.

```sh
sh thesis/build.sh
```

Run that command from the repository root. It produces `thesis/build/main.pdf`
using only these tracked source files and installed TeX tools. There is no
`legacy/` search fallback and no alternative draft selected by the build.

Current unfinished work and the limits of past reviews are in
[../meta-topics/project-management/thesis-work.md](../meta-topics/project-management/thesis-work.md). The frozen resumption PDF
at `docs/resume/thesis-resume.pdf` is diagnostic, not an accepted or submitted thesis.

The layout migration is recorded in `docs/resume/layout-migration.json`. It
maps former selected paths to these paths. Other old thesis variants can be read
from commit `35f29db4:thesis/`; they are not current sources. Figure-producing
packets are routed through [topic accounts](../README.md) and
[experiments](../experiments/README.md), with their original evidence contracts.

## 16 October working candidate

The selected symmetric-partner result is now included in
`chapters/10-symmetric-partner.tex`, after the independent-affine result. Its
visible TODO preserves pending human proof and prose acceptance. The existing
candidate proof and figure were integrated without a new research claim.

[Distance to submission](DISTANCE-TO-SUBMISSION.md) lists the remaining work,
rough hours and Jörn dependencies. The [integration record](../docs/history/thesis-integration-2026-10-16/README.md)
binds this working PDF and its bounded checks; it does not replace the frozen
October 4 reviews or establish human PASS.

The [follow-up record](../docs/history/thesis-followup-2026-10-16/README.md)
records the later code/data reference repair and checked PDF. Its
[DS scope comparison](review/data-science-scope-comparison.md),
[evidence map](../docs/thesis-evidence-map.md) and
[reproduction metadata](../docs/reproduction-metadata.md) complete source
inventories while preserving the remaining human decisions and verification TODOs.

## 17 October submission acceleration

The [risk-ranked reading guide](review/JOERN-READING-GUIDE.md) binds exact
pages and acceptance checks to the fixed 105-page candidate. Independent
[chapter and appendix reviews](review/2026-10-17/README.md), the
[audit and bounded reproduction record](../docs/history/thesis-audit-2026-10-17/README.md),
[current official requirements](../meta-topics/thesis-submission/requirements-2026-10-17.md),
and [unsent Kai packet](review/kai-packet/README.md) are prepared.
The updated [distance assessment](DISTANCE-TO-SUBMISSION.md) estimates a
conditional 4.5–6 personal hours to hand-in, including a 3 h 20 targeted
reading/endorsement pass. Jörn and Kai acceptance, signatures and individual
administrative facts remain personal.
