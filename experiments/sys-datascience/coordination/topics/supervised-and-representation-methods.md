# Supervised And Representation Methods

Use / maintenance model: seed-level topic map for in-table and representation
methods. Keep it focused on what a future topic owner or packet executor needs
to avoid repeating already-covered methods and to identify genuinely
thesis-relevant missing baselines.

Scope: in-table prediction, ranking, classification, feature representation,
PCA/clustering/anomaly methods, and standard data-science methods not yet
covered enough for thesis wording.

Status block:

- topic-status: conditional seed
- spawn-status: spawn only if thesis wording needs broader ordinary-method
  coverage or a supervised model suggests a concrete generated-candidate filter
- next-role: topic owner only under that condition
- next-action: missing-standard-method audit or feature-family ablation prompt
- review-gate: in-table performance does not validate generated-candidate
  proposers; grouped holdout/source controls required for model claims
- belief-update-owner: supervised-methods topic owner; research-map steward for
  cross-topic propagation
- last-reviewed: 2026-07-04 parked closure plan; 2026-07-06 workflow hardening
  pass added status metadata only
- source-of-truth: prediction, projection, tail-rule, and association method
  packets listed below
- stale-if: thesis wording asks for broader standard-method closure, new
  in-table packets land, or feature families change
- allowed-downstream-use: ordinary-method coverage planning and prompt seed; not
  proposer validation

Current belief: existing supervised/in-table methods expose structure but do
not validate proposers. Some cheap standard methods may still be missing if the
thesis wants broader "ordinary data-science methods" wording.

Owner-readiness/status: seed; spawn only if thesis wording needs broader
standard-method coverage or if a supervised model suggests a genuinely new
generated-candidate filter.

2026-07-04 parked closure plan: if this topic reopens for wording breadth, the
first executor should be one tiny retained-table baseline packet covering
lasso/elastic-net, gradient boosting, high-tail classification, and
feature-family ablation. Do not run SVM/kNN/kernel, neural/autoencoder,
Bayesian/GP, density/mixture/one-class, direct-search/optimization, or broader
random-distribution work unless a new thesis sentence or source interface makes
them relevant. Retained-table models never validate generated-candidate
proposers by themselves.

Evidence sources:

- `../../methods/prediction-ranking/`
- `../../methods/projection-structure/`
- `../../methods/tail-rule-mining/`
- `../../methods/statistical-associations/`

Adjacent topics to read:

- `generated-candidate-proposers.md`, because proposer claims need generated
  evaluation rather than in-table prediction alone.
- `method-surface-expansion.md`, for the broader missing-method audit.

Candidate hypotheses:

- Simple supervised models mostly learn bucket/combinatorial structure unless
  explicitly controlled.
- A high-tail classifier or gradient boosting model may discover interactions
  that scalar filters miss.
- Representation methods are more useful for explanation and diagnostics than
  direct candidate proposal.
- Grouped holdout by bucket/source is the key leakage guard for in-table claims.

Cheap discriminators:

- Add lasso/elastic-net and gradient boosting baselines if not already covered
  by current packet artifacts.
- Train high-tail classifiers under grouped holdout and inspect feature rules.
- Compare model performance with and without ridge-area feature families.

Ready packet prompts:

- Tiny retained-table standard baseline. `parked-conditional`.
  Launch only if thesis wording needs broader ordinary-method coverage.
  Scope: lasso/elastic-net, gradient boosting, high-tail classification, and
  feature-family ablation under grouped holdout. Stop after one compact
  deterministic summary; no new feature engineering and no proposer claim.

Needs topic-owner sharpening:

- Gradient boosting high-tail classifier packet.
- Feature-family ablation packet.

Opportunity-cost notes: run only if the resulting thesis wording needs broader
method coverage or if a model proposes a genuinely new generated-candidate
filter.
