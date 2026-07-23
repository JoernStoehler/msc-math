# Organize Experiment Material

Use this reference when creating, moving, splitting, joining, or documenting
experiment material, or when deciding whether similar code should be copied or
shared.

It records reasoning adapted to this repository. It does not provide a general
placement algorithm. Inspect the actual question, code, data flow, artifacts,
interpretation, and consumers before deciding where files belong.

## Situation to preserve

This repository contains many retained experiments, negative and mixed results,
alternative implementations, and superseded routes. Prior work is common
enough that searching has high expected value, but one experiment can be
related simultaneously by:

- mathematical object or thesis topic;
- empirical method;
- implementation under study;
- data producer or consumer;
- comparison set;
- provenance and evidence role;
- current status or lifecycle.

The directory tree can expose only some of these relations. Do not infer a
flat or nested taxonomy from the list. Preserve the relations, original
purpose, and reasons another change should trigger inspection so a later agent
can finish the local placement decision with the context then available.

## Explore before moving

Start with the relevant inventory or entry point, then read every plausible
local README. Follow those into:

- code and manifests;
- input paths, producer commands, and schemas;
- retained data and generated artifacts;
- proofs, certificates, and verification;
- interpretation, limitations, stopped routes, and current consumers;
- incoming and outgoing path/import references.

A failed lexical query is weak evidence of absence when related work may use
different terminology. Broaden terms and inspect nearby conceptual matches
before claiming a gap.

Do not move from the README alone when the code, data, or proof determines what
the material actually does.

## Producers and consuming experiments

A generated dataset remains with or directly attributable to the producer
program and configuration that fix its population and provenance. A consuming
experiment may edit that producer when its question requires a new output; the
change must then inspect affected consumers.

Declare the dependency where it is consumed:

- imports and Cargo manifests for code dependencies;
- scripts, configs, commands, or stable identifiers for data dependencies;
- a consumer README when a runtime path or scientific role would otherwise be
  implicit.

Use stable, grep-able producer/artifact names so repository search derives the
reverse relation. Do not maintain a manually curated producer-side list as if
it exhaustively names consumers. One omitted consumer creates a persuasive
false negative during refactoring. Explicitly scoped examples or reverse views
generated from consumer declarations are fine.

An obsolete relationship match is usually a checkable false positive. A
missing declaration may never be questioned. Favor recall in change-impact
search without treating stale scientific claims as harmless.

## Similar code and corresponding implementations

Repeated scaffolding or several instrumented versions of an algorithm do not
by themselves justify a shared import. Editing several explicit, grep-able
files can be easier to understand and verify than changing one abstraction
with hidden behavioral dependents.

Consider:

- whether identical behavior is itself a maintained contract;
- whether independent evolution or implementation comparison is valuable;
- whether all corresponding locations are easy to enumerate;
- whether tests or comparisons expose a missed update;
- whether sharing would obscure instrumentation, performance, arithmetic, or
  evidence independence.

Share code when the API is the intended dependency. Copy-edit small executable
scaffolds when divergence is allowed. Preserve enough local purpose or
correspondence for a future agent to understand why similar files exist; do not
require copied-at commits or another provenance mechanism without a concrete
checking use.

## Local README as triage

The README should help an agent decide whether deeper inspection is warranted,
not replace the underlying evidence. Depending on the material, preserve:

- original question and current use;
- important positive, negative, mixed, or superseded result;
- scope, limitations, and misleading interpretations;
- producer inputs and downstream consumers;
- authoritative code, data, proof, artifact, and interpretation paths;
- stable terms and related implementations likely to be searched;
- cheap checks versus commands that refresh retained evidence.

These are considerations, not mandatory fields. A clear conventional README
need not be rewritten into a schema.

## Correct facts while migrating

Structural migration is not a reason to preserve a statement known to be
wrong. Correct or remove it when the replacement is clear.

If a statement is known false but the replacement is unresolved, remove the
false assertion and leave a nearby explicit TODO stating what is false and
what focused check remains. If it is merely uncertain, state the uncertainty
and evidence needed to resolve it.

Record additions, factual corrections, deletions, deliberate omissions, and
unresolved TODOs separately from mechanical moves. This does not require a
complete scientific audit of every migrated area.

## Views and inventories

A conventional domain entry point should make its physical scope visible
before an agent must guess search terms. Keep any claimed physical inventory
auditable against the tree.

Topic, method, status, finding, or thesis-use views can support semantic
discovery. State their scope; do not let omission from a selective view become
evidence that related work is absent. A view supports only what it actually
checks or records and does not replace the source evidence it points to.
