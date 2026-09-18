# Human comparison across fresh microtests

These are purposively selected passages and contextual judgments, not an exhaustive labeled test set. Predictions were frozen before requests except the explicitly secondary qualifications/C overlap. Human saw some progress summaries; the trial is not blinded. Exact responses and qualifications live under [human/](../human/). Detection, diagnosis, suggested repair, and local acceptance are separate outcomes.

| ID | Human judgment | Frozen review relation |
| --- | --- | --- |
| A | Rotation observation useful | Manual KEEP agrees |
| B | Initial snippet uncertain/problematic; full paragraph PASS and high content value | Baseline/relevance endorsement agrees with full-context judgment; not a confirmed miss |
| C | Hard to read; unmotivated prespecification and significance-test denial | Manual PROBLEM agrees; narrow reviewer catches denial before answer but after request; does not catch every objection |
| D | Cannot evaluate references without context | Unscored |
| F | Seed detail shows poor reader model | Classifier identifies exact reproduction-detail burden; open reviews miss it |
| J | Excess control mechanics; concise relevant protection from standard error could help | All locate concern, but mainly seek more explanation; only partial diagnosis match, repair agreement unproven |
| K | Observation useful; “designed” adds little and “step” unclear | Relevance has partial description overlap; classifier KEEP misses local wording defects; neither fully matches |
| L | Keep definition clarification | Baseline and classifier retained correctly |
| M | Remove unused code label | Reproduction-detail specialist autonomously flags exact span and matching remedy; baseline broader access concern |
| N | Remove unraised prospective-prediction alternative; preceding clause also slop | Qualifications specialist flags exact clause but proposes retaining unclear preceding clause; detection useful, repair incomplete |
| O | Fix repository reproducibility/evidence base | Fixed-text retention judgments do not address the preferred upstream action; not a binary clean/defective label |
| P | Keep: useful algorithm comparison and plausible quantifier reminder | Confirmed false alarm from qualification specialist (likely concern) |
| Q | Keep singularity clarification | Confirmed false alarm from qualification specialist (likely concern) |
| R | Need more context | Unscored uncertain qualification flag |
| S | Keep proof clarification | Baseline and qualification specialist retain correctly |

## What this supports

- **Narrow concern generation:** unused implementation names and unraised negative alternatives can be detected in an alternate draft without supplying its human answers. M and N are concrete human-confirmed findings; their shared scientific lineage limits generalization.
- **Candidate-conditioned classification:** F shows useful judgment on a supplied passage. This alone does not establish autonomous discovery.
- **Context-aware adjudication:** B demonstrates an actual reversal from uncertainty to PASS when the real paragraph is supplied. D remains unjudged; do not fill it in from agent agreement.
- **Limited repair competence:** J and N show that locating a problem does not establish the right repair.
- **Upstream ownership:** O leads to the separate [provenance issue report](../issue-reports/historical-ds-provenance.md), sent to main.

## What it does not support

No general accuracy, precision/recall, whole-text acceptance, robustness across models/runs, or automatic-repair claim. Broad historical checks exhibit many misses, including explicit approval of a human-rejected qualification. A specialist's silence is not a clean bill of health. P and Q establish two confirmed false alarms in proof prose. Absence of corresponding historical labels elsewhere remains non-evidence of correctness.

## Final recommendation

Retain reproduction-detail review as a candidate for a bounded concern-generating pass, with exact spans and source context. It autonomously found M after development on related code/reproduction detail, and the human agreed with the specific removal. One such case does not establish reliable savings or safe automatic removal. The qualification reviewer also remains a plausible human-triaged flagger: its useful findings, false-alarm burden, unresolved flags, and incomplete repair need a net-benefit comparison. Two false alarms alone do not justify rejecting it. See [surface and utility accounting](qualification-utility.md). Broad review is not an acceptance gate.
