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

## Source Truth

Source truth is the Rust code and reproducible command output. `research/`
files may orient future work, but claims here should be checked against code,
formal proof files where relevant, and generated output from the command above.

## Pause Rule

Pause after this prediction-only milestone before adding sigma reuse, local
ascent loops, conditional bounds, canonical artifacts, or performance claims.
If the smoke exposes architecture friction, prefer a small refactor based on
the observed friction over expanding the method surface.
