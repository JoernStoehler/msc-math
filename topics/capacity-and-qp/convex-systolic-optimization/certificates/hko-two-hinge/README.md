# Two characteristic-invariant hinges: exact supporting checks

This companion checks the four literal shortest-word families, quotient-density constants, and scalar inequalities used in Appendix C. The printed proposition and appendix contain the analytic implication. The checker does not establish local maximality for arbitrary twelve-facet bodies, higher-order behavior in zero first-variation directions, or anything about non-invariant cuts.

The script requires Python 3.11 or later with only the standard library. It reuses the sibling `../hko-ascent/exact_core.py` and `../hko-ascent/expected/{primitives,catalogue}.json` from this same paper companion. Those inputs provide the exact number field and the complete base-word chart catalogue; no original research scratch directory is needed.

From this directory, choose a fresh output path and run:

```sh
python certify_amplification.py --source ../hko-ascent --out /tmp/amplification-fresh.json
cmp /tmp/amplification-fresh.json expected/amplification-PASS-portable.json
```

The checker refuses to replace an existing output file, writes only that file, and suppresses Python bytecode. A fresh copy of the paper packet produced an exact byte match. `proof-original.tex` preserves the auditor-approved module before the article split it into a short main proposition and complete Appendix C. `amplification-proof.md` is the author's analytic working note; the reader-facing proof is printed in the PDF. The scoped independent audit is retained in `../../reviews/hko-two-hinge-mathematics.md`.

`MANIFEST.sha256` pins these packet files. The hko-ascent sibling has its own manifest and replay instructions.
