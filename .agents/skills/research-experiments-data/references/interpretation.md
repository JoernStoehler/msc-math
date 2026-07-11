# Experiment Interpretation

Use this reference when the task is not merely to report that an experiment ran,
but to help Jörn understand what the result says and whether it matters for the
thesis.

The skill improves answers by making the claim evaluable in one pass. It does
not make weak experiments strong, turn associations into mechanisms, or replace
current artifact inspection. If data, code, or feature definitions may have
changed, inspect current artifacts instead of reusing old interpretation prose.
First establish artifact identity from current code, inputs, hashes, or
existing reviewed verification; recompute only when a material uncertainty
remains.

## Source Truth

Interpret current artifacts from current code:

- generated outputs and the exact command or producer that made them;
- producer, feature, parser, and plotting code over column names or plot labels;
- experiment README/MAP files as navigation, not as proof that stale numbers
  are still true;
- thesis/research text only when the question is about thesis-facing meaning or
  already accepted interpretation.

Use the core experiment conventions in the parent `SKILL.md` when editing or
reviewing durable experiment files.

## Artifact-Derived Prose

Generated artifacts and current code are the source of truth for detailed
metric rows. Do not hand-maintain README/report paragraphs that duplicate TSV,
JSON, or numeric values extracted from figures unless that prose is generated
from the artifact. A paragraph that restates artifact rows by hand is a
stale-risk surface: check it against the artifact, replace it with a source
pointer, or generate it from the artifact.

Durable experiment prose should usually carry the question, data slice,
command/artifact path, measured quantity, strength boundary, and
interpretation guardrails. Detailed rankings, budget-prefix behavior,
per-feature metric rows, and attribution tables should live in generated
artifacts or generated compact reports. Quick chat interpretation may summarize
artifact rows, but say which artifact/run it came from.

This is not a ban on interpretation prose. It is a source-truth rule: if the
claim is meant to be durable and would go stale when a TSV row changes, either
generate it or point to the artifact instead of maintaining a second copy.

## What Good Looks Like

A useful interpretation lets Jörn evaluate the result without reconstructing
the experiment from follow-up questions. It usually contains:

- the data slice: which table, bucket, split, run, candidate set, or generated
  artifact the claim is about;
- the measured object: the mathematical/domain quantity computed from each
  object, not just the code column or model label;
- the association operation: how the experiment connected the measured
  quantity to the target, e.g. `K -> (sys(K), f(K))`, threshold rule,
  correlation, holdout prediction, residual comparison, or generated-candidate
  enrichment;
- the strength: effect size with denominators or uncertainty when strength
  matters, e.g. base rate, hits/selected, hits/positives, enrichment,
  correlation, interval, or rank among controls;
- the boundary: what the result does not show, such as mechanism, theorem,
  independence from correlated features, candidate-proposer validity, or
  generalization outside the named slice;
- the recomputation path when the claim is non-obvious or durable: source path,
  artifact path, command, or producer.

These are not a required answer template. Include the parts needed for the
claim to be checked.

## Common Failure Patterns

The concrete examples below are frequent local failures. Treat them as
instances of the general pattern, not as a closed list of banned phrases.

**Working-memory gap.**
The answer refers to something that is not in Jörn's current working memory and
does not define it locally. This can be a valid repo term, code label, artifact
label, model family, plot label, baseline, or abbreviated data slice. Examples:
`source/facet/product/provenance baseline`, `metadata`, `vol1`, raw feature
names, or "the retained table" without saying retained from what.

Fix the gap by naming the object, slice, or comparison in ordinary terms before
using the short label.

**Non-expressive compression.**
The phrase is short because the agent understands it, but it is not a stable
concept for Jörn. Examples observed in this slice include "omega-regular
two-face geometry" and "not raw coordinate size alone". A longer literal
sentence is better when it says what was measured and what changed.

**Artifact handle instead of measured object.**
The answer says a column, feature group, model output, plot axis, or tree node
instead of the quantity computed from the polytope or run. Translate labels by
checking producer code when needed. For example, do not stop at
`ridge_symp_area_volnorm_sum`; say it is the sum over primal two-faces of the
volume-normalized symplectic polygon area, with the exact formula if it matters.

**Missing association operation.**
The answer gives a vector, matrix, statistic family, or feature list without
saying how it was associated with the target. Say whether the experiment formed
a scalar per row, ranked rows, learned a threshold rule, tested a holdout,
compared residuals, or enriched generated candidates.

**Missing strength calibration.**
Words such as "strong", "signal", "pattern", "important", or "beats baseline"
do not tell Jörn how much evidence the artifact provides. Use the metric the
artifact actually supports. Count metrics should be written in their natural
order: precision is hits/selected, recall is hits/positives, and base rate is
positives/rows.

**Scope/control drift.**
The answer changes surfaces without saying so: full mixed table versus fixed
bucket, product rows versus generic rows, train/test split versus stability
sweep, in-table diagnostics versus unevaluated candidate generation, or
stratification controls versus mathematical features. If a result could be a
source/product/facet/provenance artifact, say what fixed bucket, grouped split,
permutation, or baseline addresses that and what remains open.

**Example role unclear.**
Examples are useful only when their role is clear. A top artifact row is not
automatically the central explanation. If you give an example, make clear
whether it is strongest, cleanest, representative, thesis-relevant, easiest to
reason about, or merely one diagnostic row. If no role is clear, point to the
artifact instead of adding an example.

## Durable Writing

Do not make quick interpretation prose the durable source of truth unless it is
intentionally becoming a report or thesis result. Quick prose rots when code,
data, feature definitions, or claims change, and wrong prose can become sticky.

Prefer durable surfaces that future agents can re-evaluate:

- self-explaining producer and feature code;
- executable inputs, arguments, and generated outputs;
- README navigation to commands, artifacts, entry points, and source
  definitions;
- generated compact reports when detailed metrics need a readable prose/table
  view across runs;
- concise report/thesis prose only when it names the slice, command/artifact,
  measured quantity, strength, and epistemic boundary.

Process knowledge belongs in skills only when it transfers across experiments.
Concrete result numbers belong in generated artifacts or intentional reports,
not in this skill.
