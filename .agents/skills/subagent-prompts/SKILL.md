---
name: subagent-prompts
description: Use when Codex writes, reviews, or delegates subagent prompts, worker packets, reviewer packets, fresh-agent checks, or bounded first-pass/verification tasks in this repo.
---

# Subagent Prompts

## Using Subagents

- subagents are for bounded first-pass labor, bounded verification, and
  independent checks; the top-level session owns integration and final claims
- delegate output is untrusted evidence until checked
- every subagent prompt needs a required cwd, scope, ownership, success check,
  output format, reserved decisions, and stop condition
- don't prematurely prescribe the approach, focus on the outcome and how to measure success
- use `gpt-5.3-codex-spark` for super-fast low-intelligence tasks such as text
  refactoring without a need for scientific understanding or reasoning
