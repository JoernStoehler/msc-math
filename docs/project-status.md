# Project status

Updated for the navigation prototype from the former
`PROJECT_COMPLETION.md`. This is a compact state view, not mathematical source
truth.

## Milestones

1. **Ready for Kai:** Jörn accepts a full PDF for advisor review.
2. **Ready for hand-in:** Kai's feedback is incorporated and submission
   requirements are ready.
3. **Project complete:** the thesis is handed in and accepted code, data, and
   archive actions are complete.

No current hard submission deadline is recorded in the repository.

## Current thesis surface

| Area | Current state | Owner |
| --- | --- | --- |
| Foundations and generalized Reeb orbits | internally reviewed candidate; integrated reader review remains | `thesis/02-*`, `thesis/03-*`, corresponding formal sources |
| Haim--Kislev quadratic program | substantial draft; convention and finite-enumeration support remain important | `thesis/04-*`, HK2017 formal and crate owners |
| Flow graph/CH2021 | substantial conditional draft; final theorem/algorithm role must remain honest | `thesis/05-*`, flow-graph formal/crate/experiment owners |
| First-order perturbations | scaffold; arbitrary-polytope boundary remains materially different from the generic story | `thesis/06-*`, `formal/sys-first-order-local-behavior.md` |
| HKO local maximum | internally reviewed integration candidate; ordinary whole-PDF/Jörn/Kai review remains | `thesis/07-*`, `experiments/hko-local-maximum/` |
| Data-science search | broad exploration closed with bounded retained results; consumer-selected final data remains | `thesis/08-*`, `experiments/sys-datascience/` |
| Rotated regular polygons | substantial theorem/certificate draft | `thesis/09-*`, `experiments/regular-products/` |
| Visualization, numerics, published code/data, AI discussion, conclusion | incomplete thesis surfaces | matching thesis and experiment owners |
| Submission and archive | incomplete external state | `submit/`, `docs/reproducibility.md`, Jörn/mail |

## Project-wide constraints

- Do not weaken or remove the HKO, first-order, data-science, flow-graph,
  numerics, code/data, or AI-disclosure content merely because finishing it is
  expensive. A newer explicit Jörn decision may change scope.
- HKO theorem-strength use requires the exact certificate, a readable verifier
  explanation, and the mathematical implication from the certificate.
- Reproduction claims need a source-to-data-to-figure-to-PDF route and an
  explicit comparison standard.
- Jörn decides final thesis readiness; mathematical and advisor acceptance
  cannot be inferred from local checks.

## Status language

- **Recovered:** work was preserved; correctness and relevance are undecided.
- **Internally reviewed candidate:** named local review passed; stakeholder or
  integrated review may remain.
- **Accepted:** the named stakeholder gate explicitly passed.
- **Complete:** the surface satisfies its downstream use and no named gate
  remains.

## Deliberately removed from the active status view

The former file mixed current state with branch-recovery history, old packet
accounts, detailed crux tables, and operating instructions. Those details are
not reproduced here. Git history retains them; any still-current constraint
must be promoted to `docs/project-facts.md`, the relevant owner, or this file.

This omission is intentional in the prototype and must be audited before any
real migration.
