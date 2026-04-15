# Claim Review Checklist

Use for log-like writeups, thesis text, formal commentary, result summaries, captions, and review reports that contain factual claims.

For each claim, verify against the named source:
- Numbers: read cited JSONL, CSV, table, command output, or benchmark log.
- Counts: count from the data or code.
- Extremes: recompute or inspect the script that computed them.
- Code behavior: grep and read the implementation.
- Bibliography claims: check `thesis/bibliography.bib`, then inspect `papers/` when needed.
- Cross-references: check the relevant `.aux` file.
- Figure descriptions: inspect the rendered image and the script that generated it.

Results:
- `VERIFIED`: source supports the claim.
- `WRONG`: source contradicts the claim; state expected and found values.
- `UNVERIFIABLE`: source is missing, unavailable, or too ambiguous.
- `NO SOURCE CITED`: claim has no checkable source.

Do not check mathematical proof correctness here; use `references/formal-math.md`.
