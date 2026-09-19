Review started: 2026-09-18 13:06:15 UTC
Review ended: 2026-09-18 13:06:26 UTC

Prompt: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/prompts/reproduction-detail-prompt.md`
Complete reviewed input: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/numerics.tex`
Structure/notation context: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/main.tex` and `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/preamble.tex`

## Anchored concerns

1. **Span:** “It first uses a cheap induced norm bound and retries an indeterminate system with a tighter entrywise bound.”
   **Reading cost:** Interrupts the connection between the inverse-defect certificate and its arithmetic assumptions with the scheduling of two implementations of the bound.
   **Minimal relocation:** Move this sentence to a methods note. Keep the surrounding roundoff treatment, arithmetic assumptions, and exact fallback in the mathematical account.
   **Mathematical understanding lost:** The reader would no longer know why some cases avoid exact fallback after a second numerical attempt. Neither the certificate's validity nor the meaning of an indeterminate decision depends on this retry order, so that loss is minor here.
   **Confidence:** High.
   **Context that would justify retaining it here:** A nearby comparison explaining computation time or attributing fallback rates to the two successive bounds would make this operational detail relevant to the evidence.

2. **Span:** “A compiler, dependency, target, or arithmetic-contract change therefore requires the floating-point certificate tests to be repeated.”
   **Reading cost:** Turns a paragraph about the assumptions supporting a numerical proof into an instruction for future software maintenance.
   **Minimal relocation:** Move this sentence to the methods or reproduction note, retaining the preceding explicit arithmetic and evaluation-order assumptions.
   **Mathematical understanding lost:** No stated assumption or error bound would disappear. The maintenance consequence of those assumptions would become less immediate, but the reader would still know the conditions under which the certificate applies.
   **Confidence:** High.
   **Context that would justify retaining it here:** If this paragraph also specifies how readers can determine whether an available executable satisfies the stated assumptions, the maintenance instruction could help delimit the guarantee they are being offered.

3. **Span:** “The test source is
   `\path{experiments/dev-quadratic-program/tests/selected_route_correspondence.rs};`
   its execution and fixture scope are recorded in
   `\path{docs/numerics-correspondence-check/README.md}.`”
   **Reading cost:** Two long repository paths interrupt the closing discussion of what finite correspondence checks establish.
   **Minimal relocation:** Move the paths into a footnote or methods reference attached to the correspondence-check paragraph. Retain the four checked quantities and the explicit limitation to finite regression evidence in the body.
   **Mathematical understanding lost:** None from the account of the checks themselves; only immediate source discoverability would be reduced, and a footnote would preserve it.
   **Confidence:** Moderate to high.
   **Context that would justify retaining it here:** An intentional convention of giving direct source locations alongside every computational claim would support keeping these references locally, although a footnote would still serve that convention.

## Details worth retaining

1. **The exact-dyadic input contract and the facet, vertex-norm, and candidate-count limits.** They identify the object whose capacity is enclosed and the actual domain on which the implementation promises a result. Without them, a reader could incorrectly infer guarantees for underlying unrounded coordinates or arbitrary polytopes.

2. **The audit populations, adversarial conditions, indeterminate outcomes, and exact comparisons.** The 249-system audit, near-singular cases, and finite general/product comparisons delimit the empirical evidence. The raw-sign disagreements and certified indeterminacy explain why a floating-point sign alone is inadequate; the finite-evidence qualification prevents these tests from being mistaken for an unrestricted proof.

3. **The distinction between selected development routes and production, including the four-test correspondence check and its limitation.** The supplied text attributes substantial audits to development routes. This distinction is necessary for inferring how much of that evidence transfers to production: four passing fixture checks support finite correspondence, not all-input equivalence. The route distinction should survive any simplification of its project-oriented wording.
