# Systolic extrema, nonsmooth Zoll geometry, and the HKO comparison

Checked 18 September 2026. This is an open-search map with a completed statement-level comparison, not a reproof of the papers or a priority certificate.

## Consequence for the thesis

No checked result subsumes the project's fixed-ten-facet HKO local-maximality theorem. Its natural positioning is a concrete nonsmooth extremality result in a specified finite-dimensional perturbation space. Do not turn that into maximality among all nearby convex bodies. The new affine-product theorem belongs to another restricted family and likewise cannot close the unrestricted problem.

Project source read: `formal/hko-feasible-section-upper-branches.tex`, especially the quotient-template corollary and Hausdorff-domain remark at the end. The local parameter chart has forty dual-vertex coordinates, fifteen symmetry directions (translation, dilation, linear symplectic action), and a twenty-five-dimensional transverse slice. The proof route uses feasible upper branches, a positive relation among their gradients and full transverse rank. The stated Hausdorff extension retains exactly ten facets. I inspected this scope, not the Sage certificate anew.

## Most directly related source: Haim–Kislev 2025

[arXiv:2511.16644v1](https://arxiv.org/html/2511.16644v1), *Dynamical extensions of Zoll to nonsmooth convex bodies*. Read introduction, cited statements and Proposition 1.13 appendix proof; history checked.

Cuts additivity means capacity equals the sum of the capacities of both pieces for every hyperplane cut. Cuts extremality means each proper piece has strictly smaller capacity. Example 1.12 supplies numerical evidence concerning HKO cut extremality/nonadditivity; it does not prove local systolic maximality. Proposition 1.13 proves additivity near the boundary for normals `(u_p,u_q)` with `u_p ∈ N_P(v_i)` and `u_q ∈ N_T(w_j)`, `j ∈ {i−1,i,i+1}`; pure-factor normals are included. HKO's generalized-Zoll status is left unresolved. Remark 1.15 explains why the index-five numerical approach does not settle it. Conjecture 1.17 concerns smooth domains, not an equivalence available for HKO.

**Comparison (our reasoning):** capacity loss under a cut does not control its rate relative to volume loss. Moreover, introducing a cut typically introduces an additional facet; a generic such perturbation is outside the project's theorem domain. Conversely, moving ten old facets is not a single-cut operation. Consequently neither theorem supplies the other's missing implication. 

## Broader primary-source map

### Haim–Kislev–Ostrover: counterexample and replacement questions

[arXiv:2405.16513v3](https://arxiv.org/html/2405.16513v3), 20 November 2025; [published Annals article](https://annals.math.princeton.edu/2026/203-2/p05), 203 (2026), 603–622, DOI 10.4007/annals.2026.203.2.5. Read introduction, §1.1 and endpoint statement; did not redo full proof.

§1.1 explicitly asks about centrally symmetric convex domains, the largest systolic ratio, and distinct capacities. It records attainment of the convex-body supremum and constructions in higher dimensions. These supply legitimate forward-looking context, not proof that HKO is the global optimizer. Remark 1.8 also prevents treating systolic ratio one as a characterization of symplectic balls. The final publisher metadata should replace any stale “to appear” label. No later primary-source resolution of the unrestricted maximization question was found in this search.

### Matijević: a different invariant of minimizing orbits

[arXiv:2501.13856v1](https://arxiv.org/html/2501.13856v1), 23 January 2025, *Systolic S1-index and characterization of non-smooth Zoll convex bodies*. Read introduction, Theorems 1.2 and 1.5, Corollary 1.6 and Questions 1–7; proof not audited.

Theorem 1.2 identifies a Fadell–Rabinowitz index of centralized systoles with the number of initial coinciding Gutt–Hutchings capacities. Theorem 1.5 characterizes generalized Zoll by `c_1=c_n`, recovers ordinary Zoll for smooth convex bodies, and establishes Hausdorff closedness. Question 7 asks whether generalized Zoll bodies maximize the systolic ratio locally in Hausdorff topology. Thus identifying many shortest orbits, or establishing fixed-facet local maximality, is not already a generalized-Zoll criterion. Potential research lead: topology of the minimizing-orbit space is genuinely different information from scalar capacity and facet geometry. It is not a cheap feature to promise without constructing that space.

### Smooth local optimality and topology of perturbations

[Abbondandolo–Benedetti, GAFA 33 (2023), 299–363](https://doi.org/10.1007/s00039-023-00624-z); [arXiv:1912.04187](https://arxiv.org/abs/1912.04187). Publisher/abstract checked: Zoll contact forms are local systolic maximizers. This is smooth-contact background, not a result about the HKO corner geometry. Use the later result below when discussing the sharper topology.

[Abbondandolo–Benedetti–Edtmair, arXiv:2312.07363v1](https://arxiv.org/html/2312.07363v1), *Symplectic capacities of domains close to the ball and Banach–Mazur geodesics in the space of contact forms*. Abstract and bibliographic trail checked; full HTML reopened unreliably and the institutional PDF returned 403. It proves coincidence of normalized capacities in a C2 neighborhood of the ball, with failure for some C1-close smooth domains. Published Duke Math. J. 174 (2025), 1567–1646 per the HK bibliography; independently verify final theorem numbering before importing a numbered theorem. This matters because closeness topology is substantive, not editorial detail.

[Sanjay, arXiv:2512.01937v1](https://arxiv.org/html/2512.01937v1), *On the C2-local systolic optimality of Zoll odd-symplectic forms*, 1 December 2025. Read introduction and Theorem 1.4 hypotheses. Extends local optimality to odd-symplectic forms near a Zoll form within its cohomology class. Useful newer background if the thesis surveys generalizations; it does not remove the smoothness/topology obstacle for polytopes. No need to develop this apparatus in the thesis merely to cite it.

### Capacity equality in dimension four

[Abbondandolo–Edtmair–Kang, arXiv:2412.01777v1](https://arxiv.org/html/2412.01777v1), *On closed characteristics of minimal action on a convex three-sphere*. Read Theorem 1 context, Corollary 1 and uniform-convexity discussion. The dynamical statement requires uniform convexity, but Corollary 1 extends EHZ/cylindrical equality to every convex body in R4. This is the source for the already identified introduction correction. It does not imply Gromov width equals EHZ. It makes EHZ computations more consequential, but does not turn capacity computation into a general symplectic-embedding construction.

### Higher-capacity extrema: optional neighboring direction

[Baracco–Bernardi–Lange–Mazzucchelli, arXiv:2303.13348](https://arxiv.org/abs/2303.13348); [author-hosted manuscript](https://webusers.imj-prg.fr/~marco.mazzucchelli/preprints/higher_capacities.pdf). Abstract and Proposition D/Example 1.3 read; full theorem proofs unreviewed. Four-dimensional smooth star-shaped fixed-volume local maxima of higher Ekeland–Hofer capacities are related to suitable rational ellipsoids. The paper also compares global ellipsoid extrema with polydisks. This is a different objective from first-capacity systolic optimization; potentially useful context if investigating orbit spectra, not a substitute capacity routine.

## Search trail, boundaries and next actions

Queries included `symplectic systolic local maximizers convex bodies Abbondandolo Benedetti Edtmair 2025 Zoll`, `Haim Kislev Ostrover counterexample Viterbo conjecture local maximum pentagon 2026`, `Dynamical extensions of Zoll citing`, `site.arxiv.org local maximizers capacity ratios`, `site.arxiv.org generalized Zoll Matijevic`, and `site.arxiv.org Viterbo 2026 symplectic counterexample maximum`. Followed HK's references backwards and author/publication pages forwards. This is not a citation-index exhaustive forward-citation search. One ResearchGate-hosted alleged HKO refutation surfaced; not assessed, and no mathematical conclusion here relies on it or on search-result summaries of it. Ordinary name/keyword searches produce considerable unrelated “Viterbo” and Riemannian-systolic material.

1. Integrate the precise HKO comparison and AEK correction now; no open literature question blocks that.
2. If making a claim about generalized-Zoll HKO or all-convex local maximality, reopen the relevant exact question and source chain. Current project results do not answer them.
3. If importing smooth normal-form results, obtain the final ABE theorem statement rather than citing this abstract-level entry.
4. Preserve distinction between an orbit enumeration, a space of minimizing orbits, and its equivariant topology; identifying the latter is a separate research task.
5. No evidence found requiring weakening the accepted fixed-facet theorem. Absence of a subsuming result in this search is not a claim of publication priority.
