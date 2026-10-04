# Worktree consolidation, 4 October 2026

All registered worktree results were merged into local main, together with the
fetched remote divergence. Nothing was pushed. Integration retains the 101-page
thesis candidate and newer root-session postmortem.

| Source | Retained result | Custody |
| --- | --- | --- |
| Detached 60b9, checkpoint 4f2ac145 | Workflow tools, approved config trials, messaging skill, stopped/unselected proposals | Snapshot fbf9ad07; merge c86b184a |
| b0b2, postmortem branch | Postmortem history, bounded calibration results, standalone candidate and planning records | Snapshot f5bd793b; merge 1f9e925f |
| f171, thesis candidate branch | Reviewed 101-page candidate, code repairs, figures and reproduction evidence | 4dff4475; merge c784d9ef |
| Remote main | Friction records and Page-reconciliation guidance | Merge f276d50c |

## Integration corrections

The shared registry preserves results but releases stale owners. Closed windows
are not live assignments. Host-local result records establish primary reset
outcome `reset` and fallback outcome `skipped`; no new reset/account request was
made. The transient file viewer is absent; the dedicated dashboard uses main.
Retaining proposals does not approve their execution.

Merged facts establish that the fixed-pentagon/symmetric-partner result was
selected for inclusion and remains omitted from the candidate. The dashboard
exposes that gap alongside human PASS and reproduction limits. No theorem or
new thesis prose was produced during consolidation.

The standalone source moved from `thesis/candidates/symmetric-partner/` to
[its retained packet](../symmetric-partner-candidate-2026-10-03/README.md), so the
selected thesis inventory retains its original 61 inputs. The PDF remains at
`output/pdf/symmetric-partner-candidate.pdf`. Generated `tmp/pdfs/` files were
removed from the current tracked tree; they remain in snapshot f5bd793b and the
recovery archive. Frozen postmortem evaluation packets retain original paths and
bytes; this relocation record supplies their current route.

## Recovery

`/workspaces/archived/msc-math-consolidation-20261004/` contains the pre-merge Git
bundle, per-checkout HEAD/status records, binary tracked-change patches,
untracked SHA-256 manifests/tars, and ignored-file inventories/tars. Ignored build
caches were preserved too; this was more expensive than necessary. All 60
formerly untracked files were checked against archive hashes and the initial
merged tree. Ignored archives contain every listed file: 10 in 60b9, 1,976 in
b0b2 and 9,000 in f171. Cache inventory completeness does not establish individual
byte integrity. Host-local artifacts/scratch were not removed. Original source
branches remain recovery references.

## Validation and limits

The original candidate verifier passes in clean f171; both postmortem evaluation
verifiers pass on main. Documentation changed during integration, so its
historical whole-repository hash record deliberately does not match main. The
consolidation verifier checks unchanged selected manuscript/PDF, retained
acceptance bindings and explicitly recorded changed documentation. Original
acceptance manifests are not rewritten.

```bash
python3 docs/history/worktree-consolidation-2026-10-04/verify.py --allow-documentation-changes
python3 scripts/serve-project-dashboard.py --check
```

`verification.json` retains the original checks, documentation hashes and retirement
outcome. Omit `--allow-documentation-changes` to require that exact historical
documentation baseline. The maintenance flag allows later edits only to its
explicitly recorded documentation paths; manuscript, PDF and acceptance bindings
still require exact identity. It does not validate the updated documentation. Passing identity,
compilation and finite tests does not establish mathematics, human PASS, full
reproduction or useful behavior from retained guidance.

## Execution correction

The first phase used excessive serial investigation and archived unnecessary
build caches; bulk staging committed temporary files. The user stopped it for a
report. There was no five-minute task deadline: the initial example concerned a
possible inconclusive Luna investigation. The root's missed-deadline claim was
unsupported. On resumption, preservation and test owners worked independently
while root reconciled shared state/documentation. This records an allocation
change, not general efficiency or orchestration success.
