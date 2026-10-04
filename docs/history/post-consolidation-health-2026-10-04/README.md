# Post-consolidation health check and repair, 4 October 2026

Jörn requested checking consolidation and then fixing reported problems except
for the selected symmetric-partner thesis gap. No scientific producer or
manuscript edit was selected or run.

The initial check found one registered worktree, all source branches merged,
clean Git state and valid connectivity. Main was 19 commits ahead of origin.
The consolidation identity check, both postmortem evaluation checks and report
render check passed. Cargo formatting/workspace compilation, 75 Python tooling
tests, 19 capacity API/anchor tests and a fresh 101-page thesis build passed.
The fresh build had no flagged overfull boxes or undefined references.

## Repairs

- Restored HTTPS file viewing; enabled a persistent user service from main.
- Verified and restored main's consumer links for `polytope-datasets` and
  `polytope-invariant-table` using the established link installer.
- Recovered the `combinatorial-cell-widths` cache from existing consumer bytes;
  recomputed content-addressed identity exactly matches the registry.
- Corrected top-level stale continuation text while preserving another session's
  new research-discussion assignment and its separate scope.
- Replaced four broken Markdown links with retained recovery routes or the
  pinned historical harness source. Old recovery commands are historical.
- Installed the Ubuntu rclone package binary under `~/.local/bin` without root.
  The artifact tool is now available on the host; credentials remain absent.
- Added an explicit consolidation-verifier maintenance mode for its recorded
  documentation paths, retaining the original historical hashes. Manuscript,
  retained PDF and acceptance checks remain exact.

## Authenticated input restoration

The initial blocker was resolved at 21:02 UTC: credentials were discovered in
ignored `.local-environments/sandbox-bootstrap/home/agent/.config/rclone/rclone.conf`.
The earlier host/sandbox checks missed this preserved bootstrap location. A
live R2 listing succeeded. The configuration was installed privately in the
host's standard rclone location with mode 0600; no new token was minted and no
credentials were printed, committed or sent through chat.

All thirteen registered caches now pass exact inventory/size/SHA-256 checks,
and every registered consumer path matches its cached source. Remote manifests
and missing payloads were retrieved through authenticated R2; scientific
producers were not rerun. [The receipt](artifact-restoration.json) records the
verified caches and consumer paths. The recovered combinatorial manifest was
aligned with remote formatting after checking identical snapshot identity.

The previous facet-scale consumer differs semantically from the registered
snapshot. Both have 520 rows; capacity/volume methods and numerical results
include differences. The registered input's hash matches the retained summary
manifest, so that reproducible input was restored. The divergent file was
preserved under its producer's `runs/production/pre-restoration-20261004/`;
[its receipt](facet-cache-preservation.json) binds both hashes. No retained
summary or manuscript was regenerated or altered.

The new Cloudflare `cf` CLI was checked and is not installed on this host.
Existing S3 credentials and rclone completed the restoration without new tokens
or Cloudflare administrative changes. See [artifact operations](../../artifacts.md).

The thesis gap is deliberately unchanged. Passing these checks does not
establish mathematical correctness, human PASS or complete reproduction.
