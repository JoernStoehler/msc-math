# Selected symmetric-partner result: introduction and conclusion reconciliation

22 October 2026. This is a narrow writing change to an already selected result,
not a new inclusion decision. The prior introduction result summary/roadmap and
conclusion described rotation, affine pentagons and product position but omitted
the integrated symmetric-partner Section 11. The selected theorem and its proof
are unchanged.

## Reader-facing additions

The introduction now explains that fixing a regular pentagon while allowing any
centrally symmetric planar convex partner gives a different extension from
independently deforming two regular pentagons. Capacity is the largest scale of
a prescribed decagon that fits inside the partner. The sharp ratio is
$(10+6\sqrt5)/25<1$, with equality only for translates and positive dilates of
that decagon in its fixed orientation. Thus a symmetric partner cannot reproduce
HKO's ratio greater than one. The roadmap now points to the selected section.

The conclusion states the same scope and sharp bound, and emphasizes the proof's
reference-body calibration, containment and area argument. It explicitly excludes
an arbitrary asymmetric-partner bound. No 5×7 or newly checked local-cover result
has been inserted.

Exact changed prose: [introduction](../../thesis/chapters/01-introduction.tex)
and [conclusion](../../thesis/chapters/16-conclusion.tex).
Unchanged mathematical source: [selected symmetric-partner theorem](../../thesis/chapters/10b-symmetric-partners.tex).
Selection provenance: [project facts §13.2](../../docs/project-facts.md).

## Validation and limits

Independent bounded Luna source check completed on 22 October, 15:50:21 UTC,
by `/root/msc_today_scope/selected_summary_check`. It matched the sharp ratio,
partner hypotheses, scale interpretation, section reference and selected include.
It identified one precision issue: the conclusion should explicitly retain the
fixed-orientation equality qualifier. That qualifier was added before integration.
The reviewer confirmed no 5×7/local-asymmetric claim and no theorem-source edit. Coordinator matched hypotheses,
normalization, equality and theorem reference to the selected source. The source
uses a standard existing section label and ordinary prose/math; no new package,
figure or citation was introduced. The selected thesis was subsequently built successfully (105 pages, 12 seconds).
Luna and the coordinator visually inspected PDF pages 8, 9 and 87 at 120 dpi:
the added passages, Section 11 references, formula and orientation qualifier
render correctly without clipping, overlap or split paragraphs. The final log
contains zero undefined references/citations; one overfull and three underfull
boxes occur in unrelated unchanged numerics/code-data text.

The current [working PDF](../../thesis/build/main.pdf) now contains these edits.
Its SHA-256 is `00aabd301cd9dcaae0c0a67b81dc0545533789f4d1ca24007831109c32384a06`.
[Build log, inspected-page renders and input identity manifest](../../docs/history/selected-partner-navigation-20261022/manifest.json)
are retained with the evidence owner. This focused rendered-page check is not
whole-PDF acceptance or human proof/prose acceptance. The historical global
source-review manifest remains unchanged; a new build does not refresh its
scientific acceptance claim.
