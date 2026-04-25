---
name: dataset-conventions
description: Dataset conventions for tracked experiment artifacts such as `experiments/**/*.jsonl`, `.csv`, and declared input/output artifact headers. Use when a task creates, refreshes, audits, compares, or validates tracked experiment datasets, checks dataset freshness, or traces artifact provenance. Load alongside `python-conventions` or `experiment-conventions` when code or analysis changes are also in scope.
---

# Dataset Conventions

## Scope

This skill covers tracked experiment datasets and artifact declarations. Use it when the task is about generated `.jsonl` or `.csv` outputs, dataset ownership, freshness, smoke-output policy, cross-run dataset comparison, validation of tracked outputs, or artifact provenance.

## Before Touching Data

1. Identify the producer entrypoint or script that owns the dataset.
2. Read the local experiment notes when the dataset is used as evidence for an active research claim.
3. Confirm whether the task is a smoke run, a compatibility check, a canonical refresh, or an interpretation pass over existing data.
4. Load `$experiment-conventions` as well if the task also changes experiment structure, methodology, or binary/script layout.

## Ownership And Declarations

- Keep data with the producer. Avoid multiple maintained binaries or scripts writing to the same tracked output file.
- Every Cargo binary entrypoint `main.rs` and every experiment `.py` script declares `Input Artifacts:` and `Output Artifacts:` in the top doc comment or docstring.
- Use exact repo-relative paths when practical.
- Use `None` when the file does not own or consume repo artifacts.
- If one declaration line covers a maintained family, keep the family explicit and machine-readable.
- There is no generated repo-wide dataflow map. Trace provenance with targeted
  `rg` over `Input Artifacts:`, `Output Artifacts:`, artifact filenames, thesis
  sources, and nearby research notes.

## Generated Data Safety

- `.jsonl` files are generated artifacts and are tracked by Git LFS.
- Do not edit `.jsonl` with patch-style line edits.
- For smoke tests or warmup runs, write temporary output under an untracked temp directory and delete it before finishing.
- Smoke/default experiment runs should write untracked `smoke-*.jsonl` style outputs unless the caller explicitly requests a canonical refresh path.
- If a compatibility run modifies tracked outputs, restore them before finishing unless the task is explicitly to refresh data.
- If a script touches tracked outputs only for compatibility, restore those paths before finishing.
- If a tracked dataset changes unexpectedly, stop and report the exact file and command.

## Freshness And Result Claims

- If code is newer than committed data for the same experiment, report the freshness mismatch and regenerate only when the task calls for refreshed results.
- Numerical claims cite their source inline: file name, row id, command, or script output.
- Label speculation as interpretation.
- When comparing datasets across runs, state whether differences come from changed code, changed parameters, changed seeds, or a stale-baseline mismatch.

## Provenance Search

For an artifact, start with targeted search instead of rebuilding a global map:

```bash
rg -n "<artifact-name>|Input Artifacts:|Output Artifacts:" experiments thesis research tasks
```
