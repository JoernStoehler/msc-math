# opportunity-foundations-review

Review date: 22 October 2026. Checkout: `/workspaces/msc-math`, HEAD `9667e351514faf1f00e2d86796c20e9d9b6f2f10`.

**Disposition: four SOLVED backlog items; no SURVIVING finding identified in this scope; no item classified WITHDRAWN.** These are repairs visible in the current source, not new theorem claims or a human acceptance verdict.

Scope: `meta-topics/project-management/thesis-work.md:28–59`. I read all Chapter 2 input files selected by `thesis/chapters/02-preliminaries.tex:12–17` and all of Chapter 4, then checked the relevant Chapter 3 surgery and reconstruction passages. The selected manuscript includes these chapters at `thesis/main.tex:67–69`. I also compared the original exposition assessment at `docs/history/coordination/whole-exposition-assessment.md:124–152`, the imported-theorem audit, and Rudolf's published Definition 2 and Theorem 1.

Severity below records the original exposition concern; **residual severity is none** for each solved item. “Concrete fix” identifies the repair already present and whether any further edit is proposed.

## 1. SOLVED — normalized facet-row terminology

**Severity:** original low/medium; residual none.

**Exact source locations:** `thesis/chapters/02-preliminaries-lagrangian-products.tex:30–36`; prerequisite definition at `thesis/chapters/02-preliminaries-polytope-input-language.tex:4–19`; input order at `thesis/chapters/02-preliminaries.tex:15–16`.

The passage explicitly names a normalized facet row, computes its image under \(J_0\), and defers the velocity identification \(2J_0a_i\) to Chapter 3. The normalized row \(a_i=n_i/h_i\) has already been introduced. Its interpretation agrees with `thesis/chapters/03-generalized-reeb-orbits-polytopes.tex:39–63`, including the exchange of factor coordinates.

**Concrete fix:** the requested replacement of the vague support-row forward reference is already implemented at lines 30–34. Retain that sentence and the preceding definition; no additional edit is proposed.

## 2. SOLVED — Rudolf's usable input/output and local billiard objects

**Severity:** original high; residual none.

**Exact source locations:** `thesis/chapters/04-quadratic-program.tex:337–366` for the normal cone, strong billiard, simultaneous dual polygon, length and theorem consequence; `thesis/chapters/04-quadratic-program.tex:368–392` for the alternating lift, boundary containment, orientation and action normalization.

The source defines both normal-cone relations and the support-function length before use. It states that the planar convex-body theorem includes polygons, supplies an attained capacity-length value, and permits a minimizing pair with at most three bounce points without smoothness or strict convexity. This matches [Rudolf's published Definition 2 and Theorem 1](https://link.springer.com/article/10.1007/s10884-022-10228-0#Sec3). The journal numbering used by the thesis is correct; the source distinguishes strong billiards from weak ones, whose minimum can differ without strict convexity.

The subsequent lift argument locally explains why each leg remains on the product boundary and why reversing the loop converts the convention and gives positive action equal to the billiard length. Thus the previously reported missing length/pairing and boundary bridges are also present.

**Concrete fix:** the requested compact definition and theorem consequence are already implemented at lines 337–366, followed by the boundary/action bridge. Preserve “strong,” the existence quantifier and the stated nonsmooth scope; no expanded imported theorem or further edit is proposed.

## 3. SOLVED — explicit cyclic-word operation before invocation

**Severity:** original medium; residual none.

**Exact source locations:** `thesis/chapters/04-quadratic-program.tex:395–417` for the dual-competitor setting, surgery sentence and block invariant; `thesis/chapters/04-quadratic-program.tex:419–444` for minimality and boundary reconstruction. The invoked operations are proved at `thesis/chapters/03-generalized-reeb-orbits-polytopes.tex:330–349`, `thesis/chapters/03-generalized-reeb-orbits-polytopes.tex:369–390` and `thesis/chapters/03-generalized-reeb-orbits-polytopes.tex:393–408`.

Before invoking the earlier theorem, lines 402–404 say to split a mixed leg into pure directions, transfer repeated-direction duration to one occurrence, delete the other occurrence, and rescale all durations if needed. The cited proof supplies the choice that does not decrease action. The following paragraphs retain the upper bound on factor blocks, prove that dual minimality actually forces no rescaling, and only then reconstruct a boundary orbit.

**Concrete fix:** the requested one-sentence operation is already inserted at lines 402–404. Retain it and its dual/primal distinction; no structural rewrite is proposed. The original assessment at `docs/history/coordination/whole-exposition-assessment.md:144–148` itself treated this as exposition, not a missing logical step.

## 4. SOLVED — positivity, dependence perturbation and both vertex replacements

**Severity:** original medium; residual none.

**Exact source locations:** `thesis/chapters/04-quadratic-program.tex:541–549`, `thesis/chapters/04-quadratic-program.tex:551–579`, `thesis/chapters/04-quadratic-program.tex:581–593` and `thesis/chapters/04-quadratic-program.tex:595–604`.

All three requested bridges are explicit:

- **Positive mass in both factors:** lines 541–545 invoke positive global objective value and explain that support in a single Lagrangian factor gives zero objective. Consequently \(0<t<1\), making both normalized distributions well-defined.
- **Nonnegative dependence perturbations:** lines 586–592 choose a nonzero dependence supported on positive coordinates and give the concrete bound \(0<\varepsilon<\min_{d_i\ne0}x_i/|d_i|\). Both perturbations preserve the equalities and nonnegativity and have midpoint \(x\).
- **Preservation of the positive global maximum:** lines 569–579 state that the first vertex replacement preserves feasibility and cannot decrease the value, while global maximality prevents an increase. The same argument is explicitly repeated for the other factor; the face-to-ambient-vertex implication is also stated.

Lines 595–604 then preserve positivity while maximizing the factor mass at one half and deleting zero weights. The theorem's existence-only scope remains explicit at `thesis/chapters/04-quadratic-program.tex:621–626`.

**Concrete fix:** the requested sentences and explicit epsilon bound are already implemented at the locations above. Retain these bridges; no additional proof insertion, genericity assumption or stronger support claim is proposed.

## Limits and handoff

This was a bounded source/exposition review, with the named external theorem checked against the publisher text. It was not an exhaustive mathematical or implementation audit, a rendered-PDF review, or a test/reproduction run. No repository files were edited, no build was run, and no git mutation command was issued. Existing unrelated working-tree changes were left untouched. No backlog/dashboard edits are proposed by this read-only task.

The two primary obligation files had no working-tree diff from HEAD when checked. Their SHA-256 values were:

- `02-preliminaries-lagrangian-products.tex`: `8e895de3540cc5bc908ca8d69adc81d67c695cfaa94fb7e1b6c70254719e90c5`
- `04-quadratic-program.tex`: `98919a7cce75ee7868eb27931730872ac99684bf3dacb88825e3ee464f947be7`

[review: foundations_message_review]

