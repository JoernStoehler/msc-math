---
name: handoff
description: Write a handoff file to transfer task ownership to a future session. Load when work is incomplete and context would be lost without a written handoff.
---

# Handoff Workflow

Write a handoff file when work is incomplete and a future agent needs context beyond what TASKS.md contains.

## Steps

1. **Update TASKS.md** with current status, next steps, and any newly discovered tasks
2. **Write the handoff file** at `handoffs/<name>.md` using the template at `references/template.md`
3. **Verify claims** — re-read key files referenced in the handoff. Check that paths exist, stated facts match current code/data, and branch state is accurate. Handoffs with stale claims waste the next agent's time.
4. **Commit** the handoff file and TASKS.md update
