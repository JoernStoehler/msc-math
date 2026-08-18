# Scripts

Small repo helper commands.

## Physical inventory

| Path | Purpose |
| --- | --- |
| `build-release.py` | closure-only builder for the reviewed tracked tree, checked thesis PDF, and verified Zenodo ZIP |
| `artifacts.py` | publish and materialize immutable shared R2 artifact snapshots |
| `bootstrap-cloud.sh` | install and verify the normal Codex Cloud development environment |
| `repo-status-summary.sh` | read-only applicability summary for dated build/test evidence |
| `repo-status/` | dated command/result records consumed by the status helper |

## `build-release.py`

This is a final-closure command, not an ordinary build or experiment command.
It requires the exact reviewed commit, a clean tracked tree, completed
third-party cleanup, and a new output path. It rebuilds and checks the thesis,
packages the tracked tree plus PDF, and verifies the ZIP manifest. Shared R2
snapshots have their own reviewed inventory in `artifacts/registry.json`.

The full closure contract and invocation live in
`docs/reproducibility.md`. The commit argument records which reviewed tree is
being released; it is not an experiment-artifact freshness test.

## `repo-status-summary.sh`

Quick orientation helper for future sessions.

It reports:

- current `HEAD`;
- working-tree cleanliness;
- changed paths since the dated verification reference;
- changed or uncommitted paths that are likely to affect old test/build results;
- changed or uncommitted paths that affect task, map, harness, or helper
  orientation;
- the tracked-data freshness caveat and high-risk artifact-refresh areas;
- registered external-artifact availability is not checked.

Use it before asking whether old test/build results still apply:

```bash
scripts/repo-status-summary.sh
```

By default it uses the newest `scripts/repo-status/repo-status-*.md` file by
filename. Pass a status reference path to check against a specific dated report:

```bash
scripts/repo-status-summary.sh scripts/repo-status/repo-status-smoke-and-core-2026-05-31.md
```

This is a read-only summary. It does not run tests, refresh datasets, or prove
that tracked generated artifacts are fresh.

## `artifacts.py`

This is the transparent `rclone` boundary for bulk data. It publishes immutable
directory snapshots, verifies remote bytes, materializes content-addressed
caches, and links the registered files at their existing ignored paths. See
`docs/artifacts.md` for configuration and the release workflow.

## `bootstrap-cloud.sh`

Use this as the Codex Cloud environment setup command. It installs normal
Rust/Python/LaTeX and artifact tooling, persists the setup-only R2 secrets in a
private ephemeral-container rclone config, and verifies remote access. The
configuration contract is in `docs/cloud-development.md`.
