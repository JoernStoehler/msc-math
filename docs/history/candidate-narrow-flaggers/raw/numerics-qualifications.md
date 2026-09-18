# Numerics: qualifications review

Start UTC: 2026-09-18 13:06:24 UTC  
End UTC: 2026-09-18 13:06:36 UTC

Prompt: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/prompts/qualifications-prompt.md`  
Complete reviewed input: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/numerics.tex`  
Supplied context: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/main.tex` and `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/preamble.tex`.

## Concerns, ranked by reader cost

1. **Uncertain concern: the concluding repetition of scope.**

   > The guarantees above concern the exact dyadic polytope represented by the
   > input.  They do not propagate measurement error or prior rounding uncertainty
   > in the coordinates, and they apply only within the stated facet, norm, and
   > candidate-count limits.

   The preceding paragraph raises the relationship between development-route tests and production behavior. This passage instead repeats the input interpretation and production limits already stated in the contracts subsection. Removing it would not change those contracts or invalidate an inference established here. Minimal change: delete these three sentences, retaining the original input and applicability statements. This is uncertain because a concluding scope reminder can be useful after a dense section, especially if readers consult the empirical results independently. It would also earn its place if the nearby application concerned measured coordinates and risked confusing numerical certification with control of measurement error.

2. **Likely concern: denying a theorem that the local passage has not proposed.**

   > not a theorem
   > about all polytopes.

   The preceding sentences explicitly describe what the production route accepts. They raise the reason for its facet and norm restrictions, which the next sentence supplies; they do not suggest a mathematical restriction on the class of polytopes. Minimal change: “These bounds are a deliberately narrow applicability policy.” Preserve the following explanation about the scales for which the enclosures have been justified and tested. The deleted denial would be useful if a surrounding mathematical theorem used the same bounds and the reader needed to distinguish that theorem's hypotheses from implementation limits.

3. **Likely concern: repeating the already explicit fallback distinction.**

   > Exact arithmetic is therefore a selective correctness fallback,
   > not the default arithmetic for every word.

   The paragraph has already said that each surviving word uses the verified fixed-word decision, with exact fallback for every indeterminate case. Its remaining question is how the objective intervals give the capacity enclosure, which the immediately preceding sentence answers. The final sentence reopens an arithmetic-policy alternative without adding to that argument. Minimal change: delete it. No inference about aggregation or fallback is lost. It would be useful in a performance discussion comparing this method with an all-rational baseline, or as a replacement summary if the earlier fallback description were removed.

## Qualifications worth retaining

1. > it does not
   > claim to recover unavailable source coordinates from which they may have been
   > rounded.

   This fixes the object of certification: the exact dyadic input, rather than an unknown pre-rounding polytope. Without this distinction, a reader could apply the interval guarantee to the wrong geometric object.

2. > It intentionally does not enumerate all minimizing geometric branches.

   The preceding sentence promises every tied sparse winner in a particular candidate family. This qualification blocks a materially stronger interpretation of “every”: complete enumeration of geometric minimizers. It defines the output's scope rather than defending an unrelated choice.

3. > this is finite regression
   > evidence, not all-input equivalence.

   The audit concerns selected development routes, while the chapter describes production guarantees. Four fixture comparisons cannot establish equivalence of those implementations on all inputs. The qualification directly limits what the reported correspondence check supports.
