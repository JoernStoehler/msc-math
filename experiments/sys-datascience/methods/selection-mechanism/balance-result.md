# Removing factor anisotropy weakens the ridge correlation

On 320 paired Lagrangian products, making both factors isotropic in vertex
covariance weakens the pooled within-group R/sys rank correlation from **−0.929
to −0.249**. Mean numerical sys increases from **0.342 to 0.654**; 314 bodies
improve and six worsen. R decreases in 306 pairs and increases in 14.

This gives a concrete geometric contribution to the broad population pattern:
much of that ordering disappears after a specified area-preserving deformation
of the factors. It does not establish that covariance anisotropy is the unique
cause or that changing the scalar covariance ratio alone produces the effect.
The transformation changes shapes and their relative position. No conclusion
about local optimality is made.

The residual association is heterogeneous. All ten baseline group coefficients
are negative, between −.994 and −.812. After balancing, the triangle×triangle
group has coefficient **+1**, while the other nine lie between −.571 and −.098.
Whitening triangles makes them equilateral; this numerical subgroup therefore
shows an opposite ordering along relative orientations. An exact analytic
account is a separate mathematical task, not assumed by this experiment.

## The intervention

For a polygon with distinct vertices v_1,...,v_k, set

    A = (1/k) sum_i (v_i - mean(v))(v_i - mean(v))^T,
    L_A = det(A)^(1/4) A^(-1/2).

This is the centered covariance of the **uniform distribution on vertices**,
not the area distribution. The positive definite square root specifies one
map. Its determinant is one and

    L_A A L_A^T = sqrt(det(A)) I.

Apply L_A to the first planar factor and the analogous map L_B to the second.
The two factor areas and combinatorial types are preserved. The product's
vertex covariance becomes diag(sqrt(det(A)) I, sqrt(det(B)) I), so its two
Williamson eigenvalues agree and their ratio is one. The block diagonal
transformation is generally not symplectic and need not preserve capacity.

The implementation maps each dual row d to d L_A^(-1), which describes the
image of the actual input body. It reconstructs the resulting polygons using
exact rational arithmetic on the stored binary64 dual coefficients before
computing floating-point R and covariance. Thus the checks refer to the actual
rounded evaluated bodies. Maximum relative factor-area change was 3.19e−13;
maximum deviation of the balanced covariance ratio from one was 5.05e−13.

## Design and checks

Each of the ten original k×m groups, 3≤k≤m≤6, contributes the 32 names having
smallest SHA256 of `covariance-balance-v1|` followed by the source name. This
selection uses no feature or target value. Both baseline and transformed inputs
were written and hashed at **15:21:49 UTC on 18 September 2026**, before target
evaluation. No body was replaced after target evaluation. All 640 requests
succeeded; all used the same current production structural-product evaluator.
The factor vertex counts and Cartesian-product vertex counts are unchanged.

The analysis reranks R and sys inside each of the ten groups, divides ranks by
32, and computes Spearman correlation of the pooled conditional percentiles.
All groups have equal size. It retains individual paired effects and every
group coefficient, including the positive triangle subgroup.

Volume from the production evaluator agrees with the product of independently
reconstructed exact-binary64 planar areas to relative error at most 5.33e−15.
The capacity outputs contain exact rational values and outward bounds; sys
remains a numerical ratio because the final volume division is floating point.
The largest balanced ratio was 0.8398284. No threshold claim is made.

Current baseline evaluations agree with the corresponding 320 historical
stored ratios to absolute error at most 1.23e−15. This corroborates those
selected product targets; it does not validate the entire historical table.

Independent statistical review checked the paired and grouped analysis.
Independent geometry/protocol review reconstructed all 320 pairs with a second
rational-intersection implementation, verified every active facet and the
lowest salted-name hashes, and checked the whitening matrix against an
independent closed-form two-dimensional square root. Mapped and independently
reconstructed vertices agreed relatively to 9.8e−15. A separate mathematical
review verified the determinant, covariance and dual-coordinate identities.

## Cost and reproduction

Cached package compilation took 18.68 seconds. Construction took 1.80 seconds.
The first ten matched bodies (20 requests) completed in 0.25 seconds, after which
the remaining 620 requests took 8.38 seconds. Summed per-request wall time was
8.37 seconds; the largest individual request took 0.029 seconds. Timing excludes
research, coding and review. One serial worker used the unchanged current-body
adapter, five seconds per request and an overall 300-second execution cap.

Recompute summaries without new target evaluations:

```sh
OPENBLAS_NUM_THREADS=1 python3 \
  experiments/sys-datascience/methods/selection-mechanism/analyze_balance.py
```

`balance-artifacts/` retains frozen inputs, source/build receipts, exact observed
outputs (losslessly compressed), geometry, per-pair and per-group tables,
timings and the summary. The analyzer verifies freeze hashes and the complete
paired grid before producing scientific summaries; error rows would produce
only an operational summary.

For a fresh reproduction, choose a new output directory (the producer refuses
to overwrite its freeze), run `balance.py --out /tmp/new-balance`, then use
`current-body-evaluator/build.py` and `evaluate.py` as documented in that
packet. Evaluate `pilot-inputs.jsonl` and `remaining-inputs.jsonl` into the
corresponding result filenames. `analyze_balance.py --packet /tmp/new-balance`
reads them. Source bytes must match the recorded freeze. The exact historical
source snapshot is named in `balance.py`; its SHA256 is in `freeze.json`.
