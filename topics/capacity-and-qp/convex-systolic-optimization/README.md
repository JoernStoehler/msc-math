# Convex reductions in symplectic systolic optimization

Research paper from the autonomous 28–29 October 2026 run, separate from the selected MSc thesis manuscript. Component theorems have scoped independent mathematical audits. The final 56-page r6 PDF has an exact-hash assembled mathematical PASS in `reviews/REPORT-r6-delta.md` and a reader PASS with its companion in `reviews/REVIEW-r6.md`, building on the earlier reviews. Root inspected all 56 final pages; see `reviews/root-r6-layout.md`. Jörn's judgment remains separate.

Report revision: **2026-10-29-r6**. Distribute the PDF with its same-revision
source-and-evidence companion and SHA-256 manifest; finite certificate arrays
are not printed in the PDF.

## Paper and result map

- `main.tex` and `sections/` are the print source. Run `sh build.sh` here; the output is `paper.pdf`. Build intermediates stay under ignored `build/`.
- The exact twenty-facet body $H_{3/10}$ has ratio $1.051914\ldots$, exceeding the Haim–Kislev–Ostrover pentagon product. The latter nevertheless maximizes its full affine orbit, with an equality classification. Separately, the ten-cut family has positive first variation at the product; any seven selected cuts give seventeen genuine facets and a local ascent. Adding one or two characteristic-invariant affine hinges while retaining all old rows cannot give positive first variation, whether the two cuts share a facet or not. This does not prove local maximality among arbitrary twelve-facet bodies. A quantitative appendix certifies a tiny near-zero interval for all ten unit cuts only. The maximum along the family and over all convex four-bodies remains unknown.
- The four-simplex sharp bound was already in Haim–Kislev's 2019 thesis. The paper supplies a graph proof and equality description. The global six-simplex bound and unique equality class use an exact computer-assisted cdd/PPL polyhedral classification.
- The all-dimensional two-confocal-paraboloid difference-volume bound yields the sharp planar squared-body factor five. The right-quaternionic-$I$ four-dimensional capacity consequence need not have a sharp transferred constant.
- Every convex partner of a genuine corner-truncated parallelogram satisfies a strict Viterbo bound; an explicit capacity-one family disproves the proposed optimizer while staying below ratio one. A uniform gap extends the inequality to nearby second factors.

## Reproduction and evidence

- `certificates/affine-hko/`: exact Sage action witnesses and projected outer-body certificate; see `PROOF.md`. Run its `verify.py` with a Python interpreter providing `sage.all`.
- `certificates/triangle-position/`: exact Sage/PPL six-word and rank-one-hull verifier; see its `README.md`.
- `certificates/sixdim-global/`: four exact scripts, generated outputs, and a copied-package PASS receipt; see its `README.md`. The polyhedral completeness step needs exact Sage cdd and PPL backends.
- `certificates/hko-ascent/`: portable standard-library Python/C++17 replay, complete expected outputs, proof modules, and independent reviews; see its `README.md`. Its v2 companion adds the weighted-response witness and the full quantitative proof, minor bound, exact constant checks, and independent audits for the original twenty-facet family. The active paper section denotes the compatible complex structure by $\mathbb{J}$, whereas the frozen standalone base module calls that same map $J$.
- `certificates/hko-t03/`: exact Sage/C++17 certificate for the fixed $t=3/10$ body, including complete word coverage, singular KKT families, geometry, and volume; see its `README.md` and scoped reports in `reviews/`. The current `MANIFEST.sha256` covers the companion; the README records the original producer-manifest hash.
- `certificates/hko-two-hinge/`: portable standard-library exact replay of four literal shortest-word families and quotient constants supporting Appendix C. It reuses the sibling hko-ascent exact field and catalogue. The independent literal checker is in `reviews/independent-two-hinge/`.
- `reviews/`: scoped mathematical, exact-replay, and source-comparison reports. These do not approve later manuscript edits or certify novelty. `evidence/squared-scalar-proof.tex` preserves the previous exact scalar proof, which is not printed in the paper.
- `evidence/historical-comparisons/f11/` retains the separate accepted eleven-facet HKO local theorem's proof packet and acceptance record, cited to distinguish its scope from the twenty-facet ascent.
- `figures/` contains the vector figures used in the paper, with generation scripts in `figures/source/`. Selected paper sources and references are in `references.bib`.

Build success and finite replay do not validate the complete mathematical argument or the paper's readability. The repository's `PLAN.md` owns active research decisions and delivery.
