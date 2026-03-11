---
paths:
  - "**/*.tex"
---

# LaTeX Style Conventions

## Build

```bash
cd thesis/ && latexmk && ./check-build.sh
```

`check-build.sh` parses the build log for overfull hboxes (> 1pt) and undefined references. It exits non-zero if any are found. **Agents must run this after every compilation** and fix any new warnings they introduced.

## Jörn Reviews PDF, Not .tex

Jörn reads the compiled PDF. He does not read `.tex` source files for review.

**When presenting content for Jörn's review:**
1. Compile the thesis (`cd thesis/ && latexmk`)
2. Look up the rendered number from `thesis/build/main.aux`
3. Tell Jörn: "Lemma 3.43 on page 25" — not "see rank-deficiency-dismissal.tex"

**When reporting edits:**
- Describe by rendered location: "the proof conclusion of Theorem 5.1"
- Not by source location: "line 418 of simple-minimizer-proof.tex"

**When referring to theorems/sections/equations in chat:**
- Use rendered numbers: "Theorem 5.3", "Section 2.1", "equation (3.7)"
- Not label names: `thm:simple-minimizer`, `sec:algorithm`
- How to get rendered numbers:
  ```bash
  grep 'label-name' thesis/build/main.aux
  ```

Note: In `.tex` source, always use `\ref{label}` — never hardcode numbers.

## Theorem/Section Numbers

Never guess — read from `thesis/build/main.aux` after building:
```bash
grep -E 'newlabel\{(sec:|thm:|lem:|def:|rem:|cor:)' thesis/build/main.aux
```

## Default Status

All content is **agent-written and unreviewed** unless explicitly marked otherwise. When a `.tex` file has no review markers, assume nothing has been verified by Jörn.

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

### Human readers (plain `%`)
Regular LaTeX comments.

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

## Experiment-Specific

- Experiment writeups live in `experiments/<name>/<name>.tex`, wired into the thesis via `\input`
- Builds upon Thesis Writing conventions, except those specific to mathematical proofs (Proof Writing, Four Audiences' "imaginary master student" criterion)
