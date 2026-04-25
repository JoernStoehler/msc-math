# Verification Experiments Try To Falsify The Story

Property:

- higher-value verification experiments act like real falsification attempts
- they try to break the code-to-math story, not merely reconfirm it
- they preserve negative, mismatching, or indeterminate outcomes

Starter read set:

Use these surfaces in this order:

1. `AGENTS.md` for the crate-test versus experiment-validation boundary.
2. `tasks/verify-thesis-done.md`, especially the claim-support,
   references/provenance, and repo-promises final gates.
3. The relevant convention skills, especially `experiment-conventions`,
   `dataset-conventions`, `rust-conventions`, and `formal-math-conventions`
   when the experiment is checking a mathematical implementation claim.
4. The concrete verification experiments, their declared inputs/outputs, and
   the nearby research/formal notes that say what those experiments are meant to
   establish or try to refute.
5. `ROADMAP.md` and `tasks/*.md` for known open validation gaps, stale evidence, or explicitly
   deferred stronger falsification passes.

Checks:

1. Name the claim or correspondence story under review.

2. Name the failure mode that would refute it:
   - code does not implement the intended math
   - numerical shortcut disagrees with exact or alternative backend
   - two overlapping algorithm routes disagree
   - a searched family contains a counterexample the current story says should
     not appear

3. Inspect whether the experiment would fail loudly on that mode:
   - `rg -n "assert!|panic!|exit\\(|mismatch|disagree|passes_validation" <paths>`
   - if it only logs timings, plots, or summaries, it may be confirmation only

4. Check falsification patterns that exist:
   - independent backend or exact fallback
   - overlapping algorithm cross-check
   - adversarial edge-case or counterexample search
   - broad random/structured sweep where a negative result would matter
   - preserved negative, mismatching, or indeterminate outcomes

5. Check whether negative outcomes are preserved rather than silently filtered:
   - inspect artifact schema, analyzer summary, and nearby research/formal text
   - flag stale notes that still cite old failures or old successes

6. Classify the surface:
   - `real falsification attempt`
   - `useful but weak confirmation`
   - `design cannot falsify the claimed failure mode`
   - `stale or missing evidence`
   - `Jörn decision needed on sufficiency`

7. If weak, say what concrete failure mode it still cannot expose.
