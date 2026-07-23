# Repository architecture

This file is the stable codemap: the conventional project domains, their
authority boundaries, and where to begin searching. It deliberately omits
detailed file inventories and does not prescribe local file placement.

## Main domains

| Domain | Contains | Excludes |
| --- | --- | --- |
| `thesis/` | Reader-facing LaTeX, bibliography, thesis-native assets, and writing companions | exploratory proof development or producer artifacts |
| `formal/` | Mathematical statements, derivations, proof routes, audits, and unresolved obligations | final publication wording |
| `crates/` | Reusable Rust implementations, public contracts, unit tests, and crate maintenance notes | thesis-specific empirical claims |
| `experiments/` | Empirical questions, data producers, consuming analyses, retained outputs, interpretation, and reproduction commands | one exhaustive subject/method/status taxonomy or ordinary reusable library APIs |
| `papers/` | Source papers, extracted passages, and paper-specific notes | project conclusions not established by the source |
| `submit/` | Official forms, submission requirements, and administrative source notes | thesis mathematical content |
| `docs/` | Project-wide facts, current status, cross-domain capabilities, and reproduction policy | topic-local evidence or proofs |
| `scripts/` | Repository-wide maintenance and reporting utilities | scientific results |

Use conventional layouts and decide local placement from the actual files and
purpose. A tree can expose only some relations: one experiment can
simultaneously concern a mathematical object, method, implementation,
producer, comparison, thesis use, and lifecycle. Preserve those relations and
their reasoning without turning them into a global placement algorithm.

## Cross-domain paths

Some questions cross owners. Follow these routes rather than treating one
directory as the whole project:

| Question | Start | Confirm with |
| --- | --- | --- |
| What is printed in the thesis? | `thesis/main.tex`, then `thesis/README.md` | active `thesis/*.tex` and cited sources |
| Is a mathematical result established? | `formal/README.md` and the relevant thesis/experiment entry point | exact statement, proof source, certificate, and active thesis wording |
| Has an experiment already tested this? | `experiments/README.md`, then topic READMEs | producer, retained output, and interpretation |
| Can the implementation do this? | `crates/README.md`, then crate README/source/tests | public API, tests, and relevant verification experiments |
| Why was a route rejected or superseded? | current local README and nearby decision/status note | source commit/history only when the current documentation points there |
| What blocks thesis completion? | `docs/project-status.md` | named work area and stakeholder source |
| What may the repository currently rely on? | `docs/capabilities.md` | every named authoritative source |

## Authority across domains

The same topic may have several representations without duplicating authority:

```text
paper -> formal statement/proof -> implementation/experiment -> thesis prose
```

Each arrow is a checked relationship, not automatic inheritance. A passing
test does not prove a theorem; an experiment does not establish an unrestricted
claim; a formal note does not mean the theorem appears in the active thesis;
and thesis wording does not prove that code reproduces its stated result.

Project-wide summaries point to authoritative sources. Local entry points make
recoverable:

- their current role and status;
- the result or capability actually established;
- its scope and important exclusions;
- authoritative source paths;
- superseded or historical alternatives when confusion is likely;
- reproduction or verification commands when applicable.

## Experiment material

There is no repository-wide algorithm for splitting or nesting experiments.
Use the standard structure a capable agent expects, then inspect the concrete
question, programs, data flow, artifacts, interpretation, and consumers before
moving files.

Producer-generated datasets remain with or directly attributable to the
producer that fixes their population/configuration and provenance. A consuming
experiment names the producer output or data contract it actually uses.
Changing a producer may require checking several consumers without making
those consumers one physical experiment.

Experiments may share executable scaffolding or corresponding instrumented
implementations without sharing a code surface. Several explicit, grep-able
implementations can be easier to inspect and change than an abstraction.
Share code when the maintained API itself is the intended dependency, not
merely because a refactor would touch several files.

Preserve original purpose, current use, important limitations, source pointers,
and reasons another change should trigger inspection. Those facts help a later
agent finish a local placement decision; they do not imply a flat or nested
global taxonomy.

## Search behavior

Typical exploration is progressively disclosed:

1. scan the relevant domain inventory or entry point;
2. read the READMEs that are not clearly irrelevant;
3. follow promising READMEs into code, manifests, data, proofs, artifacts, and
   detailed interpretation;
4. broaden terms or use reasoning-assisted exploration when lexical search
   misses plausible synonyms;
5. search across domains before making a project-wide negative claim.

An inventory should be exhaustive for the physical scope it claims. Selective
topic, method, status, or finding views state their scope and do not make
omission evidence of absence.

Generated, vendored, build, worktree, and legacy trees are not normal
orientation surfaces. Open them only when a current entry point or concrete
task points there.

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
