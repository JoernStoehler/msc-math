# Shared Artifacts

Git is the source and reproducibility record. Large generated datasets and
pipeline caches are immutable directory snapshots in the Cloudflare R2 Standard
bucket `msc-math-artifacts`. Pipelines continue to read and write ordinary
local paths: materialization downloads a registered snapshot to a cache and
creates ignored symlinks at its established repository paths.

`artifacts/registry.json` is the reviewed map from a logical artifact name to
one content-addressed snapshot and its local paths. A snapshot id is the SHA-256
of its sorted file names, sizes, and SHA-256 values. The remote completion
manifest is uploaded last. A prefix without a valid `manifest.json` is not a
published snapshot, and publishing changed bytes creates a different prefix.

## Setup

[INSTALL.md](../INSTALL.md#r2-data) owns rclone installation, private R2
configuration and the initial download check. This file owns artifact storage,
materialization and publication contracts.

## Consume an artifact

Discover the available snapshots and materialize only the one needed:

```bash
scripts/artifacts.py list
scripts/artifacts.py materialize polytope-datasets
```

By default, each execution environment uses its standard
`$XDG_CACHE_HOME/msc-math/artifacts/` cache, falling back to
`~/.cache/msc-math/artifacts/`. All worktrees in that environment therefore
share immutable downloaded snapshots. Host and Docker Sandbox have different
home directories and may each cache the same snapshot; that bounded duplication
is intentional. Use `MSC_MATH_CACHE_ROOT` or `--cache-root` for an explicit
override.

Materialized consumer links point into the cache of the environment that
created them. Do not assume that a materialized link in one worktree is usable
from another environment. R2, not any environment cache, is the durable layer.
Codex Cloud cache reuse is opportunistic and must not be assumed across tasks.

Materialization refuses to replace an existing file or a different symlink.
It downloads into a temporary directory, checks the exact file inventory,
sizes, and hashes, then installs the cache atomically. Use `--no-link` when a
consumer needs the cache directory but not the established worktree paths.

## Publish a finalized snapshot

Publishing is a deliberate retention step, not part of an ordinary producer
run. Use `/tmp` for disposable debugging and quick iteration. Use a
producer-owned ignored directory only when mutable output should remain with the
active workspace. After performing the producer's declared checks, decide which
outputs deserve a durable packet and publish only that directory:

```bash
scripts/artifacts.py publish my-artifact path/to/my-final-packet
```

The command uploads file payloads immutably, performs a byte-level remote
check, and writes the deterministic completion manifest last. Add or update the
resulting snapshot id, provenance, and local links in
`artifacts/registry.json` only after that check succeeds. Never overwrite a
registered snapshot or use synchronization with remote deletion as a publish
operation.

Do not casually rerun expensive scientific producers merely to refresh storage
metadata. Historical evidence remains historical; its registry provenance
should say so. A new scientific run gets a fresh output directory, validation,
snapshot, and review.
