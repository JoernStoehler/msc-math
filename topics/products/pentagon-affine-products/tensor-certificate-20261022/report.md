# 5×7 full-tensor containment certificate attempt, 22 October 2026

Producer report retained from the bounded run for `/root/msc_today_scope`. The subsequent [independent review](../tensor-certificate-review-20261022.md) accepted the final bytes and source correspondence; qualifications below describe the producer-stage evidence. A standalone Python checker using only exact integers and rational arithmetic **passed** on a finite certificate for all 840 six-block tensors of the regular 5×7 pair. This is a computational proof of the tensor containment *if the checker and the source-to-formula translation withstand independent review*. It is not by itself acceptance of a new theorem, the imported sparse reduction, or manuscript promotion. No 7×7 computation or repository edit was made.

## Artifacts and execution identity

- Mathematical source: `/workspaces/msc-math-local-cover-review-20261022/topics/products/pentagon-affine-products/research.tex`, checkout HEAD `5193274f835bc1e48108fe43a0377fc3edc00904`, SHA-256 `82863ee8f129fff607b774d12d48d45692842b37bab18500a3859b955f0ee4e1`.
- Rational certificate: `weights.json`, SHA-256 `cd21c3246cb962ae2b4d59d12492856c28f19ea1782eb620c40063c2edf861fc`. It gives 70 rational numerator weights over denominator `10^12` for each labelled triple pair/order and fixed correction basis `[0,1,18,38,14]`.
- Standalone verifier: `check_certificate.py`, SHA-256 `a183fc7a8da82859694d54a9fb8ecef870695587291af2c4912a12e31c3ed4dc`. It uses Python's standard-library `int` and `Fraction`, no NumPy/SciPy/Arb dependency. Result: `check-result.json`, SHA-256 `880e64e72ffeed8fdc290f5c22f67e17912f85ad727acb53c8497cb1b74534d7`, `status=PASS`.
- Numerical witness producer: `make_weights.py`, SHA-256 `2962c4cee83dae6e19a61fa0dfe8322762ffb9d6e2a38e580052931760952dfb`. It used cached offline Python 3.13.3, NumPy 2.5.3 and SciPy 1.18.1 for 840 LPs and rationalization. The checker does not trust the LP success status or floating-point matrices; it reconstructs exact input enclosures and verifies the supplied rational weights.
- Run logs: `producer-time.txt` recorded 2.09 s wall and 2.11 s CPU. The retained final `check-time.txt` recorded 5.05 s wall and 5.05 s CPU. The original packet had two earlier successful checker runs (5.30 and 5.29 s CPU). Producer plus all three checker runs stayed below 18 s CPU, below the 60 s bound. `python -O` was also tried on the final checker and exited 1 with the intended RuntimeError before any PASS. The existing rigorous-library probe found no local or cached `python-flint`/Arb, so this checker uses self-contained rational intervals; no package installation was attempted.

From this directory, reproduce the decisive check with:

```sh
python check_certificate.py
```

The supplied run guards against replacing an existing `check-result.json`; move/remove that output in a fresh copy before rerunning. The certificate and checker were produced under `/tmp/msc-math/tensor-certificate/20261022/final/` and are now retained in this repository directory. The original packet under its parent directory remains preserved. The reported checkout status included unrelated concurrent work under `topics/products/pentagon-affine-products/tensor-discriminator-20261022/`, local-cover review outputs, and `current.json`; this task did not touch them.

## Exact implication certified by the data

For each regular `P_n`, use exact trigonometric normals `n_i=(cos(2πi/n),sin(2πi/n))`, height `h_n=cos(π/n)`, and support-normalized dual vertices `u_i=n_i/h_n`, exactly as in `research.tex`. For a sorted index triple `(i,j,k)`, the checker forms cross-product barycentric weights

`(u_j×u_k, u_k×u_i, u_i×u_j) / ((u_j×u_k)+(u_k×u_i)+(u_i×u_j))`.

Interval signs classify every one of the `C(5,3)=10` and `C(7,3)=35` triples, with no undecided or zero case. Precisely 5 and 14, respectively, have three positive weights. The checker verifies their weighted steps sum to zero. For every pair of those genuine triples it enumerates the 2 orders of the final two first-factor steps and all 6 orders of the second-factor steps after fixing one first-factor step cyclically: `5×14×12=840` labels. It forms the exact target tensor `T=½(x_1 y_1^T−x_3 y_2^T)`.

The 70 listed four-block generators are `C_j=±½ outer(n_i/(1+h_5),n_k/(1+h_7))`. Let `q_j=N_j/10^12` be one certificate row. Define the 5×5 matrix `M` whose columns are `(vec(C_j),1)` for basis indices `[0,1,18,38,14]`. The checker verifies by nonzero interval pivots that the exact `M` is invertible. Set

`δ = M^(-1) ((vec(T),1) − Σ_j q_j (vec(C_j),1))`.

Define `w_j=q_j+δ_k` for the five corresponding basis indices and `w_j=q_j` elsewhere. **The identity** `Σ_j w_j (vec(C_j),1)=(vec(T),1)` holds algebraically by this definition, without relying on rounded equality tests. The rational interval calculation proves every corrected weight is strictly positive. The smallest certified lower bound among corrected weights is the exact rational

`1249999796152923685268743/125000000000000000000000000000`, approximately `9.99999836922339e-6`.

The checker explicitly asserts that every original rational weight is at least `1e-5`, and its result records this exact minimum. Thus the displayed bound is a global lower bound for all 70 corrected weights in every row. The largest certified absolute correction is approximately `1.75384073044497e-12`. Therefore each of the 840 exact tensors is a convex combination of the 70 exact four-block generators. Since `M` is invertible, the generators affinely span four-dimensional matrix space; all-positive weights put these tensors in the interior of that hull. The finite source definitions then give `T6(P5,P7) ⊂ int T4(P5,P7)`. Using the source's decomposition and affine covariance, this would extend the width-body capacity formula from rotations to independent nonsingular affine images of the 5×7 pair; those upstream theorems were not re-reviewed in this slice.

## Why the enclosures are rigorous

The checker uses Machin's exact identity `π/4=4 arctan(1/5)−arctan(1/239)`. It can be verified directly: the tangent of `4 arctan(1/5)` is `120/119`, and subtracting `arctan(1/239)` gives tangent 1; the angle lies in `(0,π/2)`. Alternating arctangent series bound each term with its first omitted term, yielding rational lower/upper bounds on π of width about `4.78e-58`.

Every sine/cosine argument is a rational multiple of that π interval. The checker evaluates a rational Taylor polynomial at its rational midpoint and encloses the error using the derivative bound 1 and the argument interval's radius. It explicitly checks `3<π<4` and `|midpoint argument|<82`, making its chosen `|x|^81/81!` remainder valid for both 41-term cosine and sine polynomials. The resulting bounds are rounded outward to integer endpoints at scale `10^30`. The interval class performs addition, multiplication, and division with exact integer/Fraction endpoint arithmetic and outward rounding; every division checks its denominator interval excludes zero. The Gaussian elimination for `M^(-1)` uses these inclusions and nonzero-pivot checks. Consequently no floating-point roundoff enters the checker PASS.

The final checker locates `weights.json` relative to its own resolved path and rejects `python -O` before any assertion can be disabled. The independent review target is the short interval implementation and the mapping from the source's cyclic-order convention to the 840 labels. A checker defect there would invalidate the proof claim. Passing this script does not review the source's reduction from arbitrary facet words, imported capacity formula, or further manuscript claims.
