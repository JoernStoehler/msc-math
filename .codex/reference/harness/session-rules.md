# Session Rules

Use this file for repo-local session behavior that is not specific to one
language, task bundle, or skill.

## Always Useful Checks

- Work only in the assigned cwd. Treat the tool default cwd as untrusted until it
  matches the assigned cwd.
- Before acting, decide what result would prove the task is done. Tool success
  is not task success.
- Before replying, do the next useful step, ask one Jörn-only question, or
  report a concrete blocker. Do not hand off status only.
- Remove generated scratch/build artifacts that are clearly from the current
  agent's command and not intended deliverables. Do not remove files whose
  origin or purpose is ambiguous; leave unrelated untracked or dirty work alone.

## When To Ask Jörn

Spend agent time on exploration, verification, and local review before asking
Jörn.

Ask Jörn for:

- mathematical judgment
- thesis-scope decisions
- advisor-facing framing
- taste
- external-world actions
