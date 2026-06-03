# HKO Local Maximum Research Note

Status entrypoint:
`research/hko-local-maximum-status.md` is the fastest summary of current claim
strength, proof-route status, and authoritative surfaces.

Current proof-route checkpoint:
`research/hko-local-maximum-proof-route-note.md` preserves the 2026-06-03
selected-branch certificate idea, current `150`-row f64 diagnostic evidence,
and the singular-KKT obstruction.

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
  - Packet 2 is partially closed with exact prototypes:
    one six-facet endpoint family and one neighboring seven-facet midpoint-style
    segment family.
  - Packet 3 is the current blocker. Exact seed expansion still needs
    additional symmetry-inequivalent representatives before final active-gradient
    coverage reaches `rank(G)=25` with kernel matching the symmetry tangent space.
  - A newer proof-route checkpoint records a possible one-sided
    selected-branch certificate: all `150` f64 positive active rows cover the
    `25`-dimensional slice numerically, but `106` of those rows have singular
    KKT matrices and still need exact singular-family justification.
  - Sigma-word combinatorics are still: `50,400` raw, `6,240` directed-feasible,
    `717` valid KKT orbits, and `150` exact minima.
- Numerical evidence track:
  - `gradient-analysis/` has baseline sensitivity and exact-certification bank,
    with `hko-neighborhood-sensitivity.jsonl` and
    `hko-neighborhood-ascent.jsonl` showing flat first-order behavior at HKO.
  - `second-order/` gives fixed-`F=10` support: `rank(G)=25`,
    `dim ker(G)=15`, and negative curvature in all flat directions.
  - `facet-splitting/` and `cut-and-ascent/` found no `sys` increase over HKO
    on committed runs.
  - `lagrangian-boundary/` reports an anisotropic compact local `sys > 1` region
    (median per-component radius near `0.035`, strong decay by `eps ~= 0.15`).
  - `perturbation-neighborhood/` keeps historical `pentagon-perturb.jsonl` as
    background context while the current pipeline uses smoke/LICCA buckets.
  - `sage-validation/` validates the exact row bank against Sage, not a full
    theorem certificate.

## Evidence And Interpretation

- Current code and artifacts strongly support a strict local-maximum hypothesis for
  HKO in tested finite and smooth neighborhoods.
- The exact-first first-order formal certificate is not yet closed.
- The remaining exact work is primarily representative coverage and a final
  `rank(K)/kernel` comparison on a backend-agnostic witness bundle.
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
- `subdifferential-lp/phase_c_lp_test.py` is intentionally inactive until the
  `(n,h)` schema migration is done.
- Keep neighborhood falsification with `perturbation-neighborhood/` and
  `lagrangian-boundary/` as complementary checks; broad blind counterexample
  searches are not the default.
- Keep `pentagon-perturb.jsonl` as historical context; prefer committed new-format
  smoke/LICCA buckets for analysis.
- If representative coverage stalls, it is acceptable to pause and document the
  blocker instead of adding ad-hoc representative heuristics outside contract.

## History

- `perturbation-neighborhood/` historically used 101-row evidence and has since
  moved to the current smoke/LICCA flow.
- Exact widened-route development reached right-kernel `29` on current
  representative surfaces; this indicates active-row multiplicity, not affine
  mismatch, is the remaining obstruction.
- Exact route currently has two unresolved asymmetric seven-facet representative
  classes.

## Next Steps

- Primary: close exact Packet 3 by extending representative-seed coverage to final
  rank-equal active matrix and completing witness-vs-symmetry comparison.
- Immediate proof-route question: develop a theorem-valid treatment of the
  singular seven-facet, positive-beta active rows. The 2026-06-03
  padded-extension diagnostic found no nonsingular minimum-action padded-once
  rows, so the simple nonsingular padded alternative currently has no
  surviving rows. The smooth-only row set is still short by two slice
  dimensions.
- Required stop condition:
  - produce witness bundle with `rank(active_grad_G)=25`;
  - `dim(ker(G))=15`;
  - kernel matches the `R^40` symmetry tangent basis.
- Explicit blockers:
  - remaining two asymmetric seven-facet classes;
  - final packet must use reduced prototypes (`endpoint-seed-rows.json`,
    `midpoint-seed-rows.json`) plus additional exact representatives.
- Immediate execution actions:
  1. refresh `exact-clarke/numerical-permutation-orbits.json`;
  2. add missing exact representative rows in the existing witness shape;
  3. rebuild `reduced-sys-prototypes.json` and rerun the exact witness + Sage
     verifier:
     - `cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation -- --canonical`
     - `cd experiments/hko-local-maximum/sage-validation && sage -python analyze.py --canonical`;
  4. update `exact-clarke/widened-seed-witness.json` only from scripted output.
- Fallback: if exact completion is blocked by backend cost, log the obstruction and
  rerun the exact `6,240` sigma route on a faster backend before switching to the
  contingency route.
- Secondary: keep neighborhood evidence runnable for comparison and use
  `lagrangian-boundary/` and `cut-and-ascent/` as non-default regression checks.
