# Experiments

This directory contains retained empirical work, including negative results,
alternative implementations, data producers, consuming analyses, exact and
f64 checks, performance measurements, and thesis-support assets. Existing work
is common enough that searching here should be an ordinary first step before
recreating an experiment or declaring a gap.

Typical exploration:

1. scan the directory inventory below;
2. read every README that is not clearly irrelevant;
3. follow relevant READMEs into code, manifests, data, proofs, generated
   artifacts, and detailed interpretation;
4. broaden terminology when a lexical search returns no useful hit.

`ARCHITECTURE.md` explains the cross-domain authority boundaries.
`.agents/skills/empirical-research/references/experiment-packets.md` records
repo-specific considerations for creating or moving experiment material. It
does not supply a split/join algorithm.

## Directory inventory

This table covers every repo-tracked immediate research directory under
`experiments/`. Ignored caches and generated scratch directories are not
project navigation entries. The table is a physical inventory with short
search cues, not a finding index or a claim that each row is one experiment.

| Directory | What an initial reader can find there |
| --- | --- |
| `ai-use/` | session-log-backed AI-use provenance reports, prompts, scripts, and thesis disclosure inputs |
| `canonization-t-search/` | frozen coordinate-canonization search and evidence |
| `combinatorial-cells/` | boundary-event producers, shared polytope cache, cell diagnostics, retained negative results |
| `crosspolytope/` | specialized crosspolytope capacity computation and checkpointed output |
| `dev-flow-graph/` | flow-graph algorithm development spikes, diagnostics, and visualization |
| `dev-gradient-ascent/` | gradient-ascent implementations, traces, policy comparisons, and endpoint diagnostics |
| `dev-quadratic-program/` | QP route implementation research, f64/exact/fallback behavior, numerics, verification, and benchmarks |
| `dev-sys-prediction/` | `sys` prediction producers, error models, and branch/parameter probes |
| `hko-local-maximum/` | HKO theorem certificate tooling, empirical support, validation, and figures |
| `local-maxima-check/` | selected-body local-behavior comparison and retained artifacts |
| `performance/` | retained runtime, memory, and capacity-route measurement programs |
| `qp-error-bounds/` | wide QP intermediate-variable f64/exact evidence, retained-route evaluation, and soundness trials |
| `regular-products/` | rotated regular-product sweeps, pentagon empirics, figures, and exact formula certificate |
| `sys-datascience/` | random/product data producers, prepared tables, many consuming method experiments, and research state |
| `sys-landscape/` | hostile-`sys` search implementations, caches, legacy producers, and selected retained searches |
| `verification/` | capacity properties, minimum-orbit production, orbit recovery, and flow-graph falsifiers |
| `visualization/` | 4D-polytope viewer, data exporters, and thesis-support screenshots |

Keep this table auditable against the repo-tracked immediate directory tree.
Add a row when adding an immediate research directory. Do not use omission from
a selective semantic view as evidence that related work does not exist.

`FINDINGS.md` is a selective, grep-friendly view of notable bounded results,
negative results, and dispositions across packets. It supplies semantic search
terms and claim boundaries; it is not an exhaustive experiment inventory or
source of evidence.

`algorithm-comparisons.md` is a root-level routing note rather than an
experiment directory: it relates algorithm units to the experiment that
actually produces each kind of evidence.

## Local README as triage

A useful local README makes it cheap to decide whether deeper reading is
warranted. Depending on the material, useful cues include:

- original purpose and current use;
- important positive, negative, mixed, or superseded result;
- what the evidence does and does not establish;
- producer-generated inputs and consumers;
- authoritative code, data, proof, artifact, and interpretation paths;
- terms or corresponding implementations that another agent may search;
- cheap checks versus commands that refresh retained evidence.

This is context to preserve, not a mandatory field list. Do not rewrite a clear
README merely to satisfy a schema.

## Producers, consumers, and similar implementations

Producer-generated datasets remain with or directly attributable to the
program and configuration that generated them. A consumer states what it reads
at the point of use: in code, a manifest, script/config, command, dataset
identifier, or README when a runtime path or scientific role would otherwise
be implicit.

Use stable names so `rg` can derive reverse dependencies. Do not present a
manually maintained producer-side consumer list as exhaustive. An obsolete
match can be checked; an absent dependency declaration may never be
questioned.

Several experiments may use similar copied scaffolding or corresponding
instrumented implementations. That alone does not justify a shared import.
Share a library when the maintained API is itself the intended dependency;
otherwise keep explicit implementations easy to find and inspect.

## Overlapping semantic views

The inventory above answers what physical directories exist. Additional views
may group work by mathematical object, method, implementation, thesis use,
status, or finding. Such views are useful when task terminology does not match
directory names. They must state their scope because they need not be
exhaustive and can become stale.

The current examples below route only to active immediate directories and a
few notable nested experiment paths. They are not a complete finding index.

### Mathematical object or thesis use

| Search terms | Start |
| --- | --- |
| HKO local maximum, certificate, neighborhood | `hko-local-maximum/` |
| rotated regular polygons, pentagon formula, Lagrangian products | `regular-products/` |
| combinatorial boundaries, cells, crossings | `combinatorial-cells/` |
| crosspolytope capacity | `crosspolytope/` |
| selected equality bodies, local behavior | `local-maxima-check/` |
| hostile `sys`, random/product search, data science | `sys-datascience/`, then `sys-landscape/` for older/search implementations |
| explanatory 4D-polytope figures | `visualization/` |
| thesis AI-use provenance | `ai-use/` |

### Implementation under study

| Search terms | Start |
| --- | --- |
| QP/HK2017, f64/exact/fallback routes | `dev-quadratic-program/` |
| flow graph, CH2021, tubes, closed words | `dev-flow-graph/` |
| gradient ascent, traces, endpoint diagnostics | `dev-gradient-ascent/` |
| `sys` prediction and branch continuation | `dev-sys-prediction/` |
| coordinate canonization | `canonization-t-search/` |

### Evidence role

| Search terms | Start |
| --- | --- |
| f64/exact intermediate numerics, retained QP soundness | `qp-error-bounds/` and `dev-quadratic-program/numerics-audit/` |
| correctness, axioms, agreement, minimum orbits, recovery | `verification/` |
| runtime, memory, counters, profiling | `performance/` |
| random/product datasets and invariant prepared tables | `sys-datascience/produce/` and `sys-datascience/prepare/` |
| prediction, tail analysis, source transfer, proposer methods | `sys-datascience/methods/` |
| algorithm-comparison routing rather than evidence | `algorithm-comparisons.md` |

## Artifacts and commands

Generated outputs are not hand-edited. Local commands distinguish:

- cheap compile or smoke checks;
- full producers writing disposable output;
- commands intentionally refreshing tracked evidence.

Recorded hashes and Git revisions are provenance aids, not compatibility
gates. A validator may warn that current code or input bytes differ from a
retained run, but byte drift alone must not block semantic validation. Such a
warning means retained interpretation may be stale: use the printed paths,
working directory, and run timestamp to inspect the corresponding Git history
before treating the new run as equivalent. Schemas, population contracts, row
identities, joins, mathematical checks, completeness, and corruption within an
artifact bundle may still fail validation.

Generated build trees, temporary outputs, and large raw data are not navigation
surfaces.

Absence from a semantic view or lexical search result does not establish that
no experiment exists. Scan the physical inventory, inspect plausible READMEs,
and search producer names, exact algorithm paths, symbols, synonyms, and stable
mathematical terms before making a project-wide negative claim.
