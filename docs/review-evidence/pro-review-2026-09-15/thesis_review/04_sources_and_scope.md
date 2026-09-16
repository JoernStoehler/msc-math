# Sources, review scope and evidence boundaries

## Submitted document

*Probing Viterbo’s Conjecture*, Jörn Stöhler, 2026. The supplied PDF has 86 pages; its printed numbering agrees with PDF page numbering.

Original file SHA-256:

```text
d7dae9a78dffc87fe89f3005bed9b6b51d28728fa90e1a5c811bc09b236e1695
```

All 86 pages were read. All pages were rendered and visually inspected in contact sheets, with individual-page inspection of the key mathematical figures, tables and code listings. Findings about nearly empty pages, table placement and code readability refer to the actual rendered PDF, not merely to text extraction.

Page, section, theorem and equation anchors are used throughout the standalone files so that they remain useful outside the chat. The PDF annotations use the same R01–R35 and Q01–Q03 identifiers. A statement labelled a reviewer derivation is not being attributed to the original thesis.

## External primary sources used for substantive corrections

**E1. Alberto Abbondandolo, Oliver Edtmair and Jungsoo Kang.** *On closed characteristics of minimal action on a convex three-sphere*. arXiv:2412.01777v1, submitted 2 December 2024. See the abstract and **Corollary 1**, in the introduction’s “Consequences concerning symplectic capacities.” The stronger dynamical theorem concerns uniformly convex domains; the stated capacity coincidence extends to every convex body in R⁴. This is the direct source for R01.

**E2. Pazit Haim-Kislev and Yaron Ostrover.** *A counterexample to Viterbo’s conjecture*. *Annals of Mathematics* 203(2) (2026), 603–622, DOI: 10.4007/annals.2026.203.2.5. The final-publication metadata was checked on the journal site; the readable mathematical text consulted was **arXiv:2405.16513v3**. Proposition 1.4 supplies the HKO value; §1.1(iii) reports EHZ/cylindrical coincidence and cites E1. The thesis’s p. 5 pinpoint is not reconciled to that version. The analytic replacement proof uses the HKO value, not the thesis’s rotation theorem.

**E3. Alexey Balitskiy, Ivan Mitrofanov and Alexander Polyanskii.** *Triangle covering problems and the Viterbo inequality in the plane*. **arXiv:2603.12495v1**, 12 March 2026. Theorem 1.2 covers arbitrary quadrilateral factors and affinely regular hexagonal factors; §4 treats triangular factors. R02’s identification of seven excluded side-count groups is a deduction from those results and the thesis’s own sample-group list. This is a preprint; it is not described here as a refereed 2026 journal article.

**E4. Pazit Haim-Kislev.** *Dynamical extensions of Zoll to nonsmooth convex bodies*. **arXiv:2511.16644v1**, 20 November 2025. The introduction, Example 1.12 and Proposition 1.13 give the directly relevant nonsmooth/cutting context. R03 does not claim this paper already proves the thesis’s local-maximality theorem. Statements reported there as numerical evidence are not upgraded to theorems in this review.

**E5. Daniel Rudolf.** *The Minkowski Billiard Characterization of the EHZ-Capacity of Convex Lagrangian Products*. *Journal of Dynamics and Differential Equations* 36 (2024), 2773–2791, DOI: 10.1007/s10884-022-10228-0. **Definition 2 and Theorem 1 in the final journal text** were checked. The theorem supports the nonsmooth strong-billiard input used in §4.3. Different arXiv numbering is not a reason to call the thesis’s Theorem 1 citation incorrect.

**E6. Foundational variational sources.** Pazit Haim-Kislev, *On the symplectic size of convex polytopes*, DOI: 10.1007/s00039-019-00486-4, with arXiv:1712.03494 consulted; Shiri Artstein-Avidan and Yaron Ostrover, *Bounds for Minkowski billiard trajectories in convex bodies*, DOI: 10.1093/imrn/rns216, with arXiv:1111.2353 consulted. These were used to check the scope and normalization of the imported finite and nonsmooth variational results. The possible lemma-number discrepancy is left as Q03 because final-edition numbering was not established for that pinpoint.

## Repository inspection

The GitHub connector successfully accessed the repository named in the thesis, `JoernStoehler/msc-math`. The inspected material included the pentagon proof directory and the branch classifier source, the HKO packet inventory, and repository HEAD metadata.

Observed HEAD:

```text
1bec30637070effe7ab8b0a022ae62e83f6bbcab
```

Inspected pentagon classifier blob:

```text
01bc62dc0691574efaf86149bcc22cabe4593aa2
```

Path:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

These identifiers pin what was inspected during the review. They are **not** claimed to be the revision that generated the supplied PDF. That correspondence is one of the missing submission details. The repository is accessible; R09 concerns immutable identification and complete dependencies, not an invented availability failure.

## What was executed, and what was not

Executed checks are included in `verification/`: an independent exact reconstruction of the HKO witness, rational checks of the flow-genericity determinants, and a numerical enumeration/rank diagnostic for the 3,340 pentagon words. The two analytic arguments in the mathematical audit were derived during the review and checked algebraically; they are presented with their proofs.

The thesis’s full Sage pentagon classifier was **not** rerun. The complete Rust implementation, all arithmetic/compiler paths, all historical data, the full experimental pipeline and every figure producer were **not** independently reproduced. In particular, the historical correlations, prediction metrics and optimizer summaries were checked as reported claims, not re-estimated from unavailable source data. No claim of exhaustive plagiarism checking, publication-priority determination or compliance with a particular university’s AI rules is made.

This distinction explains the language of the findings: an observed textual error is called an error; missing evidence is called missing evidence; an unexecuted or unresolved implementation obligation is not called a proved computational failure. Editorial judgments identify their concrete reading cost and a repair, rather than pretending to be mathematical counterexamples.
