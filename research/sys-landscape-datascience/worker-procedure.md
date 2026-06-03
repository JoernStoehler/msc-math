# Sys-Landscape Data-Science Worker Procedure

<!--
Migration-review note: this is hard-earned process knowledge, not a polished or
approved permanent workflow. Use it as WIP guidance for future data-science
worker waves, and reassess before relying on it as the default process.
-->
Reusable procedure for bounded sys-landscape data-science worker packets. Not a
task queue. Current route and thesis relevance live in
`tasks/planning-notes.md`; idea status lives in the sys-landscape data-science
ledger/audit files.

## Lead Rules

- Use v1 subagents with `fork_context=false` after exact-reply and required-cwd
  smoke checks pass.
- Do not use the legacy/full-history launch path for this workflow.
- Give each worker an isolated worktree and, for new methods, an isolated method
  folder under `experiments/sys-landscape/datascience/methods/<slug>/`.
- Do not refactor shared Python helpers during a wave unless repeated completed
  reports prove that a shared helper reduces review cost.
- If a worker times out, inspect the worktree, durable report path, temporary
  output paths, and running processes before deciding whether to message, wait,
  or close the worker.
- If there are no files and no relevant process, send at most one corrective
  message. If the second inspection still lacks required artifacts, classify the
  attempt as `bug-redo` or `lead-repair`.

## Launch Terms

- `v1 subagents`: the available multi-agent tool for this session. If that tool
  is unavailable, write the packet to `/tmp` and ask Jörn before changing the
  workflow.
- `exact-reply`: a precheck where the worker repeats a required short string or
  field list exactly, to catch prompt-following failure before costly work.
- `required-cwd smoke check`: the worker first prints `pwd` and runs the named
  cheap command from the assigned worktree before doing the experiment.

## Worker Packet Fields

- idea slug and blocker target;
- required cwd/worktree and a first command that prints `pwd`;
- frozen dataset path, producer command, row counts, max `sys`, and `sys > 1`
  count;
- question, hypothesis, verdict meanings, allowed write scope, runtime budget,
  stop conditions, and required repo-owned evidence path;
- leakage/provenance guards and relevant statistical or numerical checks;
- required report header and sections.

## Result Review

- A completed result needs report, command/provenance, observation, inference,
  verdict, checks, caveats, thesis-use proposal, reopen trigger, and evidence
  paths.
- Result qualifiers: verdict, evidence strength, implementation trust, thesis
  use, caveat, reopen trigger.
- Dispositions: merge/promote, reject/trash, follow-up branch, bug-redo, future,
  rejected-low-VOI, lead-repair, positive-escalate.
- Code-only output does not close a blocker.
- A no-search-output result with low implementation trust does not close a
  before-submission blocker.
- A positive-escalate result stops the wave and goes to Jörn before more
  scaling.
