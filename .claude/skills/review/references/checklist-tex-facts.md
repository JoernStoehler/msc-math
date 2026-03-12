# Review Checklist: LaTeX Factual Accuracy (Phase 2)

Verification procedures for factual claims in thesis `.tex` files.

## What Counts as a Factual Claim

- **Numbers**: "27 entries", "5-16 facets", "E < 10^{-13}" → check against data files
- **Code behavior**: "the algorithm panics", "the assertion has never fired" → check actual code
- **Dataset properties**: "covering all test polytopes" → check fixture/JSONL files
- **Citations**: author names, paper titles, theorem numbers → check `thesis/bibliography.bib`
- **Cross-references**: "Lemma B.5 states X" → check the actual lemma text
- **Literature claims**: "Haim-Kislev proved X" → check the paper in `papers/`

## How to Verify

1. **Numbers against data:** Read `crates/tests/fixtures/capacity_dataset.json`, `experiments/**/*.jsonl`, etc.
2. **Code claims:** Grep `crates/src/` for the referenced function/assertion/behavior.
3. **Citations:** Read `thesis/bibliography.bib` for author names and titles. **Never trust your memory for author names** — common failure: "Cieliebak-Hutchings" instead of "Chaidez-Hutchings".
4. **Cross-references:** Build thesis (`cd thesis/ && latexmk`), read `thesis/build/main.aux` for rendered numbers.
5. **Literature claims:** Read the actual paper in `papers/<key>/` if available.

## Red Flags

- Claims without data source (where did this number come from?)
- Claims that are close but not exact (rounding errors, stale data)
- Claims about "all" or "none" that might have exceptions
- Missing `% [TODO: JÖRN -` or `% [GAP -` markers on unverifiable claims
