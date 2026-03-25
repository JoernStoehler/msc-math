# CLAUDE.md

## Project

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: mid-April 2026.
Topic: Probing Viterbo's Conjecture

Three planned deliverables:
1. A printed-quality LaTeX thesis (`thesis/build/main.pdf`)
2. A high-performance Rust library for symplectic geometry on polytopes (`crates/`)
3. A reproducible experiment pipeline (`experiments/`)

## Project Layout

```
crates/                    Rust library (the core)
  src/
    lib.rs                 crate root
    geom/                  polytopes and basic euclidean and symplectic geometry
    kkt/                   general KKT solver
    algorithms/            different algorithms for the EHZ capacity 
    derivatives.rs         derivative of the capacity in the dual vertices
    dataset.rs             polytope datasets
  main.tex                 correctness proofs for the entire library (includes subfolder math.tex files)

experiments/               each experiment is a self-contained directory
  <name>/
    run.rs                 binary to create the data files
    *.jsonl, *.csv         data files
    analyze.py             postprocessing, analysis, figures and tables
    logbook.md             experiment logbook, what was done, results, learnings, ideas
    math.tex               correctness proofs for the experiment
    thesis.tex             writeup of the experiment takeaways for the thesis

thesis/
  main.tex                 master document, includes chapters and experiment writeups
  *.tex                    chapter files
  bibliography.bib         citations
  build/                   latexmk output

papers/<abreviationYear>/*.tex  arXiv paper sources for reading
handoffs/*.md              task handoff files for future sessions

TASKS.md                   master task list, project management
CLAUDE.md, .claude/        extra information for claude code agents
```

**Key architectural patterns:**
- math.tex files live alongside code, not in thesis/. They contain proofs and derivations that back the code. Thesis chapters reference these but don't duplicate them.
- Each experiment is self-contained: own binary, own data, own logbook, own math. No shared state between experiments.
- The library (`crates/`) is the single source of truth for computation. Experiments call into it.
