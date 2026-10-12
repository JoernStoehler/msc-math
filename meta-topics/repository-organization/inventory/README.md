# Repository inventory and cleanup

This is an inventory **of** project knowledge and capabilities for Jörn and
agents to inspect, regroup and use when changing the repository. It does not
replace the proofs, code, datasets or eventual explanatory thesis.

Open [the searchable inventory and review page](index.html). For the ideas
behind the entries, read [the research guide](../../../docs/research-guide.md).

The first survey checkpoint is assembled (10 October 2026). Coverage remains
incomplete; see each area’s declared reading limits below. Its physical
`files.json` census is frozen at the pre-migration 10 October checkpoint: the
generated view's file and directory counts do not describe the current topic
layout. The semantic source paths were updated after the 12 October migration.
An inventory row describes a
result, capability or retained collection, its evidence and current assessed
state. A file accounted for in the physical manifest is not necessarily read
or understood. Unknown quality/readability is not a negative verdict.

See [verification and remaining limits](verification.md) for source reviews,
reader observations, regression checks and delivery limitations.

See [the parallel cleanup plan](parallel-plan.md) for concrete geometry,
foundations, data and testing units. The inventory is updated alongside those
streams; its completion is not a prerequisite for starting them.
The [12 October migration result](migration-results-2026-10-12.md) maps former
and current owners, bounded checks, repaired regressions and remaining gaps.

The dated [repository organization and agent workflow comparison](../organization-design.md)
compares topic packets, retrieval tools, delegated compression, resource tradeoffs
and maintenance lifecycles. It informs the current migration but is not its
canonical contract or task list. [Repository organization](../README.md)
owns the current location and retention contracts; [coordination](../../../docs/coordination/README.md)
owns current assignments and handoffs.

## Survey record contract

Topic workers own separate JSON files in `parts/`. Each has `scope`, `coverage`
(a plain explanation of searched/read/uninspected material), `items`, and
`changes` (actual edits and their verification; empty if none).

Each item has:

- `id`: stable topic-prefixed identifier; `kind`: result, result-family,
  algorithm, dataset, figure-set, workflow, reference, or collection.
- `title`, `summary`: identify what exists without rewriting its full content.
- `state`: support/status with uncertainty, not a generic pass label.
- `agent_readability`, `human_readability`: sourced observations or unknown.
  Having a proof file does not establish readability or human acceptance.
- `runtime`: quoted execution cost with source/environment, or unknown/not
  applicable. Do not rerun long producers merely to fill this field.
- `code_quality`: scoped inspected quality/check evidence, or not applicable /
  unassessed; separate numerical contract from ordinary maintainability.
- `sources`: list of objects with `path` and `supports` (what the source
  establishes). Use actual repository-relative paths, no fabricated anchors.
- `relationships`: list of objects with `relation` and `path`; e.g. supported
  by, generates, consumes, supersedes, implements, or included in.
- `action`: keep, clarify, consolidate, relocate, remove, investigate (or a
  concrete combination); `rationale`; `uncertainties`: list of actual gaps.

Prefer one substantive result per row. Use `result-family` for intentionally
grouped results and name the members in the summary. Do not present counts of
families, duplicated theorem declarations, or files as the number of distinct
theorems. Duplicate identities across topics are reconciled by the integrator.

## Simplification and changes

Simplification happens alongside discovery: correct demonstrated false current
information; remove redundant syntax narration and purposeless current guidance;
preserve costly rationale, assumptions, rejected alternatives, provenance and
recovery information. Unclear value is an investigation item, not proof of
irrelevance. Git history provides rollback for tracked edits; shared caches and
producer effects require separate handling.

Whole-crate refactoring, cross-crate API movement and changed note conventions
are within the user's scope. Choose them for demonstrated value, track their
interfaces and integrate/check affected consumers. No fixed future layout is
selected merely by this data format.

Workers own disjoint file areas. A proposed change crossing those areas goes to
the integrating root before mutation so other work is not overwritten. A worker
may delegate bounded discovery to Sol/Luna; each child has an owned output and
stop point. The root owns identity reconciliation, the human view, coverage and
cross-area integration.


## Regenerating the view

```sh
python3 scripts/build-repo-inventory.py
python3 scripts/build-repo-inventory.py --check
```

The builder validates the topic records and their paths, reconciles explicitly
identified duplicate results in `reconciliation.json`, and renders `index.json`
and `index.html`. It does not validate mathematical implications or human
readability. `files.json` is a dated physical census, not a list of semantically
reviewed files. Update source records and the census as the repository changes;
do not hand-edit the generated view.
