# Exact class-proof production

Production completed 2026-10-05 01:04 UTC under the
[selected sharp-class contract](../../comparison/orientation-class-proof-contract.md).
This packet proves the finite-candidate area gap and enumerates its equality
body. The lead owns the global theorem implications and independent acceptance
in [report.md](report.md) and [review.md](review.md).
No broader GL₂ family or arbitrary-pentagon claim is made.

## Result and inherited premises

For fixed regular Q and partner normals `diag(x,1)R_θ n_i`, arbitrary canonical
supports and `x>0`, the selected orientation chamber is

`0≤u=tanθ/τ≤1/√5`, `τ²=5−2√5`.

In the scaled covector convention
`C_u=[[1,−u],[τ²u,1]]`, put `d(u)=1+τ²u²`. At complete capacity one,

`P=Σh_i h_(i+1)−((√5−1)/4)Σh_i²`,
`sys=x d(u)/(5P)`.

Every feasible ten-fit vertex in the selected domain satisfies the exact gap

`P−x d(u)/(3+√5)≥0`.

The only finite-candidate equality body has
`x=1`, `u=1/√5`, and translation-gauged supports

`h=(-1+3√5/5, 1−√5/5, -1+3√5/5, 0, 0)`.

It is a genuine regular pentagon at the HKO angle, with complete capacity one
and direct ratio `(3+√5)/5`. Restoring translations, positive dilations and
accepted orientation symmetries gives the same regular HKO class.

The extension from these candidates to every body uses inherited premises:

- Complete all-ten fitting and all-five closure circuits from the accepted
  [joint denominator certificate](../joint-denominators/report.md).
- Canonical support reduction, existence/coercivity and the independently
  accepted **every area minimizer is a vertex** implication.
- The primary triangle/quadrilateral product bound `≤1`, excluding every
  genuine zero-edge stratum from HKO equality or violation; its source and
  hypotheses are owned by [pruning-review.md](../preflight/pruning-review.md).

Those premises are not supplied by a count or arithmetic test here. The lead
and independent reviewer own their integration. In particular, equality is
extended using every-minimizer vertexhood after the sharp bound, not by
asserting every threshold body was already a vertex.

## Formula catalogue and domain coverage

[derive.py](derive.py) reads the accepted joint certificate without running an
old producer. All ten three-row fit matrices are inverted exactly in
`F=Q(√5)` in gauge `h₃=h₄=0`, with inverse identities checked.
There are **21 open D-vectors**, **210 vector/basis occurrences**, and
**97 distinct bilinear support formulas**. All five fitting residuals and all
five canonical edge residuals are retained for each occurrence.

The 268 open joint cells retain their formula incidences. The 258 closed cells
can be labelled by 23 raw tied D-vectors if one naively chooses the first
source. Instead, each closed cell records every compatible representative
from the 21 open vectors. The final closed proof checks **both actual affine
coefficients** against its raw active representative in that cell's actual
ordered F/quadratic-F embedding. Thus closed coverage rests on exact values,
not a silent label identification.

[clip.py](clip.py) splits each candidate's orientation domain at all residual
intercept/slope zeros and all affine-fractional bound-order events. It includes
the actual D-cell x-boundaries as well as all fit and edge bounds. Quadratic
event roots use the accepted ordered field engine, with no numerical sorting.
Every event-free component has a fixed feasible lower/upper bound or is empty.
It retains closed singletons and the final unbounded x-case explicitly.

Actual clipping results:

| Quantity | Count |
| --- | ---: |
| Unique degree-at-most-two clipping event inputs | 800 |
| Open feasible components | 1,416 |
| Open singleton x-fibers | 390 |
| Open nonzero-edge-formula unbounded x-fibers | 0 |
| Domain/formula exclusions by an identically zero edge | 350 |
| Remaining domain/formula event records | 2,330 |
| Internal feasibility-cut incidences | 78 |
| Distinct internal cut orientations | 22 |

The zero-edge exclusions use exact zero residual witnesses and the strict
primary comparison `1<(3+√5)/5`; they are not discarded as numerical failures.
The 78 cut incidences are retained even if both adjacent open candidate
components are infeasible. Consequently isolated feasible u-fibers are tested.
[closure-census.json](closure-census.json) exposes their union and ownership.
These 22 internal orientations supplement the original 19 full joint fibers.

## Exact sign proof

Initial raw formula predicates numbered 1,718. This was a workload census,
not the relevant proof workload. After feasibility clipping, the actual
generated sign predicates number **400**: 104 quadratic, 159 cubic,
136 quartic and one constant. This includes actual D-cell curve endpoints.
The optional factor census found 143 irreducible cubic and 102 irreducible
quartic factors; their presence did not require body arithmetic in those fields.

[sturm.py](sturm.py) performs exact polynomial Euclidean division, gcd,
square-free decomposition and Sturm variations over F. At an inherited or
new quadratic event endpoint, evaluation stays in that endpoint's ordered
F(α), reducing by its monic event polynomial. It never averages roots from
different fields or creates a global compositum.

For a square-free polynomial, the open-interval root count is

`V(left)−V(right)−1_{p(right)=0}`,

with zero terms omitted in variations. This excludes both endpoints. Repeated
roots are treated by gcd and multiplicity. A nonzero polynomial with no
interior **odd-multiplicity** roots, a positive exact nonzero probe and
nonnegative endpoint values is nonnegative throughout the closed relevance
interval. Every original normalization multiplier is restored, so a negative
leading coefficient cannot silently reverse the predicate.

[prove_open.py](prove_open.py) retains **1,808** deduplicated sign queries and
their exact polynomials, endpoints, probes, Sturm chains and signs.
All 1,416 component gap implications passed. Their minimum arguments were:

| Minimum argument | Components |
| --- | ---: |
| Concave or linear x-gap | 591 |
| Singleton x-fiber | 390 |
| Signed lower-endpoint derivative | 400 |
| Signed upper-endpoint derivative | 35 |

For a bound `x=n(u)/d(u)`, the derivative numerator is
`q₁d+2q₂n`; its sign is multiplied by the actual sign of d. A stable lower
derivative `≥0` or upper derivative `≤0` excludes an interior minimum whenever
the x-gap is convex. If it is concave, the endpoint argument already applies.
This implication remains valid if q₂ changes sign, so a cubic membership root
does not require inventing a stable q₂ branch. No stationary-numerator fallback
was needed in the final open proof.

An endpoint attained on a canonical zero edge receives the inherited strict
primary bound. Every other endpoint uses its exact cleared gap polynomial;
the clearing denominator is squared. Artificial zeros at a pole do not become
equality bodies: closed fibers are recomputed directly in their actual field.

## Closed fibers and equality completeness

[prove_closed.py](prove_closed.py) checks every original closed joint cell and
every internal feasibility-cut incidence directly. It clips all fit/edge
inequalities, retains empty and singleton fibers, tests finite endpoints,
tests a convex stationary point exactly when it belongs to the feasible
interval, and checks the final unbounded x-tail. It also treats the excluded
`x=0` endpoint by its exact limiting constant term.

There were **2,658 attempts**: 2,580 on original closed cells and 78 internal
cut incidences. **1,700** were feasible and **958** empty. The closed proof
includes **190** tails, every exact D-mapping coefficient check, and the
identically-zero x-gap flag. All gap tests passed; no flat zero-gap fiber exists.

Positivity alone does not classify equality. [equality.py](equality.py) therefore
checks **all distinct roots**, including even-multiplicity roots, for the
**974** endpoint-gap queries actually used by the open minimum proof. None has
an interior root, and no open endpoint-gap polynomial is identically zero.
Thus every open feasible component has strict gap, using strict zero-edge
comparisons where applicable. The closed proof's **25** zero records deduplicate
to exactly the single parameter pair and support body displayed above.
There are no flat open or closed equality components.

## Fresh physical controls

[physical_control.py](physical_control.py) reconstructs the equality body and
fixed Q from explicit covectors and adjacent facet intersections. It retains:

- Five distinct K vertices, all support inequalities and five positive edge
  numerators.
- All ten original template triangles, exact fitting translations and their
  five nonnegative slacks.
- All 50 direct geometric circuit/template denominators, bound to the accepted
  270-choice raw catalogue at these parameters; active duals prove the reverse
  capacity inequality.
- Direct K and Q shoelace areas and the physical τ² product conversion, giving
  exactly `(3+√5)/5`.
- Equal physical side lengths and circumradii and the exact translated constant
  support identity for Y, establishing regular geometry and the HKO orientation.

This is a fresh control, not a rerun or citation of the earlier HKO optimizer
producer. It supplies complete capacity, actual facets and physical area for
the surviving equality body.

## Essential retention and reproduction

Essential production sources are `derive.py`, `clip.py`, `sturm.py`,
`prove_open.py`, `prove_closed.py`, `equality.py`, and `physical_control.py`.
Essential math certificates are `formulas.json`, `clipped.json`,
`feasibility-events.json`, `open-proof.json`, `closed-proof.json`, `equality.json`
and `physical-control.json`. `sturm-tests.json` retains basic oracle tests;
the open proof also runs them. Required peer inputs are the accepted preflight
catalogue, joint denominator certificate and accepted field-engine source.
Independent review sources/certificates are owned by the reviewer.

The raw and relevant `factor-workload.json` / `factor-relevant.json` files and
`factor_workload.py` are **optional workload caches**. They are not used by the
Sturm proof. `closure-census.json` is an audit convenience; its incidences are
already retained in the essential event and closed certificates.

All essential producers use Python standard library and resolve inputs relative
to their own source directory. Absolute paths in metadata are observations,
not mandatory replay locations. From this directory, the full production order
is:

```bash
python -B derive.py
python -B clip.py
python -B clip.py --events-only
python -B sturm.py
python -B prove_open.py
python -B prove_closed.py
python -B equality.py
python -B physical_control.py
```

Each command writes only its named sibling certificate: respectively
`formulas.json`, `clipped.json`, `feasibility-events.json`, `sturm-tests.json`,
`open-proof.json`, `closed-proof.json`, `equality.json`, `physical-control.json`.
The optional factor helper uses `uv run factor_workload.py` or
`uv run factor_workload.py --clipped`, writing only its workload cache.

Observed producer times were 3.953 s derivation, 34.427 s clipping,
34.111 s event-incidence extraction, 14.527 s open proof, 26.356 s closed proof,
6.564 s equality census and 0.292 s physical control. Timing fields and upstream
snapshot hashes are observational, so whole-file hashes identify retained
snapshots rather than promised bit-identical reruns. A deterministic math
projection removes recursively `elapsed_seconds`, observational absolute path
fields and snapshot-hash fields, then uses sorted-key JSON with the existing
rational-string encoding. Exact references and certificate structure remain.
The lead owns final sizes, hashes and retention-policy selection.

No paid API, numerical sorting, generic CAD, old producer rerun, broad population
producer, retained-input overwrite or manuscript promotion was performed.
