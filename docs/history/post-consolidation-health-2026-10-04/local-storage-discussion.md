# Local-storage discussion and handoff, 4 October 2026

Sender: health-check/storage discussion root `01a1089e-3b1c-7283-b526-831482155067`.
Recipient: main research root `01a1089f-b4f6-7031-9e98-b648dabb1770`.
Jörn requested sending this discussion so the main session can consider whether
migration would help. This session ends after sending; no migration was implemented.

## Latest preference and proposal

Jörn identified friction in remote storage and proposed ordinary local locations:
`/data/msc-math/`, `/tmp/msc-math/`, `~/.codex/artifacts/`, committed and gitignored
repository files and worktrees. He accepted the broad local-primary direction,
requested a premortem and concrete examples, then challenged `materialize` as
magic: direct files or explicit copying would be simpler. The final proposal
therefore supersedes the earlier suggestion to make materialization local-first.

Use ordinary retained input directories under `/data/msc-math/`, with explicit
input paths in experiment commands and READMEs. Preserve run/version identity,
provenance and hashes alongside retained inputs. Keep mutable producer outputs
separate. Commit small required inputs where suitable; use explicit `cp` for
checkout-local inputs when needed. A worktree may be a convenient copy source,
but must not be the sole retained owner of valuable output. `/tmp` is scratch;
Codex artifacts are review copies, not the only evidence copy.

Verification can remain an optional plainly named hash check. Avoid making a
custom storage command, automatic symlinking or networking prerequisites of
ordinary access. Existing consumers may need a small explicit input-path option;
check this in a bounded representative pilot rather than redesigning all code.
The `/data/msc-math` path is a workstation convention, not a universal path.

This is a proposed simplification to consider, not a selected mass migration,
remote deletion, expensive rebuild or new authorization for scientific execution.
Retain R2 until an independent backup/recovery arrangement is verified.

## Evidence and premortem

- [Health repair](README.md), commits `40f1045a` and `4db92513`: all thirteen
  registered snapshots downloaded/verified and their consumer paths restored.
  Existing credentials were found in ignored sandbox-bootstrap configuration;
  no new Cloudflare token was required. Host rclone now has private configuration.
- [Artifact restoration receipt](artifact-restoration.json): local inventory,
  byte/hash checks and the actual consumer paths. This is the registered set,
  not an exhaustive inventory of useful generated project material.
- [Facet preservation receipt](facet-cache-preservation.json): two distinct
  520-row versions existed. The registered snapshot matches retained summary
  input hashes; the differing local version was preserved under its producer.
  This motivates separating writable outputs from retained input snapshots.
- [Current artifact contract](../../artifacts.md) and
  [helper](../../../scripts/artifacts.py): materialize contacts R2 before using
  an existing cache; links target environment-specific cache locations. Existing
  verify-cache is offline. These are concrete sources of the access friction.
- At 21:29 UTC, `df`/`findmnt` showed `/data/msc-math`, the repository and the
  host cache all on `/dev/sdb1`, with about 93 GB available. Moving among these
  locations gives no independent hardware backup; avoid gratuitous duplication.
- [Sandbox definition](../../../sandbox/sbxenv.yaml) already declares `/data/msc-math`
  and MSC_MATH_DATA. Verify actual access from host and sandbox in any pilot.

Likely failure modes: loss of the sole local copy, overwrite through input
symlinks, worktree retirement losing unregistered outputs, hidden networking
surviving the migration, mismatched sandbox paths, and added configuration
complexity. A useful pilot demonstrates offline input use, unchanged hashes,
separate output writes, checkout retirement safety and independent recovery.
No migration, pilot or producer rerun happened during this discussion.

## Receiving action

Consider whether this simpler local layout improves current agent workflows;
carry the preference and concrete evidence into any storage selection. Keep
current research assignments intact. The handoff does not replace them or
require an immediate migration. Reply through the thread transport if useful;
the sending session is stopping at Jörn's request.
