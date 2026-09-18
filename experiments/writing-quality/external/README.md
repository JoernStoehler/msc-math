# External writing-feedback datasets

This directory prepares official releases for inspection and later detector/repair experiments. It does **not** run or validate a detector, assign thesis PASS labels, or impose a common quality score across unlike annotation tasks.

From repository root, with Python 3.11+ (standard library only):

```bash
python experiments/writing-quality/external/prepare.py
python experiments/writing-quality/external/prepare.py --offline
```

The first command fetches missing files and verifies SHA-256 hashes against `sources.lock.json`; the second verifies and rebuilds using only cached bytes. Downloads go through temporary `.part` files and a mismatched upstream payload is rejected. README snapshots are pinned to repository commits; SNaC's Drive file and ARIES's public S3 objects are pinned by hash because their download locations are mutable. Source archives remain untouched. No authentication, model invocation, package installation, AWS CLI or Drive connector is required.

All acquired text and derived text remain ignored in `cache/` and `normalized/`. Only preparation code, metadata, and documentation are committed. About 142 MB are downloaded and 34 MB of normalized output produced. The StoryFeedback archive contains about 148 MB of generated feedback uncompressed; the script streams it for a count and does not expand it onto disk. ARIES full text is read directly from its 94 MB compressed archive; only the 84 test-set paper versions enter normalized output. `normalized/manifest.json` records exact output counts, sizes and hashes. `expected-manifest.json` is the verified reference from setup, not a quality evaluation.

## Releases and actual contents

| Source | Prepared units | Annotation meaning | License evidence |
|---|---|---|---|
| [SCARECROW](https://github.com/yao-dou/scarecrow) | 1,308 news paragraphs; 13,056 rater responses; 41,862 marks | Natural model continuations plus human comparisons, independent human error categories/explanations/severity and native offsets | No explicit dataset license in this inspected repository; cache only, no redistribution assumption |
| [SNaC](https://github.com/tagoyal/snac) | 150 summaries; 2,466 source chunks; 5,645 aggregated error marks with 8,785 total votes | Human-identified natural coherence errors on model book/movie summaries | No explicit license in inspected repository; cache only |
| [StoryFeedback / igen](https://github.com/google-deepmind/igen) | 1,920 human-rated feedback records normalized; all 83,456 generated feedback records retained in archive | Model critiques of original or synthetically corrupted short stories, rated by humans; not direct human PASS labels on stories | README: nonsoftware materials CC-BY 4.0; software Apache 2.0 |
| [ARIES](https://github.com/allenai/aries) | 196 manual-test comment/alignment records across 42 papers, 84 full-text versions; 196 GPT edit records separately | Human review → human revision alignments; model revisions and their annotations kept separately | README: dataset ODC-BY 1.0; code Apache 2.0. Dataset LICENSE retained |

SNaC release chunks can contain multiple sentences. Their counts need not equal paper-reported sentence counts. The release's aggregated marks/votes also do not reproduce the README's rounded 9.6k annotation total; no reconciliation is invented. Read the units above literally.

## Normalized files and semantics

Each JSONL record has an `id`; related units have a `group_id` where available. Most records retain a `raw` object to avoid discarding source fields. This is a small wrapper schema, **not** a shared annotation ontology.

- `scarecrow.jsonl`: `text` is generation, `context` is the prompt. `responses` parses the source's Python-literal list using `ast.literal_eval`, preserving each rater's list (including empty lists). Each six-item mark retains `[category, explanation, severity, start, end, antecedent]`. Offsets and antecedents are source-native; no reindexing, consensus, or missing-error inference. The raw CSV row retains the exact serialized response string. `group_id` hashes the prompt to group shared contexts.
- `snac.jsonl`: `text` is one original chunk; `context_chunks` contains preceding chunks in numeric key order. `raw.errors` preserves span text, category, and votes. There are no fabricated character offsets when the same span occurs twice. Summary/model identifier is retained in `id` and `group_id`.
- `storyfeedback-human-ratings.jsonl`: `text` is the critiqued story. `raw` includes original story, corruption, prompt condition, model feedback and aggregated human ratings. Upstream nonstandard `NaN` values become JSON `null`; raw bytes stay in the archive. A missing rating is not zero. Grouping is by seed `story_id`, so corruption and prompt variants stay together. The archive's generated-only critiques must not be mistaken for human-reviewed critiques.
- `aries-test.jsonl`: preserves `raw_label`, `raw_comment`, all candidate edit indices, paper IDs and positive edits resolved into actual source/target paragraph objects. `positive_edits` means comment–edit alignment, not that the edit improved writing. `aries-papers.jsonl` supplies full original S2ORC parses for context. Resolve paragraph indices against `pdf_parse.body_text + pdf_parse.back_matter`, matching the official ARIES loader. This concatenation is essential for correct joins.
- `aries-generated.jsonl`: keeps GPT-generated edit suggestions and any human analysis annotations separate from actual author edits. These suggestions are not verified scientific findings or implemented revisions.
- Raw `aries-alignment_human_eval.jsonl` contains the separately elicited human alignment evaluation; it is intentionally not merged with original test labels. Training/dev synthetic alignments and the large review/reply corpus are not fetched.

## Selection, provenance, and experimental boundaries

These are full releases or explicitly named source subsets, not cherry-picked good/bad examples. ARIES uses its official manual test set; StoryFeedback uses its official human-rated subset. Detailed budget histories and human/AI authorship of each source scientific paper are not established by these adapters. Do not silently mark those papers as known human-only.

No train/dev/test split is created. `group_id` is a useful minimum grouping, not proof against leakage: SNaC may summarize the same underlying work under different models, and related news prompts or related papers may also overlap. Check underlying identities before assigning splits. Since repository research and setup inspected examples, these releases are not untouched blind evaluation material for these agents. Human ratings and critiques are in the normalized records: strip/hide labels and source repair outcomes when constructing a blind input packet.

External genre/model differences are substantial. An empty error list does not certify writing quality; a natural human revision is not automatically better; a story corruption label is not a complete list of defects; human approval of a critique does not prove the proposed repair works. Dataset setup leaves those distinctions intact.

## Verification performed

All downloaded files hash-checked; all normalized records serialized as strict JSON; all 41,862 SCARECROW marks checked for six fields and in-range generation offsets; ARIES test comments, positive edit IDs, all needed paper versions and source/target paragraph indices joined successfully. Two offline runs produced identical output manifests. No detector scores or quality conclusions were generated.
