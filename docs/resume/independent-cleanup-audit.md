# Independent consolidation audit

Auditor: `/root/independent_damage_audit`; inspected final repair state
`be6dbf8269d658da0305e1b7dd57a99938de2741`, 18 September 2026.
This is acceptance of the six identified consolidation repairs below, not thesis
acceptance or a proof that no undiscovered project defect exists.

## Findings and independently checked repairs

1. **Historical integrity had been coupled to editable live files.** The first
   repair merely checked object existence; the next checked deleted branch heads.
   Both were rejected: all 38 original branch heads were outside current HEAD's
   ancestry. `193c384e` instead reads each retained path at ancestor checkpoint
   `a6bbfe94d988`, comparing original branch blob IDs and dirty-file SHA-256s.
   Historical evidence no longer requires current working copies. Independently
   ran a disposable Git fixture: baseline passed; changing a live evidence file
   and deleting its working dirty snapshot still passed; a wrong original blob,
   wrong dirty hash, missing historical path, and an existing but non-ancestor
   checkpoint each failed. On final HEAD, the real verifier passed 3,370 branch
   records, 287 dirty records, the frozen PDF and 51 current build inputs.

2. **Selecting manuscript inputs had removed useful authoring tools.**
   `thesis/lookup.sh`, `thesis/tests/lookup.sh`, and `thesis/label-map.py` are
   restored. Lookup's synthetic tests passed. An independent disposable fixture
   confirmed label-map finds a nested chapter's quote and that a failed selected
   build returns failure without overwriting the prior map from stale auxiliary
   output. Its build invocation is fail-fast and its help now names `build.sh`.

3. **File-link checks missed path-pattern consumers.** The status helper now
   includes `thesis/references/*.bib`. Independently extracted its actual
   `CHECK_PATHS` and applied them through Git: all four current bibliography
   files are included. This checks the identified regression, not all possible
   consumer semantics in the repository.

4. **Dated material had been mistaken for obsolete knowledge.**
   `docs/knowledge/README.md` now directly routes HKO interpretation, optimizer
   review reasoning and the tested interaction handoff, with explicit source
   dates and authority limits. The promoted optimizer route was corrected after
   this audit found that its claimed current table label no longer exists.
   Its current subsection does exist; the table is identified as inline.

5. **Byte preservation had been mistaken for usable historical navigation.**
   The broken historical memory index is retired with an exact recovery record;
   useful destinations are provided by the knowledge/history entry points.
   `docs/history/source-recovery/README.md` now names a pinned export for the
   original baseline verifier and distinguishes it from one-time migration
   programs requiring old `.git/codex` inputs. The worker checked 59 input hashes
   and program parsing. This auditor inspected that route and limitation but did
   not rerun the expensive historical build or independently repeat those 59
   comparisons. Current source recovery is not described as fresh execution.

6. **Fewer branches had been mistaken for substantive consolidation.**
   Superseded assignment/status material and 146 duplicate preservation snapshots
   are now Git-only, not ordinary search fulltexts. Necessary exact reviews,
   reviewed inputs, scientific support and interpretation remain accessible.
   `retired-findings.md` routes factual content from mixed planning packets to
   support owners rather than deleting it as mere planning. Architecture now
   describes the actual scientific-support packets in `docs/` instead of
   forbidding what the checkout contains. Independently verified all **229**
   retirement entries: each current path is absent, its recorded commit is an
   ancestor, and its recorded blob exists exactly at that commit/path. This
   protects recoverability; it is not an exhaustive semantic review of every
   sentence retired or retained.

## Current navigation coverage

After the final retirement, independently scanned **416 tracked Markdown files**
for local Markdown links outside fenced code: **398 links, zero unresolved**.
The scope includes current scientific packet READMEs, not only entry points.
Excluded immutable `docs/history/`, source-copy/run/packet directories, and the
writing-quality local provenance copies; these preserve historical bytes.
An initial wider scan exposed only three links in that frozen provenance copy
and a syntax-highlighting regex inside INSTALL.md, not current navigation.
This checks Markdown link destinations, not every bare path or semantic claim.

## Additional preservation evidence and remaining boundary

The earlier independent audit recomputed changed-path sets for all 38 branch
inventories: no changed paths were omitted. It checked every one of the 108 tar
members against its hash and verified the archive bundle's 43 exact heads and
complete history. It compared 29 knowledge/guidance files and five human-review
responses with their pre-layout bytes, and checked existence of all six recorded
session logs. These checks were performed independently of the coordinator's
reported verifier results.

The actual cleanup command enumerated both ignored and nonignored untracked
files, but excluded `target/` and `__pycache__` files. It retained only counts
for **3,606 excluded files**, not individual names/hashes. Their complete
rebuildability therefore remains unverified. No scientific-data erasure was
established by this audit; that is not equivalent to proving none occurred.

Historical frozen texts can retain old paths as evidence. They are not promises
of direct execution from the current layout. The two missing old research caches
and untested remote redownload remain disclosed in `external-dependencies.md`.
Preserved transcripts permit further reconstruction, but neither transcript
existence nor this audit establishes that every useful conversational lesson has
already been surfaced in current documentation.

The thesis is unfinished. These results do not establish mathematical validity,
readable exposition, human acceptance, or readiness for final grading.
