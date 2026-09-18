# Complete DS chapter candidate

Build from repository root with `sh thesis/candidate/ds-complete-draft/build.sh`.
Preview: `build/preview.pdf`, including title/contents, the main chapter, a proofs and methods appendix, and bibliography. The contents page identifies the current main-text range.

For full-thesis integration, replace the selected DS chapter input with `ds-complete-draft/chapter.tex` and the selected DS appendix with `ds-complete-draft/appendix.tex`. Add `ds-complete-draft/references.bib` to the bibliography resources. The baseline/additional bibliography copies are for standalone preview only; do not load them twice. The preview's synthetic labels P and V point to the companion pentagon theorem and first-order search-model subsection; full integration resolves the existing real labels.

Figures and their paths are self-contained in this directory. Two new PDF plots redraw the same 14,336 historical points in two-column groups for legibility; their checked-input producer and point-count receipt are retained. The conditional-rank plot summarizes restriction of those same bodies by ridge area or ratio. The original supplied SVG/PDF remain for comparison. The two optimizer diagnostic PDFs are unchanged copies from the selected candidate.

Current evidence index paths require integration of the reviewed evidence packets in addition to these reader-facing files. `docs/complete-ds-chapter/reference-check.json` identifies their currently accessible owners. The parent integrator owns that source closure.

The ridge descriptor is denoted R throughout, matching the human-accepted correlation paragraph; previous complete chapter used S. Existing chapter/appendix section labels are retained. No claim of human acceptance applies to the complete revision.
