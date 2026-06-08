# Rotated Regular Polygons Success Charter

Status: living working charter for the thesis section
`thesis/rotated-regular-polygons.tex`.

This is not thesis prose, not source truth, and not final acceptance authority.
Use it to guide autonomous work and review timing for this section.

## Purpose

Guide autonomous work and review timing for the thesis section on Lagrangian
products of rotated regular polygons.

## Root Objective

Make `thesis/rotated-regular-polygons.tex` improve thesis success as much as
possible, subject to the deadline and Jörn-time constraints.

This objective is not exhausted by any checklist below. Before asking Jörn for
review or calling the section ready, judge whether more autonomous work or
Jörn review has higher expected value for thesis success.

## Thesis Role

This section should contribute a theorem-strength side result about rotated
regular polygon products. It should use empirical regular-product sweeps as
context and motivation, then prove the exact pentagon-product formula with a
credible proof route.

## Reader Outcomes

A mathematically competent reader with finite time should understand:

1. what family of products is being considered;
2. what theorem-strength result is proved for the pentagon product;
3. what is empirical motivation and what is proof input;
4. what is proved by ordinary mathematical argument;
5. what is reduced to finite exact algebraic checks;
6. why the executable Sage run is a trustworthy certificate for those checks.

These outcomes are not the root objective. They are the main reader-facing
conditions through which the section contributes to thesis success.

## Scope And Non-Scope

Scope:

1. broad empirical sweeps for rotated regular polygon products;
2. the exact formula for
   `sys(P_5 x_L R(theta)P_5)`;
3. the proof route for the pentagon formula;
4. the executable-certificate step used for the finite algebraic checks.

Non-scope:

1. The section does not prove a general theorem for all regular polygon
   products.
2. Do not turn the broad empirical sweeps into theorem-strength claims.
3. Do not prove the finite-candidate theorem inside this section; import it
   from the algorithm chapter once written.
4. Do not make the Sage script or full stdout part of the main text beyond
   short explanatory excerpts and exact run facts.
5. Do not include every available empirical artifact. Include only figures that
   solve a reader problem in the thesis section.
6. Do not polish a general publication paper about regular products; this is a
   thesis side-result section.

## Source Truth

1. Active thesis text:
   `thesis/rotated-regular-polygons.tex`.
2. Exact proof source:
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`.
3. Exact proof output:
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt`.
4. Empirical producers and figures:
   `experiments/regular-products/rotated-regular-products/`,
   `experiments/regular-products/pentagon-rotation-empirics/`.
5. Imported theorem dependency:
   `thesis/quadratic-program-algorithm-hk2019.tex`, once the finite-candidate
   theorem is written there.
6. Thesis-level acceptance authority:
   `tasks/definition-of-success.md`, Jörn decisions, and later Kai/Elizabeth
   feedback where theorem strength or thesis readiness depends on them.

## Authority Boundaries

1. Codex may edit prose, figures, listings, and local explanatory structure.
2. Codex may verify source consistency against code, stdout, data producers,
   and the PDF build.
3. Codex may recommend review targets and likely remaining risks.
4. Jörn decides whether the exposition is acceptable, whether review timing is
   worth his attention, and whether caveats are acceptable for the thesis.
5. Jörn/Kai/Elizabeth may be required for final theorem-strength acceptance,
   especially for the imported finite-candidate theorem and the trust level of
   the executable certificate.

## Working Criteria

1. The title and opening match the content: broad rotated regular products are
   introduced, and the pentagon theorem is identified as the theorem-strength
   specialization.
2. The empirical figures are included only when they solve a reader problem.
   Captions must distinguish empirical motivation from proof input.
3. The theorem proof has one coherent path:
   symmetry reduction, active branch, finite-candidate reduction, executable
   certificate, continuity, return to all real theta.
4. The text must not say or imply that Sage proves the theorem by itself.
   It should say that the mathematical reduction leaves finite exact checks and
   the Sage run certifies those checks.
5. The finite-candidate theorem dependency is explicit.
6. The combinatorics are unambiguous: q/p blocks, block count versus raw facet
   entries, raw sigmas, and transition pruning are explained.
7. Code listings have line numbers and syntax highlighting, and are included
   only when they help the reader trust or understand the proof step.
8. The PDF builds cleanly and has no relevant broken references, overfull boxes,
   or listing-rendering problems.
9. The section makes the role of each included figure clear:
   broad empirical context, pentagon parameter setup, pentagon curve
   motivation, orbit intuition, or branch-action diagnostic.
10. The proof can be read without consulting the Sage source, while the source
    paths and listings make the executable certificate auditable.

## Review Readiness

Ask Jörn for review only if the expected value of his next review exceeds the
expected value of continued autonomous cleanup.

Before asking, state:

1. what rendered PDF excerpt to review, using section titles or quoted text
   anchors rather than source paths or page ranges;
2. the exact question to answer;
3. the highest-value uncertainties;
4. what autonomous checks were already done;
5. why review now is better than more Codex work.

Do not ask Jörn to review source files or page ranges unless he requested that.

If the next likely feedback is about obvious missing work that Codex can find
or fix alone, do not ask yet.

## Final Self-Review Before Asking Jörn

Before asking Jörn for review, perform a chapter-level judgment, not only a
checklist pass:

1. Does the section, as rendered, improve thesis success more than it costs in
   reader time and thesis space?
2. Would a finite-time reader understand what is claimed, why it matters, and
   why the proof route is credible?
3. Are the remaining uncertainties ones where Jörn has higher value of
   information than further autonomous cleanup?
4. If every working criterion above is satisfied, can the section still be bad?
   If yes, identify that failure mode before asking for review.

## Known Failure Modes

1. Treating the latest local patch as chapter readiness.
2. Confusing source-truth audit with reader-ready exposition.
3. Letting scaffold subsection structure drive the final proof structure.
4. Saying "Sage proves" instead of "finite checks are certified".
5. Ignoring the broad regular-product scope implied by the section title.
6. Including figures or listings because they exist rather than because they
   solve a reader problem.
7. Asking Jörn for review before autonomous cleanup has lower expected value.
8. Writing a success charter reactively and then optimizing the charter instead
   of the thesis section.
9. Treating a broad checklist as proof of readiness instead of making a
   chapter-level judgment.

## Update Rule

Update this file when new feedback or source checks change future decisions.
Do not update it just to record that work happened.

Before relying on this charter after major restructuring, check it against
`tasks/references/writing-success-charters.md`.
