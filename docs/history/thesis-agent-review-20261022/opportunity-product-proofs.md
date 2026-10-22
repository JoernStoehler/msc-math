# Product-family finite and limiting proof bridges

Read-only review of `/workspaces/msc-math`, 22 October 2026. Task: `opportunity-product-proofs`; authorized ceiling: 40 minutes. No repository edits or git writes were made.

**Result:** no surviving proof defect found in the named bridges. Nine obligations are **solved** in the inspected source; two candidate objections are **withdrawn**. “Solved” means an existing repair was verified, not that this review changed the manuscript. Severity **none** denotes a closed obligation, rather than an outstanding defect. This is a bounded mathematical/source review, not human acceptance, formal verification, implementation certification, or literature-novelty review.

## Source identity and coverage

The controlling scope is `meta-topics/project-management/thesis-work.md:96`–`136`, especially its three explicit selected paths at lines 106–108. I read those three files completely. `thesis/main.tex:74`–`77` now inserts `10b-symmetric-partners.tex` before product-position, so the last audited file is printed after Section 11. This review follows the backlog's file list; it does not claim a full audit of the new symmetric-partner section.

Checkout HEAD observed: `9667e351514faf1f00e2d86796c20e9d9b6f2f10`. Inspected-file SHA-256:

| Source | SHA-256 |
| --- | --- |
| `thesis/chapters/09-rotated-regular-polygons.tex` | `42f9e1f927faefd8a8c4b3a70787d992c7fa7d63c45a540322dc027a3916de36` |
| `thesis/chapters/10-affine-pentagons.tex` | `b29918db7ffa205d1fd6cf6838a241f06ae9bde0b2bd44bd7ec468d520fdd486` |
| `thesis/chapters/11-product-position.tex` | `01b91794a8fb1764a39b193fd4f93533b7e7025425dd54f196c0c5867a4a3f06` |

Supporting inspection covered the preliminary capacity/continuity proposition, the Chapter 4 normalization and word convention, the affine research note's corresponding finite/equality arguments and source contract, and the product-rotation proof/checker. Primary-source checks used local HK Theorem 1.1 and HKO proposition text and their arXiv versions. No PDF build/render, production capacity solver, numerical sweep, or historical Sage enumeration was run.

## Disposition of the proof bridges

### P1 — Ordered pairings and endpoint normalization

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/09-rotated-regular-polygons.tex:86`–`98`, `:159`–`189`; `thesis/chapters/04-quadratic-program.tex:25`–`30`, `:59`–`67`.
- The four displayed mixed pairings have signs `+,-,+,+` in the stated earlier-entry-first order, giving `2 u·v`. The support normalization is already absorbed in the displayed `u=n_3/[2(1+h)]`; no additional `h^-2` belongs in that grouped formula. The resulting `Q=cos(theta)/[2(1+h)^2]` agrees with `c=1/(2 Qmax)`. The HKO endpoint import correctly changes the paper's `dp∧dq` convention by reflecting the p-factor and reduces `pi/2` to `pi/10`.
- **Concrete proposed fix:** none required; retain the four ordered pairings and the convention-change paragraph together with the explicit witness.

### P2 — Endpoint interpolation supplies a bound, and the same witness supplies sharpness

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/09-rotated-regular-polygons.tex:146`–`154`, `:201`–`245`, `:250`–`265`.
- Closure is independent of angle. Each fixed feasible choice has the form `A cos(theta)+B sin(theta)`, and the displayed interpolation coefficients are nonnegative on the full closed interval. Their sum is `cos(theta)/cos(pi/10)`. Inserting the endpoint maxima gives the stated upper bound on every Q. Crucially, the word from P1 attains that bound throughout the interval; endpoint equality alone is not being used as an attainment argument. Symmetry and the area calculation then give the claimed ratio and its maximizing angles.
- **Concrete proposed fix:** none required; preserve both the interpolation inequality and the common attaining word. Do not replace the latter by a statement that endpoint agreement forces interior equality.

### P3 — Vertex replacement, factor mass and cyclic alternation

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/10-affine-pentagons.tex:65`–`111`.
- Extending a maximizing word by zero-weight facets permits optimization on the full factor closure polytopes. Successive linear maximizations preserve a positive global optimum and can choose vertices in both factors. A two-sided dependence perturbation rules out support greater than three. Positivity then gives mass `t=1/2`. Cyclic invariance follows from closure; after grouping adjacent same-factor entries, there are two, four or six groups. The two-group value is zero. The prefix calculation gives `Q=(x_1·y_1-x_3·y_2)/2` with both half-masses included.
- **Concrete proposed fix:** none required; retain “some maximizer” at line 87 and the distinction between normalized factor steps and actual product weights at lines 98–101. These arguments establish existence, not sparsity of every optimizer.

### P4 — The five closure triples and the four-group hull

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/10-affine-pentagons.tex:130`–`156`.
- Positive barycentric closure puts the origin inside the triangle of three distinct circular normals. A gap at least pi would put that triangle in a closed half-plane through the origin, a contradiction. Thus the integer gaps are cyclic `(1,2,2)`, producing exactly five triples. The displayed weights and steps close correctly. Every proper grouped sum is a step or its negative, whose combined hull is `sD`. Expanding independent convex combinations yields membership of every four-group matrix in `rho mathcal D`; the converse generators have actual grouped-word realizations.
- **Concrete proposed fix:** none required; keep the barycentric/half-plane justification, bilinear expansion, and generator realization. An inclusion based only on step hulls would be insufficient without the displayed expansion and converse.

### P5 — Exact tensor identities exhaust the six-group cases and survive affine changes

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/10-affine-pentagons.tex:158`–`208`, `:117`–`121`, `:213`–`219`.
- Both tensor identities hold exactly. Their unsigned coefficients have totals `2 lambda^2+lambda^3=1` and `2 lambda^2<1`; the latter uses the available zero matrix for its remaining probability mass. The cyclic cut defines `x_1=se`, `x_3=sw`, `y_1=sv`, `y_2=sz` before the six-case table. All six entries check; left reflection handles the other q-order, while independent rotations handle every pair of the five closure triples. The same linear map `T -> G^-T T H^-1` acts on both matrix families, so full tensor containment, rather than angular evidence, justifies arbitrary nonsingular independent factor maps.
- **Concrete proposed fix:** none required; preserve the explicit cyclic-cut definitions, signed-generator interpretation and affine covariance. Keep the qualification that six-group words can tie the maximum.

### P6 — Sharp ratio and complete equality class within the affine family

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/10-affine-pentagons.tex:221`–`253`, `:256`–`272`, `:284`–`289`.
- The twenty-five absolute pairings are equivalent to `LD subset D°`. Adjacent supporting lines give `D°=sec(alpha) R_alpha D`, so area containment bounds `|det L|`. Equal area forces equality of these full-dimensional convex bodies. The induced automorphism permutes vertices and preserves or reverses adjacency, and agrees with an orthogonal decagon symmetry on two independent adjacent vertices. This proves precisely `M=t R_alpha U`, including orientation-reversing maps. The displayed symplectic transformation to `P × M^-T P`, reciprocal rescaling, `UP=P` or `-P`, and angle congruences correctly identify the HKO equality bodies. The conclusion remains restricted to affinely regular factors.
- **Concrete proposed fix:** none required; retain the adjacent-vertex argument and existing symplectic map/angle calculation. No additional equality-case theorem for arbitrary pentagons or arbitrary ten-facet bodies follows.

### P7 — Product-position mass bound, signs, squeeze and actual rigid rotation

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/11-product-position.tex:50`–`106`, `:147`–`157`.
- Reversing the planar B-order converts its `-J_2` contribution to the planar `J_2` maximum, giving the correct `(1-t)^2/b` bound. Rescaling the factor weights to masses one half gives the mixed denominator `4t(1-t)`, hence the bound `4t(1-t)/ell`. The diagonal and off-diagonal blocks have the stated nonnegative cos/sin coefficients. After subtracting `U`, the two endpoint values of the affine inner bracket are `V-U` and `W-2U`; both are nonpositive under `tan(theta)<=ell/(2a)`. The all-A competitor gives the other side of the reciprocal squeeze. The skew generator and its square verify the orthogonal conjugation, identifying a real rotation through angle magnitude theta/2.
- **Concrete proposed fix:** none required; retain the reversed order, mixed normalization and explicit two-sided squeeze. Describe the interval as sufficient; this proof does not identify the first loss of optimality of the one-factor branch.

### P8 — Hausdorff approximation closes the nonpolygonal and boundary cases

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/11-product-position.tex:108`–`144`; `thesis/chapters/02-preliminaries-ehz-capacity.tex:99`–`126`.
- Translation supplies positive inradii. Uniform support-function convergence gives the common `(1±epsilon_m)` factor inclusions, hence product inclusions, area convergence, and the quadratic capacity sandwich for every fixed theta. In particular `ell_m -> ell>0`. The unconditioned reciprocal bounds pass to the limit because their polynomials converge uniformly on `[0,1]`, and therefore so do their maxima. Only then is the threshold imposed. This handles its equality boundary without needing any approximating pair to satisfy that boundary condition. The continuity result is actually proved locally from monotonicity and homogeneity, so the bridge does not depend on an absent external continuity citation.
- **Concrete proposed fix:** none required; preserve the order “pass both bounds to the limit, then impose the threshold” and the reference to the preliminary sandwich proof.

### P9 — Triangle endpoint and exact/numerical evidence remain distinct

- **Status:** solved. **Severity:** none.
- **Locations:** `thesis/chapters/11-product-position.tex:159`–`187`; `thesis/chapters/09-rotated-regular-polygons.tex:27`–`42`; `thesis/chapters/10-affine-pentagons.tex:274`–`282`.
- The rational triangle witness has raw cross-pairing sum 5 and gives `ell<=18/5<9/2`, which is sufficient for the claimed endpoint comparison. The manuscript correctly states an upper bound rather than silently inferring an exact endpoint from one feasible word. The separate checker establishes the exact endpoint by complete triangle-order bounds, but the thesis does not need that stronger assertion. Sampled blue rotation markers are explicitly numerical; the analytic proofs do not consume either the sampled curve or the historical Sage enumeration. For the interior-maximum remark, continuity in theta follows from the explicit continuous rotation of a fixed compact body and the preliminary continuity proposition.
- **Concrete proposed fix:** none required; retain the inequality in the triangle example and the numerical/proof distinction in the figure caption and provenance paragraph.

## Candidate objections withdrawn

### W1 — “The affine theorem must exclude all six-group maximizers”

- **Status:** withdrawn. **Severity:** none.
- **Locations:** `thesis/chapters/10-affine-pentagons.tex:87`–`88`, `:218`–`219`.
- This would demand a stronger classification than the theorem needs and is not what the text asserts. Convex containment excludes a strictly greater six-group value; ties are expressly allowed.
- **Concrete proposed fix:** no change; withdraw any request to replace the sparse-existence conclusion by a classification of every optimizing word.

### W2 — “The limiting proof needs approximants satisfying the final threshold (and a_m<=b_m)”

- **Status:** withdrawn. **Severity:** none.
- **Locations:** `thesis/chapters/11-product-position.tex:59`–`90`, `:133`–`140`.
- The raw mass bound and all-A lower bound hold before using either the threshold or the area ordering. Both pass to the limit. The final algebra uses only the limiting `a<=b` and limiting threshold, so equality of areas or a reversed approximate ordering causes no gap.
- **Concrete proposed fix:** no change; retain passage of the unconditioned inequalities. Do not attempt to pass only the conditional polygon equality at the threshold.

## Fresh checks and their limits

1. Read and ran `python3 -B topics/products/product-rotation/verify.py`. It passed the rational support/closure and witness checks, all six prefix bounds, all 120 cyclic triangle orders, and the integer generator identities. These support the finite triangle example and conjugation algebra; they do not prove the arbitrary-factor theorem.
2. Independently recomputed the pentagon tensor identities, all six table entries and their reflected counterparts, five positive gap triples, raw six-group prefix tensor, and both scalar ratio constants using `Fraction` coefficients in the exact field with `lambda^2+lambda=1` and `z^2=(3+lambda)/4`, `z=sin(2pi/5)`. All final checks passed. A purely combinatorial enumeration of the 120 cyclic orders of six labelled facets gave 36 two-group, 72 four-group, and 12 six-group orders. This enumeration checks the local group classification; it is not a rerun of the historical 3,340-order Sage certificate.
3. During checker construction, an initial reflection assertion incorrectly reflected the p-steps as well as applying a left q-reflection. Correcting the audit checker to act on the left factor alone resolved it; there was no manuscript discrepancy. SymPy was unavailable; no packages were installed.
4. The HK prefactor, support-weight substitution, distinct-word equivalence and reversal convention match local primary source `topics/literature/sources/hk2017/EHZ-polytopes.tex:66`–`80` and [HK Theorem 1.1](https://arxiv.org/html/1712.03494v3#Thmtheorem1). HKO's endpoint and opposite form convention match `topics/literature/sources/hko2024/counterexample.tex:85`, `:165`–`178` and [HKO v3 Proposition 1.4](https://arxiv.org/html/2405.16513v3). The older v2 numbered this Proposition 1.3; this does not invalidate the thesis's current 1.4 citation.

The exact field calculations check the displayed finite algebra. The analytic vertex, convexity, equality and limiting implications were reviewed separately above. Neither kind of check certifies production implementation coverage, floating-point samples, all later product-family research, or human acceptance.

[review: /root/message_review]
