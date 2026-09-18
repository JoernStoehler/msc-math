# Outcome coverage beyond the proposed sessions

Source discovery on 2026-09-07, not an exhaustive audit or assignment list.
The [session portfolio](team-plan.md) selects useful initial work; it does not
represent everything needed for a good thesis. Findings below come from bounded
source inspection by four parallel discovery agents, with selected source reads
during integration. No scientific computation, PDF review or external source
verification was performed. Source locations and currentness may change.

## Whole-thesis composition and mathematical support

The previous portfolio foregrounded one theorem presentation and code work.
`docs/project-facts.md` scope item 8 and `thesis/main.tex` also include Reeb/HK
foundations, first-order perturbations, numerics, reproducibility, AI and useful
preliminaries. Item 9 rejects a forced tight narrative. The broader authoring
outcome includes reader prerequisites, consistent conventions and meaningful
connections across these contributions, not merely individually good sections.
Smallest useful inquiry: recover the intended roles and prerequisite relationships
for the first selected contribution; retain unresolved whole-thesis questions
without turning that into a compulsory global outline rewrite.

Excluding the long proof from an initial HKO presentation trial does not remove
eventual proof exposition/support. `thesis/12-published-code-data.tex` explicitly
separates Rust witness choices, Sage predicates and the mathematical implication;
the pentagon result likewise needs enumeration, sign-to-action and continuity
arguments. This is already-known substantive work omitted from the narrow session
view, not a newly discovered theorem defect. The relevant authoring owner should
preserve those boundaries while expanding beyond the initial unit.

## Data science: value-bearing content, not only limitations

The existing [scope note](datascience-scope.md) owns the known 14336-row frozen
table lineage gap and appendix evidence limits. New discovery found positive
subthreshold enrichment evidence worth comparing with the active account:

- `experiments/sys-datascience/methods/alternative-source-transfer/README.md`
  and `artifacts/transfer-v1/analysis.json` describe a prospectively frozen
  transfer packet with enrichment relative to comparison samples.
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/review.md`
  distinguishes enrichment from a successful threshold-exceeding proposer.
- `thesis/08-black-box-datascience.tex` already mentions mean improvement from
  low-ridge rules, alongside its 1675-row bounded negative search account.

This is not yet evidence of an unintended omission or Jörn's acceptance of a
particular presentation. Compare the strongest two or three retained packets
with the chapter's scientific account: useful output is identifying what the
results teach and how to explain it, not automatically expanding the chapter.

`thesis/06-first-order-perturbations.tex` describes a branch-window model lacking
reproducible finite-distance validation, while chapter 08 and the appendix retain
recovery/KKT diagnostics. These may concern distinct models. Trace definitions
and packet identities before treating this as a contradiction or transferring
support from one to the other.

## Architecture changes can alter scientific meaning

The [architecture findings](architecture-migration.md) already identify semantic
differences among live implementations. `thesis/11-numerics.tex` additionally
depends on rounding, gradual underflow and operation order, and distinguishes
candidate completeness, certified exclusion and fallback behavior. Compiler or
dependency changes can therefore matter beyond ordinary API compatibility.
Migration characterization should identify which scientific contracts a proposed
change touches, including input representation and facet/candidate ordering.
This belongs inside each migration, not a mandatory separate comprehensive audit.

`experiments/performance/README.md` and
`experiments/dev-quadratic-program/performance/README.md` prominently concern
older floating-point/fallback routes. Coverage of the current certified scalar
API and its resource limits was not established. Before using these reports to
choose an optimization or predict production cost, identify the evaluator and
workload they actually measure; no benchmark run is currently justified merely
by this uncertainty.

## Source study, annotations and maintained consumers

Concrete workflow opportunities surfaced beyond generic navigation cleanup:

- `papers/citation-index.md` tells agents to use it instead of re-searching
  sources, refers to old `crates/**/math.tex` consumers, and gives BBLM2023 authors
  inconsistent with the thesis bibliography. The author discrepancy is internal
  evidence of inconsistency, not an externally resolved citation. Recover the
  original source when using that citation; make the index support source study
  rather than substitute for it.
- `thesis/label-map.py --build` invokes `latexmk -f` without requiring success
  before reading auxiliary output. Its label filter also omits equation, figure,
  table, algorithm and corollary labels. The latter may be intentional for a
  section map, but a contextual annotation locator may need more. A synthetic
  failed-build/stale-aux fixture can establish the defect cheaply; actual review
  needs should determine any scope expansion.
- `experiments/dev-flow-graph/visualize-tube/README.md` describes regeneration
  without completing the handoff to the thesis copy. Selected flow-graph,
  hypercube and pentagon-landscape copies were byte-identical during discovery:
  the issue is future maintenance exposure, not demonstrated stale figures.
- `thesis/09-rotated-regular-polygons-sage-source.tex` is a manually adapted
  source excerpt. Its producer's certificate instructions did not expose the
  consumer update route; divergence was not established. A producer/consumer
  link can make future changes discoverable without requiring generated prose.

These are possible execution/authoring improvements, not grounds to create a
new permanent review-tools team. The first annotation cycle can reveal which
locator/context support is actually valuable.

## AI account and recoverable publication

`thesis/13-use-of-ai.tex` limits itself to one mutation/replay case and records an
August 28 revision not yet accepted as final prose. Older project-fact scope
describes broader AI reflection. Recover any later decision before proposing
expansion or asking Jörn to repeat it: old breadth is not a current requirement.

The replay has a concrete history dependency:
`experiments/ai-use/sign-replay/sign_replay.py:replay_case` creates a worktree at
`f3d36cc968716132af582282dbe6c137a2857ec4`. The snapshot archive produced by
`scripts/build-release.py` excludes `.git`; no frozen source fixture was found
in the replay packet. Thus that snapshot alone cannot supply the historical
checkout. A smallest closure inquiry is identifying the exact required source
and choosing a reviewed fixture or durable history route, not bundling all
mixed-rights Git history by default.

Longer-term public reproduction/archive delivery remains an outcome. Project
facts 35–39.2 discuss a roughly two-year reproduction target;
`thesis/12-published-code-data.tex` honestly disclaims complete plain-checkout
reproduction and a frozen DOI release. The archive checklist is a route, not
proof that public inputs or publication exist. Current deadline/status was not
established. Facts 7.1 (Kai feedback after Jörn is satisfied) and 75 (no fast
review arrangement) are dated context to recover when scheduling becomes
relevant, not urgent assignments.

## Retained uncertainties that do not justify new work by themselves

The changed TeX was rebuilt on September 11, with the selected checks recorded
in the task map. That is not evidence of full-manuscript quality or a reason by
itself to request Jörn's final review. The pentagon certificate discussion
and empirical README disclose that equal Rust/Sage word counts do not establish
equal families. A cheap word-set comparison could clarify their relationship if
a consumer needs it; this is not a demonstrated gap in the exact proof.

No new peer assignments follow from this note. Whole-thesis composition, long
proofs and final delivery remain visible while bounded session work proceeds;
new source-study, scientific-impact and feedback hooks can strengthen existing
owners rather than multiplying permanent teams.
