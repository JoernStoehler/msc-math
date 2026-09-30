# Jörn's browser views

The selected workflow work currently has a private Tailscale view at
<http://joern-pc.tailc5e761.ts.net:8766/> (observed 2026-09-30). It serves this
worktree, `/home/joern/.codex/worktrees/60b9/msc-math`, through the transient user
service `msc-math-workflow-dashboard.service`. The existing port-8765 service
serves `/workspaces/msc-math`; it is a different checkout and does not display
this worktree's edits. A worktree update is not a browser publication until the
served state has been checked against the edited checkout.


Run `python3 scripts/serve-project-dashboard.py` from the repository root and
open <http://127.0.0.1:8765/> in Chrome beside the Codex terminal.

- [index.html](index.html) shows selected work, owners, dependencies, decisions,
  resource observations and recent changes from `docs/coordination/current.json`.
  It refreshes every 15 seconds and clearly labels older observations.
- [thesis.html](thesis.html) is the retained scientific brief. Its current
  scientific baseline is 25 September, with reconciliation still open. The
  server compares cited sources against [thesis-sources.json](thesis-sources.json);
  hashes do not establish that the brief is complete or correct.

Keep browser presentation here and assignment truth in
[../coordination/](../coordination/README.md). Neither page is a task authority
or an autonomous process monitor. The server reads only explicitly allowed
project files and serves no write endpoint. Old dashboard URLs redirect here.
