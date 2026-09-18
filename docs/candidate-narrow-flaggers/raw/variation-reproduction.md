# Variation — reproduction-detail review

Review start (UTC): 2026-09-18 13:06:31 UTC
Prompt: /workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/prompts/reproduction-detail-prompt.md
Complete reviewed input: /workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/variation.tex
Structure/notation context: /workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/main.tex and /workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/preamble.tex

## Anchored concern

1. **Anchor:** “What the first-order model contributes to search,” the following span:

   > A later comparison tests seven fixed policies on sixty-four matched starts,
   > including policies based on finite-gap and branch-history models. This is a
   > separate experiment on a historical numerical objective, not an enlargement
   > of the twelve-start panel. Its inputs and results are recorded in
   > \path{experiments/dev-gradient-ascent/optimizer-comparison/}.

   **Reading cost:** After the twelve-start experiment has established finite-step progress without endpoint maximality, this introduces another experimental population, policy set, and objective but supplies no result from that comparison. The reader must track a second study without learning what it shows about the first-order model. Its chronological placement and artifact pointer chiefly inventory retained work.

   **Minimal relocation:** Move this span together to a methods or experiment note. Keep its historical-objective and separate-panel qualifications attached to the comparison there.

   **Mathematical understanding lost:** The reader would lose notice that finite-gap and branch-history policies were also tested, but no stated outcome or inference about their performance. The distinction between the two populations matters if both are discussed; moving this entire span preserves the twelve-start evidence and avoids needing that distinction here.

   **Confidence:** High that the span currently contributes experimental inventory rather than an explanatory result; moderate that relocation is preferable without seeing the comparison's role elsewhere.

   **Context that would justify retaining it here:** A specific comparison finding used to explain whether the finite-gap model improves search, with these sample and objective qualifications necessary to interpret that finding. No such finding is stated in the supplied passage.

## Details worth retaining

1. **“twelve random ten-facet starts,” “eight configured trace iterations,” and the report that every endpoint scan found a further above-threshold move.** These delimit the finite evidence and explain why iteration-cap termination supplies no endpoint local-maximality evidence. Moving these restrictions away would weaken the reader's ability to interpret the progress claim.

2. **The three-case diagnostic's smaller-step differences approaching analytical slopes while the volume calculation remains consistent.** These observations support the explanation that the tested finite-step branch prediction can fail despite correct infinitesimal slopes, and motivate conditioning-dependent step scales. They are evidence for the mathematical explanation, not merely debugging history.

3. **The distinction between regression checks and the HKO verifier's exact checks in the relevant algebraic number field.** Together with the stated limits of the regression checks, this distinguishes implementation agreement from the exact derivative evidence used by the upper-bound argument. The subsection already identifies itself as implementation and validation status; retaining that distinction there makes the computational support interpretable without attributing branch coverage or local maximality to finite-difference agreement.

Review end (UTC): 2026-09-18 13:07:10 UTC
