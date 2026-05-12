---
name: goal-tool
description: Use before creating, updating, checkpointing, or completing `/goal`.
---

# /goal Tool

## Bad

Do not replace the real objective with a rewritten `/goal` objective. Do not
introduce drift by rewriting/rephrasing the objective. This caused one thesis
session to end prematurely on an incomplete milestone that optimized too
strongly for some quality aspects and traded off others that were more
important to real thesis success.

## Good

Put the objective in a charter, and make `/goal` point to that charter:

```text
Execute the objective charter at <path>. Mark complete only under the
charter's stopping conditions.
```

The charter preserves the real objective, stopping conditions, and which support
tasks are not completion criteria. Do not simplify those. Do not break down
objectives prematurely more than you'd do anyway.

The only purpose of `/goal` is to later recall the objective without any loss.
