---
name: tex-format
description: How thesis .tex files should look — comment markers, file headers, theorem environments, figure/table inclusion. Load when formatting or structuring thesis .tex files in thesis/. Does NOT cover what the content should say (see tex-content) or build commands (see tex-build). For math.tex in crates/ or experiments/, load `math-tex` instead.
---

# LaTeX Format Conventions

## Comment Conventions

Use prefixed comments to separate meta information by audience:

### Jörn's review status (`% Jörn:`)

Three levels, strictly ordered: **text > math > structure**. Each level is a refinement — "math approved" implies structure is also correct; "text approved" implies both math and structure are correct. Only record the highest approved level.

1. **Structure**: proof approach/strategy is correct, section organization is right
2. **Math**: mathematical content is correct (but writing may need polish)
3. **Text**: the written prose is correct (final quality)

```latex
% Jörn: structure approved (abc1234) — from \subsection{Sampling procedure} to \end{proof}
% Jörn: text approved (abc1234) — from \subsection{Sampling procedure} to "Acceptance rate sweep"
```

Only one marker per scope. When a higher level is approved, replace the lower marker. Scope must be explicit. Content outside any `% Jörn:` marker is unreviewed.

**Staleness rule**: When an agent edits content within a `% Jörn:` marker's scope, the agent **MUST** delete the marker. Detection: if content within the marker's scope was edited after the marker's commit hash, the marker is stale.

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

This gives agents immediate context when reading any file. Do NOT put review status in the header — it grows stale as content changes; use `% Jörn:` markers in the body instead.

## Format Rules

- Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof` environments
- Use `\remark` and `\example` for context/intuition/illustrations
- No prose paragraphs outside environments, except minimal connective text between environments
- Calculations displayed as formulas, not described in English prose

## Labels and Cross-References

- All `\ref{}` labels must be defined (check `thesis/build/main.aux`).
- No hardcoded theorem/section numbers in `.tex` source — always use `\ref{label}`.
- Notation must match `appendix-notation.tex` exactly.

## Anti-Patterns

**AP4: Overwrought language.** Flag adjective clusters (2+ adjectives before a noun) and dramatic words (irrevocable, catastrophic, critical) unless they carry technical meaning. Agents tend to produce confident-sounding filler that obscures gaps in reasoning — flagging overwrought language surfaces these.

**AP5: Rust/CS notation in mathematical text.** Flag any `\texttt{...}` inside definition/lemma/theorem/remark environments. Programming terms belong in implementation sections, not mathematical statements.

**AP7: Setup text outside the environment it belongs to.** For each lemma/theorem environment, check if it references notation defined only in the preceding paragraph. If so, flag — fold the setup into the environment. Reason: readers reach lemmas via `\ref` and need them to be self-contained.

## Figures and Tables

All figure formatting is handled in Python. LaTeX is a 1:1 pass-through (`\includegraphics{file.png}`, no `width=`/`scale=`). Reason: easier to reason about figure appearance where it's created; LaTeX should be boring to review.

**Tables (LaTeX):**
- Use `booktabs` (`\toprule`/`\midrule`/`\bottomrule`), not `\hline`.
- Table body text must not go below `\small`; no `\scriptsize` or `\tiny`.
- Column headers must have units or be self-explanatory. Numbers: consistent decimal places within each column.

**Captions:**
- Captions state observations and comparisons (relating to an explicit reference).
- Interpretations and speculation belong in body text, NOT in captions.

## Scope

This skill covers thesis .tex files in `thesis/`. For math.tex files in `crates/` or `experiments/`, load the `math-tex` skill. The comment conventions (`% Jörn:`, `% QC:`, `% [TODO:`, `% [GAP:`) apply to both.
