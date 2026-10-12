# Dominance screening and complete one-circuit orientation events

Completed 2026-10-05 00:11 UTC under the
[bounded orientation preflight contract](../../comparison/orientation-preflight-contract.md).
This is a complete **circuit-0 denominator** topology over the orientation
strip, plus exact global dominance screening for all five circuits. It is
**not** complete two-parameter capacity coverage or a global ratio theorem.
No other circuit's kinetic events or partner bodies were enumerated.

## Inputs, field and actual reduction

Put `z=√5`, `τ²=5−2z`, `U=1/√5=z/5`. The domain is
`x>0`, `0≤u≤U`. Computational covectors are `N=(n_x,τ n_y)` and
the correct scaled normal is

`G_x C_u N=(x(N_x−uN_y),τ²uN_x+N_y)`.

The original ten fixed-Q templates and five regular closure rays are copied
locally in [active_events.py](active_events.py). No old producer is rerun.
Their conventions are owned by the
[accepted regular core](../../geometry-discovery/fixed-q-rank/equiangular-capacity-core.md).
All coefficients use exact rational-pair arithmetic in `Q(√5)`.
The candidate coefficient order is `(1,x,u,xu)`.

Each ray has three positive entries. Independent vertex maximization gives
27 affine choices per triangle, 270 per circuit and **1,350 raw choices**.
After identifying equal bilinear polynomials there are exactly **121 distinct
candidates per circuit**, retaining every original template/vertex source label.

For candidates `r,s`, write their difference as

`r−s=(a₀+a₁u)x+(b₀+b₁u)`.

Candidate `r` dominates `s` throughout the closed orientation strip if
the slope and intercept differences are nonnegative at **both** `u=0,U`.
Affine interpolation proves they are then nonnegative for every `u` in the
strip and hence for every `x>0`. The actual screen gives:

| Circuit | Distinct input | Surviving candidates | Eliminated candidates |
| --- | ---: | ---: | ---: |
| 0 | 121 | 5 | 116 |
| 1 | 121 | 6 | 115 |
| 2 | 121 | 8 | 113 |
| 3 | 121 | 7 | 114 |
| 4 | 121 | 6 | 115 |
| Total | 605 | **32** | 573 |

Every eliminated candidate has retained dominating-index witnesses, exact
endpoint slope/intercept differences, and boundary whole-x and x-zero tie
flags. A witness chain ends at a survivor: dominance is transitive; two-way
dominance would force both affine differences to vanish identically, hence
the two bilinear polynomials to be equal, which deduplication excludes.
Thus this finite strict partial order has no directed cycle.

A distinct dominated polynomial is strictly lower for `0<u<U`, `x>0`:
each nonnegative affine difference is positive in the interior unless
identically zero, and both cannot vanish identically for distinct polynomials.
Consequently removing it cannot lose an interior active candidate.
Boundary-only ties are retained through the original sources and explicit raw
endpoint hulls. Equal polynomials were merged only by retaining all their
source labels, not by discarding those labels.

The 32 survivors would give **89 pair comparisons** and **141 triple
comparisons** across all circuits. These are reduced prospective counts;
only circuit 0's **10 pairs and 10 triples** were processed here.

## Complete event calculation for circuit 0

Treat its five candidates as x-lines

`L_i(x,u)=a_i(u)x+b_i(u)`.

Slope differences and intercept differences are affine in `u`. Triple
concurrency is the degree-at-most-two polynomial

`(a_i−a_j)(b_i−b_k)−(a_i−a_k)(b_i−b_j)`.

The calculation retains **30 event polynomials**: ten pair-slope, ten
pair-intercept and ten triple-concurrency polynomials. Every triple was
processed; no identically-zero event polynomial occurred. Repeated roots are
retained by exact root deduplication, so a stationary double-root event would
not be skipped. The endpoint/vertex quadratic range test is exact and includes
the quadratic extremum whenever it lies inside the strip. It excludes only
polynomials strictly positive or strictly negative throughout the closed strip.

The actual roots in the strip are **only its two endpoints**, `u=0,U`:

| Orientation | Event causes, using retained candidate indices |
| --- | --- |
| `u=0` | Intercept equality `(60,107)`; triple `(12,14,36)` |
| `u=U` | Slope equality `(12,14)`; triple `(36,60,107)` |

There is **no interior event orientation**. All retained roots lie in
`Q(√5)` and are exactly ordered there. Some outside-strip event polynomials
have nonsquare discriminants; their roots were excluded by exact quadratic
range signs, without adjoining or approximately ordering them. A genuine
unexcluded quadratic root would lie in `Q(√5,√Δ)` (degree at most four over
`Q`) and the script explicitly stops if its exact square root is unavailable
in `Q(√5)`. No such stop is needed for this completed in-strip calculation.

### Why the list cannot miss a topology change

An x-line upper hull can change its combinatorial order only when:

- Two slopes coincide, allowing a parallel-line or infinite-tail transition.
- An intersection passes through `x=0`, requiring an intercept equality.
- Two consecutive hull intersections collide, requiring triple concurrency.

These are exactly the retained pair and triple events. The x-lines have
continuous affine coefficients. In an event-free open orientation interval,
distinct slopes remain ordered, pair intersections remain continuous, no
intersection crosses zero, and no two intersection orders can reverse.
Hence a hull verified at one exact interior orientation has the same active
sequence throughout that interval. Parallel and persistent-concurrency cases
were checked through all event polynomials; none persists here. The x>0
clipping and unbounded x-tail transitions are expressly included.

At the exact midpoint `u=U/2`, the hull contains **all five survivors**, in
increasing-x order **`60,107,36,14,12`**. Therefore throughout `0<u<U` there
are **five x-cells** with this sequence. Their four boundaries are retained
symbolically as exact ratios of affine functions of `u`:

`x_ij(u)=[b_i(u)−b_j(u)]/[a_j(u)−a_i(u)]`.

The JSON records numerator and denominator coefficients, the adjacent
candidate IDs, and the sample hull. There is exactly **one open orientation
patch**, not a sampled collection of fixed-angle results.

For convenient independent inspection, the five active bilinear functions are:

| ID | Constant | x coefficient | u coefficient | xu coefficient |
| --- | --- | --- | --- | --- |
| 60 | `(5−z)/20` | `(z−1)/20` | `(5−z)/20` | `(15−7z)/20` |
| 107 | `(5−z)/20` | `(13−5z)/20` | `(7−3z)/4` | `(15−7z)/20` |
| 36 | `(3z−5)/20` | `(1+z)/20` | `(3z−5)/20` | `(5−3z)/20` |
| 14 | `(5−2z)/10` | `(4−z)/10` | `(9z−20)/10` | `(2z−5)/10` |
| 12 | `0` | `1/5` | `0` | `(2z−5)/5` |

## Endpoint and isolated ties

Both endpoint orientations are recomputed using **all 121 original distinct
polynomials**, with all their 270 source choices retained. These raw hulls
recover discarded boundary-only candidates and every isolated tie at a finite
x-breakpoint. The JSON also records x-zero ties and maximum-slope candidates
at infinity separately from candidates attaining the actual finite tail.
The open-patch sample uses the full raw hull as well, preserving x-zero and
finite-intersection ties. No whole-cell source list is substituted for these
endpoint tie lists.

At `u=0`, the finite positive hull has three x-cells, with boundaries
`x=z/3,z`. At `u=U`, it also has three x-cells, with boundaries `x=1,z`.
The JSON retains the extra endpoint candidates whose polynomials coincide
there, and the candidates touching only at a breakpoint. These agree with
the separately accepted aligned and HKO-angle slice formulas; their producers
were not rerun to perform this comparison.

## Reproduction, cost and remaining proof scope

```bash
python -B /workspaces/msc-math/docs/empirical-viterbo-design/night-20261005/orientation-wide/preflight/active_events.py
```

The script needs Python standard library only and writes **only** its sibling
[active-events.json](active-events.json), deterministically apart from measured
runtime fields. Exact field elements encode `["a","b"] = a+b√5` with rational
strings. The initial all-five dominance screen took **9.53 seconds** locally;
the final run with full witness values and complete circuit-0 event/tie output
took **10.07 seconds**. Each command remained below 30 seconds. No full joint
arrangement, CAD, body enumeration, positivity theorem or paid API was run.

This result substantially reduces the proposed denominator stage: a complete
circuit-0 topology needed only 30 event polynomials and one open orientation
patch. It does not prove analogous complexity for the remaining four circuits.
Their kinetic sweeps, joint refinement, ten-fit-triple feasibility, area gaps,
stationary cases, physical normalization and equality classification remain
outside this executed preflight. The lead owns any subsequent allocation and
the independent reviewer owns the no-miss and dominance audit.
