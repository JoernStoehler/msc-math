# Data And Figures Are Traceable

## Use When

Use this packet when checking thesis figures, tables, datasets, experiment
artifacts, or generated outputs that support thesis-facing text.

## Property

Every thesis-used figure, table, dataset, or experiment artifact has a named
source, provenance path, and interpretation that matches what the thesis says.

## Starter Read Set

1. `tasks/reproducibility.md` and `tasks/writing.md`.
2. The thesis source that cites the figure, table, dataset, or experiment.
3. The relevant `research/*.md` interpretation note.
4. The producer under `experiments/` and any `Input Artifacts:` /
   `Output Artifacts:` declarations.
5. `DATAFLOW.md` only when the generated declared-artifact audit is useful;
   targeted grep/local inspection is enough for small questions.

## Checks

1. Name the figure, table, dataset, or artifact under review.
2. Identify producer, consumer, and thesis citation.
3. Check that thesis wording matches one of:
   - rerunnable artifact with command and expected output;
   - preserved historical artifact;
   - illustrative figure not used as evidence;
   - future/cut material.
4. Flag stale provenance, missing producer, missing consumer, freshness risk,
   or overread cache wording.
5. Route failures to `tasks/reproducibility.md`, `tasks/writing.md`, or the
   relevant topic bundle.
