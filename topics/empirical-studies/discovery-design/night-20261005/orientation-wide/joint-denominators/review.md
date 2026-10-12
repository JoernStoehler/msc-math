# Independent complete joint-denominator review

Receiving lead: `direct_cover_lead`. Selected stage received 2026-10-05
00:20 UTC. Owned outputs are this review and `review_joint.py` only. No
body-feasibility, body-positivity, or global HKO computation is part of this
review. Production inputs and outputs belong to the extraction specialist.

**Accepted at 00:36 UTC: complete exact joint-denominator coverage. The
independent read-only audit exited zero and passed all individual events,
cross-circuit refinement, formula vectors, and raw closed fibers. No body
optimization or global HKO theorem is asserted.**

## Required chain and conceptual validity

The accepted preflight gives complete finite source candidates and exact
dominance screening, with survivor counts \((5,6,8,7,6)\). Distinct screened
lines are strictly dominated at interior u and positive x, so the survivor
envelopes determine every open interior region. The raw 121-candidate
catalogue per circuit must nevertheless be restored on every closed
orientation event fiber, and for boundary-only or persistent x=0 ties.
Circuit zero's previous one-strip review alone gives no topology statement
about the other four circuits.

For each circuit the complete event predicates are pair slope differences,
pair intercept differences, and triple slope/intercept determinants. Their
degrees in u are at most one, one, and two, respectively. On a root-free open
u strip, slope order and triple orientation are fixed, so the upper hull's
combinatorics is fixed. Intercept events cover entry through x=0, slope events
cover parallelism and changes at infinity, and triple events cover isolated
winners and changes of finite boundary order. Identically zero predicates
must be distinguished from isolated roots and incorporated as persistent
coincident/collinear relations. Double roots still belong to the event set;
the absence of a sign change is not an exclusion proof.

Every retained active x boundary is of the form \(N(u)/D(u)\), with N and D
affine. Its denominator must be nonzero throughout its open topology strip
with certified sign; the ordered-slope description normally proves this.
An endpoint zero of D is allowed only with its separate endpoint fiber and
infinite-tail interpretation. Constant or coincident line cases cannot be
represented by dividing by zero.

Individual hull topology does not fix the order of boundaries from different
circuits. A complete joint certificate therefore refines each individual
topology strip at every relevant root of
\(N_iD_j-N_jD_i\), of degree at most two, for its actual active curves.
An identically zero numerator identifies a shared curve. Simple and double
roots are retained; a root may be discarded only with an exact domain or
inactive-curve proof. After refinement, a sample's ordered union is valid
throughout that substrip only because all equality and denominator roots
have been exhausted. Equal curves keep all incidence labels.

The ordered union plus x=0 and the final unbounded tail defines the joint
D-vector cells. Each cell records one exact attaining bilinear formula for
each of five circuits, with all tied alternatives retained where needed.
Within a circuit adjacent formulas agree exactly on its break curve. A
closed orientation fiber requires an independently computed whole raw
upper envelope in x, including zero-width cells, isolated/shared ties,
boundary-only raw winners, and asymptotic maximal-slope labels. Adjacent
open-strip records cannot replace that fiber computation.

## Root fields and physical normalization

Coefficients remain in \(\mathbb F=\mathbb Q(\sqrt5)\). Individual and
cross-curve quadratic roots can require degree-two extensions of that field.
If a root leaves \(\mathbb F\), a complete certificate must provide an
exact real embedding, isolation interval, exact ordering, and zero/sign
evaluation with the selected positive \(\sqrt5\). Rational resultant roots
from the conjugate negative-\(\sqrt5\) embedding must not be inadvertently
included as physical events. A nonsquare discriminant is harmless only when
an exact whole-strip range test excludes every root; otherwise an unsupported
extension engine is a stopping condition. A finite-precision ordering is
insufficient.

The reviewed covector transform is
\(C_u=\left(\begin{smallmatrix}1&-u\\\tau^2u&1\end{smallmatrix}\right)\),
where \(\tau^2=5-2\sqrt5\). Its physical normal determinant is
\(xd(u)\), \(d(u)=1+\tau^2u^2\). Thus the canonical support area numerator
P gives \(\operatorname{area}K=P/[xd(u)\sin(2\pi/5)]\), and at complete
capacity one the ratio is \(xd(u)/(5P)\). Invertibility preserves all five
closure rays. The denominators must continue to use all ten genuine
templates and the same scaled covector/support convention. This stage
certifies those five denominators, not any area-gap positivity conclusion.

## Consequential audit specification

The independent read-only audit was required to rederive event polynomials from the
retained source coefficients, verify all roots/embeddings and interval
orders, reconstruct raw closed fibers, certify active curve denominators,
rederive cross-circuit refinement events, and check the joint cell formulas
and shared-boundary compatibility. Actual counts, cost, accepted coverage,
or a precise stopping obstruction are recorded below.

## Actual first field stop and consequential qualification

The historical first producer packet completed circuit zero and stopped while
extracting circuit one's triple \((61,76,119)\). The stop polynomial, in
ascending u powers over the positive-\(\sqrt5\) embedding, is

\[
\left(\frac{17}{10}-\frac{19\sqrt5}{25}\right)
+\left(\frac{21}{5}-\frac{47\sqrt5}{25}\right)u
+\left(-\frac{17}{2}+\frac{19\sqrt5}{5}\right)u^2.
\]

Its discriminant is \((18859-8434\sqrt5)/125\), with rational norm
\(101/15625\). This norm is not a rational square, so the discriminant has
no square root in \(\mathbb Q(\sqrt5)\). The root field obligation is genuine
for this full survivor arrangement polynomial. The producer retained its
rational norm polynomial and an exact rational sign-changing bracket inside
\((0,U)\). The derivative is negative throughout the entire strip, proving
that this is its unique in-strip root, not merely one sampled root.

The independent read-only `review_joint.py` rederives the triple polynomial,
checks the discriminant and nonsquare norm, reproduces the rational norm
polynomial, verifies the positive-\(\sqrt5\) embedding's bracket signs and
derivative, and verifies the current source catalogue's SHA-256 and exact
identity with the accepted preflight input. Circuit zero's completed result
matches the accepted review exactly. Replay passed in approximately half a
second without writes.

Crucially, this is **not** an active-envelope event. Write the concurrent
x coordinate of lines 61 and 76 as \(N(u)/D(u)\), where
\(N=b_{61}-b_{76}\) and \(D=a_{76}-a_{61}\). Exact endpoint signs certify D
nonzero and this x positive throughout the retained rational bracket. Raw
candidate 75 is strictly above their concurrent value throughout that whole
bracket: the numerator

\[
(a_{75}-a_{61})N+(b_{75}-b_{61})D
\]

has the same strict sign as D, proved by exact endpoint and quadratic-vertex
extrema. At the triple root, all three lines are therefore below the true
upper envelope. The independently retained checker executes this exclusion.

Consequently that first stop correctly established that the initial engine
does not enumerate this extended-field arrangement root. It does **not**
establish that an extended-field root engine is indispensable for the active
denominator envelopes: this specific event can be discarded using the exact
inactive-event witness just proved. Other events may still require such an
engine. A targeted continuation can retain this witness, exclude this unique
inactive root, and inspect the next event; alternatively a conservative
implementation can adjoin the quadratic extension. Neither complete
individual circuits 1–4 nor cross-circuit joint cells have been accepted yet.

Read-only replay (the historical obstruction is retained as
`first-field-stop.json`, and the same checker proceeds to the final packet):

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/joint-denominators/review_joint.py
```

## Accepted individual extraction and independent field arithmetic

The producer subsequently supplied individually isolated quadratic-extension
roots and completed all five individual envelopes. Its event counts including
orientation endpoints are \((2,4,14,8,5)\), hence open-strip counts
\((1,3,13,7,4)\). The independent auditor rederived all 319 event polynomials,
checked every complete in-strip root set and real embedding, checked their
exact order, and certified all 61 entire raw-catalogue x fibers: 28 open-strip
samples and 33 closed individual event fibers. This earlier individual audit
passed. There is one persistent zero predicate; it was retained explicitly
instead of being confused with an isolated event.

`review_joint.py` imports only the previously retained independent reviewer's
rational-pair arithmetic definitions. It imports no producer arithmetic or
extraction code. Its separately implemented quadratic-fiber arithmetic reduces
values to \(p+q\alpha\) over \(\mathbb Q(\sqrt5)\). Each root has a checked
irreducible monic quadratic, branch, and sign-changing isolating interval
with nonzero derivative. The sign of \(p+q\alpha\) is decided by refining the
root interval against \(-p/q\); irreducibility excludes equality with that
base-field threshold. Different irreducible root polynomials are ordered by
exact interval refinement, with equal normalized polynomial/branch records
identified. No compositum arithmetic or floating-point ordering is used.

Root completeness is independently checked by linear solving, quadratic
Vieta conjugation when one root is present, or endpoint/vertex range exclusion
when none is recorded. This covers repeated roots and the positive-\(\sqrt5\)
embedding without assuming that a discriminant is square. For a persistent
zero polynomial, no isolated root list is required; its relation remains
identified throughout the strip.

Whole closed-fiber certification checks every recorded cell against every
one of its circuit's 121 raw lines: affine differences are nonnegative at
both finite x endpoints, or at the left endpoint and in leading slope on an
infinite tail. Cells cover x from zero to infinity without gaps. All finite
boundary ties are recomputed from all raw values. Thus removing zero-width
lines from the positive-width cell list does not lose isolated winners: they
are present among the full raw ties at that boundary. Asymptotic maximal
slopes are checked separately.

## Accepted final joint audit

The final packet records 18 merged individual orientation events and 17
topology strips. Actual active boundary counts are 14–18 per strip. It records
1,659 cross-circuit comparisons, 40 persistent shared-curve comparison
occurrences, one additional
orientation event, 19 final closed fibers, 18 joint open strips, 268 open
cells, and 258 closed cells. The independent final audit passed all coverage
checks for these records, including every one of the 95 merged raw circuit
fibers. It exited zero at approximately 00:36 UTC.

The independent audit reconstructs each topology strip's actual active curves
from the already certified individual patches; verifies their numerators,
denominators, and nonzero positive denominator signs throughout the open
strip; rederives every cross-circuit equality polynomial; and independently
checks all roots within the strip, including roots excluded at strip endpoints
and persistent identities. It then checks the exact ordered boundary groups,
all joint-cell attaining formulas, and all shared-boundary D vectors against
the five raw catalogues. Every merged closed orientation fiber is certified
for all five full raw x envelopes, plus the joint-cell and point D vectors.

Boundary agreement follows both from the checked line-pair identity defining
each break curve and from exact raw maximum values. Finite maxima of bilinear
formulas are continuous, so closed-event values agree with the neighboring
open-cell limits; no continuity of symplectic capacity is needed here.

Physical scope remains the fixed distortion axes relative to fixed regular
Q, with normals \(\operatorname{diag}(x,1)R_\theta n_i\). The period/reflection
reduction covers every theta in that family; it does not enlarge the class
to arbitrary \(GL_2\) transforms. Capacity uses all ten templates and all five
circuits in the reviewed scaled normal convention. No area-gap positivity,
body-feasibility result, or global HKO conclusion is asserted by this stage.

The accepted `joint-denominators.json` has SHA-256
`602a61a96420a9f75ceb112d94473e0a7def14a34f467ea0bdf08f47ea88fa7f`.
This hash was independently recomputed after the audit. The final executable
PASS reports 319 individual event polynomials, 1,659 cross-curve comparisons,
19 merged orientation events, 18 joint open strips, 268 open cells, and 95
entire closed raw circuit fibers. The checker imports no producer arithmetic
and writes no files. Historical preliminary output about the first incomplete
stop is followed by the final `PASS complete joint denominator coverage`;
the former is preserved evidence, not the final acceptance status.
