# Repo Conventions Draft

<!--
Working draft for rebuilding repo conventions after the GPT-5.5 harness reset.
This is not active policy. It tries one structure: abstract instrumental
objectives first, then surface-specific consequences.

Why keep objective text near convention bullets: agents often enter with a
concrete surface in mind, while the reason a convention exists is often
cross-cutting. Keeping the objective visible should make it easier to improve or
delete the convention without turning it into unexplained process.
-->

## How To Use This Draft

- Treat the bullet lists as incomplete.
- Prefer adding missing objectives or better measurements before adding many
  surface-specific rules.
- Use `<!-- comments -->` for non-obvious why-context that should not distract a
  first read.
- Promote content out of this draft only after the target surface is clear.

TODO: decide whether final conventions are grouped by objective, by surface, or
by a short cross-cutting file plus surface skills.

## Quick Navigation

Objective: agents find the relevant files, sections, symbols, and related
surfaces in few sequential actions.

Observable signals:

- Agents can state which surface owns a fact before editing it.
- Agents do not repeatedly open unrelated files before finding the relevant
  surface.
- Agents do not miss relevant source surfaces in post-mortems or reviews.
- Search results have high precision because names and references use stable
  terminology.
- Maps and indexes point outward to live source truth instead of duplicating
  stale detail.

General conventions:

- Use speaking filenames, section names, skill names, and symbol names.
- Prefer one hop with enough listed options over many small hops that each hide
  most of the remaining search space.
  <!--
  This is the intended meaning of "flatten indirection": e.g. one map listing N
  relevant surfaces is often cheaper for agents than log(N) nested maps with
  two choices each, because every hop costs context and can branch wrong.
  -->
- Use grep-friendly standard terminology. Avoid metaphors, local coinages, and
  vague placeholder words when a standard term or literal phrase exists.
- Mention related files with explicit paths, labels, symbols, or commands when
  that relation affects future work.
- Distinguish source truth, cached summaries, draft material, historical
  extraction, and raw git-history fallback.
- Delete stale navigation surfaces instead of preserving them as decoys.

TODO: decide how to measure "few sequential actions" without adding process
overhead.

### Skills

- Skill names and descriptions are the first navigation layer. They should tell
  agents when to load a skill and when not to.
- Do not list every skill in `AGENTS.md`; active skill names/descriptions are
  discoverable outside the file.
- A skill body should make its authority and surface clear before giving local
  suggestions.
- Use draft references for migration material or unsettled convention designs.

### `AGENTS.md`

- `AGENTS.md` is the always-loaded repo map.
- It should help all agents quickly identify the project, major source
  surfaces, harness surface shape, and common commands.
- It may carry broadly useful context such as project objective, quality
  objectives, environment notes, and quick commands.
- It should not become a complete skill index, detailed workflow manual, or
  duplicated source summary.

### Map And Index Files

- `MAP.md` / `INDEX.md` files should answer "where do I look next?"
- They should state enough ownership and routing context to prevent wrong
  first reads.
- They should link to source truth rather than copy implementation detail.

TODO: specialize for `research/INDEX.md`, `crates/MAP.md`, and
`experiments/MAP.md` after their current roles are rechecked.

### Formal Math LaTeX

- Use stable `\label{...}` names and `\ref{...}` references.
- Make label lookup cheap:

```bash
rg -n -A 10 -F '\label{LABEL_NAME}' formal/*.tex
```

- Make thesis label number/page lookup cheap after a thesis build:

```bash
perl -ne 'if (/\\newlabel\{LABEL_NAME\}\{\{([^}]*)\}\{([^}]*)\}/) { print "number=$1 page=$2\n" }' thesis/build/main.aux
```

TODO: decide whether formal label prefixes belong here or in a future
`formal-math` surface.

## Easy To Understand

Objective: agents understand what a file or section claims, does, and does not
claim without spending much reasoning budget on wording or structure.

Observable signals:

- A fresh agent can summarize the file's role without inventing missing
  authority.
- Reviewers do not flag ambiguity about whether text is a hard constraint, soft
  suggestion, draft, history, or source truth.
- Agents do not need to ask Jörn about what the text literally means before
  doing ordinary agent work.
- Agents can remove or revise text without preserving it only because its
  purpose was unclear.

General conventions:

- Use plain, literal language.
- Prefer standard terminology narrowed by nearby words over new terminology.
- Comment the why; make the what clear in names, structure, or code.
- If the why is long or mostly design history, put it in an adjacent draft,
  reference note, task note, or commit message rather than inline prose.
- State strength explicitly: hard constraint, soft suggestion, example,
  candidate, draft, historical fact, inference, or Jörn decision.
- Use sections to group related content and narrow scope.
- Avoid unnecessary clarifications that create false search hits or reduce
  recall of the important sentence.

Possible checks:

- Ask a fresh non-fork subagent what the file says, does not say, and where it
  is ambiguous.
- Ask a fresh non-fork subagent which parts looked obvious or unnecessary.
- Search for temporary chat terminology, metaphors, or vague words; rewrite
  only true positives.

<!--
"Possible checks" means these are optional instruments. They become worthwhile
when ambiguity would be expensive, when a file is meant to steer many future
agents, or when Jörn asks for review. They are not required every time because
running them has a cost and can slow trivial edits.
-->

### Rust Code

- Reference mathematical theory in `formal/` instead of trying to reproduce
  proofs in comments.
- Keep orchestration code understandable through direct local control flow
  where possible.

TODO: rebuild Rust-specific clarity conventions from current code and the old
harness extraction.

### Harness Text

- Avoid analogies when a literal objective or authority statement works.
- Do not make a process binding unless the path is part of success or prevents
  a known expensive failure.
- Use `TODO` when a list is intentionally incomplete.

## Maintainable Across Future Sessions

Objective: the repo and harness should converge over long project timelines:
errors, stale text, missing conventions, and overlooked improvements should
eventually become easier to detect and fix rather than accumulating.

Observable signals:

- Stale or contradictory instructions are found and removed during ordinary
  work.
- New insights have an obvious destination or are intentionally left in `/tmp/`
  as non-durable.
- Agents can update one surface without silently breaking another surface's
  authority.
- Reviews and post-mortems identify recurring failure modes and route them to
  the right durable surface.
- The repo does not preserve unused instructions, generated caches, or
  historical notes as active guidance.

General conventions:

- Delete unneeded content to reduce maintained surface area.
- Prefer durable files only when the future benefit justifies maintenance cost.
- Use `.draft.md` for unfinished, unsettled, or migration-only material.
- Keep non-obvious why-context recoverable without forcing every first reader
  through it.
- Do not optimize for smallest textual diff when a clearer replacement changes
  more lines.
- Avoid touching unrelated content; wide propagation is appropriate only when a
  real insight affects multiple surfaces.
- Make invalidation triggers visible for cached summaries.

<!--
The convergence objective is intentionally strong. A long-running agent-heavy
project needs mechanisms that tend to repair drift over time. If a convention
does not converge eventually, it can become harmful quickly because future
agents inherit stale active text without shared human memory.
-->

### Harness

- Separate active policy, draft design, old extraction, and git-history archive.
- Do not repair references inside old surfaces that are being deleted.
- Before deleting a large active surface, preserve a usable extraction unless
  Jörn says git history is sufficient.
- Record non-obvious retention/deletion reasoning in comments, adjacent drafts,
  task notes, or commit messages.

TODO: decide which why-context should be HTML comments versus adjacent files.

### Task And Roadmap Files

- Preserve steering decisions that are expensive for Jörn or agents to
  reconstruct.
- Prune stale schedules, obsolete ownership, old packet queues, and derivable
  state.

TODO: rebuild task/roadmap conventions against current `tasks/README.md`,
`ROADMAP.md`, and the old extraction.

## Verifiable And Correct

Objective: claims, code, math, data, and harness statements should have
checkable support at the level needed for the task.

Observable signals:

- Claims point to source files, data, commands, labels, or explicit Jörn
  decisions when the support is not obvious.
- Verification commands or review checks are close to the surfaces they check.
- Agents report residual gaps instead of silently treating a passed command as
  task success.
- Mathematical judgment is separated from agent-checkable support.

General conventions:

- State the evidence type: proof, formal note, code, test, experiment, data,
  rendered artifact, command output, source citation, or Jörn decision.
- Keep cheap checks cheap and near ordinary development loops.
- Keep broad or slow validation in experiment or verification surfaces.
- Do not ask Jörn to perform agent-checkable inventory, grep, build, or
  comparison work.

TODO: decide whether this objective becomes `repo-quality`, `verification`, or
surface-local sections.

## Reproducible And Traceable

Objective: future agents can reconstruct how important thesis-facing results,
figures, datasets, and repo promises were produced or preserved.

Observable signals:

- Generated artifacts have identifiable producers and consumers.
- Thesis-facing figures/tables/data have provenance paths and interpretation.
- Smoke runs do not mutate canonical tracked data by accident.
- Preserved historical artifacts are distinguished from rerunnable outputs.
- Repo-facing promises match actual commands, prerequisites, and artifacts.

General conventions:

- Put generated data near the producer that owns it.
- Avoid multiple maintained producers for the same tracked artifact.
- Use targeted search over artifact filenames, producer declarations, thesis
  text, and research notes before rebuilding global provenance maps.
- Treat `.jsonl` as generated data, not hand-edited prose.

TODO: specialize after experiment and dataset conventions are rebuilt.

## Safe Agent Work

Objective: agents and subagents should advance work without corrupting state,
overwriting others, or pretending delegated evidence is authority.

Observable signals:

- Agents know the cwd/worktree before editing.
- Delegates receive bounded objectives and return checkable evidence.
- Top-level sessions verify delegated claims before reporting them as fact.
- Jörn is asked for decisions only after agent-checkable evidence is gathered.
- Commits are scoped rollback points.

General conventions:

- Treat tool default cwd as untrusted until checked.
- Use worktrees when isolation or parallel edits matter.
- Name required cwd in subagent prompts.
- Treat delegate output as evidence, not authority.
- Ask Jörn for mathematical judgment, thesis scope, advisor-facing framing,
  taste, external actions, and design pivots.

TODO: decide whether safe agent work belongs in a future `agent-work` skill.
