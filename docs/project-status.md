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

| Area | Current state | Source routes |
| --- | --- | --- |
| Foundations and generalized Reeb orbits | internally reviewed candidate; the Clarke/simple-minimizer proof boundary is settled; integrated reader review remains | `thesis/02-*`, `thesis/03-*`, corresponding formal sources |
| Haim--Kislev quadratic program and numerics | integrated draft now includes the twelve-facet branch family, six-facet product-capacity reduction, certified production algorithms, input contract, and numerical evidence; Kai/expert convention and integrated reader review remain appropriate | `thesis/04-*`, `thesis/11-numerics.tex`, `thesis/quadratic-program-algorithm-hk2019-content.md`, QP formal, crate, and experiment sources |
| Flow graph/CH2021 | substantial conditional draft; final theorem/algorithm role must remain honest | `thesis/05-*`, flow-graph formal, crate, and experiment sources |
| First-order perturbations | scaffold; arbitrary-polytope boundary remains materially different from the generic story | `thesis/06-*`, `formal/sys-first-order-local-behavior.md` |
| HKO local maximum | internally reviewed integration candidate; ordinary whole-PDF/Jörn/Kai review remains | `thesis/07-*`, `experiments/hko-local-maximum/` |
| Data-science search | integration candidate: the bounded random/product method table, held-out ten-facet optimizer comparison, convergence diagnostics, and predictor controls are integrated; independent closeout and ordinary whole-PDF/Jörn/Kai review remain | `thesis/08-*`, `experiments/sys-datascience/`, `experiments/dev-gradient-ascent/` |
| Rotated regular polygons | substantial theorem/certificate draft | `thesis/09-*`, `experiments/regular-products/` |
| AI use | the separate factual disclosure page is Jörn-accepted; the numbered research-process discussion remains provisional and materially incomplete | `thesis/ai-use-disclosure-content.md`, `thesis/use-of-ai-content.md`, `experiments/ai-use/` |
| Visualization, published code/data, conclusion | incomplete thesis surfaces | matching thesis and experiment sources |
| Submission and archive | incomplete external state | `submit/`, `docs/reproducibility.md`, Jörn/mail |

## Current cross-domain decisions and gates

- **Clarke/simple-minimizer proof boundary is settled.** Cite the literature for
  dual attainment and the nonsmooth multiplier input. Retain the local
  free-period multiplier calculation, coefficient, Reeb reconstruction, and
  characteristic identity rather than compressing the developed explanation
  into citations. Jörn accepted this proof-versus-citation boundary; integrated
  reader review still applies. See `thesis/theory-authoring-map.md` and the
  active foundation text.
- **Product finite enumeration is Jörn-accepted, not merely empirical.** The
  classical two-/three-bounce result plus the simple active-orbit reduction
  places a capacity minimizer in the enumerated family. The computation
  corroborates this implication but does not replace it. Kai/expert line review
  of the project-derived convention lift remains appropriate. See
  `thesis/quadratic-program-algorithm-hk2019-content.md` and
  `thesis/rotated-regular-polygons-content.md`.
- **Product capacity has a separate six-facet reduction.** Bilinear extremality
  in the two planar normalized closure polytopes proves that some capacity
  maximizer uses at most three facets from each factor. This supports the
  KKT-free scalar production route but does not replace the twelve-facet block
  family used for rotated-pentagon branch comparison or classify every
  minimizing orbit. Jörn reviewed and accepted the proof on 2026-07-28; the
  earlier independent review and 10,240-product falsification check remain
  supporting evidence. See
  `formal/product-qp-six-facet-reduction.tex` and the QP companion.
- **First-order and flow-graph strength remain open.** Generic smooth-branch
  first-order behavior does not settle the arbitrary-polytope boundary. The
  flow-graph chapter must match the hypotheses and support actually established
  by its formal, implementation, and experiment sources.
- **Data-science fact 34.1 is historical status, not a request for another
  dataset.** It said that work was incomplete at the time and should continue.
  The current integration candidate combines the later packets, closed method
  table, held-out optimizer comparison, endpoint continuation, HKO control, and
  branch-model failure audits.  The optimizer results support bounded
  statements about their recorded historical evaluator field, not a certified
  mathematical capacity, convergence, or local maximality.  Project fact 30
  still makes random/gradient search a major novel method for the data-science
  part. Its final thesis emphasis is a Jörn writing/design decision, distinct
  from whether the bounded empirical statements are supported.
- **External closure state remains partly private or time-sensitive.** Current
  official submission requirements need refreshing. The registration note
  still needs to be handed to the `Prüfungsamt` unless Jörn reports that action
  complete. Kai review state comes from Jörn/mail, not repository inference.

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

## Deliberately omitted from the active status view

The former file mixed current state with branch-recovery history, old packet
accounts, detailed crux tables, and operating instructions. Those details are
not reproduced here. The navigation migration compared the former
`PROJECT_COMPLETION.md` with this status view, `docs/project-facts.md`, the
active domain entry points, and the named local sources. Still-current
cross-project decisions and gates are retained above or in
`docs/project-facts.md`; topic-local detail remains at its source. Git history
retains dated recovery accounts, branch hashes, superseded dependency order,
and old crux identifiers.
