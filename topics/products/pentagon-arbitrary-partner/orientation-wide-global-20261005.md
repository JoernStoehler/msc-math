# Orientation-wide fixed-axis partner family: sharp HKO bound and equality

Research source owner: fixed regular pentagon / arbitrary-partner desk.
Initial bound source integration: 5 October 2026, 01:01 UTC, following
acceptance at 00:59 UTC. Updated at 01:06 UTC after complete scientific acceptance
at 01:03 UTC. Status: **independently checked sharp theorem and equality**
relative to named mathematical inputs. No selected manuscript promotion, human
acceptance or external novelty determination.

## Accepted theorem and exact family

Fix the regular circumradius-one pentagon Q with unit outward normals
n_i=(cos(2πi/5),sin(2πi/5)). For every x>0 and every real theta consider
all bounded bodies with nonempty interior of the form

    K={q:(diag(x,1) R_theta n_i)·q<=h_i, i=0,...,4}.

The offsets h_i are arbitrary. Redundant raw inequalities are canonicalized
without changing the body; genuine three- and four-edge boundary bodies are
included. In the project's systolic-ratio convention,

    sys(K×_L Q)=c_EHZ(K×_L Q)²/[2 area(K) area(Q)]
                <=(3+sqrt(5))/5.

Equality holds exactly when x=1, theta≡π/10 modulo π/5, and K is a
translation and positive dilation of the regular HKO partner. This proves
sharpness and excludes nonregular or lower-edge equality bodies in the class.
This theorem covers all orientations **within the explicit fixed-axis family**
diag(x,1)R_theta: the distortion axes remain fixed relative to Q.
It does not establish a bound for arbitrary GL(2) normal transformations,
all pentagon pairs, unrestricted partners or general convex four-bodies.
No new ambient local theorem is asserted.

The accepted period/reflection argument reduces theta to [0,π/10] without
interchanging distortion axes or assuming x and 1/x equivalent. With
u=tan(theta)/tau and tau²=5−2sqrt(5), this is 0≤u≤1/sqrt(5), x>0.
The [aligned](aligned-nonconformal-global-20261005.md),
[HKO-angle](hko-angle-nonconformal-global-20261005.md) and
[equiangular](equiangular-partner-global-20261005.md) theorems are earlier
accepted slices/controls. The new accepted bound covers their relative-angle
continuation in the stated family, including non-affinely-regular partners.

## Why the exact finite proof covers arbitrary offsets

Invertibility preserves the complete five-ray normal-closure cone. The imported
complete ten normal-triangle fitting theorem supplies all five capacity
inequalities. Whole-cell joint denominator coverage includes every open cell,
closed fiber and boundary tie. It is not inferred from a sampled angle grid.

At each fixed pair of parameters, canonical support offsets form a closed
Minkowski type cone. The capacity-at-least-one polyhedron has compact area
sublevels modulo translation. Its area minimum exists and has capacity exactly
one. Brunn–Minkowski and its equality condition prove that every minimizing
support class is a vertex. The primary triangle/quadrilateral theorem bounds
all zero-edge bodies by one, strictly below HKO. A benchmark-relevant five-edge
vertex therefore has three independent active fitting rows and belongs to the
complete ten-basis list.

The independently checked gap covers 1,416 feasible open-orientation components,
390 singleton-x components and the full internal event catalogue. Closed coverage
checks all 2,580 original basis incidences and 78 added internal-cut incidences:
1,700 feasible and 958 empty fibers, including 190 infinite tails.
Exact endpoint, stationary-exclusion and tail predicates give nonnegative gap
on every feasible formula. The complete minimizer implication transfers this
to every support vector, not just the enumerated bodies.

The physical Jacobian is explicit. For the computational common normal scale
1/cos(theta), put d(u)=1+(5−2sqrt(5))u² and
P=Σh_i h_(i+1)−((sqrt(5)−1)/4)Σh_i². At capacity one,

    area(K)=P/[x d(u) sin(2π/5)],
    sys=x d(u)/(5P),
    sharp gap=P−x d(u)/(3+sqrt(5)).

This coordinate formula and normalization are reviewed mathematical inputs.
Exact arithmetic does not supply a missing universal minimizer implication.

## Complete equality and evidence owners

- [Final integrating acceptance](../../empirical-studies/discovery-design/night-20261005/comparison/orientation-class-final-acceptance.md)
  owns the accepted theorem and complete equality. The [earlier bound checkpoint](../../empirical-studies/discovery-design/night-20261005/comparison/orientation-class-bound-acceptance.md)
  retains the exact open/closed coverage and its then-pending equality boundary.
- [Class-proof derivation](../../empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/report.md)
  owns the minimizer bridge, formula construction and physical normalization;
  its final equality and physical synthesis has passed scientific review.
- [Independent class-proof review](../../empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review.md)
  accepts the minimizer implication, all formula mappings, open clipping,
  internal events, closed-fiber signs, full equality census and physical controls.
- [Accepted complete joint denominator stage](../../empirical-studies/discovery-design/night-20261005/comparison/orientation-joint-final-acceptance.md)
  routes [its report](../../empirical-studies/discovery-design/night-20261005/orientation-wide/joint-denominators/report.md),
  [independent review](../../empirical-studies/discovery-design/night-20261005/orientation-wide/joint-denominators/review.md)
  and [retained certificate](../../empirical-studies/discovery-design/night-20261005/orientation-wide/joint-denominators/joint-denominators.json).
- [Period/reflection and canonical-minimizer review](../../empirical-studies/discovery-design/night-20261005/orientation-wide/preflight/pruning-review.md)
  routes the exact parameter reduction and inherited global-vertex argument.

The independent equality audit checks all 974 endpoint polynomials with full
odd and even root counts, excluding even-root, flat-gap and singleton equality
in every open component. No endpoint polynomial or closed x-gap is identically
zero. All 25 closed zero occurrences deduplicate to one normalized body at
(x,u)=(1,1/sqrt(5)). After the bound, every equality body is an area minimizer;
the reviewed every-minimizer-is-a-vertex argument excludes additional nonvertex
equality bodies.

[The fresh physical certificate](../../empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/physical-control.json)
and independent physical review reconstruct five positive edges, all ten primal
fits and twenty active circuit/template dual pairs. They establish capacity
exactly one; all fifty direct denominator sums bind the accepted joint catalogue.
Direct physical K/Q shoelaces give the stated ratio, while equal physical side
lengths/circumradii and translated canonical constant supports establish regularity.
Restoring period/reflection, translations and positive dilations gives the complete
equality class above. Raw redundant supports add representations of that body,
not additional equality geometries.

Imported inputs include [Balitskiy–Mitrofanov–Polyanskii's complete fixed-pentagon
normal-triangle theorem](https://arxiv.org/html/2603.12495v1), the accepted
canonical vertex-minimum argument and the primary triangle/quadrilateral bound.
The fixed-axis class is part of the theorem, not a coordinate shortcut to all
GL(2) transforms. External novelty remains unassessed.

## Reproduction and thesis boundary

The currently accepted independent audits are read-only. Run from repository root:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_formulas.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_clip.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_signs.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_open.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_closed.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_equality.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/review_physical.py
```

These replay retained inputs and write no evidence files. The [production
instructions](../../empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/extraction.md)
and [retention inventory](../../empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/retention.md)
distinguish essential proof certificates from optional factor-workload caches.
Producer regeneration is a separate operation. Its essential sequence, from
repository root, is:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/derive.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/clip.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/clip.py --events-only
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/sturm.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/prove_open.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/prove_closed.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/equality.py
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/physical_control.py
```

These standard-library commands overwrite, in order, only their sibling
formulas.json, clipped.json, feasibility-events.json, sturm-tests.json,
open-proof.json, closed-proof.json, equality.json and physical-control.json.
They read inherited peer inputs and do not rerun earlier producer stages.
The [read-only inventory script](../../empirical-studies/discovery-design/night-20261005/orientation-wide/class-proof/retention.py)
prints recomputed snapshot/math-section hashes to stdout. Timing and observational
provenance fields prevent a promise of byte-identical regenerated JSON; retained
snapshot hashes identify the reviewed evidence, while independent audits verify
mathematical content. Optional factor caches are not final-proof inputs.

This source integration ran no producers, replaced no evidence and did not
repeat the independent arithmetic replays. No clean-environment or
proof-assistant verification follows from this record.

**Selected thesis location: absent. PDF check: absent.** Chapters 9–10 contain
related regular/affine results, not this arbitrary-support orientation-wide
fixed-axis theorem. The retained 101-page manuscript/PDF was unchanged; its
23:08 UTC hash check on 4 October was not repeated. Inclusion, human acceptance
and affected-page PDF review remain separate. Selected symmetric-partner
inclusion, human PASS, data-science scope, flow-code correspondence and full
reproduction remain open. The global source-hash baseline and broader
research-planning reconciliation remain pending.
