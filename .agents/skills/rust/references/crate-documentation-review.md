# Rust Crate Documentation Review

Review claims against the crate manifest, public API entry points, examples,
tests, and any named owner-local caller or task surface. Do not invent a caller,
historical rationale, or verification witness when the inspected sources do not
establish one.

## README: consumer documentation

A capable caller should be able to perform ordinary use without opening
`src/`. Check whether:

- starter imports and examples show the intended caller path without ownership
  or implementation noise;
- public names express domain concepts rather than plumbing;
- advertised operations have a current caller, example, compile witness, or
  test witness;
- maintained prose duplicates code or tests without making ordinary use easier;
- maintainer navigation, internal workflows, and rejected designs should move
  to DEVELOPMENT.

## DEVELOPMENT: maintainer documentation

A future maintainer should not need to reconstruct current scope, API rationale,
edit locations, important rejected/deferred alternatives, or evidence meaning
from code and Git history. Check whether:

- current required scope is distinguished from durable rationale;
- architecture notes identify where changes belong;
- verification commands say what each result witnesses;
- rejected or deferred alternatives name both the comparison and reason when
  that information saves future reconstruction;
- normative objectives, current evidence, predictions, semantic reasons,
  implementation evidence, and open decisions are distinguishable;
- consumer tutorials and copyable examples should move to README.
