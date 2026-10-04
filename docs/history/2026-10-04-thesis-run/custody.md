# Stopped-run custody audit

Observed 2026-10-04 approximately 00:28 UTC. Audit scope: read-only repository/trace/PDF metadata; no build, test, producer, commit, scientific edit or resumption. Receiving owner: `/root`. This report is a preservation handoff, not an accepted candidate or fresh mathematical review.

## Exact checkout and custody

- Worktree: `/home/joern/.codex/worktrees/f171/msc-math`.
- Branch: `thesis/2026-10-04-candidate`.
- HEAD/base of all present uncommitted edits: `52b1526c8ff5613a400f2f4028b4290f35841b21` (Retain October session postmortem and reusable causal knowledge).
- This HEAD is also the other worktree branch `codex/session-postmortem-2026-10-04`; do not confuse the new dirty candidate branch with that completed historical postmortem.
- `main` is in `/workspaces/msc-math` at `15cfd185`, reported ahead 2/behind 2 of origin/main. Merge base of this HEAD and main is `45ebc516c54c3a8b77a0eee512e8be6f55222b1a`; no merge/rebase is implied or needed for preservation.
- User explicitly stopped thesis work at 00:17:28 UTC. Root reports all ten original workers completed/interrupted and no matching build/test processes at 00:18:10. That is a dated observation, not a permanent liveness claim. Current task allows handoff preparation, not resumed thesis execution.
- Preserve all 33 dirty tracked files and the untracked dated audit; no staged edits were shown. Parent plans a preservation checkpoint and truthful stopped-state repair. Checkpointing these changes would retain unreviewed work, not endorse it.

## Ownership of dirty output

Ownership below is assignment/report attribution, not a claim of byte-level authorship verification. Root is receiving integrator for every row.

| Owner | Dirty outputs |
| --- | --- |
| foundations (returned final report) | Chapter 02 EHZ, Chapter 02 Lagrangian products, Chapter 03, Chapter 04 |
| hko (interrupted) | Chapter 06, Chapter 07, `hko-lemma-bounds.tex`, `hko-sections.tex`, HKO appendix |
| products (interrupted; independent reviewer returned) | Chapters 09, 10, 11 |
| empirical (interrupted) | Chapters 08, 12, 13; data-science appendix |
| artifacts (interrupted) | `INSTALL.md`, Chapter 14, bootstrap-cloud.sh, repo-status-summary.sh |
| repo_health (returned) | seven navigation files: formal FINDINGS/README/affine README; experiments README/regular-products README/certificate README; knowledge README; untracked dated repo-audit.md |
| root | current.json, graph DOT/SVG, render-workflow-graph.py, serve-project-dashboard.py; introduction clarity repair |
| independent reviewers | reported read-only; no authored edits claimed |

Exact observed status:

```text
 M INSTALL.md
 M docs/coordination/current.json
 M docs/coordination/graphs/tasks.dot
 M docs/coordination/graphs/tasks.svg
 M docs/knowledge/README.md
 M experiments/README.md
 M experiments/regular-products/README.md
 M experiments/regular-products/pentagon-rotation-formula-proof/README.md
 M formal/FINDINGS.md
 M formal/README.md
 M formal/pentagon-affine-products/README.md
 M scripts/bootstrap-cloud.sh
 M scripts/render-workflow-graph.py
 M scripts/repo-status-summary.sh
 M scripts/serve-project-dashboard.py
 M thesis/appendices/data-science.tex
 M thesis/appendices/hko-certificate.tex
 M thesis/chapters/01-introduction.tex
 M thesis/chapters/02-preliminaries-ehz-capacity.tex
 M thesis/chapters/02-preliminaries-lagrangian-products.tex
 M thesis/chapters/03-generalized-reeb-orbits-polytopes.tex
 M thesis/chapters/04-quadratic-program.tex
 M thesis/chapters/06-variation.tex
 M thesis/chapters/07-hko.tex
 M thesis/chapters/08-data-science.tex
 M thesis/chapters/09-rotated-regular-polygons.tex
 M thesis/chapters/10-affine-pentagons.tex
 M thesis/chapters/11-product-position.tex
 M thesis/chapters/12-visualization.tex
 M thesis/chapters/13-numerics.tex
 M thesis/chapters/14-code-data.tex
 M thesis/chapters/hko-lemma-bounds.tex
 M thesis/chapters/hko-sections.tex
?? docs/history/2026-10-04-thesis-run/
```

## Checks and PDFs: do not combine snapshots into acceptance

| PDF | Pages | SHA-256 | Filesystem mtime UTC |
| --- | --- | --- | --- |
| `thesis/build/main.pdf` | 96 | `3ca74e17fe45fb718c4d7fb77b4c529092cb6ac27a1e63885e9ad71276d3d20f` | 2026-10-04T00:14:41.396244+00:00 |
| `/tmp/hko-thesis-audit/main.pdf` | 101 | `9cd80c7df566c55eb6c41b857bf580fb6dbc682ceab1ccb018ffcd1a8e55edf2` | 2026-10-04T00:17:27.728674+00:00 |
| `/tmp/foundations-thesis-qa/main.pdf` | 98 | `34c4cfbf7c6d794f410c26b6b3dac7b4549508fc04a5ac841fe0101e0edf082c` | 2026-10-04T00:15:49.869421+00:00 |
| `/tmp/msc-products-20261004/main.pdf` | 101 | `44d84c29a1afc41ff91515a877a4bf58ed53c412878cc852b7dff3ee3362c69e` | 2026-10-04T00:17:40.670707+00:00 |
| `docs/resume/thesis-resume.pdf` | 94 | `5b487580ee5bed7473e7de3233ea3ccaacbb3c7400d685883b99a081aa582fea` | 2026-10-04T00:06:43.112007+00:00 |

- `thesis/build/main.pdf` is the initial 96-page live-tree build; initial log ends successfully at 00:14:41. Its output predates most returned edits. No frozen input manifest was found for it, so it does not certify the dirty tree. Baseline page images/contact sheet are under `/tmp/msc-thesis-qa/baseline/`; root viewed one contact sheet, not whole-PDF acceptance.
- Foundations final report (trace source line 270): stable snapshot build, rendered revised pages inspected, scoped diff check, exact Chapter 5 calculation. Snapshot source is `/tmp/foundations-thesis-snapshot`; build log `/tmp/foundations-thesis-snapshot-build.log`. These results cover that snapshot and bounded scope, not current whole tree.
- Product theorem reviewer final report (line 274): no high/medium defect found in Chapters 09–11; checked affine cases, threshold/Hausdorff argument and exact triangle enumeration. No files changed; no novelty, human acceptance or final PDF verdict.
- Independent summary reviewer final report (line 212): no high/medium defect found in assigned summary/provenance scope; two minor suggestions; no edits, certificate reruns or final PDF acceptance.
- Repo-health final report (line 266) and retained `repo-audit.md`: seven documents' paths/Markdown links resolve, inventory matches 20 tracked directories, scoped diff check passed. No mathematical or integrated-PDF verdict.
- Frozen `docs/resume/thesis-resume.pdf` is an older 94-page diagnostic. `build-verification.json` ties it to source commit `e3615f32c1c8b497f3615cbaf3bb335a8d005ad5`; source manifest already mismatched seven HEAD files on 30 September and has not been refreshed. Do not transfer that build result to current source.
- Many worker MESSAGE bodies and spawn assignment texts in the supplied trace are encrypted. This audit could read only their plain final reports and root's summaries. Preserve parent-visible reports separately if they contain acceptance details; the trace alone is insufficient for independent recovery of all findings.

## Known outstanding integration defects and caveats

These are direct surviving coordination observations or root's retained report (trace line 338), not newly rerun results:

1. Dashboard HTTP contract remains inconsistent: `INSTALL.md` was newly allowlisted in `serve-project-dashboard.py`, while `test_project_dashboard.py` still expects its route to be 404. Root explicitly reported this unresolved. No assertion that the whole dashboard test suite passes.
2. Reported overfull paths: approximately 20.8 pt Chapter 13 and 35.3 pt Chapter 14. No repair or final rendered check established.
3. Numerical reviewer raised an unresolved action-window enumeration contract: minimizer-specific pruning may omit stationary words promised by broad enumeration. Fresh reproduction unfinished. This does not establish wrong scalar capacity values; no Rust files are currently dirty.
4. Installation rewrite is 421 changed lines and was not integration-reviewed for loss of useful setup knowledge. `lmodern` addition is explicit in bootstrap diff.
5. Non-HKO evidence paths reportedly resolve through a nested retained archive, not stated direct locations; evidence was not established lost.
6. Independent summary review suggested explicit theorem references in conclusion line 21. Conclusion is not dirty, so that suggestion was not integrated in this run. Introduction wording was edited.
7. Mathematical worker fixes remain preserved but collectively unaccepted. Build success, scope-limited source review and exact arithmetic checks do not provide a whole-thesis verdict.

## Stale continuation surfaces requiring truthful stop repair

- `current.json` at 00:15:11 still calls the original seven workers and root candidate RUNNING and directs integration/build before 02:00 UTC. It omits three additional reviewer assignments. Correct current scope/summary, oct04 tasks, relevant project milestone next steps, restart/selection decisions and resource observation so old authorization is not read as resumed execution. Null released owners; preserve returned/interrupted outcomes and unmet integration dependencies. Do not mark candidate done.
- `handoff.md` still opens with 30 September's `/workspaces/msc-math` main as sole registered worktree and says cleanup-only authorization. Replace this continuation entry with the exact branch/base, stop, dirty/checkpoint custody and unresolved results; preserve historical recovery links and existing PASS definition as historical acceptance context, not a gate on this stopped-run checkpoint.
- Graph DOT/SVG should follow repaired current.json. Parent owns regeneration and any focused structural check; this auditor ran none.
- Scientific dashboard remains explicitly dated 25 September and says reconciliation open; source baseline note also says path migration only. Keep it visibly stale, now naming 4 October unintegrated changes if cheaply possible. Do not refresh hashes or label it current without scientific/PDF reconciliation. `thesis-work.md` was not reconciled during the run.

## Minimal durable package

1. Preservation checkpoint all present tracked edits plus dated repo audit, with a commit message saying stopped/unreviewed, after truthful stop-state doc repair. No scientific changes required merely to make stopped custody truthful.
2. Retain this report and readable worker stop reports in `docs/history/2026-10-04-thesis-run/`, plus exact patch/checkpoint commit ID, PDFs/hashes and applicable logs/input snapshots. The current `/tmp` locations are recovery pointers, not durable archive promises. If copying PDFs is too costly under the handoff deadline, name that limitation rather than imply freezing succeeded.
3. A fresh session should read current handoff and this dated audit, verify branch/checkpoint, preserve unresolved edits, and ask no reconstruction of old agent panes. It should not run experiments, integrate fixes or resume thesis work until the user explicitly resumes/selects that task. Once resumed, compare approaches and allocate integration/verification ownership before new production; the named unresolved numerical contract and PDF layout are review items, not permission to execute while stopped.

No accepted integrated PDF exists on the evidence available to this audit. A truthful preservation checkpoint can be made now; readiness to continue is a separate authorization and planning claim.
