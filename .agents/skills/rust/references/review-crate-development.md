# Rust Crate DEVELOPMENT Review Prompt

## How To Use

Use this reference when reviewing a Rust crate `DEVELOPMENT.md`, including API
changes that touched or relied on DEVELOPMENT guidance. The actual reusable
prompt starts at `## Prompt`.

## Prompt

# DEVELOPMENT.md Review Prompt

## Review Objective

Review a crate DEVELOPMENT.md before the worker declares the maintainer-docs
part of the change done. The review should catch scope, evidence, and
maintainability issues that would otherwise make future agents redo context
reconstruction or preserve bad API decisions.

## Artifact Audience

DEVELOPMENT.md is for a developer maintaining, extending, refactoring, or
reviewing the crate internals. It should be a compact source of truth for the
current scope, why the API shape exists, where to edit, what was rejected, and
what evidence supports the current claims.

## Review Output Audience

The review output is for the worker who can still edit the crate. It should
identify incorrect, unverifiable, misplaced, or non-actionable maintainer claims
with enough evidence that the worker can fix them without redoing the whole
review.

## Evidence Surface

The review request must provide the crate path. If no crate path is provided,
ask for it before reviewing. Start by naming the crate path and the surfaces
checked.

Minimum review surface: DEVELOPMENT.md, README, crate manifest, public API entry
points, examples, and tests index. Read deeper only when needed to verify a
maintainer claim. For task/context evidence, inspect only files explicitly
referenced by the review request or by DEVELOPMENT.md. If DEVELOPMENT.md names a
caller symbol, package, task id, or experiment path, run at most one targeted
`rg` for that exact name/path within `experiments/`, `tasks/`, and `research/`;
otherwise mark caller context unverifiable from reviewed context.

Do not review implementation correctness or redesign the crate except where
DEVELOPMENT.md makes a maintainability, scope, verification, or API-shape claim
that needs checking. If the current caller, historical reason, or evidence for a
claim cannot be established from the checked surfaces, say that it is
unverifiable instead of reconstructing intent from taste.

Treat current caller, intended scope, historical reason, and verification
evidence as established only by explicit DEVELOPMENT.md/README text,
tests/examples, public API usage, task files, or named downstream callers
inspected in this review.

## Review Criteria

Look for:

- a spec that confuses current required scope with durable maintainer rationale:
  missing a real capability, adding an unused capability without rationale, or
  duplicating the same criterion;
- public API surface that does not beat ordinary Rust syntax, standard-library
  traits/types, or already-present crate data;
- duplicate sources of truth in code or docs, unless the duplication saves more
  reviewer/user labor than it costs;
- recoverable-error framing for programmer bugs that callers should not recover
  from;
- architecture notes that fail to help an agent know where to edit, or that
  re-teach consumer usage better left in README;
- verification text that names commands but not what each command witnesses, or
  claims evidence that tests/examples do not actually provide;
- rejected/deferred approaches that do not compare against the accepted choice;
- rejected/deferred approaches that do not state the reason;
- rejected/deferred approaches whose breadcrumbs do not save future reviewer
  labor;
- claims with unclear epistemic status: normative objective, current evidence,
  predicted cost, semantic reason, implementation evidence, and open decision
  should be distinguishable;
- consumer-facing tutorial material, import snippets, or copyable usage examples
  that should live in README instead.

These are examples, not a closed checklist. Use judgment.

## Evidence Status

Distinguish what you inspected from what you ran. If a verification claim
depends on a compile/test/example witness and you did not run it, say so. Do not
imply stronger evidence than the review actually gathered.

## Completeness Condition

A no-findings review is meaningful only if it names the checked surfaces and
explains why those surfaces were enough to evaluate DEVELOPMENT.md as maintainer
documentation.

## Output Contract

Findings first. For each finding, include a line reference or section heading
plus a short excerpt, and recommend the smallest useful action: delete, rewrite,
move to README, add a verification witness, or mark unverifiable. Do not list
things to leave as-is as findings; put justified non-issues in a brief note only
when useful.
