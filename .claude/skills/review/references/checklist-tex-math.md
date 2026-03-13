# Review Checklist: LaTeX Mathematical Correctness (Phase 2)

Verification procedures for proofs, definitions, and theorem statements.
Run on clean files (after phase 1 fixes).

**Efficiency rule:** After reading the reviewed file(s) in Step 1, check all mathematical content from the content already in context. Only use grep/read for **cross-file verification** (checking what a referenced lemma actually says, verifying numbers against data files). Do not re-read or re-grep the reviewed files.

**Important limitation:** Agents cannot reliably verify proof correctness — they overlook gaps, errors, and subtle logical issues. The job is to catch what you can and flag everything for Jörn's expert review. Be honest about confidence levels.

## Confidence Levels

Be explicit:
- **High confidence**: "This is wrong because [specific reason]"
- **Moderate confidence**: "This step seems to skip [specific gap], but I may be missing something"
- **Low confidence / needs Jörn**: "I cannot verify this step — it may be correct but I cannot confirm it"

## For Each Definition

- Is it self-contained (no deferred definitions)?
- Does notation match `correspondence.tex`?
- Are all symbols either standard, defined within, or cross-referenced?
- Is the definition correct as stated (not vacuously satisfied, not too restrictive)?

## For Each Theorem/Lemma Statement

- Are hypotheses complete and precise?
- Is the conclusion correctly stated?
- Does it match what the proof actually proves?

## For Each Proof

- **Structure**: Assumptions → Claim → Overview → Steps → Conclusion?
- **Overview**: Is there a paragraph explaining the proof strategy?
- **Each step**: Does it follow from previous steps? Which theorem/lemma is used? Are hypotheses satisfied?
- **Non-obvious steps**: Are they annotated with the specific result used?
- **Gaps**: Any "clearly", "obviously", "it follows" that isn't actually obvious?
- **External citations**: Are external results stated as Claims within the proof (not cited mid-proof)?
- **Quantifiers**: Are forall/exists used correctly? Any missing quantifiers?

## Cross-Reference Verification

- Do `\ref{}` labels point to the right theorem/definition?
- When a proof cites "by Lemma X", does Lemma X actually say what's claimed?
