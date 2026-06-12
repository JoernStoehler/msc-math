# Gradient Ascent Development Stubs

Status: superseded stub surface. New gradient-ascent method development should
start at `experiments/dev-gradient-ascent/`.

This directory is a placeholder surface for planned gradient-ascent method
development tools. The current binaries are stubs:

- `step-calibration/main.rs`
- `strategy-comparison/main.rs`

They are exposed in two manifests:

- `experiments/sys-landscape/Cargo.toml` as `sys-step-calibration` and
  `sys-strategy-comparison`, so the root package command contract can name them.
- `experiments/sys-landscape/gradient-ascent-dev/Cargo.toml` as
  `dev_step_calibration` and `dev_strategy_comparison`, preserving the local
  incubator package.

They write no artifacts and are not current evidence producers. Use
`experiments/sys-landscape/README.md` for the root command contract and
`research/sys-landscape.md` or `tasks/planning-notes.md` for experiment
purpose.
