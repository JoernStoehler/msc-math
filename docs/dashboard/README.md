# Jörn's browser views

The consolidated checkout is `/workspaces/msc-math` on `main`. The dedicated
dashboard process was observed serving it on port 8765 during 4 October
consolidation. Start it with `python3 scripts/serve-project-dashboard.py` and
open <http://127.0.0.1:8765/> locally.

Use the selected agent client's built-in file/document viewer for these local
HTML artifacts. The custom viewer service and `/files` route were retired on
1 November; do not start `scripts/serve-files.py` for ordinary agent work. Its
source and historical setup notes remain in [file-viewer history](../file-viewer.md).

- [index.html](index.html) reads selected work and resources from
  `meta-topics/project-management/current.json`; it refreshes every 15 seconds.
- [thesis.html](thesis.html) retains the 4 October candidate's bounded review,
  reconciled with merged scope facts. The selected symmetric-partner result
  is still omitted from that candidate. Human PASS remains unknown.
- [thesis-sources.json](thesis-sources.json) binds the reconciled source set.
  Changed hashes signal drift; matching hashes do not prove correctness or PASS.

Assignment truth belongs in [coordination](../../meta-topics/project-management/coordination.md).
Neither page is a task authority or a process liveness monitor. Old worktree
URLs and listener observations are historical recovery context.
