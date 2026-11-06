# Structural verification, second pass — 6 November 2026

Independent follow-up to [the first report](structural-verification-2026-11-06.md),
using its S-identifiers. Receiving owner: the assigning research coordinator.
This is verification evidence, not thesis acceptance or selection of further research.

## Scope, method and verdicts

Work was performed in `/workspaces/msc-math`, starting at commit
`fed92bad1937e8b79a6738c055426604d3bb997e`. Existing shared edits, including
PLAN/STATUS, were deliberately left untouched under the one-new-file instruction.
All fresh code and logs are under `/tmp/structural-verify-2/`; no branch producer,
checker, helper or fixture code was imported or executed. Mathematical input
formulas and retained candidate certificate values were read. Where serialization
orders mattered, the producer's row layout was inspected, then reconstructed in
new Sage code. All assertions below were actually executed.

Priority was exact chamber geometry, since S12 depends on its volume, followed
by the three finite LP certificate packages, contact calculus, and the small
analytic lead. This reused the first report's adequate dependency comparison;
a literature search or new parameter search would not answer these exact
certificate obligations. Luna performed short read-only source triage; the
primary agent wrote and ran the mathematics/code. Sol independently reviewed
S11/S13 and the support correction in S20. Two-parameter formulas (S22) were
excluded as requested and remain owned by their separate verification report.

| Claim | Verdict | Scope and outstanding obligation |
| --- | --- | --- |
| S11 exact chamber and volume | **Holds** | 75 simple vertices on closed `[3/10,2/5]`; all determinant/slack predicates and rational volume independently recomputed. |
| S13 exact sampled exclusion | **Holds** | Exact volumes and feasible eight-word bounds at `2/5,9/20` exclude improvements over the stated `3/10` benchmark. The `7/20` capacity certificate remains separate. |
| S12 volume qualification | **Removed** | S11 now establishes the volume input to the first report's already checked derivative/crossover argument. That argument was not re-executed here. |
| S14 four original directional slices | **Holds** | Rebuilt exact primal/dual feasibility, zero gap, and hull area for all four slices. |
| S16 sharp Steiner optima | **Holds** | All four directional LP optima and axis kite attainment checked; the first report supplies the independent geometric lower-bound proof. |
| S17 restoration certificate | **Holds** | Exact optimum for the one specified offset vector, with its relation to the retained axis Steiner optimizer freshly checked. No uniform restoration bound follows. |
| S9 remaining contact calculus | **Holds** | Fresh quadratic form, negative spectrum, rank-four elimination, twelve discriminant-zero pairs, and all twelve whole-ray deficits. No general copositivity conclusion. |
| S20 disjoint-support lead | **Fails as literally unrestricted expansion; corrected reduction holds** | The displayed expansion omits ambient-complement terms unless `r,s` are supported in `I union J`. Those terms cancel in the target inequality. Full proof below. |
| S19 finite probes | **Unchecked as complete experiments** | All twelve retained slope witness ratios freshly recomputed; hinge response/volume calculations and outer-search histories were not replayed. |
| S21 universal inequalities | **Undetermined** | No unrestricted rank-four/slope inequality, global original-template bound, or arbitrary three-hinge obstruction is proved by these scoped checks. |
| S22 two-parameter formulas | **Skipped** | Explicitly excluded; see the separately owned `hko-cut-family/certificate-two-parameter-20261029/verification-2026-11-06.md`. |

There is no failure of S11, S13, S14, S16, S17 or S9 in these recomputations.
The S20 failure is an omitted-support qualification, not a counterexample to
the slope inequality.

## Exact chamber and samples: S11/S13

Source definitions: `topics/variation-and-local-maxima/hko-cut-family/README.md`
and `continuation-20261029/README.md` in that directory. The fresh script is
`/tmp/structural-verify-2/geometry.py`; observed output is `geometry.log`.
It constructs the ten fixed support rows and ten rows `n_i+t D_i` literally
from the README, over the real embedding of `Q[a]/(a^4-10a^2+5)` with
`a=sqrt(5-2sqrt(5))`, then works over its rational-function field in `t`.

1. Enumerate every one of the `binomial(20,4)=4,845` bases at `t=3/10`.
   Reject singular bases; solve every remaining basis exactly and test every
   slack in the ordered algebraic field. Exactly 75 bases are feasible;
   each has exactly four zero slacks, so these are distinct simple vertices.
2. For each feasible basis, invert its symbolic matrix. Check its determinant
   numerator/denominator and all 16 inactive slack numerators/denominators
   have no root in the **closed** interval `[3/10,2/5]`. Sage AA polynomial
   root isolation supplies exact algebraic roots, not approximate root sampling.
   Every slack is positive at `3/10`. This executes all 75 determinant and
   1,200 strict inactive-slack obligations. Repeated identical predicates are
   memoized, without dropping any obligation.
3. The fixed ten rows contain the body in bounded HKO throughout; the origin
   is strictly interior throughout. A first combinatorial event must make a
   persisting vertex acquire an additional active row or lose its invertible
   active basis. Neither can occur. Boundedness prevents appearance from
   infinity. Thus these 75 simple vertices are the entire vertex set throughout.
4. At a simple vertex with active outward-row matrix `B`, vertex `v=B^-1 1`,
   inward tangent rays are `-B^-1 e_i`. The dimension-four Lawrence contribution
   for a generic covector `c` is
   `(c v)^4 / (24 |det B| product_i (c B^-1)_i)`.
   The four minus signs cancel. Summing independently for
   `c=(1,7,31,109)` and `(2,11,43,137)` gives identical rational functions and
   exactly the README's cubic-over-cubic formula, coefficient by coefficient.
   Possible isolated nongeneric covector parameters are removable in the
   rational identity; volume continuity extends the equality there.
5. At each of `2/5,9/20`, independently re-enumerate all bases and sum exact
   Lawrence terms. Each again has 75 simple vertices. The exact volumes equal
   the stated sample formulas. Solve the full augmented KKT system for word
   `(0,6,5,2,19,11,8,4)`; all weights are strictly positive, normalized, and
   closed. Recompute `Q=beta^T G beta/2` and compare the resulting exact ratio
   `1/(8 V Q^2)` against the benchmark field element.

| Parameter | Exact volume | Exact feasible Q | Approximate ratio bound |
| --- | --- | --- | --- |
| `2/5` | `-1968673 a^2/5198400+5954147/1039680` | `(-1444 a^3+2042071 a)/10113991` | `1.05178360352569` |
| `9/20` | `-613821503 a^2/1628542400+1858768317/325708480` | `(-6906384 a^3+8244042281 a)/40743770701` | `1.05106391321357` |

Both are strictly below `1.0519141388294875...`, using the benchmark formula
in the source README. This establishes the exclusions conditional only on
that separately certified benchmark, without a new capacity-maximization
search. It does not locate an optimum of the family. The fresh geometry script
completed in about 24.4 seconds. No `7/20` capacity predicates were replayed.

## Directional, Steiner and restoration certificates: S14/S16/S17

Sources are `alternative/README.md`, `steiner-gap.md` and the three result JSONs
in this report's directory. `/tmp/structural-verify-2/lp.py` reconstructs the
five planar normals from their mathematical formulas. It solves closure and
normalization for every triple, retains the five positive triples, and forms
both traversal orders for each, subtracting their centroids. This independently
constructs all ten triangle templates; the existing fixture is not executed.

For each original slice, add the supplied base translations then apply the
area-preserving shear `(x,y)->(x,y-q*x)`. For Steiner slices, independently
intersect each triangle with each distinct vertex height, compute the section
width, center its endpoints, take their convex hull, then swap coordinates.
Every reconstructed symmetral vertex set equals its saved proposal. Saved
vertex order is used only to address the serialized dual multipliers.

For every case, reconstruct the distinct height levels and LP rows directly:
`L_i-shift_j<=x`, `-R_i+shift_j<=-x`, followed by convex-left and concave-right
slope inequalities. Trapezoid weights give the area objective. With saved
primal `z` and nonnegative dual `y`, check exactly
`Az<=b`, `y>=0`, `A^T y=-c`, and `c.z=-b.y`.
Weak duality alone proves these optima; no numerical optimizer is used.
Independently compute the convex hull of every shifted point set using exact
orientation signs and verify its area equals the LP value.

| Case | Rows | Area / HKO area | Hull vertices |
| --- | ---: | ---: | ---: |
| Original, HKO horizontal | 66 | `1` exactly | 5 |
| Original, HKO slope `1/7` | 76 | `1` exactly | 5 |
| Original, centered horizontal | 84 | `1.22305090227266` | 10 |
| Original, specified rational offsets | 104 | `1.36508721764725` | 9 |
| Steiner, slope `0` | 78 | `0.723606797749979` | 4 |
| Steiner, slope `1/7` | 98 | `0.632774595940188` | 4 |
| Steiner, slope `1/2` | 98 | `0.620707970992628` | 4 |
| Steiner, slope `1` | 98 | `0.624668954934556` | 4 |
| Restored original templates | 70 | `1.15278640450004` | 6 |

The exact axis area is `a/5`; its four-vertex hull attains the earlier report's
segment lower bound, establishing sharpness. The restoration area is
`13a^3/100+a/4`. The script also checks every restored vertical offset equals
the corresponding slide variable of the axis Steiner solution (template zero
anchored), so this is indeed the claimed restoration experiment. The ratio
is `(8-sqrt(5))/5`. A first scratch run applied translations after shear,
failed a row-count assertion, and was corrected to apply the shear to the
translations as well; the final complete run passed all nine certificates.
That implementation error is not evidence against a source claim.

## Contact calculus: S9

Source: `rank-four-restart-20261029/continuation-20261029/contact-calculus.json`
under `topics/empirical-studies/discovery-design/desk/zonotope-research/`.
Fresh script: `/tmp/structural-verify-2/contact.py`; output: `contact.log`.

Use the eight actual old generators `e_i,-Ae_i/3`. Normalize each candidate
contact direction by its independently computed incident pairing norm `a(u)`.
For normalized directions `v_i`, expand the quadratic deficit at the base:
`H_ii=1` and
`H_ij=1+32|omega(v_i,v_j)|-2 sum_{p<q old}|Pf(p,q,v_i,v_j)|`.
Every entry matches the proposed 32 by 32 matrix. Exact algebraic isolation
of its characteristic roots counts **13 negative eigenvalues**.

The old-coordinate block has rank four. Its specified four pivot coordinates
are nonsingular. Eliminate them and independently calculate
`S=H_new,new-H_new,pivot H_pivot,pivot^-1 H_pivot,new`.
It agrees entrywise with the proposed 24 by 24 Schur matrix. Enumerating all
276 coordinate pairs gives exactly the listed twelve pairs satisfying all
three tested predicates: positive diagonal entries, negative off-diagonal
entry, and `S_ij^2=S_ii S_jj`. This establishes completeness for these
discriminant-zero pairs; it does not classify every possible two-coordinate
or multi-coordinate zero vector.

Finally use actual segment lengths `8+z r_i` on old normalized directions and
`z r_i` on new ones. For each retained ray, directly sum all absolute pairings
and four-Pfaffians with their length products. Old lengths stay positive for
sufficiently small positive `z`, including the six signed rays; new lengths
are nonnegative. The first six gap polynomials vanish. Rays 6 through 11 give,
in order,
`(9/500)z^4, (9/2000)z^4, (9/500)z^4, (9/2000)z^4,
(1/162)z^4, (1/162)z^4`.
All quadratic coefficients vanish. The calculation took about 0.6 seconds.
These checks do not establish local maximality or general copositivity.

## Disjoint-support reduction: S20

Source: final analytic lead in the same rank-four directory's
`rank-four-stop-state.md`. Let `C=g wedge h`, `W=g wedge r+h wedge s`, with
nonzero disjoint supports `I,J`, `G=||g||_1`, `H=||h||_1`. Write
`B=sum_l ||W wedge C_col(l)||_1` and `K=(I union J)^c`.

The displayed B formula is correct if `r_K=s_K=0`. Without that qualification,
it is false: `g=e1,h=e2,r=e3,s=0` gives `W=e1 wedge e3`, `C=e1 wedge e2`,
`B=1`, while every displayed term is zero. There was no explicit restriction
on r,s in the frozen lead. This is the literal failure recorded above.

The corrected formula is

```
B = GH (||g wedge r_I||_1 + ||h wedge s_J||_1)
    + G^2 ||h wedge r_J||_1 + H^2 ||g wedge s_I||_1
    + G^2 H ||r_K||_1 + G H^2 ||s_K||_1.
```

Proof: for `l in I`, `C_col(l)=-g_l h` and hence the summed contribution is
`G ||g wedge h wedge r||_1`. Split its triples by `IIJ`, `IJJ` and `IJK`;
the contributions are respectively
`GH||g wedge r_I||_1`, `G^2||h wedge r_J||_1`, and
`G^2 H||r_K||_1`. The `l in J` terms similarly give
`H||g wedge h wedge s||_1` and the other three terms. Columns outside the
supports vanish. Triple classes are disjoint, so no triangle inequality or
unjustified interchange of absolute values occurs.

Meanwhile the disjoint coordinate classes of W have norm

```
||W||_1 = ||g wedge r_I||_1 + ||h wedge s_J||_1
          + sum_{i in I,j in J}|g_i r_j-h_j s_i|
          + G||r_K||_1 + H||s_K||_1.
```

The extra K terms cancel when comparing `B<=GH||W||_1`. Set probabilities
`p_i=|g_i|/G`, `q_j=|h_j|/H`, values `X_i=s_i/g_i`, `Y_j=r_j/h_j`.
Then the remaining inequality is exactly
`(E|X-X'|+E|Y-Y'|)/2 <= E|X-Y|`.
For independent copies, use
`|x-y|=integral |1_{x>t}-1_{y>t}| dt`.
The right side minus the left equals
`integral (F_X(t)-F_Y(t))^2 dt >= 0`.
All distributions are finite, so the integral is finite and all interchanges
are finite sums. Signs of g,h are handled by the absolute-value probabilities.
If either vector is zero, C is zero and the original inequality is trivial.
Thus the proposed slope estimate **holds whenever g,h have disjoint supports**,
including arbitrary ambient r,s. It does not resolve overlapping supports.

`/tmp/structural-verify-2/slope.py` additionally tested the corrected identity
and inequality with exact rational arithmetic on 100 seeded eight-coordinate
examples, including two coordinates outside `I union J`. All passed. These
finite tests are supporting diagnostics; the proof is the decomposition above.

## Finite probes, remaining limits and handoff

The same fresh script independently reconstructs C and the twelve saved W
matrices from `slope-lp-results.json`, then directly computes every triple/column
term of B. All ratios agree with the recorded numbers within `1e-12` and are
below one; the maximum is `0.9921924333525534`. The largest floating
`W wedge C` residual is about `2.67e-14`. This checks the saved numerical
witnesses, not their exact feasibility or the optimization history of all starts.
It neither proves the inequality nor certifies the finite search's coverage.

The hinge-probe README and `probe-results.json` and outer-search README/result
schema were inspected. Their volume integrals, chart responses, all evaluated
outer candidates and termination histories were **not independently replayed**.
S19 remains unchecked as that larger experiment package. Next useful action:
reconstruct hinge chart evaluation and section-density integration from their
mathematical definitions, then rescore the saved floating profiles; for outer
searches reconstruct and rescore candidate placements before considering a
new search. Receiving coordinator owns prioritization; this assigns no new run.
S21 stays open rather than treating this verification as a universal theorem.

Independent reviewer `/root/sol_review` (gpt-6.1-sol, medium effort) read the
fresh geometry code/log and source definitions, and agreed with S11/S13 at
the scope stated above. The reviewer checked first-event sufficiency, exact
closed-interval isolation, Lawrence normalization, KKT and ratio conventions,
and separately agreed with the S20 correction/cancellation. No branch code
was executed by the reviewer. S9 and S14/S16/S17 were supported by our fresh
logs, not independently mathematically reviewed by Sol. Luna source triage
included a purported retained chamber JSON filename that was not needed or
used as evidence here.

This pass used two short native children, fresh exact arithmetic, and no API
calls from computation code. Work stopped conservatively within the roughly
30-minute allowance to respect the approximately $1 API-equivalent spending
budget; no reliable dollar meter was available, so that amount is not reported
as an observed charge. Scratch scripts/logs remain available at the absolute
paths above; they are not committed artifacts. Only this report is selected for
commit and push. Existing shared files are unchanged by this pass.
