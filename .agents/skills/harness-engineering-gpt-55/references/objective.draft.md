# GPT-5.5 Harness Migration Notes

<!--
This draft is working material for redesigning the repo-local harness around
GPT-5.5. It is not active project policy.

During this migration, all old harness material is legacy input. Existing
`AGENTS.md` sections, `SKILL.md` files, `.codex/reference/harness/*`, subagent
definitions, task-routing conventions, and prompt packets may still be useful,
but they should be treated as suggestions and evidence to improve upon rather
than as settled target-state policy.
-->

## Scope

This file is about maintaining the harness for this repo during the GPT-5.5
migration.

It does not teach general harness engineering. Jörn owns general
harness-engineering judgment and design pivots. Agents may propose local
repairs, identify contradictions, and implement agreed changes, but they should
not independently move the repo to a different harness design.

## Migration Goal

The migration goal is to convert the current harness into a GPT-5.5-oriented
harness for this repo while using Jörn's harness-engineering judgment for design
choices.

During migration:

- Treat old harness files as legacy input, not settled policy.
- Preserve behavior that is currently helping agents succeed.
- Remove, downgrade, or mark material that reflects old-model scaffolding,
  stale paths, stale authority, historical context, or process instructions that
  no longer serve this repo.
- Prefer concrete objective, authority, evidence, side-effect, and stopping
  criteria over detailed process descriptions.
- Do not document the post-migration state here yet. That should happen after
  the migrated harness settles.

## Maintenance During Migration

When maintaining this repo's harness during the migration:

- Keep changes tied to this repo's thesis work and observed agent behavior.
- Preserve current harness behavior that is helping agents succeed.
- Remove, downgrade, or mark legacy text when it reflects old-model
  scaffolding, stale paths, stale authority, or historical context.
- Prefer concrete objective, authority, and evidence language over process
  descriptions.
- Do not convert Jörn's design ideas into binding rules unless the rule is part
  of the chosen local design or prevents a known expensive failure.
- Do not use the repo to document general harness-engineering theory. Put only
  the local consequences that future agents need to maintain this harness.
- Keep proposed edits small enough that Jörn can review the design consequence.

## Agent Role

Agents working on this migration should:

- inspect current harness files before editing;
- treat the old harness as evidence and suggestions, not target-state policy;
- surface contradictions, stale assumptions, and possible simplifications;
- implement local edits once Jörn has settled the design direction;
- report what changed and which validation was run.

Agents should not:

- pivot the harness design;
- add general theory unless it changes concrete repo maintenance behavior;
- preserve text only because it sounds useful;
- promote temporary or historical notes into current authority.

## Later Documentation

After migration, document the post-migration harness state separately. That
future documentation should describe the settled local harness, not the migration
process and not general harness-engineering theory.
