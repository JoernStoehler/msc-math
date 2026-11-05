# Next independent pattern crux: preserve all unsigned ridge areas, change capacity

Prepared 17 October 2026 from a bounded source pass. Receiving owner: root;
execution and any retained result belong on the patterns branch. This memo is
scratch only. No experiments or PM mutations were performed for this selection.

## Concrete end state and necessary steps

The preferred success is an exact rational pair of polygon products with the
same **complete multiset** of normalized individual unsigned symplectic ridge
areas, including multiplicities, but materially different systolic ratios.
That would eliminate every symmetric function of that list, including
area-dependent weights, as a universal predictor. It would leave incidence-aware,
signed, and geometry-dependent weights open. A matched list is necessary to
make the obstruction stronger than the existing sum/max counterexamples;
matched volume and exact capacities are necessary to avoid normalization and
historical-label loopholes.

This task is independent of all five pending author judgments. The selection is
not an assertion that the counterexample exists. A structural failure of this
construction is a useful bounded negative; no numerical fitting follows it.

## Alternatives and competing rubrics

| Option | Expected insight and upper-tail/global route | What a naive rubric rewards wrongly | Bounded preliminary cost and decision |
| --- | --- | --- | --- |
| Signed Minkowski-summand flip preserving the full unsigned ridge list | Tests whether Jörn's individual-area prior retains enough information at all; a positive result forces closure/order/sign information before pursuing upper-tail conjectures. It is an obstruction, not an upper-tail predictor or maximality theorem. | Rewarding any tiny capacity difference, or claiming all possible weighted ridges fail when only area-only symmetric functions fail. | Two exact 6×3 bodies first; at most a small predeclared rational family if the seed ties. Standard-library rational enumeration, no dataset or optimizer. **Recommend.** |
| Explain one stopped triangle selector's failure through angular compatibility | Could identify a genuinely geometric compatibility term missing from separately minimizing outer areas, eventually informing target-free selection. Use named worst rows in FROZEN-TAIL.md, not another complete data fit. | Merely selecting the next-best support after observing capacity; a tighter upper bound alone is no mechanism. | Several-row geometry inspection plus exact witness comparison is bounded, but distinguishing a new rule from post-target support tuning is less clean. Second choice only after a concrete invariant hypothesis. |
| Fixed-pentagon general-matrix cone positivity | A proof or counterexample would substantially expand/refute the strongest asymmetric-partner HKO route. | Spending heavily on a parked seven-variable, 610-row reduction because its desired conclusion is attractive. | Existing reduction is not a small implementation task; only inspect its obstruction first. Defer substantial execution to a concrete separate selection/spending packet. |

Rubrics considered: regression scores (reward bulk fit), number of certified
rejections (reward easy cases), smallest bound (reward polishing), and one
information-theoretic threshold or domain change. The last favors option 1;
user rating E=1 limits its direct chapter value, but it prunes the entire stated
area-only prior rather than repeating a single failed statistic. Stop if it
only produces tiny differences without a clean symbolic explanation.

## Minimal task ready to assign now

Use rational triangles, initially

* A = conv{(0,2),(-1,-1),(1,-1)}, symmetric about the y-axis;
* B = conv{(3,0),(-1,-2),(-1,2)}, symmetric about the x-axis;
* Q = conv{(0,0),(3,0),(1,2)}, a scalene triangle.

Translate each triangle by its centroid before forming P+ = A+B and P− = A−B;
translation does not affect the argument. Take the exact convex hull of nine
vertex sums for each. There are no parallel edge directions among the six
summand directions in this seed, so neither product needs a two-normal closure
case. Recheck this explicitly rather than assuming it for variant seeds.

Reflection in the y-axis fixes A and sends B to −B (up to translation), so
P+ and P− have equal area. Minkowski addition merges cyclic edge lists: their
edge multisets differ only by signs on B's three edges. For any common Q, the
complete nonzero ridge list of P×_L Q is {|e_i·f_j|}; hence P+×_L Q and
P−×_L Q have exactly equal lists, equal volumes and equal normalized lists.
Include the zero factor-face ridges with their equal multiplicities when
stating the complete invariant. The cyclic incidence order may differ, which
is precisely information the unsigned multiset discards.

Compute exact capacities of these **two controlled examples**, using all
positive normal-closure triples and all 120 cyclic interleavings per pair.
The point of complete enumeration here is to prove an exact separating
example; this is not selected as an all-body capacity method or as selector
tuning. Enumerate all supports, not only the minimum-area triangles. Compare
with an independently reconstructed HK objective and the sparse-product theorem.
If the two capacities tie, a small fixed rational variant list in A's height,
B's aspect, and Q's scalene vertex is permissible; freeze it before values are
read, cap execution at five wall minutes, and report ties/partial failure rather
than launching global search. A difference of at least 5% in systolic ratio is
a useful preliminary screening target, not a new scientific materiality rule.
If only tiny effects occur, retain the obstruction assessment and stop.

## Exact entry points

All repository paths below are relative to
`/workspaces/msc-math-worktrees/patterns-2026-10-17`.

* `patterns/upper-tail/triangle_envelope.py`: `cross`, `dot`, `polar_order`,
  `vertices`, `area`, `bound`, `witness`, and `ORDERS` give standard-library exact
  building blocks. Its `triangles` currently returns **only minimum-area**
  triples; use a scratch variant returning every positive closure triple.
* `topics/products/product-qp-six-facet-reduction.tex`: accepted sparse support
  completeness source, needed to turn exhaustive support evaluation into exact
  capacity. Do not confuse a feasible witness with a lower capacity bound.
* `patterns/upper-tail/verify_frozen_tail.py` and
  `patterns/upper-tail/verify_triangle_certificate.py`:
  independently reconstructing areas and checking a direct HK word are useful
  verifier patterns; verifier feasibility alone does not prove exhaustive minima.
* `patterns/ridge-functionals/analyze.py`: `edges` and the `abs(edges(p) @
  edges(q).T)` expression specify the product ridge-area convention.
* `patterns/upper-tail/FROZEN-TAIL.md` and `tail-rows.jsonl.gz`: option 2's exact
  stopped-selector controls, especially `random_5x5_s1_221` and
  `random_5x6_s2_150`; do not rerun the 3,072-row diagnostic.
* `topics/products/pentagon-arbitrary-partner/general-matrix-cone-reduction-20261005.md`
  and its parent README: option 3's existing reduction and parked boundary.

No retained historical input or numeric label is needed for the recommended
task. Output script, exact input vertices, full matched ridge lists, exhaustive
support accounting and certificate/checker go first under
`/tmp/msc-math/next-pattern-task/20261017/` (separate owned subdirectory).

## Sources inspected and limits

Read `patterns/OWNER-FOLLOWUP.md`, original README/GALLERY,
`patterns/upper-tail/FROZEN-TAIL.md`, REPORT and selected script functions;
`topics/products/README.md`, fixed-pentagon partner README, and the historical
triangle-classification empirical README. A targeted text search located
existing circulation and closure routes. This is not an exhaustive repository
or literature search; no novelty claim or completed construction is inferred.
