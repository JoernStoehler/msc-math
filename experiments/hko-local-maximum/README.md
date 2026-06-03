# HKO Local Maximum

This topic directory mixes theorem-facing exact work and supporting evidence.

Start with `research/hko-local-maximum-status.md` before reading subfolders.

## Directory Roles

- `exact-clarke/`
  The intended theorem route for the `M_10` result. This is where the exact
  witness contract, exact artifacts, and independent Sage verification live.
- `gradient-analysis/`
  First-order numerical support and gradient/orbit bookkeeping.
- `second-order/`
  Older flat-direction curvature evidence. Keep this as supporting evidence,
  not as the preferred final theorem route.
- `perturbation-neighborhood/`
  Random local perturbation evidence in the fixed `F=10` neighborhood.
- `facet-splitting/`
  `F=10 -> 11` ambient-space falsification attempts.
- `cut-and-ascent/`
  Cut-then-ascent falsification attempts beyond the fixed `F=10` cell.
- `lagrangian-boundary/`
  Local `sys > 1` neighborhood geometry in the Lagrangian-product parameter
  surface.
- `sage-validation/`
  Sage cross-checks for existing exact row-bank artifacts; not the final
  theorem certificate by itself.
- `active-branch-diagnostic/`
  Rust diagnostic for branches active at HKO2024, their `D_a sys` rows, the
  symmetry tangent directions, and numerical slice/cone checks. This is
  theorem-route triage, not the final certificate.
- `subdifferential-lp/`
  Historical or inactive route from the older `(n,h)` parameterization. Read
  only when reconstructing provenance.
- `src/`
  Topic-local shared Rust helpers.

## Rust Command Contract

- `hko-gradient-analysis -- --smoke` writes
  `gradient-analysis/hko-neighborhood-sensitivity-smoke.jsonl`; full mode
  writes the tracked sensitivity/ascent outputs. `--exact-bank` defaults to
  smoke output; add `--canonical` only when refreshing the tracked exact bank.
- `hko-facet-splitting -- --smoke` writes
  `facet-splitting/hko-neighborhood-splitting-smoke.jsonl`; full mode writes
  `facet-splitting/hko-neighborhood-splitting.jsonl`.
- `hko-lagrangian-boundary -- --smoke` writes smoke-level search outputs; full
  mode writes `lagrangian-search*.jsonl`.
- `hko-lagrangian-probe -- --smoke` writes
  `lagrangian-boundary/lagrangian-probe-smoke.jsonl`; full mode writes
  `lagrangian-boundary/lagrangian-probe.jsonl`.
- `hko-perturbation` defaults to an untracked temp smoke output. Use `--out`
  only when intentionally choosing the output path.
- `hko-second-order -- --smoke` runs the phase-1 probe without writing tracked
  outputs; full mode writes `second-order/*.jsonl`.
- `hko-cut-and-ascent -- --smoke` writes `cut-and-ascent-smoke.jsonl`; full mode
  appends to `cut-and-ascent.jsonl` unless `--fresh` is given.
- `hko-sage-validation` defaults to smoke input. Use `--canonical` only when
  refreshing `sage-validation-input.jsonl`.
- `hko-active-branch-diagnostic` writes an ignored smoke JSON by default and
  exact-checks no branches unless `--exact-limit N` or `--all-exact` is passed.
  Use `--canonical` only when intentionally refreshing the tracked diagnostic
  artifact. Each f64 active branch row includes `kkt_f64.singular`; theorem
  use of `d_sys_flat_f64` as a smooth-gradient witness should ignore singular
  KKT rows unless a separate nonsmooth or family-gradient argument is supplied.

## Fast Reading Order

1. `research/hko-local-maximum-status.md`
2. `research/hko-local-maximum.md`
3. `research/hko-local-maximum-exact-clarke.md`
4. `exact-clarke/`
5. supporting-evidence directories as needed

## Rule Of Thumb

If a question is "what proves the theorem?", start in `exact-clarke/`.

If a question is "what evidence supports the local-maximality story?", read the
other experiment folders after the status note.
