# Jörn's browser views

The 30 September observation recorded a private Tailscale view at
<http://joern-pc.tailc5e761.ts.net:8766/>, serving worktree
`/home/joern/.codex/worktrees/60b9/msc-math` through the transient user service
`msc-math-workflow-dashboard.service`. Port 8765 then served the separate
`/workspaces/msc-math` checkout. Those are dated observations, not verified
current listeners or routes to this `f171` worktree. A worktree update is not
a browser publication until the served state has been checked against the
edited checkout.


Run `python3 scripts/serve-project-dashboard.py` from the repository root and
open <http://127.0.0.1:8765/> in Chrome beside the Codex terminal.

- [index.html](index.html) shows selected work, owners, dependencies, decisions,
  resource observations and recent changes from `docs/coordination/current.json`.
  It refreshes every 15 seconds and clearly labels older observations.
- [thesis.html](thesis.html) is the scientific brief reconciled during the
  4 October restart against required scope, promoted claims, evidence owners
  and the retained candidate. It preserves bounded review and remaining
  acceptance gaps. The server compares selected sources against
  [thesis-sources.json](thesis-sources.json); changed hashes invalidate the
  recorded reconciliation, while matching hashes do not establish scientific
  correctness or human PASS.

Keep browser presentation here and assignment truth in
[../coordination/](../coordination/README.md). Neither page is a task authority
or an autonomous process monitor. The server reads only explicitly allowed
project files and serves no write endpoint. Old dashboard URLs redirect here.
