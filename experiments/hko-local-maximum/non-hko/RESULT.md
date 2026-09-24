# Result: a certified non-HKO ten-facet local maximum

**Status: completed computer-assisted local-maximality proof.**

The rational body `hexagon_quadrilateral_v1_2_s1_12`, defined in
`candidates.json`, has exactly ten genuine facets, 24 vertices,

\[
\operatorname{Vol}_4(P_*)=\tfrac12,\qquad
c_{\rm EHZ}(P_*)=1,\qquad \rho(P_*)=1.
\]

There exists a Hausdorff neighborhood U of P* such that **every convex
four-polytope in U with exactly ten genuine facets has ratio at most 1**.
This quantifier includes nonproduct, nonsymmetric, and non-affine-image
perturbations. The maximum is non-strict: the explicit equality family
E(v,s) in `PROOF.md` has two independent parameters modulo translations,
positive scaling, and linear symplectic transformations near
(v,s) = (1/2,1/12).

P* is a Lagrangian product of a genuine hexagon and a genuine quadrilateral,
not an eight- or nine-facet body padded to ten rows. Its 24 vertices, compared
with HKO's 25, rule out even general invertible affine equivalence, hence also
all the excluded symplectic and anti-symplectic equivalences. Its exact ratio
1 also differs from HKO's (3+sqrt(5))/5.

## The certificate

| Predicate | Exact outcome |
|---|---:|
| Genuine facets / vertices | 10 / 24 |
| Complete product capacity enumeration | 624 cyclically ordered cases |
| Maximum Q / capacity | 1/2 / 1 |
| Tight product cases / strict gap for the others | 12 / 1/14 in Q |
| Touching feasible sections | 24 |
| Minimum positive section weight | 1/320 |
| Rank of ambient upper-section gradient bank | 22 |
| Minimum positive normalized relation coefficient | 3/36617 |
| Symmetry tangent rank | 15 |
| Independent equality parameters modulo symmetry | 2 |
| Remaining transverse directions | 1 |
| Full local chart rank | 40 |
| Second derivative in the remaining direction | -13/432 |

The contacts and the extra kernel vector continue by exact identities over
QQ(v,s). Along the explicitly displayed remaining direction W*, an independent
rational-function calculation gives, for all 24 chosen sections,

\[
V(A_*+zW_*)=\tfrac12+\tfrac{13}{1728}z^2,\qquad
Q_j(A_*+zW_*,\beta_j(z))=\tfrac12,
\qquad U_j(z)=\frac{864}{864+13z^2}.
\]

The uniform Taylor argument in `PROOF.md`, Section 8, combines this negative
second-order term with the positive gradient relation. The exact rank-40
inverse-function check supplies a full symmetry slice, rather than assuming
that a restricted family covers nearby bodies.

## Evidence and scope

`evidence/exact/verify.log` is an actual complete run and ends with
`ALL EXACT CHECKS PASS`. The entry point is `verify.py`; it reads the rational
witness but does not trust or read the retained reports for acceptance.
`evidence/reproduction/` records a second complete run from an extracted
archive, together with deterministic result comparisons.

The proof uses exact rational and rational-function arithmetic, not an interval
claim derived from float64 samples. It is a computer-assisted mathematical
proof, **not a proof-assistant formalization**. Its neighborhood radius is
existential, justified by strict finite predicates and continuity; no numerical
radius or rectangular parameter range is claimed. No unresolved mathematical
blocker is identified. Standard trust in the HK capacity formula, the analytic
argument, and the exact algebra implementation remains explicit.

The initial numerical discovery, including an aborted full-capacity polling
attempt, is preserved and labelled in `discovery/` and `RUN_LEDGER.md`. None of
those heuristic computations is a premise of the completed theorem. The
underlying product equality construction was motivated by the 2026 planar
triangle-covering paper cited in `SOURCES.md`; the packet does not claim priority
for that product family. Its contribution here is the full ten-facet
local-maximality certificate, including nonproduct transverse directions.

One successful example is supplied. No uniqueness theorem, global record
claim, or changing-facet-count conclusion is made.
