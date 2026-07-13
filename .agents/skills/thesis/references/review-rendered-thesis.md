# Review The Rendered Thesis

Use this reference when source changes can affect the final PDF or when a gate
depends on the actual reading surface.

Build `thesis/build/main.pdf` from the current worktree and run
`thesis/check-build.sh`. Verify that the inspected PDF belongs to the candidate
source under review. Never substitute a PDF from Main, another worktree, or an
unverified cache; if the candidate cannot be built, state that rendered review
is unavailable.

Inspect the changed pages at normal whole-page reading scale together with
neighboring pages. Check equation breaks, theorem placement, labels,
references, citations, captions, floats, whitespace, page competition, and
whether visual information is perceptible without unusual zooming.

Confirm that the PDF includes the reviewed source and intended asset copy. A
clean build establishes only the conditions checked by the build scripts; it
does not establish mathematical correctness, reader understanding, asset
freshness, or publication readiness.

After structural reordering or substantial rewriting, reconsider earlier
reviews whose object or context changed. Preserve expensive, non-obvious
rendered findings with the owning companion or asset; do not accumulate
routine build narration.
