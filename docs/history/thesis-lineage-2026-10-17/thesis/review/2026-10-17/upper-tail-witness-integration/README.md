# A geometric selector can certify a ceiling without predicting capacity

17 October 2026. The selected Chapter 8 now has subsection **8.9, “From a geometric score to a ratio bound”**, on working-PDF page **63**. It derives $\operatorname{sys}(K)\leq1/(8Vq^2)$ from one positive feasible HK candidate $q$, and makes the sufficient threshold $q\geq(8SV)^{-1/2}$ explicit. For a proposed HKO ceiling, the missing result is a geometric selection meeting this threshold throughout the stated class. Tight capacity prediction is unnecessary for that purpose. The primal-edge ridge sum and the signed, closure-weighted dual-facet capacity expression retain different structure.

This is a self-contained exposition advance from the existing selected Chapter 4 HK theorem and six-facet reduction, not a new theorem, predictor or bound. The uniform threshold is not proved. Finite success or failure of a particular selector does not settle HKO maximality.

## How the failure review changed the chapter

The bounded read-only review in the separate patterns worktree recovered three complementary limitations:

| Existing failure | Consequence for the selected explanation | Evidence owner in patterns checkout |
| --- | --- | --- |
| The minimum-area triangle rule improves many bulk bounds but retains large principal-tail gaps | Average improvement does not establish adequacy near the largest ratios | `patterns/upper-tail/FROZEN-TAIL.md` |
| Fixed cone-volume closure weights miss exact HKO despite optimized order | Retaining signed geometry does not remove the need for suitable weights | `patterns/upper-tail/fixed-closure/REPORT.md` |
| Width-contact recovers HKO, yet another feasible support gives a materially stronger bound elsewhere | Recovery of one reference does not establish support selection across a class | `patterns/upper-tail/geometric-selector/REPORT.md` |

The one-response near-HKO miss is repaired by later opposite completion; it is not an obstruction to the current completed rule. The [manifest](source-manifest.json) binds inspected scientific reports to the separate patterns branch. No new experiment or producer replay was run. Detailed counterexample fixtures remain in research notes, consistent with Jörn’s placement preference; the pending containment and contact-loss choices remain pending.

The [end-state/method comparison](PLAN.md) rejects a method chronology, generic validation advice and full LP theory in favor of this small mathematical bridge. Bulk fit, reference recovery and mere shortness would each reward the wrong outcome. The selected passage relies only on mathematics already printed in Chapter 4; no two-shadow theorem, LP supplement or new external citation is required.

## Verification and exact-PDF boundary

[Verification receipt](verification.json): fresh selected build passes, all 174 old labels remain and one new label is added. There are 37 changed label entries, mostly later page shifts plus the local-search subsection number. Only the designated insertion changes the chapter source. The current working PDF is **106 pages**; [page 63](page-063.png) and [page 64](page-064.png) are retained views, with pages 62–65 inspected. This bounded layout check and independent mathematical reading do not establish human prose or whole-PDF acceptance.

The [Kai attachment](../../kai-packet/thesis-for-kai.pdf) remains the **frozen 105-page** candidate. Existing PM questions and its reading guide still refer to that exact file; it is no longer byte-identical to the working build. Do not transfer its page map or historical checks to the new working PDF. Build the working version with `bash thesis/check-build.sh` from the thesis checkout root. Root owns integration, local commit and cleanup; the patterns branch is unchanged.
