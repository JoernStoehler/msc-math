# Closure-time rights, data and archive inventory

Reviewed checkout: `/workspaces/msc-math`, HEAD `9667e351514faf1f00e2d86796c20e9d9b6f2f10`, 22 October 2026 UTC. This is an inventory of dispositions and questions, not a release-readiness verdict or authorization to delete, pack, submit or publish. The repository was read only; the requested report and disposable message-review packet are the only intended writes. Existing modifications to `docs/file-viewer.md`, `meta-topics/project-management/current.json`, `scripts/serve-files.py` and `scripts/test_serve_files.py` were left alone. A later status check also showed a new untracked `docs/history/thesis-agent-review-20261022/` directory; its authorship was not inspected and this review left it untouched.

## Scope and evidence limits

Started from `README.md`, `meta-topics/thesis-submission/archive-closure-checklist.md` and `docs/project-facts.md`; followed the rights audit, reproduction/figure contracts, release/materialization implementations, registry and its producer READMEs, relevant literature/harness owners and current PDF manifests. Inspected the full tracked path/mode inventory (6,993 entries); did not conduct an exhaustive content/privacy or legal audit of every file. No raw session stores, credentials, private communications, current remote services or Zenodo account were inspected. No producers, builds, cold downloads, archive construction/extraction or publication were run. The 40-minute limit was a ceiling, not a request to consume the whole interval.

Read-only checks: all 13 registered local snapshots passed `python3 -B scripts/artifacts.py verify-cache NAME`; all snapshot payload names exactly match registry link source names, with no collisions against tracked target paths. These checks hashed 39 files / **1,475,450,088 bytes** and checked content-addressed manifest identities. They establish local cache integrity, not current R2 accessibility, public access or empty-cache recovery. All 52 input hashes in the 22 October working-PDF manifest match current files; the existing working PDF matches its recorded SHA-256 `00aabd301cd9dcaae0c0a67b81dc0545533789f4d1ca24007831109c32384a06`. No PDF was rebuilt or visually reviewed here.

Status vocabulary: **surviving** means a supported gap or explicit decision remains; **solved** means the scoped concern is already met by inspected evidence; **withdrawn** means the broader suspected defect is unsupported or conflicts with accepted scope. No fix was applied by this review. High severity means a material rights/payload/identity blocker at closure; medium means a material missing disposition or verification route; low means misleading maintenance text; informational means a boundary or completed prerequisite.

## Registry disposition inventory

All producer paths exist. `release:true` selects **12 snapshots / 21 ordinary payload files / 748,982,368 bytes**. The sole false entry holds **18 files / 726,467,720 bytes**. Each row's proposed disposition preserves existing scientific caveats; release selection is not mathematical acceptance.

| Artifact and exact registry reference | Release | Verified local payload bytes | Proposed disposition |
| --- | --- | ---: | --- |
| `alternative-source-transfer-v1`, `artifacts/registry.json:3` | true | 80,051,327 | Retain frozen source table and matched compact analysis; include after final rights/claim checks. |
| `combinatorial-cell-widths`, `artifacts/registry.json:13` | true | 11,012,601 | Retain sampled profiling rows as finite evidence. |
| `covariance-rho-frozen-packet`, `artifacts/registry.json:23` | false | 726,467,720 | Keep durable R2 snapshot; explicitly decide ZIP inclusion for row-level continuation (F07). |
| `facet-scale-production-cache`, `artifacts/registry.json:50` | true | 12,010,922 | Retain expensive production-panel cache with historical evaluator caveats. |
| `optimizer-candidate-history-f10-128`, `artifacts/registry.json:60` | true | 77,189,318 | Retain tuning trajectories, distinguished from held-out evidence. |
| `optimizer-heldout-f10-64-finalists`, `artifacts/registry.json:72` | true | 107,830,221 | Retain held-out finalist trajectories and comparison contracts. |
| `pentagon-kkt-branch-landscape`, `artifacts/registry.json:84` | true | 22,341,369 | Retain sampled landscape as explanatory finite evidence. Its consumer path is absent here, but its full cache verifies; do not regenerate it to repair a local link. |
| `polytope-datasets`, `artifacts/registry.json:94` | true | 252,398,018 | Retain original source geometries and shared producer cache. |
| `polytope-invariant-table`, `artifacts/registry.json:106` | true | 24,250,539 | Retain historical method-facing table; distinguish the restored feature derivative. |
| `product-bounce-class-degeneration`, `artifacts/registry.json:116` | true | 40,423,996 | Retain exact pair/row diagnostics with finite-population scope. |
| `product-bounce-distribution`, `artifacts/registry.json:127` | true | 12,075,189 | Retain class minima and their source identities. |
| `qp-error-bounds-broad`, `artifacts/registry.json:137` | true | 72,319,165 | Retain broad raw rows and matched formula evaluations. |
| `qp-error-bounds-soundness-v2`, `artifacts/registry.json:148` | true | 37,079,703 | Retain separate soundness-v2 evidence, without replacing the broad population. |

## Findings

### F01 — Surviving · High — Known third-party cleanup still blocks the current tree

**Evidence:** `scripts/build-release.py:67`–`77` refuses any tracked path under `topics/literature/sources/` and the three downloaded-form prefixes. The current index contains **36 paths** in that mixed source directory (30 inside the three paper directories, six root notes/metadata paths) and **eight form/conversion paths**. `topics/literature/README.md:16`–`20` explicitly says the paper collection lacks a per-file license/complete upstream manifest and presence does not establish redistribution permission. The required boundary is `meta-topics/thesis-submission/archive-closure-checklist.md:12`–`17`.

**Proposed fix/disposition:** At closure, relocate project-authored `citation-index.md`, `matlab-extraction.md`, `sign_convention_verification.md` and useful collection metadata before excluding the cached article sources/figures. Update consumers and preserve source references. Disposition the eight downloaded university forms separately once their submission use is known. Then curate the final tree; do not delete the whole mixed directory or remove administration solely by name. This audit neither removes anything nor asks to rewrite history.

### F02 — Surviving · High — The newer vendor corpus and imported code exceed the old rights boundary

**Evidence:** `.agents/skills/agent-platform-reference/README.md:5` identifies vendor article mirrors, reading views and raw responses; `.agents/skills/agent-platform-reference/VERIFICATION.md:7` reports 638 acquired sources, and `.agents/skills/agent-platform-reference/README.md:35` deliberately retains them for offline portability. All 638 records in `.agents/skills/agent-platform-reference/corpus/manifest.jsonl:1` onward have provenance/hash fields but no license/permission/rights fields. Imported Codex source excerpts have upstream commit/path provenance, e.g. `.agents/skills/agent-platform-reference/evidence/model-catalog-source/provenance.json:3`–`10`. The packager's blacklist at `scripts/build-release.py:68`–`77` does not cover this package; afterward it includes every tracked path. `LICENSE.md:14`–`23` expressly excludes automatic relicensing of third-party material.

**Proposed fix/disposition:** Add a source-class/per-path rights ledger for corpus articles, raw responses, derived views and copied code excerpts. Establish applicable terms and preserve required upstream copyright/license/NOTICE material for items selected for distribution. Exclude or replace unclear vendor text with source metadata and original project notes, while retaining the useful project-authored skill, curation, demonstrations and negative evidence. If exclusions break offline corpus validation, document the reduced public capability and retrieval route rather than promise complete offline portability. Whether any particular upstream permission suffices remains unassessed; this is a clearance gap, not a legal infringement conclusion.

### F03 — Surviving · Medium — Privacy and disposable-output selection lacks a completed path-level review

**Evidence:** `meta-topics/thesis-submission/archive-rights-and-exclusions.md:25`–`27` leaves focused privacy/quotation review of AI-use reports open; `experiments/ai-use/README.md:77`–`85` distinguishes ordinary metadata from content/scale risks. `scripts/build-release.py:79`–`87` includes all tracked paths, with no reviewed privacy/disposition manifest. A concrete retention candidate is the six tracked files under `experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/smoke/`: `experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/smoke/run-provenance.json:2`–`14` records `--smoke`, and the producer's `experiments/dev-gradient-ascent/literal-naive-gradient/README.md:142`–`148` gives its cheap regeneration route. `.gitignore:69`–`72` does not untrack files already in Git.

**Proposed fix/disposition:** Produce a reviewed include/exclude/disposition ledger with rationale and owner for mixed reports, retained excerpts and generated outputs. Inspect report quotations/content before public inclusion, without automatically censoring names or paths. For the named smoke packet, check actual consumers: retain it with a regression/interpretive purpose or exclude its generated outputs while preserving producer/configuration and any useful finding. Extend the release checker to enforce the reviewed decisions. No secret, private correspondence or raw-session leak was demonstrated by this bounded inventory.

### F04 — Surviving · High — ZIP construction loses continuation-critical links and executable modes

**Evidence:** `scripts/build-release.py:28`–`31` obtains names without Git modes, `:80`–`87` reads raw Git blobs, and `:156`–`158` writes each as an ordinary ZIP member without mode/link handling. The index has **14 symlinks**: 13 `.claude/skills/*` discovery links and `docs/coordination/current.json`. For example, `git show HEAD:docs/coordination/current.json` is the literal text `../../meta-topics/project-management/current.json`; those bytes would become a regular file, not JSON or a usable link. The intended skill-link contract is `.agents/skills/README.md:19`. There are also **51 tracked executable files** whose executable modes are not captured. `verify_zip` at `scripts/build-release.py:38`–`47` checks only names/bytes, so these structural failures would pass its checks.

**Proposed fix/disposition:** Enumerate committed object types and modes. Choose a documented portable policy: safely preserve internal symlinks with a supported extraction route, or expand them to usable regular files/directories with origin recorded; never store link-target text as the intended consumer content. Preserve executable modes or provide an explicit supported restoration step. Include types/modes/link targets in verification, then test Claude discovery, coordination JSON loading and a direct executable command after extraction without `.git`.

### F05 — Surviving · Medium — Empty-cache, clean-environment and extracted-bundle checks remain open

**Evidence:** `meta-topics/thesis-submission/archive-closure-checklist.md:36`–`38` requires empty-cache materialization, `:45` a clean documented environment, and `:50`–`51` an extracted archive inventory/hash check. `scripts/build-release.py:93`–`100` passes no explicit fresh cache root; `scripts/artifacts.py:288`–`299` can reuse an existing verified payload cache (it still fetches the remote manifest). `scripts/build-release.py:163` reopens the ZIP but never extracts it. `docs/reproducibility.md:42`–`57` accurately limits the prior environment check to a repaired existing image and frozen older candidate.

**Proposed fix/disposition:** In a later authorized closure run, record a fresh cache root via the supported cache override, materialize all 12 selected release snapshots, and retain download/inventory/hash receipts. Build the selected source in the documented clean environment with version drift recorded. Extract the reviewed ZIP outside any Git checkout and verify payload bytes, structure and supported entry points. Do not count this audit's successful warm-cache hashes as any of those tests, and do not repeat expensive scientific computations merely to test storage.

### F06 — Surviving · Medium — Documented figure reproduction assumes a Git checkout and private cache

**Evidence:** `docs/figure-reproduction.md:51`–`60` resolves the table through `verify-cache` or authenticated `materialize`; `scripts/artifacts.py:342`–`346` requires a cache manifest. The release exports the ordinary table at its registry target (`scripts/build-release.py:107`–`125`), without creating that cache layout. Later the figure staging recipe invokes `git ls-files` at `docs/figure-reproduction.md:189`, although the accepted extracted bundle has no `.git`. The checked-in thesis itself uses retained figure copies (`docs/figure-reproduction.md:159`–`163`), so this is a figure-regeneration route gap, not evidence that the ordinary thesis build is broken.

**Proposed fix/disposition:** Add an archive-specific entry point that verifies and reads the published table directly at `topics/empirical-studies/polytope-invariant-table/polytope-table.jsonl`, and stages thesis inputs using the archive manifest instead of Git. Alternatively provide a documented local cache-seeding route from the bundled payloads. Test those instructions with an empty user cache, no R2 credentials and no `.git`; distinguish dependency installation/network needs from access to already bundled research data.

### F07 — Surviving question · Medium — Explicitly decide continuation value of the sole excluded snapshot

**Evidence:** `artifacts/registry.json:23`–`48` registers the recovered covariance-rho pre-target packet but sets `release:false`. The producer documents recovered hashes and durable R2 materialization at `topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md:80`–`96`. Its compact result's `topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/covariance-rho-frozen-validation/README.md:43`–`49` says large caches are unnecessary for interpreting/verifying the compact result but useful for row-level pre-target inspection/full regeneration. Accepted retention includes continuation without first repeating expensive computation (`meta-topics/thesis-submission/archive-closure-checklist.md:25`–`35`).

**Proposed fix/question:** Keep the existing immutable R2 snapshot. At closure, decide whether independent row-level inspection belongs in the public continuation bundle; if yes, include the reviewed packet (726,467,720 bytes) or a deliberately selected sufficient subset with provenance. If no, record the reason for `release:false`, the retained compact files and the dependency on authenticated R2 for deeper continuation. Do not infer that a cache is disposable from its directory name or that omission invalidates the compact scientific result.

### F08 — Surviving · High — Working-PDF identity exists; a final reviewed release identity does not follow

**Evidence:** `docs/history/selected-partner-navigation-20261022/manifest.json:2` records only the abbreviated source label `6b02c04f`, `:3` the matched 105-page PDF hash, and `:10` limits review to changed introduction/roadmap/conclusion pages, excluding whole-PDF PASS/human acceptance. All 52 named inputs and the existing PDF match that manifest in this audit. `meta-topics/project-management/thesis-work.md:375`–`384` keeps the exact final candidate and human threshold unresolved. `meta-topics/thesis-submission/archive-closure-checklist.md:46`–`49` requires the exact release commit/PDF and an independently reviewed, externally recorded full SHA. The builder compares a literal SHA at `scripts/build-release.py:57`–`61`, but neither verifies an independent approval nor binds to a previously human-reviewed PDF hash; it rebuilds at `:127`–`140`.

**Proposed fix/disposition:** Freeze the actual closure candidate after selected changes; record its full 40-character commit, all selected source/input hashes, exact PDF hash, review scopes and human verdict outside that commit's self-description. Build from an isolated copy of those committed inputs. Compare the resulting PDF to the reviewed PDF; if bytes differ, record a justified comparison and renewed review/rebind rather than silently replace the reviewed artifact. Supply that literal approved SHA to the existing guard. Current unrelated dirty files are not cleanup authority and must be dispositioned by their owner before a clean release run.

### F09 — Surviving · Medium — Figure/font and dependency-notice clearance is still a distinct closure obligation

**Evidence:** `meta-topics/thesis-submission/archive-closure-checklist.md:17` and `:52` leave notice retention and final figure/font/rights review unchecked. `docs/figure-reproduction.md:25`–`39` supplies useful producer provenance for the thirteen external graphics; provenance is not a completed rights/notice review. The PDF uses installed `lmodern` (`thesis/main.tex:10`–`11`), and the visualizer loads third-party scripts remotely (`experiments/visualization/viewer/index.html:224`–`226`). No tracked font files were found by extension, which does not settle rights for embedded PDF fonts or any copied assets.

**Proposed fix/disposition:** Add per-selected-asset origin/producer, author/rights holder, applicable terms, attribution and clearance status; include the newly selected chapter's native drawing as appropriate. Inspect the frozen PDF's embedded fonts and third-party visual content, and retain applicable upstream notices for copied code/assets. Distinguish externally fetched runtime dependencies from redistributed bytes; do not add notices for every Cargo dependency merely because it appears in a lockfile. No specific font/figure violation is asserted here.

### F10 — Surviving · Medium — Citation, mixed-license metadata and archive-facing identity packet are unfinished

**Evidence:** `meta-topics/thesis-submission/archive-closure-checklist.md:53`–`54` defers stable `CITATION.cff`/Zenodo metadata. No tracked `CITATION.cff`, `.zenodo.json` or dedicated release-metadata file was found in the full path inventory. `meta-topics/thesis-submission/archive-rights-and-exclusions.md:49`–`54` requires honest mixed-license metadata, and `:104`–`105` calls for archive README, environment identity, commit identity and checksums. The builder's generated manifest (`scripts/build-release.py:145`–`149`) contains only schema version, source commit and file records; general repository documentation exists but there is no assembled archive-specific identity/environment packet in this flow.

**Proposed fix/disposition:** Once final title/authorship/identifiers and payload are stable, prepare citation and Zenodo metadata with the accepted Apache-2.0/CC-BY-4.0 material-type distinction, third-party exceptions, related repository URL, exact commit/PDF/payload identities and reproducibility limitations. Add a short archive README pointing to supported no-Git workflows and a version/dependency receipt for the closure build. Keep reserved identifiers distinct from published ones and do not fabricate a DOI. Whether any final software/data object has independent citation use is a question only if such an object actually emerges; default remains one record (`docs/project-facts.md:68`–`81`).

### F11 — Surviving external-status question · Informational — Publication is neither established nor selected by this audit

**Evidence:** `docs/project-facts.md:76`–`81` records account creation and unchecked record/DOI/upload/publication actions; it prohibits claiming publication without external confirmation. `thesis/chapters/14-code-data.tex:112`–`116` makes no frozen-release/Zenodo/DOI claim. `meta-topics/thesis-submission/archive-closure-checklist.md:55`–`56` assigns final draft review and exterior publication to Jörn.

**Proposed fix/question:** At the future publication handoff, establish whether any draft/reserved DOI already exists; inspect that actual record, reconcile its uploaded files/metadata with the reviewed manifest, and have Jörn perform the final publish action. Then record the DOI, record/version URLs and final identities. This repository-only review cannot establish present external account state and creates no request to publish now. Submission status and current university requirements are also unknown here; they are separate from archive selection.

### F12 — Surviving · Low — Release guidance still contains stale paths and conflicting preliminary wording

**Evidence:** `LICENSE.md:17`–`19` names former `papers/` and `submit/`, and `:34` points to a nonexistent `submit/archive-rights-and-exclusions.md`; `docs/project-facts.md:80` similarly points to the old checklist location. `meta-topics/thesis-submission/archive-rights-and-exclusions.md:109` says Software Heritage preserves history, while current accepted policy at `docs/project-facts.md:71`–`74` makes it optional and normally omitted. The older rights audit's `:114` broadly excludes submission administration, whereas `LICENSE.md:27`–`30` and confirmed facts `:60`–`65` permit useful ordinary administration after review. `docs/dashboard/thesis-sources.json:4`–`5` visibly remains unreconciled and still describes a 102-page PDF/absent symmetric-partner chapter, despite the current 105-page manifest cited in F08.

**Proposed fix/disposition:** Correct canonical links and mark preliminary/historical exclusion language as superseded by the accepted content-based boundary. Say `.git` is excluded and history is available at the identified repository commit, without promising Software Heritage ingestion. Update the dashboard's current working-status description while retaining `reconciled:false`; refresh accepted source/PDF digests only after their required reconciliation and PASS review. Preserve historical manifests rather than rewriting their dates or scope.

### F13 — Solved · Informational — The broad “missing root licenses” concern is already addressed

**Evidence:** `LICENSE.md:3`–`12` implements the accepted material-type split and points to the present `LICENSES/Apache-2.0.txt:1` and `LICENSES/CC-BY-4.0.txt:1` texts. `meta-topics/thesis-submission/archive-rights-and-exclusions.md:10`–`15` records implementation and the deliberate mixed-package publication boundary.

**Proposed fix/disposition:** Retain these files and the limited grant; correct the stale paths under F12. Do not reopen the accepted license choice or assign one undifferentiated license to the whole archive. This solved prerequisite does not clear third-party notices or the vendor corpus.

### F14 — Solved · Informational — Registered local data is hash-verifiable and completely mapped

**Evidence:** All 13 entries in `artifacts/registry.json:3`–`157` passed the read-only verifier implemented at `scripts/artifacts.py:331`–`355`; its name/size/hash checks are at `:193`–`210`. Every cached manifest payload has exactly one registry source mapping; no registered archive target collides with a tracked path. Counts and bytes are recorded above. The pentagon landscape cache verifies despite the missing consumer link.

**Proposed fix/disposition:** Preserve current immutable identities. Install a missing consumer link only in a later authorized materialization run if needed; do not rerun the scientific producer or republish unchanged data. Remote accessibility, cold-cache transfer and correct ZIP extraction remain F05, and public no-cache consumption remains F06.

### F15 — Withdrawn · Informational — Broad exclusion of harness/context/history is contrary to the accepted outcome

**Evidence:** `docs/project-facts.md:56`–`66` explicitly includes useful harness and ordinary context, while `:71`–`74` makes Software Heritage optional; `meta-topics/thesis-submission/archive-closure-checklist.md:18`–`21` explicitly rejects rewriting Git history merely for this snapshot. The compact covariance packet independently explains its interpretation/verification sufficiency at `topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/covariance-rho-frozen-validation/README.md:43`–`49`.

**Proposed fix/disposition:** Withdraw any blanket recommendation to drop `.agents/`, coordination records, local-path references or all historical research; any claim that the omitted covariance caches invalidate the compact result; and any mandatory history rewrite/Software Heritage ingestion. Apply the narrower rights/content and continuation-value decisions in F01–F07 instead. No further action is justified by these broader suspicions.

## Questions retained for the eventual closure owner

1. Does independent row-level pre-target inspection justify including the 726 MB covariance-rho snapshot, or should authenticated R2 remain its explicitly documented continuation route? (F07)
2. Which copied vendor documentation/code can be redistributed under evidenced terms, and which must be represented only by original notes and retrieval metadata? (F02)
3. What exact full commit/PDF will receive the final review, and what human verdict actually applies to it? The 105-page working identity is available, but is not final PASS. (F08)
4. At closure, which final citation identifiers and actual Zenodo draft/DOI state apply, and are downloaded forms still administratively needed? These questions do not authorize submission/publication or assume that submission is outstanding. (F01, F10–F11)

No deletions, packaging or publication are recommended for immediate execution by this inventory.

[review: archive_review]
