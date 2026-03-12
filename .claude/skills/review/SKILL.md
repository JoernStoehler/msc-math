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

Each concern has a checklist reference doc in `references/` with detection rules, grep patterns, and verification procedures. The subagent reads the relevant checklist(s) and works through items sequentially.

### Phase 0 — Module sanity (run first if builds might be broken)

**Module sanity** — builds, tests, pipeline consistency, data freshness. Checklist: `references/checklist-modules.md`.

### Phase 1 — Syntax and style (fix before phase 2)

**LaTeX style** — file headers, environments, comment conventions, figure/table inclusion, label format, build warnings, mechanical anti-patterns (AP4/AP5/AP7). Skills: `tex-format`, `tex-build`. Checklist: `references/checklist-tex-style.md`.

**Rust style** — coding conventions, module structure, cross-ref format, magic number docs, coordinate convention. Skill: `rust-conventions`. Checklist: `references/checklist-rust-style.md`.

**Python style** — script headers, paths, error messages, figure sizing, DPI, visual quality, colors, caption epistemology. Skill: `python-conventions`. Checklist: `references/checklist-python-style.md`.

**Figure visual quality** — view each PNG with the Read tool, check for title collisions, label clipping, font readability at 5.4" width, legend overlap, layout balance, LaTeX rendering. Checklist: `references/checklist-python-figures.md`.

**Notes style** — README structure, assumptions documented, experiment philosophy alignment. Covered in Part C of `references/checklist-experiment.md`.

### Phase 2 — Semantics and content (on clean files)

**LaTeX math correctness** — proofs: gaps, unclear steps, mistakes, definition mismatches. Each proof one-by-one. Flag for Jörn's verification. Skill: `tex-content`. Checklist: `references/checklist-tex-math.md`.

**LaTeX pedagogical quality** — audience fit, forward refs, emphasis proportional to importance, standard definitions, semantic anti-patterns (AP2/AP6/AP9/AP10). Skill: `tex-content`. Checklist: `references/checklist-tex-educational.md`.

**LaTeX factual accuracy** — claims vs evidence: numbers vs data files, code refs vs actual code, citations vs bibliography.bib. Checklist: `references/checklist-tex-facts.md`.

**Rust math-code correctness + test quality** — doc comment formulas match code, invariant enforcement, test philosophy, coverage, input diversity. Skills: `rust-conventions`, `rust-tests`. Checklist: `references/checklist-rust-content.md`.

**Experiment accuracy + interpretation** — reported facts vs JSONL/output data, overreach, editorializing, causal claims from correlations, README quality. Skill: `experiment-conventions`. Checklist: `references/checklist-experiment.md`.

## How to spawn reviews (main agent)

Spawn the `review` subagent (defined in `.claude/agents/review.md`) with the Agent tool. It preloads all convention skills. Specify concern + files + report path in the prompt.

See `references/how-to-spawn-reviews.md` for concrete examples and the spawning pattern.

Spawn multiple instances in parallel with `run_in_background=true`. Each writes its report to a separate file. Agent teams (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`, already enabled) are an alternative for larger reviews where reviewers benefit from communicating with each other.

## How review subagents must work

Each subagent then follows this workflow:

### Step 1: Read everything first
Read all assigned files in full. Don't skim — read completely. Understanding context prevents false positives.

### Step 2: Load the checklist
Read the checklist reference doc for your concern (listed under "Review concerns" above). This contains the specific detection rules, grep patterns, and verification procedures. Use the task/todo tool to track items.

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
