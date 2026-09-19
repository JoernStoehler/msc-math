# Introduction: qualifications review

Review start (UTC): 2026-09-18 13:06:04 UTC

Review end (UTC): 2026-09-18 13:06:37 UTC

Prompt: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/prompts/qualifications-prompt.md`

Complete reviewed input: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/introduction.tex`

Supplied context: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/main.tex` and `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/preamble.tex`.

## Concerns, ranked by reader cost

1. **Likely concern:** “without classifying every minimizing orbit.”

   The preceding claim is existential: some maximizing word uses at most three facets from each factor. It raises the question of how this reduces the capacity search, which the first half of the sentence answers. The appended qualification introduces exhaustive orbit classification as a separate undertaking without explaining why the reader would expect it. Removing it does not change the stated existential result or imply a classification of all minimizers.

   Minimal change: end the sentence after “capacity”: “This gives a smaller finite search for the capacity.” The qualification would be useful where an earlier argument had explicitly required classifying all minimizing orbits, or where the text compared this method with a classification-based approach.

2. **Uncertain concern:** “but does not include arbitrary pentagons.”

   The paragraph has just specified independent linear deformations of regular pentagons. That positively states the family to which the extension applies; the reader's immediate question is the scope of the extension from rotations, rather than whether every pentagon has been covered. The appended denial repeats a boundary implicit in that precise family description. Deleting it leaves the capacity formula and sharp bound restricted to the stated family.

   Minimal change: “This family is larger than the rotation family.” Alternatively, omit that whole final sentence if the preceding description suffices. The denial would be useful if the surrounding chapter introduced arbitrary pentagons as the next target, or if a preceding informal description had called this a result for pentagon products without specifying regularity and linear deformation. Because the introduction asks about other families with large ratios, a brief scope reminder here is plausibly helpful; this concern is weaker than the first.

## Qualifications worth retaining

- “Allowing facets to appear or disappear is a different local problem, which
  the theorem leaves open.” The motivating question concerns local maximality among convex bodies, whereas the reported theorem concerns nearby ten-facet polytopes. This qualification prevents the reader from treating the theorem as an answer to the full motivating question, especially after the informal sentence about a “small change of shape.”

- “In a smaller neighborhood, equality occurs precisely
  for translations, positive dilations and linear symplectic images of
  $K_{\rm HKO}$” specifies both the neighborhood restriction and the equality cases. These are needed to interpret the ensuing strict decrease claim modulo symmetries.

- “The experiments
  find patterns below systolic ratio one, but do not establish their extension
  above one.” This gives the evidence boundary for the statistical observations in an introduction centrally motivated by ratios above one. Removing the limitation would leave the relevance of those observations to the counterexample regime ambiguous.
