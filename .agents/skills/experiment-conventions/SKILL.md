---
name: experiment-conventions
description: Experiment package conventions for `experiments/**`, including Rust binaries, Python analysis scripts, figures, Cargo package layout, methodology notes, and links to topic files under `formal/`. Use before designing, editing, running, reviewing, or documenting experiments. Load alongside `python-conventions` for `analyze.py` work and alongside `dataset-conventions` when the task creates, refreshes, audits, compares, or validates tracked experiment datasets or artifact declarations.
---

# Experiment Conventions

## Scope

Experiments answer research questions. They produce data, figures, and evidence for the thesis and for later library work.

Current layout:

```text
experiments/
  figure_config.py
  <topic>/
    Cargo.toml
    src/lib.rs                 # optional package/incubator API index
    <experiment>/
      main.rs                  # Rust binary entrypoint
      *.rs                     # helper modules owned by this experiment
      analyze.py               # optional Python analysis and figures
      *.jsonl, *.csv           # generated data
      *.png, *.tex             # generated figures/tables
```

Formal mathematics for experiments lives in root-level `formal/*.tex` files
named by the formal object or proof cluster, not beside `main.rs`.

## Before Editing Or Running

For run-only tasks, read the local entrypoint, declared artifacts, and nearby
execution notes needed to run the command without changing tracked outputs.
Use the full editing read set below when the task changes code, methodology,
artifact declarations, or interpretation.

1. Read sibling experiments in the same `experiments/<topic>/` package.
2. Read the relevant `research/<topic>.md` note and any narrower `research/<topic>-*.md` note when the task involves methodology, current interpretation, or decision history. Read Markdown inside `experiments/` only when it is execution-facing packet documentation.
3. Grep `formal/*.tex` for the cited label or proof object when the experiment implements or tests a formal claim.
4. Load `$rust-conventions` for `main.rs`, `src/lib.rs`, or tests.
5. Load `$python-conventions` for `analyze.py` or figure/table generation.
6. Load `$formal-math-conventions` for formal statements, labels, or proof updates.
7. Load `$dataset-conventions` as well when the task creates or refreshes `.jsonl` or `.csv` outputs, changes `Input Artifacts:` / `Output Artifacts:` declarations, traces artifact provenance, compares tracked datasets across runs, or depends on dataset freshness or validation.

## Methodology First

For new experiments or changed measurements, write down:
- the research question;
- the quantity being measured;
- the generated data file names;
- the observation that would support or refute the hypothesis;
- the limitations that would make the result non-conclusive.

Do this before implementation. If the method choice changes the thesis direction or compute budget, ask Jörn after preparing concrete options.

## Validation Experiments

Use experiments for slow or broad mathematical checks: algorithm agreement across datasets, random or seeded edge-case searches, validation against literature values, invariant sweeps such as conformality or symplectic invariance, and generated evidence files.

When a validation experiment replaces crate fixture coverage, record the boundary explicitly: crate tests keep small live smoke/regression checks, while the experiment owns broad evidence and data freshness. Put research interpretation in `research/`; keep regeneration and verification commands in execution-facing docs or code comments near the producer.

## Rust Pipeline

- Register binaries in `experiments/<topic>/Cargo.toml`.
- Binary paths use `experiments/<topic>/<experiment>/main.rs`.
- Cargo binary names use hyphens, matching existing package style.
- Most experiments are scripts: put helper `.rs` files beside the binary or in
  the smallest shared parent directory that contains all binaries using them.
- Use `src/` only when the experiment subtree is Rust-heavy enough to act as a
  package, or when it incubates code that may later move to `crates/`.
- When `src/` exists, `src/lib.rs` is an index for named modules. Do not put
  substantial helper code directly in `src/lib.rs`.
- Keep case-specific or exploratory helpers in the binary or workflow that owns
  them even when another binary has a structurally similar pipeline.
- When the split criteria in `$rust-conventions` are met, keep `main.rs` thin
  by routing stable stages into local modules or `src/lib.rs`.
- Exploratory behavior stays in `experiments/`; stable approved algorithms migrate to `crates/`.
- Use semantic experiment paths. Do not reorganize a topic just to make the subtree visually balanced.
- Durable reusable Sage validation belongs under `experiments/verification/sage/`. Topic-local Sage can stay with its producer until it stabilizes into a reusable comparison surface.
- Rust experiment entrypoints follow the Rust rule "pass the data a function actually needs": prefer explicit inputs and flat local structs over oversized context arguments.

Run examples:

```bash
cargo build -p exp-<topic> --release
cargo run -p exp-<topic> --release --bin <binary-name>
cargo build --workspace --release
```

## Python And Figures

- Run analysis scripts with `uv run analyze.py`.
- Python scripts are self-contained and read files from their experiment directory.
- Shared figure styling comes from `experiments/figure_config.py`.
- Generated thesis-facing figures should be readable at thesis text width.
- Captions in generated `.tex` tables or thesis text state observations; interpretations belong in body text.

## Reporting Results

- Numerical claims cite their source inline: file name, row id, command, or script output.
- Label speculation as interpretation.
- Record dead ends with the reason they failed so future agents do not retry them. Prefer a terse `research/*.md` history entry over a long chronological dump.
- Keep thesis-facing conclusions aligned with the relevant `research/` note,
  `tasks/*.md` obligations, and `formal/` source.
