# Archived active-support resampling smoke

The complete active-support conditional-resampling smoke was preserved in Git
commit `2fcd3843` (`Add active-support resampling smoke`) and then removed from
the final tree during this wrap-up.  That commit contains the complete former
directory
`experiments/sys-datascience/methods/product-bounce-active-resampling/`,
including Rust and Python sources, the reviewed smoke artifacts, provenance,
interpretation, and review.

The smoke compiled and ran successfully.  It established the bounded plumbing
and failure-layer gate for four selected retained random `5x5` bases under two
conditional resampling laws: 16 accepted proposals per base and law, 128 exact
target evaluations, exact fixed-word action agreement in all recorded rows,
and the intended separation of candidate-stream, exact-action, and f64
recovery diagnostics.  It was not broken.

It is not retained on Main because it is a stopped, four-base feasibility
smoke rather than current evidence for an inactive-facet mechanism or a
two-/three-bounce class effect.  The retained mechanism and width packets give
the active current route: the mechanism packet resolves the cheap existing-row
question, and the width packet plus
`formal/product-two-bounce-class.tex` resolves the two-bounce component.

To inspect the archived packet without changing the current tree, run

```bash
git show 2fcd3843:experiments/sys-datascience/methods/product-bounce-active-resampling/README.md
git ls-tree -r --name-only 2fcd3843 -- experiments/sys-datascience/methods/product-bounce-active-resampling
```

To recover it into a new branch or worktree, restore that directory from
`2fcd3843` with `git restore --source=2fcd3843 --
experiments/sys-datascience/methods/product-bounce-active-resampling/`, then
reassess the archived provenance and bounded interpretation before reuse.
