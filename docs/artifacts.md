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

## Configure R2

The development image includes pinned `rclone`. Configure a remote named
`mscmath` with a bucket-scoped R2 object read/write token. On persistent
machines, keep the secret in the ordinary private rclone configuration. A
generic ephemeral shell can instead configure the remote from environment
variables:

```text
RCLONE_CONFIG_MSCMATH_TYPE=s3
RCLONE_CONFIG_MSCMATH_PROVIDER=Cloudflare
RCLONE_CONFIG_MSCMATH_ACCESS_KEY_ID=<R2 access key id>
RCLONE_CONFIG_MSCMATH_SECRET_ACCESS_KEY=<R2 secret access key>
RCLONE_CONFIG_MSCMATH_ENDPOINT=https://ef19d5c4c89e0b61a5a1560041679e2d.r2.cloudflarestorage.com
```

The helper always passes an empty S3 ACL because R2 does not implement object
ACLs, and disables bucket creation/checking so a bucket-scoped token can
transfer objects without account-level bucket-list or bucket-create
permission. Neither key belongs in Git.

Codex Cloud removes secrets after its setup phase, so do not use the generic
environment-variable arrangement there. Follow
[`docs/cloud-development.md`](cloud-development.md): configure the two setup
secrets it names and run `scripts/bootstrap-cloud.sh`, which writes the private
rclone configuration needed by subsequent agent commands.

## Consume an artifact

Discover the available snapshots and materialize only the one needed:

```bash
scripts/artifacts.py list
scripts/artifacts.py materialize polytope-datasets
```

On the persistent development machine, Compose mounts one host directory at
`/data`; materialized snapshots default to `/data/cache/msc-math`. All
worktrees therefore reuse the same verified bytes. Mutable or newly generated
work belongs under `/data/work`. In an ephemeral environment without `/data`,
the cache defaults to `~/.cache/msc-math/artifacts`. Override either case with
`MSC_MATH_CACHE_ROOT` or `--cache-root`.

Materialization refuses to replace an existing file or a different symlink.
It downloads into a temporary directory, checks the exact file inventory,
sizes, and hashes, then installs the cache atomically. Use `--no-link` when a
consumer needs the cache directory but not the established worktree paths.

## Publish a finalized snapshot

Publishing is a release step, not part of an ordinary producer run. First run
the producer into a fresh mutable directory, perform its declared cheap checks,
and decide which outputs form the durable packet. Then publish that directory:

```bash
scripts/artifacts.py publish my-artifact /data/work/my-final-packet
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
