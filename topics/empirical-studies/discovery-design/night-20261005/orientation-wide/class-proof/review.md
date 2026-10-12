# Independent orientation-wide class-proof review

Selected stage received 2026-10-05 00:39 UTC. Receiving owner:
`direct_cover_lead`. Owned outputs are this review and independent reviewer
code/evidence in this directory only. No producer code or certificate is
owned by this reviewer. The target is the fixed-axis family with normals
\(\operatorname{diag}(x,1)R_\theta n_i\), not general GL2 or arbitrary
pentagon partners.

**Final status: full sharp bound and complete equality accepted for the
selected fixed-axis family, conditional on the explicitly inherited complete
denominator, canonical minimization and primary lower-edge inputs. All fresh
formula, domain, sign, equality and physical controls passed independently.**

## Consequential minimization and equality implication

The inherited complete denominator certificate, SHA-256
`602a61a96420a9f75ceb112d94473e0a7def14a34f467ea0bdf08f47ea88fa7f`,
defines complete capacity by all ten templates and all five closure circuits.
The selected domain is \(x>0\), \(0\le u\le1/\sqrt5\), with
\(u=\tan\theta/\tau\), \(\tau^2=5-2\sqrt5\). The reviewed physical
normal/support convention gives capacity-one ratio
\(x[1+\tau^2u^2]/[5P(h)]\).

At each fixed parameter, canonical capacity-at-least-one supports form a
closed three-dimensional translation-quotient polyhedron. Its area minimum
exists and has capacity exactly one. Every minimizing body class is a
vertex, by canonical Minkowski linearity and Brunn–Minkowski equality; this
strong statement remains valid at canonical zero-edge boundaries. Raw
redundant offsets are body-equivalent after canonicalization and cannot add
body equality classes.

The source-backed bound at most one for triangular/quadrilateral partners
excludes every zero-edge vertex from attaining or exceeding HKO. Thus a
counterexample yields, by area minimization at its fixed parameter, a genuine
five-edge counterexample vertex with three independent active fitting rows.
All ten triples of fitting rows are nonsingular. Exhaustively testing their
feasible formulas therefore suffices for the upper bound.

After this upper bound is proved, every normalized HKO equality body is an
area minimizer in its own fixed fan and therefore is one of those vertices.
This excludes additional nonvertex body equality, even if raw support
coordinates have flat redundant fibers. Before the upper bound is proved,
one must not already classify every threshold-level body as a vertex.
Extra active fits are handled by duplicate basis incidences, not omitted as
singular placements. A formula with a zero edge at an endpoint is excluded
from HKO equality by the strict gap between one and HKO, rather than deleted
from feasibility coverage.

## Formula and domain review specification

Each accepted D-vector cell supplies five bilinear formulas over
\(\mathbb F=\mathbb Q(\sqrt5)\). A fixed fitting-basis inverse is constant,
so each candidate canonical support vector is bilinear in x,u. Every fit
and edge residual is also bilinear. For each D-domain incidence, intersect
all ten residual inequalities with that joint cell's x interval and the
strict physical condition x>0. Retain every disconnected u relevance domain,
empty/singleton fiber, vanishing slope, persistent/shared bound, both u
endpoints, closed algebraic orientation fibers, and infinite tail.

Deduplicating support formulas must not discard their D-domain incidences.
A candidate may be feasible in only part of one cell, including an isolated
fiber. In particular a closed orientation fiber is evaluated in its actual
ordered quadratic extension; plugging a nearby rational orientation into
the same nominal formula does not certify that fiber.

## Gap and stationary relevance

For each candidate
\(q=P-x[1+\tau^2u^2]/(3+\sqrt5)\) is quadratic in x, with coefficient
polynomials in u of degree at most two. A finite linear-fractional x bound
\(-b(u)/a(u)\) yields an endpoint numerator of degree at most four after
clearing the positive square a(u)^2. Convex stationary minima have numerator
\(4q_0q_2-q_1^2\), also of degree at most four. Infinite tails require the
quadratic leading sign, then the linear sign when it vanishes. Linear or
constant x gaps and identically zero numerators require separate handling.

The stationary point \(-q_1/(2q_2)\) matters only where q2>0 and it lies in
the feasible x fiber. Comparison with a linear-fractional bound introduces
numerators of degree at most three, with denominator signs fixed first.
An unconstrained negative stationary value outside the feasible fiber is
not a counterexample or a failed bound.

On a fixed relevance domain, square-free factorization and Sturm counts can
certify all signs and multiplicities. A positive probe with no interior
odd-multiplicity roots establishes constant positive sign away from even
zeros; boundary signs and every even zero must still be tested exactly.
A repeated quartic factor has degree at most two, but clipping endpoints
can independently require ordered cubic roots. Hence a quadratic-only
equality adapter is insufficient by itself for all relevance-domain signs.
Exact gcd/shared-root tests and isolating-interval evaluation can avoid a
global compositum; numerical sorting cannot replace them.

## Physical acceptance controls

Every surviving equality or above-benchmark body requires five positive
physical edges, complete capacity one from all ten primal fits plus an
active maximal-template dual, and exact physical area/product shoelace.
All parameter/body zero components must be deduplicated with their complete
domains. Translation, positive dilation, and the accepted orientation
period/reflection are the allowed reconstruction symmetries. The known
regular HKO body at (x,u)=(1,1/sqrt5) is a control, not an assumed exhaustive
answer.

Actual workload, independent arithmetic checks, unresolved predicates, and
final accepted scope will be recorded below as evidence becomes available.

## 00:47 checkpoint: accepted coefficient derivation and closed mapping

The read-only independent `review_formulas.py` uses Cramer's rule over rational
pairs \(a+b\sqrt5\), rather than the producer's matrix inverse or arithmetic.
It rederived all 21 open D-vectors and 210 fitting-basis formulas, giving 97
distinct open support formulas. It checked every fit and canonical edge
residual, area and gap coefficient, finite residual-bound endpoint numerator,
stationary numerator, and stationary-membership numerator against
`formulas.json`. Every normalized polynomial reference was reconstructed with
its original nonzero multiplier, including its sign; monic normalization was
not treated as preserving inequalities automatically.

Naively taking the first attaining raw label in closed-fiber cells yields
23 D-vector representatives and 230 basis formulas, with 106 distinct supports
over that larger representative set. This does not create missing body
domains. The producer retains, for every closed cell, matching vectors among
the 21 open representatives. The reviewer independently verified equality
of **both slope and intercept** at the actual ordered quadratic-extension
orientation, rather than accepting just attaining IDs. All 258 closed
domains pass this identity. It also verified 497 adjacent closed-point
incidence value equalities, so alternative formulas agree at the shared x
endpoints. These checks preserve all 268 open and 258 closed D-domain
incidences, corresponding to 2,680 and 2,580 basis incidences.

The five distinct open support classes with an identically zero edge,
occurring in 20 of the 210 formula occurrences, were independently confirmed.
Where feasible they are genuine lower-edge bodies and have strict HKO gap
by the accepted at-most-one theorem. This optional pruning does not omit
their domain incidences or any other class acquiring a zero edge only at
some parameters.

Before relevance clipping, the raw residual-bound endpoint numerator
occurrences have degrees: 1,300 quartic, 30 quadratic, and 44 identically
zero. All 210 stationary numerators are quartic. Raw stationary-membership
occurrences are 1,344 cubic and 30 quadratic. These are occurrence counts,
not numbers of relevant predicates or a complete sign-proof workload.
In particular they exclude the extra predicates arising from D-cell x
boundaries and bound-order switches, which remain necessary unless proved
redundant by an exact domain argument.

Replay, without writes:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_formulas.py
```

This replay passed. It reuses only definitions from the independently
reviewed quadratic-fiber adapter, without running the prior reviewer or
producer. The accepted lead report's minimization/capacity/equality implication
was also read-reviewed and is valid at the stated fixed-axis scope.

Acceptance so far is **coefficient derivation and domain-equivalence mapping**.
Full feasibility clipping, relevant sign queries, equality-zero enumeration,
and physical optimizer controls remain unaccepted.

## 00:55 checkpoint: open clipping and exact sign machinery accepted

`review_clip.py` reconstructs the complete open feasibility clipping with
independent rational-pair arithmetic, its own quadratic root formulas and
monotone isolating brackets, and the earlier independently reviewed ordered
boundary adapter. It imports no producer implementation. It reconstructs all
residual slope/intercept and bound-order events, including actual D-cell
curves, and tests every resulting interval before comparing accepted segments.
The replay checked 800 unique event polynomials and 2,408 partition intervals,
matching exactly 1,416 feasible components, 390 singleton x components and
350 intrinsically zero-edge basis/domain incidences. All 9,912 signed relevant
predicate references were reconstructed. Runtime was 38.126 seconds.

`review_signs.py` independently implements polynomial Euclidean division, gcd,
Sturm sequences and one-sided endpoint signs over Q(sqrt(5)). Fifteen tests
include repeated roots, roots at either excluded endpoint, shared factors,
positive sqrt(5) embedding, cubic/quartic roots and actual ordered quadratic
extension endpoints. Construction and remainder-identity validation of all
400 relevant polynomial sequences took 0.506 seconds.

`review_open.py` independently checks every query in `open-proof.json`: an
alternative multiplicity-layer algorithm computes its odd root factor;
Sturm counts, retained sequence identities, exact probe and endpoint signs
are recomputed. It then checks each minimum implication against the original
signed formula references and bound-denominator signs. All 1,416 component
proofs pass: 591 concave/linear, 390 singleton, 400 lower-endpoint derivative
and 35 upper-endpoint derivative proofs. There are 1,808 exact queries, of
which 1,584 pass. The 224 rejected exploratory queries do not invalidate the
proof: their segment uses a different accepted query. No infinite feasible
x tail remains in this open clipping. Runtime was 13.591 seconds.

Skipping an endpoint whose active canonical edge residual vanishes is justified
by the accepted strict HKO gap for lower-edge bodies. The reviewer checked
the exact zero-edge substitution identity rather than accepting its label.

Replay these read-only audits with `python -B` on `review_clip.py`,
`review_signs.py`, and `review_open.py` in this directory.

This accepts nonnegative gap throughout the retained **open orientation
components**. Full class coverage still requires all original closed D
orientation fibers and every new internal feasibility-cut orientation,
including isolated feasible fibers. Odd-root sign certification does not
classify equality: all even roots, endpoint zeros, flat x fibers and actual
body/capacity controls remain necessary.

## 00:58 checkpoint: full closed coverage and sharp bound accepted

The revised `review_clip.py` also reconstructs and compares the complete
internal feasibility-event catalog: all 2,330 non-intrinsic-zero-edge
domain/formula entries and all 78 internal cut incidences match
`feasibility-events.json`, including events with infeasible adjacent open
components. This replay took 37.879 seconds. Excluded intrinsic zero-edge
formulas satisfy the strict primary lower-edge gap everywhere, so their
internal-cut coverage is unnecessary for the selected sharp bound or equality.

`review_closed.py` exhaustively re-solves all 258 original closed D cells
with ten bases, and all 78 new internal feasibility-cut incidences, using
independent arithmetic in each actual ordered F(alpha). It independently
checks the original D slope/intercept equivalences, feasibility bounds,
empty/singleton cases, evaluated gap coefficients, endpoint/stationary
values, edge signs, excluded-x=0 limit and infinite-tail criteria. All
1,700 feasible checks and 958 empty fibers match. The 190 feasible infinite
tails satisfy their quadratic/linear leading-sign predicates. Runtime was
23.358 seconds. Original closed-point D equivalences were independently
accepted at the earlier coefficient checkpoint and are unchanged.

There are 25 zero test occurrences, all at u=1/sqrt(5). None of the 1,700
closed fibers has an identically zero polynomial gap in x. These are
occurrences, not yet distinct physical equality classes.

Combining the accepted open and closed checks proves the sharp HKO gap
bound on the selected fixed-axis normal family, conditional on the
explicitly inherited complete denominator certificate, canonical vertex
minimization implication and primary triangle/quadrilateral bound. The
known HKO control supplies sharpness. Complete equality classification
still requires exclusion/enumeration of even endpoint-gap zeros in the
open components, possible flat open gap fibers, and exact physical
capacity/area/body controls for the surviving zero bodies.

## 00:59 checkpoint: complete candidate equality census accepted

`review_equality.py` independently recomputes all 974 distinct endpoint-gap
**full** root counts, including even roots. Every count is zero in its open
orientation interval; there are no identically zero endpoint polynomials.
The accepted open minimum modes are all endpoint-controlled, so strict
endpoint gaps also exclude interior-x equality, including linear/constant
quadratic degeneracies. No stationary-only zero remains untested.

The reviewer independently deduplicated all 25 closed zero occurrences
by exact parameter and canonical support values. They yield exactly one
canonical body at x=1, u=1/sqrt(5). No identically zero closed x-gap fiber
exists. The replay took 3.952 seconds and reads only retained certificates.

Complete **candidate** equality is accepted. The remaining acceptance
obligation is fresh exact physical reconstruction, all-ten primal fits
and active maximal-template dual, direct area/product shoelace and body
interpretation under the stated symmetries.

## Final acceptance, 01:03 UTC

`review_physical.py` freshly reconstructs the ten fixed-Q triangles and
regular source covectors using independent rational-pair arithmetic. It
recomputes the rotated physical normal convention at x=1, u=1/sqrt(5),
adjacent support-line intersections, actual five positive sides, and direct
K/Q shoelaces. It finds all ten primal template translations and 20 active
maximal-template/circuit dual pairs, proving complete capacity exactly one.
All 50 direct geometric template/circuit denominators agree with the new
physical certificate; their maxima also agree with the accepted complete
joint denominator catalogue evaluated at the equality point.

It independently verifies the physical metric's equal five side lengths
and circumradii, and the canonical Y support's identity as a translate of
constant regular support D. The direct physical product ratio is exactly
(3+sqrt(5))/5. Fresh normals, template vertices, placements/slacks, K/Q
vertices/areas, metric values and translation identities agree with
`physical-control.json`. The read-only replay took 0.242 seconds.

**Accepted result.** For fixed regular Q and the selected family of partner
normals diag(x,1) R_theta n_i with x>0 and arbitrary support offsets, the
sharp HKO bound holds. In the accepted reduced orientation domain, the only
normalized canonical equality body occurs at x=1, u=1/sqrt(5) and is the
regular HKO partner with covector rotation +pi/10. Translation, positive
dilation and the previously accepted orientation-period/reflection
symmetries restore the equality class outside the reduced normalization.
Raw redundant support offsets add representations of that body, not
additional body classes. The accepted every-minimizer-is-a-vertex
implication excludes additional nonvertex equality bodies after the bound;
the strict primary lower-edge bound excludes three-/four-edge equality.

This is a theorem for the explicitly selected fixed-axis normal family.
No general GL2 or arbitrary-pentagon classification, literature-novelty
claim, thesis promotion or formal-source change is established here. The
inherited inputs are the accepted complete fixed-Q denominator catalogue,
canonical support minimization/Minkowski-equality argument and primary
triangle/quadrilateral bound; all newly generated arrows downstream of
those inputs were independently checked as described above.

Reviewer outputs: `review.md`, `review_formulas.py`, `review_signs.py`,
`review_clip.py`, `review_open.py`, `review_closed.py`,
`review_equality.py`, `review_physical.py`. The code is read-only. No
producer code, prior certificate or other owner's files were changed.
