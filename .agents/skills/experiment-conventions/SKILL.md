---
name: experiment-conventions
description: Experiment package conventions for `experiments/**`, including Rust binaries, Python analysis scripts, generated `.jsonl` data, figures, Cargo package layout, and links to topic files under `formal/`. Use before designing, editing, running, reviewing, or documenting experiments.
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
    src/lib.rs                 # optional shared helpers for the topic package
    <experiment>/
      main.rs                  # Rust binary entrypoint
      analyze.py               # optional Python analysis and figures
      *.jsonl, *.csv           # generated data
      *.png, *.tex             # generated figures/tables
```

Formal mathematics for experiments lives in `formal/<topic>/*.tex`, not beside `main.rs`.

## Before Editing

1. Read sibling experiments in the same `experiments/<topic>/` package.
2. Read local topic notes such as `REASONING.md`, `DECISIONS.md`, and `NEXT-STEPS.md` in `experiments/<topic>/` or a parent experiment folder when the task involves methodology or current interpretation.
3. Read the corresponding `formal/<topic>/*.tex` file when the experiment implements or tests a formal claim.
4. Load `$rust-conventions` for `main.rs`, `src/lib.rs`, or tests.
5. Load `$python-conventions` for `analyze.py` or figure/table generation.
6. Load `$formal-math-conventions` for formal statements, labels, or proof updates.

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

When a validation experiment replaces crate fixture coverage, record the boundary explicitly: crate tests keep small live smoke/regression checks, while the experiment owns broad evidence and data freshness. Put the regeneration and verification commands in a local note with a narrow name such as `REASONING.md`, `DECISIONS.md`, or `NEXT-STEPS.md`; do not create giant logbooks.

## Rust Pipeline

- Register binaries in `experiments/<topic>/Cargo.toml`.
- Binary paths use `experiments/<topic>/<experiment>/main.rs`.
- Cargo binary names use hyphens, matching existing package style.
- Shared helpers used by multiple binaries in the same topic belong in `experiments/<topic>/src/lib.rs`.
- Per-experiment helpers stay in that experiment's `main.rs`.
- Exploratory behavior stays in `experiments/`; stable approved algorithms migrate to `crates/`.
- Keep data with the producer. Avoid multiple maintained binaries writing to the same tracked output file.
- Use semantic experiment paths. Do not reorganize a topic just to make the subtree visually balanced.
- Durable reusable Sage validation belongs under `experiments/verification/sage/`. Topic-local Sage can stay with its producer until it stabilizes into a reusable comparison surface.
- Every Cargo binary entrypoint `main.rs` and every experiment `.py` script declares
  `Input Artifacts:` and `Output Artifacts:` in the top doc comment/docstring.
- Use exact repo-relative paths when practical. Use `None` when the file does not
  own or consume repo artifacts. If one declaration line covers a maintained
  family, keep the family explicit and machine-readable.
- `scripts/dataflow.sh` parses these declarations and regenerates `DATAFLOW.md` for the repo's artifact audit.

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

## Data Safety

- `.jsonl` files are generated artifacts and are tracked by Git LFS.
- Do not edit `.jsonl` with patch-style line edits.
- For smoke tests, write temporary output under an untracked temp directory and delete it before finishing.
- For smoke or warmup runs, write temporary datasets under an untracked temp directory and delete them after the run.
- Smoke/default experiment runs should write untracked `smoke-*.jsonl` style outputs unless the caller explicitly requests a canonical refresh path.
- If a compatibility run modifies tracked outputs, restore them before finishing unless the task is explicitly to refresh data.
- If a script touches tracked outputs only for compatibility, restore those paths before finishing.
- If a tracked `.jsonl` changes unexpectedly, stop and report the exact file and command.
- If code is newer than committed data for the same experiment, report the freshness mismatch and regenerate only when the task calls for refreshed results.
- Use `scripts/dataflow.sh` to regenerate `DATAFLOW.md`.
- `DATAFLOW.md` is built from declared `Input Artifacts:` / `Output Artifacts:` headers on experiment Python scripts and Cargo binary entrypoints. Treat that audit as declared artifact ownership plus local timestamp metadata, not as full transitive source provenance.

## Reporting Results

- Numerical claims cite their source inline: file name, row id, command, or script output.
- Label speculation as interpretation.
- Record dead ends with the reason they failed so future agents do not retry them. Prefer a terse `DECISIONS.md` entry over a long chronological dump.
- Keep thesis-facing conclusions aligned with `RESULTS.md` and the relevant `formal/` source.
