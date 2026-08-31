# Scripts

Small repo helper commands.

## Physical inventory

| Path | Purpose |
| --- | --- |
| `build-release.py` | closure-only builder for the reviewed tracked tree, checked thesis PDF, and verified Zenodo ZIP |
| `artifacts.py` | publish and materialize immutable shared R2 artifact snapshots |
| `check_no_git_lfs.py` | reject active Git LFS attributes or pointer files in the tracked tree |
| `bootstrap-cloud.sh` | install and verify the normal Codex Cloud development environment |
| `maintain-cloud.sh` | refresh repository-dependent caches when a cloud environment resumes |
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
caches in the environment's standard XDG cache, and links the registered files
at their existing ignored paths. See `docs/artifacts.md` for configuration,
cache overrides, and the release workflow.

## `check_no_git_lfs.py`

Git LFS is retired in favor of the registered R2 artifact workflow. Run this
check after changing artifact storage or tracked data:

```bash
scripts/check_no_git_lfs.py
```

The final release builder runs it automatically. Historical prose and tests
may still name legacy Git LFS pointers; the check rejects only active
attributes and files whose contents are actual pointer records.

## `bootstrap-cloud.sh`

Use this as the Codex Cloud environment setup command. It installs normal
LaTeX and artifact tooling missing from the universal image, verifies the
selected Rust/Python runtimes, copies the setup-secret R2 credentials into a
private rclone config available to agent commands and cached resumes, and
verifies remote access. The configuration and credential-lifetime contract is
in `docs/development-environments.md` and `docs/artifacts.md`.

## `maintain-cloud.sh`

Use this as the Codex Cloud maintenance command. It runs after the requested
branch is checked out in a resumed cached environment and fetches the locked
root-workspace Cargo dependency graph without the setup-secret environment
variables.
