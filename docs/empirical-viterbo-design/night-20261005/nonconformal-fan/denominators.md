# Complete nonconformal-fan denominators

Exact bounded computation completed 2026-10-04 23:35 UTC for the
[selected contract](../comparison/nonconformal-fan-contract.md).
Owned result: all five complete denominators on **every** `x=1+s>0`.
This supplies the capacity constraints; it does not classify minimizing bodies.

Put `z=√5`. The circuit labels, ten template labels, computational coordinates
and normalization are exactly those in
[the regular core](../geometry-discovery/fixed-q-rank/equiangular-capacity-core.md)
and its [coefficient verifier](../geometry-discovery/fixed-q-rank/verify_equiangular_denominators.py).
In particular, computational points are `(physical x,physical y/τ)`,
`τ²=5−2√5`; computational covectors are `(physical n_x,τ physical n_y)`.
Their ordinary computational dot product is the physical pairing. Thus
`(G_x n_i)·v = x n_ix v_x + n_iy v_y` in these coordinates.
No Euclidean-unit renormalization of `G_x n_i` is performed.

## Exact formulas

Let

`A(x)=((13−5z)x+5−z)/20`,
`B(x)=((1+z)x−5+3z)/20`,
`C(x)=((z−2)x+z)/10`,
`E(x)=(3z−5)(x+1)/20`,
`F(x)=((4−z)x+5−2z)/10`.

The complete maxima `D_l(x)=max_j Σ_i λ_li h_Tj(G_x n_i)` are:

| Circuits | First cell | Middle cell | Final cell |
| --- | --- | --- | --- |
| `l=0,4` | `A(x)`, `0<x≤z/3` | `B(x)`, `z/3≤x≤z` | `x/5`, `x≥z` |
| `l=1,3` | `C(x)`, `0<x≤(5+4z)/11` | `A(x)`, `(5+4z)/11≤x≤2+z` | `(z−1)x/10`, `x≥2+z` |
| `l=2` | `E(x)`, `0<x≤(4z−5)/11` | `F(x)`, `(4z−5)/11≤x≤z/3` | `(z−1)x/5`, `x≥z/3` |

Adjacent formulas agree exactly at every shared endpoint; either formula is
valid there. There are **15 circuit cells**, and **six common parameter cells**,
separated by the strictly ordered positive breakpoints

`(4z−5)/11 < z/3 < (5+4z)/11 < z < 2+z`.

For `s`, subtract one from each endpoint. The first cell approaches `s=−1`
from above and the final cell extends without an upper bound.

The exact boundary limits are

`D_0(0)=D_4(0)=(5−z)/20`,
`D_1(0)=D_3(0)=z/10`,
`D_2(0)=(3z−5)/20`.

These are positive continuous extensions of the denominator formulas; `x=0`
itself is excluded from the invertible-fan body theorem. For `x→∞`, the
respective limiting slopes are `1/5,(z−1)/10,(z−1)/5,(z−1)/10,1/5`.

At the regular control `s=0`, hence `x=1`, **all five** denominators equal
`(z−1)/5`, exactly as in the independent regular verifier.

## Finite reduction and why this is the complete envelope

The regular normalized closure rays are the five shifts of

`λ^(0)=(1,1,0,2b,0)/(2(1+b))`, `b=(1+z)/4`.

For `x>0`, invertibility gives
`Σ_i λ_i G_x n_i=0 ⇔ Σ_i λ_i n_i=0`.
Consequently the entire nonnegative closure cone, its five extreme rays, their
normalization `Σλ_i=1`, and their completeness remain unchanged. Each ray has
three strictly positive entries.

For a fixed triangle and circuit, distributivity of maxima with nonnegative
weights gives

`Σ_i λ_i max_{k=0,1,2}(a_ik x+b_ik)`
`=max_{k_1,k_2,k_3∈{0,1,2}} Σ_{r=1}^3 λ_i_r(a_i_r,k_r x+b_i_r,k_r)`.

Thus there are **27 affine candidates per triangle**, **270 per circuit**,
and **1,350 raw candidates total**, derived before enumeration. In circuit
order the numbers of distinct affine functions are **71,69,49,69,71**.
These counts include candidates from all ten templates and do not enumerate
any partner body.

The script sorts exact slopes, retains maximal intercepts at equal slope, and
constructs the convex upper hull using exact consecutive intersection
comparisons. It then clips this all-real hull to `x>0`. As an additional exact
certificate, for every reported cell it checks **every raw candidate** lies
below its reported affine function: affine differences need only the two
finite endpoints, or the left endpoint and limiting slope on an unbounded
cell. Every retained hull line is an actual candidate. This proves both
directions of equality over the full cell, rather than relying on samples.
All adjacent endpoint equalities are also checked. Direct geometric support
evaluation at an exact interior point in each of the 15 complete cells checks
the pairing and support expansion separately.

## Positivity and capacity use

All **50** triangle–circuit denominators are strictly positive on `x>0`.
Geometrically, translate any nondegenerate template so it contains the origin
in its interior. Its support is positive in every nonzero direction. Since
`G_x` is invertible and the ray entries are positive on their support, the
weighted sum is positive; translating a triangle leaves this sum unchanged
by normal closure.

The retained JSON also contains a separate exact upper envelope for each of
the 50 denominators. Every finite cell endpoint has strictly positive value,
and each final cell has positive slope. Hence an independent piecewise affine
positivity certificate holds even for the continuous extension `x≥0`.

For a bounded full-dimensional `K_x(h)`, its circuit numerators `λ_l·h` are
positive by the same translation-to-interior argument. The complete template
fitting formula therefore yields

`c_EHZ(K_x(h)×_L Q)=min_l (λ_l·h)/D_l(x)`.

Indeed each template dual is a positive section of the unchanged closure
cone, with vertices `λ_l/d_jl(x)`. Its minimum is the minimum of those five
ratios; minimizing over the ten complete templates then uses their **largest**
positive denominator for each fixed numerator. Thus the five inequalities
`λ_l·h≥D_l(x)` are the complete capacity-at-least-one constraints throughout
the selected parameter domain. This statement depends on the complete ten
template fitting theorem already accepted in the regular core; this task
checks its nonconformal denominator reduction, not that literature theorem.

## Reproduction, encoding, bounded costs and write effects

Run from any directory with Python 3 standard library only:

```bash
python /workspaces/msc-math/docs/empirical-viterbo-design/night-20261005/nonconformal-fan/denominators.py
```

[denominators.py](denominators.py) uses exact rational pairs representing
`Q(√5)`; no floating-point comparison or numerical breakpoint discovery occurs.
Ordering uses signs and rational comparisons of `a²` and `5b²` when the two
terms have opposite signs. Division uses the exact quadratic-field norm.
The deterministic output [denominators.json](denominators.json) encodes every
field element as `["a","b"]`, meaning `a+b√5`. It includes normals, triangle
vertices, circuits, complete cells, raw/unique candidate counts, template
support choices, all 50 individual envelopes and their `x=1` values.
`right: null` denotes infinity; all finite boundaries use continuous formulas.
Source choice tuples are `[template, vertex_for_first_active_normal,
vertex_for_second_active_normal, vertex_for_third_active_normal]`, with active
normal indices in increasing order. Listed choices attain a whole cell;
additional isolated endpoint ties need not be listed.

The initial hull calculation took **0.50 seconds** locally; the strengthened
run including every-candidate whole-cell certificates took **1.04 seconds**.
The only script write is its sibling `denominators.json`, replaced
deterministically. No data producer, body candidate enumeration, paid API,
manuscript edit or broad experiment was run. The original regular verifier
was separately rerun successfully in **4.60 seconds** and writes no files.

The next scientific step belongs to the lead: minimize area over the closed
support cone with these five complete constraints, handle boundary bodies and
recession/coercivity, and classify actual optimizers across these six cells.
The differing denominators alone are preparatory evidence, not a global
ratio bound or a counterexample.
