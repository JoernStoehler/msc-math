# Rust Review Checklist

Load `$rust-conventions` first.

Check:
- Referenced formal labels exist in `formal/**/*.tex`.
- The cited formal statement describes the computation in the code.
- Non-trivial mathematical code has a formal reference or a TODO naming the missing reference.
- Mathematical code and orchestration code are distinguished cleanly; glue code is not carrying avoidable formal or abstraction burden.
- Invariants stated in comments are either enforced by code or documented clearly enough that the construction sites can be reviewed against them.
- Interfaces expose the inputs a caller actually needs; they do not force extra precomputed intermediates without a clear reuse or semantic reason.
- Error handling uses the clearest surface: meaningful mathematical case splits are explicit, and violated internal invariants do not fail silently.
- Performance claims include benchmark source, date, and input range.
- Experiment-specific behavior stays in `experiments/` unless Jörn approved migration to `crates/`.

Report missing tests when code changes behavior and no test or experiment run covers the changed branch.
