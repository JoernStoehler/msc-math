# Exact Geometry And Symmetry

Status: route-history and setup note. The current theorem-facing verifier
reconstructs the HKO geometry directly in
`../feasible-section-certificate/verify.sage.py`.

Facts retained for thesis navigation:

- The exact HKO field is `Q(tan(pi/5))`.
- The field generator `t` is the real root in `(0,1)` of
  `t^4 - 10 t^2 + 5`.
- Coordinates use the project convention `(q1, q2, p1, p2)`.
- The feasible-section verifier reconstructs the HKO dual vertices, volume,
  volume derivative row, and symmetry tangent generators from formulas.
- `../feasible-section-certificate/verification-summary.json` records that
  Sage checked symmetry tangent rank `15`.

Older exact-witness Packet 1 files such as `../exact-witness/check.sage`,
`../exact-witness/hko-geometry.json`,
`../exact-witness/hko-symmetry-tangent.json`, and
`../exact-witness/hko-volume-derivative.json` verified related exact geometry
and symmetry facts for the earlier route. They are route-history and fallback
material, not the current theorem certificate.

Source pointers:

- `../feasible-section-certificate/README.md`
- `../feasible-section-certificate/verify.sage.py`
- `../feasible-section-certificate/verification-summary.json`
- `research/hko-local-maximum-status.md`
- `formal/hko-feasible-section-upper-branches.tex`
