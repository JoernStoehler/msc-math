# Algorithm Contracts

This directory owns the canonical cross-surface contract for nontrivial
algorithms.

Each contract records:
- the algorithm correspondence contract
- the algorithm verification contract

Use one file per important algorithm id. Keep local code comments short and
point them here instead of duplicating the whole correctness story.

Related local notes such as `REASONING.md`, `DECISIONS.md`, or `NEXT-STEPS.md`
stay near the code or experiment they describe. They are local reasoning or
operational notes, not canonical cross-surface contracts.
