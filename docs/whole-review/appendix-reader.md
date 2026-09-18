# Appendix reader review of whole-v1

Reviewed artifact: `docs/whole-review/thesis-v1-5458356d.pdf`, SHA-256 `8c7bbb690af1a563cac2e6de984b9137907b8107e1f6384da4c82d4325a50a6b`, source revision `5458356d8e60a7594a0ae029f735fe77f875090f`.

Coverage: printed pages 78–91, all fourteen pages read as extracted text and inspected as individual rendered pages. This covers Appendix A and the first two pages of Appendix B. I additionally read the text of the program's conclusion on page 92 and the relevant local-search passage on pages 58–59 to check the appendix's backward references. This is an agent reader-legibility review, not human acceptance, a new theorem audit, or an experiment rerun.

## Material findings

1. **Define the distance and step normalization used in the optimizer diagnostics.** Pages 84–87 report fractions of the “distance to HKO” removed, plot “source distance from HKO” in Figure 12, prescribe “relative radii” in A.9, and use “normalized radius” in Figure 13. Neither these passages nor the associated main-text passage supplies the norm, denominator, or coordinate representation defining those quantities. Consequently a reader cannot interpret the scale of the perturbations, the reported 77.9%–100% distance removal, or the path length 0.01. Add the actual distance and step formulas, including whether symmetry directions or an alignment are removed, before the first calibration paragraph; explicitly distinguish different conventions if the retained diagnostics use them. Briefly identify the four Figure 12 direction labels: “random 000”, “random 001”, “pentagon-tangent sentinel”, and “slice-basis sentinel” currently name runs without explaining their geometric construction. This is a definition repair, not a request for further experiments.

2. **State the beta admissibility rule behind Table 2.** Page 84 says the history method uses a “normalized beta allowance 0.3” and discards “beta-inadmissible solutions”, but never defines the normalization or inequality. The main quadratic-program constraint is nonnegative beta, so the unexplained allowance also leaves unclear whether the implementation relaxes that constraint or uses a different filter. State the actual rule in one formula or sentence immediately below the table and explain what the 0.3 measures. The retained implementation should supply the definition; this review does not infer it from the parameter name.

Neither finding indicates a demonstrated numerical or mathematical error. Both materially affect interpretation of the claimed reproducible optimizer method and its diagnostic figures, and can be repaired locally.

## What read coherently

The triangle proof on pages 78–79 can be followed from the main-text family definition and the quadratic-program convention: normal labels, closure weights, cyclic-word cases, attaining order, capacity normalization, ridge sum, and angle reduction form a continuous argument. The ideal-versus-rounded balancing distinction is explicit.

Pages 79–83 distinguish original observations from retrospective checks, capacity bounds from numerical systolic ratios, deterministic comparisons from asserted random controls, conditional resampling from repeated-pool uncertainty, and finite effects from universal rules. The factor-balancing subsection states both the intervention and its limits. I found no new contradiction among these passages.

The adaptive-sampling appendix on page 88 defines the log-gap and log-length coordinates before reconstruction and gives the ordinary and circular update rules. Its admission revision and coordinate dependence are explained alongside the actual result, so the technical information serves interpretation rather than merely listing retained files.

Table 3 on page 89 is a useful result-to-source map with a stated repository root convention. It is an index, not a standalone one-command reproduction procedure, and does not claim otherwise. Appendix B explains what each code block contributes to Lemma 7.4, distinguishes assigned exact data from computed derivatives, and connects the finite checks to the subsequent geometric argument. The continuation on page 92 identifies the executable, command, assertion requirement, and expected output. The long exact assignment list is justified by the promised complete certificate.

No clipped formulas, overlapping text, broken tables, or unreadable plot labels were found in the fourteen rendered pages. The small code font remains legible when zoomed. Figure floats interrupt the failure-audit paragraph between pages 84 and 86, but the continuation remains identifiable; I do not treat that pagination as a material defect.

## Limits

No producers, analyses, CAS programs, or builds were rerun. Repository path availability, numerical values, mathematical correctness, and full executable provenance retain their earlier specialist audits. Page 92 was read only to finish Appendix B's exposition; the bibliography and the remainder of the thesis are outside this review. The findings concern the frozen whole-v1 PDF, not later source edits.
