---
paths:
  - "experiments/**"
---

# Experiment Philosophy

## Always investigative

Experiments are always investigative — even mature ones with thesis-ready writeups remain open to revisiting, expansion, and updating.

Progression is fluid, with no clear cutoff points:
- From [nothing] → [idea] → [plan with preliminary findings] → [active hypotheses with mysteries] → [findings from non-runnable code] → [failed attempt summary] → [cleanup commit in git log]
- From [nothing] → [full bundle: scripts + thesis section + datasets + extra rust code]

When cleaning up code that's no longer useful:
- If learnings worth preserving: create `experiments/<topic>.md` with git ref to last commit
- Otherwise: just delete (it's in git history)

## Quality Standards

**Rerunnable from zero:**
- Starting from empty experiment directories, running all scripts should reproduce all outputs
- No manual steps, no "run this once, then comment it out"

**Document assumptions:**
- If script assumes file exists, document it in header and error message

**Not production code:** No exhaustive testing required, but must be reproducible. Focus on clarity and correctness over performance.

## Library Stability Boundary

Only stable, proven code goes into `crates/` library. New algorithm variants are self-contained in experiment binaries. Copy library internals into the binary where needed. If a variant is later promoted to production, it enters the library then.
