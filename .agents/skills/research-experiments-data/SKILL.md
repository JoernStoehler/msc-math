---
name: research-experiments-data
description: Use when Codex writes, edits, reviews, or delegates research notes, experiment design, experiment execution code, data/report/figure provenance, generated artifacts, or experiment-result interpretation in this repo.
---

# Research Experiments Data

## Owner-Local Interpretation Notes

- the audience is future agents, and indirectly (via chat) Jörn
- write plainly, focus on content, make reasoning traceable by providing arguments and intermediate steps instead of just conclusions whenever the elevated hypothesis alone is not obviously true already
- track the epistemic status of claims
- link the relevant owner and task surfaces when an interpretation note changes
  what future agents must find or do
- split experiments when it becomes hard to achieve multiple purposes/answer multiple questions in one experiment, copy and edit code cheaply
- track carefully the current prioritized subquestions/subgoals, in particular distinguish exploring the feasibility of an idea, strengthening the evidence of a weak result, aiming to falsify, aiming to distinguish between hypotheses, producing evidence that is more legible even though it contains no new/additional information, refactoring/cleaning the experiment for long-term maintainability, and so on. Often multiple subgoals can be pursued at once - but not always all of them.
- experiments should be reproducible from scratch given all related owner-local
  notes and artifacts
- repo state: current experiment work is organized around mostly settled main
  and side-result lines of inquiry; each experiment should support one line of
  inquiry unless an owner-local note explains otherwise

Put interpretation where future work should update it. Common owners are:

- experiment or method-packet `README.md` files for experiment-local purpose,
  result interpretation, provenance, and follow-up constraints;
- `thesis/*-content.md` companions for thesis-facing writing inventory, source
  pointers, caveats, fallback branches, and review gates;
- `formal/` notes or TeX files for proof-route and developer-facing
  mathematical interpretation;
- crate `README.md` or `DEVELOPMENT.md` files for reusable code contracts;
- `tasks/current-state.md` and `tasks/planning-notes.md` for cross-surface
  current-state caches and route reasoning.

Do not recreate a top-level `research/` ownership layer. Create a separate
owner-local note only when inline README/prose would become too large or mix
unrelated purposes.

## Experiments

- owner-local notes describe what experiments are for and interpret their
  results. Before interpreting results or planning follow-up experiments, use
  the relevant README, method packet, thesis companion, formal note, or task
  cache rather than inferring purpose from experiment artifacts alone.
- sibling experiments should be mostly independent from each other, to facilitate rapid development
- data is located next to the producer
- do not patch-edit generated `.jsonl`, `.csv`, or figure outputs; regenerate
  them or document the needed refresh
- if tracked generated data changes unexpectedly, stop and report the file and
  command
- use script-like python and rust binaries, make the pipeline simple and reproducible and documented
- for development, provide smoke paths (smoke input data, smoke output data, smoke parameter settings)
- for large datasets, provide a Slurm job script to be run on LICCA
- shared code is owned by the parent of the experiments that use it
- we use JSONL for data, because agents can manipulate it easily, and it is flexible enough for the Rust row types we have
