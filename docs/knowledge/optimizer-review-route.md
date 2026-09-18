> Retained source-interpretation knowledge, not a current assignment or fresh audit.
> Original text: `ee282e065004adc1ccd64e51140edccd075845de:docs/history/memories/optimizer-review-route.md`. Original source routes below describe the September 5 source state; current selected source is `thesis/chapters/07-hko.tex` (HKO) or `thesis/chapters/08-data-science.tex` (DS).

# Pilot: reviewing the seven-policy optimizer comparison

This pilot preserves a bounded source-to-thesis interpretation for authors and
reviewers, not an independent validation of the experiment. Inspected
2026-09-05 at repository HEAD `3a00abcc6ada31341b7064c98bf861076857c5da`;
the working-tree source text was read, so HEAD is a recovery baseline, not a
claim that every inspected byte was committed. Current sources override this
entry. The task boundary and Jörn's exclusion of additional optimizer
comparisons are owned by [current work](../WORK_REMAINING.md).

## Claim and shortest useful route

Start with [the active finite-budget subsection](../../thesis/chapters/08-data-science.tex),
label `subsec:black-box-datascience-finite-budget-optimization`, opening
paragraph and table `tab:black-box-datascience-f10-optimizer-comparison`.
The claim is that four-anchor history had the largest terminal median on 64
matched F=10 starts among seven fixed implementations; no recorded run reached
the evaluator threshold 1. It is explicitly limited to this population,
allocation rule and historical evaluator.

The [comparison README](../../experiments/dev-gradient-ascent/optimizer-comparison/README.md)
owns the retained-packet route and stopping/evaluator explanation. Within
`experiments/dev-gradient-ascent/optimizer-comparison/artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/`:

- `final-summary.csv`: `n`, `median_final_sys`, `q10_final_sys`, and
  `q90_final_sys` support the table's counts, medians and distribution ranges.
- `SUMMARY.md`: the termination table supplies maxima and counts reaching
  `sys >= 1`; it also records the analyzer's validation assertion.
- `analysis-provenance.json`: identifies analyzer and input hashes, linking
  the analysis to its resolved plan, producer provenance and four raw tables.

The non-obvious name mapping is `history-baseline` → four-anchor branch
history; `directional-above-8e-2` → history with transition prediction;
`gap-w1e-1-adaptive-d1e-1` → finite-gap affine model. The other IDs are
`literal-eta1e-2`, `safeguarded-adaptive-d1e-1`, `cma-s1e-1-l8`, and
`pattern-r3e-2`, corresponding to the literal gradient, safeguarded gradient,
CMA-ES and coordinate-search rows.

The [held-out manifest](../../experiments/dev-gradient-ascent/optimizer-runs/manifests/heldout-f10-64-finalists.json)
owns exact configurations and start-selection instructions: four random F=10
prefixes, offset 34 and 16 starts per prefix; 128 charged calls, 1000 ms,
threshold 1, uncharged initial evaluation, serial execution and evaluator
flags. Actual resolved identities belong to the resolved plan, not merely
this selection recipe.

## Reasoning needed to use those objects correctly

The terminal statistic is best-so-far historical evaluator value, not
necessarily the optimizer's final selected state. The common 1000 ms limit
allows work to start below the cutoff and finish atomically above it, including
`tell`; an empty final `ask` may add time without a new value. Early algorithm
termination and the call cap also affect realized compute. Consequently this
is a common allocation rule, not a comparison uniformly truncated at exactly
1000 ms. The README owns the detailed overrun counts and mechanics.

The JSON name `sys` means the thesis's estimated quantity
`hat(sys)`, not certified mathematical systolic ratio. Manifest flags specify
f64 geometry/volume, acceptance of indeterminate geometry and no exact fallback.
The thesis explains the historical legacy search and untested candidate-family
completeness. A common evaluator permits the recorded comparison but does not
ensure equal error at different visited points or establish the mathematical
ranking. Those evaluator explanations were read, not independently code-audited.

`held_out` is a declared role. Neither it nor the producer's clean Git state
establishes tuning disjointness or configuration-selection history. The thesis
reports later development reuse of 16 starts; that reuse was not independently
checked here and alone does not demonstrate selection leakage.

The 10–90% ranges describe variation across starts, not uncertainty about the
median. The nearby paired median advantage is also distinct from subtracting
the two group medians; its support is `paired-final-comparisons.csv`, which
was not inspected in this pilot.

## Checked layer and recoverable dependencies

Read the active TeX and exactly six support files: comparison README,
`final-summary.csv`, `SUMMARY.md`, `analysis-provenance.json`, held-out
manifest, and retained `run-provenance.json`. The seven rounded medians and
quantile ranges agree with the CSV; maxima and zero-threshold counts agree
with the generated summary. The summary asserts strict validation of 448
runs, 28,824 evaluations, 28,376 proposals and 16,338 rounds. This is
summary-to-TeX agreement, not raw-data validation or an endorsement of the
result. No raw packets, code, full datasets, builds, scripts, tests,
computations or hash validation were performed. Endpoint controls,
convergence diagnostics and independence were not audited.

The retained [producer provenance](../../experiments/dev-gradient-ascent/optimizer-runs/artifacts/heldout-f10-64-finalists-19a8b4dfd/run-provenance.json)
declares clean producer commit `19a8b4dfd988779e4b29f759710565b0b57edb65`,
executable BLAKE3, manifest BLAKE3 and resolved-plan hash. Analysis provenance
records analyzer SHA256
`da87351c9cc186f0c44f8abc25eae2268fb611e429d08478b2ad43fdfe96869b`
and six input hashes. Its absolute analyzer path is a historical worktree path;
it is not a portable reproduction command. Those declarations were read,
not recomputed, and a complete runnable reconstruction was not established.

Reopen only the relevant layer when the question changes:

- Changed wording or numbers: reopen the TeX and the specific summary fields.
- Regenerated packet or changed analyzer/input hashes: reestablish aggregation
  and provenance before reusing this linkage.
- Changed budget semantics, evaluator, parameters or starts: consult the
  producer revision and resolved plan; current implementations do not
  retroactively describe the frozen run.
- Independence question: seek selection/tuning history; the role flag cannot
  answer it.
- Paired-effect question: inspect `paired-final-comparisons.csv`; endpoint or
  local-maximality questions require the separate diagnostic sources linked
  by the README.

The maintained documents own values, contracts, configurations and provenance.
This pilot owns the cross-source interpretation and checked-layer boundary;
it deliberately does not duplicate the result table or establish a new task.
