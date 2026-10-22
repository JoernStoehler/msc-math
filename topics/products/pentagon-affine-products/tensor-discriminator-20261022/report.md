# Bounded full-tensor discriminator, 22 October 2026

Private report for `/root/msc_today_scope`. This is a numerical probe of the stronger four-dimensional containment `T6 ⊆ T4` from `topics/products/pentagon-affine-products/research.tex`, not an exact proof or theorem/counterexample. It stopped after the 3×3 and 5×5 controls and the 5×7 case; no 7×7 run, broader search, source edit, registry edit, or manuscript edit.

## Identity and method

Source checkout: `/workspaces/msc-math-local-cover-review-20261022`, HEAD `5193274f835bc1e48108fe43a0377fc3edc00904`. `research.tex` SHA-256 `82863ee8f129fff607b774d12d48d45692842b37bab18500a3859b955f0ee4e1`. Producer `/tmp/msc-math/tensor-discriminator/20261022/discriminator.py` SHA-256 `2f68bd8dbb62c8f77bdad88634edf1ca8b4eb8dc03dc5508a2fbee6c4493e4b3`. Full generated inputs, triples, orders, four-block generators, six-block matrices, facet residuals, and worst records are in `results.json` (SHA-256 `7a0fb9ecda78aaf586ef29405c7ac4afb712a70357ee147de91cc831ce9b1ccb`). `lp_crosscheck.py` and `lp_crosscheck.json` provide an independent membership check of each worst tensor.

The script evaluated unit normals `n_i=(cos(2πi/n),sin(2πi/n))`, `h_n=cos(π/n)`, and duals `u_i=n_i/h_n` in floating point. For every index triple it computed the unique closure barycentric weights by the cross-product/Cramer's-rule identity, retained strictly positive triples, and checked closure numerically. It used every pair of genuine triples and twelve cyclic alternating orders: a fixed first A step, both orders of the other A steps, and all six B permutations. For each order it formed `T6=0.5*(x1*y1^T−x3*y2^T)` as a full row-major 2×2 matrix. It formed the four-dimensional hull from `±0.5*outer(n_i/(1+h_n),n_j/(1+h_m))`. SciPy `ConvexHull` facet inequalities have unit Euclidean normals; maximum `normal·T+offset` is the signed residual, positive outside. The classification tolerance was `1e-10`.

Run used cached offline `uv` with Python 3.13.3, NumPy 2.5.3, SciPy 1.18.1, one BLAS thread, a shell CPU limit of 30 seconds, and no package installation. `/usr/bin/time` recorded 0.56 seconds wall, 0.34 user + 0.11 system CPU for the producer; the three-worst-tensor LP cross-check took 0.52 seconds wall, 0.43 user + 0.08 system CPU. Total producer/check CPU 0.96 seconds. Logs: `time.txt`, `lp_time.txt`. The only task output was under this `/tmp` directory. A concurrent/unrelated checkout status showed modified `meta-topics/project-management/current.json` and an untracked local-cover verification directory; this task touched neither.

## Results

| Pair | Genuine triples | Triple pairs | T6 orders | T4 signed generators | T4 facets | Largest signed facet residual | Numerical result |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 3×3 negative control | 1, 1 | 1 | 12 | 18 | 66 | +0.13608276348795426 | outside |
| 5×5 positive control | 5, 5 | 25 | 300 | 50 | 266 | +5.55e-17 | boundary within rounding |
| 5×7 discriminator | 5, 14 | 70 | 840 | 70 | 490 | −0.000348071292960675 | all tested tensors strictly inside numerically |

The 5 and 14 triple counts agree with the classifications stated in the source. The negative control's worst tensor has an LP membership problem reported infeasible, and its direct support violation is +0.13608276348795423. The positive control's worst tensor has an LP convex combination with residual 1.55e-15 and zero direct support gap. The 5×7 worst tensor has an LP convex combination with residual 1.39e-17; direct support recomputation gives −0.000348071292960689, agreeing with the hull equation. Since this is the maximum residual over all 840 orders and 490 facets, the test found no 5×7 outside tensor.

For reproducibility, the 5×7 closest tensor comes from A triple index 4, normal indices `[1,3,4]`, with local order `[0,2,1]`, and B triple index 4, normal indices `[0,3,5]`, with local order `[0,2,1]`. Its matrix is

```
[[ 0.03255899757294125, -0.027219155786518096],
 [ 0.11481297695837854, -0.0197758742553086 ]]
```

Its closest four-block supporting facet has unit-normal direction

```
[[ 0.07565291585310213, -0.6832598779750233 ],
 [ 0.7244495596527689,   0.0510432266945279 ]]
```

with support 0.1035757976251119 and determinant +0.4988488866614022. This direction is **not** a numerical separator: tensor value falls short of support by 0.000348071292960689. The determinant is only a diagnostic of the four-dimensional direction. The negative control's separating direction has determinant −0.5.

## Interpretation and boundary

The numerical 5×7 run supports full tensor containment, a stronger condition than the source's proved angular containment. It may point to an independent-affine extension, but the rounded trigonometric inputs, positive-triple classification, hull facets, and strict margin have not been certified with exact interval/rational arithmetic. The two controls and LP checks reduce obvious convention and implementation risk; they do not make this a proof. The source's decomposition theorem and its imported hypotheses were not independently re-reviewed in this slice. No 7×7 test was run.
