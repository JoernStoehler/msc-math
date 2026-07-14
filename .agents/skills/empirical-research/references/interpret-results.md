# Interpret Empirical Results

Use this reference when the task is not merely to report that an experiment ran,
but to help Jörn understand what the result says and whether it matters to the
thesis. Do not give an interpretation more strength than the current evidence.

Keep distinct where relevant:

- direct observations and their measurement conditions;
- hypotheses or mechanisms that could explain them;
- inferences linking multiple observations or assumptions;
- predicted outcomes under competing hypotheses;
- proposed experiments and their possible outcome branches;
- beliefs about plausibility, uncertainty, and expected information value;
- judgments about thesis value, future option value, and execution cost.

These objects can form an argument or decision graph without sharing one
schema. State enough source and reasoning detail that another researcher can
locate disagreement in an observation, assumption, inference, prediction, or
value estimate rather than having to accept a compressed conclusion.

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

Interpret threshold events relative to the packet's declared inputs and
question. Before escalating a positive row or artifact, distinguish a newly
produced candidate or new source from a known positive reference/control that
was included deliberately. An expected known positive can validate plumbing or
provide comparison evidence; its mere presence is not a discovery. Escalate it
only when its value, provenance, or relation to the active question is itself
new or inconsistent.

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
