# Session: Paranoia Check — Conjectures + Interpretations

Flag-only audit. Produce a ranked list of claims that would be most embarrassing if wrong. Do NOT fix anything — Jörn reviews the list.

## Scope

All files that make claims about what our results mean:
- `thesis/handwritten-notes.md` — the thesis narrative with conjectures
- `TASKS.md` — status summaries that include interpretive claims
- All `crates/exp-*/*/logbook.md` — experiment interpretations
- All `crates/dev-*/*/logbook.md` — development interpretations
- `crates/**/math.tex` — mathematical claims (lemmas, propositions)

## What to flag

### A. Conjectures presented as stronger than evidence supports
- "HKO2024 is a local maximum" — we conjecture this. Is it always hedged as a conjecture, never stated as proven?
- "the only sys>1 polytope" — what's the evidence strength? Is it always qualified?
- Any "we show" or "we prove" for things we only have numerical evidence for

### B. Causal claims without mechanism
- "X because Y" where Y is an interpretation, not a proven mechanism
- "suggests", "indicates", "due to" — are these backed by data or just pattern-matching?

### C. Logical gaps in arguments
- "all 15 flat directions have negative curvature, therefore local max" — is the sufficiency argument actually made? Non-smooth second-order conditions aren't trivial.
- "536 cuts all decrease sys" — does this actually prove anything about the F=11 neighborhood, or just 536 specific directions?
- "gradient ascent converges to sys<1 local optima" — from how many starts? Is "usually" quantified?

### D. Overconfident quantitative claims
- "~10^-31 volume fraction" — how sensitive is this to methodology? Would a different sampling strategy give a very different number?
- "characteristic radius ~0.035" — is this robust?

### E. Missing hedges on heuristic-as-conjecture
- Conjectures that were used as working assumptions (e.g., in experiment design) — are they clearly labeled as conjectures in the writeup?
- Any "we assume X" where X is actually unproven

### F. Claims that depend on numerical stability
- Any conclusion that would change if the capacity computation had errors >1e-6
- Claims near the sys=1 boundary where numerical precision matters

## Output

Produce a report file at `crates/paranoia-conjectures.md`:
- Ranked list, most-embarrassing-if-wrong first
- For each flag: quote the claim, cite the source file:line, state what's missing or overconfident
- Severity: [critical] claims that could invalidate a thesis section, [moderate] hedging needed, [minor] style/precision
- Do NOT suggest fixes — just flag clearly so Jörn can review

## Conventions
- Read `.claude/rules/*.md` for project conventions
- Read `CLAUDE.md` for general guidelines
- This is a READ-ONLY session. No edits to any files except producing the report.
- Work in a branch, not on main. Don't merge.
