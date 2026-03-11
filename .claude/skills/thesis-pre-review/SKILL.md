---
name: thesis-pre-review
description: Pre-review .tex content before handing to Jörn. Produces a confidence-graded report that catches surface issues so Jörn can focus on proof correctness and deep domain issues.
user_invocable: true
---

# Thesis Pre-Review

Perform a thorough preliminary review of the specified .tex content. Goal: minimize Jörn's review time by catching everything an agent can catch.

**You cannot reliably verify proofs.** You can overlook gaps, errors, and subtle logical issues. Be honest about this. But you *can* catch: typos, notation inconsistencies, unclear exposition, missing definitions, referencing errors, formatting issues, and sometimes suspicious proof steps.

## Output format

Structure the report with these sections:

### 1. FIX BEFORE JÖRN (high confidence)
Typos, broken refs, notation inconsistencies, formatting, undefined terms, TODO markers, bibliography issues. For each: location, issue, suggested fix.

### 2. LIKELY ISSUES (moderate confidence)
Unclear passages, proof steps missing justification, claims stronger than support, non-standard notation, structural issues. For each: location, issue, why uncertain, suggested action.

### 3. FLAGS FOR JÖRN (low confidence)
Places where something feels off but you can't determine if there's a real problem. Proof steps that might have gaps, possibly insufficient hypotheses, complex logic you can't follow. For each: location, what made you uneasy, what Jörn should look at.

### 4. SUMMARY
Total issues by category, overall readiness impression, which sections are most polished vs rough.

## Review methodology

1. **Surface**: language, grammar, typos, formatting, LaTeX issues
2. **Clarity**: can a knowledgeable reader follow? Definitions before use? Transitions smooth?
3. **Math content**: precise statements? Proper quantifiers? Steps follow logically? "Clearly" actually clear?
4. **Consistency**: definitions match later usage? Conventions maintained?

## Domain-specific checks

- **Local vs global**: "Reeb orbit" = *closed* trajectory. Transitions are trajectory segments, not orbits. Grep for `\borbit\b` in transition/KKT/pruning contexts.
- **Quantifiers**: "for all" vs "there exists", order of quantifiers, universal vs existential.
- **Proof-critical language**: "without loss of generality," "clearly," "it follows that," "by assumption" — verify these are justified.

## Guidelines

- Be specific: "line 4 of the proof of Theorem 3.5, 'continous' → 'continuous'" not "there might be a typo in Section 3"
- Err on flagging: false positive costs 10 seconds to dismiss, false negative costs Jörn minutes
- Don't rewrite — suggest concise fixes
- Convention checks (formatting, figures, tables) are covered by `.claude/rules/tex-style.md` which auto-loads — focus on content quality
