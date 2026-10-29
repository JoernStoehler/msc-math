# Convex reductions in symplectic systolic optimization

Research paper from the autonomous 28–29 October 2026 run. This packet is separate from the selected MSc thesis manuscript. The selected component theorems have scoped independent mathematical audits, and the assembled 48-page r3 mathematical delta has an agent PASS recorded in `reviews/REPORT-r3-delta.md` alongside the full r2 review. Root's final visual/editorial inspection and Jörn's judgment remain open.

Report revision: **2026-10-29-r3**. Distribute the PDF with its same-revision
source-and-evidence companion and SHA-256 manifest; finite certificate arrays
are not printed in the PDF.

## Paper and result map

- `main.tex` and `sections/` are the print source. Run `sh build.sh` here; the output is `paper.pdf`. Build intermediates stay under ignored `build/`.
- The full affine orbit of the Haim–Kislev–Ostrover pentagon product has a sharp upper bound and equality classification. A fixed ten-cut inward family has a positive exact first variation. Any seven selected cuts give seventeen genuine facets and positive variation. For all ten cuts, the explicit but tiny parameter $t=10^{-30}$ improves the ratio by more than $5\cdot10^{-33}$ relatively. The seventeen-facet result has no certified fixed parameter here.
- The four-simplex sharp bound was already in Haim–Kislev's 2019 thesis. The paper supplies a graph proof and equality description. The global six-simplex bound and unique equality class use an exact computer-assisted cdd/PPL polyhedral classification.
- The all-dimensional two-confocal-paraboloid difference-volume bound yields the sharp planar squared-body factor five. The right-quaternionic-$I$ four-dimensional capacity consequence need not have a sharp transferred constant.
- Every convex partner of a genuine corner-truncated parallelogram satisfies a strict Viterbo bound; an explicit capacity-one family disproves the proposed optimizer while staying below ratio one. A uniform gap extends the inequality to nearby second factors.

## Reproduction and evidence

- `certificates/affine-hko/`: exact Sage action witnesses and projected outer-body certificate; see `PROOF.md`. Run its `verify.py` with a Python interpreter providing `sage.all`.
- `certificates/triangle-position/`: exact Sage/PPL six-word and rank-one-hull verifier; see its `README.md`.
- `certificates/sixdim-global/`: four exact scripts, generated outputs, and a copied-package PASS receipt; see its `README.md`. The polyhedral completeness step needs exact Sage cdd and PPL backends.
- `certificates/hko-ascent/`: portable standard-library Python/C++17 replay, complete expected outputs, proof modules, and independent reviews; see its `README.md`. Its v2 companion adds the weighted-response witness and the full quantitative proof, minor bound, exact constant checks, and independent audits for the original twenty-facet family. The active paper section denotes the compatible complex structure by $\mathbb{J}$, whereas the frozen standalone base module calls that same map $J$.
- `reviews/`: scoped mathematical, exact-replay, and source-comparison reports. These do not approve later manuscript edits or certify novelty. `evidence/squared-scalar-proof.tex` preserves the previous exact scalar proof, which is not printed in the paper.
- `evidence/historical-comparisons/f11/` retains the separate accepted eleven-facet HKO local theorem's proof packet and acceptance record, cited to distinguish its scope from the twenty-facet ascent.
- `figures/` contains the vector figures used in the paper, with generation scripts in `figures/source/`. Selected paper sources and references are in `references.bib`.

Build success and finite replay do not validate the complete mathematical argument or the paper's readability. The repository's `PLAN.md` owns active research decisions, including final assembled review and delivery.
