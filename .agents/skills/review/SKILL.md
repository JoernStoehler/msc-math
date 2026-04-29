---
name: review
description: Repo review workflow for changed files, drafts, proofs, claims, figures, and pre-merge checks. Use when asked to review work, when preparing work for Jörn, when running pre-merge review, or when a subagent receives a review assignment.
---

# Review

## Core Workflow

1. Read the assignment and identify the changed or named files.
2. Identify the review surface: committed branch diff (`git diff main...HEAD`), single commit (`git show HEAD`), uncommitted diff (`git diff HEAD`), or the exact named files.
3. Read `AGENTS.md`.
4. Load matching convention skills by their descriptions. If the parent prompt names skills, load those first. For exact-file prompt audits, load only skills that change how those files should be judged.
5. Load only the review reference files below that match the assignment.
6. Read target files in full before reporting findings. If the assignment names
   a snippet-only review, survey pass, or large/generated artifact, review the
   assigned surface and state the partial surface explicitly.
7. Verify each finding against the cited file or command output before including it.

Use batched reads for independent files. Do not edit files during a review unless the assignment explicitly asks for fixes.

## References

- Rust code: `references/rust.md`
- Python scripts and figures produced from them: `references/python.md`
- Formal math and proof surface checks: `references/formal-math.md`
- Thesis `.tex`: `references/thesis.md`
- Factual claims against data, code, figures, or bibliography: `references/claims.md`
- Figure production chain and rendered PNGs: `references/figures.md`

## Refactor Checklist

When reviewing a simplification or helper-extraction patch, explicitly scan for
these items even if most will be green:

- behavior drift: changed control flow, filtering, ordering, thresholds, or
  serialization semantics
- lost math-code correspondence: removed formal labels, explanatory comments, or
  stated invariants on moved non-trivial code
- stale duplicate surfaces: old constants, helper copies, or comments left
  behind after the shared helper moved
- boundary widening: a local cleanup that now hides real policy differences
- compatibility risks: output paths, checkpoint shapes, CLI behavior, or
  tracked-artifact handling changed unintentionally

## Output Format

Findings come first, ordered by severity.

For each finding:
- Severity: `FIX` for a clear violation or bug, `FLAG` for judgment required.
- Location: file and line number when available.
- Evidence: what the file or command shows.
- Action: the concrete change or decision needed.

Then include:
- Review surface: the command or exact file list reviewed.
- Open questions or assumptions.
- Test or build gaps.
- Brief change summary only if it helps interpret the findings.

If no issues are found, say that clearly and list the remaining verification gaps.
