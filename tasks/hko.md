<!--
Purpose: HKO2024 local-maximality roadmap.
Context: main thesis result and potential publication-grade follow-up surface.
-->

# HKO Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-25.
- Source surfaces: `research/hko-local-maximum.md`,
  `research/hko-local-maximum-status.md`,
  `research/hko-local-maximum-exact-clarke.md`,
  `experiments/hko-local-maximum/`, `tasks/verify-thesis-done.md`.
- Refresh when: exact-Clarke route, HKO theorem wording, or LICCA evidence
  changes.

## Steering Cache

- [accepted 2026-04-15] HKO2024 local maximality is part of the thesis spine.
  Source: Kai/Jorn state in legacy tracker, now routed through
  `research/INDEX.md` and this bundle.
  Why it matters: HKO compression is mainline thesis work.
- [accepted 2026-04-24] LICCA large HKO runs are optional publication-grade
  polish, not required for thesis sufficiency unless results already exist with
  low integration cost or Jorn chooses the external action.
  Source: Jorn finish-mode reset.
  Why it matters: prevents compute work from delaying thesis writing by default.
- [accepted 2026-04-24] Exact first-order certificate is the preferred stronger
  route if it becomes trusted; otherwise wording can fall back to supported
  numerical/conditional evidence.
  Source: legacy exact-Clarke row and `tasks/verify-thesis-done.md`.
  Why it matters: theorem strength depends on certification status.

## Current Decomposition

This is the expanded meaning of "HKO is a local maximum" for thesis closeout.

### Thesis Theory Writing

- Broad story: HKO2024 is conjecturally a local maximum of `sys`.
- Thesis-ready subresult target: local maximality on `M_10` modulo the natural
  `sys` symmetries: translations, scaling, and linear symplectic maps.
- Thesis statement must not claim strict local maximality in raw `R^40`.
- If the exact first-order certificate closes, write the theorem around
  "Clarke-flat first-order directions equal the 15-dimensional symmetry tangent
  space".
- If the exact certificate does not close, weaken the theorem and present the
  current first-order/second-order/empirical evidence honestly.

### Formalizations And Proofs

- Exact first-order route is the preferred proof route.
- Packet 1 is essentially closed: exact dual coordinates, rank-15 symmetry
  tangent basis, and exact `R^40` setup exist.
- Packet 2 is partially closed: endpoint/midpoint prototypes and combinatorics
  reduce the active surface but do not yet close theorem-facing coverage.
- Packet 3 is the current proof blocker: exact representative coverage must be
  widened until the active-gradient matrix has rank `25`, kernel dimension `15`,
  and kernel equal to the symmetry tangent space.
- Current exact field is the quartic `Q(tan(pi/5))`, not `Q(sqrt(5))`.
- `formal/hko-local-maximum/gradient-analysis.tex` still contains old
  `44`-orbit / `10`-gradient and unverified symmetry prose that conflicts with
  the current `150` exact minima bookkeeping.
- `formal/hko-local-maximum/second-order.tex` is an older route with
  non-smooth-analysis TODOs; it should become supporting evidence unless Jorn
  chooses to repair it as a proof route.

### Experiment Execution

- `exact-clarke/` is theorem-facing and owns exact witness artifacts plus Sage
  verification.
- `gradient-analysis/` is first-order numerical support and active-gradient
  bookkeeping.
- `second-order/` supports the local-maximality story with fixed-`F=10`
  curvature evidence, but is not the preferred final proof route.
- `perturbation-neighborhood/`, `facet-splitting/`, `cut-and-ascent/`, and
  `lagrangian-boundary/` are empirical falsification/neighborhood evidence.
- LICCA-scale F=10 perturbation and higher-F checks are future/follow-up unless
  results already exist with low integration cost or Jorn chooses that external
  action.

### Interpretation

- Current repo evidence strongly supports the `M_10` local-maximality story, but
  the exact theorem certificate is not closed.
- Supporting experiments can justify a cautious empirical paragraph; they cannot
  replace the missing exact first-order certificate for a theorem-strength
  claim.
- The thesis should separate `M_10` theorem/evidence from beyond-`M_10`
  falsification attempts.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| HKO theorem/evidence/blocker split | `[done]` | map input | agent | Current decomposition captured in this file; refresh after exact-route or thesis-claim changes. | `research/hko-local-maximum*.md` |
| Claim statement freeze | `[Jorn]` | mainline thesis | Jorn | Choose final HKO thesis wording obligation: exact `M_10` theorem if Packet 3 closes, or weaker theorem/evidence wording if not. | `research/hko-local-maximum.md`, `tasks/verify-thesis-done.md` |
| Exact certificate field mismatch | `[active]` | mainline thesis | agent then Jorn | Final gate was generalized on 2026-04-24; propagate quartic `Q(tan(pi/5))` field wording into theorem-facing prose and do not target a `Q(sqrt(5))` certificate. | `research/hko-local-maximum.md`, `research/hko-local-maximum-exact-clarke.md` |
| Packet 3 representative coverage | `[active]` | mainline thesis or contingent | dedicated exact route sessions | Exactify the two unresolved asymmetric seven-facet representative classes and rebuild the witness until the final active-gradient rank/kernel comparison is visible. | `experiments/hko-local-maximum/exact-clarke/`, `research/hko-local-maximum-exact-clarke.md` |
| Witness contract and verifier | `[active]` | mainline thesis or contingent | dedicated exact route sessions | Extend the existing backend-neutral witness from partial Packet 3 support to final active rows, ranks, kernel basis, and symmetry inclusion/equality checks; verify with Sage. | `widened-seed-witness.json`, `widened-seed-witness-verification.json`, `verify_widened_seed_witness.sage` |
| Stale `44/10` reconciliation | `[active]` | mainline thesis | agent | Replace or caveat old `44`-orbit / `10`-gradient prose with current `150` minima / `20` subsets / `28` gradient-pattern bookkeeping. | `experiments/hko-local-maximum/exact-clarke/numerical-minima-summary.json`, `formal/hko-local-maximum/gradient-analysis.tex` |
| Formal theorem writeup | `[blocked]` | mainline thesis | exact route or weaker claim | After proof/wording route is frozen, write/update the formal and thesis-facing proof route. | `formal/hko-local-maximum/`, `thesis/` |
| h-space / Danskin proof check | `[Jorn]` | mainline thesis if retained | Jorn | Verify the non-smooth first-order/Danskin argument only if retained in theorem route. | `formal/hko-local-maximum/second-order.tex` |
| second-order proposition status | `[Jorn]` | contingent during writing | Jorn | Decide whether second-order note is proof route, supporting evidence, or future/cut after exact-route status is clear. | `formal/hko-local-maximum/second-order.tex` |
| HKO empirical wording | `[blocked]` | contingent during writing | retained thesis text | Word perturbation, facet-splitting, cut-and-ascent, and neighborhood evidence only as strongly as existing artifacts support. | `thesis-stories-are-supported.md`, `data-and-figures-are-traceable.md` |
| HKO figures/tables | `[blocked]` | contingent during writing | thesis outline | Create or cite only figures/tables that the final HKO section actually uses. | `data-and-figures-are-traceable.md` |
| higher-F perturbation | `[future]` | future/follow-up | Jorn/external compute | Leave F=12/F=13 validation as pending/future unless cheap results already exist. | `research/hko-local-maximum.md` |
| LICCA F=10 neighborhood | `[future]` | future/follow-up | Jorn/external compute | Reopen only if Jorn chooses LICCA action or results already returned. | `experiments/hko-local-maximum/perturbation-neighborhood/` |

## Agent Cache

- [fresh 2026-04-24] Current local HKO evidence includes first-order
  positive-span signal, second-order negative curvature samples, facet-splitting
  checks, cut-and-ascent checks, and a perturbation-neighborhood artifact.
  Refresh by: reading `research/hko-local-maximum.md` and the linked experiment
  directories.
- [fresh 2026-04-24] Exact-Clarke route state is nuanced: current Sage
  representative-first route weakened the old SymPy cost objection, but active
  row multiplicity remains the obstruction.
  Refresh by: reading `research/hko-local-maximum-exact-clarke.md` and current
  exact-Clarke artifacts.
- [fresh 2026-04-24] Current exact route field is the quartic
  `Q(tan(pi/5))`; older `Q(sqrt(5))` wording is stale for certificate targets.
  Refresh by: reading `research/hko-local-maximum.md` "Decisions" and
  `research/hko-local-maximum-exact-clarke.md` "Field Note".
- [fresh 2026-04-25] Current widened Packet 3 witness is real but partial: the
  Sage verifier passes for quartic field reconstruction, exact symmetry rank,
  representative-row closure/normalization/common-scalar checks, and current
  endpoint/midpoint row ranks, but it does not yet verify the two unresolved
  asymmetric seven-facet families, final active-gradient matrix, final cone
  certificate, or kernel-equals-symmetry theorem reduction.
  Refresh by: reading `research/hko-local-maximum-exact-clarke.md` "Scope
  Boundary" and "Sage Note".
- [fresh 2026-04-25] Current exact-minimum bookkeeping is `150` exact action
  orbits, `20` distinct visited subsets, and `28` distinct height gradients.
  This conflicts with old `formal/hko-local-maximum/gradient-analysis.tex`
  prose that still says `44` near-optimal orbits and `10` distinct gradients.
  Refresh by: checking `experiments/hko-local-maximum/exact-clarke/numerical-minima-summary.json`
  and the opening of `formal/hko-local-maximum/gradient-analysis.tex`.
- [fresh 2026-04-24] Before LICCA submission, the remote repo layout must match
  the current `experiments/...` package layout, not old `crates/exp-*` paths.
  Refresh by: checking `tasks/submit-thesis.md` and current LICCA scripts.

## Pruned / Stale

- [stale 2026-04-24] Treat old "all HKO polish before April cutoff" scheduling
  as superseded. Retain only thesis-spine proof/evidence choices.
