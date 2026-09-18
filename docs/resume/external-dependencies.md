# Dependencies of the consolidated handoff

The manuscript's clean-export build is checked in `build-verification.json`.
All selected LaTeX, bibliography and figure inputs are tracked in this branch.
The clean build uses the installed TeX distribution, `latexmk`, and `biber`; it
requires neither another worktree nor a `/tmp` input retained from a past run.
The scratch directory used for validation is disposable and removed afterwards.

Research data can exceed what belongs in Git. The canonical registry is
`artifacts/registry.json`; `scripts/artifacts.py` materializes registered snapshots.
The historical source geometry snapshot is
`f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96` for
`polytope-datasets`. At consolidation it is present under
`/home/joern/.cache/msc-math/artifacts/polytope-datasets/`.
The recovered geometry consists of random.jsonl, random-product.jsonl and
shared-cache.jsonl. It is not stored in `/tmp` or owned by a disposable worktree.
The invariant table and provenance table have their own registry entries.

On a configured host, use `python3 scripts/artifacts.py materialize
polytope-datasets --no-link`; repeat for the table artifacts as needed. Consult
`scripts/artifacts.py --help` and `docs/artifacts.md` for the configured source
transport. The recorded recovery used the authenticated sandbox transport;
credentials are not included in this handoff. This consolidation did not read
credential contents, rerun capacity producers, or test remote redownload.

Compressed revalidation outputs and their receipts are tracked under
`docs/ds-retrospective-revalidation/`. Restore raw derivatives by the documented
local decompression commands before replaying an analysis. A historical log
naming `/tmp/...` as an output location is not a dependency if the retained output
is tracked at the location given by its report. Historical absolute worktree paths
are preserved as provenance; `branch-inventory.json` maps their changed files to
this tree. Review browser URLs may expire; their underlying PDFs, annotation
records and source packets are retained here.

The only deliberately unimported dirty file is main's machine-local
`.codex/config.toml`. It is not a manuscript/evidence dependency. The obsolete checkouts were subsequently removed after archiving their history
and non-rebuildable ignored files and retiring their review servers. Only main
and the resume checkout remain. Recovery is described in
`/workspaces/archived/workspaces-root/msc-math-session-20260918/README.md`.
The archive includes the downloaded/normalized external writing datasets previously
ignored under writing-detector-data; extract those members deliberately if needed.


## Discovered historical replay limitation

The older `ridge-mechanism-discriminator/analyze.py` defaults to two `/tmp`
inputs. Both are absent on the host at consolidation: tail-rule diagnostics and
the 100k generated feature cache. Its tracked compact results remain available,
but a full replay is not currently ready. Its README documents regeneration
from `tail-rule-mining` and `extreme-scalar-rejection-proposer/configs/100k-promising-scalars-durable.json`.
Use explicit durable input/output paths; do not run the config's `stage=all`
blindly because that can launch capacities and replace retained outputs. This
consolidation did not regenerate those caches or claim their reproduction tested.
This is a pre-existing research replay limitation, not a dependency of the thesis
PDF build. It is surfaced here so a successor does not assume every historical
experiment is executable merely because all its tracked records were consolidated.

The three `scripts/subtree-usage*` accounting files were also promoted byte-for-byte
from main's retained dirty snapshots into their canonical script locations. They
operate on durable Codex session logs, not ephemeral handoff files.
