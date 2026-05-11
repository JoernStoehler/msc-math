# Appendix Data-Science Source Contract

Status: source and structure contract for
`thesis/chapters/appendix-data-science.tex`.

This file is not final appendix prose. It tells a future writer what the
appendix may say, what must stay in the main hostile-landscape chapter, and
which source surfaces must be checked before any claim becomes thesis-facing.

## Purpose Of The Appendix

The data-science appendix should make the empirical data-science layer behind
the hostile-search-landscape chapter auditable without turning the main chapter
into a repository index.

The appendix may:

- describe the maintained data-science pipeline at the producer, table, and
  method-report levels;
- record the current dataset snapshot used by the retained method spikes;
- classify data-science method rows by evidential status;
- explain leakage, provenance, transfer, and exact-vs-f64 validity caveats;
- provide compact source tables that let a reviewer trace each empirical
  sentence back to a committed report, script, or research ledger.

The appendix must not:

- prove or imply that no further `Sys > 1` examples exist;
- present the data-science probes as an exhaustive survey of machine learning,
  black-box optimization, or convex-geometry heuristics;
- treat repo paths, temporary `/tmp` dataset paths, or scripts as reader-facing
  results without an adjacent interpretation boundary;
- silently repair weak source chains by writing smoother prose.

## Main Chapter Versus Appendix

Keep the main chapter focused on thesis narrative:

- the local question after the HKO positive anchor;
- the status categories: positive, negative, supporting-only, inconclusive,
  skipped/future, and inapplicable;
- the bounded headline conclusion that retained searches and method spikes did
  not produce a second transferable `Sys > 1` search regime;
- one compact provenance table or appendix pointer;
- short method paragraphs only where they affect the hostile-landscape
  conclusion.

Move to the appendix:

- the producer/table/method pipeline shape;
- dataset snapshot details and row provenance;
- the method-status ledger for feature-block regression, regime
  classification, PCA/clustering/anomaly, supervised alternatives,
  exact-vs-f64 spot checks, endpoint-residualized regression, SVMs, neural
  networks, and Bayesian optimization;
- feature-block definitions and leakage guards if they are needed to interpret
  metrics;
- exact source paths, command/provenance records, and report-level caveats;
- optional tables or schematic figures that would distract from the main
  chapter.

Do not duplicate the main chapter's synthesis. The appendix supports the
chapter; it should not become a second narrative conclusion.

## Source Inventory

### Thesis And Review Surfaces

| Source | Role | Appendix Use | Guard |
| --- | --- | --- | --- |
| `thesis/chapters/07-hostile-search-landscape.tex` | Current polished status draft | Keep appendix structure consistent with the chapter's categories and claims. | Do not strengthen any method row beyond the chapter. |
| `thesis/gpt55-exploration/landscape-claim-register.md` | Claim boundary and source map | Primary source for allowed claim strength, especially L11--L18. | Preserve all caveats, especially bounded/current/retained wording. |
| `thesis/gpt55-exploration/landscape-c1-review.md` | Risk review of earlier landscape draft | Use for weak-source warnings and appendix motivation. | Feature-block regression remains the highest-risk source chain. |
| `thesis/gpt55-exploration/landscape-polish-report.md` | Report of chapter polish decisions | Use for why provenance was centralized and categories were defined first. | This is a process report, not independent evidence. |
| `thesis/gpt55-exploration/landscape-polish-review.md` | Review of polished chapter | Use for final category and caveat checks. | `polish-success` means current best scaffold, not thesis-final approval. |
| `thesis/gpt55-exploration/thesis-exposition-dependency-graph.md` | Ordering constraints | Use the dependencies "empirical result categories before methods" and "row/data provenance before learned-pattern interpretation". | If appendix prose violates these, rewrite it. |
| `thesis/gpt55-exploration/sentence-ledger.md` | Safe phrasing cache | Use the hostile-landscape and empirical-category safe phrasings. | Do not copy unsafe alternatives. |

### Research Ledgers

| Source | Role | Appendix Use | Guard |
| --- | --- | --- | --- |
| `research/sys-landscape.md` | Topic-level narrative and current state | Source for row counts, maxima, regular-product status, and hostile-landscape interpretation. | Narrative only; tool-by-tool verdicts belong in the toolbox audit. |
| `research/sys-landscape-toolbox-audit.md` | Canonical method ledger for this empirical claim | Main source for method categories, thesis use, validity guards, and reopen triggers. | Treat rating vocabulary strictly. |
| `research/sys-landscape-datascience/idea-ledger.md` | Data-science idea-exhaustion ledger | Source for dataset snapshot, worker-process context, and deferred/future rows. | The `/tmp` dataset paths are run snapshots, not durable thesis artifacts. |
| `research/sys-landscape-datascience/method-ledger.md` and taxonomy files | Broader method universe | Use only to explain coverage caveats if needed. | Do not imply all taxonomy families were attempted. |
| `tasks/landscape.md` | Current task and blocker surface | Use for active blockers such as residualized regression or future method gates. | Refresh before final thesis writing if the task state changed. |

### Experiment Pipeline Sources

| Source | Role | Appendix Use | Guard |
| --- | --- | --- | --- |
| `experiments/sys-landscape/datascience/produce/README.md` | Producer-stage contract | Explain that producer caches own geometry, witness payloads, provenance, and traces. | Do not edit or patch generated `.jsonl` files in an appendix pass. |
| `experiments/sys-landscape/datascience/produce/*.jsonl` | Committed producer caches | Durable data source behind table snapshots. | Do not manually inspect large JSONL for thesis numbers unless a source refresh is requested. |
| `experiments/sys-landscape/datascience/tables/README.md` | Table-stage contract | Explain `polytope-table.jsonl` and `observation-table.jsonl` as method inputs. | Tables are generated outputs; cite table-stage code/report guards, not stale temp paths alone. |
| `experiments/sys-landscape/datascience/tables/*.rs` | Table builder and feature definitions | Use for feature-family definitions if appendix needs technical detail. | Do not turn implementation details into mathematical claims. |
| `experiments/sys-landscape/datascience/methods/README.md` | Method-stage contract | Explain that methods read tables as black-box inputs. | New method spikes should have their own folder; the appendix should not redesign the method layer. |

### Method Reports And Scripts

| Method Surface | Source | Allowed Thesis Use | Guard |
| --- | --- | --- | --- |
| Dataset snapshot | `research/sys-landscape-datascience/idea-ledger.md`, plus report guard sections | Setup only: `282` rows, counts by producer, max `sys ~= 0.906316153431123`, zero `sys > 1`. | Field-count numbers differ across planning/report surfaces (`133/47` versus `135/53` union fields). Avoid field counts unless freshly verified. |
| Feature-block regression | `research/sys-landscape-toolbox-audit.md`, `research/sys-landscape-datascience/idea-ledger.md`, `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py` | Attempted negative transfer story with weak source chain. | No committed feature-block thesis report exists; `stat-sanity` is non-load-bearing unless repaired. |
| Regime classification | `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime-classification-report.md`, `regime_classification_summary.md` | Supporting/caveat only: endpoint and random rows are distinguishable in the current table. | Not a label-free generator or negative search result; metadata separation is a provenance warning. |
| PCA, KMeans, IsolationForest | `experiments/sys-landscape/datascience/methods/pca-cluster-spike/report.md` | Bounded negative method spike with supporting diagnostics. | Does not rule out all embeddings, clustering, manifold learning, or future generator-side rules. |
| Cheap supervised alternatives | `experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/REPORT.md` | Bounded negative transfer check for lasso, elastic net, histogram gradient boosting, extra trees, and kNN. | Small method panel; implementation trust is medium and process required lead repair. |
| Exact-vs-f64 spot check | `experiments/sys-landscape/datascience/methods/exact-f64-spot-check/report.md` | Supporting-only validity caveat for sampled rational vertex encodings and selected geometry scalars. | Does not validate exact semantics of volume, capacity, skeleton, ridge, transition, or orbit-search quantities. |
| Endpoint-residualized regression | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_residual.py`, audit row | Inconclusive or future only. | No reviewed current-contract report; do not count as negative evidence. |
| SVMs, neural networks, Bayesian optimization | toolbox audit and idea ledger rows | Skipped, rejected-low-VOI, or future coverage caveats. | Never phrase as failed methods in the current evidence set. |

## Result Categories For Appendix Tables

Use these categories exactly enough that a reviewer can sort every row.

| Category | Meaning | Rows In Scope |
| --- | --- | --- |
| `positive anchor` | Known positive reference point, not discovered by the data-science methods. | HKO pentagon-pentagon geometry. |
| `setup` | Dataset or pipeline surface needed to interpret method results. | Producer caches, table builder, 282-row snapshot. |
| `attempted negative` | A tried method did not produce a usable search rule in its declared input range. | Feature-block transfer, PCA/clustering/anomaly, cheap supervised alternatives. |
| `local optimization only` | Evidence concerns local endpoints or continuation paths, not global search. | Fixed-`F` ascent and variable-`F` continuation when summarized in appendix context. |
| `supporting-only diagnostic` | Checks mechanics, provenance, or consistency but is not failed-search evidence. | Regime classification, exact-vs-f64 spot check, rejection calibration if mentioned. |
| `inconclusive` | Existing artifact lacks a reviewed report or disposition strong enough for thesis use. | Endpoint-residualized regression. |
| `skipped/future` | Method family was not attempted or was rejected/deferred for value, cost, data-size, or design reasons. | SVM baseline, neural/deep models, Bayesian optimization, interpretable tail rules unless later repaired. |
| `inapplicable` | A nearby result answers a different question. | HKO-local perturbation evidence if discussed as generic landscape evidence. |

## Tables And Figures

Tables that could be copied into the appendix after source checking:

- Dataset snapshot table: producer family, row count, role, and source path.
  Mark as "snapshot from current retained method reports"; do not use temporary
  `/tmp` paths as the durable source.
- Method-status table: method, category, source surface, allowed thesis use,
  validity guard, and reopen trigger. This should be the appendix's central
  table.
- Feature-block or feature-family table: only if the appendix discusses
  classifier/regression metrics. Source definitions should come from
  `regime_classification_summary.md` and table-stage feature files.
- Small metrics table for regime classification or supervised alternatives:
  only include metrics that are explicitly tied to a supporting/caveat or
  bounded-negative interpretation.
- Exact-vs-f64 spot-check table: sample policy and checked column classes, not
  every sampled row, unless Jörn wants a detailed reproducibility appendix.

Figures or schematics that could be used only if explicitly marked:

- `regime_classification_bars.png` may be copied into `thesis/` only if the
  caption says it is a supporting regime-separation diagnostic, not a search
  rule.
- `feature_pattern_search_ridge.png`, `feature_pattern_search_rf.png`, and
  `feature_pattern_search_residual.png` should not be used as claim-bearing
  figures without a repaired report for the corresponding feature-block or
  residualized-regression result.
- A hand-drawn pipeline diagram may be mocked only if it is labeled as a
  schematic of `produce -> tables -> methods`, not generated data.
- Do not include screenshots or untracked `/tmp` artifacts.

If a future writer copies any generated figure into `thesis/`, the copy must be
deliberate and self-contained under `thesis/`, with a caption naming the source
report and the method category.

## Unsafe Overclaims

Reject or rewrite these claim shapes:

- "Data science failed to find all other high-`Sys` regimes."
- "The table proves there is no second `Sys > 1` example."
- "The 282-row dataset is representative of all convex bodies."
- "The row counts support a density or typicality claim."
- "Regime classification gives a search rule."
- "Perfect metadata classification is geometric evidence."
- "Non-provenance classification is independent of producer effects."
- "Within-regime regression signal transfers to endpoint search."
- "`stat-sanity` establishes thesis-facing uncertainty intervals."
- "PCA or clustering proves no low-dimensional structure is useful."
- "IsolationForest proves anomalies are irrelevant."
- "Exact-vs-f64 spot checks validate all exact semantics."
- "Endpoint-residualized regression was negative."
- "SVMs, neural networks, or Bayesian optimization failed."
- "Skipped methods enlarge the negative evidence."
- "The appendix can omit caveats because the main chapter already has them."

Safe shape:

> The retained black-box and standard data-science attempts did not produce a
> new transferable high-`Sys` regime. This is negative evidence about the
> attempted search strategies, not a proof that no such regime exists.

## Exact Reviewer Checklist

A reviewer of `thesis/chapters/appendix-data-science.tex` should check every
item below.

- The appendix defines method-result categories before any method list.
- The known HKO positive anchor is named as context, not as a data-science
  discovery.
- Dataset snapshot claims state `282` rows, the five producer counts, max
  `sys ~= 0.906316153431123`, and zero `sys > 1` only if the cited report or
  ledger still says so.
- Temporary `/tmp` paths are treated as provenance for a run, not durable
  source truth.
- Producer, table, and method layers are separated.
- Main chapter synthesis is not duplicated as a second conclusion.
- Every method row has a category, source path, validity guard, and thesis-use
  label.
- Feature-block regression keeps the weak-source-chain caveat adjacent to the
  claim.
- `stat-sanity` remains non-load-bearing unless a committed script/report repair
  lands and is reviewed.
- Regime classification is supporting/caveat only and not counted as a negative
  search surface.
- Metadata classification is described as a provenance warning.
- PCA, clustering, anomaly, and supervised alternatives are bounded current
  spikes, not exhaustive method-family failures.
- Exact-vs-f64 is limited to selected sampled encodings and geometry columns.
- Endpoint-residualized regression is inconclusive or future unless a reviewed
  current-contract report exists.
- SVMs, neural networks, Bayesian optimization, and interpretable tail rules are
  skipped/future/rejected-low-VOI caveats, not failures.
- Any table copied from a report preserves the report's caveat and category.
- Any figure copied into `thesis/` is self-contained, source-labeled, and
  captioned with a non-overclaiming interpretation.
- No sentence states a density theorem, impossibility theorem, classification
  theorem, or practical impossibility claim.
- No sentence uses "standard data science" as if the retained method panel were
  exhaustive.
- Review passes list whether experiments were rerun. The default for this
  appendix pass is "no experiments rerun" unless Jörn explicitly asks for a
  refresh.

## Future Writer Prompt

Use this prompt when assigning the actual appendix draft:

```text
You are writing `thesis/chapters/appendix-data-science.tex` in repo
`/workspaces/msc-math`. Read AGENTS.md and the skills `thesis-conventions`,
`project-quality`, and `research-experiments-data`. Read
`thesis/gpt55-exploration/appendix-datascience-source-contract.md`,
`thesis/gpt55-exploration/landscape-claim-register.md`,
`thesis/gpt55-exploration/landscape-polish-review.md`,
`thesis/gpt55-exploration/thesis-exposition-dependency-graph.md`,
`thesis/gpt55-exploration/sentence-ledger.md`, and current
`thesis/chapters/07-hostile-search-landscape.tex`.

Draft reader-facing appendix prose, not another source contract. Keep the
appendix self-contained under `thesis/`. Do not rerun experiments unless Jörn
explicitly requests a source refresh. Do not edit generated JSONL or method
outputs. Use the appendix to expose source provenance, row provenance,
method-result categories, and data-science validity caveats. Keep the main
landscape synthesis in the main chapter.

Required structure:
1. purpose and relation to the hostile-landscape chapter;
2. pipeline and dataset snapshot;
3. method-status table;
4. short notes on feature-block transfer, regime classification,
   PCA/clustering/anomaly, supervised alternatives, exact-vs-f64, and
   inconclusive/skipped methods;
5. source and validity checklist.

Review before final response against every item in the contract's reviewer
checklist. Final response must list review passes performed and whether any
review subagents were used.
```

## Contract Review Passes Performed

- Read the required skills: `thesis-conventions`, `project-quality`,
  `research-experiments-data`, and `git-worktrees-merge`.
- Read the requested exploration files:
  `landscape-claim-register.md`, `landscape-c1-review.md`,
  `landscape-polish-report.md`, `landscape-polish-review.md`,
  `thesis-exposition-dependency-graph.md`, and `sentence-ledger.md`.
- Read current `thesis/chapters/07-hostile-search-landscape.tex`.
- Grepped the relevant research and experiment surfaces under
  `research/sys-landscape*`, `tasks/landscape.md`, and
  `experiments/sys-landscape/datascience/`.
- Read the durable data-science README surfaces and committed method reports
  for regime classification, PCA/clustering/anomaly, supervised alternatives,
  and exact-vs-f64.
- Did not rerun experiments.
- Did not use review subagents; this was a non-fork contract task.
