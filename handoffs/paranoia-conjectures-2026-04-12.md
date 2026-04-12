# Paranoia: Conjectures + Interpretations Audit

Generated 2026-04-12. Flag-only audit across 62 files
(`logbook.md` + `math.tex` under `crates/`). Companion to the 2026-04-07
numerics paranoia pass (`paranoia-numerics-report.md`).

## Summary
- 62 files scanned (20 exp logbooks + 14 dev logbooks + 28 math.tex)
- 42 flags recorded across 5 claim types
- belief: 5, causal: 11, unhedged: 12, interpretation: 13, conjecture: 1
- self-verify: 5 random flags re-checked by orchestration agent (5/5 matched source)

## How this was produced
- Three parallel inventory subagents on disjoint file sets (A: exp logbooks, B: dev logbooks + library md, C: math.tex)
- Grep-seeded, sentence-read for each hit to filter false positives
- Claim types: conjecture | belief | unhedged | causal | interpretation
- Row schema: file:line / type / verbatim / context
- Orchestration agent aggregated + ranked

## Ranked flags — most embarrassing if wrong → least

> This ranking is an agent heuristic produced without Jörn's thesis-narrative
> calibration. Each rank has a one-line rationale so Jörn can audit the
> reasoning, not just the order. Agent confidence in ordering is low; treat
> this as a starting point for review, not a validated priority list. Absence
> of blatant errors in a rank is NOT evidence the rank is correct — most
> errors would require thesis-narrative knowledge the agent does not have.

1. `crates/library/src/geom/math.tex:163` [unhedged] — `"This follows from the existence of a global area-preserving (hence symplectic) diffeomorphism mapping A to a disk"`
   rationale: Inside \begin{unverified} with explicit GAP comment flagging the entire argument as unverified agent-generated math; "This follows from" asserts a conclusion with no valid proof in the most public-facing file (compiled into crates/main.pdf, read by advisor).

2. `crates/dev-algorithm-comparison/ablation/math.tex:49` [unhedged] — `"which proves the condition is both necessary and sufficient (together with an LP feasibility check on the blocking set)"`
   rationale: "proves" is stated in free prose referring forward to a lemma wrapped in \begin{unverified} — claims proven status for unverified content; dev math.tex is advisor-visible.

3. `crates/dev-gradient/numerics-subdifferential/math.tex:154` [causal] — `"Hence c(a(t)) ≥ …"` (conclusion stated as established; TODO comment acknowledges unresolved gap for strictly-infeasible orbits becoming feasible)
   rationale: Theorem conclusion stated as proved while an in-proof TODO explicitly flags the lower-bound argument has a formal gap; causal chain from feasibility transition to bound is unresolved.

4. `crates/library/src/algorithms/math.tex:918` [unhedged] — `"polytopes form a measure-zero subset: a random polytope is almost surely simple"`
   rationale: "Almost surely" asserts a probabilistic claim in a remark without specifying the probability measure; library/math.tex is the most scrutinized document; only a codimension sketch is given.

5. `crates/exp-hko-local-maximum/second-order/math.tex:154` [unhedged] — `"the first-order decrease from d_perp dominates the second-order behavior along d_parallel"`
   rationale: Inside a labeled proof sketch; the dominance claim stands in for an argument, and the TODO on lines 135–138 explicitly acknowledges the compactness argument needs uniform bounds — the sketch contains a gap where a proof step should be.

6. `crates/exp-hko-local-maximum/second-order/math.tex:179` [interpretation] — `"The LP test confirms 0 ∈ conv{∇sys_i}"`
   rationale: "Confirms" applied to a numerical LP output in the computational-results section, while the proposition it supports (prop:second-order-local-max) has an acknowledged incomplete proof sketch; conflates computational output with proof support.

7. `crates/exp-combinatorial-cells/boundary-characterization/math.tex:168` [interpretation] — `"The experiment confirms continuity empirically … (limited by the finite step-over ε, not by a discontinuity)"`
   rationale: "Confirms continuity" overstates: the proof above establishes only lower semicontinuity; upper semicontinuity is not established, making "continuity" an unproven claim presented as empirically confirmed in a remark environment.

8. `crates/exp-hko-local-maximum/gradient-analysis/math.tex:145` [unhedged] — `"This group is exactly the orbit symmetry group." … "(empirically, all 44 orbits follow this pattern)"`
   rationale: "Exactly" and "is" assert equality as fact in free prose; the justification given is only empirical pattern-matching on 44 orbits, not a proof; outside any proof environment.

9. `crates/exp-sys-landscape/rotated-regular-products/math.tex:26` [unhedged] — `"counterexamples are known to exist"`
   rationale: Plural "counterexamples" and "known to exist" implies a broader established class; only one counterexample (HKO pentagon at θ=18°) has been demonstrated; stated as fact in public introductory prose.

10. `crates/dev-numerical-analysis/unknown-predicates/math.tex:52` [interpretation] — `"The algorithm is empirically exact up to machine-precision rounding … no UNKNOWN predicates influenced the capacity computation in any meaningful way"`
    rationale: Strong interpretive conclusion on a preliminary dataset of 162 polytopes; 29 unverified cases are noted only in the next sentence, making the claim misleadingly broad; "in any meaningful way" is unquantified.

11. `crates/dev-numerical-analysis/unknown-predicates/math.tex:37` [unhedged] — `"attributable to f64 rounding in the billiard capacity computation rather than genuine numerical ambiguity"`
    rationale: Causal attribution between two possible causes based on magnitude alone, without a formal argument; stating "rather than" as a definitive exclusion is unhedged.

12. `crates/dev-numerical-analysis/error-bounds/logbook.md:271` [interpretation] — `"Double-singular adds no error beyond κ(C). The H near-singularity is harmless; only C near-singularity matters."`
    rationale: Exclusive causal claim ("only C … matters") inferred from stress-test data; excludes H singularity without a theoretical argument; the "only" makes this strongly falsifiable and the evidence is a single test family.

13. `crates/dev-numerical-analysis/error-bounds/logbook.md:213` [causal] — `"The quadratic bound fails for well-conditioned problems because the Q error floor is set by floating-point rounding in the Q_raw computation, not by β perturbation."`
    rationale: Causal attribution to rounding vs. β-perturbation asserted without a supporting calculation; the "not by β perturbation" exclusion is unverified.

14. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:262` [interpretation] — `"The lack of interpretable structure supports a narrative that Viterbo's conjecture is 'almost true' with rare combinatorial exceptions."`
    rationale: Load-bearing thesis-narrative claim derived from a rough volume estimate and unstructured boundary data; "supports a narrative" is soft but the claim could shape the thesis conclusion; based on assumptions in a speculative section.

15. `crates/exp-hko-local-maximum/second-order/logbook.md:135` [interpretation] — `"This constitutes strong numerical evidence for negative definiteness of the generalized Hessian on the 15D flat subspace, supporting that HKO2024 is a strict local maximum of sys among F=10 polytopes."`
    rationale: Central thesis claim (HKO2024 strict local max) supported by 15 FD curvatures plus 100 random directions; "strong" is unquantified; the generalized Hessian claim for a non-smooth function is not a standard result.

16. `crates/exp-hko-local-maximum/second-order/math.tex:206` [belief] — `"This does not constitute a proof … but provides strong numerical evidence that HKO2024 is a strict local maximum of sys among F=10 polytopes in R^4."`
    rationale: Key thesis claim stated as "strong numerical evidence"; hedging ("does not constitute a proof") is present but "strong" is unquantified; relies on incomplete proof sketch in same file.

17. `crates/dev-numerical-analysis/error-bounds/logbook.md:390` [interpretation] — `"This is a solver algorithm limitation, not a numerical accuracy issue."`
    rationale: Distinction between "algorithm limitation" and "numerical accuracy" not proven, only inferred from a 5-step mechanism; this classification could affect conclusions about solver reliability.

18. `crates/dev-numerical-analysis/error-bounds/logbook.md:141` [causal] — `"Ill-conditioned C causes large e2e errors and panics."`
    rationale: "Causes" asserts mechanism not just correlation; tested via stress-test families but no formal analysis of the causal chain from κ(C) to error magnitude is given.

19. `crates/dev-numerical-analysis/error-bounds/logbook.md:139` [causal] — `"Tiny eigenvalues of H cause zero extra error. Both solvers handle them fine to machine epsilon."`
    rationale: "Cause zero extra error" is a strong zero-claim mechanism assertion; "fine to machine epsilon" on the stress-test families; no theoretical argument for why H eigenvalues are irrelevant given C.

20. `crates/dev-numerical-analysis/error-bounds/logbook.md:104` [causal] — `"When b_prime = 0, the sign error is invisible."`
    rationale: Causal chain (SVD orthogonality → b_prime = 0 → sign error invisible) asserted without citing a proof step; explains why a bug was missed, but the chain is not formally verified.

21. `crates/dev-numerical-analysis/error-bounds/logbook.md:394` [interpretation] — `"P5 is not a useful conjecture."`
    rationale: Strong dismissal after 15 violations on natural data; "not useful" is a definitive interpretive conclusion that forecloses a direction; may warrant re-examination under different parameterizations.

22. `crates/dev-numerical-analysis/kkt-inertia/logbook.md:44` [interpretation] — `"The inertia formula itself is not violated; the mismatch is a classification artifact."`
    rationale: Distinguishing formula failure from classification artifact relies on the causal claim below; the distinction is interpretive, not proved; affects conclusions about the inertia formula's correctness.

23. `crates/dev-numerical-analysis/kkt-inertia/logbook.md:44` [causal] — `"the threshold-based classifier reports n_-(M) = 5 because M has three eigenvalues at ~1e-16 whose signs cannot be resolved"`
    rationale: Causal explanation for 5 mismatches asserted from eigenvalue magnitudes; the sign-resolution mechanism is plausible but not derived from classifier source analysis.

24. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:260` [unhedged] — `"Random sampling for new counterexamples is quantitatively hopeless." Volume fraction ~10^-31 (rough estimate: (0.035/1.24)^20).`
    rationale: "Quantitatively hopeless" is a strong conclusion from a rough estimate with several geometric assumptions; the assumptions (spherical shape, independence of coordinates, uniformly distributed full LP(5,5) space) are not verified.

25. `crates/exp-sys-landscape/variable-f-ascent/logbook.md:68` [unhedged] — `"F=11 gradient ascent starting from barely-perturbed F=10 local maxima consistently improves sys."`
    rationale: "Consistently" overstates: 43/50 (86%) trials improved, but 2/5 for one source and 3/5 for another; 14% failure rate is not captured by "consistently."

26. `crates/dev-numerical-analysis/error-bounds/logbook.md:386` [causal] — `"Shifted β violates Cβ = d (constraint residual ~0.6) because the eigenvector isn't truly in null(M)"`
    rationale: Root-cause attribution from near-threshold eigenvector to constraint violation is asserted; the causal link from eigenvector near-null to constraint residual could have other explanations.

27. `crates/exp-hko-local-maximum/second-order/logbook.md:97` [unhedged] — `"Still clearly negative, not ambiguous."` (direction 0 curvature −0.018, 17× smaller than largest)
    rationale: "Clearly negative, not ambiguous" for the direction with the largest CV and smallest curvature magnitude; the FD discretization error is not quantified, so "not ambiguous" is asserted rather than demonstrated.

28. `crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:67` [interpretation] — `"The HKO counterexample's Lagrangian orbit ridges are a consequence of its construction as a perturbed Lagrangian product, not a general mechanism."`
    rationale: Asserts construction-as-cause and rules out a general mechanism; the evidence is the fact that HKO has ridge min|omega|=0, but ruling out a general mechanism requires a broader argument not given.

29. `crates/dev-algorithm-comparison/ablation/logbook.md:80` [causal] — `"Bipyramids (F=10): 96-98% reduction … because bipyramid apices lie on 5 facets, creating many vertex-adjacent but infeasible transitions."`
    rationale: Causal explanation for high pruning rate from bipyramid geometry asserted without derivation; plausible but the quantitative link from apex-facet count to pruning percentage is not shown.

30. `crates/exp-sys-landscape/rotated-regular-products/logbook.md:67` [interpretation] — `"Pentagon x pentagon at theta = 18 degrees achieves sys ≈ 1.0472 … confirming the HKO counterexample."`
    rationale: Conflates numerical recomputation (computing sys for a known polygon) with theoretical confirmation of HKO2024's proof; muddles levels of evidence even though the numerical value matches.

31. `crates/exp-sys-landscape/rotated-regular-products/logbook.md:9` [interpretation] — `"Pentagon x pentagon at theta = 18 degrees confirms the HKO counterexample (sys ~ 1.047, lagrangian-products-5x5.jsonl)."`
    rationale: Same conflation as rank 30 but in the status summary line; rank 30 is slightly more prominent (findings section), so this is ranked below it.

32. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:140` [interpretation] — `"The R²≈0 confirms gradient ≈ 0 (this doesn't require C²)."`
    rationale: Using R² from a linear regression as mathematical confirmation that the gradient is approximately zero; R²≈0 is consistent with gradient≈0 but does not confirm it — the R² could also reflect high noise.

33. `crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:65` [causal] — `"The KKT optimizer compensates — it adjusts beta* and selects orbits that maximize Q despite small individual omega_0 contributions. … the optimizer redistributes weight across the orbit's transitions."`
    rationale: Redistribution mechanism asserted without showing it from the data; explains why the omega hypothesis fails, but the compensation mechanism is constructed post-hoc without direct evidence.

34. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:124` [causal] — `"gradient-descent … cannot cross combinatorial boundaries to reach HKO's basin"`
    rationale: Causal barrier (inability to cross combinatorial boundaries) explains the sys≈0.9 ceiling; the mechanism is asserted without demonstrating that starting from HKO's basin region with gradient descent reaches the actual value.

35. `crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:61` [causal] — `"The orbit preferentially uses transitions with large |omega|, consistent with Q-maximization: the optimizer seeks large omega terms to maximize Q."`
    rationale: Causal interpretation of observed higher median |omega| in orbit ridges as Q-maximization mechanism; "consistent with" is somewhat hedged but "the optimizer seeks" is stated as mechanism, not hypothesis.

36. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:248` [belief] — `"Shape likely determined by the combinatorial structure of degenerate minimum-action orbits at HKO2024, not by smooth geometry"`
    rationale: "Likely" hedges appropriately; belief about geometric cause of unstructured boundary shape is speculative but in a logbook context; "not by smooth geometry" is an exclusion claim with no supporting argument.

37. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:254` [belief] — `"Each 'facet' of the sys > 1 boundary likely corresponds to a different orbit becoming cheaper than HKO's optimal orbit."`
    rationale: "Likely" hedges; geometric prediction in a speculative section of a working logbook; consistent with known LP structure but not tested directly.

38. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:119` [unhedged] — `"polygons perturbed by more than ~3% per dual-vertex component almost certainly drop below sys = 1"`
    rationale: "Almost certainly" is informal quantification of the boundary radius; derived from empirical fraction-vs-ε data with geometric assumptions; in a working logbook, so lower public exposure.

39. `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:146` [belief] — `"The 4 positive eigenvalues likely reflect directions where one sheet rises while another (lower) sheet falls — the quadratic form can't represent this."`
    rationale: "Likely" hedges; interpretation of positive Hessian eigenvalues as projection artifacts of a piecewise-smooth surface is speculative but labeled as such; in a working logbook.

40. `crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:57` [belief] — `"this is likely confounded by facet count: more facets produce more ridge pairs (more chances for small |omega| by chance), and facet count itself correlates positively with sys (rho = +0.37)."`
    rationale: "Likely" hedges; statistical confounding hypothesis in a working logbook; the mechanism is internally consistent and the supporting correlation is cited.

41. `crates/exp-sys-landscape/rejection-calibration/math.tex:21` [unhedged] — `"To probe Viterbo's conjecture computationally, we need large datasets of random convex polytopes in R^4."`
    rationale: "We need" frames large random datasets as a factual requirement without argument; low impact — introductory framing in a methodological math.tex, and the approach is broadly standard.

42. `crates/dev-numerical-analysis/error-bounds/logbook.md:462` [conjecture] — `"Empirical conjecture: per-eigendirection β error (2026-04-01)"`
    rationale: Explicitly labeled "Empirical conjecture" and confirmed on 364 problems but not proved; the honest labeling makes this the least embarrassing flag — it is flagged only because the conjecture is open, not because the framing is misleading.

## Full flat inventory (unranked, grouped by file)

---

- file: crates/dev-algorithm-comparison/ablation/logbook.md:80
  type: causal
  verbatim: "Bipyramids (F=10): 96-98% reduction (5-14k -> 213 candidates), because bipyramid apices lie on 5 facets, creating many vertex-adjacent but infeasible transitions."
  context: Explaining why A3 prunes far more candidates than A2 on non-simple polytopes; causal link from geometry to pruning rate asserted without derivation.

- file: crates/dev-algorithm-comparison/ablation/math.tex:49
  type: unhedged
  verbatim: "which proves the condition is both necessary and sufficient\n(together with an LP feasibility check on the blocking set)."
  context: Free prose paragraph before Lemma lem:transition-feasibility; "proves the condition is both necessary and sufficient" is stated in the paragraph text referring forward to a lemma that itself is wrapped in \begin{unverified}.

---

- file: crates/dev-gradient/numerics-subdifferential/math.tex:154
  type: causal
  verbatim: "% [TODO: JÖRN - this argument requires that every orbit sigma\n%  outside R with beta(sigma; a_0) >= 0 has A_sigma(a_0) > c(a_0)\n%  strictly. … This is generically true but not formally proved here.]"
  context: Comment inside a proof (thm:subdiff-with-appearance, Step 5 lower bound); the proof's conclusion "Hence c(a(t)) ≥ …" is stated as established, but the TODO comment acknowledges the lower-bound argument has an unresolved gap for orbits that are strictly infeasible at a_0 but become feasible at a(t).

---

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:104
  type: causal
  verbatim: "The projection solver tests use H = I or simple block structures where b_prime = V^T H β₀ = V^T β₀ = 0 (because β₀ is the min-norm SVD solution, orthogonal to ker(C) = range(V)). When b_prime = 0, the sign error is invisible."
  context: Explaining why the projection solver sign bug was not caught by existing tests; the causal chain from SVD orthogonality to invisibility is asserted without citing a proof step.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:139
  type: causal
  verbatim: "**Tiny eigenvalues of H cause zero extra error.** Both solvers handle them fine to machine epsilon."
  context: Summary claim from stress-test results comparing families by κ(H) and κ(C); "cause" asserts a mechanism, not just correlation.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:141
  type: causal
  verbatim: "**Ill-conditioned C causes large e2e errors and panics.**"
  context: Summary of stress-test finding that κ(C) > 10^8 produces Q errors up to 10^{-1} and 70 panics; "causes" asserts mechanism, not just correlation.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:213
  type: causal
  verbatim: "The quadratic bound fails for well-conditioned problems because the Q error floor is set by floating-point rounding in the Q_raw computation, not by β perturbation."
  context: Explaining why the quadratic bound has max ratio 8.3e13 on well-conditioned problems; the attribution to rounding vs β-perturbation is asserted without a supporting calculation.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:271
  type: interpretation
  verbatim: "**Double-singular adds no error beyond κ(C).** The H near-singularity is harmless; only C near-singularity matters."
  context: Interpreting the double_singular stress-test results (max err 2.0e-1) as showing H singularity is irrelevant; "only C near-singularity matters" is an exclusive causal claim.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:386
  type: causal
  verbatim: "3. Shifted β violates Cβ = d (constraint residual ~0.6) because the eigenvector isn't truly in null(M)"
  context: Root-cause analysis for 9 false-negative β > 0 classifications; causal link from near-threshold eigenvector to constraint violation.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:390
  type: interpretation
  verbatim: "This is a solver algorithm limitation, not a numerical accuracy issue."
  context: Classifying the false-negative root cause as algorithmic rather than numerical; distinction not proven, only inferred from the 5-step mechanism above.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:394
  type: interpretation
  verbatim: "P5 is not a useful conjecture."
  context: Concluding that the conjecture ‖H‖/σ_min(C) ≤ 100 should be dropped after 15 violations on natural data; "not useful" is a strong dismissal that may warrant re-examination.

- file: crates/dev-numerical-analysis/error-bounds/logbook.md:462
  type: conjecture
  verbatim: "### Empirical conjecture: per-eigendirection β error (2026-04-01)"
  context: Open mathematical conjecture |δα_j| ≈ ε_mach / |γ_j| for eigendirection error scaling, labeled empirical and confirmed on 364 I1 problems but not proven.

---

- file: crates/dev-numerical-analysis/kkt-inertia/logbook.md:44
  type: causal
  verbatim: "the threshold-based classifier reports n_-(M) = 5 because M has three eigenvalues at ~1e-16 whose signs cannot be resolved."
  context: Explaining 5 mismatches between inertia-formula prediction and classifier output on hko_pentagon; causal link from sign-resolution failure to mismatch asserted.

- file: crates/dev-numerical-analysis/kkt-inertia/logbook.md:44
  type: interpretation
  verbatim: "The inertia formula itself is not violated; the mismatch is a classification artifact."
  context: Interpreting 5 inertia mismatches as a threshold artifact rather than a formula failure; distinction relies on the causal claim above.

---

- file: crates/dev-numerical-analysis/unknown-predicates/math.tex:37
  type: unhedged
  verbatim: "all below $10^{-10}$, and are attributable to \\texttt{f64} rounding in the\nbilliard capacity computation rather than genuine numerical ambiguity."
  context: Results paragraph, free prose; "attributable to f64 rounding … rather than genuine numerical ambiguity" is a causal claim based on magnitude alone — no formal argument distinguishes the two causes.

- file: crates/dev-numerical-analysis/unknown-predicates/math.tex:52
  type: interpretation
  verbatim: "The algorithm is empirically exact up to machine-precision rounding at\n\\texttt{f64} on our datasets: no UNKNOWN predicates influenced the capacity\ncomputation in any meaningful way."
  context: Conclusion paragraph, free prose; "empirically exact … no UNKNOWN predicates influenced … in any meaningful way" — strong interpretive claim applied to preliminary dataset (162 polytopes), with the caveat about 29 unverified cases noted only in the next sentence.

---

- file: crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:57
  type: belief
  verbatim: "this is likely confounded by facet count: more facets produce more ridge pairs (more chances for small |omega| by chance), and facet count itself correlates positively with sys (rho = +0.37)."
  context: Explaining the weak negative correlation (rho=-0.20) between ridge min|omega| and sys as possibly confounded by facet count.

- file: crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:61
  type: causal
  verbatim: "The orbit preferentially uses transitions with large |omega|, consistent with Q-maximization: the optimizer seeks large omega terms to maximize Q."
  context: Interpreting the empirical observation that orbit ridges have higher median |omega| than non-orbit ridges as explained by the KKT optimizer maximizing Q.

- file: crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:65
  type: causal
  verbatim: "The KKT optimizer compensates — it adjusts beta* and selects orbits that maximize Q despite small individual omega_0 contributions. Small omega_0 values on individual ridges do not translate into small Q (or large capacity) because the optimizer redistributes weight across the orbit's transitions."
  context: Causal explanation for why the near-Lagrangian omega hypothesis fails; the redistribution mechanism is asserted without showing it from the data.

- file: crates/exp-combinatorial-cells/omega-hypothesis/logbook.md:67
  type: interpretation
  verbatim: "The HKO counterexample's Lagrangian orbit ridges are a consequence of its construction as a perturbed Lagrangian product, not a general mechanism."
  context: Interpreting why HKO has ridge min|omega|=0 — asserts construction-as-cause and rules out a general mechanism without direct evidence.

---

- file: crates/exp-combinatorial-cells/boundary-characterization/math.tex:168
  type: interpretation
  verbatim: "The experiment confirms continuity empirically: across 873 boundary crossings,\n$\max |\Delta\operatorname{sys}| = 2.91 \times 10^{-4}$\n(limited by the finite step-over~$\varepsilon$, not by a discontinuity)."
  context: Remark environment (rem:sys-continuous-empirical); "confirms continuity" applied to empirical data — the proof above (prop:sys-continuous) shows only lower semicontinuity via the polytope mechanism, and Approach 2 explicitly does not establish upper semicontinuity without citing general theory.

---

- file: crates/exp-hko-local-maximum/gradient-analysis/math.tex:145
  type: unhedged
  verbatim: "\emph{This group is exactly the orbit symmetry group.}\nEach of the 44 near-optimal orbits visits exactly\n3~$q$-facets and 3~$p$-facets (empirically, all 44 orbits follow this\npattern)."
  context: Free prose (subsubsection, outside any proof); the claim that the group is *exactly* the orbit symmetry group is stated without proof—only empirical pattern-matching is given.

---

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:119
  type: unhedged
  verbatim: "The ~2.8% relative radius means that polygons perturbed by more than ~3% per dual-vertex component almost certainly drop below sys = 1."
  context: Interpreting the characteristic perturbation radius ε*≈0.035 from the fraction-vs-ε curve in the Phase 1 sweep.

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:124
  type: causal
  verbatim: "gradient-descent (random starts, within-cell optimization): reaches sys ≈ 0.9 but cannot cross combinatorial boundaries to reach HKO's basin"
  context: Explaining why gradient-descent results top out near 0.9; the causal barrier (inability to cross combinatorial boundaries) is asserted, not demonstrated.

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:140
  type: interpretation
  verbatim: "The R²≈0 confirms gradient ≈ 0 (this doesn't require C²)."
  context: Using a near-zero R² from a linear regression fit as confirmation that the mathematical gradient is approximately zero at HKO2024.

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:146
  type: belief
  verbatim: "The 4 positive eigenvalues likely reflect directions where one sheet rises while another (lower) sheet falls — the quadratic form can't represent this."
  context: Interpreting the 4 positive fitted Hessian eigenvalues from an anisotropic quadratic model as artifacts of projecting a piecewise-smooth surface.

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:248
  type: belief
  verbatim: "Shape likely determined by the combinatorial structure of degenerate minimum-action orbits at HKO2024, not by smooth geometry"
  context: Interpreting why the directional boundary radius r(u) appears unstructured (R²=0.066 for all 20 components).

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:254
  type: belief
  verbatim: "Each 'facet' of the sys > 1 boundary likely corresponds to a different orbit becoming cheaper than HKO's optimal orbit."
  context: Speculative section: predicting the geometric structure of the sys>1 boundary based on orbit-sheet reasoning.

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:260
  type: unhedged
  verbatim: "**4. Random sampling for new counterexamples is quantitatively hopeless.** Volume fraction of the sys > 1 region in the full LP(5,5) space is ~10⁻³¹ (rough estimate: (0.035/1.24)^20)."
  context: Speculative section: concluding that blind random sampling cannot find new counterexamples based on a rough volume-fraction estimate with several geometric assumptions.

- file: crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:262
  type: interpretation
  verbatim: "The counterexample arises from a specific orbit coincidence (multiple orbits tying at minimum action with the right geometry), not from a geometric principle that would predict families of counterexamples. […] The lack of interpretable structure supports a narrative that Viterbo's conjecture is 'almost true' with rare combinatorial exceptions."
  context: Speculative section: interpreting the unstructured shape of the sys>1 region as evidence that HKO is an accidental exception rather than a systematic one.

---

- file: crates/exp-hko-local-maximum/second-order/logbook.md:97
  type: unhedged
  verbatim: "Direction 0 has the smallest curvature magnitude (−0.018), about 17× smaller than the largest (direction 5, −0.309). Still clearly negative, not ambiguous."
  context: Describing the smallest measured finite-difference curvature and asserting it is unambiguously negative despite having the largest CV and being subject to FD discretization error.

- file: crates/exp-hko-local-maximum/second-order/logbook.md:135
  type: interpretation
  verbatim: "This constitutes strong numerical evidence for negative definiteness of the generalized Hessian on the 15D flat subspace, supporting that HKO2024 is a strict local maximum of sys among F=10 polytopes."
  context: Summary of second-order analysis: interpreting 15 basis-direction FD curvatures plus 100 random-direction curvatures as evidence for strict local maximality.

---

- file: crates/exp-hko-local-maximum/second-order/math.tex:154
  type: unhedged
  verbatim: "For mixed directions: the first-order decrease from $d_\perp$ dominates\n  the second-order behavior along $d_\parallel$."
  context: Inside a labeled "Proof sketch" (prop:second-order-local-max); the dominance claim replaces an argument — the TODO comment on lines 135–138 acknowledges the compactness argument needs uniform bounds.

- file: crates/exp-hko-local-maximum/second-order/math.tex:179
  type: interpretation
  verbatim: "The LP test confirms $0 \in \operatorname{conv}\{\nabla\operatorname{sys}_i\}$"
  context: Computational-results section (outside any proof environment); "confirms" applied to a numerical LP output, but the proposition it supports (prop:second-order-local-max) has an acknowledged incomplete proof sketch.

- file: crates/exp-hko-local-maximum/second-order/math.tex:206
  type: belief
  verbatim: "This does not constitute a proof of\ncondition~(2) for all $d \in \ker(G)$, but provides strong numerical\nevidence that HKO2024 is a strict local maximum of $\operatorname{sys}$\namong $F = 10$ polytopes in $\mathbb{R}^4$."
  context: Free prose in computational-results section; "strong numerical evidence" for a strict local maximum claim that is not proved.

---

- file: crates/exp-sys-landscape/rejection-calibration/math.tex:21
  type: unhedged
  verbatim: "To probe Viterbo's conjecture computationally, we need large datasets of\nrandom convex polytopes in~$\\mathbb{R}^4$."
  context: Free introductory prose; "we need large datasets" is stated as a factual requirement without argument for why large random samples are the right approach for probing the conjecture (minor — but falls under unhedged causal framing).

---

- file: crates/exp-sys-landscape/rotated-regular-products/logbook.md:9
  type: interpretation
  verbatim: "Pentagon x pentagon at theta = 18 degrees confirms the HKO counterexample (sys ~ 1.047, `lagrangian-products-5x5.jsonl`)."
  context: Status line asserting that numerically reproducing sys≈1.047 confirms the HKO theoretical result.

- file: crates/exp-sys-landscape/rotated-regular-products/logbook.md:67
  type: interpretation
  verbatim: "**Pentagon x pentagon at theta = 18 degrees achieves sys ≈ 1.0472** (`lagrangian-products-5x5.jsonl` row 19: sys=1.047214), confirming the HKO counterexample."
  context: Findings section: asserting that a numerical computation confirms the HKO2024 theoretical counterexample.

- file: crates/exp-sys-landscape/rotated-regular-products/math.tex:26
  type: unhedged
  verbatim: "We now probe the conjecture by searching systematically in the space of Lagrangian products, where counterexamples are known to exist."
  context: Free introductory prose; "counterexamples are known to exist" is stated as established fact — the only known counterexample is the single HKO pentagon; the plural and the phrase "are known to exist" implies a broader class than has been demonstrated.

---

- file: crates/exp-sys-landscape/variable-f-ascent/logbook.md:68
  type: unhedged
  verbatim: "F=11 gradient ascent starting from barely-perturbed F=10 local maxima **consistently improves sys**."
  context: Summary of RQ1 results where 43/50 (86%) trials improved, including 2/5 for one source and 3/5 for another — the word "consistently" overstates the uniformity.

---

- file: crates/library/src/algorithms/math.tex:918
  type: unhedged
  verbatim: "polytopes form a measure-zero subset: a random polytope is almost surely simple."
  context: Remark (rem:simplicity-generic), outside any proof, approved by Jörn; "almost surely" is used to assert a probabilistic claim about random polytopes without specifying the probability measure, though the codimension argument is sketched.

---

- file: crates/library/src/geom/math.tex:163
  type: unhedged
  verbatim: "We first show $c_{\\mathrm{EHZ}}(A) = \\operatorname{area}(A)$\nfor every convex body~$A$ in~$\\mathbb{R}^2$.\nThis follows from the existence of a global area-preserving\n(hence symplectic) diffeomorphism mapping~$A$ to a disk"
  context: Body of an \begin{unverified} lemma proof (Lagrangian product capacity lemma); a GAP comment on line 157–162 explicitly flags the entire argument as unverified agent-generated math — the phrase "This follows from" asserts a conclusion without a valid proof.

## Self-verify corrections

All 5 spot-checks matched source character-for-character — no corrections needed. The randomly-selected flags re-verified by the orchestration agent were:

- `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:119` ✓
- `crates/exp-hko-local-maximum/lagrangian-boundary/logbook.md:260` ✓
- `crates/exp-sys-landscape/variable-f-ascent/logbook.md:68` ✓
- `crates/dev-numerical-analysis/error-bounds/logbook.md:104` ✓
- `crates/exp-hko-local-maximum/second-order/math.tex:206` ✓
