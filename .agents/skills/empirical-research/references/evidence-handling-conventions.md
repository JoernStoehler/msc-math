# Evidence-Handling Conventions

These are binding operating conventions.

- Generated artifacts/producers own detailed metrics. Regenerate rather than
  hand-edit or duplicate metric rows. Identify unexpected tracked changes
  before evidence use.
- Preserve the command, inputs, parameters, seeds/selection rules,
  non-obvious dependencies, and comparison contract needed for the intended
  claim or rerun. Required sources must be recoverable, not only untracked or
  absolute local paths.
- A producer run must not fall through from a failed build to a stale binary.
  Concurrent producers use separate outputs; merge only validated results.
- Before irreversible target evaluation, freeze the actual evaluator, source,
  dependencies, and inputs in a recoverable state. Material changes require a
  narrow recheck before exposure.
- Keep durable evidence and interpretation with the narrowest owner future
  consumers need. Publication-facing assets route through `$thesis`.
