# Complete denominators for the HKO-angle nonconformal slice

Exact bounded computation completed 2026-10-04 23:50 UTC under the
[selected contract](../comparison/hko-angle-contract.md). This packet supplies
the five complete capacity constraints over **all `x>0`**. It does not assert
a sharp ratio bound or classify partner bodies.

## Coordinates and audited normal convention

Write `z=√5`, `τ²=5−2z`, `u=z/5=1/√5`, `θ=π/10`.
Computational points are `(q_x,q_y/τ)` and covectors are
`N=(n_x,τ n_y)`, so their ordinary dot product is the physical pairing.
Let `S=diag(1,τ)`. The covector-coordinate conjugate of physical rotation is

`S R_θ S⁻¹ / cosθ = [[1,−tanθ/τ],[τ tanθ,1]]`
`= C = [[1,−u],[τ²u,1]]`.

This directly distinguishes covectors from the point-coordinate conjugate.
For scaled normal `G_x C N_i`, the pairing with a computational vertex `v` is

`x(N_ix−u N_iy)v_x + (τ²u N_ix+N_iy)v_y`.

The common positive scale `cosθ` is absorbed into every arbitrary support;
all displayed denominators use these **scaled normals**. The original
unit-normal convention would multiply every displayed denominator by
`cosθ`. The scaled normal fan defines exactly the same family of bodies after
the corresponding support rescaling.

The exact determinant is

`d=det C=1+τ²u²=2−2z/5>0`.

It is retained in the JSON. Dropping the common normal scale does **not** permit
dropping `d` from the lead's area transformation. The lead owns the physical
shoelace/area verification and final ratio normalization.

The regular source covectors, original ten template triangles and five circuit
labels reproduce the conventions in the
[accepted regular core](../geometry-discovery/fixed-q-rank/equiangular-capacity-core.md).
The templates are not rotated or replaced for this calculation.

## Full exact envelope

Define the following affine functions:

`A(x)=((z−2)x+1)/5`,
`B(x)=(z−1)(x+1)/10`,
`C₁(x)=((3−z)x+7−3z)/5`,
`E(x)=((7−3z)x+z−1)/10`,
`F(x)=(5−2z)(x+1)/5`,
`H(x)=((3z−5)x+3−z)/10`,
`I(x)=(z−1)x/5`,
`J(x)=(z−1)/5`,
`L(x)=2(z−2)(x+1)/5`.

Then `D_l(x)=max_j Σ_i λ_li h_Tj(G_x C N_i)` is:

| Circuits | Parameter interval | Exact denominator |
| --- | --- | --- |
| `0,1` | `0<x≤1` | `A(x)` |
| `0,1` | `1≤x≤z` | `B(x)` |
| `0,1` | `x≥z` | `C₁(x)` |
| `2,4` | `0<x≤z−2` | `E(x)` |
| `2,4` | `z−2≤x≤z/5` | `F(x)` |
| `2,4` | `z/5≤x≤1` | `H(x)` |
| `2,4` | `x≥1` | `I(x)` |
| `3` | `0<x≤1` | `J(x)` |
| `3` | `1≤x≤z` | `A(x)` |
| `3` | `x≥z` | `L(x)` |

Every adjacent formula agrees exactly at its shared endpoint. There are
**17 circuit cells**, with per-circuit counts **`[3,3,4,3,4]`**. Their common
refinement has **five cells**, separated by

`0 < z−2 < z/5 < 1 < z < ∞`.

The continuous `x→0+` limits are

`D_0=D_1→1/5`, `D_2=D_4→(z−1)/10`, `D_3→(z−1)/5`.

These positive boundary values concern the denominator extension only; `x=0`
is excluded from the invertible full-dimensional fan problem.
The respective `x→∞` slopes are

`[(3−z)/5,(3−z)/5,(z−1)/5,2(z−2)/5,(z−1)/5]`.

At **`x=1`**, all five complete scaled denominators are exactly
**`(z−1)/5`**. This was checked from the full 270 candidates per circuit
and directly from geometric support maxima, not assumed from symmetry.
It agrees with the regular-angle capacity control after restoring `cosθ`.

## Completeness, counts and exact certificates

The regular closure rays are five cyclic shifts of

`λ^(0)=(1,1,0,2b,0)/(2(1+b))`, `b=(1+z)/4`.

For `x>0`, `G_x C` is invertible. Therefore its normal-closure cone is exactly
the original regular cone: `Σλ_i G_x C N_i=0` if and only if `Σλ_i N_i=0`.
The same five normalized rays are complete. Each has three positive entries.

For a fixed circuit and triangle, its weighted sum of three vertex maxima
is the maximum of **27** affine candidates, because nonnegative weights
allow the three maximizing vertex choices to be selected independently.
With ten triangles, this derives **270 candidates per circuit**, hence
**1,350 raw candidates total**, before enumeration. The actual distinct affine
candidate counts are **`[67,67,71,47,71]`**, totaling **323** with circuit labels.
No candidate partner body is enumerated here.

The retained script uses exact `Q(√5)` arithmetic. It sorts slopes, removes
inferior parallel lines, constructs the full convex upper hull with exact
intersection comparisons, and clips that hull to `x>0`. In addition:

- Every raw candidate is checked below the reported hull over **every complete
  cell**, using the two finite endpoints or the left endpoint and asymptotic
  slope on an unbounded cell. Every reported hull line is an actual candidate.
- All finite adjacent endpoint equalities are checked exactly.
- Direct geometric support maxima are evaluated at an exact interior point
  of each complete cell, independently of the affine-choice expansion.
- Direct geometric maxima and **all** maximizing raw support choices are
  retained for every circuit at **every common-refinement endpoint**, including
  its `x=0` extension and endpoints where that circuit's formula does not change.
- Separate exact envelopes for all **50** triangle–circuit denominators are
  retained and certified strictly positive on `x≥0`: finite cell endpoints
  are positive and the final affine slope is positive.

This supplies whole-cell equality certificates, not a numerical sampling
argument. At an endpoint the JSON records every attaining template and every
attaining vertex-choice tuple, including choices that attain only that point.

Geometric positivity on the intended `x>0` domain also follows directly:
translate a nondegenerate triangle so it contains zero in its interior.
Every transformed normal is nonzero, so each support is positive; the positive
circuit weights give a strictly positive sum. The sum is translation invariant
because the transformed normals close. The same interior translation proves
`λ_l·h>0` for every bounded full-dimensional partner.

Using the complete ten-template fitting theorem from the regular core, each
template dual section has precisely the five ray vertices `λ_l/d_jl(x)`.
Taking the minimum over templates and rays therefore gives

`c_EHZ(K_x(h)×_L Q)=min_l (λ_l·h)/D_l(x)`.

The maximum denominator is essential because the fixed numerator is positive.
Thus **`λ_l·h≥D_l(x)` for all five rays** is exactly the complete
capacity-at-least-one condition in the scaled support convention.

## Reproduction and write effects

```bash
python -B /workspaces/msc-math/docs/empirical-viterbo-design/night-20261005/hko-angle-nonconformal/denominators.py
```

[denominators.py](denominators.py) requires Python standard library only.
The exact field encoding in [denominators.json](denominators.json) is
`["a","b"] = a+b√5`, with rational strings. `right: null` means infinity.
The JSON retains source and scaled covectors, determinant, original template
vertices, circuits, counts, all individual and complete envelopes, and endpoint
ties. Source tuples have order `[template,vertex_for_first_active_normal,
vertex_for_second_active_normal,vertex_for_third_active_normal]`, with active
normal indices in increasing order.

Initial full computation took **1.03 seconds** locally. The strengthened final
run, including all common endpoint ties and direct geometric endpoint checks,
took **1.81 seconds**. The sole script write is its sibling `denominators.json`,
replaced deterministically. Existing regular or aligned scripts and producers
were not rerun or modified. No paid API or bivariate cell enumeration was used.

The lead can now use these five parameter cells for closed-support area
minimization, physical determinant checks, edge-loss handling and equality
classification. Those are separate proof obligations from this denominator
packet.
