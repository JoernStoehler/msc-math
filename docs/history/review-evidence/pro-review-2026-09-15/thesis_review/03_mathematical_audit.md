# Mathematical audit and replacement arguments

This file distinguishes the thesis’s arguments from calculations and proofs supplied during this review. Locations refer to the supplied 86-page PDF. External sources E1–E6 are identified in `04_sources_and_scope.md`.

## 1. Main result: the exact HKO witness survives independent reconstruction

**I found no mathematical refutation of Theorem 7.1.** More positively, I independently reconstructed the printed 26-section certificate using exact arithmetic in SymPy’s algebraic number field

\[
\mathbb Q(t),\qquad t^4-10t^2+5=0,\quad t=\sqrt{5-2\sqrt5}\in(0,1).
\]

This did not execute the thesis’s Sage program. The assignment data were recovered from Appendix C, and the derivative calculation was independently organized through the stationary-value identity rather than copied from the printed feasible-section differentiation code.

### What was checked exactly

For every one of the 26 assignments, the selected five-column closure minor is invertible, the reconstructed weights satisfy closure and normalization, every weight is strictly positive, and the quadratic value equals the known HKO value. The base weights also satisfy the equality-constrained stationarity equations. Using independently constructed symplectic Lie algebra generators, the resulting derivative matrix annihilates all fifteen symmetry columns. Its rank is 25; the symmetry-column rank is 15; and its one-dimensional left kernel contains a vector with all 26 entries strictly positive. Normalizing that vector gives an exact relation with sum one.

The important distinction is that **the ranks, signs and identities were checked exactly**. The following decimals are only readable diagnostics:

| Diagnostic | Reconstructed value |
|---|---:|
| Smallest weight among the 26 assignments | approximately 0.01790666645 |
| Smallest normalized positive-relation coefficient | approximately 0.002186171686 |
| Largest normalized positive-relation coefficient | approximately 0.08029271605 |
| Worked derivative in §7.3, entry 2, facet 2, momentum coordinate 1 | approximately −0.18017073246472 |

The last number agrees with the printed −0.1801707325. The full normalized coefficient vector and verification output are in `verification/`.

### Why the independent derivative route is valid

At the base point, write

\[
H_\sigma\beta=C_\sigma^T\lambda,
\qquad C_\sigma\beta=e.
\]

For any differentiable feasible section through that point, differentiating feasibility gives

\[
C_\sigma D\beta=-(DC_\sigma)\beta.
\]

Consequently

\[
Dq[h]=D_aQ[h]-\langle\lambda,(D_aC[h])\beta\rangle.
\]

This identity uses stationarity **at the base point** and feasibility of the chosen section. It does not require an invertible KKT matrix or a smoothly continuing optimizer. It is therefore valid for the singular seven-facet sections. With the independently reconstructed volume derivative, differentiation of

\[
U=\frac{1}{2V}\left(\frac{1}{2q}\right)^2
\]

gives the required 26 rows.

### What arithmetic alone does not prove

These computations verify the finite content of Lemma 7.4, conditional on the known HKO capacity and the identification of the displayed rows with the specified body. They do not replace the geometric argument. I also checked the following logical links in the text:

* A sufficiently close ten-facet body can be labelled by ten polar vertices close to the original ones. The vertex-count argument is essential; it would not cover arbitrary facet creation.
* Simplicity and strict slack give the stable local face structure and smooth volume used by the proof.
* A positive feasible weight section gives an upper bound, even when it is not a nearby optimizing branch.
* Rank 25, annihilation of the 15-dimensional symmetry space and a strictly positive relation imply a negative lower-envelope slope in every nonzero transverse direction.
* Compactness of the transverse unit sphere and a common first-order remainder estimate give a genuine neighborhood, not merely separately chosen radii along rays.
* The inverse function theorem for the group action and a complementary slice transports the strict slice statement to nearby bodies and yields the equality characterization.

These links are adequate as written. The absence of an explicit numerical neighborhood radius is **not** a gap in an existential local-maximality theorem. Nor is a Hessian test needed: the proof uses a nonsmooth, linearly decreasing envelope of upper bounds.

There remains ordinary reliance on the exact-arithmetic implementation used for this review, as there is on Sage in the thesis. This is independent corroboration, not a formally verified proof assistant development or an exhaustive audit of the whole software repository.

## 2. An analytic replacement for the rotated-pentagon certificate

**Reviewer-supplied proof.** This replaces the enumeration-dependent part of Theorem 9.1; it does not assume that theorem’s conclusion. It uses the independently known HKO capacity, the Haim–Kislev formula and the explicit feasible word already displayed on pp. 59–60.

Set

\[
a=\frac{\pi}{10},\qquad b=\frac{\pi}{5},\qquad
D=(1+\cos b)^2,
\qquad K_\theta=P_5\times_L R(\theta)P_5.
\]

We prove

\[
c_{\rm EHZ}(K_\theta)=D\sec\theta
\quad\text{for }-a\leq\theta\leq a.
\]

The symmetries already established in the thesis then give the complete profile.

### Step 1: the feasible set does not depend on the relative angle

Write the factor rows as \((u_i,0)\) and \((0,R(\theta)v_j)\). For a fixed word, closure is equivalent to

\[
\sum_i\beta_i^q u_i=0,
\qquad
R(\theta)\sum_j\beta_j^p v_j=0.
\]

Since the rotation is invertible, these constraints, together with nonnegativity and total weight one, are independent of \(\theta\). We may therefore compare **the same feasible weight vector and word** at different angles. This is the key point that makes the following interpolation legitimate.

### Step 2: each candidate is a first harmonic

Same-factor symplectic pairings vanish. Each mixed pairing is a scalar product involving \(R(\theta)\), hence is linear in \(\cos\theta\) and \(\sin\theta\). Thus every fixed feasible candidate has

\[
Q_{\sigma,\beta}(\theta)=A_{\sigma,\beta}\cos\theta
+B_{\sigma,\beta}\sin\theta.
\]

There is no stationarity or nondegeneracy assumption here.

### Step 3: both endpoint maxima are already known

The angle \(-\pi/2\) of the HKO body is congruent to \(-a\) modulo the pentagon’s \(2\pi/5\) rotational symmetry. Simultaneous reflection of both factors identifies the capacities at \(-a\) and \(+a\). Therefore the independently known HKO capacity gives

\[
Q_{\max}(-a)=Q_{\max}(a)=q_*:=\frac1{2c_{\rm HKO}}.
\]

Every feasible candidate is consequently at most \(q_*\) at both endpoints.

### Step 4: positive interpolation bounds every candidate at once

For \(-a\leq\theta\leq a\), the elementary identity

\[
Q_{\sigma,\beta}(\theta)
=
\frac{\sin(a-\theta)}{\sin(2a)}Q_{\sigma,\beta}(-a)
+
\frac{\sin(a+\theta)}{\sin(2a)}Q_{\sigma,\beta}(a)
\]

has nonnegative coefficients. Their sum is \(\cos\theta/\cos a\). Hence

\[
Q_{\sigma,\beta}(\theta)
\leq q_*\frac{\cos\theta}{\cos a}.
\]

Taking the maximum over all words and all feasible weights preserves this bound. Using the HKO value printed on p. 40,

\[
c_{\rm HKO}=2\cos a(1+\cos b),
\]

and \(2\cos^2a=1+\cos b\), we obtain

\[
c_{\rm HKO}\cos a=D,
\qquad
Q_{\max}(\theta)\leq\frac{\cos\theta}{2D}.
\]

### Step 5: the thesis’s displayed word attains the bound

For the constant positive weights associated with \(\sigma_{\rm act}=(3,8,1,0,5,6)\), the calculation on p. 59 gives

\[
Q_{\rm act}(\theta)=\frac{\cos\theta}{2D}.
\]

This candidate is feasible throughout the interval. Thus it supplies the reverse inequality for \(Q_{\max}\), and

\[
Q_{\max}(\theta)=\frac{\cos\theta}{2D},
\qquad
c_{\rm EHZ}(K_\theta)=D\sec\theta.
\]

Since the volume is constant, the thesis’s elementary area calculation yields

\[
\operatorname{sys}(K_\theta)
=\frac{5+2\sqrt5}{10}\sec^2\theta
\quad(-a\leq\theta\leq a).
\]

The previously established reflection, factor-exchange and pentagon symmetries yield the formula with \(d(\theta)\) for every real \(\theta\). This proof includes the endpoints directly. There are no exceptional interior parameters to recover by continuity, and no singular stationary systems to classify. ∎

### What this changes in the thesis

This is not merely a faster implementation of the same certificate. It changes the mathematical explanation: the angle-independent feasible set and its harmonic objectives control all competitors simultaneously. The long computer-assisted proof can become a short structural argument, with the computational work retained as discovery history or a test of the algorithms.

A related optional observation follows from Theorem 4.6. For any two fixed planar polygons, the closure-vertex weights are independent of relative rotation, and there are finitely many support/order candidates. Thus \(Q_{\max}(\theta)\) is the support function of a finite coefficient polygon evaluated at \((\cos\theta,\sin\theta)\). It is a finite upper envelope of first harmonics. This could provide a clearer mathematical explanation of rotation profiles than a collection of rational KKT branches. Developing this general observation further is an optional enrichment, not a requirement for passing a master’s thesis.

## 3. The missing bridge between the empirical descriptor and the exact profile

**Reviewer-supplied proposition.** For the regular-pentagon family of §9, with \(S\) defined in §8.2,

\[
\boxed{S(K_\theta)=16\sin(\pi/5)\cos d(\theta)}
\]

and therefore

\[
\boxed{\operatorname{sys}(K_\theta)\,S(K_\theta)^2
=16(3+\sqrt5).}
\]

This is an exact restricted-family relation on both sides of \(\operatorname{sys}=1\). It is not a universal law for polytopes.

### Proof

Let \(b=\pi/5\). The edge length of a circumradius-one regular pentagon is \(\ell=2\sin b\), and its area is

\[
B=5\sin b\cos b.
\]

The edge directions are equally spaced by \(2\pi/5\). Among the 25 pairs of edges from the two factors, each difference of edge-direction indices occurs five times. The exact product-ridge formula on p. 51 therefore gives

\[
S(K_\theta)
=\frac{5\ell^2}{B}\sum_{k=0}^4
\left|\cos\left(\theta+\frac{2\pi k}{5}\right)\right|
=4\tan b\sum_{k=0}^4
\left|\cos\left(\theta+\frac{2\pi k}{5}\right)\right|.
\]

On \(-\pi/10\leq\theta\leq\pi/10\), the signs of the five cosine terms are \((+,+,-,-,+)\), allowing zeros at endpoints. Pairing the symmetric terms gives

\[
\sum_{k=0}^4\left|\cos\left(\theta+\frac{2\pi k}{5}\right)\right|
=
\left(1+2\cos\frac{2\pi}{5}-2\cos\frac{4\pi}{5}\right)\cos\theta
=(1+\sqrt5)\cos\theta.
\]

Since \(1+\sqrt5=4\cos b\), this is precisely

\[
S(K_\theta)=16\sin b\cos\theta.
\]

The absolute-cosine sum is even and has period \(\pi/5\), so the formula extends with \(\theta\) replaced by \(d(\theta)\). Multiplying its square by the proven systolic profile and using

\[
\sin^2(\pi/5)=\frac{5-\sqrt5}{8}
\]

gives

\[
256\frac{5-\sqrt5}{8}\frac{5+2\sqrt5}{10}
=16(3+\sqrt5).
\]

This proves both identities. ∎

At HKO, the formula gives \(S=4\sqrt5\), agreeing with the reported 8.944272. At aligned pentagons it gives approximately 9.404564. The systolic ratio exceeds one exactly when

\[
d(\theta)>\arccos\sqrt{\frac{5+2\sqrt5}{10}}
\approx13.2825256^\circ,
\]

within a half-period ending at \(18^\circ\). Thus an above-one structured test set is readily available; it is not necessary to wait for random sampling to discover a previously unknown counterexample before examining the descriptor there.

The scientific conclusion must remain narrow: this proves an inverse-square law for one rotation family. It neither explains the pooled rank correlation for the original random distribution nor rescues universal monotonicity, which the thesis correctly disproves using the cube.

## 4. The finite capacity formula can be derived directly from the preceding chapters

This is the bounded repair for R20, not a correction to the value of the formula.

Let \(c=c_{\rm EHZ}(K)\). For a feasible word/weight pair with \(q=Q_\sigma(\beta)>0\), construct the pure-velocity closed loop with period \(T=1/(2q)\), as in Proposition 4.2. The closure equation, shoelace identity and pure-velocity support values give

\[
\mathcal A(z)=I_K(z)=T.
\]

It is dual feasible, so \(c\leq T\), or \(q\leq1/(2c)\). This establishes an upper bound for every positive candidate Q-value.

Conversely, Theorem 3.4 provides a simple minimizing generalized Reeb orbit of period \(c\). Normalize its dwell times by \(c\). The resulting feasible weights have

\[
Q_\sigma(\beta)=\frac1{2c}>0.
\]

The global maximum is therefore positive and equal to \(1/(2c)\). This proves Theorem 4.1 from the dual principle and simple-minimizer theorem already developed, instead of relying on a second import of the whole finite formula.

## 5. Other mathematical checks and their limits

| Portion of thesis | Review outcome |
|---|---|
| §2.1–2.3: symplectic signs, primitive, action and contact normalization | The convention is internally consistent. In particular, \(R_i=2J_0a_i\) has \(\lambda_0(R_i)=1\) on its facet. No sign/factor-of-two error identified. |
| Theorem 2.6 and reconstruction | The conjugate \(H_K^*=h_K^2/4\), multiplier identity and simultaneous space/time rescaling are consistent. Existence and the nonsmooth multiplier theorem remain imported results, as acknowledged. |
| Lemmas 3.1–3.9 and Theorem 3.4 | The action formula, base-point recovery and dual splitting/merging/rescaling argument are coherent. The proof is an existence result for a simple minimizer, not a classification of all minimizing orbits. |
| §4.2: singular stationarity systems | The proof that Q is constant on an affine stationary solution set is correct. Positive feasibility must still be decided; this distinction matters for implementation. |
| Theorem 4.4 | The cited final Rudolf theorem covers nonsmooth convex factors and at most three strong-billiard bounce points in the planar case. The thesis’s reversal of convention and recovery after dual surgery are appropriate. No missing strict-convexity hypothesis was found. |
| Theorem 4.6 | The bilinear argument and vertex support bound are sound. This is one of the clearest mathematical pieces in the thesis. It does not claim every minimizer has at most six facets. |
| Theorem 5.4 and Corollary 5.5 | Completeness follows on the expressly stated regular class from a simple minimizer and tube semantics. It is not a theorem for arbitrary nongeneric input or the full CH Type 1/2/3 model. |
| Proposition 5.6 | Independently computed exact return-map determinants \(-10643/600\) for ABCDE and \(1/168\) for ACDE, and checked several AB-repeat specializations. The polynomial nonvanishing argument does not require those repeated-row specializations to be irredundant geometric inputs. |
| §6 and volume calculations in §7 | The row-volume derivative, feasible-section bounds, envelope derivative and conditional finite-minimum derivative are consistent. The finite coverage assumption in §6.3 is important and is explicitly stated. |
| Theorem 7.1 | Geometric proof reviewed; finite witness independently reconstructed exactly as above. No counterexample or fatal proof gap identified. |
| §8.4: ridge formula and cube example | The mixed ridge area, edge-width identity and cube comparison are correct. The missed issue is their integration with §9, not a wrong formula. |
| Theorem 9.1 | Analytic proof supplied above. The full Sage classification was not rerun. The generic singular-classifier concern is isolated as Q01, not asserted to invalidate the theorem. |
| Lemma 11.1 | The inverse-defect estimate is correct with its stated outward bounds. Turning it into certified machine arithmetic requires the additional implementation details identified in R10. |
| §11.3 curvature pruning | A positive tangent-curvature witness extends by zeros to an order-preserving superword and excludes an interior fixed-word maximum. I found no flaw in this mathematical pruning argument. I did not audit every corresponding Rust implementation path. |

### Classifier inspection: why the uncertainty is specifically bounded

The inspected `classify_sigma` implementation contains the control-flow issue described in Q01. It should not be promoted into a finding that the thesis’s 410 positive-gap classifications are wrong. An independent numerical enumeration reproduced the 50,400 raw representatives and 3,340 distinct pruned words at the interior angle \(\pi/20\). Among them, 470 systems were consistent and numerically singular; all had numerically zero stationary quadratic value. Twenty-five systems were inconsistent at that angle. This is consistent with the reported special outcomes and suggests that the problematic positive-Q singular success path may never be used here.

That diagnostic is not an exact generic-rank/identity proof. A rigorous guard or exact run-level assertion is the right repair if the classifier is retained. The analytic argument above avoids relying on the question at all.

## 6. Criticisms deliberately not made

I have **not** treated any of the following as mistakes: using exact rational choices that happen to resemble rounded decimals; using singular KKT data to construct smooth feasible sections; proving local maximality only with exactly ten facets; using 26 upper bounds rather than classifying all minimizers; leaving the neighborhood radius existential; using finite computation in a mathematical proof; failing to discover a new above-one body by random search; distinguishing dyadic input from unrounded source geometry; or reporting negative experiments with appropriately limited conclusions.

I also have not inferred plagiarism, fabricated data, prohibited AI use, or lack of personal understanding from the disclosure. Those would require evidence not available here. The review’s concerns about final verification and examination responsibility are narrower and explicit.
