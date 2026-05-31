# Scripts

Small repo helper commands.

## `repo-status-summary.sh`

Quick orientation helper for future sessions.

It reports:

- current `HEAD`;
- working-tree cleanliness;
- changed paths since the dated verification reference;
- changed or uncommitted paths that are likely to affect old test/build results;
- changed or uncommitted paths that affect task, map, or helper orientation;
- the tracked-data freshness caveat;
- Git LFS payload presence.

Use it before asking whether old test/build results still apply:

```bash
scripts/repo-status-summary.sh
```

By default it uses the newest `tasks/references/repo-status-*.md` file by
filename. Pass a status reference path to check against a specific dated report:

```bash
scripts/repo-status-summary.sh tasks/references/repo-status-smoke-and-core-2026-05-31.md
```

This is a read-only summary. It does not run tests, refresh datasets, or prove
that tracked generated artifacts are fresh.
