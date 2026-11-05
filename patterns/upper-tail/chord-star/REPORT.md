# A geometric coverage certificate for closure-chord approximation

17 October 2026. The broad upper-tail predictor remains open. This packet contributes a universal, target-free relative-capacity certificate for a small feasible optimizer. Its geometry preserves all closure tangent directions and remains continuous under fixed-facet perturbations, escaping both the principal-mode and reflected-contact discontinuities.

For six existing rational above-one controls near HKO, the new certificate raises the conditional-LP lower capacity bounds by **12.66–16.82%**, reducing the previous interval factors **1.159–1.170** to **1.00166–1.02909** after retaining the prior upper endpoint. Those capacities already had exact sparse-oracle evaluations; the result is the coverage theorem and a new approximation certificate, not a new capacity value or best global ratio bound. Four hard random controls expose poor global coverage. No broad scan is selected.

## Construction and invariant geometry

Write the centroid-normalized primal factors as

$$P=\{x:a_i\cdot x\le1\},\qquad Q=\{y:b_j\cdot y\le1\}.$$

Their closure polytopes and tangent projectors are

$$C_A=\{\alpha\ge0:\mathbf1^T\alpha=1,\ A^T\alpha=0\},\quad
 \Pi_A=\operatorname{proj}_{\ker[\mathbf1^T;A^T]},$$

and similarly for B. The center $c_A$ is the cone-volume closure: the area of each triangle from the primal centroid to one edge, divided by total factor area. Every component is positive. This uses exact rational areas and centroids on the pilot geometries.

For each nonzero $r_i=\Pi_Ae_i$, intersect $c_A+\mathbb Rr_i$ with $C_A$ and retain both endpoints. Equivalently, the endpoint parameters are

$$t_i^- =\max_{k:r_{ik}>0}\frac{-c_{Ak}}{r_{ik}},\qquad
 t_i^+ =\min_{k:r_{ik}<0}\frac{-c_{Ak}}{r_{ik}}.$$

Let $X_A$ be the convex hull of these endpoints and the center; define $X_B$ likewise. Both are full-dimensional in their respective closure affine spaces: the projector columns span the tangent, and every retained segment crosses the center. Set

$$\eta_A=\max\{\eta\in[0,1]:c_A+\eta(C_A-c_A)\subseteq X_A\},\qquad
 \eta=\max(\eta_A,\eta_B).$$

Thus $0<\eta\le1$. The finite code obtains the exact rational factor cover fractions by constructing the star hull and testing all individual closure vertices. The independent checker also constructs exact convex combinations proving that every contracted closure vertex lies in the star hull. It requires no paired capacity labels.

For each of the two full angular shadow words $s$, retain the actual bilinear HK payoff $Q_s(\alpha,\gamma)$, with each factor carrying mass one half. At every A-star endpoint, maximize this linear function over $C_B$ once; also exchange the factors. Let $q_\star$ be the largest resulting payoff over both words and roles. Define

$$C_\star=\frac1{2q_\star},\qquad
 S_\star=\frac{C_\star^2}{2\operatorname{area}P\operatorname{area}Q}.$$

An exact linear response may enumerate each factor's closure vertices. The rule does not enumerate the Cartesian product of the original closure vertices. It remains a small bilinear optimizer, with additional geometric containment work; no speed or complexity advantage over the existing tiny exact capacity problem is claimed.

Paired linear product symplectic changes preserve the cross pairings, cone-volume coordinates and tangent row spaces. Hence they leave the weight-space Euclidean projectors unchanged. Facet permutations conjugate them. Centroid normalization removes translations; positive factor scaling leaves the closure geometry unchanged and scales all payoffs homogeneously. Both roles and both signed shadows cover factor interchange and orientation reversal. These observations prove that the ideal scalar certificate and ratio ceiling are invariants of the Lagrangian product with its splitting.

No John or Löwner ellipsoid or physical-coordinate metric optimization is needed for this construction.

## Universal coverage theorem

Let $q_\ast=1/(2c_{\mathrm{EHZ}}(P\times_LQ))$. The imported sparse product reduction and two-shadow theorem give

$$q_\ast=\max_{s,\ \alpha\in C_A,\ \gamma\in C_B}Q_s(\alpha,\gamma).$$

Here $Q_s\ge0$ on every feasible pair. To justify this without importing a new arbitrary-support ordering theorem, decompose each closure into closure vertices. Every vertex has at most three entries. The existing two-shadow theorem identifies each of its two signed angular payoffs with a nonnegative mixed area. Deleting or inserting zero-weight labels leaves the objective unchanged. Bilinearity then expresses $Q_s$ at arbitrary closures as a convex combination of these nonnegative vertex-pair values. This establishes the required sign, while making no arbitrary-support claim about the optimal word.

**Theorem.** For every full-dimensional planar polygon pair with irredundant facets,

$$\boxed{\eta q_\ast\le q_\star\le q_\ast,\qquad
 \eta C_\star\le c_{\mathrm{EHZ}}\le C_\star,\qquad
 \eta^2S_\star\le\operatorname{sys}\le S_\star.}$$

**Proof.** Every evaluated star point and response is feasible, so $q_\star\le q_\ast$. Choose a globally maximizing pair $(\alpha_\ast,\gamma_\ast)$ and word $s$. By the definition of $\eta_A$,

$$\alpha'=\eta_A\alpha_\ast+(1-\eta_A)c_A\in X_A.$$

Since $X_A$ is the convex hull of the sampled points, one sampled point $x$ satisfies

$$Q_s(x,\gamma_\ast)\ge Q_s(\alpha',\gamma_\ast)
 =\eta_Aq_\ast+(1-\eta_A)Q_s(c_A,\gamma_\ast)\ge\eta_Aq_\ast.$$

The exact opposite response at $x$ attains at least this value. Exchanging factors gives the same conclusion with $\eta_B$. Both roles are included, so $q_\star\ge\max(\eta_A,\eta_B)q_\ast$. Invert the positive payoffs and square the capacity bracket to obtain the other statements. ∎

This proof supplies a genuine bound implication. It uses the complete imported sparse capacity characterization to establish the theorem; applying the certificate to a new input uses its geometry alone.

## HKO and continuity

At a regular pentagon, the cone-volume center is uniform. In cyclic labels the tangent projector has columns

$$ (\Pi_Ae_i)_k=\frac25\cos\frac{4\pi(k-i)}5.$$

Each positive radial endpoint hits two zero coordinates and is a closure vertex. The five endpoints supply every vertex of the regular closure pentagon. Therefore $X_A=C_A$, and similarly B, so $\eta=1$. The coverage theorem proves exact HKO capacity recovery. The quadratic-field control independently verifies $C_\star=5$, $\eta=1$ and $S_\star=(3+\sqrt5)/5$ in the established transformed normal convention.

The construction is continuous on each irredundant fixed-facet stratum. For $n\ge4$, a projector column cannot vanish: deleting one extreme dual vertex still leaves affine rank three, so that coordinate can vary in the closure tangent. The center, projectors and radial endpoints therefore vary continuously. Star hulls remain full-dimensional around their interior centers, so their containment fractions vary continuously as well. The finite maximum of continuous bilinear payoffs over a continuously varying compact closure polytope is continuous. Triangles have a singleton closure and are treated directly.

This excludes the preceding order-one directional error caused by dropping a split eigendirection or retaining only an exposed asymmetry face. It does not claim first-order accuracy, a numerical neighborhood radius, or a universal lower bound on $\eta$ independent of facet count and shape.

## Ten frozen retrospective controls

Membership and the mathematical rule were recorded in PLAN.md before the feature run. The first six rows of the existing near-HKO input packet, the three existing conditional-LP rational pilots, and the existing exact global counterexample were all included. New predictions were saved before the numerical score join. These geometries and comparison labels were already known to the project; this is a retrospective certificate diagnostic, with no held-out accuracy claim.

The exact HKO control and exact permutation, factor-swap, translation, positive paired-GL and negative paired-GL controls passed before the pilot run.

| Existing control class | Geometry-only $\eta$ | Standalone capacity interval factor $1/\eta$ | Actual capacity ceiling miss |
|---|---:|---:|---:|
| Six nonregular above-one near-HKO controls | 0.968263–0.998338 | 1.001664–1.032778 | 0.000747%–0.358000% |
| `random_5x5_s1_64` | 0.709533 | 1.409378 | 2.598235% |
| `random_5x6_s1_198` | 0.649050 | 1.540714 | True capacity unresolved; new ceiling is 1.987570% above an existing feasible upper bound |
| `random_6x6_s3_124` | 0.307649 | 3.250456 | 0.837380% |
| Exact counterexample `random_5x5_s0_92` | 0.647066 | 1.545438 | 0.041634% |

The near-HKO conditional LP intervals previously had factors 1.159423–1.170139. The new geometry-only lower endpoint improves them by 12.6645–16.8204%; retaining the old upper endpoint gives factors 1.001657–1.029093. The imported exact capacities lie inside every new bracket. Thus the coverage mechanism supplies substantially tighter approximation certificates in a regime where the conditional relaxation has a structural gap.

For the four hard random controls, every new ceiling is weaker than the applicable strongest retained feasible upper bound, and the new lower endpoints supply no improvement over the conditional certificates. The last control is particularly revealing: an accurate ceiling coexists with a weak coverage certificate. The geometry certificate is conservative, and near-HKO coverage does not imply broad coverage. No tested ratio ceiling exceeds HKO, but ten selected controls cannot support a global-ceiling conjecture.

**Stop:** retain the coverage theorem and bounded certificate diagnostic; do not launch a broad scan, add rays, tune weights, replace the center, or promote this to the requested broad predictor. Future broad progress needs a new argument controlling closure-body coverage or a different structural route. A new near-HKO cohort would repeat the local regime rather than settle that question.

## Verification and sources

`verify.py` imports no discovery code or numerical optimizer. It reconstructs every individual closure vertex, checks all projector kernel/idempotence identities and radial endpoints, verifies exact barycentric representations of every contracted closure vertex, recomputes all star-response maxima, and checks **2,364** actual four-dimensional HK sums. Its receipt records PASS for all ten geometries. This establishes finite feasible bounds and the geometric inclusions; the preceding argument supplies the universal implication. Imported capacities retain the acceptance limits of their evidence owners.

The exact feature run took 48.91 seconds; independent finite verification took 7.76 seconds. These are local wall-clock measurements, not account expenditure. No account quota or spend was queried. All output began in `/tmp/msc-math/upper-tail-next/20261017/chord-star/`. Root owns independent review, retention, integration and commits. No original-body capacity producer, paired capacity oracle, broad scan, fit, cloud work, installation, network action, thesis edit, PM mutation, push, merge or public action occurred.

Focused inspected sources were README/research/product/empirical and collaboration guides; the upper-tail synthesis; reflected-contact, spectral-mode, conditional-coupling/rounding, seed-coverage, fixed-closure, mixed-area-order and full-partner reports and their small helper implementations. The sparse product reduction and established two-shadow theorem are imported mathematical premises. This is a bounded local comparison, not an exhaustive repository/literature search or external novelty claim.

Reproduction in the patterns checkout, using a fresh scratch directory containing PLAN.md and these scripts:

```sh
python3 -B probe.py controls
python3 -B probe.py predict
python3 -B score.py
python3 -B verify.py
```

Full predictions remain an immutable scratch output; the compact certificate projection retains everything the independent checker needs. Replay the retained compact certificate with `python3 -B verify.py --input certificates.json.gz --out /tmp/msc-math/chord-star-replay.json`. Source hashes are recorded in sources.json. Independent agent scientific and exact-text review is recorded separately; human proof acceptance and external novelty are unassessed.

[review: /root/closure_review]
