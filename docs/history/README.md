# Retained review and recovery evidence

Current assignments belong only to [WORK_REMAINING](../../meta-topics/project-management/thesis-work.md). This directory retains exact reviewed texts, feedback, result packets and recovery evidence needed to interpret current obligations. Old proposed next steps inside evidence are statements from that experiment, not renewed assignments.

## Read by question

- Human writing judgments: `review-evidence/human-feedback/`, `reviewer-trial/human/`, `ds-result-trial/human-feedback.md`, `ds-full-review/`, `ai-reflection-review/`, and `pentagon-chapter/user-reading-feedback.md`; matched reviewed inputs/PDFs remain alongside them. Also see `experiments/writing-quality/human-review/responses/`.
- Corrected whole-thesis assessment: `coordination/whole-exposition-assessment.md`; exact frozen PDFs and scoped agent findings: `whole-review/`. Current dispositions are in `docs/coordination/thesis-work.md`.
- Reusable interpretation and observed interaction methods: [knowledge owner router](../knowledge/README.md). These are not discarded as obsolete plans.
- Review/authoring experiments: `reviewer-trial/`, [workflow design](../../meta-topics/agent-workflow/review-workflow-design/README.md), `scientific-writing-solutions/`, `writing-quality-research/`. They retain failed approaches and evidential limitations; none establishes a PASS gate.
- Source adoption evidence: `assembly-source-map/`, `candidate-assembly-integration/`; baseline replay boundaries: [source-recovery](source-recovery/README.md).

## Git-only retired material

[retired-paths.json](retired-paths.json) records each removed path, immutable commit, blob and reason. Superseded task queues, status cards, launch briefs and duplicate preservation snapshots need no fulltext in HEAD.

The separate provisional conceptual goal map was retired on 10 October 2026
after Jörn said he did not use it and keeping such messages in the repository
proved unhelpful (inventory Q11). Its original model and rendered discussion
remain recoverable through the pinned entries; the selected execution dependency
graph has a separate purpose and remains current.

Retrieve any entry with:

```sh
git show <entry-commit>:<entry-path>
git grep -n 'search terms' <entry-commit> -- docs/history
```

Those commits are ancestors of the resume branch; normal repository history retains them. For original pre-migration path interpretation, use `35f29db4` and `docs/resume/layout-migration.json`. Exact transcripts and source copies deliberately preserve their original references; an old absolute worktree or temporary path is historical provenance, not a promise that the path remains live. The original baseline replay route is documented separately above.

## Operating evidence moved by purpose

Agent collaboration investigations, process-audit evidence and matched session
postmortem inputs now belong to [agent workflow](../../meta-topics/agent-workflow/README.md).
Reusable planning context belongs to [project management](../../meta-topics/project-management/README.md),
and dated code-organization analysis to [repository organization](../../meta-topics/repository-organization/README.md).
Exact frozen excerpts and manifests keep their original path strings as historical
provenance; consult those owners for maintained commands and current source routes.
Disposable exact message-review packets have been removed from active search;
[their retained findings](../../meta-topics/agent-workflow/message-review-retention.md)
record the consequential disposition and Git/scratch recovery boundary.
