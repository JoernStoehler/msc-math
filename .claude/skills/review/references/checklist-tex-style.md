# Review Checklist: LaTeX Style (Phase 1)

Detection rules and verification procedures for LaTeX formatting conventions.
The review subagent works through these items sequentially per the review skill methodology.

## 1. Build

```bash
cd thesis/ && latexmk && ./check-build.sh
```
Report overfull hboxes (> 1pt) and undefined references.

## 2. File Headers

Every `.tex` file starts with a `%` header block containing:
1. **Identity**: `% filename.tex — \input'd from parent.tex`
2. **Sources**: where the content comes from
3. **Structure**: outline of sections/subsections

No review status in the header (that goes in inline `% Jörn:` markers).

## 3. Environments

- Definitions, lemmas, theorems, propositions use `\definition`, `\lemma`, `\theorem`, `\proposition` environments.
- Context/intuition uses `\remark` and `\example` environments.
- **No prose paragraphs outside environments** (except minimal connective text).
- All calculations displayed as formulas, never described in English sentences.

## 4. Comment Conventions

- `% Jörn:` for review status — **staleness check**: if content within scope was edited after the marker's commit hash, marker must be deleted.
- `% QC:` for verification instructions.
- `% Downstream:` for Rust implementation notes.
- `% [TODO: JÖRN -` for content needing Jörn's attention.
- `% [GAP -` for known mathematical gaps.

## 5. Figure and Table Inclusion

**Figures:**
- Detection: grep for `\includegraphics\[` — any `width=`, `height=`, or `scale=` parameter is a violation (all formatting is in Python).
- `\includegraphics{file.png}` with no options is correct.

**Tables:**
- No `\scriptsize` or `\tiny` inside or near `\begin{table}`. Body text must not go below `\small`.
- Use `booktabs` (`\toprule`/`\midrule`/`\bottomrule`), not `\hline`.
- Column headers must have units or be self-explanatory.
- Numbers: consistent decimal places within each column.

## 6. Labels and Cross-References

- All `\ref{}` labels must be defined (check `thesis/build/main.aux`).
- No hardcoded theorem/section numbers in `.tex` source (always `\ref{label}`).
- Notation must match `correspondence.tex` exactly.

## 7. Proof Structure Template

Recommended structure: Assumptions → Claim → Overview → Steps → Conclusion.
Check each proof has an overview paragraph.
(Mathematical soundness is a phase 2 concern — this checks only the structural template.)

## 8. Mechanical Anti-Patterns

**AP4: Overwrought language**
Detection: Flag adjective clusters (2+ adjectives before a noun) and dramatic words (irrevocable, catastrophic, critical) unless they carry technical meaning.

**AP5: Rust/CS notation in mathematical text**
Detection: Flag any `\texttt{...}` inside definition/lemma/theorem/remark environments. Programming terms belong in implementation sections, not mathematical statements.

**AP7: Setup text outside the environment it belongs to**
Detection: For each lemma/theorem environment, check if it references notation defined only in the preceding paragraph. If so, flag — fold the setup into the environment.

## 9. Citation Format

- Author names verified against `thesis/bibliography.bib` — grep each cited author name and confirm it matches the bib entry.
- No author names from memory (common failure: "Cieliebak-Hutchings" instead of "Chaidez-Hutchings").
