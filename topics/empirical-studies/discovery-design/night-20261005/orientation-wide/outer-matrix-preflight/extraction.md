# General-matrix quadratic-cone preflight

Exact production completed 2026-10-05 02:20 UTC under the
[selected contract](../../comparison/outer-matrix-preflight-contract.md).
Owned outputs are [generate.py](generate.py), [cone.json](cone.json) and the
read-only [replay.py](replay.py). The lead owns [report.md](report.md) and the
reviewer owns independent acceptance. This packet is a reduction and finite
inventory; **no quadratic nonnegativity, optimizer or global theorem is proved**.

## Physical and computational bases

Let A be the general physical matrix acting on regular unit-normal columns.
Put `τ²=5−2√5`, `S=diag(1,τ)`. Computational points and covectors are

`q_c=S⁻¹q_phys`, `N_i=S n_i`.

The computational normal matrix is therefore

`B=S A S⁻¹`, `det B=det A`.

This is a covector transform, not the point-coordinate conjugate. Their dot
product gives exactly the physical pairing. In these coordinates

`(B N_i)·q_c=N_i·(Bᵀq_c)`.

Consequently `Y_c=BᵀK_c` corresponds to **`Y_phys=AᵀK_phys`** and has the
regular normal fan with the same support offsets. For invertible A,

`area(K_phys)=area(Y_phys)/|det B|`.

If `P(h)=Σh_i h_(i+1)−aΣh_i²`, `a=(√5−1)/4`, then
`area(Y_phys)=P(h)/sin(2π/5)` and the fixed circumradius-one regular Q has
area `(5/2)sin(2π/5)`. Thus for actual complete capacity c,

`sys(K×_L Q)=c²|det B|/(5P(h))`.

The absolute determinant is essential. No capacity invariance under changing
one factor alone is assumed.

## Translation quotient and complete linear constraints

Translations act on regular-fan offsets through `h_i→h_i+N_i·z_Y`.
The two independent normals N₃,N₄ allow the exact gauge `h₃=h₄=0`.
The packet retains the 3×5 gauge projection, its 5×3 representative basis and
the two translation columns. It verifies exactly:

- Gauge projection times the translation basis is zero.
- Every closure and canonical edge row equals its truncated row times the
  gauge projection.
- The full area matrix satisfies `H₅=Q_gaugeᵀH₃Q_gauge`; equivalently its
  translation columns lie in the kernel.

The variables are

`(h₀,h₁,h₂,b₀₀,b₀₁,b₁₀,b₁₁)`.

For each of the complete five closure rays λ and ten original fixed-Q
triangles T, the transformed support is

`d_(λ,T)(B)=Σ_i λ_i max_(v∈T) [(B N_i)·v]`.

Every ray has exactly three positive entries. Nonnegative weights permit
independent vertex choices, so each such sum is exactly the maximum of **27**
linear forms in the four B entries. Every one of those choices is retained.
No attaining-label or redundancy pruning is performed.

For invertible B, every denominator is positive: translate a nondegenerate
triangle to contain zero in its interior, use nonzero transformed normals,
and then use closure to remove the translation. Complete template fitting
therefore gives

`c(K×_L Q)=min_l (λ_l·h)/D_l(B)`,
`D_l(B)=max_T d_(λ_l,T)(B)`.

The numerator is positive for a genuine full-dimensional body. Hence complete
capacity at least one is equivalent to **every** expanded inequality

`λ_l·h−Σ_i λ_li [(B N_i)·v_choice_i]≥0`.

The five canonical edge inequalities are retained separately. Zero edges stay
visible; this preflight removes no candidate body. The resulting linear
homogeneous cone C is seven-dimensional in the displayed variables.

## Counts, direction-preserving normalization and controls

| Quantity | Exact measured result |
| --- | ---: |
| Vertex choices per triangle/circuit | 27 |
| Fit rows per circuit | 270 |
| Raw fit rows | 1,350 |
| Raw canonical edge rows | 5 |
| Raw total | 1,355 |
| Distinct positive-normalized fit rows | 605 |
| Distinct positive-normalized edge rows | 5 |
| Distinct total | **610** |
| Full constraint rank | **7** |
| Cone lineality dimension | **0** |
| Maximum rational coefficient bit size in normalized rows | **5** |

All coefficients lie in `F=Q(√5)`. Rows are normalized by dividing by the
absolute value of their first nonzero coefficient in the actual positive-√5
embedding. This divisor is strictly positive; rows with opposite directions
are not identified. Every original source label and divisor is retained.

An exact regular-support witness uses constant ungauged support one and B=I.
After gauging, every one of the 610 normalized inequalities is strict.
This proves full dimension, independently of row rank. The same gauged
supports with **B=0** also satisfy every linear inequality strictly.
Therefore `det B=0` need **not** lie on the polyhedral boundary of C. It is a
boundary of the invertible/sign physical sector; singular matrices do not
define a body through the inverse transfer.

Every one of the 1,350 raw source choices is bound to the accepted scalar
catalogue by substituting

`B=[[x,−xu],[τ²u,1]]`.

The resulting coefficient order `(1,x,u,xu)` is exactly the retained preflight
catalogue. Its source hash is recorded. This is a coefficient identity, not a
rerun of a previous producer or inference from sampled matrices.

The known HKO point is checked directly at `x=1,u=1/√5` with its exact gauged
supports: all cone inequalities hold and
`P−det B/(3+√5)=0`. Its reflected negative-determinant representation is also
checked. These are **controls only**, not evidence of global positivity.

## Negative determinant and scalar normalization

Let `J=diag(1,−1)`. The regular normals obey `J N_i=N_(-i mod5)`.
Replacing B by B J and offsets by `h'_i=h_(-i)` merely relabels the same
physical K. It reverses det B. Restoring the support gauge yields the retained
exact 7×7 linear reflection map. The generator checks its square is identity,
that it permutes all 610 cone rows with positive normalization, and that it
preserves P while negating determinant. Thus the negative sector is explicitly
covered, rather than silently discarded.

For positive t, replacing `(B,h)` by `(tB,th)` gives the same physical body.
The template denominators scale by t, P by t², determinant by t², and complete
capacity is unchanged. Negative scaling is not asserted to be this equivalence.

For arbitrary capacity c>0, **keep B fixed** and replace h by h/c. This scales
the physical body by 1/c and puts it on the complete-capacity-one slice:

`P(h/c)=P(h)/c²`.

A proved cone bound `P(h/c)≥|det B|/(3+√5)` would therefore imply
`c²|det B|/(5P(h))≤(3+√5)/5`. The c² factor is not lost for bodies with
capacity greater than one.

## Actual next crux

On the positive determinant sector the proposed homogeneous quadratic is

`Q_+=P(h)−(b₀₀b₁₁−b₀₁b₁₀)/(3+√5)`.

Its exact symmetric 7×7 matrix is retained. The support block has inertia
`(1+,2−)` by its exact principal minors; the determinant block has inertia
`(2+,2−)`. Hence the ambient form has **inertia `(3+,4−)`**. It is not PSD,
and no PSD or quadratic-copositivity certificate is claimed.

Canonical edge geometry gives P≥0. Thus Q₊ is automatically nonnegative when
det B≤0; the substantive condition is its nonnegativity on C's positive-det
sector. Together with the exact reflection, this would settle the absolute
determinant body target. Singular matrices remain algebraic-domain points,
including interior cone points; they are not physical optimizers.

The next selected question would be exact quadratic copositivity on this
610-row seven-dimensional cone, or a precise finite-face/certificate reduction.
Neither a sampled ray nor ambient inertia resolves the mixed quadratic terms
on positive combinations. No extreme rays, LP pruning, arrangement, CAD or
positivity calculation was run, and dimension/counts supply no runtime or
success-probability estimate for that next stage.

## Reproduction and writes

```bash
python -B /workspaces/msc-math/topics/empirical-studies/discovery-design/night-20261005/orientation-wide/outer-matrix-preflight/generate.py
python -B /workspaces/msc-math/topics/empirical-studies/discovery-design/night-20261005/orientation-wide/outer-matrix-preflight/replay.py
```

Python standard library only. Generation reads the accepted scalar catalogue
and writes **only** its sibling `cone.json`. Final measured generation took
**2.340 seconds**. Replay recomputes the exact inventory and controls and compares
the entire mathematical payload, omitting only `elapsed_seconds`; it writes
**no files**. Field encoding is `["a","b"]=a+b√5` with rational strings.
Peer paths resolve from the source directory. Retained old inputs and other
agents' files were not overwritten.
