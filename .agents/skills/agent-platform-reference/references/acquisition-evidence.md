# Source inventory and acquisition evidence (reviewed 2026-10-13)

## Codex source and measured acquisition

The GitHub Releases API was queried directly on 2026-10-13 at `https://api.github.com/repos/openai/codex/releases/latest`; it returned `rust-v0.162.1`, published 2026-10-09T19:44:51Z. The same tag was used by the benchmark. Its annotated tag object is `7a078f6bb4ce0dc2a81408b334ce52979d92c603`; the peeled commit checked out by both Git methods is `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d`.

A successful local run compared the release archive, depth-1 tag clone, and depth-1 `blob:none` sparse checkout. Measured times were 17.494 s, 10.983 s, and 21.753 s respectively, including extraction/checkout. The complete archive and shallow checkout each contained 9,018 path entries (9,017 regular files and one symlink) with 89,296,870 logical path bytes (89,296,863 regular-file bytes plus 7 symlink-path bytes). The archive occupied 112,951,296 allocated bytes after extraction and its verified compressed payload is 17,186,781 bytes. The shallow clone had the same worktree plus 21,175,692 logical bytes of Git metadata. The sparse checkout (`docs`, `codex-rs/core`) had 1,031 files and 19,510,343 logical checkout bytes, plus 5,531,416 Git metadata bytes. Full logical and allocated byte details and measurement limitations are in [`../evidence/benchmark-verification.md`](../evidence/benchmark-verification.md) and machine-readable [`../evidence/benchmark-details.json`](../evidence/benchmark-details.json).

These are one serial run in this environment. Local allocation is not logical size; the benchmark TSV `du -sk` values are rounded to 1 KiB. Archive bytes and Git metadata sizes are not directly comparable wire-transfer measurements, and no network-interface byte counter was enabled. Sparse checkout is intentionally incomplete and may fetch omitted blobs later. The retained archive passed `tar -tzf` integrity validation and every regular tar entry matched the retained extracted tree. A copy of the benchmark log and raw latest-release API responses are retained under `evidence/`.

Re-run with `bash scripts/benchmark_codex.sh rust-v0.162.1` in a clean output directory. That script measures elapsed time and directory allocation; add an independent network monitor if actual wire bytes are needed. Always query the latest release API before interpreting a pinned run as current.

## Other source acquisition observations

- Anthropic Claude Code publishes a machine-oriented Markdown index at `https://code.claude.com/docs/llms.txt` with direct Markdown endpoints. Treat this as an index for discovery; record version/surface and retrieval metadata per fetched page.
- OpenAI Help Center pages are HTML sources; no verified complete Markdown corpus or equivalent single machine-readable index was established during this inventory.
- `openai/codex` source is not the full user-facing Codex documentation. At the pinned release, `docs/skills.md` is a brief pointer to the online Codex Skills guide, and `docs/config.md` primarily points to online documentation with a managed-hooks implementation note. Repository acquisition alone therefore leaves documentation gaps.
- Agent Skills supports optional `scripts/`, `references/`, `assets/`, and additional directories; there is no required `data/` directory or universal mirror-manifest schema. Client implementations can support different subsets.

## Historical context (not a current-release measurement)

A public `openai/codex` issue dated 2026-03-11 reported a 341 MiB full clone, attributed largely to binary blobs removed from the current tree but retained in history: https://github.com/openai/codex/issues/14324. This is historical issue-reported context, not a measurement of the October release, shallow-clone traffic, archive size, or a current clone guarantee.

## Refresh and staleness

Use the manifest as both inventory and an audit of what was fetched. Recheck current vendor pages and release tags before relying on version-sensitive claims. Preserve immutable source snapshots separately from curated corrections and generated reading views; refresh should not silently erase correction notes. For any source mirror, record source URL, resolved URL, retrieval time, response/cache metadata when available, content hash, applicable product surface/version, and conversion errors. Reject login/error pages instead of labeling them valid documentation.
