# Independent geometry and benchmark review

Reviewer: `/root/core_review`, 17 October 2026. Read-only toward producer/project files; reviewer-owned script and verification result live beside this note.

Sources inspected: the assignment and frozen PLAN, current OWNER-NOTES/OWNER-FOLLOWUP/ROUTE-TRIAGE, the scout derivation, `topics/products/pentagon-affine-products/research.tex`, producer `analyze.py`, its retained row/summary outputs, the pinned original `random-product.jsonl`, and the pinned width table. This is a scoped source review, not an exhaustive repository or literature search.

## Verdict

The fixed candidate is mathematically coherent and correctly implemented. Containment would give a monotonicity lower bound in exact arithmetic; numerical implementation remains binary64 and has tiny positive halfplane residuals. The existing affine-pentagon theorem supplies the exact capacity formula on each fitted core. The covariance construction recovers an affine regular pentagon: its whitened vertex set is an orthogonal image of the regular set, and reflections are absorbed by pentagon reflection symmetry. Thus the HKO control is recovered without label fitting. Existing width equality on this affine family justifies the bracket-center control.

The finite retrospective diagnostic supports stopping this fixed rule as an upper-tail explanation. Its principal 63 tail cases have no capacity-error improvements over width. Exactness on HKO alone gives no evidence of useful approximation on irregular retained leaders. No universal impossibility or global-maximality consequence follows.

## Independent verification

`verify.py` imports no producer code and performs no capacity solve. It uses scalar 2×2 inverses and scalar normal pairings for the known family formula; original source halfplanes for every retained core; independently reconstructed vertices and shoelace areas; source labels and pinned width rows; the frozen name partition; source-defined top-21 selection in every family; and independent error aggregates.

`verification.json` records PASS for all 10,240 products / 20,480 factor cores. Maximum halfplane excess is `5.81756864903582e-13`; maximum relative scalar-formula disagreement is `3.3240077357277187e-13`; maximum coverage disagreement is `1.6089352072867769e-12`; maximum relative volume disagreement is `7.838174553853605e-13`. All 10 family-tail memberships and all/calibration/diagnostic/per-family/tail capacity and ratio relative-error quantiles and capacity MAEs pass. Separate regular-rotation and HKO scalar controls pass.

These checks do not certify historical numerical capacity labels, exact containment, or future generalization. The verifier checks every selected matrix's validity and resulting score; source review, rather than a second implementation, checks the prescribed phase/scale-selection algorithm. All labels are historically exposed, so the split is retrospective rather than an unexposed holdout.

No consequential mathematical/code objection remains in the reviewed candidate and recorded diagnostic. Exact completed REPORT text is reviewed separately before delivery.

## Exact completed report review

APPROVE REPORT-v2 at `worker/REPORT.md`, SHA-256 `32872337c2cc3f546acceff72e4709da97d97296a422bfc3280f11a2daa93e5a`, including `[review: /root/core_review]`. The two REPORT-v1 transcription/description issues were corrected: the named row prediction is 0.351135, and summary contents are described as error summaries. The scientific conclusion, source limits, retrospective label status, scalar errors, coverage decomposition, controls, stop decision and ownership handoff are supported. No unresolved consequential objection. Figures were visually inspected by the producing owner; this independent review verifies the numerical claims and their source relation.
