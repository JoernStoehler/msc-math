# Probing Viterbo's conjecture

Jörn Stöhler's MSc thesis combines mathematical proofs, computational symplectic
geometry and empirical investigations. The manuscript is unfinished and has not
been submitted for final grading.

- **Continue the work:** [docs/WORK_REMAINING.md](docs/WORK_REMAINING.md) is the
  single current work list. [RESUME.md](RESUME.md) records the handoff, session
  identities, acceptance constraints and resource state.
- **Read or edit the thesis:** [thesis/README.md](thesis/README.md).
  Build with `sh thesis/build.sh`; output is `thesis/build/main.pdf`.
- **Find mathematical and computational support:** [formal/](formal/README.md),
  [experiments/](experiments/README.md), [crates/](crates/README.md),
  [papers/](papers/README.md), and the [evidence index](docs/README.md).
- **Set up or reproduce:** [INSTALL.md](INSTALL.md),
  [artifact contracts](docs/artifacts.md), and [test scope](docs/algorithm-testing.md).

`thesis/` contains only the selected manuscript and its build inputs. Superseded
manuscripts remain in Git history, not alongside the active chapters.
`docs/history/` contains dated plans, authoring trials and review records; none
is a current assignment. The current work list distinguishes findings still open
from findings already repaired or withdrawn.

[ARCHITECTURE.md](ARCHITECTURE.md) maps domain ownership. Passing a build or test
does not establish mathematical validity, adequate exposition or thesis acceptance.
