# Prospective geometry-only surrogate ceiling crux

17 October 2026; written before new star evaluations. Owner: ceiling_crux;
root owns selection, independent review, integration, commits and cleanup.

End state: either an exact rational polygon pair refuting the proposed global
surrogate ceiling, or a bounded failure to refute on the complete named
geometry-only hard-width cohort. The latter leaves a proof question, not a
conjecture endorsement. This changes the branch decision by distinguishing a
plausible sharp geometric covering route from a surrogate already too weak.

## Options and selection

1. Test the new ceiling on all retained 5x5/5x6/6x6 polygons whose existing
   geometry-only two-bounce ratio exceeds HKO: highest direct falsification
   value for the global surrogate; selected.
2. More local 5x5 cohorts: useful continuity diagnostics but rewards repeating
   the already successful HKO neighbourhood, and does not address hard shapes.
3. Numerical calibration against capacity labels: rewards fit rather than the
   geometry inequality; a capacity oracle is unnecessary for this crux.
4. Broad eta scan: measures approximation coverage but eta is irrelevant to
   whether this feasible capacity ceiling stays below the sharp global value.

Rubric: information about a high-branching geometric inequality first, exact
decisiveness second, resource cost third. Raw sample size rewards easy rejected
width cases; best numerical error rewards polishing; number of above-one
examples rewards local selection. None is the selection metric here.

## Frozen formula and population

For centroid-normalized primal factor normals, use the chord-star packet's
cone-volume closure center and every column of the exact tangent projector.
Retain both endpoints of every full tangent chord and the center. For each
role and each signed angular word (reverse the negative word), maximize its
HK payoff at every star point against all individual opposite closure vertices.
Let q_star be the largest payoff and C_star=1/(2q_star). Compute exact factor
areas and exact two-bounce width c2 using the difference-body normal identity.
Test min(c2,C_star)^2/(2 area(P) area(Q)) <= (3+sqrt(5))/5.
No eta computation; no paired full capacity enumeration; no capacity labels.

Eligibility uses only name,k,m,sys2 from the retained features.csv.gz, with
sys2 > (3+sqrt(5))/5. There are 14 eligible names, so all are frozen, in source
feature order, with no downsampling:

- random_5x5_s0_12
- random_5x5_s0_170
- random_5x5_s0_203
- random_5x5_s3_247
- random_5x6_s0_20
- random_5x6_s0_145
- random_5x6_s1_55
- random_5x6_s1_86
- random_5x6_s2_71
- random_5x6_s2_190
- random_5x6_s2_220
- random_5x6_s3_186
- random_6x6_s1_194
- random_6x6_s2_162

Source random-product.jsonl SHA256:
66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736.
Extract only exact rational normals/name/side counts from this registered local
cache; historical capacity labels are not joined. Recompute every width and
volume exactly and verify the eligibility inequality in Q(sqrt(5)).

Controls: exact HKO control from chord-star, without computing eta; exact
candidate closure/nonnegativity/normalization and direct 4D HK sums for every
retained best word. Existing transformation controls are imported evidence,
not duplicated compute. Stop at first strict surrogate counterexample and
return its exact feasible witness and volume. A counterexample refutes the
surrogate inequality only, not true global HKO maximality.

Budget: <=20 minutes wall and 300 CPU seconds locally, no cloud, network,
installation, producer reruns, thesis/PM/aggregate edits. Only this scratch
directory is written. Root arranges independent scientific verification.
