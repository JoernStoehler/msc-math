# General-matrix regular normal fans: accepted finite-cone reduction

Research source owner: fixed regular pentagon / arbitrary-partner desk.
Source integration: 5 October 2026, 02:26 UTC. Final mathematical acceptance
was read at 02:23 UTC and the final source package was ready at 02:24 UTC.
Status: **independently checked reduction and finite inventory**.
Quadratic positivity, a sharp general-matrix HKO theorem and equality remain
open. Their unpriced continuation is parked, not selected by this note.

## Exact physical family and coordinate transfer

Fix the regular circumradius-one pentagon Q with unit outward normals n_i.
For any invertible real physical matrix A consider every bounded partner with
interior represented by normals A n_i and arbitrary canonical support offsets.
Redundant raw offsets can be replaced by the actual body supports. This
extends the earlier [fixed-axis orientation-wide theorem](orientation-wide-global-20261005.md)
to general matrix coordinates as a reduction only. It still does not cover
arbitrary five-normal fans, arbitrary partners or general convex four-bodies.

Put tau²=5−2sqrt(5), tau>0, S=diag(1,tau), and

    B=S A S^(-1),   A=S^(-1) B S,
    A=[[B00,tau B01],[B10/tau,B11]].

The four entries of B are independent real variables. Using Q(sqrt(5))
coefficients in the computational basis does not restrict the physical A.
The off-diagonal inverse scaling above is essential. Computational points
are S^(-1)q_phys and covectors N_i=S n_i, preserving their pairing.
Y_phys=A^T K_phys has the regular normal fan with the same support offsets;
detB=detA.

For canonical supports define

    P(h)=Σh_i h_(i+1)−((sqrt(5)−1)/4)Σh_i².

Then area(K)=P/[|detB| sin(2π/5)]. With actual complete capacity c>0,

    sys(K×_L Q)=c²|detB|/(5P).

Changing one product factor alone is not asserted to preserve capacity.
At fixed B, supports h/c give complete capacity one and P(h/c)=P(h)/c²;
the required capacity-squared factor is retained in the sharp-bound reduction.

## Complete homogeneous seven-variable cone

The inherited ten normal-triangle fitting problems and five closure circuits
give the complete capacity-at-least-one inequalities. Each circuit has three
positive weights, so every template's weighted support maximum expands to
27 independent vertex choices. All 1,350 fitting inequalities are retained,
plus five canonical edge inequalities. No attaining-label, LP redundancy or
body pruning is performed.

Translation gauge h_3=h_4=0 leaves three support variables alongside four B
entries. The homogeneous cone C has seven variables and coefficients in
Q(sqrt(5)). Exact positive-normalized deduplication gives 610 distinct rows:
605 fitting rows and five edge rows. Rank is seven and lineality is zero.
An independently checked strict regular-support witness proves full dimension
separately from rank. Maximum normalized rational coefficient bit-size is five.
These finite measurements do not price the unresolved positivity problem.

Right reflection B→B diag(1,−1), support reversal and exact re-gauging permute
the cone rows, preserve P and reverse determinant. This includes negative-
determinant bodies by relabeling the same physical normal-ray set, rather than
assuming an outer-angle symmetry. Singular matrices do not define bodies under
the inverse transfer, but they need not lie on C's boundary: B=0 with the
retained strict canonical-support witness is an interior cone point.

## Accepted reduction and unresolved inequality

A proof of

    Q_+(h,B)=P(h)−detB/(3+sqrt(5))>=0 on C

would give the sharp HKO bound for the general-matrix family, after complete
capacity normalization and the accepted reflection. Canonical geometry gives
P>=0, so Q_+ is automatic on detB<=0. The substantive unknown is constrained
quadratic nonnegativity on C's positive-determinant body sector.

The ambient seven-variable quadratic form has exact inertia (3 positive,
4 negative, 0 zero). This rules out an ambient PSD assertion; it does not
supply a negative feasible body or disprove positivity on C. The retained
negative ambient control violates canonical edge constraints. Neither ambient
indefiniteness nor sampled rays settles the constrained inequality.

The known HKO equality and every earlier scalar-chart coefficient pullback
are exact controls, not a general-matrix positivity proof. Sharp cone positivity,
general-matrix equality, extreme-ray/face enumeration and optimizer search were
not performed. No unrestricted global theorem, new maximum or counterexample
is claimed. The constrained positivity continuation remains parked and unpriced;
row counts and runtimes give no success probability or execution authorization.
External novelty and human acceptance remain unassessed.

## Evidence and reproduction effects

- [Final integrating acceptance](../../night-20261005/comparison/outer-matrix-preflight-final-acceptance.md)
  owns the accepted transfer, complete inventory, determinant/body caveats and
  unresolved positivity boundary.
- [Lead report](../../night-20261005/orientation-wide/outer-matrix-preflight/report.md)
  and [production definitions](../../night-20261005/orientation-wide/outer-matrix-preflight/extraction.md)
  own physical/gauge coordinates, source pullbacks and write effects.
- [Independent source/arithmetic review](../../night-20261005/orientation-wide/outer-matrix-preflight/review.md)
  verifies every source choice, normalization sign, rank, strict witnesses,
  reflection, physical ratio and ambient inertia using separate arithmetic.
- [Exact cone packet](../../night-20261005/orientation-wide/outer-matrix-preflight/cone.json)
  retains the 1,355 occurrences, 610 rows, target and source/witness controls.

The complete fixed-Q ten-template/five-circuit capacity representation and
canonical polygon-area interpretation are imported accepted inputs; the
[fixed-axis theorem](orientation-wide-global-20261005.md) remains a separate
completed result. This reduction does not extend its proved positivity by
analogy. Every newly selected transfer and inventory implication is accepted;
the next quadratic inequality remains unproved.

Read-only reproduction from repository root:

```sh
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/outer-matrix-preflight/replay.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/outer-matrix-preflight/review_cone.py
```

Both standard-library scripts write no files. The optional regeneration command

```sh
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/outer-matrix-preflight/generate.py
```

replaces only sibling cone.json. It reads accepted peer inputs relative to its
source file and reruns no earlier certificate producer. The retained snapshot
SHA-256 is
`9f02548e305ed586b83b2709719818281847ce2d2e5b66981b35973acc07e9b6`.
Observed generation was 2.340 seconds and independent final replay 5.971 seconds.
These are local observations, not costs for copositivity. An elapsed-time field
changes regenerated byte hashes; read-only replay compares every mathematical
field after removing only elapsed_seconds. This source integration reran no
scripts and replaced no evidence. No clean-environment or proof-assistant
verification is inferred.

**Selected thesis location: absent. PDF check: absent.** The retained 101-page
manuscript/PDF was unchanged; its 23:08 UTC hash check on 4 October was not
repeated. Chapters 9–10 contain related regular/affine results, not this cone
reduction. Manuscript inclusion, human acceptance and PDF review remain separate.
Selected symmetric-partner inclusion, human PASS, data-science scope, flow-code
correspondence and full reproduction remain open. The global source-hash baseline
and broader research-planning reconciliation remain pending.
