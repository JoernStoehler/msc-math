# Shared Artifacts

Git is the source and reproducibility record for code, manifests, and
deliberately retained small datasets and fixtures. Large retained datasets and
pipeline caches currently use immutable directory snapshots in the
Cloudflare R2 Standard bucket `msc-math-artifacts`. Pipelines continue to read
and write ordinary local paths: materialization downloads a registered snapshot to a cache and
creates ignored symlinks at its established repository paths.

This is the selected practical route, not a finding that R2 or the temporary
directory layout is best among storage designs. Jörn's adverse experience was
specifically with GitHub LFS quotas, speed and agent habits; other approaches
have not been evaluated here.

Agent-created inputs, outputs and metadata are working material until their
owner deliberately selects a lasting result. Put each run under
`/tmp/msc-math/<task>/<run>/`, where subagents in different worktrees can access
it and multiple outputs can coexist. Use a producer-owned ignored directory
only if mutable output must persist in an active workspace. Do not put a run in
the active worktree or shared long-term storage merely because it was generated;
discard unpromoted material when the task closes. Keep any promoted
small packet with its source/evidence owner in Git; publish a promoted large
packet through the R2 snapshot contract below and record its identity in Git.

`artifacts/registry.json` is the reviewed map from a logical artifact name to
one content-addressed snapshot and its local paths. A snapshot id is the SHA-256
of its sorted file names, sizes, and SHA-256 values. The remote completion
manifest is uploaded last. A prefix without a valid `manifest.json` is not a
published snapshot, and publishing changed bytes creates a different prefix.

## Setup

[the installation workflow](../meta-topics/development-environment/installation-workflow.md#r2-data) owns rclone installation, private R2
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

If the registered snapshot is already cached, verify its local inventory and
payload without credentials, network access, symlink installation or writes:

```bash
python3 scripts/artifacts.py verify-cache polytope-invariant-table
```

This checks the local manifest against the snapshot identity in the registry,
then checks every payload's name, size and SHA-256. Its JSON result supplies the
verified cache directory for consumers with explicit input-path options. It
does not check whether R2 is reachable or still holds the snapshot. A missing
cache still requires `materialize`; the offline command never downloads data.
The same cache-root overrides apply to both commands.

## Publish a finalized snapshot

Publishing is a deliberate retention step, not part of an ordinary producer
run. Use a distinct task/run directory in `/tmp/msc-math/` for working material.
After performing the producer's declared checks, decide which large outputs
deserve a durable packet and publish only that directory:

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
