# Future Ideas: Pruning Experiment Extensions

Ideas identified during the ablation study, not in scope for the current branch.

## A1 Effectiveness vs F

How does vertex-adjacency pruning (A1) scale with facet count? Currently A1=A0
at F=5,6 for many polytopes (all facets are vertex-adjacent). At what F does A1
start providing meaningful pruning? The current data hints at F≥7 but the sample
size is too small to characterize the scaling exponent.

## Non-Simple Polytope Dataset

The cut simplex (F=6) is the only non-simple polytope in the dataset. A dedicated
dataset of non-simple polytopes would characterize the A2≠A3 gap more thoroughly.
Possible generators:
- Cut simplices with varying cut depths
- Bipyramids over 3-polytopes (apex on 5+ facets)
- Products of simplices truncated at vertices

## Scaling Exponent Analysis

The A3/A0 ratio appears to scale exponentially with F. With more data points
(F=9,10,...), the exponent could be estimated. This would quantify the practical
benefit of pruning for larger polytopes.

## Unknown Predicates Empirical Check

The three-valued predicate logic (TRUE/FALSE/UNKNOWN) from the thesis appendix
isn't used in the ablation (which uses exact TRUE/FALSE with tolerance). An
experiment that intentionally introduces near-degenerate polytopes could test
how often UNKNOWN verdicts arise and how they affect capacity bounds.

## Face Lattice / Skeleton Data Structure

(From Jörn, 2026-02-22)

Represent each k-face by its maximal facet index set. The A3 feasibility check
then becomes purely combinatorial once the face lattice is known: find a face
containing both facet indices, then check ω₀ signs for all facets in that face.
This avoids the LP entirely. Would require implementing the face lattice
computation (vertex-facet incidence → all k-faces) and testing it against the
current LP-based A3.

## Exact Skeleton Predicates via Perturbation

(From Jörn, 2026-02-22)

Replace three-valued predicates (TRUE/FALSE/UNKNOWN) in skeleton computation
with deterministic rounding: if ω₀(n_i,n_j) ≈ 0, round to TRUE or FALSE and
argue via small perturbation that the capacity changes by at most the
perturbation. This separates exact combinatorial decisions (skeleton) from
approximate numerical decisions (KKT solver). Requires careful analysis of
when the perturbation direction matters.
