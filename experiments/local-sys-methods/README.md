# Local Sys Methods

This package is a cross-result method-development surface for local systolic
ratio methods.

## Purpose

The first question is:

```text
Given local HK/KKT/gradient data at a base polytope a0, how useful is the
first-order Clarke prediction for sys(a0 + t d), compared with full HK2017
recomputation?
```

This package is not thesis evidence by itself. It is not a global
sys-landscape result, not a datascience table method, and not performance
profiling. Global profiling belongs elsewhere.

## Start Here

Run the smoke prediction packet with:

```bash
cargo run -p exp-local-sys-methods --release --bin local-sys-prediction-smoke
```

By default it writes JSONL to:

```text
/tmp/local-sys-methods/smoke-local-prediction.jsonl
```

Pass `--output <path>` to write somewhere else. Do not add generated smoke
output to git unless Jörn explicitly asks for a canonical evidence artifact.

The command exits nonzero if it cannot produce at least one successful row for
the deterministic generic basepoint. HKO rows may succeed or fail, but failures
should be explicit in the row `status`.

## Reading Rows

Each row compares one first-order prediction with one full recomputation:

- `predicted_sys` is `sys0 + t * min_i <grad_sys_i, d>`;
- `recomputed_sys` is from a fresh HK2017 solve at `a0 + t d`;
- `abs_prediction_error` and `rel_prediction_error` compare those two numbers.

This is a heuristic local comparison. The row does not certify a local error
bound for `sys(a0 + t d)`.

The warning fields say how much trust to put in the comparison:

- `active_orbit_count` and `active_action_spread` describe the active orbit set
  used for the Clarke prediction;
- `target_best_sigma_in_base_active_set` says whether the recomputed target
  best sigma was among the base active sigmas;
- `base_candidate_orbit_count`, `base_candidate_action_gap`, and
  `target_best_sigma_in_base_candidate_window` describe a small base action-gap
  candidate window used only as a switch diagnostic;
- `active_min_beta_margin` and `active_max_q_error_bound` summarize the active
  set used for the subdifferential;
- `best_beta_margin` and `best_q_error_bound` are only best-orbit diagnostics.

`q_error_bound` is a KKT `Q` error bound from the capacity solver. It is useful
as a numerical warning. It is not a proven first-order prediction-error bound.

The base candidate window is not a completeness claim over finite distances.
HK2017 can miss a sigma that becomes relevant after moving away from `a0`.
Use the window only as a heuristic signal for whether an observed target switch
was already visible near `a0`.

The HKO pentagon product is included because it is a non-generic stress case.
Many active orbits or target sigmas outside the base active set should be read as
switching or conditioning signals, not as clean local-prediction evidence.

## Source Truth

Source truth is the Rust code and reproducible command output. `research/`
files may orient future work, but claims here should be checked against code,
formal proof files where relevant, and generated output from the command above.

## Pause Rule

Pause after this prediction-only milestone before adding sigma reuse, local
ascent loops, conditional bounds, canonical artifacts, or performance claims.
If the smoke exposes architecture friction, prefer a small refactor based on
the observed friction over expanding the method surface.
