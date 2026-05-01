Status: complete
Idea slug: regime-classification / M012
Objective: Decide whether endpoint-vs-random regime classification from the current feature tables gives a thesis-usable observation, a caveated supporting-only observation, or a result to omit before submission, while separating provenance/metadata separation from non-provenance geometric or orbit-feature separation.
Base dataset snapshot: /tmp/sys-ds-reset-pilot-tables-VJ6D0P, produced by `cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P`
Dataset filtering/subsetting: No rows filtered; all 282 observations were used. Endpoint label is `gradient_ascent_general`, `gradient_ascent_products`, or `variable_f_ascent`; random label is `random_sample` or `random_product_sample`. Grouped CV uses `root_group_id` when present, with `source_name`/`lineage_id`/`observation_id` fallback from `common.py`.
Command run: `uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py --dataset-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P`; `uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py --dataset-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P`
Verdict: negative
Evidence strength: Moderate diagnostic evidence only. Grouped-CV metrics are far above null for both metadata and non-provenance geometric/orbit blocks, but the result is a regime-separation diagnostic, not a target-case search rule.
Implementation trust: Medium. Dataset guards and feature row counts match the reset packet, the script now writes a markdown summary, and grouped CV blocks direct duplicate-lineage leakage; no permutation or independent fresh-table replication was run.
Thesis/project use: supporting/caveat only
Caveat: The clearest separation is provenance metadata, and even non-provenance separation distinguishes how the current dataset was generated rather than giving a direct recipe for finding `sys > 1` examples.
Reopen trigger: Reopen if a fresh/larger table changes the dataset guards, contains any `sys > 1` row, or someone proposes a generator-side sampling rule derived without endpoint labels, dataset identity, optimizer provenance, or post-hoc target inspection.
Evidence paths: `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py`; `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime_classification_summary.md`; `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime_classification_bars.png`

## Command/Provenance

Dataset snapshot checks on `/tmp/sys-ds-reset-pilot-tables-VJ6D0P`:

- polytope rows: `282`
- observation rows: `282`
- polytope union field count: `135`
- observation union field count: `53`
- observation dataset counts: `gradient_ascent_general=10`, `gradient_ascent_products=12`, `random_product_sample=100`, `random_sample=70`, `variable_f_ascent=90`
- maximum `sys`: `0.906316153431123`
- rows with `sys > 1`: `0`

The feature-refresh command regenerated the local feature tables from the same dataset snapshot; each feature table had `282` rows after the run. Only the regime-classification script, summary, report, and regime-classification bar plot remain as tracked changes from this worker.

The classifier command used all `282` joined observations and `212` grouped-CV groups. No `summary.json` or other machine-readable sidecar is required.

## Observations

Null baseline, with no features, is balanced accuracy `0.5000` and ROC AUC `0.4946` for both classifier families.

Provenance-heavy metadata separates regimes perfectly:

- logistic `provenance_metadata`: balanced accuracy `1.0000`, ROC AUC `1.0000`
- random forest `provenance_metadata`: balanced accuracy `1.0000`, ROC AUC `1.0000`
- full `metadata` and `all` also score `1.0000`, but those blocks include dataset/family/role/search-space/optimizer/backend and should not be read as geometry evidence.

Non-provenance geometric/orbit blocks also separate the current regimes above null:

- logistic best geometry/orbit block: `skeleton`, balanced accuracy `0.8728`, ROC AUC `0.9223`
- random-forest best geometry/orbit block: `face_geometry`, balanced accuracy `0.9275`, ROC AUC `0.9831`
- random-forest `geometry`, `face_symplectic`, and `omega` are also high: balanced accuracy `0.9200`, `0.9155`, and `0.9170`
- `facet_count` alone is above null: logistic balanced accuracy `0.8235`, random-forest balanced accuracy `0.8676`

Mixed procedure/orbit blocks need caveats:

- `orbit_search` is procedure/cache-adjacent and reaches balanced accuracy around `0.90`.
- the aggregate `orbit` block is strongest among non-metadata blocks, especially for random forest at balanced accuracy `0.9464` and ROC AUC `0.9869`, but it mixes combinatorics, geometry, and search/KKT availability scalars.
- `trajectory` is weak here at balanced accuracy `0.5982` for both models.

## Inference

The claim-bearing observation uses both metadata and non-provenance features, but with different epistemic status. Metadata proves the producer/regime labels are encoded in the table. Non-provenance geometric and orbit-adjacent blocks show that endpoint-produced and random-produced samples occupy distinguishable regions of the current feature table under grouped CV.

That observation is not by itself a thesis-usable positive result. It does not find a `sys > 1` row, and it does not give a direct rule for sampling target cases without using labels, producer identity, or post-hoc knowledge that a region was endpoint-heavy. Under the packet verdict meanings, this is `negative`: useful as a caveat/supporting diagnostic if the thesis discusses dataset-regime differences, but not an escalation or a conjectured-positive search rule.

## Checks Run

- Created this report path before full method changes.
- Validated dataset guards against the packet: row counts, union field counts, dataset counts, max `sys`, and `sys > 1` count.
- Ran `uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py --dataset-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P`.
- Ran `uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py --dataset-dir /tmp/sys-ds-reset-pilot-tables-VJ6D0P`.
- Confirmed all seven local feature tables have `282` rows after refresh.
- Removed unrelated `feature_pattern_search_ridge.png` and `feature_pattern_search_rf.png` rewrites from the feature-refresh command output.
- Confirmed the root checkout touched-path status is clean after accidentally applying the first draft patch outside this worktree and reverting only those own edits.

## Failure Modes/Caveats

- The metadata block intentionally includes variables that define or nearly define regime provenance. Treat its perfect score as a leakage/provenance demonstration, not as geometry.
- Grouped CV reduces duplicate-lineage leakage but does not create an independent external validation set.
- The sample is small and imbalanced by source: `112` endpoint rows versus `170` random rows, with endpoint rows dominated by `variable_f_ascent=90`.
- The non-provenance feature signal can still reflect producer selection effects: endpoints are outputs of ascent procedures, random rows are not.
- No permutation test, bootstrap interval, or fresh producer rerun was performed in this 10-minute local-budget packet.
- No causal or constructive rule was extracted from the fitted classifiers.

## Thesis-Use Proposal

Use as `supporting/caveat only`. A cautious thesis sentence could say that the current endpoint and random regimes are easily distinguished by provenance metadata and also separable by several geometric/orbit feature families under grouped CV; therefore cross-regime comparisons should not be interpreted as iid sampling from one feature distribution. Do not cite this as evidence for a target-search method before submission unless a separate generator-side rule is created and tested.

## Reopen Trigger

Reopen if any of these happen:

- a refreshed/larger dataset changes the row guards or adds a `sys > 1` row;
- feature provenance changes, especially if orbit/search availability scalars are removed or normalized;
- a lead proposes a concrete feature-space sampling rule that can be applied before labels, producer identity, or `sys` are inspected;
- Jörn wants a thesis-facing caveat backed by a permutation/null interval rather than grouped-CV point metrics.

## Actionable Target-Case Search

This result does not give an actionable way to search for target cases. It tells us that the current endpoint and random rows are classifiable, and it highlights candidate descriptors such as face geometry, omega summaries, skeleton counts, and the mixed orbit packet. It does not specify a label-free sampling region or optimization objective that should produce higher `sys`, so it remains supporting/caveat evidence rather than a positive search rule.
