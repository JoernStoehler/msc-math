# The local contact law has no exponential or reciprocal global upper bound

The exact rational pentagon product `random_5x5_s0_92` refutes both proposed universal extensions

$$\operatorname{sys}(P\times_LQ)\leq\rho_H e^{-I(P,Q)},\qquad
 \operatorname{sys}(P\times_LQ)\leq\frac{\rho_H}{1+I(P,Q)}.$$

Its exact ratio is approximately **0.8385328153862055**, while its exact canonical contact invariant is approximately **0.8731831290403553**. The reciprocal expression is only **0.5590556413117242**, a gap of **0.2794771740744813**. This is a large, exact obstruction to those two global functional inequalities; it leaves the local first-order law and HKO global-maximality conjecture intact.

## Frozen formulas and selected falsifier

[CANONICAL-PLAN.md](CANONICAL-PLAN.md) froze the two parameter-free transformations $\rho_H e^{-I}$ and $\rho_H/(1+I)$ before the fresh canonical challenge and the separate retained 5×5 transfer scoring. The [broad transfer packet](../canonical-transfer/REPORT.md) then exposed large errors. Its selected row supplied this adversarial certificate target. This body is therefore a post-score falsifier of a universal claim, rather than another blind holdout success or an above-one example.

[global-counterexample-input.json](global-counterexample-input.json) contains all ten exact rational dual normals, with $P=\{x:a_i\cdot x\leq1\}$ and $Q=\{y:b_j\cdot y\leq1\}$. They are binary-rational source coordinates; no rounding or regular-polygon replacement is used. The source identity is retained in the linked transfer packet. Both polygons have five genuine facets. Their closure polytopes have three and five vertices, respectively.

## Exact separation

[global-counterexample.py](global-counterexample.py) computes the canonical invariant in $\mathbb Q(\sqrt5)$, retaining every valid canonical gauge and its exact twenty-form maximum. The minimum over all fifty valid gauges is $I$. It independently computes capacity and volume with rational arithmetic, using every factor closure pair and the two complete angular words. Full coefficients and witnesses are in [global-counterexample.json](global-counterexample.json); their long exact fractions are machine-readable rather than rounded in this statement.

The independent [verify-global.py](verify-global.py) imports neither the canonical normalizer nor the counterexample producer. It reconstructs each of the hundred gauge candidates by solving affine-triangle equations with Gaussian elimination, checks every valid normalized coordinate and every defect, and then enumerates **all 1,800 cyclic words** on the $3\times5$ closure pairs. It also reconstructs the primal vertices and exact area product, verifying five genuine facets per factor. [global-verification.json](global-verification.json) records PASS.

For a compact exact certificate, the verifier establishes the strict rational separations

$$I>\frac45,\qquad \operatorname{sys}>\frac45,\qquad
 \frac{\rho_H}{1+I}<\frac35.$$

Thus the reciprocal proposed bound is below the true ratio by more than $1/5$. Since $I>0$ and $e^I\geq1+I$,

$$\rho_H e^{-I}\leq\frac{\rho_H}{1+I}<\operatorname{sys}.$$

This proves the exponential failure without needing a transcendental numerical certificate. The exact full-word capacity is conditional on the same imported HK/product reduction as the local theorem; the finite minimum and the strict comparisons use exact ordered-field arithmetic.

## What the obstruction changes

The canonical score measures the HKO first-order contact defect and gives accurate local upper-tail point predictions. A global argument needs another mechanism once the shape leaves the reference contact neighborhood: a globally valid upper branch, a nonlinear defect with a proven comparison inequality, or a controlled continuation across contact switches. The two simplest frozen transformations of this invariant already fail by a large margin. Increasing the amount of numerical fitting to those transformations would not repair the missing global implication.

Audit from the repository root:

```sh
python3 -B patterns/upper-tail/contact-stability/verify-global.py --repo "$PWD" --directory patterns/upper-tail/contact-stability
```

This replaces only the verification receipt. To reconstruct the full exact certificate, run `global-counterexample.py` in a scratch copy containing its pinned input, canonical API and frozen contact bank.

[review: /root/review_contact]
