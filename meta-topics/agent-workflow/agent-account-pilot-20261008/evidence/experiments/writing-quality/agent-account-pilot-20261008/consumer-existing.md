# Closed-input response: existing account

The account proves a fixed-regular-pentagon capacity formula for every centrally symmetric planar partner. It calibrates a regular decagon using a triangle-covering theorem, then combines the universal two-bounce upper bound with containment of a scaled decagon. An area comparison gives the sharp symmetric ratio. Its asymmetric consequences use central symmetrization; they do not extend the equality formula to every partner.

**1. Exact theorem and freedoms.** In the opening definitions and **Claim**, Q is a regular pentagon of circumradius one, with outward normals n_i=(cos(2πi/5),sin(2πi/5)) and facet height h=cos(π/5). Set D=Q−Q, w=1+h, B=(1/2)D^polar=conv{±n_i/(2w)}, and W(K)=max{r≥0:rD^polar⊆K−K}. For every centrally symmetric planar convex body K with nonempty interior,
c_EHZ(K×_L Q)=W(K).

The sharp ratio c²/[2 area(K) area(Q)] is (10+6√5)/25≈0.9366563146. Equality holds exactly for translates and positive dilates of B in its specified orientation relative to Q. Partners may have arbitrary symmetric shapes, including smooth boundaries; polygon size is unrestricted. Translation and positive scaling are permitted. An arbitrary rotation or affine image of B is not included in the stated equality class.

**2. Calibration and extension.** **Proof**, first three paragraphs: the imported Balitskiy–Mitrofanov–Polyanskii Theorem 3.3 and §6.1 identify ten normal unit-Q-perimeter triangles whose translational fit is equivalent to capacity ≥1. Their angles are π/5, π/5, 3π/5. With R=1/[2(1+h)], unit perimeter forces the longest edge to have length 2R. Its centered endpoints are antipodal decagon vertices. The third vertex has distance R tan(π/5) from zero, strictly below B's incircle radius R cos(π/10), by the displayed exact trigonometric inequality. Convexity then gives all ten fits and c(B×Q)≥1.

**Proof**, “Next”: two-bounce segments give c(K×Q)≤W(K) for every K. A segment of displacement v has Q-length h_D(v), and translates into K exactly when v∈K−K. The imported Theorem 2.5 covering characterization supplies the capacity implication. Since B−B=D^polar, W(B)=1, completing the calibration c(B×Q)=1.

For centered symmetric K, K−K=2K yields W(K)B⊆K. Monotonicity and one-factor homogeneity give the reverse capacity inequality. These capacity properties are invoked, not established here. The final **Proof** paragraphs derive the area bound and equality class. W's identification with the repository's stronger billiard-class A2 is expressly unnecessary to this argument. Imported source theorems remain unverified in this closed-input pass.

**3. Asymmetry and paths.** **Interpretation and follow-up consequences** supplies S=(K−K)/2 and W(K)=c(S×Q), hence c(K×Q)≤c(S×Q). The quotient c(K×Q)/W(K) measures retained symmetrized capacity within the chosen factors.

For K_t=(1−t)K+t(−K), W stays fixed. The account asserts capacity concavity and symmetry about 1/2, and proves midpoint attainment from the symmetric theorem. Concavity's underlying theorem is not identified or proved here. Given these assertions and the ceiling, attainment at t₀≤1/2 forces attainment throughout [t₀,1−t₀]. Early plateau locations and their geometric dependence are empirical. Strict asymmetric gaps, including regular-triangle and irregular partners, are reported collection observations. Global optimality of HKO is unresolved.

With δ=area(K−K)/[4 area(K)]−1, the account derives sys(K×Q)≤κ_sym(1+δ). Thus sys>1 requires δ>1/κ_sym−1≈0.06762746. Sufficiency and sharpness are explicitly unclaimed.

**4. Computational checks.** The opening identifies `pentagon_partner_research/symmetric_partner_check` as an independent check but supplies neither its procedure nor outputs. **Literature and numerical boundaries** identifies `universal_symmetric_reference` in `symmetrization-v1.jsonl`, retaining B's floating-point geometry, capacity, W, and ten fits. Reported c≈0.9999999999999996 and W≈0.9999999999999998 check implementation consistency. They are not exact certificates or mathematical proof. Full artifact paths and reproduction commands are absent.

**5. Figure.** A candidate figure, its scale, and traversal convention are absent. The proof's useful geometric features are the diameter endpoints, normal-aligned edges, and strict incircle containment of the third vertex. Their depicted appearance cannot replace the perimeter and containment arguments. Any inferred drawing would be my reconstruction, not supplied material.

**6. Reader-facing status.** No reader-facing candidate location, selected-thesis interface, build result, or integration status is supplied. The opening dates the agent-derived account 2026-09-18 and explicitly says not human-accepted. Literature attribution names the imported source; the closing section reports only a narrow nonduplication check, with publication-level novelty judgment outstanding.

**Proposed edit: reject.** Removing central symmetry asserts the unsupported reverse inequality c(K×Q)≥W(K) for asymmetric K. The proof obtains that inequality through K−K=2K, which requires centered symmetry. The general result is c(K×Q)≤W(K)=c(S×Q), and the account reports strict gaps. Its floating-point reference fixture cannot certify a universal extension.
