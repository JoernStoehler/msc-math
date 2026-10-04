# Preliminary calibration extensions, 3 October 2026

Initial bounded preparation for programme selection. Subsequent independently
agent-checked results are in `odd-regular-symmetric-partner.md` and
`local-calibration-stability.md`: all odd regular factors with at least five
sides now calibrate, as do fixed-facet nonregular neighborhoods. This note
retains the initial criterion, pilot and triangle obstruction; it is not an
assignment to rediscover those extensions. No human acceptance or substantial
research-run approval is established.

## Affinely regular pentagons

The selected symmetric-partner statement immediately extends to an affinely
regular pentagon `Q_G=GQ`, for invertible `G`. The linear symplectic map
`diag(G^T,G^-1)` sends `K ×_L GQ` to `G^T K ×_L Q`. Central symmetry is
preserved, so the selected theorem yields

```text
c(K ×_L Q_G) = max{r: r(Q_G-Q_G)^polar ⊆ K-K}.
sys(K ×_L Q_G) <= (10+6 sqrt(5))/25.
```

Indeed `(G(Q-Q))^polar=G^-T(Q-Q)^polar`; multiplying the inclusion defining
`W_Q(G^TK)` by `G^-T` proves the first equality. The sharp equality partner
is a translate of a positive dilate of `G^-T B`. Product areas are unchanged
by this coordinate transformation. This increases the factor class without
introducing new covering machinery.

## What extension of the calibration method actually requires

For any planar polygon `Q`, let `B_Q=(Q-Q)^polar/2`. The segment argument gives
`c(K ×_L Q)<=W_Q(K)` for every partner. If `K` is centrally symmetric, it
contains a translate of `W_Q(K) B_Q`, giving

```text
W_Q(K) c(B_Q ×_L Q) <= c(K ×_L Q) <= W_Q(K).
```

Consequently the exact formula `c=W_Q` holds for *all* centrally symmetric
partners if and only if `c(B_Q ×_L Q)=1`: sufficiency follows from these
bounds, and necessity follows by taking `K=B_Q`, where `W_Q(B_Q)=1`.
By BMP's covering criterion, this is equivalent to every normalized normal
triangle of `Q` translating into `B_Q`.

Thus the proposed research question has a precise criterion, not a generic
request for more examples. A failed template supplies a counterexample to the
unrestricted extension, and a complete template cover proves an infinite class.

## Small numerical discrimination

`regular-factor-calibration-probe.py` enumerates positive three-normal closures
for circumradius-one regular polygons with 3, 5 and 7 sides, normalizes their
oriented support-function length, and solves a translation/dilation LP into
`B_Q`. It only prints new evidence; no retained dataset or producer is replaced.
The JSON output records each worst template and residual.

| Sides | Positive closure triples | Largest minimum cover dilation |
| --- | ---: | ---: |
| 3 | 1 | 1.3333333333333335 |
| 5 | 5 | 1.0000000000000002 |
| 7 | 14 | 1.0 |

These counts omit reversed cyclic traversals: reversing the edge order centrally inverts
its underlying triangle up to translation, and the reference body is centrally symmetric.
The LP evidence is floating-point exploratory evidence, not an exact proof of
the heptagonal extension. The pentagonal result is already supported by the
separate analytic argument. A bounded exact heptagon proof was subsequently obtained and independently
checked in `heptagon-symmetric-partner.md`. It remains agent-derived and lacks
human acceptance or a novelty determination.

Reproduce this small probe with
`uv run docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/regular-factor-calibration-probe.py`.

## Exact obstruction for the regular triangle

With normals at angles `0,2pi/3,4pi/3`, a unit-length normal triangle has
vertices `0,(2/3,0),(1/3,sqrt(3)/3)`. The circumradius-one factor has vertices
`(1/2,sqrt(3)/2),(-1,0),(1/2,-sqrt(3)/2)`.
Three difference vectors are

```text
d1=(-3/2,-sqrt(3)/2), d2=(3/2,-sqrt(3)/2), d3=(0,sqrt(3)).
d1+d2+d3=0.
```

The template's support values in these directions are `0,1,1`. If a translate
fits in `rB_Q`, all three support values after translation are at most `r/2`.
Summing cancels the translation and gives `2<=3r/2`, hence `r>=4/3`.
Translation by `(-1/3,-sqrt(3)/9)` attains this value for all six difference
directions. Thus the reference body does not calibrate at capacity one:
BMP's criterion and homogeneity give `c(B_Q ×_L Q)=3/4`, while `W_Q(B_Q)=1`.

## Selection implication and limits

The initial heptagon target has been solved and subsumed by the all-odd-regular
theorem. The geometric larger-class target has an open fixed-facet neighborhood
result and an exact rational nonregular control in
`rational-calibration-control.md`. These results do not establish novelty, an
all-polygon theorem, a quantified neighborhood radius, or an accepted prose
improvement. Do not spend a multi-hour programme on proving the already solved
extension. Reuse its proof in the supported product account and compare any
genuinely unresolved objective against proof simplification and identified
reproduction-route repair before substantial execution.
