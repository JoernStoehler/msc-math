# Derivation status

This note indexes the mathematical status of the endpoint-path packet. It is
not thesis prose and does not replace the detailed derivations in `notes/`.

## Established within the reviewed notes

- For full-dimensional convex polygons, the ridge feature is
  `R(P,Q) = sum_ij |a_i dot b_j| / sqrt(area(P) area(Q))`, provided the
  product face-ordering contract succeeds. The mixed-area identity uses
  `area(K+tL)=area(K)+2t V(K,L)+t^2 area(L)` and the explicit convention
  `J(x,y)=(-y,x)`.
- The reviewed mixed-area/difference-body argument proves `R >= 8`, with the
  stated centrally symmetric equality family, and proves the interior `3x6`
  endpoint `R = 4 sqrt(6)`.
- The reviewed endpoint derivations prove `sys=1/2` for the `4x4` square
  endpoint family and `sys=3/4` for the normalized `3x6` triangle/difference-
  body endpoint family, subject to the cited billiard correspondence.

These are mathematical derivations reviewed in
`notes/direct-optimization-review.md` and
`notes/endpoint-predictions-review.md`. They are not a formalization and are
not Jörn approval or thesis-ready exposition.

## Numerical observation

The eight retained rows are a frozen, deterministic construction: three
feature-CDF placements and one exact endpoint in each of the `3x6` and `4x4`
families. Their capacity and `sys` values are numerical outputs of the current
implementation, with the ordinary MinimaSafe diagnostics retained per row and
an exact-minimizer aggregation certificate only for `3x6/q01`.

The q01 certificate is for the f64-derived rational candidate and the
enumerated billiard stream. It is not a symbolic certificate for an ideal
trigonometric family and does not establish global all-sigma symbolic
minimality.

## Deliberately unresolved

- The `3x3` value `R=12` in the direct-optimization note is a strong
  derivation/conjecture, not an established global minimum in this packet.
- No monotonicity, mechanism, or population statement follows from one
  rotation path in each of two buckets.
- The stored winner signatures alone do not prove a billiard-branch switch.
