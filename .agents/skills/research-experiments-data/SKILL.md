---
name: research-experiments-data
description: Use when Codex designs, implements, runs, reviews, interprets, documents, or delegates experiment work; changes experiment data, generated artifacts, reports, figures, provenance, or owner-local research state; or assesses whether an experiment packet can serve a downstream consumer. Provides shared conventions and routes to task-specific references for interpretation, packet readiness, and workflow refinement.
---

# Research Experiments Data

Read only the task-specific references that apply:

- `references/interpretation.md` for mathematical/domain claims and thesis
  usability;
- `references/packet-readiness.md` for whether a combined packet can serve a
  downstream consumer;
- both when interpretation is part of a packet handoff;
- `references/workflow-learning.md` only during explicitly authorized
  experiment-workflow refinement;
- the relevant language skill, `$thesis` for reader-facing figure/table/asset
  design, or `$licca` for LICCA commands.

## Purpose And Ownership

- Read the owner-local question and downstream use before inferring purpose
  from artifacts. Distinguish purposes that affect choices, such as feasibility,
  falsification, hypothesis discrimination, evidence strengthening,
  legibility, and maintenance; do not force a fixed taxonomy.
- Put local purpose, provenance, interpretation, disposition, and reopen
  constraints in the experiment or method-packet README; detailed rows in
  generated artifacts/reports; thesis inventory and caveats in thesis
  companions; proof development in `formal/`; reusable contracts in crate docs.
- Link another owner when a result changes what its future work must find or do.
  Do not create a top-level `research/` ownership layer.
- Split experiments when their questions or update cycles interfere. Sibling
  experiments normally consume shared data rather than push into each other.
  Promote code or data to their narrowest shared parent after multiple current
  consumers need it or duplication creates a concrete risk, not speculatively.

## Reproduction And Artifacts

- Keep data beside its producer unless an established broader owner applies.
  Preserve the command, inputs, parameters, non-obvious dependencies, and
  comparison contract needed for downstream use.
- Do not patch generated JSONL, CSV, tables, or figures. Regenerate them or
  record the missing refresh. Stop and identify unexpected tracked changes
  before treating them as evidence.
- Do not maintain README prose as a second store of generated metrics. Point to
  the artifact or generate a compact readable report.
- Provide a cheap smoke path when it materially reduces development/review cost
  and checks the relevant input, output, parameter, or resume contract. Do not
  retain smoke artifacts without downstream use.
- Use `$licca` before preparing commands for a run that belongs on LICCA.

JSONL currently works well for row-oriented heterogeneous records because it
streams and diffs cleanly, composes with Rust row types, and is directly
inspectable by agents. This is a design rationale, not a timeless mandate:
choose or migrate to another format when its task-specific properties are
better, and inspect current producers and consumers for the de-facto format.

Detailed experiment facts belong to their owners, not this skill. Revise this
skill only for conventions that transfer across experiments.
