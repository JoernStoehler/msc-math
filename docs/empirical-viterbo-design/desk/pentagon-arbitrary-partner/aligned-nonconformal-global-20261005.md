# Aligned nonconformal partners: sharp global bound and unique optimizer

Research source owner: fixed regular pentagon / arbitrary-partner desk.
Source integration: 4 October 2026 UTC, following final integrating mathematical
acceptance at 23:45 UTC. Status: **independently checked research**; no selected
manuscript theorem, human acceptance or external novelty determination.

## The complete family and sharp equality class

Fix the regular circumradius-one pentagon Q with outward unit normals
n_i=(cos(2πi/5),sin(2πi/5)). For every x>0 let G_x=diag(x,1). Consider every
bounded planar body with interior

    K_x(h)={q:(G_x n_i)·q<=h_i, i=0,...,4}.

Support offsets are arbitrary. Redundant inequalities and actual three- and
four-edge boundary bodies are included. Across this whole aligned family,

    sys(K_x(h)×_L Q)<=3(1+sqrt(5))/10≈0.97082039325.

Equality occurs exactly at x=sqrt(5)/3 and, modulo translation and positive
dilation, the capacity-one support class

    h=((sqrt(5)−1)/6,(sqrt(5)+1)/6,1/3,0,0),

in gauge h_3=h_4=0. Its five edges are genuine. The equality body is not
affinely regular: all ten cyclic or reversed affine vertex correspondences
with a regular pentagon fail exactly.

This is a global optimum within the aligned distorted-five-normal family,
strictly below one and the HKO benchmark (3+sqrt(5))/5. It does not assert an
arbitrary-partner local maximum, an HKO equality body, or an unrestricted
four-dimensional bound. Relative orientations outside this aligned class are
not covered. The separately started HKO-angle slice had no accepted result at
this source integration and is not incorporated here.

## Why the finite certificate covers every body

The complete ten normal-triangle fitting problems share the five extreme rays
of the normal-closure cone. Invertibility of G_x preserves that entire cone.
The exact capacity is the minimum of five circuit support sums divided by their
positive maximum-template denominators. Complete exact envelopes give six
parameter cells covering all x>0, including the unbounded final cell.

Canonical supports lie in the closed five-edge type cone, including zero
edges. After quotienting translations, its capacity-one feasible polyhedron
has five edge inequalities and five fitting inequalities. Area sublevels are
compact; a minimum exists. Brunn–Minkowski and its equality condition show
that every minimizing support class is a vertex. This implication is essential
for both the universal bound and uniqueness; enumeration alone would not prove it.

The exact certificate covers 600 nonsingular cell systems, 88 affine candidate
records, 19 singleton intervals and eight unbounded intervals. All 170 required
quadratic comparisons pass. The eight zero-containing records give the same
x and support class above. Capacity-one primal fits for all ten templates,
an active dual, positive edges and direct shoelace area independently check
the optimizer. Ratios tend to zero as x tends to zero or infinity.

## Evidence owners and imported inputs

- [Final integrating acceptance](../../night-20261005/comparison/nonconformal-final-acceptance.md)
  closes the denominator, global-vertex and equality interfaces.
- [Scientific theorem and full derivation](../../night-20261005/nonconformal-fan/report.md)
  owns formulas, exact conventions, parameter cells and scientific scope.
- [Denominator proof](../../night-20261005/nonconformal-fan/denominators.md) and
  [certificate](../../night-20261005/nonconformal-fan/denominators.json) cover the
  full ten-template input over whole cells.
- [Independent premise and arithmetic review](../../night-20261005/nonconformal-fan/vertex-premise-review.md)
  proves the closed-cone, coercivity, vertex and equality implications and audits
  the candidate arithmetic with an independent rational-pair/Cramer implementation.
- [Candidate certificate](../../night-20261005/nonconformal-fan/body-candidates.json)
  and [optimizer certificate](../../night-20261005/nonconformal-fan/optimizer-certificate.json)
  remain owned by their producers.

Imported capacity input: [Balitskiy–Mitrofanov–Polyanskii, Definition 3.2,
Theorem 3.3 and §6.1](https://arxiv.org/html/2603.12495v1), together with the
[checked regular-five circuit argument](equiangular-partner-global-20261005.md).
The finite-vertex method has an older project owner; its use is not claimed as
new. The new checked result is the complete aligned parameter theorem and
non-affinely-regular optimizer. External novelty remains unassessed.

## Reproduction and thesis boundary

Run sequentially from the repository root:

```sh
python docs/empirical-viterbo-design/night-20261005/nonconformal-fan/denominators.py
uv run docs/empirical-viterbo-design/night-20261005/nonconformal-fan/body_candidates.py
uv run docs/empirical-viterbo-design/night-20261005/nonconformal-fan/verify_optimizer.py
python docs/empirical-viterbo-design/night-20261005/nonconformal-fan/review_candidates.py
```

The first three commands overwrite only their respective owned JSON evidence:
denominators.json, body-candidates.json and optimizer-certificate.json.
The final independent auditor writes no evidence file. Arithmetic and ordering
are exact in Q(sqrt(5)); no floating grid supplies universal coverage. The
scientific owner reports about one second per retained computation locally;
this is not a clean-environment reproduction or a cost promise for extensions.
This source integration reran no producers and replaced no evidence.

**Selected thesis location: absent. PDF check: absent.** The retained 101-page
candidate was not edited. Chapters 9–10 contain related regular/affine results,
not this arbitrary-support theorem. The prior 23:08 UTC source/PDF hash check
was not repeated. Manuscript inclusion, human proof/prose acceptance and affected
PDF-page review remain separate. The selected symmetric-partner inclusion gap,
full reproduction, data-science scope and flow-code correspondence remain open.
