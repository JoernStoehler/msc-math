---
name: research-experiments-data
description: Use when Codex designs, implements, runs, reviews, interprets, documents, or delegates experiment work; changes experiment data, generated artifacts, reports, figures, provenance, or owner-local research state; or assesses whether an experiment packet can serve a downstream consumer. Provides shared conventions and routes to task-specific references for interpretation, packet readiness, and workflow refinement.
---

# Research Experiments Data

This skill supplies conventions shared by experiment work. Read only the
task-specific references that apply:

- read `references/interpretation.md` when translating outputs into evaluable
  mathematical or domain claims, including claim-level thesis usability;
- read `references/packet-readiness.md` when judging whether a combined packet
  can serve a downstream consumer;
- read both when interpretation is one component of a packet handoff;
- read `references/workflow-learning.md` only during explicitly authorized
  experiment-workflow refinement;
- use the relevant language, thesis-asset, or LICCA skill when those surfaces
  are involved.

## Owner-Local Research State

Write for the future agent that will update or use the experiment, and
indirectly for Jörn. Keep reasoning traceable when a conclusion is not obvious,
and preserve the epistemic status of claims.

Before designing follow-up work or interpreting results, read the current
owner-local question and downstream purpose. Useful distinctions can include
feasibility exploration, falsification, discriminating hypotheses,
strengthening weak evidence, making existing evidence legible, or maintaining
the implementation. These examples are not a required classification; record
the distinctions that affect actual choices.

Put durable knowledge where future work should update it. Common owners include:

- experiment or method-packet `README.md` files for local purpose, provenance,
  interpretation, disposition, and reopen constraints;
- generated artifacts or generated compact reports for detailed per-run rows;
- `thesis/*-content.md` companions for thesis-facing inventory, source pointers,
  caveats, and writing gates;
- `formal/` notes or TeX for proof development;
- crate documentation for reusable code contracts;
- a broader control surface only when no narrower owner can hold the state.

Link other owner or task surfaces when a change affects what they must find or
do. Do not create a top-level `research/` ownership layer. Split an experiment
or note when its purposes or update cycles interfere; do not split merely to
fit a fixed template.

## Reproducibility And Artifacts

- Keep data beside its producer unless an existing broader data owner applies.
- A result used by later research or the thesis needs a source-to-artifact
  route reproducible under the comparison contract appropriate to that use.
- Do not patch-edit generated JSONL, CSV, tables, or figures. Regenerate them,
  or document the required refresh when regeneration is outside the task.
- If tracked generated data changes unexpectedly, stop and identify the file
  and command before treating the change as evidence.
- Do not make hand-maintained README prose a second source of detailed generated
  metrics. Point to the artifact or generate the readable table/report.
- Prefer simple runnable producers and analyzers using the repo's supported
  environment. Document the entry command, inputs, outputs, and non-obvious
  dependencies.
- Provide a cheap smoke path when it materially reduces development or review
  cost; do not retain smoke artifacts that have no downstream use.
- Use LICCA and a Slurm script when the selected computation actually requires
  cluster execution; use `$licca` before preparing commands for Jörn.
- JSONL is the default for flexible row-oriented experiment data when it fits
  the producer and consumers. Use another format when its semantics or tooling
  are materially better, and keep that choice legible.
- Put shared code at the narrowest common owner of the experiments that use it.

Detailed experiment facts and metrics belong to their owners, not to this
skill. Revise this skill only for conventions that transfer across experiments.
