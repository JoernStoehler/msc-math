# Equiangular partners: exact capacity core and sharp global bound

Research source owner: fixed regular pentagon / arbitrary-partner desk.
Derived and independently checked on 4 October 2026 UTC during the selected
5 October research night. Root selected this source integration at23:31 UTC.
Status: **independently checked research**, not a selected manuscript theorem,
human proof/prose acceptance, or an external novelty determination.

## Exact class, capacity and equality

Let Q be the regular circumradius-one pentagon with outward unit normals
n_i=(cos(2πi/5),sin(2πi/5)), i=0,…,4. Set Pθ=RθQ. Consider every bounded
planar convex body with interior of the form

    K={x:(Rθ n_i)·x<=h_i, i=0,…,4},

with arbitrary support offsets h_i. Genuine equiangular pentagons and bodies
obtained by losing edges are included; inequalities may be redundant.
No additional normal directions are allowed.

Let ιθ(K) be the largest t for which a translated copy of tPθ is contained
in K. The new exact relation is

    c_EHZ(K×_L Q)=c_EHZ(Pθ×_L Q) ιθ(K).

For d=dist(θ,(π/5)Z)∈[0,π/10], this yields the sharp global class bound

    sys(K×_L Q)<=ρ0 sec²d,   ρ0=(5+2sqrt5)/10.

Equality for fixed θ holds exactly for translates and positive dilates of
Pθ. Across all θ, the maximum is (3+sqrt5)/5; attaining it additionally
requires d=π/10, equivalently θ=π/10 moduloπ/5. These are HKO angles in
the convention above.

This is global within the prescribed regular-five normal fan, for all support
offsets and collapsed-edge boundaries. It does not cover arbitrary normal
fans, all arbitrary pentagon pairs, unrestricted convex partners, or ambient
four-dimensional bodies. The exact local support criterion and the improving
decagon control are separate research results, not assumptions of this theorem.

## Mechanism and evidence

The normalized nonnegative normal-closure cone has exactly five extreme rays:
cyclic shifts of

    λ=(1,1,0,2cos(π/5),0)/[2(1+cos(π/5))].

They form one rotational orbit. The complete ten normal-triangle fitting LPs
read those same rays. After taking the minimum over all ten fitting scales,
rotation symmetry forces one common positive normalization on all five rays.
The largest translated inner Pθ is a translation-and-scale LP on exactly
the same rays. This proves the capacity identity. Containment then gives
area(K)≥ιθ(K)²area(Pθ), with equality only when the contained regular
pentagon is the whole body.

The consequential sources are:

- [Standalone exact theorem and proof](../../night-20261005/geometry-discovery/fixed-q-rank/equiangular-capacity-core.md):
  complete circuit enumeration, all-ten min/max direction, positivity,
  Farkas scale, attained containment and equality.
- [Independent consequential review](../../night-20261005/geometry-discovery/fixed-q-rank/equiangular-global-review.md):
  all-angle and boundary interfaces passed; all50 aligned support denominators
  were compared exactly in Q(sqrt5), with common maximum (sqrt5−1)/5.
- [Independent integrating implication check](../../night-20261005/comparison/equiangular-core-interface-check.md):
  reconstructs the closure-to-capacity-to-contained-core-to-global-bound arrows.
- [Retained exact coefficient verifier](../../night-20261005/geometry-discovery/fixed-q-rank/verify_equiangular_denominators.py):
  rerun successfully by the integration lead; writes no files.
- [Research integration report](../../night-20261005/geometry-discovery/fixed-q-rank/report.md):
  separates this global theorem from the local active-hull criterion,
  the all-ten decagon ascent and earlier representation controls.

The regular-product coefficient is an imported result:

    c(Pθ×_L Q)=(1+cos(π/5))² sec d.

Its maintained owner is
[the independent affine-pentagon proof](../../../../formal/pentagon-affine-products/research.tex);
the selected manuscript has the related regular-angle result in
[Chapter9](../../../../thesis/chapters/09-rotated-regular-polygons.tex).
It is not claimed as a new derivation here.
The finite normal-triangle capacity input is
[Balitskiy–Mitrofanov–Polyanskii, Definition3.2, Theorem3.3 and §6.1](https://arxiv.org/html/2603.12495v1).

## Existing solved slice and scientific increment

The prescribed ten-direction sharp theorem was already retained inside

    docs/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip
    → incoming/hko_fixed_pentagon_results.zip
    → mathematics/TEN_DIRECTION_PROOF.md.

Its prescribed fan includes θ=π/10 moduloπ/5. The
[baseline check](../../night-20261005/comparison/fan-baseline-check.md) identifies
this overlap; its797-vertex certificate was not rerun.
The new increment is the all-angle contained-regular-pentagon capacity identity
and sharp equality consequence for arbitrary supports in a regular-five fan.
External novelty remains unassessed.

## Reproduction and thesis status

From the repository root, the bounded exact coefficient check is

    uv run docs/empirical-viterbo-design/night-20261005/geometry-discovery/fixed-q-rank/verify_equiangular_denominators.py

It checks all50 positive denominators and the aligned coefficient using exact
Q(sqrt5) comparisons. It is corroboration of the stated arithmetic; the
standalone proof supplies the universal angle, boundary and equality arguments.
No production capacity implementation, large dataset producer, full fan
enumeration, clean-environment reproduction or proof-assistant verification was
performed for this result.

**Selected thesis location: absent. PDF check: absent.**
The selected manuscript and retained101-page candidate were not changed.
Chapters9–10 contain related regular/affine results, not this arbitrary-support
capacity-core theorem. Its future inclusion and any resulting PDF review remain
separate work; no human PASS or manuscript promotion follows from these checks.
The previously selected symmetric-partner inclusion remains a distinct gap.

Nonconformal fans, capacity-cut criteria and unrestricted systolic inequalities
remain separate research questions. This record does not promote unfinished
work in those directions.
