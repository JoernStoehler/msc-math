# Probing Viterbo's Conjecture

Master thesis repository for Jörn Stöhler, University of Augsburg.

The repository produces:

- `thesis/build/main.pdf`: the thesis;
- `crates/`: reusable Rust libraries for the mathematical computations;
- `experiments/`: reproducible producers, retained evidence, and
  interpretation used by the thesis.

## First entry points

1. Read `ARCHITECTURE.md` to choose the relevant project domain.
2. Read that domain's `README.md`.
3. Inspect the named source, producer, proof note, test, or active thesis file
   before relying on a summarized claim.

Project-wide information:

- `docs/project-status.md`: milestones, current state, and open gates.
- `docs/project-facts.md`: Jörn-confirmed facts and external decisions.
- `docs/capabilities.md`: compact view of what the repository can currently
  support.
- `docs/reproducibility.md`: code, data, and archive route.
- `submit/README.md`: submission sources.

Domain entry points:

- `thesis/README.md`
- `formal/README.md`
- `experiments/README.md`
- `crates/README.md`
- `papers/README.md`

`README.md` and `docs/capabilities.md` are navigation views. The relevant
source files, tests, data, proof notes, producer outputs, active thesis text,
and accepted Jörn/Kai decisions are authoritative.

## Current outcome

The final state is not merely a compiling PDF. Retained thesis claims must have
support and caveats matching their strength; referenced code, figures, data,
and certificates must resolve; reproduction and archive promises must be true;
submission requirements must be satisfied; and Jörn must accept the thesis as
ready. See `docs/project-status.md`.
