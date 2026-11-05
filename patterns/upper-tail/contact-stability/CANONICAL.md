# A canonical product invariant with the same local loss law

The raw twenty-form contact defect becomes an actual invariant of unlabelled pentagon products after an explicit affine normalization. The resulting score $I(P,Q)$ is unchanged by translations, common dilation, product-preserving symplectic linear transformations and factor interchange. Its parameter-free prediction $\rho_H e^{-I}$ was frozen before a second fresh challenge: on **23 above-one cases among 45 distinct untransformed products**, median relative ratio error was **0.00656%**, worst error **0.12860%**, and $R^2=0.99879$. It classified all 45 cases correctly relative to one. These counts are finite observations, not a neighborhood guarantee for threshold classification.

The invariant is defined for products of convex pentagons. It is not asserted invariant under arbitrary four-dimensional symplectic transformations that destroy the selected product splitting. Its exponential and reciprocal transforms are first-order point predictions, not certified ratio bounds. The separate [broad-domain transfer](../canonical-transfer/REPORT.md) tests their failure away from the HKO neighborhood; a local theorem does not extend them globally.

## Definition: nine gauge parameters, eleven remaining directions

Let $p_0,\ldots,p_4$ be the cyclic primal vertices of $P$, with $p_i$ the intersection of dual facet rows $a_i,a_{i+1}$. Fix the reference HKO vertices $p_i^*$. Its triangle $(p_0^*,p_2^*,p_4^*)$ contains the origin strictly.

For each labelled ordered product:

1. Choose the unique affine map $x\mapsto Lx+t$ sending $(p_0,p_2,p_4)$ to $(p_0^*,p_2^*,p_4^*)$. Apply its product symplectic linear part to the other factor: $Q\mapsto L^{-T}Q$. The translation acts on $P$ alone. The six parameters remove the two A translations and four product linear-symplectic parameters.
2. Translate the transformed $Q$ by its negative area centroid. This removes its two translations.
3. Positively scale this centered $Q$ until the y coordinate of its first dual normal equals $b_{0,y}^*=1$. Keep only labels for which that coordinate was positive. This removes the remaining scale parameter.

The normalized $P$ contains the fixed triangle, so all its dual row denominators are positive. Centering puts the origin in the interior of $Q$ as well. A bounded polygon has normals with positive y projection, so at least one choice of its cyclic start is valid. The normalization fixes **nine parameters in the twenty-coordinate chart**; its local transversal has dimension eleven.

For an unlabelled product, first sort both normal lists counterclockwise. Enumerate both factor roles, both simultaneous orientations, and five starts in each factor: at most $2\times2\times5\times5=100$ candidates. Let $C_h(P,Q)$ be each valid normalized twenty-coordinate tuple. Define

$$\boxed{I(P,Q)=\min_{h\text{ valid}}D(C_h(P,Q)-z_0).}$$

Thus $I\geq0$. The only optimization is a finite minimum of a fixed geometric formula; there is no new capacity or closure-pair maximization. [canonical.py](canonical.py) implements these rational operations with generic float, rational or $\mathbb Q(\sqrt5)$ arithmetic. The predeclared formulas are

$$\widehat\rho_{\exp}=\rho_H e^{-I},\qquad
 \widehat\rho_{\rm rec}=\frac{\rho_H}{1+I}.$$

Both have the same first derivative at $I=0$. Neither has fitted coefficients. [CANONICAL-PLAN.md](CANONICAL-PLAN.md) froze the normalization, finite label set and both formulas before the new truth and before the separate retained 5×5 transfer scoring.

## Exact invariance proof

For fixed corresponding facet labels, replace the product by

$$P'=\lambda GP+u,\qquad Q'=\lambda G^{-T}Q+v,
 \qquad \lambda>0,\ G\in\operatorname{GL}(2).$$

Uniqueness of the affine triangle normalization gives $L'=\lambda^{-1}LG^{-1}$. Its normalized A polygon is exactly the original normalized A polygon. Applying $L'^{-T}$ to $Q'$ gives a translate of $\lambda^2L^{-T}Q$. Centering removes the translate. The remaining dual normals differ by the positive common factor $\lambda^{-2}$, which the final y-projection normalization removes. The normalized twenty coordinates therefore agree exactly. The sign deciding candidate validity also agrees.

Facet cyclic order is preserved by an affine map with positive determinant and reversed for both factors when the determinant is negative. Translating an interior origin rescales each dual normal by a positive number, preserving that cyclic order. The enumerated starts and simultaneous orientations therefore give a bijection of the candidate sets. Independent input label permutations disappear after the initial angular sort, and factor interchange permutes the two role choices. Taking the minimum proves the claimed invariance. This argument does not require a numerical optimizer or small perturbations.

## Why the canonical minimum has the local theorem

At exact HKO there are fifty valid candidates. The exact census in [canonical-verification.json](canonical-verification.json) verifies:

- Exactly ten have $D=0$, and **each has normalized coordinates exactly equal to $z_0$**.
- Every other candidate has $D\geq2\sqrt5-4>0$.
- Every one of the hundred final projection tests is strict, so the valid candidate family is locally unchanged.
- The reference triangle contains zero with strictly positive exact barycentric weights.

For each zero candidate, its normalization is an analytic ratio-preserving map fixing $z_0$. Apply the [raw local theorem](REPORT.md) to that normalized product. The normalized displacement is $O(\|z-z_0\|)$, so

$$\log\frac{\operatorname{sys}(P,Q)}{\rho_H}
 =-D(C_h(P,Q)-z_0)+O(\|z-z_0\|^2).$$

There are only ten such maps, hence one uniform remainder bound works for all. The strict positive gap excludes the other candidates nearby. Taking the finite minimum yields

$$\boxed{\log\frac{\operatorname{sys}(P,Q)}{\rho_H}
 =-I(P,Q)+O(\|z-z_0\|^2).}$$

As with the raw theorem, this is an existential local statement relative to the imported HK/product reduction, with no explicit radius or remainder constant. The exact census rules out a far normalized HKO candidate with zero raw gauge; raw tangent-kernel information alone would not have done so.

## Second fresh challenge and finite coordinate interventions

[freeze-canonical.py](freeze-canonical.py) constructs five independent SHA256 directions in the new namespace `contact-canonical-third-v1`, three perturbation mechanisms and three scales $0.01,0.04,0.1$: **45 distinct untransformed products**. Nine predetermined scale-$0.04$ products also receive the same fixed rational product symplectic transformation, common dilation and translations, producing **nine coordinate copies**. All 54 geometries are frozen regardless of their labels. There are 45 distinct untransformed cases from five shared seed directions, three mechanisms and three scales; these are correlated case groups, not 45 independent statistical draws. The nine copies are invariance controls.

The prediction hash was recorded before the oracle: [canonical-predictions.json](canonical-predictions.json), SHA256 `0a7e9275915f180e98c9560258d58eccf2d5dc1b30236e35892d4311b29d8fbe`. All exact rational capacities and ratios are in [canonical-truth.json](canonical-truth.json). Independent geometry and direct-HK checks replay all 2,700 objectives and verify five genuine facets per factor.

| Untransformed evaluation set | Count | Exp median relative error | Exp worst error | Exp ratio $R^2$ | Reciprocal median / worst |
|---|---:|---:|---:|---:|---:|
| All fresh perturbations | 45 | 0.06190% | 1.84873% | 0.99446 | 0.06980% / 2.23613% |
| Exact ratio above one | 23 | 0.00656% | 0.12860% | 0.99879 | 0.01190% / 0.16628% |
| Highest twelve ratios | 12 | 0.00387% | 0.02220% | 0.99909 | 0.00456% / 0.02672% |
| Scale 0.01 | 15 | 0.00456% | 0.02220% | 0.99976 | 0.00791% / 0.02840% |
| Scale 0.04 | 15 | 0.06462% | 0.33396% | 0.99648 | 0.11861% / 0.41220% |
| Scale 0.1 | 15 | 0.37852% | 1.84873% | 0.98111 | 0.65464% / 2.23613% |

Both canonical formulas classified all 45 distinct untransformed products correctly at one, including all 23 above-one cases. The unchanged raw-chart predictor missed one classification. The exponential prediction lies below the true ratio in 32 of the 45 cases, demonstrating directly why it must not be called an upper bound.

For all nine finite transformations, the ordered canonical coordinates agree **exactly** in $\mathbb Q(\sqrt5)$, and the independently recomputed rational ratios agree exactly. Numerical unlabelled scores differ by at most $3.9\times10^{-16}$. On these same coordinate copies, the raw fixed-chart exponential predictor has median relative error 17.28% and worst error 23.06%; the canonical exponential retains median 0.07042% and worst 0.33396%. This is a coordinate intervention on matched products, not nine additional prediction successes.

[canonical-metrics.json](canonical-metrics.json) retains all per-body predictions and errors, including all below-one cases, both formulas, and the ordered/raw diagnostics. No new data were used to fit or select the formula.

## Reproduction

Audit the retained packet:

```sh
python3 -B patterns/upper-tail/contact-stability/metrics-canonical.py --directory patterns/upper-tail/contact-stability
python3 -B patterns/upper-tail/contact-stability/verify-canonical.py --repo "$PWD" --directory patterns/upper-tail/contact-stability
```

For a fresh replay, copy the plans and executables to an empty scratch directory, together with the frozen first contact bank `predictions.json`. Run `freeze-canonical.py`, then `predict-canonical.py`, freeze its hash, then run `oracle-canonical.py`, metrics and verifier. Prediction and oracle refuse replacement. Nothing in these commands changes thesis sources or the original empirical corpus.

[review: /root/review_contact]
