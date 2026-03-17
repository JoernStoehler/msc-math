---
name: tex-format
description: LaTeX formatting conventions for thesis .tex files in thesis/. Load when writing or editing thesis .tex files. Covers comment conventions (Jörn review markers, QC, TODO, GAP), file headers, theorem environments, figure/table inclusion. For math.tex files in crates/ or experiments/, load `math-tex` instead.
---

# LaTeX Format Conventions

## Comment Conventions

Use prefixed comments to separate meta information by audience:

### Jörn's review status (`% Jörn:`)

Three levels, strictly ordered: **text > math > structure**. Only record the highest approved level.

1. **Structure**: proof approach/strategy is correct, section organization is right
2. **Math**: mathematical content is correct (but writing may need polish)
3. **Text**: the written prose is correct (final quality)

```latex
% Jörn: structure approved (abc1234) — from \subsection{Sampling procedure} to \end{proof}
% Jörn: text approved (abc1234) — from \subsection{Sampling procedure} to "Acceptance rate sweep"
```

Only one marker per scope. When a higher level is approved, replace the lower marker. Scope must be explicit. Content outside any `% Jörn:` marker is unreviewed.

**Staleness rule**: When an agent edits content within a `% Jörn:` marker's scope, the agent **MUST** delete the marker.

### QC agent findings (`% QC:`)
```latex
% QC: polytope per Definition~\ref{def:polytope}, facet data per Definition~\ref{def:facets}
```

### Developer agents (`% Downstream:`)
```latex
% Downstream: R_i = (2.0 / h_i) * J_0 * n_i
```

### Writing agents (`% [TODO: JÖRN -`)
```latex
% [TODO: JÖRN - verify this E-L derivation.]
```

### Gap tracking (`% [GAP -`)
```latex
% [GAP - AGENT CONFIDENCE 70%: ...]
```

## File Headers

Every `.tex` file starts with a `%` header block containing:
1. **Identity**: `% filename.tex — \input'd from parent.tex`
2. **Sources**: where the content comes from
3. **Structure**: outline of sections/subsections

Do NOT put review status in the header.

## Format Rules

- Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof` environments
- Use `\remark` and `\example` for context/intuition/illustrations
- No prose paragraphs outside environments, except minimal connective text between environments
- Calculations displayed as formulas, not described in English prose

## Figures and Tables

All figure formatting is handled in Python. LaTeX is a 1:1 pass-through (`\includegraphics{file.png}`, no `width=`/`scale=`).

**Tables (LaTeX):**
- Use `booktabs` (`\toprule`/`\midrule`/`\bottomrule`), not `\hline`.
- Table body text must not go below `\small`; no `\scriptsize` or `\tiny`.
- Column headers must have units or be self-explanatory. Numbers: consistent decimal places within each column.

**Captions:**
- Captions state observations and comparisons (relating to an explicit reference).
- Interpretations and speculation belong in body text, NOT in captions.

## Scope

This skill covers thesis .tex files in `thesis/`. For math.tex files in `crates/` or `experiments/`, load the `math-tex` skill. The comment conventions (`% Jörn:`, `% QC:`, `% [TODO:`, `% [GAP:`) apply to both.
