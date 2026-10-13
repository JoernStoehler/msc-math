# Architecture comparison

## Decision

Use separate canonical corpus and curated references (B), preserving immutable raw snapshots and supplying optional generated correction-enriched views (C). The canonical content owners remain separate: snapshots preserve upstream response bytes; normalized articles are refreshable; corrections are hand maintained; views are disposable. Source code stays a tagged dependency rather than a bundled full Git checkout.

## Measured comparison

The measured corpus contains 636 articles: 19,673,784 normalized bytes, 34,923,615 raw snapshot bytes and 20,093,862 generated-view bytes, excluding metadata. Four groups have identical normalized content; IDs are retained because URLs/scopes differ. The final source count and exact byte totals are generated in `evidence/inventory.json`; the source-by-source discovery decisions are in `corpus/discovery.json`. The A/B/C retrieval experiment is in `evidence/architecture-evaluation.json`. It uses identical acquired article bodies for all layouts and fixed factual phrases independent of source IDs. It measures lexical availability and whether linked corrections occur in the same source reading file. It does not measure agent task success, semantic retrieval accuracy, or model correction-miss rates.

| Layout | Retrieval fidelity | Correction visibility | Refresh safety | Packaging |
| --- | --- | --- | --- | --- |
| A: all vendor material and curation under references/ | Same normalized source bodies; same lexical coverage | Curated register is available but separate from an article | Safe only with generated/vendor subdirectory ownership; otherwise easy to mix edits | Standard directory name; no special client guarantee that all files are loaded |
| B: corpus plus curated references | Same source bodies; manifest and `rg` locate them | Requires reader to consult register separately | Separate ownership protects corrections | Simple filesystem skill package; hosted upload limits remain surface specific |
| C: B plus immutable responses and generated reading views | View retains normalized article verbatim; raw response auditable | Linked source corrections appear with article; unsupported alias matches remain a risk | Rebuild views; never overwrite hand curation | Larger package; regenerate views for CLI or ship thin summaries for hosted surfaces |

The layout comparison does not establish a task-success winner. Independent A/B/C agents all missed two consequential caveats, and the downstream handoff used curated references rather than full reading views. B owns canonical maintenance; C is a convenience whose contextual visibility/regeneration properties are tested, with additional storage and generation cost. A and B retain equivalent source-text availability, so no retrieval superiority is claimed. `scripts/build_views.py --evaluate` reproduces the comparison after refresh.

## Consumer boundaries

Codex CLI and Claude Code can use local corpus search when filesystem tools are allowed. ChatGPT ordinary chat uses project sources or eligible installed Skills/plugins; a ZIP alone does not add shell tools. Claude chat/Cowork uses eligible project knowledge or Skills, not a CLI installation by implication. OpenAI API and Claude Agent SDK callers must explicitly mount files and expose retrieval tools. See `skill-deployment.md` for exact routes and documented limitations.

Large hosted bundles should use a thin entry point plus selected references, with the full corpus in an explicitly accessible store. Do not invent a portable size limit: check current client settings and documentation. Live-documentation lookup is the fallback for freshness, failed mirrors, unsupported scope or changed versions.

The original lexical comparison is retained as an availability check only. See `REVIEW.md` and `evidence/decision-evaluation/` for the decision-level negative results and revised conclusions. Final measured corpus sizes are in `evidence/inventory.json`; old totals above describe the initial 636-source snapshot.

## Consultation categories, 13 October 2026

Jörn proposed distinguishing documented product mechanisms from model psychology
and retaining a specialized harness-design trigger. The entry point and index
now separate technical consultation from model-behavior evidence and
intervention design. Existing references remain at stable paths; no wholesale
corpus move or second auto-triggered model-psychology skill was created.
Harness-engineering retains workflow-specific design/evaluation advice and
conditionally consults this package for mechanics. The instruction-design page
compares several guides and attributes Jörn's bootstrap/retrieval judgment.
This is a source-organization choice, not a behavioral success claim. A future
independent behavior-research consumer may justify a separate package; current
behavior evidence and experiments do not establish a mature general psychology
workflow. The agents.md format site's missing mirror is recorded as a specific
coverage gap. Broad corpus completeness remains unverified.

Jörn then qualified this as an unnatural document partition: loading mechanics
and behavioral effects are often discussed together. The routes are therefore
entry points for different questions, not disjoint subject silos. Claim-level
provenance and uncertainty remain the main distinction, including interactions
between formal message roles, material representation and model response.
