# Core reader B: bounded PDF reading

Reviewed frozen `docs/whole-review/thesis-v1-5458356d.pdf` on 18 September 2026.
Page references below are the printed PDF numbers, which match the PDF page indices.
This is an independent reader review of exposition and proof legibility, not another
certificate or implementation audit. No manuscript sources were consulted or changed.

## Coverage and judgment

Read all prose, displayed statements, and proof steps on pp. 29–48 and 59–73:
the flow-graph construction and genericity argument; first-order variation; HKO
local maximality; rotated pentagons; independent linear factor deformations;
rotation out of symplectic position; visualization; numerics; and availability.
Also read the HKO assignment table on p. 49. The data-science material on p. 59
was read as the transition into the assigned rotation chapter, without auditing
its evidence.

At the coordinator's additional request, read the abstract on p. 1 and the
**complete conclusion, pp. 77–78**. The conclusion continues onto p. 78 before
Appendix A begins. These were actual prose readings, not consistency-only checks.

Selective prerequisite reading covered the notation on pp. 8–9 and 11, dual
reconstruction on pp. 13–16 and 20–21, and the finite QP and product reductions
on pp. 21–28. Appendix B, pp. 90–92, was inspected to establish what the main
proof's program reference supplies; the program was not executed or independently
audited. Rendered-page inspection covered pp. 1, 31, 43, 46–47, 60, 65–67, 69–70,
and 77, including every figure in the assigned mathematical chapters.

No substantial exposition or proof-legibility blocker surfaced in this bounded
reading. In particular, the HKO text supplies the steps from smooth feasible
sections through a uniform transverse estimate to an actual neighborhood and the
equality cases. The rotation and affine-product proofs explain their respective
comparison mechanisms. The flow-graph assumptions and the distinction between
the two capacity algorithms are explicit. The abstract and conclusion communicate
the main results and their scope without requiring substantive rewriting.

The following two minor repairs would remove local reading friction. Their combined
editing effort should be comfortably below ten minutes, apart from rebuilding.
This assessment does not certify the full thesis as submission-ready: exact
identities, code behavior, literature accuracy, and external artifact availability
were not revalidated here.

## Findings

### B1. Name the right-hand side in the inverse-defect lemma

- **Page:** 70, Section 13.2 and Lemma 13.1.
- **Quote:** “Suppose outward arithmetic establishes” followed by
  `r̄ ≥ ‖𝒦x̂ − b‖∞`.
- **Severity:** Minor; locally undefined symbol.
- **Why it matters:** The preceding display gives the right-hand side as the
  block vector `(0,d)` but never names it `b`. The lemma then uses `b` as though
  already defined. Its meaning is inferable, but a central certification statement
  should specify its equation and dimensions without that inference.
- **Minimal repair:** Immediately after the stationary-system display, write
  “Set `b=(0,d)^T` and `x=(β,λ)^T`, so the system is `𝒦x=b`,” with the block
  notation formatted consistently with the display. No mathematical change is
  needed.

### B2. Give the HKO assignment table an explicit destination

- **Pages:** 45 and 49, Section 7.4.
- **Quote:** “The accompanying table gives the exact fixed values for every
  section, including the worked entry.”
- **Severity:** Minor; proof navigation.
- **Why it matters:** The table appears four pages later, after the theorem proof
  on p. 48, immediately before Section 8. It has the heading “The twenty-six exact
  choices” but no table number. While following the reconstruction of the
  derivative matrix, the reader has no precise pointer to the required finite
  assignments.
- **Minimal repair:** Number and reference the table, or add an explicit page
  reference to its heading. Relocating or rewriting the proof is unnecessary.

## Limits and rejected concerns

The genericity proof's repeated-row specialization and the HKO certificate were
read for their logical role, not recomputed. The numerical chapter states how its
implementation supplies outward enclosures; this review did not establish that
the implementation meets those contracts. The availability chapter was read for
what it promises, without visiting the repository or recovering datasets.

Text extraction made the product-rotation chapter's `J₀` and `𝒥₀` look identical.
Rendered pp. 66–67 distinguish them, so this is **not** a notation finding.
The decagon `D` and matrix convex hull `𝒟` are likewise distinct in the rendered
affine-product proof. No broad rewrite is recommended from this review.
