# HKO-angle nonconformal partners: sharp global slice theorem

Research source owner: fixed regular pentagon / arbitrary-partner desk.
Source integration: 4 October 2026 UTC, after final integrating mathematical
acceptance at 23:56 UTC. Status: **independently checked research**; no selected
manuscript promotion, human proof/prose acceptance or external novelty judgment.

## Exact class and equality

Let Q be the regular circumradius-one pentagon with outward unit normals
n_i=(cos(2πi/5),sin(2πi/5)). For every x>0 set G_x=diag(x,1), and consider
all bounded planar bodies with interior

    K_x(h)={q:(G_x R_(pi/10) n_i)·q<=h_i, i=0,...,4}.

Offsets are arbitrary. Raw redundant inequalities are lowered to canonical
supports without changing the body. Canonical three- and four-edge bodies
are included. Across this entire HKO-angle distorted-five-normal slice,

    sys(K_x(h)×_L Q)<=(3+sqrt(5))/5.

Equality holds precisely at x=1 for translations and positive dilations of
the regular HKO partner. The class contains non-affinely-regular partners,
but none supplies another equality body or an above-HKO ratio.

The theorem fixes the relative angle π/10 before the distortion. It does not
cover all relative angles of distorted fans, all arbitrary pentagon pairs,
unrestricted partners or general convex four-bodies. The separately checked
[aligned slice](aligned-nonconformal-global-20261005.md) has a different,
non-affinely-regular optimizer and a smaller sharp value. Neither theorem
implies an orientation-wide result.

## Complete capacity, area and equality implications

An invertible normal transformation preserves the complete five-ray
normal-closure cone. The imported complete ten normal-triangle fitting theorem
then gives five capacity inequalities with positive maximum-template
denominators. Whole-cell exact support comparisons cover all 1,350 raw affine
choices, certify attained envelope lines and endpoint ties, and check all 50
individual denominators positive. The five common parameter cells have boundaries

    0, sqrt(5)−2, 1/sqrt(5), 1, sqrt(5), infinity.

The independently checked canonical closed type cone, translation quotient,
coercivity and Brunn–Minkowski equality argument reduce every area minimizer
to a vertex. Zero edges remain legitimate throughout this reduction.
The independent audit reconstructs all 500 nonsingular cell systems, 65 affine
candidate records, ten singleton intervals and eleven unbounded intervals.
All 123 endpoint, stationary and tail predicates pass. No target polynomial
vanishes identically; every zero occurs at x=1 and all eight zero-containing
records give the same canonical support class modulo translation.

The arithmetic uses covectors scaled by 1/cos(π/10), which is harmless for
arbitrary offsets. In this convention the capacity-one gauge h_3=h_4=0 is

    h=(-1+3sqrt(5)/5,1−sqrt(5)/5,-1+3sqrt(5)/5,0,0).

With d=sec²(π/10)=2−2/sqrt(5), canonical transformed area numerator P gives
physical area(K)=P/[x d sin(2π/5)] and sys=x d/(5P) at capacity one.
The independent review checks this Jacobian and direct shoelace area. All ten
primal template fits plus an active complete-denominator dual prove capacity
exactly one. Equal positive edge numerators make the canonical body regular;
its inverse physical transform gives the regular positive-angle HKO partner.
These checks close both attainment and complete equality classification.

## Evidence owners and imported inputs

- [Final integrating acceptance](../../empirical-studies/discovery-design/night-20261005/comparison/hko-angle-final-acceptance.md)
  is the authoritative acceptance of complete denominators, global reduction,
  arithmetic, physical normalization and equality.
- [Final scientific report](../../empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/report.md)
  was confirmed ready by the integrating comparator at 23:57 UTC and owns
  the complete proof/certificate synthesis and measured reproduction costs.
- [Denominator derivation](../../empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/denominators.md)
  and [exact envelope certificate](../../empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/denominators.json)
  own whole-cell and endpoint-tie coverage.
- [Independent mathematical and arithmetic review](../../empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/review.md)
  routes coordinate, vertex, equality, shoelace and all-system checks.
- [Candidate certificate](../../empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/body-candidates.json)
  and [optimizer certificate](../../empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/optimizer-certificate.json)
  retain exact candidate intervals, comparisons and primal/dual placements.

Imported capacity input: [Balitskiy–Mitrofanov–Polyanskii, Definition 3.2,
Theorem 3.3 and §6.1](https://arxiv.org/html/2603.12495v1).
The [regular-five capacity core](equiangular-partner-global-20261005.md) supplies
closure-ray completeness, and the [aligned vertex review](../../empirical-studies/discovery-design/night-20261005/nonconformal-fan/vertex-premise-review.md)
supplies the canonical-cone/global-minimizer argument. Their transfer is reviewed
for the new transform; no claim of inventing the finite-vertex method is made.
External novelty remains unassessed. The previous regular/equiangular/aligned
results and their producer evidence were preserved.

## Reproduction and thesis boundary

Run sequentially from the repository root:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/denominators.py
uv run topics/empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/body_candidates.py
uv run topics/empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/verify_optimizer.py
python -B topics/empirical-studies/discovery-design/night-20261005/hko-angle-nonconformal/review_candidates.py
```

The first three commands overwrite only their respective new sibling evidence:
denominators.json, body-candidates.json and optimizer-certificate.json.
The final independent auditor is read-only. Denominators and the auditor use
standard-library exact rational-pair arithmetic; the middle scripts declare
inline SymPy dependencies. No floating-angle samples supply the universal
parameter claim. Final acceptance records an unchanged-copy denominator replay
whose scratch JSON matched the retained output exactly. This source integration
reran no producers and replaced no certificates. These checks do not establish
clean-environment reproduction or proof-assistant verification.

**Selected thesis location: absent. PDF check: absent.** Chapters 9–10 contain
related regular/affine results, not this arbitrary-support distorted-fan theorem.
The selected 101-page manuscript/PDF was unchanged; its 23:08 UTC hash check
was not repeated. Inclusion, human acceptance and affected-page PDF review remain
separate. Selected symmetric-partner inclusion, human PASS, data-science scope,
flow-code correspondence and full reproduction remain unresolved. The global
source-hash baseline and broader research-planning reconciliation remain pending.
