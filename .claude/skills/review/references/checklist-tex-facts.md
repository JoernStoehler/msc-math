# Review Checklist: LaTeX Factual Accuracy (Phase 2)

Verification procedures for factual claims in thesis `.tex` files.

**Efficiency rule:** After reading the reviewed file(s) in Step 1, identify all factual claims from the content already in context. Only use grep/read for **cross-file verification** (data files, bib, aux, code). Do not re-read or re-grep the reviewed files.

## What Counts as a Factual Claim

- **Numbers**: "27 entries", "5-16 facets", "E < 10^{-13}" → check against data files
- **Code behavior**: "the algorithm panics", "the assertion has never fired" → check actual code
- **Dataset properties**: "covering all test polytopes" → check fixture/JSONL files
- **Citations**: author names, paper titles, theorem numbers → check `thesis/bibliography.bib`
- **Cross-references**: "Lemma B.5 states X" → check the actual lemma text
- **Literature claims**: "Haim-Kislev proved X" → check the paper in `papers/`

## How to Verify

Identify all claims from the file content in context, then batch your cross-file lookups:

1. **Numbers against data:** Read relevant data files once (JSONL, fixtures), check all numeric claims against them.
2. **Code claims:** Grep `crates/src/` for referenced functions/assertions (cross-file).
3. **Citations:** Read `thesis/bibliography.bib` once, check all citation keys and author names. **Never trust your memory for author names** — common failure: "Cieliebak-Hutchings" instead of "Chaidez-Hutchings".
4. **Cross-references:** Read `thesis/build/main.aux` once, check all `\ref{}` labels resolve.
5. **Literature claims:** Read papers in `papers/<key>/` only if specific claims need verification.

## Red Flags

- Claims without data source (where did this number come from?)
- Claims that are close but not exact (rounding errors, stale data)
- Claims about "all" or "none" that might have exceptions
- Missing `% [TODO: JÖRN -` or `% [GAP -` markers on unverifiable claims
