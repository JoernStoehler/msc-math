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
