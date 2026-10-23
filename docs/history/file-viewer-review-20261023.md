# File viewer diff review — 2026-10-23

## Inspected

Reviewed the complete unstaged diffs for `docs/file-viewer.md`, `scripts/serve-files.py`, and `scripts/test_serve_files.py`, plus the full current viewer implementation and its documented purpose. Current purpose is a read-only, loopback-default browser viewer over explicitly configured roots, with escaped source rendering, raw/static serving and dashboard use; current docs note browser-executed HTML and that roots are the security boundary. The diff scopes a transfer optimization to two fixed Workbench artifact directories under `~/.codex/artifacts`. No credentials, session logs, unrelated personal files, server launch, repository edit, or test run.

## Recommendation: keep the change, with one small coverage improvement

The implementation keeps authorization in `translate_path` before artifact handling and requires the resolved file and lexical URL to stay in the same configured artifact subtree. It adds no serving root and no endpoint. It leaves directories outside those two builds on the prior behavior. For selected assets, it bounds in-memory handling at 4 MiB, uses a 16-entry LRU representation cache keyed by path plus mtime/ctime/size/compression, negotiates gzip only for HTML/JS/CSS, varies on `Accept-Encoding`, and uses representation-byte ETags. Immutable caching is limited to matching hash-shaped names below `assets/`; HTML, JSON, unhashed files, and rendered-source paths remain `no-store`. This matches the documented purpose of improving repeat transfer of the two published Workbench builds without broadening what the viewer exposes.

The added tests are focused on raw body/type/cache policy, gzip and identity negotiation, encoded-vs-identity validators/HEAD/304, directories, rendered source, outside roots, symlinks resolving outside a build, and republishing HTML. I found no concrete security or privacy regression in the diff during this bounded read.

## Specific note

The “hashed filename” rule is a naming-convention check, not verification that the filename digest matches file bytes. That is reasonable for these controlled Vite/VitePress build outputs and is scoped to their two asset trees; the docs should be understood that way. If the directories can contain hand-authored or non-build content later, tighten the scope or verify the asset manifest before promising immutable caching.

The tests do not directly exercise the invalidation behavior for a cached hashed asset, although the cache key includes nanosecond mtime, ctime and size and the HTML invalidation test exercises the same mechanism. Suggested exact follow-up: parameterize the republish test with `assets/app-AbCd1234.js` and rewrite its bytes, assert a new ETag/body, and retain the immutable cache-control assertion. This is a coverage improvement, not a blocker.

## Validation and limits

No tests run: the focused test module constructs and launches a localhost HTTP server, which the task explicitly prohibited. No live Workbench artifact contents, external service, or browser cache behavior inspected. Recommendation is limited to the actual diff and source paths above, not a full security audit of the existing viewer, its systemd/Tailscale deployment, or any unpublished Workbench payload.


# File viewer focused tests — 2026-10-23

## Fixture safety check

Before execution, inspected all of `scripts/test_serve_files.py`. Test writes are confined to `tempfile.TemporaryDirectory` fixtures (`msc-files-*`, `workbench-assets-*`): fixture text/data/assets, one symlink under that fixture, and a source file append by the Node source-check test. The tests start ephemeral HTTP servers bound to `127.0.0.1` on port 0 and shut them down in `tearDownClass`; they do not invoke the live viewer, systemd, Tailscale, project producers, or external services. Pandoc (if installed) and Node (if installed) are invoked only for bounded rendering/source-check cases, with subprocess timeouts. No host mutation outside temporary fixtures was apparent.

## Result

Command, from `/workspaces/msc-math`:

```sh
python3 scripts/test_serve_files.py
```

Exit status 0. **14 tests ran in 1.765s; all passed.** No skips were reported. This includes the new `WorkbenchArtifactTests` and existing viewer/dashboard contracts. Temporary fixtures and ephemeral servers were cleaned up by test teardown; no repository edits were made.

## Limits

This is the existing focused test module only. It does not establish browser behavior beyond its HTTP assertions, live deployment state, security of the full default-root configuration, or correctness of the served Workbench artifact payloads.
