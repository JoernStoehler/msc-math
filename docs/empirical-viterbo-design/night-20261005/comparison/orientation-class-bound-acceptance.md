# Accepted fixed-axis orientation-wide sharp bound

Integrating owner: `scientific_compare_initial`; receiving owner: root.
Acceptance: 2026-10-05 00:59 UTC. Complete equality classification is pending.

## Exact supported statement

Fix a regular pentagon Q. If K is any bounded convex polygon with nonempty interior, represented by the
five normals `diag(x,1) R_theta n_i`, with x>0 and arbitrary support offsets,
then

`c_EHZ(K×Q)^2 / (2 area(K) area(Q)) <= (3+sqrt(5))/5`.

The distortion axes are fixed relative to Q. All theta in this stated
family are covered, using the accepted period/reflection reduction.
Redundant support inequalities and three-/four-edge boundaries are included
through canonicalization and the inherited primary lower-edge bound.
The known regular HKO pair attains the constant, so the bound is sharp.
No complete equality classification is accepted at this checkpoint.

This is a restricted global theorem. It does not assert the same theorem
for arbitrary GL2 transforms of the five normals, arbitrary pentagon pairs,
arbitrary convex four-bodies or a new ambient local theorem.

## Source and complete implication

[Lead report](../orientation-wide/class-proof/report.md) owns the canonical
capacity-one minimizer implication, exact physical normalization and all-ten
candidate formula construction. [Independent review](../orientation-wide/class-proof/review.md)
accepts that implication and the following exact arithmetic coverage:

- All 21 open denominator vectors, 210 basis occurrences/97 distinct supports,
  signed residual/area/gap coefficients, and all 258 closed denominator mappings.
- Complete open clipping: 800 event-polynomial inputs, 2,408 partition intervals,
  1,416 feasible components, 390 singleton-x components and 350 intrinsic
  zero-edge exclusions. Each feasible component has a sufficient accepted
  minimum mode; rejected exploratory alternatives are not proof obligations.
- Complete internal event catalogue for 2,330 non-intrinsic-zero-edge
  domain/formula incidences and all 78 added quadratic-cut incidences,
  including cuts whose adjacent open intervals are infeasible.
- All 2,580 original closed basis incidences plus 78 added event incidences:
  1,700 feasible and 958 empty fibers, with exact ordered F(alpha) clipping,
  minimum values, x>0 boundaries and all 190 closed infinite-tail criteria.

Every canonical capacity-one area minimum is a vertex modulo translation,
by the fixed-normal Minkowski identity and Brunn–Minkowski equality. A
benchmark-relevant five-edge vertex has three independent active fit rows,
hence is in the ten-basis list. Zero-edge bodies have ratio at most one,
strictly below HKO. The exhaustive nonnegative vertex gap therefore gives
the bound for all support offsets, rather than merely for enumerated bodies.

Inherited inputs remain explicit: the [complete denominator acceptance](orientation-joint-final-acceptance.md),
the canonical vertex-minimum argument accepted in the independent review,
and the primary triangle/quadrilateral theorem. All ten templates and all
five closure circuits remain part of the capacity formula.

## Equality and final review boundary

Closed testing has 25 raw zero occurrences, all at u=1/sqrt(5), and no
identically zero x-gap among its 1,700 feasible checks. This does not yet
exclude open even-root zeros or flat/singleton gap components, deduplicate
physical body classes, or verify all-ten primal fits/active dual and direct
physical shoelace for each surviving equality body. Those remain selected
within the unchanged 01:23 cap. Root owns domain-source/dashboard integration.

Read-only replay entry points are `review_formulas.py`, `review_clip.py`,
`review_signs.py`, `review_open.py` and `review_closed.py` in the class-proof
directory, run using `python -B` from the repository root. These replay exact
retained inputs; regenerating producer evidence is a separate operation.
