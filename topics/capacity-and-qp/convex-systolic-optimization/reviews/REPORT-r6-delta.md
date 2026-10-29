# Assembled mathematical review — final r6 analytic appendices and two-hinge obstruction

Reviewer `/root/assembled_math_review`, 29 October 2026. Read-only review; no new agents, canonical edits, or expensive reruns.

**PASS for the final 56-page r6 mathematical integration**, together with `REPORT-r2.md`, `REPORT-r3-delta.md`, and `REPORT-r4-delta.md`. No mathematical blocker remains. This final report subsumes the provisional r5 delta.

Exact reviewed objects:

- `paper.pdf`: `1b2c6753f72e9b726c20f7fad70201c9053523f52c7f94277eee2c8061c52ed8`.
- `main.tex`: `73f6ea49b426871125d000ab0c7d0aebd0f104a1c716d8e8919396bb72c49861`.
- `sections/hko-effective-step.tex`: `e23b2b0261860564a8e8839a97b0b96cfaf684333b3d76d505bf7ebf59ff666e`.
- `sections/hko-two-hinge-main.tex`: `506c59e1bcc30c7b1bbd2489f895ea14026f2d48b348b183953d7291762c68d6`.
- `sections/hko-two-hinge-appendix.tex`: `87c831418a95b73f50fc55fb4192a7027b94ff08db8235e3deaf775ea30ac598`.
- `reviews/hko-two-hinge-mathematics.md`: `48564dbdac10b27c8af68ccd26f04d6b7ae6f8be0ee20c719958f57d9c3e882f`.

`reviewed-r6/` preserves these exact inputs, all other paper sections, the new finite packet, quantitative proof notes, and the extracted PDF text. `reviewed-r6-hashes.json` pins every copied file. The final HKO two-hinge packet manifest was checked successfully.


This review extends the complete r2 review and r3/r4 delta reports. Compared with reviewed r4, the mathematical changes are the expanded Appendix B and the new two-invariant-hinge proposition with complete Appendix C. The finite-parameter theorem and seventeen-facet local result are unchanged. The exact source/PDF manifest above pins the integration.

## Appendix B: complete quantitative near-zero proof

I read the complete printed Appendix B source against the already audited `effective-step.md` and `minor-bound.md`, checked its rendered PDF text, and rechecked the logical interfaces and constants. The conversion now includes the complete analytic bridge that the shorter r4 PDF delegated to its companion:

- The moving-word residual estimate treats retained rows as having zero velocity. The positive-action bound forces both factor masses away from zero; the nearly closed pentagon distributions yield three sufficiently weighted distinct directions per factor. Two q directions and three p directions provide the determinant and inverse estimates.
- The fixed positive pivot repairs an actual moving global maximizer to a base-feasible probability vector. The differentiated constraint equations give the stated chart derivative, Taylor remainder and derivative-Lipschitz bounds, including on the larger auxiliary interval where positivity is unnecessary for those estimates.
- The primitive action gap bounds the bad triangle-pair product mass. Deleting small marginal coefficients leaves only winning pairs. The quantified distance to an exact maximizing vector preserves every pivot coordinate, so its fixed-pivot chart is a feasible continuation and the all-word base response bound legitimately applies.
- Completing the square absorbs the loss from near-optimality into the quadratic remainder for the true global action. The independent radial-volume upper bound has the correct direction; multiplication and reciprocation give the stated relative systolic improvement for every `0<t<=10^-30`.

The full argument retains the essential restrictions: the twenty-row bank contains two copies of each base normal, the global maximizing word is simple in these twenty labels, and the effective parameter statement concerns the original all-ten-unit-cut family. It does not transfer the finite gain to the seventeen-facet variants or establish monotonicity up to the separate `t=3/10` example.

The finite primitive action gap and all-word response certificate remain clearly named computer-assisted inputs. Printing the analytic proof does not remove those finite evidence dependencies or claim that a LaTeX build validates them. This review relies on their earlier audits/replays and does not claim a new full computation.

## Corrections made during this delta

The editor accepted three harmless inequality repairs: the third-largest pentagon mass is at least `r/3`, the residual comparison at `t=0` is non-strict, and the rounding bound uses `<=2000 sqrt(d)` to include `d=0`. The last repair is also applied to the retained quantitative Markdown. The final explicit `simple word` restriction in Lemma B.2 makes the lemma's occurrence bound match its proof and its sole application. None changes constants or the proposition.

## Appendix C: two characteristic-invariant hinges

I read the full final author TeX module and `amplification-proof.md`, the independent analytic audit, and the portable four-family checker. I reviewed the scalar proof over every threshold interval, including zero slope, zero hinge, full-support affine cases and the `z=1` endpoint excluded from division. The secant, perfect-square and concavity comparisons have the correct directions. The projected density has the stated normalization and mean; the product facet measure supplies independent edge and partner coordinates, and cone-volume contributes the required factor `1/10`.

The four families only provide lower bounds on the response supremum. Their existence is sufficient; neither completeness of the 190 charts nor a common responding occupation ensemble is assumed. At the zero-weight endpoint, the segment contributes zero and the limiting occupation measure is permitted. The piecewise weight curves may differ at their shared coordinate because they belong to different valid families.

Convexity in the edge coordinate bounds the volume mean by the average of the two edge means. Each scalar edge mean is at most five times its own response supremum, hence at most five times the common all-word response `R(f)`. This proves `R(f)>=2 nu(f)`. For two different labeled facets, the retained-row splitting profile is `f+g`; for two rows on the same facet it is `max(f,g)`. Nonnegativity and the single-hinge bound give `R(profile)>=nu(profile)` in both cases. The existing fixed-bank derivative theorem consequently gives nonpositive right logarithmic systolic variation. Taking one hinge zero also covers fewer than two rows.

The scope must remain characteristic invariance `D·J a_i=0`, all original rows retained, and a first-order conclusion. Neither a finite-neighborhood result, local maximality among all twelve-facet polytopes, non-invariant hinges, nor higher-order behavior in zero-derivative directions follows. The reviewed integration states these limits explicitly. The main result is Proposition 4.7, linked to complete Appendix C; its references resolve in the actual PDF. The introduction and concluding scope paragraph match that statement. Appendix B is Proposition B.1 with the corrected simple-word Lemma B.2. The earlier HKO finite theorem, weighted seventeen-facet statement, other sections and bibliography are byte-identical to reviewed r4.

### New independent exact check

`check-two-hinge-literals.py` uses my standard-library field arithmetic from `check-weighted-witness.py`, with no imports from the submitted implementation. It checks the literal ordered words and endpoint weights printed in the paper, including their orientation so the quotient parameter increases. It independently verifies exact normalization, nonnegativity, closure, action `tau/5`, full KKT equations, all segment endpoint facet equalities and inequalities, both edge coordinates, the affine partner coordinates, and the occupation-weight laws at endpoints and midpoint. Affinity/quadratic degree then supplies the stated interpolation argument.

The same independent script verifies the scalar perfect-square identity, all printed endpoint comparisons, concavity sign, upper-density ratio, and density normalization and mean. Signs use rational isolation of the specified algebraic root, not floating-point tolerances. The run passed. `two-hinge-literals.out` and `two-hinge-independent-receipt.json` pin the execution and input hashes; the helper file is a required local dependency and is retained with the checker. This supplements the primary portable certificate rather than replacing the analytic argument.

The final author TeX hash is `e20b6fc6a49ac3cf04cf56299249ff667d00e0e6234c6c501faa21cba1674d94`. The final author Markdown hash is `7fcf285f7627e57459de71aea5f97591f7bb17325c270fb1f66ba4f772265247`; the freshly read independent audit also pins this final version and explains the endpoint/same-facet updates. An earlier stale audit copy should not be interpreted as the final provenance record.

Root owns final reader/visual review, retained report integration, source-and-evidence archive assembly, status, commits, and delivery. This review does not cover later research claims or source changes.

## Final retained-row summary clarification — 01:14 UTC

**PASS carries forward to the final r6 delivery PDF.** The only TeX change since the above freeze is in the §10 scope paragraph: the two-hinge summary now says “while adding two arbitrary characteristic-invariant affine hinges and retaining the original facet inequalities cannot give positive first variation.” This makes the summary's hypotheses explicit and agrees with the already reviewed proposition and Appendix C. Every section source and the bibliography are byte-identical to the above freeze; no proof or other claim changed. I checked the exact main-source diff and the revised sentence in extracted text from the rebuilt 56-page PDF.

Final delivery hashes, superseding the earlier PDF/main hashes above:

- `paper.pdf`: `a87d422c1ee43ca10454b5c7fa25eeb3f35f1b9ea424734aa501fe6c7e13f0b3`.
- `main.tex`: `74fc391bbf627e640127cbd2565e5a1c63d9f20a9f69f5fe0a34164fc335b27c`.

`reviewed-r6-scope-final/` preserves the replacement main source, PDF, extracted text, and hashes. Together with the unchanged `reviewed-r6/` inputs and manifest, it pins the final reviewed mathematics. No new mathematical issue or additional proof review was required for this hypothesis clarification.
