# Independent adversarial review — Appendix B

Reviewer: `/root/review_appb`, 17 October 2026. Scope: exact current baseline
`/tmp/msc-math/thesis3/20261017/baseline.pdf`, printed/physical pages 101–103;
`thesis/appendices/hko-certificate.tex` and `hko_core.py`; their interfaces to
Chapter 7's geometry, worked section and exact-choice table; the canonical HKO
witness/verifier and retained execution receipts. Source inspection and PDF text
inspection were used; this review does not claim a fresh visual page inspection
or a fresh Sage execution.

Baseline SHA-256:
`76fb3ac4e48d10c0175b1f41f5977ae57fd0ee95a272467b422b2ad8d24dd236`.
Printed program SHA-256:
`4b82246259a04e5d4c3cc89134bf4a94de6c1899c2abca3fae8efc63119ac403`.

## Findings and disposition

**No confirmed consequential defect in the inspected appendix. No source repair
is required by this review.** This is a bounded adversarial review, not a proof
of absence of defects or human acceptance of the theorem.

| Severity | Location | Attack/check | Result and disposition |
| --- | --- | --- | --- |
| High if wrong; no defect found | p. 102, core lines 40–67; `hko-lemma-bounds.tex:164–177` | A positive base vector alone would not supply a smooth touching branch; wrong chain-rule signs could invalidate every derivative. | The inverse-minor section keeps precisely the fixed positions constant, differentiates `C beta=e` with RHS `-beta` in the changed coordinate, includes both geometric terms in `DQ`, and uses `DA=-DQ/(2Q²)` and `DU=A DA/V-A² DV/(2V²)`. These agree with the displayed mathematical formulas. |
| High if wrong; no defect found | pp. 101–102, core lines 12–38; `hko-sections.tex:18–43` | Decimal-looking exact values or one-/zero-based conversion could select different sections. | Fresh ordinary-Python parsing compared all 26 words, solved positions and fixed positions exactly against `witness.json`. A separate rational/polynomial evaluator compared every displayed Chapter 7 table value with every printed core value exactly, including the exceptional entry 22. All agree. No floating-point tolerance was used. |
| High if wrong; no defect found | p. 103, core lines 69–91; `hko-certificate.tex:59–81` | Rank alone would not force negative slope; the symmetry span or positive relation might be missing. | Code checks each symplectic generator, tangent rank 15, row rank 25, exact annihilation, a one-dimensional left kernel and strictly positive normalized coefficients. Appendix explicitly leaves the neighborhood/equality implication to geometric lemmas rather than claiming the CAS checks it. |
| Medium if wrong; no defect found | pp. 101–103, `hko-certificate.tex:37–75` | Printed excerpts might omit executable lines or their line references might point at different predicates. | Blocks cover lines 1–11, 12–39, 40–68 and 69–92 without a gap. Predicate references to lines 84–91 are accurate. The extracted command contains literal `--name` and `PYTHONOPTIMIZE=0`. |
| Medium residual dependency | p. 102, core lines 48–49; `hko-lemma-geometry.tex:103–135` | Correct linear algebra cannot validate the imported capacity identity or the geometric volume derivative. | Geometry, field, volume and volume-derivative formulas match their manuscript source. Independent acceptance of that derivation belongs in the Chapter 7 reading item below, not in a 26-row manual audit. |

## Execution evidence and its limits

The current `hko_core.py`, canonical `verify.sage.py` and `witness.json` have
exactly the hashes recorded in
`docs/history/thesis-continuation-2026-10-04/hko-execution/receipt.json`:

- core: `4b82246259a04e5d4c3cc89134bf4a94de6c1899c2abca3fae8efc63119ac403`;
- full verifier: `004ea0d83fd3706f93f934e8eea3897fed2d495c46ec46ce089c65479fb2b2b8`;
- witness: `98362e00408d1bc6b3085b2ed6cc23174c3270c6b0ad6219a00e3a7732f5472c`.

That receipt records both exact programs exiting zero under Sage 10.9 with
optimization disabled; the core log contains the output quoted on p. 103. This
is retained execution of identical computational sources, not a new execution
in this run. No Sage installation or expensive producer rerun was performed.
The fresh checks here parsed the current source, compared all section data and
read the derivative and symmetry constructions. They do not independently
reconstruct the Sage field arithmetic or every rank/positivity calculation.

## Essential Jörn reading contribution

Appendix B adds **no separate line-by-line reading obligation**. Fold this
single check into the Chapter 7 high-risk item:

| Pages | Claim to verify | Why risky | What OK looks like | Minutes |
| --- | --- | --- | --- | --- |
| 44–48 (Chapter 7), with pp. 102–103 as the implementation map | Positive feasible inverse-minor sections give upper bounds touching the true HKO ratio, and the displayed `DU` is their derivative in the full 40-coordinate chart. | The finite rank/positive-relation certificate proves the intended theorem only if these are derivatives of actual touching upper bounds with the correct geometry/sign conventions. | Jörn can trace the capacity inequality, the exact HKO contact value, the volume derivative and one worked coordinate through the displayed product rule, then recognize the same section and chain rule in core lines 40–67. The Chapter 7 reviewer owns the later transverse/equality implication. | 8–12, included in the Chapter 7 item rather than added twice |

Do not require Jörn to retype all 26 assignments, manually repeat the ranks, or
read both full verifiers: those tasks duplicate the exact source/data checks and
retained unchanged-source execution without resolving the human mathematical
acceptance question.
