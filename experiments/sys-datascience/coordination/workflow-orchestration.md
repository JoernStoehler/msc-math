# Sys-Datascience Workflow Orchestration

Use: operational conventions for running the sys-datascience thesis slice with
parallel sessions and subagents. This file describes how agents coordinate.
Read `workflow-design-rationale.md` for why this workflow is predicted to work.

## Roles

This agent helps Jörn decide what research sessions to spawn, stop, split,
merge, or rescope across the whole sys-datascience thesis slice. It keeps a
global view, looks for missing topic areas, notices opportunity-cost issues,
and turns vague uncertainties into candidate topic seeds. It does not execute
experiments deeply. Once a line becomes substantial, it should hand it to a
topic-owning session. Its externally useful output is session-decision advice:
which sessions should exist, what each should own, why it matters, and which
considered topics are not worth spawning yet.

Handle: `surface scout` or `global research scout`.

This agent maintains the cross-topic research state: recent global belief
state, live hypotheses, evidence/update traces, open discriminators, parked or
tainted work, and prioritization-relevant uncertainty. It should be
source-linked and conservative about cross-topic conclusions. It should not
record every brainstormed idea or local experiment detail.

Handle: `research-map steward`.

This agent owns one research topic after it has become worth sustained
attention. It develops the local ontology of questions and hypotheses, designs
packet-sized experiments, interprets reviewed packet results in that topic, and
keeps the topic's local notes coherent. It can spawn packet executors and
reviewers.

Handle: `topic owner` or `topic research lead`.

This agent executes one bounded experiment or code packet end to end. It
receives the motivating question, relevant context, expected artifacts, review
standard, and stopping condition. It writes code/data/docs needed to reproduce
and interpret the packet, then reports externally relevant results, changed
files, commands, and risks.

Handle: `packet executor`.

This agent reviews a packet for whether it should update topic beliefs, be
revised, be parked, be rewritten, or be discarded. It checks code correctness,
reproducibility, artifacts, provenance, interpretation claims, and whether the
packet actually answers its motivating question.

Handle: `packet reviewer`.

This agent focuses on what a packet result means after code/artifacts are
plausibly correct. It extracts belief updates, caveats, implications for other
hypotheses, and next discriminating questions. It should be willing to say
"this result is real but not globally important."

Handle: `interpretation reviewer`.

## Fresh-Session Read Path

All sys-datascience research sessions should start with `README.md` and this
file. Then:

- surface scouts read `next-session-candidates.md`, `research-ledger.md`,
  `workflow-design-rationale.md`, `process-learnings.md`, and relevant
  `topics/*.md`;
- research-map stewards read `research-ledger.md`, `process-learnings.md`, and
  topic files whose updates may affect global prioritization;
- topic owners read `workflow-design-rationale.md`, `process-learnings.md`,
  their topic file, adjacent topic files likely to transfer ideas, and the
  packet READMEs/artifacts named by those topic files;
- packet executors read the prompt, the relevant topic file, and the source
  files named by the prompt; they should not need to reconstruct the whole
  global map;
- packet reviewers read the packet branch/artifacts, the motivating topic file,
  and the review criteria in `workflow-design-rationale.md`;
- interpretation reviewers read the packet artifacts, the motivating topic
  file, and any cross-topic files whose hypotheses the result might affect.

## Downstream Uses

- A topic lead resumes after compaction or a deep dive by rereading its own
  topic file to recover what it currently thinks, why, and what it was about to
  try next.
- Another topic lead reads a topic file to borrow hypotheses, experiment
  architecture, feature designs, failed approaches, or interpretation patterns.
- A surface scout or topic lead uses a topic file to write a packet-executor
  prompt with motivation, context, success criteria, caveats, and known
  low-value paths.
- Jörn or a surface scout uses this folder to decide which research sessions to
  spawn, stop, split, merge, or rescope.
- A packet reviewer reads the relevant topic file to understand what the packet
  was supposed to answer and which claims would matter.
- An interpretation reviewer compares packet results against the topic file's
  prior beliefs and extracts what changed, what stayed uncertain, and what
  became tainted.
- A thesis-writing session uses topic files as an index to candidate thesis
  claims, figures, caveats, and source artifacts. It must still verify against
  packet artifacts.
- Future sessions use topic files to avoid rediscovering known low-value ideas
  unless a new reason appears.

## Update Fanout

- Update topic files for local topic beliefs, hypotheses, packet prompts, and
  topic-owner resumption state.
- Update `next-session-candidates.md` for current spawn/rescope/stop state.
- Update `research-ledger.md` when a topic-level result changes cross-topic
  beliefs, thesis wording, or global prioritization.
- Update `process-learnings.md` when a workflow failure or success changes
  future agent behavior.
- Update `prompt-templates.md` when a recurring prompt shape becomes useful.

## Packet Lifecycle

1. A surface scout or topic owner records a seed question and why it may matter.
2. A topic owner sharpens the seed into a packet objective, expected evidence,
   likely costs, and interpretation boundaries.
3. A packet executor runs the packet in a worktree or subagent-owned branch.
4. A packet reviewer checks code, artifacts, provenance, and claims.
5. An interpretation reviewer or topic owner extracts belief updates and
   caveats.
6. The topic owner decides whether the packet should cause topic-level belief
   updates, further work, parking, rewrite, or discard.
7. The research-map steward updates the global ledger when a topic-level update
   affects prioritization, thesis wording, or future packet choice.

Packets may stop early when the result is decisive, when the implementation
shows the idea is malformed, or when opportunity cost dominates further polish.

Workflow-test prompts and reports should carry this visible header:

```text
Workflow-test: yes/no
Research conclusions may update beliefs: no unless later normal review
Process evidence to report:
```

Use `Workflow-test: yes` only when the packet is selected mainly to learn about
the workflow or prompt material rather than because its research output is a
priority.

## Storage Rules

- Put durable global coordination and cross-session working memory in this
  folder.
- Put topic-specific active notes in `topics/*.md` unless a topic owner creates
  a better local layout.
- Put experiment code, generated data, and packet-local interpretation in the
  owning method, producer, prepare, or topic experiment folder.
- Do not make this folder a second source of truth for generated metrics.
  Link to packet artifacts and summarize only the belief update.
- Each owned file should say what use it is optimized for and how it is being
  maintained.
- Topic owners may split, replace, or restructure their own surfaces when the
  use case changes. Preserve or link the old surface when other sessions still
  rely on it.
- Use `/tmp` for scratch prompts and chat drafts only.
- Do not keep uncommitted coordination state on main. Work in a branch or
  topic worktree, then merge after review.
