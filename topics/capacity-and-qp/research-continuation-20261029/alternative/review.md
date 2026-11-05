# Independent scoped check — 29 October 2026

Reviewer: native agent `/root/alternative_mechanism/cover_contact_lemma`.
Receiving owner: `/root/alternative_mechanism`; root owns final integration.
The reviewer first checked the mathematical envelope construction separately,
then read the complete producer and four-case output. Its private native return
reported **PASS** for those scopes. This is agent mathematical/code review,
not human acceptance, external refereeing or novelty assessment.

Reviewed producer SHA-256:
`f644c47e8d613e9e4464a86dbce078006d2dcc74764c8c9251405c9dbdff6dfc`.
Reviewed four-case output SHA-256:
`a4bd40b5de251cc330d01273a979e7421feb9a276c7f6c1a44ba56945ccd3df9`.
The written proof was assembled from the same checked argument; source review
of the exact final prose is separate from that argument check.

The reviewer checked distinct-height sorting, interval interpolation, convex
left/concave right slope inequalities, containment of every point, area equality
with the trapezoid objective, and reconstruction from actual hull endpoints.
Repeated heights and one/two-height cases were considered explicitly. For a
nonhorizontal anchored group, the reviewer independently derived pointedness
and boundedness of the optimum face from the absence of nonzero zero-cost
recession directions. It warned correctly that contact rank is not a forest
or a statement about every hull edge's direction.

The implementation audit checked exact height sorting/grouping, convexity-row
signs, point containment, weights, dual convention `A^T y=-c`, nonnegative dual
weights, exact full primal slacks, exact primal/dual objective equality, and
exact reconstructed hull area. The shear `(x,y)->(x,y-s*x)` has determinant one
and correctly converts original direction `(1,s)` to horizontal. The floating
solver only proposes active sets; exact reconstruction rejects erroneous
proposals. The reviewer found no concrete defect.

The receiving owner then added `replay_envelope.py` and ran it successfully
against all retained cases, without invoking an LP solver. That replay shares
the reviewed LP construction and existing fixture arithmetic; it is an exact
certificate replay, not a separately implemented independent evaluator.

## Final Steiner and center-restoration prose check

The same independent reviewer separately read the exact final `steiner-gap.md`
(SHA-256 `c8e9138b2e49beb601bd3ccd9ed16a2f153e8c11af065377b73d7f16f3e868bc`) and reported
scoped PASS for the section-width containment, segment lower-bound proof,
Horner-normalized formulas, kite characterization and limitation of the
center-restoration conclusion to its stated offsets. The owner replayed all
four Steiner certificates and the center-restoration certificate exactly.
No LP solver was invoked by those replays.
