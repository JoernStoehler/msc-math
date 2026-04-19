# HKO Local Maximum Reasoning

This topic currently has two coupled tracks that should be interpreted together.

Exact-first proof track (`exact-clarke/`):
- The exact geometry and symmetry groundwork is committed (`hko-geometry.json`, `hko-symmetry-tangent.json`) and the active field is the quartic `Q(tan(pi/5))`.
- Packet 1 is essentially complete: exact dual coordinates, rank-15 symmetry tangent basis, and exact `R^40` setup are in place.
- Packet 2 is partially closed. There are exact prototypes for:
  - one six-facet endpoint family (`endpoint-prototype-certificate.json`),
  - one neighboring seven-facet midpoint-style segment (`segment-*-reduction.json` family),
  - exact `sys`/capacity/volume row families in `hko-volume-derivative.json` and `reduced-sys-prototypes.json`.
- The blocker is Packet 3 coverage: exact seed expansion currently needs additional symmetry-inequivalent representatives before the final active-gradient certificate can reach the target `rank(G)=25` with `ker(G)` matching the symmetry tangent space.
- `billiard-sigma-counts.json` still gives the feasibility frontier (`50,400` raw / `6,240` directed-feasible / `717` valid KKT orbits / `150` exact minima); `billiard-exact-probe.json` indicates the full `6,240` route is not yet cheap in the current exact stack, so the current default remains the reduced exact candidate route.

Numerical evidence track:
- `gradient-analysis/` has the baseline sensitivity and exact-certification bank, with `hko-neighborhood-sensitivity.jsonl` and `hko-neighborhood-ascent.jsonl` showing flat first-order behavior at HKO in the current neighborhood.
- `second-order/` gives fixed-`F=10` numerical support: `rank(G)=25`, `dim ker(G)=15`, all 15 flat-direction curvatures negative, and 100 random flat-subspace probes also negative.
- `facet-splitting/` and `cut-and-ascent/` tested the facet split and one-step continuation route (F=10→11) and did not find a `sys` increase over HKO in the committed runs.
- `lagrangian-boundary/` shows a compact, anisotropic local `sys > 1` region around HKO in Lagrangian perturbation coordinates (median per-component radius around `0.035`, strong decay by `ε≈0.15`).
- `perturbation-neighborhood/` has historical 101-row evidence and a refactored production flow for LICCA; current analyzer reads smoke/LICCA bucket files and keeps historical `pentagon-perturb.jsonl` as background context only.
- `sage-validation/` confirms selected exact rows against Sage on the shared canonical export; it validates the bank, not a finished theorem certificate.

Interpretation for the next agent:
- The visible code+artifacts currently support a strong working hypothesis that HKO2024 is a strict local maximum in the tested finite and smooth neighborhoods, but the formal first-order exact certificate is not yet closed.
- Future work should treat existing neighborhood evidence as support, and finish the exact active-set closure without changing the artifact contract.
