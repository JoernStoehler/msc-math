# Data-Science Appendix Content Notes

Status: section-local content companion for
`thesis/a-datascience-results.tex`. Not source truth.

Purpose: gather appendix-level material for detailed data-science experiment
results.

Overruled by: `experiments/sys-datascience/`,
`experiments/sys-datascience/methods/`,
`experiments/sys-landscape/legacy-ascent-continuation-debt.md`, generated
tables/figures, and Jörn/Kai review.

Lifecycle: keep while the appendix is being assembled. After the appendix is
stable, delete this file or reduce it to a short maintenance note.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Content Inventory

- Active appendix role: compact support surface for
  `thesis/08-black-box-datascience.tex`, not an independent method-churn
  chapter.
- Retained input facts:
  - `14336` trusted random/product rows;
  - `4096` generic random rows and `10240` random Lagrangian-product rows;
  - no duplicate polytope rows;
  - max retained `sys` about `0.863`;
  - median retained `sys` about `0.311`;
  - retained 99th percentile about `0.752`;
  - zero retained rows with `sys > 1`.
  Sources: `experiments/sys-datascience/methods/trusted-random-dataset/README.md`;
  `experiments/sys-datascience/methods/random-tail-eda/README.md`.
- Retained-table diagnostics to summarize:
  - direct scan and tail EDA;
  - scalar associations;
  - projection, clustering, anomaly checks;
  - supervised prediction/ranking;
  - tail-rule mining.
- Generated-candidate evidence:
  - 100k scalar-proposer packet over random products;
  - `485` unique selected rows, `1195` unique baseline rows, `1675` evaluated
    selected-or-baseline rows;
  - no evaluated `sys > 1` row;
  - max evaluated and selected `sys` about `0.868`.
  Source:
  `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md`.
- Mechanism/reference diagnostics:
  - ridge-mechanism discriminator;
  - HKO reference coverage and ridge-source smoke.
  Source pointers:
  `experiments/sys-datascience/methods/ridge-mechanism-discriminator/README.md`;
  `experiments/sys-datascience/methods/hko-reference-coverage/`;
  `experiments/sys-datascience/methods/hko-ridge-source-smoke/`.
- Inactive for this appendix unless explicitly reopened: old ascent,
  continuation, endpoint-stability, local-behavior, and perturbation panels.

## Wording Guardrails

- Do not present retained-table diagnostics as generated-candidate validation.
- Do not claim random search cannot find a counterexample.
- Do not claim all useful invariant features were exhausted.
- Do not claim the scalar-proposer packet validates a proposer for actually
  finding `sys > 1`.
- Do not use HKO reference diagnostics as evidence for HKO local maximality.
- Keep detailed artifact rankings, thresholds, and per-feature rows in method
  packet artifacts unless they are needed for a thesis-facing claim.
