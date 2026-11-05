# Project-history removal — 17 October 2026

The intended end state is a thesis that presents mathematics, empirical methods,
results and present reproduction routes without narrating the project's history.
Historical receipts remain repository evidence; their absence is not a thesis
result. Mathematical and methodological caveats remain wherever removing them
would overstate a claim. The source edit cannot establish fresh reproduction or
human proof acceptance; build verification belongs to the integrated candidate.

The review started from `AGENTS.md`, `README.md`, `thesis/README.md` and
`docs/project-facts.md`, inspected the selected `main.tex` inputs, and searched
chapters, appendices and the disclosure for dates, historical/recovery language,
execution receipts, chronology, pending inclusion and provenance. Matching
passages and their surrounding sections were read; this was an editorial review,
not a new line-by-line mathematical audit. Present artifact membership was
checked against `artifacts/registry.json`; the method-facing table belongs to
`polytope-invariant-table`, separately from source geometries in `polytope-datasets`.

## Removed or rewritten passages

These entries identify every edited source passage for the final commit message.

| File | Removed passage or narrative | Replacement / retained support |
| --- | --- | --- |
| `thesis/chapters/06-variation.tex` | “historical heuristic numerical objective” and later “historical numerical objective” | Names the heuristic numerical objective directly; finite-step and endpoint limitations remain. |
| `thesis/chapters/08-data-science.tex` | “original” ratios and “reevaluation now”; “historical numerical objective” and “not every original executable or proposal trace was recovered” | Stored ratios and separate capacity checks; explicit distinction between heuristic comparison objective and certified capacity implementation; present source/trace availability is specified in appendix. |
| `thesis/chapters/10-affine-pentagons.tex` | “original exhaustive SageMath argument … computational provenance” | Available independent computational route; analytic proofs still do not depend on it. |
| `thesis/chapters/10-symmetric-partner.tex` | 3 October integration comment and inclusion-history paragraph | Personal proof/prose verification remains visibly pending, with the proof-source path. |
| `thesis/chapters/12-visualization.tex` | “were also explored”, “In practice”, “We found no visual pattern” and retrospective exploration outcome | Describes projection dependence and the lack of an established selection rule as present limitations. |
| `thesis/chapters/13-numerics.tex` | Original per-system audit-output absence phrased as historical provenance | States that the summary and producer are available; per-system output can be produced. Audit counts and finite-evidence caveats remain. |
| `thesis/chapters/14-code-data.tex`, opening/HKO paragraphs | 18 September check date, “subsequent research additions”, old-to-new source migration narrative, and appendix exclusion from the public commit | Fixed public source commit retained; accompanying-repository versus public-commit availability stated directly; both path conventions remain resolvable. |
| `thesis/chapters/14-code-data.tex`, pentagon/partner paragraphs | “original computational proof route”; inclusion and pending-review history path | Direct proof-source descriptions; computational versus analytic trust boundaries unchanged. |
| `thesis/chapters/14-code-data.tex`, numerical evidence paragraphs | Original per-system output contrasted with new execution; “historical” geometry join and first-order experiment | Describes available summary, producer, paired datasets and finite panel. |
| `thesis/chapters/14-code-data.tex`, dataset paragraph | “used during the project”, source-geometry recovery narrative, “this recovery does not supply missing historical execution receipts”, restored-feature timeline, historical numerical-ratio terminology | Present external-input requirements (separate `polytope-datasets` and `polytope-invariant-table` artifacts), matched source bodies, feature recomputation and 14,335 capacity intervals; stored numerical ratios still uncertified by capacity intervals alone. |
| `thesis/chapters/14-code-data.tex`, closing paragraph | Speculative later archival deposit, time-of-check history and later-local-additions handoff | Fixed public version and accompanying source/input requirements; no frozen release or DOI claimed. |
| `thesis/appendices/data-science.tex`, sampling paragraph | Recovered-cache seed/attempt narrative | Present stored-body reproduction route; unverifiable seed/attempt reproduction is not promised. |
| `thesis/appendices/data-science.tex`, evaluation subsection | Recovery/restoration timeline, initial 54/37 refusals and subsequent 90-body recovery narrative, original-execution certification caveat, “historical” ratio terminology | Current 14,289 narrow scalars + 46 wider intervals + one input-policy exception; exact rescaling method; direct comparison population and errors; stored numerical ratios remain distinct from capacity certificates. |
| `thesis/appendices/data-science.tex`, paired balancing paragraph | “original current evaluations” / “retained historical numerical ratios” | Names untransformed bodies and stored numerical ratios. |
| `thesis/appendices/data-science.tex`, optimizer introduction/budget | Claimed held-out label, later reuse of 16 starts and unresolved earlier selection history; “historical-evaluator” terminology | No independently held-out performance claim: start/configuration independence is unestablished. Numerical budget and outcomes retained. |
| `thesis/appendices/data-science.tex`, probe suite | “Incomplete historical source provenance” | Finite directions and broad capacity bounds still preclude certified local maximality. |
| `thesis/appendices/data-science.tex`, adaptive sampling | Stopped-first-version and supported-version chronology | Admission-conditioned implemented comparison; the protocol was chosen after an unsupported input, retained as a methodological caveat. |
| `thesis/appendices/data-science.tex`, evidence index | Original optimizer recovery failure and “Historical population” / “Restored features” labels | Present availability of fixed steps/full-evaluator traces, missing complete per-method executables/internal proposal traces, and neutral source labels. |
| `thesis/appendices/hko-certificate.tex` | Verifier migration wording and historical-public-commit disclaimer | States the accompanying repository's verifier and included appendix program directly. |
| `thesis/ai-use-disclosure.tex` | AI review chronology, earlier computational exploration, 18 September provenance date, 13 July proofread-manuscript account and subsequent-revision history | Specific ChatGPT contributions and agent review retained. HKO contribution facts moved here from the reflection's anecdote. Current personal-review incompleteness remains explicit. |
| `thesis/chapters/15-ai-reflection.tex`, opening/HKO subsection | Agents becoming more capable; shift of personal work; GPT-5 request/suggestion sequence; initial failed derivative approach; model's subsequent construction sequence | Nonchronological discussion of method discovery and the HKO proof obligations; limits of the anecdotal evidence are explicit. Factual HKO AI contribution is preserved in the disclosure. |
| `thesis/chapters/15-ai-reflection.tex`, task/exposition discussion | “Other tasks produced…”, increasingly important long conversations, increasingly needed intervention, prolonged later writing struggle and persistent retrospective difficulty | Personal methodological observations and practical criteria remain without a project-development timeline; training explanations remain hypotheses. |

## Boundaries and editorial choice

Three readings of the request were considered: remove only the receipt sentence;
remove all retrospective first-person material including the entire AI reflection;
or remove project chronology while preserving the selected AI methodology
reflection. The criteria were faithfulness to the new instruction, mathematical
honesty, truthful present reproducibility and preservation of explicitly selected
content. A date-count rubric alone would wrongly remove reproducibility seeds and
scientific citations; a “no first person” rubric would wrongly remove the selected
AI reflection. The third approach satisfies the criteria best. It respects
`docs/project-facts.md` items 8.8 and 45, which distinguish the factual disclosure
from the research-process reflection, while removing its development narrative.

Intentionally retained:

- Scientific/literature history and dates in citations; reproducibility seed IDs;
  geometric orbit recovery and algorithmic branch-history names.
- The fixed public commit and current source paths: readers need a resolvable
  source version rather than an implied publication of local additions.
- Stored exploratory values versus current interval-evaluator outputs, unchanged
  population membership, one unsupported input, floating-point volume limitations,
  unavailable per-method execution detail, and no universal reproduction promise.
- The post-observation admission change and posthoc Gaussian reference choice:
  these affect interpretation of empirical claims, rather than merely describing
  project history. The unknown independence of optimizer starts from tuning is
  retained without reconstructing the development timeline.
- Personal interpretations of AI use as the selected reflection topic, specific
  AI contribution provenance, and present author-review limitations. No factual
  claim of author acceptance was added.

No theorem statement, proof equation, experiment result, dataset membership or
numerical outcome was changed. No producer, certificate computation or experiment
was run for this editorial cleanup. Source checks: `git diff --check` passed;
search found no remaining manuscript project-history dates or historical-receipt
passages. Root integration owns the fresh PDF build and visual verification.
