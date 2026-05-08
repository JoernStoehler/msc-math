---
name: research-experiments-data
description: Use when Codex writes, edits, reviews, or delegates research notes, experiment design, experiment execution code, data/report/figure provenance, generated artifacts, or experiment-result interpretation in this repo.
---

# Research Experiments Data

## Research notes

- the audience is future agents, and indirectly (via chat) Jörn
- write plainly, focus on content, make reasoning traceable by providing arguments and intermediate steps instead of just conclusions whenever the elevated hypothesis alone is not obviously true already
- track the epistemic status of claims
- link the relevant index and task surfaces when a research note changes what
  future agents must find or do
- split experiments when it becomes hard to achieve multiple purposes/answer multiple questions in one experiment, copy and edit code cheaply
- track carefully the current prioritized subquestions/subgoals, in particular distinguish exploring the feasibility of an idea, strengthening the evidence of a weak result, aiming to falsify, aiming to distinguish between hypotheses, producing evidence that is more legible even though it contains no new/additional information, refactoring/cleaning the experiment for long-term maintainability, and so on. Often multiple subgoals can be pursued at once - but not always all of them.
- experiments should be reproducible from scratch given all related research notes
- repo state: we now have the main and side results nailed down, and each experiment supports only one line of inquiry

## Experiments

- the research notes describe what the experiments are for, and interpret their results.
- `research/` owns experiment design, research purpose, and interpretation;
  `experiments/` owns execution code, commands, data, reports, and figures.
  Before interpreting results or planning follow-up experiments, use the
  relevant research note rather than inferring purpose from experiment artifacts.
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
