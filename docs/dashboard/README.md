# Jörn's browser views

Use the [file viewer](../file-viewer.md): prefix a dashboard's absolute path with
`https://joern-pc.tailc5e761.ts.net/files`. The URL identifies the checkout;
relative state, evidence and asset links stay in that checkout. For this worktree:

<https://joern-pc.tailc5e761.ts.net/files/home/joern/.codex/worktrees/60b9/msc-math/docs/dashboard/index.html>

The earlier port-8766 worktree service was unavailable when checked on 3 October.
Port 8765 serves `/workspaces/msc-math`, a separate checkout. The shared file
viewer does not replace or restart that existing service.


Run `python3 scripts/serve-files.py` for a local file view, then use
`http://127.0.0.1:8770/files` plus the dashboard's absolute path. The older
`python3 scripts/serve-project-dashboard.py` entry point remains supported at
<http://127.0.0.1:8765/>.

- [index.html](index.html) shows selected work, owners, dependencies, decisions,
  resource observations and recent changes from `docs/coordination/current.json`.
  It refreshes every 15 seconds and clearly labels older observations.
- [thesis.html](thesis.html) is the retained scientific brief. Its current
  scientific baseline is 25 September, with reconciliation still open. The
  file view compares cited sources in the browser against [thesis-sources.json](thesis-sources.json);
  hashes do not establish that the brief is complete or correct.

Keep browser presentation here and assignment truth in
[../coordination/](../coordination/README.md). Neither page is a task authority
or an autonomous process monitor. The file viewer reads its configured directories
and has no write endpoint. The older dedicated server reads only its allowlisted
project files and redirects old dashboard URLs here. Older checkouts need the
relative-link changes before their dashboards work through the generic viewer.
