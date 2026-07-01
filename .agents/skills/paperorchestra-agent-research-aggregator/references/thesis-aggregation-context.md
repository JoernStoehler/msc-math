# Thesis Aggregation Context

Use this reference when the aggregator target is a thesis, monograph-like
manuscript, or multi-strand research project rather than a single conference
paper.

## Interface Versus Meaning

PaperOrchestra expects `workspace/inputs/idea.md` and
`workspace/inputs/experimental_log.md`. For a thesis run, those filenames are an
interface, not a semantic limit.

Do not force thesis material into the conference-paper shape
`Problem / Hypothesis / Method / Key Contributions / Raw Numeric Data` when that
would lose information. It is acceptable and preferred to create thesis-native
sidecar artifacts under `workspace/ara/` and then write adapter-style
`idea.md`/`experimental_log.md` from them.

Useful thesis-native artifacts include:

- `workspace/ara/thesis-context.md`: target, audience, known TOC, writing
  priorities, and claim-boundary rules.
- `workspace/ara/thesis-source-manifest.json`: repo files, reports, logs, and
  predecessor labels considered.
- `workspace/ara/thesis-strand-records.json`: extraction records grouped by
  thesis strand.
- `workspace/ara/thesis-strand-classification.md`: reviewable table of each
  strand, support level, sources, PaperOrchestra role, do-not-claim notes, and
  open gates.
- `workspace/ara/raw-log-spotcheck-delta.md`: compact deltas from targeted
  session-log spot checks.

## What To Collect

Collect both experiment evidence and project context. For this thesis repo, the
aggregator should inspect current repo sources, experiment reports, thesis
companions, source-truth maps, and targeted session logs. In particular, it
should look for:

- current thesis structure and section roles;
- proof-facing theorem results;
- executable proof or exact-certificate evidence;
- empirical and data-science evidence;
- verification, regression, and trust evidence;
- exposition/foundation material that exists to make the thesis self-contained;
- unfinished or background routes that should be mentioned only with caveats;
- AI-use disclosure and drafting constraints;
- reader priorities and advisor-facing expectations.

Do not decide what to foreground before collection. Explore broadly first; then
classify. Inclusion, demotion, or omission from the final adapter files is an
output of classification, not an input filter.

## Support Levels

Every extracted strand should carry a support level. Use ordinary words; do not
invent labels for their own sake. Typical levels are:

- theorem/proof in thesis text;
- executable proof or exact certificate;
- empirical/data-science evidence;
- verification or regression support;
- exposition or foundation;
- unfinished/background material;
- drafting/disclosure constraint.

For each strand, record:

- current claim or role;
- source files or log packet ids;
- evidence type and strength;
- what PaperOrchestra should do with it;
- what PaperOrchestra must not claim;
- open gates needing Jörn/Kai or source review.

## Known First-Trial Lessons

The first thesis aggregator attempt on 2026-07-01 produced reviewable inputs but
failed in ways future runs should avoid:

- It treated proof evidence, empirical search evidence, verification support,
  unfinished diagnostics, and AI-log coverage as comparable result tables.
- It used an overlong thesis-frame sentence as the H1/title, which would be bad
  material for downstream drafting.
- It elevated AI-use provenance to a key contribution. For this thesis, AI-use
  is disclosure/trust/drafting context, not a mathematical contribution.
- It described HKO from certificate internals outward. Thesis-facing text should
  state the scoped result first, then the certificate evidence.
- It surfaced flow-graph scratch-count diagnostics instead of the promoted
  verification packet and current caveats.
- It skipped targeted raw-log extraction without a clear value/cost decision.

## Current Flow-Graph Guidance

As of the 2026-07-01 review, the faithful flow-graph/CH2021 classification was:

- There is a real exact implementation surface with exact tube resolution,
  exhaustive search, f64 diagnostics, and word enumeration.
- There is promoted verification evidence, including exact FG capacity compared
  against certified HK/QP on selected generated cases, retained-word
  consistency, cutoff equality, and rejection rows.
- The current thesis section states a conditional flow-graph capacity reduction,
  not a broad finished theorem that the Rust implementation computes `c_EHZ`.
- The proof/writeup route remains caveated; formal/Rust correspondence, f64
  rounding, singular classifier scope, and final genericity are not fully
  discharged.
- Scratch F5/F7 frontier or singular-status counts are not thesis-facing result
  tables unless a later task promotes them with command, input path, commit, and
  interpretation.

Therefore, aggregate flow-graph as bounded implementation and verification
support with an unfinished proof/writeup route unless current sources have
changed.

## Targeted Raw-Log Spot Checks

For thesis-scale aggregation, broad raw-log extraction is usually too noisy, but
skipping logs entirely risks stale or wrong context. Use targeted spot checks
when they can change the PaperOrchestra inputs.

Good questions for log packets:

- Is a current claim missing an earlier precursor result or correction?
- Is theorem wording or support level represented too strongly?
- Did a user correction reject an agent-written framing?
- Is a strand currently finished, caveated, or abandoned?
- Does AI-use provenance constrain drafting without becoming a contribution?

Use subagents for log packets. The main aggregator should receive compact
deltas, not raw transcripts. A useful packet output schema is:

```json
{
  "packet": "<name>",
  "answer": "<what the spot check changes or confirms>",
  "input_change": "<specific change to thesis records or PaperOrchestra input>",
  "confidence": "high | medium | low",
  "source_ids": ["<short id, not full transcript dump>"],
  "do_not_claim": ["<unsafe inference>"],
  "open_question": "<if any>"
}
```

Keep raw paths, commands, and detailed evidence in `/tmp` if needed. Do not
commit raw transcript excerpts, broad per-session tables, API keys, credentials,
or unrelated personal material.

Rough value/cost estimate from the first trial: a targeted 10-15 packet
spot-check may cost about 6-10 Codex-agent hours plus 0.5-1 main-agent hour,
and can plausibly save 2-6 hours of Jörn review/rewrite time while reducing the
risk of a major scope or provenance overclaim. This is positive expected value
when the output will feed a real PaperOrchestra run.

