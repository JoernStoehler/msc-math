# Harness Migration Plan Draft

<!--
Working draft for the GPT-5.5 harness migration in this repo. This is not
active project policy.
-->

## Migration Loop

Migrate the harness one aspect or file group at a time.

For each cluster:

1. Pick the file group.
   Examples: `AGENTS.md`, skill taxonomy, one skill, `.codex/reference/harness`,
   task-routing conventions, subagent definitions.
2. State the local target state for that cluster.
   This is a design target for the next edits, not final global documentation.
3. Audit current files against the target state.
   Classify material as:
   - keep: still useful for GPT-5.5 and this repo;
   - delete: stale, generic, old-model scaffolding, or no longer worth cost;
   - downgrade: from binding rule to context or suggestion;
   - move: right content, wrong file or skill;
   - rewrite: right objective, bad wording or wrong specificity;
   - split/merge: wrong granularity;
   - open: requires Jörn's design judgment.
4. Extract surviving content into a review packet.
   The extraction should identify repo-local commitments, decisions, failure
   modes, and validation needs. It should not preserve the old file's
   presentation, section shape, or process choreography unless that structure is
   itself the useful content.
5. Delete or move old active files once their surviving content has been
   extracted and Jörn has had a chance to reject the extraction.
6. Write fresh target files only after their active purpose is clear. Do not
   create an active skill merely to hold draft process notes or migration
   theory, and do not treat old files as drafts to polish in place when their
   purpose, authority, or granularity is wrong.
7. Separate mechanical work from judgment work.
   - Use `sed`, small scripts, or fast text-rewriter agents for obvious stale
     paths, repeated wording, moves, formatting, and bounded classification.
   - Use GPT-5.5-level judgment for target-state reasoning, rewriting,
     splitting/merging skills, authority wording, and objective/evidence
     wording.
8. Validate the touched files.
   Use file-appropriate checks such as skill validation, `git diff --check`,
   `scripts/toc.sh`, stale-reference searches, or fresh reviewer/subagent
   probes when wording is meant to change agent behavior.
9. Commit a scoped checkpoint.
10. Record the decision and remaining open questions in chat. Promote a compact
   tracker only if chat compaction stops being enough.

## What To Track

Track lightly while the design is moving:

- local target state;
- files or file groups in scope;
- decisions made;
- open questions for Jörn;
- progress status: not started, audited, target stated, patched, validated,
  committed, or deferred.

## Target-State Descriptions During Migration

Writing a target-state description is part of migration. The target state says
what this repo currently wants for a specific harness aspect so that edits have
a direction.

Treat these descriptions as local and revisable. After migration evidence or
live use, update the target and continue. Do not treat an early target-state
description as final global harness documentation.

## Likely Migration Order

Initial order:

1. Migration framing notes.
2. Skill taxonomy.
3. Keep harness-edit process notes in draft form unless a future active skill
   has a clear, non-circular purpose. The attempted `harness` skill was deleted
   as not useful enough and potentially harmful.
4. `AGENTS.md`.
5. High-use procedural skills:
   - `pre-merge`
   - `review`
   - `subagent-delegation`
   - `verification`
   - `cached-map-maintenance`
   - `roadmap-maintenance`
6. Domain convention skills:
   - Rust
   - experiment
   - Python
   - formal math
   - thesis TeX
   - dataset conventions

This order can change when Jörn chooses a different local target or when a
blocking contradiction appears.
