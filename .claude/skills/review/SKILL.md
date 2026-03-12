---
name: review
description: Review workflow for checking code and writing quality before presenting to Jörn. Load when reviewing .tex, .rs, or .py deliverables, or when asked to run a review. Explains sequential checklist methodology, phase ordering, and how to use review subagents.
---

# Review Workflow

## When to review

Mandatory before presenting `.tex` deliverables to Jörn. Recommended for all deliverables.

## Principle: fix syntax before semantics

Phase 1 (syntax/style) issues distract from phase 2 (content/correctness). Fix formatting, broken refs, and convention violations first. Then review semantics on clean files.

## How to run a review

1. Run `git diff main...HEAD --name-only` to identify changed files.
2. Decide which review concerns apply (see concern list below).
3. Spawn review subagents in parallel — one per concern per file group. Err towards running too many: agent time is free ($0/h), especially when parallelized and Jörn isn't waiting.
4. Fix phase 1 findings.
5. Spawn phase 2 subagents on the cleaned files.
6. Present merged report to Jörn.

## Review concerns

### Phase 1 — Syntax and style (fix first)

**LaTeX style** — file headers, environments, comment conventions, figure/table inclusion, label format, build warnings. Load `tex-format` skill.

**Rust style** — coding conventions, module structure, cross-ref format, magic number docs, coordinate convention. Load `rust-conventions` skill.

**Python style** — script headers, paths, error messages, figure sizing, DPI, visual quality, colors. Load `python-conventions` skill.

**Notes style** — README structure, assumptions documented, experiment philosophy alignment.

### Phase 2 — Semantics and content (on clean files)

**LaTeX math correctness** — proofs: gaps, unclear steps, mistakes, definition mismatches. Each proof one-by-one. Load `tex-content` skill. Flag for Jörn's verification.

**LaTeX pedagogical quality** — audience fit, forward refs, emphasis proportional to importance, standard definitions. Load `tex-content` skill.

**LaTeX factual accuracy** — claims vs evidence: numbers vs data files, code refs vs actual code, citations vs bibliography.bib.

**Rust math-code correctness** — doc comment formulas match code, invariant enforcement, thesis cross-ref content verification. Load `rust-conventions` skill.

**Rust test quality** — test philosophy, coverage patterns, input diversity, property verification. Load `rust-tests` skill.

**Experiment factual accuracy** — reported facts in .tex writeups vs actual JSONL/output data. Load `experiment-conventions` skill.

**Experiment interpretation quality** — overreach, editorializing, unlabeled speculation, causal claims from correlations.

## How review subagents must work

The main agent spawns generic review subagents, specifying:
- Which files to review
- Which concern(s) to focus on
- Which convention skill(s) to load

Each subagent then follows this workflow:

### Step 1: Read everything first
Read all assigned files in full. Don't skim — read completely. Understanding context prevents false positives.

### Step 2: Build a checklist
Based on the concern and the loaded convention skill, create a concrete checklist of items to verify. Use the task/todo tool to track items.

### Step 3: Work through items ONE AT A TIME
For each checklist item:
1. Search/grep for relevant patterns
2. Evaluate findings
3. Append findings to the output report immediately

Do NOT attempt to hold all items in working memory and write the report at the end. That produces 10% attention on 10 items instead of 100% attention on each item sequentially.

### Step 4: Summarize
After all items are processed, write a summary: total issues by severity, overall readiness, which sections are cleanest vs roughest.

## Output format

```
## [Concern]: [Files reviewed]

### Item 1: [checklist item]
- Finding: [what was found]
- Location: [file:line or rendered theorem number]
- Severity: FIX / LIKELY ISSUE / FLAG FOR JÖRN
- Suggested action: [concrete fix or question]

### Item 2: ...

## Summary
- N issues found (X fix, Y likely, Z flags)
- Readiness: [ready / needs fixes / needs Jörn attention]
```
