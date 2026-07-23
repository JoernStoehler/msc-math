# HKO Local Maximum

This topic directory separates theorem-facing certificate work from empirical
support checks for the HKO2024 local-maximum result packet.

Start here before reading subfolders. This README gives the common context for
the HKO slice.

## Current Status

The current theorem-facing route is the feasible-section certificate in
`theorem/`, together with the mathematical implication in
`formal/hko-feasible-section-upper-branches.tex`.

The exact Sage verifier checks 26 feasible-section rows, exact row rank `25`,
exact symmetry tangent rank `15`, positive exact lambdas summing to `1`, and an
exact lambda-weighted row sum of `0`. Rust selects finite verifier input only;
Sage reconstructs and verifies the exact algebraic objects.

The formal implication has been agent-line-checked against the verifier
propositions. Jörn quick-reviewed the rebuilt formal PDF on 2026-06-05 and
spotted no gaps. Later review of the integrated thesis PDF may still request
wording or proof-framing changes. The empirical folders are support and sanity
checks, not proof substitutes.

The verifier's owner-local explainability and trust-boundary contract is in
`theorem/README.md`; use it when reviewing code together with thesis prose.

## Result Strands

| Strand | Path | Role |
| --- | --- | --- |
| Theorem certificate | `theorem/` | Active feasible-section certificate: generate witness, verify exact predicate. |
| Smooth-only rank defect | `smooth-only-rank-defect/` | f64 diagnostic summary: nonsingular positive-beta branches have rank `23` in the `25`-dimensional quotient. |
| Empirical sampling | `empirical/` | Numerical and sampling evidence that supports or illustrates the local-maximum picture but is not the final proof. |
| Assets | `assets/` | Explanatory figure scripts and images. These are not theorem evidence. |
| Row-bank validation | `row-bank-validation/` | Rust/Sage validation of selected exact-bank rows. This is validation machinery, not theorem proof. |
| Shared Rust helpers | `src/` | Topic-local code shared by theorem and empirical binaries. |

Generated JSON/JSONL/figure artifacts stay beside the producer or sampler that
creates them. Do not create output-only buckets detached from the script or Rust
sampler that owns the artifact.

## Current Layout

```text
experiments/hko-local-maximum/
|-- theorem/
|   |-- active_branch_diagnostic.rs
|   |-- generate.rs
|   |-- verify.sage.py
|   |-- witness.json
|   `-- verification-summary.json
|-- smooth-only-rank-defect/
|-- row-bank-validation/
|-- assets/
|-- empirical/
|   |-- first-order/
|   |-- second-order/
|   |-- m11-ascent/
|   `-- neighborhood-sampling/
`-- src/
```

Older inactive routes and deleted exact-route scripts live only in git history
unless a current local README or generated summary points to a live consequence.

## Rust Command Contract

- `hko-first-order --smoke` computes the sensitivity probe and exits without
  writing. Full mode writes the tracked sensitivity/ascent outputs.
  `--exact-bank` defaults to an untracked smoke output; add `--canonical` only
  when refreshing the tracked exact bank.
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
- `hko-neighborhood-sampling m10-quotient-ray --smoke --out-dir <new-empty-dir>`
  runs the short event-labelled quotient-slice screen. The reviewed 32-ray mode
  additionally requires `--frozen-panel --launch-packet <reviewed-packet.json>`;
  see `empirical/neighborhood-sampling/README.md` for its provenance contract.
- `hko-second-order --smoke` runs the phase-1 probe without writing tracked
  outputs; full mode writes `empirical/second-order/*.jsonl`.
- `hko-m11-ascent --smoke` writes
  `empirical/m11-ascent/m11-ascent-smoke.jsonl`; full mode appends to
  `empirical/m11-ascent/m11-ascent.jsonl` unless `--fresh` is given.
- `hko-row-bank-validation` defaults to smoke input. Use `--canonical` only
  when refreshing `row-bank-validation/row-bank-validation-input.jsonl`.
- `hko-active-branch-diagnostic` writes an ignored smoke JSON by default and
  exact-checks no branches unless `--exact-limit N` or `--all-exact` is passed.
  Use `--canonical` only when intentionally refreshing a theorem-facing
  diagnostic source for downstream generation. The smoke diagnostic JSON in
  `theorem/` is a local ignored artifact, not a tracked proof object. Each f64
  active branch row includes `kkt_f64.singular`; theorem use of `d_sys_flat_f64`
  from singular KKT rows requires a separate theorem argument. The
  `singular_constraint_sections` block records the numerical and exact
  closure/normalization minor check for the current feasible-section route. The
  `feasible_section_rows` block computes the f64 derivative rows of the
  resulting explicit feasible beta sections and reruns the slice/cone checks.
- `hko-smooth-only-rank-defect` reads the active-branch diagnostic and writes
  `smooth-only-rank-defect/summary.json`. It records f64 evidence
  that the nonsingular positive-beta branch attempt has projected rank `23` in
  the `25`-dimensional quotient. This is not theorem proof.
- `hko-feasible-section-generate --canonical --input <diagnostic.json>` writes
  `theorem/witness.json` from the active-branch diagnostic. The explicit
  `--input` is required because current diagnostic sources are ignored local
  artifacts. `theorem/verify.sage.py` reads that witness, computes the exact
  algebraic data in SageMath, and checks the exact 26-entry feasible-section
  certificate predicate.

## Fast Reading Order

1. This `README.md`
2. `theorem/README.md` if the question is what currently verifies the
   theorem-facing finite certificate
3. `smooth-only-rank-defect/` if the question is why the smooth
   nonsingular positive-beta branch attempt did not close
4. `row-bank-validation/` if the question is about exact-bank cross-checks
5. `assets/` if the question is about explanatory figures
6. `empirical/README.md` if the question is about supporting evidence

## Rule Of Thumb

If a question is "what proves the theorem?", start in `theorem/`.

If a question is "what evidence supports the local-maximality story?", read
`empirical/` after this README.

If a question is "what figure can illustrate the HKO local picture?", read
`assets/`.
