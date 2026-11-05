# A global Steiner relaxation and its exact loss

29 October 2026. The global reduction is proved below. Its axis-direction LP
optimum is exactly certified, independently reconstructed by a second agent,
and replayable without a numerical solver. It is a lower bound on cover area;
its optimizer is not asserted to be an original-template cover.

## The global lower-bound mechanism

Fix a horizontal direction and write `S(K)` for Steiner symmetrization in the
vertical axis: replace every horizontal section of K by a centered interval of
the same length. For a convex K this remains convex and preserves area. Indeed,
if the section is `[L(y),R(y)]`, convexity makes L convex and R concave; the
width `w=R-L` is concave. The centered boundaries `-w/2,w/2` are therefore
convex/concave, and section-length integration preserves area.

Suppose K contains translates `T_j+(a_j,s_j)` of all templates. At each height
the section length of K dominates the corresponding section length of each
translated template. Thus

```
S(K) contains S(T_j)+(0,s_j) for every j,
area(K) >= area(conv(union_j(S(T_j)+(0,s_j)))).
```

Minimizing the right side over all relative vertical shifts gives a **global**
lower bound for arbitrary original two-dimensional placements. The relaxed
problem is itself a common-direction translation problem, so the finite
[envelope LP](envelope-lemma.md) solves it exactly after swapping axes.
A triangle's symmetral has at most four vertices, obtained from the centered
section widths at its distinct vertex heights. No smoothness is needed.

A determinant-one shear `(x,y)->(x,y-q*x)` permits other directions with exact
rational q. Centering horizontal sections in the sheared frame is an affine
version of Steiner symmetrization; it has the same area and convexity argument.

## Exact pentagon result

Put `tau=sqrt(5-2sqrt(5))`. In the unchanged exact fixture convention,

```
m_H = tau^3/20+tau/4 = tau(5-sqrt(5))/10.
```

For the symmetry-axis direction, the minimum relaxed cover has area

```
m_S=tau/5,
m_S/m_H=(5+sqrt(5))/10=0.7236067977499789... .
```

This is an exact optimum of the relaxed problem, certified by feasible primal
and dual solutions with zero gap in [steiner-results.json](steiner-results.json).
Its optimizer hull is a reflection-symmetric kite with four vertices. It is
not a parallelogram or an original-template cover. The loss relative to the
HKO area is exactly `(5-sqrt(5))/10`, about 27.64 percent.

The same producer checked three other exact directions:

| Direction slope | Relaxed area / m_H |
| --- | ---: |
| 0 | 0.7236067977499789 |
| 1/7 | 0.6327745959401879 |
| 1/2 | 0.6207079709926282 |
| 1 | 0.6246689549345563 |

Each row is an exact directional-relaxation optimum. These four values do not
establish an optimum over all directions. They give no reason to infer that
an orientation grid would close the original global problem.

## Why the axis relaxation is exhausted by two segments

Its exact dual uses point constraints from only symmetrized templates 0 and 4,
plus envelope-slope constraints. Template 0 supplies a vertical segment of
length `h=tau(5+sqrt(5))/10`; template 4 supplies a horizontal segment of length
`w=1-sqrt(5)/5`. Any convex body containing translates of these segments has
area at least `wh/2=tau/5`.

For the segment inequality, affine-normalize the horizontal segment to join
`(0,0)` to `(1,0)` and the vertical one to have length 1 at abscissa a. If
`0<=a<=1`, the two triangles with that vertical segment as base and the two
horizontal endpoints as apices lie on opposite sides and have total area
`a/2+(1-a)/2=1/2`. If a lies outside this interval, one of these triangles alone
has area at least 1/2. Scaling back gives the determinant bound.

The exact relaxed primal places all ten symmetrals in a kite attaining this
bound. Thus the gap is a loss of section-center compatibility, not failure to
optimize the relaxation. The same determinant lower bound is already available
from the original template-0 endpoint difference and template-4 horizontal
edge, so this relaxed optimum provides no stronger bound than those two
segments in this direction.

## Reproduction and review scope

From the checkout root:

```sh
uv run topics/capacity-and-qp/research-continuation-20261029/alternative/replay_steiner.py \
  topics/capacity-and-qp/research-continuation-20261029/alternative/steiner-results.json
```

This reconstructs every symmetral, checks the exact primal and dual constraints,
verifies zero gap and independently computes each reconstructed hull area.
The [producer](steiner_bound.py) additionally runs numerical basis discovery;
it accepts `--out` and refuses an existing output path.

The second agent independently implemented section intersections at each
triangle's vertex heights, ran the shared reviewed LP certificate routine, and
obtained the same axis value `tau/5`. It also separately checked the geometric
Steiner implication. Its independent construction used redundant zero-width
vertices (106 rows instead of 78), which left the exact optimum unchanged.
An early private message misread Horner coefficient arrays as constant terms;
the numeric result and certificates were correct, and the formulas above use
the corrected descending-power interpretation.

The primary unresolved step is a lower bound that retains or controls the
section-center compatibility discarded by Steiner symmetrization. This packet
does not claim that a pointwise correction at one relaxed optimizer supplies a
uniform global correction, nor that the HKO cover is the global minimum.

## An exact center-restoration test

Take the vertical offsets in the retained axis-direction Steiner optimizer.
Keep those offsets, restore the original ten triangles, and optimize all nine
horizontal shifts using the original envelope LP. The exact optimum is

```
A_restore=13 tau^3/100+tau/4,
A_restore/m_H=(8-sqrt(5))/5=1.152786404500042...,
A_restore-m_S=13 tau^3/100+tau/20=0.0861841645009744... .
```

[center-restoration.json](center-restoration.json) retains the complete exact
primal and dual certificate and offsets. This demonstrates a real compatibility
loss: no horizontal completion of these relaxed-optimal offsets even reaches
HKO. It is not a lower bound uniform over every relaxed minimizer, and it does
not rule out other perpendicular-offset vectors with smaller original area.

Replay without a numerical LP solve:

```sh
uv run topics/capacity-and-qp/research-continuation-20261029/alternative/replay_envelope.py \
  topics/capacity-and-qp/research-continuation-20261029/alternative/center-restoration.json
```

The [producer](center_restore.py) takes explicit `--steiner` and `--out` paths;
its source uses the first retained Steiner case, which is the horizontal case.
Its new output is refused if the destination already exists. The exact
certificate replay checks the same LP construction as the main producer.
