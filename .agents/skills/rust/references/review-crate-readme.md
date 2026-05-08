# Rust Crate README Review Prompt

## How To Use

Use this reference when reviewing a Rust crate `README.md` before declaring a
crate or API change done. The actual reusable prompt starts at `## Prompt`.

## Source Status

This prompt preserves the concrete critic shape produced during the
`crates/algebraic-numbers` README review session in May 2026. It is a Rust
crate README prompt, not a general documentation theory; adapt it when a crate
has a different audience or evidence surface.

## Prompt

# README.md Review Prompt

## Review Objective

Review a crate README before the worker declares the crate done. The review
should catch KISS/YAGNI and consumer-ergonomics issues that would otherwise
force Jörn to switch context after the fact.

## Artifact Audience

The README is for a capable caller who wants to perform ordinary crate use
without opening `src/`. It should make the intended public API feel small,
inevitable, and copyable.

## Review Output Audience

The review output is for the worker who can still edit the crate. It should give
actionable findings with enough evidence that the worker can fix issues without
redoing the whole review.

## Evidence Surface

The review request must provide the crate path. If no crate path is provided,
ask for it before reviewing. Start by naming the crate path and the surfaces
checked.

Minimum review surface: the README, crate manifest, public API entry points,
examples, and tests index. Read deeper only when needed to verify a README
claim. If the review request points to a task file, experiment, or downstream
caller, inspect that specific surface too; otherwise keep the review
crate-local. Do not review implementation correctness or redesign the crate
except where the README exposes API friction to consumers.

If the current caller, intended main path, or witness for a claim cannot be
established from the checked surfaces, say that it is unverifiable from reviewed
context instead of inventing project intent.

Treat current caller, intended main path, and witness evidence as established
only by explicit README text, tests/examples, public API usage, task files, or
named downstream callers inspected in this review.

## Review Criteria

Look for:

- starter imports or examples heavier than the main consumer path;
- technically valid syntax that is awkward, asymmetric, or not what users
  should copy;
- local helpers, one-use bindings, `.clone()`, `.unwrap()`, or ownership noise
  that obscure the mathematical expression being demonstrated;
- README-foregrounded names that expose implementation plumbing instead of
  domain concepts;
- advertised operations without an example, compile witness, test witness, or
  current caller reason;
- maintainer navigation, internal file maps, test-location notes, or workflow
  detail that belongs in maintainer documentation;
- scope sections with vague labels instead of concrete missing capabilities.

These are examples, not a closed checklist. Use judgment.

## Evidence Status

Distinguish what you inspected from what you ran. If a claim depends on a
compile/test/example witness and you did not run it, say so. Do not imply
stronger evidence than the review actually gathered.

## Completeness Condition

A no-findings review is meaningful only if it names the checked surfaces and
explains why those surfaces were enough to evaluate the README as consumer
documentation.

## Output Contract

Findings first. For each finding, include a line reference or section heading
plus a short excerpt, and recommend the smallest useful action: delete, rewrite,
move to maintainer docs, add a witness test, or mark unverifiable. Do not list
things to leave as-is as findings; put justified non-issues in a brief note only
when useful.
