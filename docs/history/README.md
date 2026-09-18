# Retained review and recovery evidence

Current assignments belong only to [WORK_REMAINING](../WORK_REMAINING.md). This directory retains exact reviewed texts, feedback, result packets and recovery evidence needed to interpret current obligations. Old proposed next steps inside evidence are statements from that experiment, not renewed assignments.

## Read by question

- Human writing judgments: `review-evidence/human-feedback/`, `reviewer-trial/human/`, `ds-result-trial/human-feedback.md`, `ds-full-review/`, `ai-reflection-review/`, and `pentagon-chapter/user-reading-feedback.md`; matched reviewed inputs/PDFs remain alongside them. Also see `experiments/writing-quality/human-review/responses/`.
- Corrected whole-thesis assessment: `coordination/whole-exposition-assessment.md`; exact frozen PDFs and scoped agent findings: `whole-review/`. Current dispositions are in WORK_REMAINING.
- Reusable interpretation and observed interaction methods: [knowledge index](../knowledge/README.md). These are not discarded as obsolete plans.
- Review/authoring experiments: `reviewer-trial/`, `review-workflow-design/`, `scientific-writing-solutions/`, `writing-quality-research/`. They retain failed approaches and evidential limitations; none establishes a PASS gate.
- Source adoption evidence: `assembly-source-map/`, `candidate-assembly-integration/`; baseline replay boundaries: [source-recovery](source-recovery/README.md).

## Git-only retired material

[retired-paths.json](retired-paths.json) records each removed path, immutable commit, blob and reason. Superseded task queues, status cards, launch briefs and duplicate preservation snapshots need no fulltext in HEAD. Retrieve any entry with:

```sh
git show <entry-commit>:<entry-path>
git grep -n 'search terms' <entry-commit> -- docs/history
```

Those commits are ancestors of the resume branch; normal repository history retains them. For original pre-migration path interpretation, use `35f29db4` and `docs/resume/layout-migration.json`. Exact transcripts and source copies deliberately preserve their original references; an old absolute worktree or temporary path is historical provenance, not a promise that the path remains live. The original baseline replay route is documented separately above.
