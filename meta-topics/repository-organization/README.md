# Repository organization

This topic owns the rationale and contracts for locating, migrating and retaining
project material. Scientific accounts belong to their topic; setup belongs to
[development environment](../development-environment/README.md); administrative
release belongs to [thesis submission](../thesis-submission/README.md).

The [inventory](inventory/README.md) owns the current source ledger
and browser projection. The dated [organization comparison](organization-design.md)
owns the structure and lifecycle analysis. The inventory's
[execution plan](inventory/parallel-plan.md)
separates consolidation of existing results from new research selection, thesis
promotion and release. The coordinating owner maintains its old-to-new map;
this README does not establish a second inventory or current task list.

The lasting retention rule is purpose-based: keep a claim, canonical proof,
implementation/producer, caveat and necessary evidence together; keep matched
human judgments with the reviewed input; keep exact message-review working
packets in `/tmp/msc-math/message-reviews/` and transfer consequential findings
before clearing them. Conventional discovery/build interfaces remain where their
actual consumers require them. An old location alone is not a reason to retain a
second active owner. Recoverable historical provenance need not be rewritten as
if a frozen experiment ran under today's layout.

[September architecture migration opportunities](architecture-migration.md)
records contract-sensitive reuse candidates and distinctions between exact,
approximate and historical routes. It is dated source inspection, not a promise
that the proposed extraction is already implemented. Current canonical code and
topic accounts determine which opportunities have since been resolved.

## Ownership and migration contracts

The selected reader-facing manuscript remains `thesis/`; proof development,
reusable Rust packages and empirical producers may share a scientific topic
without acquiring one another's authority. Source literature records what a
paper establishes, while administrative release material belongs to thesis
submission. Coordination owns current assignments; dashboards present dated
state and synthesis. Summaries route readers to the authoritative source.

The scientific dependency remains:

```text
paper -> formal statement/proof -> implementation/experiment -> thesis prose
```

Check each relevant arrow. A passing test does not prove a theorem; an experiment
does not prove an unrestricted claim; a formal statement is not automatically
printed in the thesis; prose does not establish a working reproduction route.
A local entry point should expose current role/status, established result or
capability, scope/exclusions, canonical source, confusing historical alternatives,
and relevant verification/reproduction commands.

Keep generated datasets attributable to the producer fixing their population,
configuration and provenance. Consumers identify the output or contract they
actually use. Shared data or scaffolding need not force consumers into one
physical experiment. Preserve original purpose, current use, limitations,
source pointers and the changes that would require reinspection. Explicit
corresponding implementations may be easier to maintain than an abstraction;
share code when the API itself is the intended dependency.

An inventory states its physical coverage; omission in a selective view does
not establish absence. Search terms, symbols and artifact paths before inferring
missing work. Generated, vendored and legacy trees are usually poor orientation
surfaces but may own particular evidence. Scientific topics retain normal Cargo
packages (`Cargo.toml`, `src/`, and useful `tests/`, `examples/`, `benches/`), rather
than requiring every topic to adopt identical proof/code/data folders.

[The scoped meta migration review](meta-migration-review-2026-10-12.md) records
independent source/routing/retention approval and focused checks, with the
assembled-repository and scientific acceptance boundaries explicit.

- [Main checkpoint reconciliation](main-reconciliation-2026-10-12.md): combined research/client outcomes, remaining differences and bounded checks.
