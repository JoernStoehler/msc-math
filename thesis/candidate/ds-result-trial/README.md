# Result-first DS passage trial

This is a proposed replacement for the selected chapter's fresh-selection,
transfer, support-height, ambient-rotation and adaptive-sampling passages.
It is not selected by `thesis/candidate/main.tex` and has no human PASS.

Build from repository root with
`sh thesis/candidate/ds-result-trial/build.sh`.
The preview includes a short context paragraph; it is not proposed as a new
paragraph in the complete chapter. Full integration requires moving displaced
method details to the existing appendix and maintaining its references.

Direct human feedback on a shorter rotation rewrite, 18 September 2026:
the ambient transformation was ambiguous, the number of evaluations was
overemphasized, and the account narrated procedure instead of presenting
results and their support. The revised passage distinguishes the SO(4)
action from rotation of one factor and leads subsections with findings.

Evidence is the same as for `ds-complete-draft/chapter.tex`, with one added
result from the retained initial scalar-selection experiment: the row
`per_bucket_low_ridge_symp_area_sum_over_volume_sqrt_top_10` of
`experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selection-summary.tsv`
reports selected and comparison means 0.6261166821 and 0.3155039434.
No new experiment was run. All reported systolic ratios remain numerical.

The revision explains covariance through its Williamson form and proves
its invariance; for products it also expresses the squared Williamson
eigenvalues through the two planar covariance matrices. It distinguishes
largest-ridge dominance from overall evenness, the generic mean-ridge
selector from the product sum selector, and a successful population filter
from an improving change to every individual body. The adaptive result
retains its coordinate-dependent admission limitation.

The opening is organized around the collective scientific inference:
two different affine-symplectic summaries identify higher-mean selected
populations, without providing a capacity law or a universally improving
deformation. A single table collects the selection contrasts after the
covariance definition and argument; it replaces the initial experiment-by-
experiment narrative. This is a proposed remedy for the direct human
criticism of the earlier opening, not a claim that the remedy is accepted.

Following Jörn's judgment that the finite rotation-allocation result does
not merit a main subsection, its comparison is retained in `appendix.tex`.
The main passage contains only its conclusion and an appendix pointer.

The compact [evidence note](evidence.md) records claim sources, mathematical
additions, checks, and remaining integration choices.
