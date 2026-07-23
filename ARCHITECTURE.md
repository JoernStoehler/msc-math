# Repository architecture

This file is the stable codemap: where a kind of project knowledge belongs and
where to search for it. It deliberately omits detailed file inventories.

## Main domains

| Domain | Owns | Does not own |
| --- | --- | --- |
| `thesis/` | Reader-facing LaTeX, bibliography, thesis-native assets, and writing companions | exploratory proof development or producer artifacts |
| `formal/` | Mathematical statements, derivations, proof routes, audits, and unresolved obligations | final publication wording |
| `crates/` | Reusable Rust implementations, public contracts, unit tests, and crate maintenance notes | thesis-specific empirical claims |
| `experiments/` | Self-contained experiment packets: questions, producers, inputs, retained outputs, interpretation, and reproduction commands | a single subject/method/status taxonomy or reusable library APIs |
| `papers/` | Source papers, extracted passages, and paper-specific notes | project conclusions not established by the source |
| `submit/` | Official forms, submission requirements, and administrative source notes | thesis mathematical content |
| `docs/` | Project-wide facts, current status, cross-owner capabilities, and reproduction policy | topic-local evidence or proofs |
| `scripts/` | Repository-wide maintenance and reporting utilities | scientific results |

The directory tree follows ownership: material that should change and be
reviewed together belongs together.

## Cross-domain paths

Some questions cross owners. Follow these routes rather than treating one
directory as the whole project:

| Question | Start | Confirm with |
| --- | --- | --- |
| What is printed in the thesis? | `thesis/main.tex`, then `thesis/README.md` | active `thesis/*.tex` and cited sources |
| Is a mathematical result established? | `formal/README.md` and the relevant thesis/experiment owner | exact statement, proof source, certificate, and active thesis wording |
| Has an experiment already tested this? | `experiments/README.md`, then topic READMEs | producer, retained output, and interpretation |
| Can the implementation do this? | `crates/README.md`, then crate README/source/tests | public API, tests, and relevant verification experiments |
| Why was a route rejected or superseded? | current owner README and nearby decision/status note | source commit/history only when the current owner points there |
| What blocks thesis completion? | `docs/project-status.md` | named owner and stakeholder source |
| What may the repository currently rely on? | `docs/capabilities.md` | every named owner source |

## Ownership boundaries

The same topic may have several representations without duplicating authority:

```text
paper -> formal statement/proof -> implementation/experiment -> thesis prose
```

Each arrow is a checked relationship, not automatic inheritance. A passing
test does not prove a theorem; an experiment does not establish an unrestricted
claim; a formal note does not mean the theorem appears in the active thesis;
and thesis wording does not prove that code reproduces its stated result.

Project-wide summaries point to owners. Owners state:

- their current role and status;
- the result or capability actually established;
- its scope and important exclusions;
- authoritative source paths;
- superseded or historical alternatives when confusion is likely;
- reproduction or verification commands when applicable.

## Experiment packets

An experiment packet is the physical ownership unit. It contains the material
that must remain together for one empirical question to be reproduced and
interpreted: its producer, input contract, retained evidence, analysis,
limitations, and current use.

Packets are direct children of `experiments/` by default. Their directory names
describe a durable question or measured object, not a temporary status such as
`dev`, or a single classification such as `numerics`, `verification`, or
`performance`. Status changes do not move a packet.

Subject, method, implementation, comparison, thesis use, provenance, and
lifecycle are independent relations. Record them in the packet README with
exact paths and stable terms; expose useful collections as views in
`experiments/README.md`. Those relations do not create parent directories.

Split a packet when each part has an independently intelligible question,
producer/evidence contract, interpretation, and maintenance lifecycle. Keep
several runs or diagnostics together when they cannot be interpreted safely
without their common question or comparison contract.

Existing category and `dev-*` directories are transitional evidence owners in
this disposable prototype. Their executable paths were not moved because path
and import churn does not test the navigation model.

## Search behavior

Start from the owner, then use:

1. filename search for conventional entry points and stable topic terms;
2. text search for theorem labels, algorithm names, symbols, and dispositions;
3. targeted reads of the promising owner and its source pointers;
4. broader cross-owner search before making a project-wide negative claim.

Generated, vendored, build, worktree, and legacy trees are not normal
orientation surfaces. Open them only when the current owner points there.

## Stable implementation layout

Rust packages keep standard Cargo structure:

```text
<package>/
|-- Cargo.toml
|-- README.md
|-- DEVELOPMENT.md      optional maintainer notes
|-- src/
|-- tests/              when integration tests are useful
|-- examples/           when public examples are useful
`-- benches/            when benchmarks are retained
```

This navigation prototype does not move executable packages merely to improve
the appearance of the tree.
