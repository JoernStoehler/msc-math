# Probing Viterbo's Conjecture

Master thesis repository for Jörn Stöhler, University of Augsburg.

The repository produces:

- `thesis/build/main.pdf`: the thesis;
- `crates/`: reusable Rust libraries for the mathematical computations;
- `experiments/`: reproducible producers, retained evidence, and
  interpretation used by the thesis.

## First entry points

`ARCHITECTURE.md` maps the domains; their READMEs provide search cues and local
context. Start from the question: search a claim, symbol, artifact path or topic
across relevant domains, or use an entry point when the terminology is unfamiliar.
The entries below are alternatives, not a required reading sequence.

Project-wide information:

- [Current work and ownership](memories/todos.md): live assignments, proposals
  and known scope decisions; historical research plans are not the task queue.
- `INSTALL.md`: environment setup and reproduction entry point; known working
  tools, current gaps, Sage, LaTeX, R2 and Micro configuration.
- `docs/project-facts.md`: Jörn-confirmed facts and external decisions.
- [`docs/algorithm-testing.md`](docs/algorithm-testing.md): fast development
  checks, slower confidence layers, command side effects, and current gaps.
- `docs/reproducibility.md`: code, data, and archive route.
- `docs/artifacts.md`: materializing and publishing shared R2 snapshots.
- `docs/development-environments.md`: execution environments, clients,
  toolchain contracts, and current setup boundaries.
- `submit/README.md`: submission sources.

Domain entry points:

- `thesis/README.md`
- `formal/README.md`
- `experiments/README.md`
- `crates/README.md`
- `papers/README.md`

`README.md` is a navigation view. Follow claims to their source files, proof
arguments, producer outputs or attributed decisions; a summary or path alone
does not establish correctness.

## Current outcome

The final state is not merely a compiling PDF. Retained thesis claims must have
support and caveats matching their strength; referenced code, figures, data,
and certificates must resolve; reproduction and archive promises must be true;
submission requirements must be satisfied; and Jörn must accept the thesis as
ready.
