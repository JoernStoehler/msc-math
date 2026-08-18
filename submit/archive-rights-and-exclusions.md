# Archive Rights And Exclusions

Status: active archive-boundary audit, 2026-07-14. Jörn accepted the recommended
Apache-2.0 plus CC BY 4.0 outcome on 2026-07-14 while noting that he is not a
licensing expert. This records an informed project-outcome choice, not legal
advice, and does not authorize publication.

## Current findings

- The branch implements a root material-type scope notice and official copies
  of Apache-2.0 and CC BY 4.0. Rust packages are marked `publish = false` and
  deliberately omit Cargo's package-level `license` field because their source
  trees contain both Apache-licensed software and CC-licensed documentation or
  research material. A future crates.io release needs a narrower package
  boundary and matching package metadata.
- The tracked tree contains Jörn's thesis/research work, generated experiment
  data and figures, agent harness material, downloaded paper sources and
  figures under `papers/`, and downloaded official forms under `submit/`.
  These classes must not be covered by one undifferentiated license statement.
- Git reports almost all commits under Jörn's identities, plus commits named
  `Claude`. Commit attribution is provenance, not proof of copyright ownership.
- Bulk generated JSONL is selected in `artifacts/registry.json` rather than
  Git LFS. A closure bundle must materialize and verify registry entries marked
  for release; a GitHub source archive alone is not sufficient.
- `experiments/ai-use/` contains retained research reports, scripts, and
  figures, but its ignored artifacts and source session logs are not tracked.
  The retained reports still need focused privacy and quotation review.

## Accepted licensing outcome

Use a clear two-part permissive scheme:

1. **Apache License 2.0 for software.** Apply it to Rust, Python, SageMath,
   shell, and other executable source written for this project. Apache-2.0
   permits commercial and noncommercial use, modification, and redistribution,
   requires preservation of notices and marking changes, and includes an
   express patent grant.
2. **Creative Commons Attribution 4.0 International (CC BY 4.0) for Jörn-owned
   thesis, documentation, original figures, and project-generated datasets.**
   It permits copying and adaptation, including commercial use, with
   attribution. Version 4.0 also addresses database rights.

This best matches the accepted continuation outcome: future researchers may
reuse both implementation and research material, attribution remains required,
and there is no noncommercial or share-alike compatibility barrier. Creative
Commons itself recommends a software-specific license for code rather than a
CC license.

Use a root `LICENSES/` arrangement and short scope table, not per-file
boilerplate throughout the repository. The Zenodo record should describe both
licenses and point to the scope file; its landing-page license selector must
not imply that excluded or third-party material is relicensed. Cargo packages
remain unpublished until package contents can state their mixed or narrowed
license boundary honestly.

Official references:

- Apache-2.0 summary and application:
  <https://choosealicense.com/licenses/apache-2.0/>
- CC BY 4.0 permissions:
  <https://creativecommons.org/licenses/by/4.0/deed.en>
- Creative Commons FAQ on software and databases:
  <https://creativecommons.org/faq/>

## Other worthwhile options

### MIT for software plus CC BY 4.0 for research material

Nearly the same practical reuse outcome and slightly simpler software license
text. MIT requires retention of its copyright and license notice but has no
express patent grant. This is sound if minimal license machinery is more
valuable than Apache-2.0's explicit patent terms. For this repository, the
user-facing complexity difference is small once two scopes already exist, so
Apache-2.0 is preferred.

### Apache-2.0 for software, CC BY-NC 4.0 for research material

Preserves noncommercial academic continuation but blocks or creates ambiguity
for commercial research groups, consulting, commercial tooling, and some mixed
datasets. It provides little practical protection against unwanted use while
making continuation harder. This conflicts with the stated continuation goal.

### Keep all rights reserved and publish only for inspection

Lowest licensing effort, but it fails the continuation goal: researchers can
read and run what copyright exceptions and local law permit, yet cannot rely on
a clear right to adapt or redistribute improvements. This is not recommended.

Copyleft software licensing (GPL/AGPL) and CC BY-SA for content are defensible
when reciprocal openness is a primary value. They add compatibility and reuse
constraints without a stated project need, so they are not on the practical
frontier here.

## Preliminary archive boundary

### Include after normal review

- final thesis PDF and thesis-owned source/assets;
- root entry points and project facts needed to understand the closure state;
- `AGENTS.md`, repo-local `.agents/` skills, and `.codex/` project agents;
- `crates/`, `formal/`, and retained `experiments/` code, data, reports,
  figures, provenance, and owner-local instructions;
- reproducible environment definitions and repository scripts;
- archive README, payload manifest, dependency/environment identity, Git commit
  identity, and checksums.

### Exclude from the Zenodo continuation bundle

- `.git/` (the exact public commit and Software Heritage preserve history);
- `papers/`, including downloaded/cached article sources and article figures;
- downloaded official forms and their conversions under `submit/`;
- raw Codex/Claude session logs and ignored `experiments/ai-use/artifacts/`;
- credentials, `.env`, user configuration, editor state, local worktrees,
  caches, build directories, and submission administration;
- any later-discovered third-party asset without clear redistribution rights.

The archive may retain a small public-facing subset of `submit/` containing
only archive instructions and manifests, but not private/admin documents or
downloaded forms.

### Needs focused review before inclusion

- retained `experiments/ai-use/` reports for personal identifiers, local paths,
  private-content fragments, and quotations from model/provider output;
- legacy/imported algorithm notes whose origin is unclear;
- every thesis and experiment figure for project-local producer provenance;
- dependency locks and vendored material, if any, for their existing notices;
- repo-local agent instructions for any text copied from external proprietary
  sources rather than authored for this project.

## Remaining work after the license choice

1. Produce a path-level include/exclude manifest and automated checker.
2. Classify all retained non-code assets by producer and rights status.
3. Review the current Git history before asking Software Heritage to ingest it.
4. Check the implemented license files and scope notice, and prepare
   `CITATION.cff` once final citation metadata is stable.
5. Build an unlicensed dry-run closure bundle and verify extraction, LFS object
   identity, privacy exclusions, and a cold agent start.
