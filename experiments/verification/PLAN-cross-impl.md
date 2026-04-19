# Cross-impl Validation Plan

Scope: split and handoff between `all-minimum` and `orbit-recovery`.

1. Keep `all-minimum/` responsible for:
   - local-first dataset selection
   - minimum-action sigma set extraction
   - writing `all-minimum*.jsonl`
2. Keep `orbit-recovery/` responsible for:
   - rebuilding one-sigma KKT data from trusted rows
   - geometric orbit reconstruction checks (closure, facet/on-`K` compliance, action)
3. Keep `correctness/` as the global property gate for broad capacity claims.
4. Keep `algorithm-comparison/` to justify algorithm choices for any changed solver stack.

Boundary rule:
- do not pass algorithm design details through logs; pass schema, assumptions, and trust boundaries only through packets and their `jsonl` outputs.
