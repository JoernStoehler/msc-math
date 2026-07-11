# Experiment Packet Readiness

Judge readiness for a named downstream use, not completeness in the abstract.
A packet can be useful to further research and unusable for thesis writing. A
clean implementation can still have weak evidence; polished prose does not
strengthen evidence.

## Start From The Transition

Establish:

- the packet and its owner-local source truth;
- the intended consumer and what that consumer must be able to do;
- the concrete readiness target;
- the claims the packet may support and prohibited stronger conclusions;
- evidence or review that could materially change the conclusion.

Before honoring a saved status, phase label, launch contract, or stopping rule,
establish whether it had authority to settle the transition. At a claimed
research-phase boundary, explicitly identify and disposition each current
instruction, accepted stakeholder decision, or higher-authority owner fact that
requires further work or withholds closure. If one contradicts agent-authored
closure, surface the conflict and do not carry the closure forward or use a
narrower saved contract to reject the work it may require. Recovered state,
proposals, control caches, and agent-authored closure do not themselves resolve
such a conflict. A bounded packet may still be ready for one use while the
broader research phase remains open.

The contexts and criteria below are recurring examples, not a complete
classification. Select, combine, adapt, or add checks according to what the
consumer needs. Do not expand every plausible criterion into work; make
consequential gaps and deliberate omissions visible in ordinary language.

## Review The Packet

Inspect current code, inputs, generated artifacts, owner-local interpretation,
and the commands or provenance connecting them. Do not infer packet purpose
from artifacts alone or copy detailed metrics into a new control document.
Use existing reviewed verification when it still identifies the current
artifact. Do not rerun expensive producers or analyses merely to make evidence
look fresh; rerun only when a cheaper identity check cannot resolve an
uncertainty material to the transition.

Consider materially different dimensions independently rather than averaging
them into one impression. Depending on the packet, these may include question
fit, implementation validity, artifact identity and provenance, interpretation,
reproducibility, review independence, and usability by the consumer. Add,
combine, or omit dimensions based on actual failure modes and downstream use.

Use a fresh reviewer when context independence is itself evidence or likely
error reduction justifies the cost. Self-review can suffice for low-risk,
source-obvious changes. Combine review roles when they use the same evidence;
split them when technical/provenance and domain-interpretation checks can fail
independently or would crowd each other out.

Return a bounded conclusion about whether the packet can serve the named use.
Where material, explain limits the consumer must preserve, independently usable
parts, exact repairs, and whether promotion is worth doing. Labels such as
`ready`, `repairable`, or `mixed/tainted` are optional shorthand.

Do not launch another dataset, method, model, plot, or polish pass merely to
make the packet look complete. New work needs a named downstream claim or
decision that current evidence cannot support, plus an explicit stopping and
review rule.

Record packet facts, commands, claims, caveats, and disposition beside the
experiment. Keep scratch matrices and prompt drafts in `/tmp`. A readiness
conclusion does not replace Jörn/Kai acceptance or final thesis review.

## Recurring Contexts And Criteria

The contexts below are common downstream uses that expose different concerns.
They can overlap, appear in another order, or fail to describe a particular
transition. The criteria are prompts for judgment, not a complete taxonomy or
a queue of extra experiments.

### Plumbing or smoke use

The packet can test that the path works or estimate feasibility. It needs a
bounded smoke input, a command that runs, inspectable output, and an explicit
warning that clean plumbing or smoke-scale observations are not research or
thesis evidence.

### Reuse in further research

A later research session can use the packet without reconstructing hidden
context or unknowingly building on invalid evidence.

Normally check:

- the local question, experimental object, target quantity, and why the result
  could change a research decision;
- exact inputs, selection rules, feature definitions, transformations, and
  producer or sampling boundary;
- runnable producer/analyzer path, smoke path where useful, retained artifacts,
  and enough provenance to identify the run;
- sanity checks, leakage/taint risks, known failed routes, and whether selection
  happened before or after observing the target;
- observations separated from inference, current beliefs, uncertainty, and
  prohibited conclusions;
- what later work may reuse, what must be rerun or repaired, and a stopping or
  reopen trigger;
- a review proportionate to the chance that an error would misdirect later
  research.

Exploratory flexibility is allowed. A packet can be research-ready with
model-sensitive or negative evidence if that status and its allowed use are
clear. It need not have publication-quality assets or thesis prose.

### Demonstration or polish

The evidence question is closed enough that the next task is to make existing
results legible and reproducible, not to search for a better result.

Where relevant, also check:

- the selected result has a source-backed role in the intended account;
- detailed numbers and tables are generated from owning artifacts rather than
  copied into a second hand-maintained source;
- the asset/table purpose, audience, comparison, units, labels, and caveats are
  fixed before visual polish;
- a reproduction command and expected comparison rule exist;
- current reviewed verification or a proportionate check identifies the
  selected artifact; an expensive full rerun is not required when it would add
  no readiness information;
- post-hoc diagnostics, frozen tests, smoke evidence, and independent
  validation remain visibly distinct;
- no dormant plan, feasible method, or open checklist item is mistaken for a
  selected next action.

### Thesis writeup

A writing session can make a source-backed thesis section without conducting a
new experiment or reverse-engineering the evidence package.

Where relevant, also check:

- the exact thesis claim ladder is mapped to current artifacts, literature,
  and owner-local interpretation;
- the mathematical/domain object and association operation are expressible in
  reader-facing language, not only column or model names;
- strength is calibrated with the metric and denominator the artifact supports;
- scope boundaries, confounders, model dependence, selection timing, negative
  results, and failed transfer are explicit;
- examples and assets have a named explanatory role;
- every retained claim has a recomputation/source path and no stale draft is
  treated as evidence;
- an interpretation review has checked that technical validity did not become
  a stronger scientific or mathematical claim;
- remaining Jörn/Kai decisions and ordinary integrated-PDF review are named.

Being usable for thesis writeup does not mean final wording is accepted or the
whole chapter is complete.

### Publication or reproduction

Use the repo's current publication/reproduction contract. Normally add clean
checkout execution, environment and external-compute boundaries, durable input
availability, timing/resource facts where promised, and a byte-identical or
explicitly accepted output comparison class. Do not invent a stronger archive
promise merely because the packet reproduces locally.

## Cross-Cutting Failure Checks

For a consequential packet, actively test the locally plausible failures:

- untracked required files or local absolute paths;
- README, schema, defaults, command, and artifact disagreement;
- generated output changed without deliberate regeneration;
- smoke output presented as full evidence;
- one run or one method standing in for a broader claimed surface;
- in-table association or ranking presented as independent candidate proposal;
- post-target selection presented as frozen validation or mechanism;
- source, generator, bucket, seed, facet, or product structure driving the
  apparent result;
- a saved proposal treated as evidence or execution authority;
- a polished plot or paragraph hiding weak provenance or epistemic status;
- one tainted claim contaminating usable parts, or usable parts laundering the
  tainted claim;
- reviewer overload causing code/provenance or interpretation checks to vanish;
- expensive recomputation used as a substitute for checking existing artifact
  identity, review records, or the actual unresolved readiness question.

## Recording The Assessment

When a durable readiness record is useful, keep it compact and evidence-linked.
Ordinary prose is usually enough. Shorthand can help repeated local reviews;
for example:

- `satisfied`, with the evidence pointer;
- `limited-sufficient`, with the preserved boundary;
- `repair`, with the exact action and recheck;
- `omitted`, with the reason omission is safe for this use;
- `inapplicable`, with a short reason.

These terms are neither required nor exhaustive. Do not use a bare `open`
label without an owner, decision role, and stopping condition. An
unowned open item easily becomes a future execution queue.
