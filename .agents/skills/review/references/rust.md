# Rust Review Checklist

Load `$rust-conventions` first.

Check:
- Referenced formal labels exist in `formal/**/*.tex`.
- The cited formal statement describes the computation in the code.
- Non-trivial mathematical code has a formal reference or a TODO naming the missing reference.
- Invariants stated in doc comments are enforced by constructors, types, assertions, or tests.
- Error handling follows the math-code rule: mathematical cases use enums; impossible cases use assertions; callers match variants locally.
- Performance claims include benchmark source, date, and input range.
- Experiment-specific behavior stays in `experiments/` unless Jörn approved migration to `library/`.

Report missing tests when code changes behavior and no test or experiment run covers the changed branch.
