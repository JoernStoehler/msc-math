# Writing-quality evidence datasets

Usable historical reading inputs and separate feedback for developing future writing detectors and editors. Setup does not assess detector capability or authorize thesis rewriting. The local bundle is committed in ordinary repository storage; no runtime dependency on `.git/codex` or session logs remains. External official releases are reproducibly fetched into ignored caches.

## Quick use

From repository root, Python 3 and Poppler (`pdftotext`, `pdfinfo`) on PATH:

```bash
python3 experiments/writing-quality/dataset.py inventory
python3 experiments/writing-quality/dataset.py validate --reextract
python3 experiments/writing-quality/test_dataset.py
python3 experiments/writing-quality/dataset.py export --cases c001,c002 --output /tmp/writing-inputs
python3 experiments/writing-quality/dataset.py partition --holdout c001
python3 experiments/writing-quality/external/prepare.py
```

The export directory must not exist. It contains only input PDFs/text/page maps, a neutral-ID index and reading cautions. Hand **that directory alone** to a fresh evaluator, not this README, the case manifest, labels, provenance, or research reports. Neutral filenames remove obvious label leakage but do not erase subject identity, intrinsic PDF metadata, prior model knowledge, or earlier root exposure. Do not call this a contamination-free holdout.

`partition` conservatively excludes every case sharing any lineage cluster with the requested holdout. All these local cases share the September thesis project/feedback history; consequently a c001 holdout leaves no independent local training set. This is an intentional warning, not a reason to randomly split sentences. A topic-specific comparison can still be designed explicitly as a transfer experiment with disclosed shared history.

## Local inventory

| ID | Immutable input | Physical pages | Human evidence |
|---|---|---:|---|
| c001 | Standalone HKO chapter | 7 | Seven localized objections and completed chapter PASS |
| c002 | DS prose sample | 2 | Comfortable prose, rejected narrative, requested structure; later openings excluded |
| c003 | 17:00 thesis snapshot | 81 | Selective abstract/introduction feedback |
| c004 | 17:30 thesis snapshot | 86 | Rejected preliminaries, Chapter 4 borderline PASS, attribution correction, withdrawn objection |
| c005 | Frozen thesis checkpoint | 86 | Reported whole-thesis FAIL; separate AI self/Pro reviews |
| c006 | Four quoted pentagon fragments | — | Qualified records only; original served PDF was overwritten |
| c007 | Pentagon replacement v2 | 4 | No human writing judgment established; post-feedback AI review only |

There are **37 bounded human observations**, not 37 independent binary grades. Human judgments were recorded by agents; original message logs were not recovered. `direct_human_quoted` means a verbatim quotation explicitly attributed to Jörn in the retained record, not independent message-log verification. All writing was intended to be good under constraints, not synthetically corrupted; exact prompts, model versions, generation budgets and complete upstream authorship chains remain partly unknown. Selection reflects actual project review and retrospective investigations, not a random population sample.

The whole-thesis PDFs overlap the standalone chapters and each other. c003 → c004 → c005 is a revision chain conditioned on online feedback. c007 follows feedback on overwritten pentagon material represented only partially by c006. Do not silently pool these as independent observations.

## Files and schema

- `local/inputs/`: original PDF bytes, UTF-8 layout text and JSONL page maps. c006 has only quoted text.
- `local/cases.jsonl`: source path/hash, context scope, lineage clusters, intended quality, selection mechanism, authorship transformation chain, explicit unknowns and links to preserved records.
- `local/labels.jsonl`: bounded human observations with exact source quotations, scope, dimension, judgment, attribution and active/withdrawn status. Vocabulary preserves nuance rather than coercing everything to PASS/FAIL.
- `local/reviews.jsonl`: separate provenance index for human-feedback records and AI predictions/audits, with timing/scope caveats.
- `local/evidence/`: byte-preserved review records including AI material. Shared records sometimes discuss multiple revisions; label-level scope determines applicability.
- `local/provenance/`: byte-preserved source maps, selected authoring sources and recovery records. Original hidden-source files are stored under `provenance/hidden-codex/`; source-file-map.json records their original paths. Selected TeX is not promised to rebuild every PDF; immutable PDFs are the reviewed inputs.
- `local/anchors.jsonl`: **candidate** literal quoted locations, with offsets and physical PDF pages. Matching normalizes whitespace and Unicode compatibility forms. Quoted suggested repairs can also match; these are not human highlighter spans. Unlocated comments retain their source evidence and scope.
- `local/manifest.json`: every local data/evidence file's SHA-256 and byte size, extraction tool/version and offset conventions.

Page numbers are physical 1-based PDF indices, not necessarily printed thesis page numbers. Character offsets are zero-based Unicode code-point intervals `[start_char,end_char)` into the corresponding UTF-8 text decoded as text. Page texts are joined with a single form-feed character. Line breaks and formula layout are preserved as emitted by `pdftotext -layout`; formulas can still be garbled, so PDFs remain authoritative. No extraction error is itself labeled a writing defect.

PASS permits imperfections. An unmentioned span is unknown, not clean. Proof approval is not exposition approval. AI review is not human ground truth. A withdrawn objection is retained but not an active defect. Pro findings remain AI findings; a human providing that report does not adjudicate its contents.

## Reproduction and verification

A normal checkout already contains all local originals. Rebuild derived text/page maps with `dataset.py extract`, then anchors with `anchor_labels.py`. Validate against the retained manifest **before** resealing. Different Poppler versions can change extraction bytes; investigate changes rather than silently accepting a new manifest. `dataset.py seal` is a maintenance operation for intentional reviewed dataset changes, not an integrity check.

Initial setup compared the five historically recorded PDF hashes with their actual sources; the new c007 source hash is recorded. All review/provenance files were copied without modification. Extraction page counts equal `pdfinfo`; a second full extraction matches saved page text. Every bounded feedback quote is an exact substring of its preserved source. Render checks compared c001 physical page 3, c002 page 1 and c004 page 7 with extracted prose and candidate locations. They preserve the expected rejected passages and surrounding context; this is sampling, not a claim that every extracted mathematical glyph is correct.

Tests additionally detect deliberately corrupted source bytes, enforce input-only exports and reject silent shared-lineage partitioning. External preparation checks release hashes, native schema assumptions and source/target joins, and reproduces its normalized manifest offline; see [external README](external/README.md) for licenses, sizes and exact units.

To inspect small records without importing libraries:

```bash
head -n 1 experiments/writing-quality/local/labels.jsonl
head -n 1 experiments/writing-quality/local/anchors.jsonl
head -n 1 experiments/writing-quality/external/normalized/aries-test.jsonl
```

These examples contain labels; do not include them in a fresh evaluator's input packet. No experiment prompts, evaluation scores, statistical reliability claims, or authoring changes are part of this setup.
