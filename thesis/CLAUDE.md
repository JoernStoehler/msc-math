# Thesis Writing Conventions

This directory contains the LaTeX source for Jörn's master thesis. All agents working in `thesis/` must follow these conventions.

## Build

```bash
cd thesis/ && latexmk
```

Available: TeX Live 2023, pdflatex, xelatex, lualatex, latexmk, biber, chktex.

## Four Audiences

Every line of LaTeX must work for all four audiences simultaneously:

1. **Human readers** (Jörn, Kai, Elizabeth)
   - Want the main result upfront
   - Will skim definitions, revisit if confused
   - All proofs are skippable
   - Value: algorithm, proof ideas, geometric intuition

2. **Imaginary master student** (nominal target)
   - Typical math master background: linear algebra, analysis, basic topology, intro symplectic geometry, intro optimization
   - Every definition stated in full, not deferred to literature
   - Must follow the chapter linearly without external references

3. **QC agents** (verification)
   - Verify one chunk at a time, trusting previously verified chunks
   - For every proof step: must immediately confirm "yes, that follows directly"
   - Words must have clear, specific meanings
   - Never state anything incorrect, even if non-fatal

4. **Downstream agents** (Rust implementers, test writers)
   - Need full detail in all definitions, lemmas, proofs
   - Need ALL properties listed (including unused ones) for generating tests
   - Need concrete values and example calculations

## Default Status

All content is **agent-written and unreviewed** unless explicitly marked otherwise. When a `.tex` file has no review markers, assume nothing has been verified by Jörn.

## Comment Conventions

Use prefixed comments to separate meta information by audience:

### Jörn's review status (`% Jörn:`)

Three levels, strictly ordered: **text > math > structure**. Only record the highest approved level — higher implies all lower levels.

1. **Structure**: proof approach/strategy is correct, section organization is right
2. **Math**: mathematical content is correct (but writing may need polish)
3. **Text**: the written prose is correct (final quality)

```latex
% Jörn: structure approved (abc1234) — from \subsection{Sampling procedure} to \end{proof}
% Jörn: text approved (abc1234) — from \subsection{Sampling procedure} to "Acceptance rate sweep"
```

The commit hash is from `git rev-parse HEAD` after committing the approved version — it's the commit the agent is already making.

Only one marker per scope. When a higher level is approved, replace the lower marker (e.g., `structure` → `text`). Scope must be explicit (section names or line ranges). Content outside any `% Jörn:` marker is unreviewed.

**Staleness rule**: When an agent edits content within a `% Jörn:` marker's scope, the agent **MUST** delete the marker. The edited content reverts to the default status (agent-written, unreviewed). The commit hash serves as a backup: if a marker survived an edit, anyone can diff the file since that commit to detect staleness.

Jörn reviews the **PDF-visible text** (rendered output), not the `%` comments. The `% Jörn:` markers record what he approved in the PDF; they do not mean he reviewed the LaTeX source comments.

### QC agent findings (`% QC:`)
```latex
% QC: polytope per Definition~\ref{def:polytope}, facet data per Definition~\ref{def:facets}
```
→ Instructions for QC agent on what to verify, or resolved QC findings that only a pedantic verifier would want spelled out. If a QC finding matters to human readers, expand it in the text instead.

### Developer agents (`% Downstream:`)
```latex
% Downstream: R_i = (2.0 / h_i) * J_0 * n_i
% Downstream: Test: |R_i| = 2/h_i for all i
```
→ How to implement in Rust, what tests to write

### Writing agents (`% [TODO: JÖRN -`)
```latex
% [TODO: JÖRN - verify this E-L derivation. Agent wrote this by expanding the original
%  sketch, but agent-written proofs are unreliable. Check for errors in the calculation.]
```
→ Marks content needing Jörn's attention

### Gap tracking (`% [GAP -`)
```latex
% [GAP - AGENT CONFIDENCE 70%: The derivation above shows X, but the equation below
%  claims Y. Agent verified lines A-B are correct, but cannot connect them to lines C-D.
%  JÖRN: verify if gap is real, fix if so, or explain the connection if agent missed it.]
```
→ Known mathematical gaps with epistemic confidence

### Human readers (plain `%`)
```latex
% Use J_0^2 = -I here
```
→ Regular LaTeX comments for humans reading the source

## File Headers

Every `.tex` file starts with a `%` header block containing:

1. **Identity**: `% filename.tex — \input'd from parent.tex`
2. **Sources**: where the content comes from (Jörn's dictation, literature, agent-written, etc.)
3. **Structure**: outline of sections/subsections

Do NOT put review status in the header. Review status lives inline via `% Jörn:` markers (see above).

## Content Rules

1. **Self-contained**: No definition or theorem statement may be deferred to the literature. Every definition is stated in full. Every theorem is stated in full.

2. **Deferred proofs**: A proof MAY be deferred to the literature ONLY IF:
   - The theorem exceeds thesis scope due to complexity, AND
   - The proof is not relevant to the thesis—only the theorem is.

3. **Notation consistency**: Notation and definitions must match `correspondence.tex` exactly. If correspondence.tex uses symbol X, this file uses symbol X. No synonyms, no alternative forms, no "equivalent" restatements unless a lemma proving equivalence is included.

4. **Writing rule**: Proofs cannot cite external sources mid-proof. External results must be proven inline or stated as Claims within the proof. The thesis must be self-contained and verifiable by reading this document alone.

## Proof Writing

**Structure**: Assumptions → Claim → Overview → Steps → Conclusion

**Level of detail**:
- Detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave — if a step is non-trivial, say so explicitly

**Agent limitations**:
- Agents cannot reliably verify mathematical proofs
- Agent-written proofs are drafts until Jörn reviews them
- Never claim Jörn "approved" content unless he explicitly verified the math

**What agents CAN do**:
- Turn natural language descriptions into proofs
- Improve proof writing
- Fix errors in proofs
- Detect spots in proofs (but not with high reliability)
- Report unclear or suspicious proof steps

**What agents CANNOT do**:
- Provide final high-reliability verification (that must come from Jörn)

## Emphasis and Structure

- **Emphasis proportional to importance**: Don't dedicate a full section to a trivial identity
- **Geometric definitions first, formulas derived**: E.g., action = integral of Liouville form, not the coordinate expression
- **Standard definitions**: Use them exactly as in the literature. If you use a different form, state a lemma proving equivalence

## Format Rules

- Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof` environments
- Use `\remark` and `\example` for context/intuition/illustrations
- No prose paragraphs outside environments (except minimal connective text)
- All calculations displayed as formulas, never described in English sentences

## Workflow: dictation.md → LaTeX

See `thesis/dictation.md` for the workflow:
- Jörn writes natural language content, marks `[ready]`
- Agents read `[ready]` items, translate to LaTeX, run QC, update thesis
- Agents mark items `[question]` if something is unclear
- Jörn answers questions, re-marks `[ready]`

Statuses: `[draft]` (Jörn working) | `[ready]` (translate now) | `[question]` (agent needs input) | `[done]` (in thesis, QC passed)
