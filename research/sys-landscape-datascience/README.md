# Sys-Landscape Data-Science Taxonomies

## Purpose

- Provide frozen taxonomy anchors for the hostile-landscape method audit.
- Let later audit work say "this method family exists in taxonomy X" without inventing the family list from scratch.
- Keep the taxonomy layer separate from repo-state summaries and from thesis claims.

## Authority

- These files are authoritative only for the frozen taxonomy snapshot they record.
- They are **not** authoritative for what the repo actually did.
- The repo contents remain the source of truth for code, runs, artifacts, and actual attempted methods.

## Working Rule

- Prefer adding ledger or audit references that point to stable item IDs in these files.
- Do not cache repo-state tags such as `[attempted]` or `[skipped]` here.
- If a taxonomy turns out to be incomplete or ill-suited, prefer adding a new taxonomy file or a clearly labeled extension file instead of silently rewriting the old one.

## Files

- `taxonomy-islr.md`: adapted from the broad chapter-level statistical-learning toolbox familiar from ISLR.
- `taxonomy-esl.md`: adapted from the broader classical statistical-learning taxonomy in ESL.
- `taxonomy-murphy.md`: adapted from a probabilistic machine-learning taxonomy that naturally covers latent-variable, density, and sequential models.
- `taxonomy-dfo.md`: derivative-free and black-box optimization families for random/direct/surrogate-guided search.
- `taxonomy-numerical-optimization.md`: classical numerical-optimization families for gradient-based local search.
- `taxonomy-continuation.md`: continuation / homotopy families for changed-surface local search.
- `taxonomy-bayesian-optimization.md`: Bayesian optimization families for surrogate + acquisition search.
- `taxonomy-eda-visualization.md`: exploratory-data-analysis and visualization families.
- `taxonomy-statistical-inference.md`: statistical-inference and hypothesis-testing families.
- `taxonomy-time-series.md`: time-series and sequential-analysis families.
- `method-ledger.md`: cached index of attempted repo methods and their taxonomy references.

Current limitation:

- The frozen external taxonomies are now stronger on learning, optimization/search, exploratory analysis, and sequential-analysis methods than before.
- When a ledger row has no taxonomy reference yet, that means the method is present in the repo but not yet mapped to a frozen external taxonomy item.

## Stable-ID Rule

- Each taxonomy item has a stable ID.
- Later ledger or audit surfaces should cite those IDs rather than line numbers where possible.
- If an item needs refinement, prefer adding a child item instead of renaming the existing ID.
