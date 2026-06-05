# HKO Local Maximum Research Note

Status entrypoint:
`research/hko-local-maximum-status.md` is the fastest summary of current claim
strength, proof-route status, and authoritative surfaces.

Current proof-route checkpoint:
`research/hko-local-maximum-proof-route-note.md` preserves the 2026-06-03
selected-branch certificate idea, current `150`-row f64 diagnostic evidence,
and the singular-KKT obstruction.

Current proof-control packet:
`research/hko-local-maximum-proof-control-packet.md` is the Packet 1 control
surface for the next HKO work. It records the theorem target, sufficient Sage
decision problem, proof tree, feasible-section row target, verifier boundary,
fallback branches, and review questions.

Current theorem certificate:
`experiments/hko-local-maximum/theorem/feasible-section-certificate/` contains
the exact feasible-section certificate generated on 2026-06-04. The verifier
checks 26 rows, exact row rank `25`, exact symmetry rank `15`, positive
lambdas, and exact lambda-weighted row sum `0`.

## Scope

This note is the consolidated research-facing record for `hko-local-maximum`.
It combines the former topic note set while keeping experiment artifacts under
`experiments/hko-local-maximum/`.

## Current State

- Exact-first route:
  - Exact geometry and symmetry groundwork is committed:
    `hko-geometry.json` and `hko-symmetry-tangent.json`.
  - Packet 1 is essentially closed: exact dual coordinates, rank-15 symmetry
    tangent basis, and exact `R^40` setup are in place.
  - The older exact-witness Packet 2 is partially closed with prototypes:
    one six-facet endpoint family and one neighboring seven-facet midpoint-style
    segment family. This is not the current theorem-closing route.
  - The older representative-expansion route remains partial.
  - The current route is the selected feasible-section upper-branch
    certificate. Rust exports a 26-row candidate; Sage constructs exact witness
    values; the exception-based Sage verifier checks the exact finite decision
    problem.
  - Current exact verifier status: 26 rows, exact row rank `25`, exact symmetry
    tangent rank `15`, positive exact lambdas summing to `1`, and exact
    lambda-weighted row sum `0`.
  - Sigma-word combinatorics are still: `50,400` raw, `6,240` directed-feasible,
    `717` valid KKT orbits, and `150` exact minima.
- Numerical evidence track:
  - `empirical/first-order/` has baseline sensitivity and exact-certification bank,
    with `hko-neighborhood-sensitivity.jsonl` and
    `hko-neighborhood-ascent.jsonl` showing flat first-order behavior at HKO.
  - `empirical/second-order/` gives fixed-`F=10` support: `rank(G)=25`,
    `dim ker(G)=15`, and negative curvature in all flat directions.
  - `empirical/neighborhood-sampling/m11/` and `empirical/m11-ascent/` found no
    `sys` increase over HKO on committed runs.
  - `empirical/neighborhood-sampling/m10-lagrangian-product/` reports an
    anisotropic compact local `sys > 1` region (median per-component radius
    near `0.035`, strong decay by `eps ~= 0.15`).
  - `empirical/neighborhood-sampling/m10/` keeps historical
    `pentagon-perturb.jsonl` as background context while the current pipeline
    uses smoke/LICCA buckets.
  - `theorem/row-bank-validation/` validates the exact row bank against Sage,
    but is not a full theorem certificate.

## Evidence And Interpretation

- Current code and artifacts strongly support a strict local-maximum hypothesis for
  HKO in tested finite and smooth neighborhoods.
- The exact finite certificate for the feasible-section route is now closed at
  the Sage-verifier level.
- The formal implication from the verifier propositions to the quotient-local
  maximum theorem has been agent-line-checked. Jörn quick-reviewed the rebuilt
  PDF on 2026-06-05 and spotted no gaps; he judged any remaining mistakes
  likely closeable before thesis release. The remaining theorem-strength gate is
  cheap formal clarity/checkability repair and final theorem wording/Kai review.
- Historical neighborhood evidence is treated as support, with priority on finishing
  exact active-set closure without altering artifact contracts.

## Decisions

- Primary proof route remains exact first-order in `R^40` with a backend-agnostic
  contract.
- Keep the theorem field as the quartic extension
  `t = tan(pi/5)`, with minimal polynomial `t^4 - 10 t^2 + 5 = 0`.
- Use dual coordinates `a_i` (description `K = {x : a_i * x <= 1}`) to avoid
  `(n,h)` gauge artifacts.
- First-order witnesses must include exact active rows, row counts, ranks, kernel
  basis, and symmetry inclusion/equality checks.
- The reduced exact route is preferred; the raw `6,240` directed-feasible sigma
  route is valid but remains too slow in current tooling as a default.
- The old `subdifferential-lp/phase_c_lp_test.py` route was deleted during the
  experiment layout migration; git history is the archive for that broken
  `(n,h)` Phase C attempt.
- Keep neighborhood falsification with `empirical/neighborhood-sampling/m10/`
  and `empirical/neighborhood-sampling/m10-lagrangian-product/` as
  complementary checks; broad blind counterexample searches are not the default.
- Keep `pentagon-perturb.jsonl` as historical context; prefer committed new-format
  smoke/LICCA buckets for analysis.
- Do not revive the older representative-coverage route unless formal review
  rejects the feasible-section certificate route.

## History

- `empirical/neighborhood-sampling/m10/` historically used 101-row evidence and
  has since moved to the current smoke/LICCA flow.
- Exact widened-route development reached right-kernel `29` on current
  representative surfaces. That is historical context for why the route moved
  to selected feasible-section upper branches.
- The older exact representative route still has two unresolved asymmetric
  seven-facet representative classes, but they are no longer blockers for the
  current feasible-section certificate.

## Next Steps

- Primary: patch the cheap clarity/checkability issues from the 2026-06-05
  subagent pass in `formal/hko-feasible-section-upper-branches.tex`.
- Then update the thesis-writing companion so Jörn can write the HKO section
  from fixed theorem/proof content instead of re-deciding the proof structure.
- Fallback: if review finds a real gap in the feasible-section implication,
  return to this note and choose between repairing the implication and
  weakening the theorem wording.
- Secondary: keep neighborhood evidence runnable for comparison and use
  `empirical/neighborhood-sampling/m10-lagrangian-product/` and
  `empirical/m11-ascent/` as non-default regression checks.
