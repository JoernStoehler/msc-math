# HKO Local Maximum

This topic directory separates theorem-facing certificate work from empirical
support checks for the HKO2024 local-maximum result packet.

Start with `research/hko-local-maximum-status.md` before reading subfolders.

## Result Strands

| Strand | Path | Role |
| --- | --- | --- |
| Theorem certificate | `theorem/` | Exact witness generation, Sage-backed row-bank validation, and active-branch diagnostics for the proof route. |
| Empirical support | `empirical/` | Numerical and sampling evidence that supports the local-maximum picture but is not the final proof. |
| Shared Rust helpers | `src/` | Topic-local code shared by theorem and empirical binaries. |

Generated JSON/JSONL/figure artifacts stay beside the producer or sampler that
creates them. Do not create output-only buckets detached from the script or Rust
sampler that owns the artifact.

## Current Layout

```text
experiments/hko-local-maximum/
|-- theorem/
|   |-- exact-witness/
|   |-- active-branch-diagnostic/
|   |-- feasible-section-certificate/
|   `-- row-bank-validation/
|-- empirical/
|   |-- first-order/
|   |-- second-order/
|   |-- m11-ascent/
|   `-- neighborhood-sampling/
`-- src/
```

The old inactive `subdifferential-lp/` route was deleted in the layout
migration. Git history is the archive for that broken `(n,h)` Phase C attempt;
`empirical/second-order/` records why it was replaced.

## Rust Command Contract

- `hko-first-order --smoke` writes
  `empirical/first-order/hko-neighborhood-sensitivity-smoke.jsonl`; full mode
  writes the tracked sensitivity/ascent outputs. `--exact-bank` defaults to
  smoke output; add `--canonical` only when refreshing the tracked exact bank.
- `hko-neighborhood-sampling m10 ...` samples general fixed-`F=10`
  dual-vertex perturbations. The canonical tracked artifact is
  `empirical/neighborhood-sampling/m10/pentagon-perturb.jsonl`.
- `hko-neighborhood-sampling m11 --smoke` writes
  `empirical/neighborhood-sampling/m11/hko-neighborhood-splitting-smoke.jsonl`;
  full mode writes `empirical/neighborhood-sampling/m11/hko-neighborhood-splitting.jsonl`.
- `hko-neighborhood-sampling m10-lagrangian-product --smoke` writes
  smoke-level Lagrangian-product sweep outputs; full mode writes
  `empirical/neighborhood-sampling/m10-lagrangian-product/lagrangian-search*.jsonl`.
- `hko-neighborhood-sampling m10-lagrangian-product-probe --smoke` writes
  `empirical/neighborhood-sampling/m10-lagrangian-product/lagrangian-probe-smoke.jsonl`;
  full mode writes `empirical/neighborhood-sampling/m10-lagrangian-product/lagrangian-probe.jsonl`.
- `hko-second-order --smoke` runs the phase-1 probe without writing tracked
  outputs; full mode writes `empirical/second-order/*.jsonl`.
- `hko-m11-ascent --smoke` writes
  `empirical/m11-ascent/m11-ascent-smoke.jsonl`; full mode appends to
  `empirical/m11-ascent/m11-ascent.jsonl` unless `--fresh` is given.
- `hko-row-bank-validation` defaults to smoke input. Use `--canonical` only
  when refreshing `theorem/row-bank-validation/row-bank-validation-input.jsonl`.
- `hko-active-branch-diagnostic` writes an ignored smoke JSON by default and
  exact-checks no branches unless `--exact-limit N` or `--all-exact` is passed.
  Use `--canonical` only when intentionally refreshing the tracked diagnostic
  artifact. Each f64 active branch row includes `kkt_f64.singular`; theorem
  use of `d_sys_flat_f64` from singular KKT rows requires a separate theorem
  argument. The `singular_constraint_sections` block records the numerical
  and exact closure/normalization minor check for the current feasible-section
  route. The `feasible_section_rows` block computes the f64 derivative rows of
  the resulting explicit feasible beta sections and reruns the slice/cone
  checks.
- `hko-feasible-section-certificate --canonical --input <diagnostic.json>` writes
  `theorem/feasible-section-certificate/candidate-certificate.json` from the
  active-branch diagnostic. The explicit `--input` is required because current
  diagnostic sources are ignored local artifacts. The Sage constructor/verifier
  in that folder then produces and checks the exact 26-row feasible-section
  certificate.

## Fast Reading Order

1. `research/hko-local-maximum-status.md`
2. `research/hko-local-maximum.md`
3. `research/hko-local-maximum-proof-control-packet.md` if the question is
   what to do next for the theorem route
4. `research/hko-local-maximum-proof-route-note.md`
5. `theorem/README.md`
6. `empirical/README.md` if the question is about supporting evidence

## Rule Of Thumb

If a question is "what proves the theorem?", start in `theorem/`.

If a question is "what evidence supports the local-maximality story?", read
`empirical/` after the status note.
