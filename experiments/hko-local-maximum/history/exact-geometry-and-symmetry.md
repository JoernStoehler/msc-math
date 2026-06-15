# Exact Geometry And Symmetry

Status: route-history and setup note. The current theorem-facing verifier
reconstructs the HKO geometry directly in
`../theorem/verify.sage.py`.

Facts retained for thesis navigation:

- The exact HKO field is `Q(tan(pi/5))`.
- The field generator `t` is the real root in `(0,1)` of
  `t^4 - 10 t^2 + 5`.
- Coordinates use the project convention `(q1, q2, p1, p2)`.
- The feasible-section verifier reconstructs the HKO dual vertices, volume,
  volume derivative row, and symmetry tangent generators from formulas.
- `../theorem/verification-summary.json` records that
  Sage checked symmetry tangent rank `15`.

Older Packet 1 implementations of these checks were deleted from the live tree.
The facts above are retained here because they remain useful for thesis
navigation, and the current source of truth is the feasible-section verifier.

Source pointers:

- `../theorem/README.md`
- `../theorem/verify.sage.py`
- `../theorem/verification-summary.json`
- `research/hko-local-maximum-status.md`
- `formal/hko-feasible-section-upper-branches.tex`
