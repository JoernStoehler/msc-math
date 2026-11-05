# Exact geometry at t = 7/20

The companion `exact_geometry.py` reconstructs the 20 support rows over
`Q(tau)` from `common.py`, where `tau` is the specified real root of
`tau^4 - 10 tau^2 + 5`. All determinant, feasibility, active-set, pairing,
and volume equalities are exact in that number field. Sign comparisons use
the explicit real embedding into Sage's exact `AA` field.

For each four-row subset of `N(t)`, the script discards singular matrices,
solves `N_I v = 1`, and keeps the solution precisely when all 20 slacks are
nonnegative. Every vertex of a bounded full-dimensional polytope has four
independent active normals; the origin is strictly inside because all
support constants are one, and boundedness follows from `K_t subset H`.
Thus this enumeration finds all vertices. Every retained active set has
exactly four rows, proving simplicity. The union of active labels is all 20,
so each listed inequality supports a nonempty facet and is genuine.

For every vertex, the directed graph records `i -> j` exactly when those
facets meet there and `omega(n_i,n_j) >= 0`. This is the necessary transition
condition used in the frozen coverage proof. The script writes the freshly
computed matrix to `graph.txt`; downstream DFS must read this output rather
than the frozen `t=3/10` graph.

Volume uses Lawrence's simple-polytope vertex-cone formula in dimension four:
for generic `ell`, sum over vertices `v` with active matrix `M=N_I` the terms
`(ell.v)^4 / (4! |det M| product(ell M^{-1}))`. The code checks all slopes are
nonzero and evaluates the formula for `ell=(1,2,4,8)` and `(1,3,9,27)`; the
two exact field elements must agree and be positive. The mathematical input
is Jim Lawrence, *Polytope Volume Computation*, NISTIR 89-4123 (1989), §2,
Theorem 1, printed page 4 (PDF page 8):
<https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir89-4123.pdf>.

The coarse inclusion needed by the singular-family rejection is checked
independently from the cut construction. The code enumerates all vertices of
`H={x:N0 x<=1}` and maximizes each of the 20 exact linear functionals given
by rows of `N(t)` over those vertices. Since `H` is bounded, this computes
each support maximum. It verifies every maximum is at most `1+2t/5`, so for
every `x in H/(1+2t/5)` one has `N(t)x<=1`, proving
`H/(1+2t/5) subset K_t`. This check is specific to the exact geometry at
`t=7/20`; no frozen support bound is reused.

`exact-geometry-summary.json` is the machine receipt. Its finite predicates
do not prove the separate simple-minimizer theorem or Lawrence's formula;
those are the cited mathematical inputs to the full certificate.
