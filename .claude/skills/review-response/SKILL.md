---
name: review-response
description: Use when processing a code review report. Ensures every finding is investigated and accounted for — none dismissed without explanation.
---

# Process a code review

A review report follows. Process EVERY finding systematically.

## Rules

1. **Parse every finding.** Extract each distinct finding from the review — don't skip cosmetic ones or ones labeled "minor."
2. **Investigate before responding.** For each finding, read the relevant code, run the relevant command, or check the relevant state. Do not respond to a finding based on memory or assumption.
3. **Never dismiss without explanation.** If you believe a finding is wrong, you must explain why the reviewer arrived at that conclusion and what they saw that led them there. "The reviewer was mistaken" is not an acceptable response without this explanation.
4. **Check the reviewer's base.** Did the reviewer state what commit/branch they compared against? If not, flag this — it affects the reliability of all findings. If the reviewer compared against `origin/main` instead of local `main`, their findings about "extra files" or "conflicts" may reflect stale commits, not real problems.
5. **Output an accountability table.** For every finding:

| # | Finding | Status | Action taken |
|---|---------|--------|-------------|
| 1 | ... | Fixed / Already done / Investigated: not applicable because ... | ... |

6. **Flag anything you cannot verify.** If a finding requires information you don't have (e.g., the reviewer saw something in a diff you can't reproduce), say so and investigate the discrepancy — don't silently skip it.

## Review to process

$ARGUMENTS
