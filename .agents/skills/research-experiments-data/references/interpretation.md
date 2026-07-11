# Experiment Interpretation

Use this reference when the task is not merely to report that an experiment ran,
but to help Jörn understand what the result says and whether it matters to the
thesis. Do not give an interpretation more strength than the current evidence.

## Source Truth

Interpret current artifacts from current code:

- generated outputs and the exact command or producer that made them;
- producer, feature, parser, and plotting code over column names or plot labels;
- experiment README/MAP files as navigation, not proof that cached numbers are
  current;
- thesis/research prose only for thesis-facing meaning or accepted
  interpretation.

When code, inputs, data, or feature definitions may have changed, establish
artifact identity from current code, inputs, hashes, or reviewed verification.
Recompute only when a material uncertainty remains; do not reuse stale prose or
rerun an expensive producer merely by default.

Use the parent skill when editing durable files. Detailed metric rows belong in
generated artifacts or generated reports, not hand-maintained prose copies.
Durable prose should record the question, data slice, producer/artifact path,
measured quantity, evidence boundary, and interpretation guardrails.

## Interpretation Dimensions

Include the dimensions needed to make the claim evaluable; this is not a fixed
answer template.

- **Slice:** table, bucket, split, run, candidate set, or artifact.
- **Measured object:** the mathematical/domain quantity, not only a column,
  plot axis, feature, or model label.
- **Association:** how the experiment connected that quantity to the target,
  such as a threshold, correlation, holdout prediction, residual comparison,
  ranking, or candidate enrichment.
- **Strength:** supported effect size, denominators, uncertainty, enrichment,
  rank, or comparison when it matters.
- **Boundary:** what remains unresolved, such as mechanism, theorem status,
  correlated features, proposer validity, confounding, or generalization.
- **Recomputation:** for non-obvious or durable claims, the source, artifact,
  command, or producer needed to check them again.

Define local labels that Jörn may not have in working memory. When examples are
included, say whether each is strongest, cleanest, representative,
thesis-relevant, or merely diagnostic.
