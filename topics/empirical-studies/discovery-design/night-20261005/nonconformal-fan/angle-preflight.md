# Angled nonconformal fan: allocation preflight only

Completed 2026-10-04 23:43 UTC. This records exact input counts and a proposed
first discriminating operation. **No angled parameter-cell decomposition,
partner-body enumeration, ratio optimization or new allocation was executed.**

## Pairing and parameter reduction

Consider physical covectors `G_x R_θ n_i`, `x>0`. Put
`y=tanθ/τ`, `τ²=5−2√5`. In the computational coordinates of
[denominators.md](denominators.md), a regular covector is `N_i=(N_ix,N_iy)`.
The transformed covector is exactly

`cosθ · (x(N_ix−y N_iy), N_iy+τ²y N_ix)`.

This follows by writing `sinθ=cosθ τy` and
`N_iy=τ n_iy`; it preserves the physical covector–point pairing.
On `0≤θ≤π/10`, `cosθ>0`, so this common factor may be removed from
the support maxima. For a template vertex `v=(v_x,v_y)`, the reduced pairing is

`N_iy v_y + x N_ix v_x + y τ² N_ix v_y − xy N_iy v_x`.

Thus each expanded weighted support candidate is bilinear in `(x,y)`,
with coefficient order **`(1,x,y,xy)`**, over `Q(√5)`.
The actual denominator is `cosθ` times the reduced upper envelope.
Dropping that factor during later capacity or area normalization would be an
error: `cos²θ=1/(1+τ²y²)` must be restored.

The family has orientation period `π/5` for its optimal ratio over arbitrary
supports. Indeed `R_(π/5)n_i=−n_(i+3)`; hence replacing `θ` by `θ+π/5`
permutes normals and centrally reflects the partner. The complete ten
templates obey `T_(j+5)=−T_j`, so template fitting capacity is invariant under
that reflection of the partner. Area is invariant too. This proves the period
without assuming the fixed pentagon itself is centrally symmetric.
Reflection in the horizontal axis commutes with `G_x`, permutes the regular
normal labels, preserves fixed `Q` and permutes the templates. It identifies
`θ` and `−θ`. Together these symmetries reduce orientation to

`0≤θ≤π/10`, equivalently **`0≤y≤1/√5`**,

using `tan(π/10)/tan(π/5)=1/√5`. These are symmetries of the complete
family and its capacity; a particular support vector generally requires the
corresponding relabeling or reflection.

## Actual finite counts and bounded cost

The closure rays remain the same five regular rays, since both `G_x` and
`R_θ` are invertible. Each ray has three positive entries. Exactly as for
the aligned calculation, expanding three vertex maxima gives `3³=27`
candidates per triangle, hence **270 per circuit and 1,350 raw total**.

The read-only exact count in [angle_preflight.py](angle_preflight.py) found:

| Quantity | Actual count |
| --- | ---: |
| Distinct bilinear candidates in each circuit | 121 |
| Distinct candidates with circuit labels retained | 605 |
| Distinct candidate coefficient tuples across all circuits | 371 |
| Unordered distinct candidate pairs per circuit | 7,260 |
| Pair comparisons across five circuits | 36,300 |
| Distinct pair-equality polynomials up to nonzero field scalar, each circuit | 1,365 |
| Union of these normalized equality polynomials across circuits | 4,420 |

An equality polynomial has degree at most two and bidegree at most `(1,1)`.
The **4,420** union count is an upper bound on distinct pair-equality curves
to consider, not an actual active envelope cell count. Some curves miss the
domain, some correspond only to inactive candidates, and reducible curves
may share components. The active arrangement, its actual cells and its useful
runtime are **unknown**. The aligned six-cell result does not estimate them.
A naive global arrangement can have quadratic growth in this curve count;
global semialgebraic decomposition is therefore a materially different
allocation from counting candidates.

Reproduction:

```bash
python -B /workspaces/msc-math/topics/empirical-studies/discovery-design/night-20261005/nonconformal-fan/angle_preflight.py
```

The script imports the existing exact quadratic-field arithmetic, enumerates
only bilinear coefficient tuples and normalized differences, and prints
counts. It ran locally in **4.72 seconds** with Python standard library only.
With `-B` it writes no files. No source data producer or paid API was used.

## Proposed first discriminating operation

The first useful scientific operation would be an exact **local wedge around
the lead-selected aligned optimizer `(x*,y)=(√5/3,0)`**, with `y≥0`.
It should not start by enumerating arbitrary examples or all global cells.

1. Evaluate all 121 reduced candidates per circuit at that exact base point.
   Retain its tied active candidates and certify a positive gap to every
   inactive candidate. Use continuity with explicit field bounds to obtain a
   small wedge where the inactive candidates remain inactive.
2. Derive the equality curves and dominance regions only for those tied
   candidates. This determines the actual local denominator branches,
   including the aligned breakpoint rather than assuming smoothness there.
3. Reuse the lead's fixed quotient constraints and nonsingular triple matrices.
   On each local denominator branch, obtain the feasible candidate supports
   and the corresponding area expressions. Include edge-loss candidates.
   Restore the `cosθ` scaling throughout. If the lead's aligned area convention
   is `sys=x/(5P)`, reduced supports give
   `sys=x(1+τ²y²)/(5P_reduced)`.
4. Decide whether any feasible branch improves on the aligned optimizer for
   arbitrarily small positive `y`, or certify local non-improvement on an
   explicit wedge, with equality and boundary cases stated.

This is a prospective plan, not a certificate that the wedge or an improving
branch exists. The actual active tie count and resulting branch count should
be the first measured costs before committing to broader execution.
The useful adverse stop is a certified local non-improvement or an exposed
failure of the inherited finite-vertex premise; retain the exact obstruction
and compare its scientific value before proposing a global search.
If local tie branching already requires a large semialgebraic arrangement,
return its measured count and a concrete alternative instead of silently
expanding into global execution. Candidate counts alone supply no sharp bound,
counterexample or justification for substantial expenditure.
