# Jörn's browser views

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
