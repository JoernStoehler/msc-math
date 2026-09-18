# Historical comparison: c004 section 2 preliminaries

## Scope and audience

This compares the frozen [baseline](baseline-preliminaries.md) and [prerequisite specialist](prerequisite-transfer.md) against only c004 labels applying to section 2, using [the exact section 2 extraction](../inputs/preliminaries-transfer.txt). Both review hashes were verified against [preliminaries-freeze.json](preliminaries-freeze.json), frozen at `2026-09-18T12:31:45.189984+00:00`. No predictions were added or revised during scoring. Only c004 records and their associated `docs/review-evidence/calibration/whole-thesis-reading-1700.md` anchor were consulted for this task.

Both reviewers use the same explicit audience override: an MSc student who knows graduate linear algebra, real analysis, and basic differential geometry, with no assumed knowledge of symplectic capacities or contact geometry. That override was introduced for this preliminaries task; the later-chapter expertise assumed elsewhere in the trial is not inherited here.

The eligible labels are **l025 and l026**, both active, attributed to `coordinator_record_of_human`. Labels l027–l030 concern other sections/chapters and are excluded, as are abstract, introduction, and whole-thesis judgments in the associated anchor. In particular, the withdrawn Chapter 4 annotation is neither a section 2 defect nor a positive section 2 label; the Chapter 4 qualified PASS is not transferred here.

**Direct detection** identifies the local objection and mechanism; **partial** identifies part of that objection; **missed** means the review does not identify it. **Explicit disagreement** requires approving a feature the human specifically rejects. Reviewer concerns without corresponding adjudication remain **unadjudicated**, not automatically false positives.

## Exact eligible human labels

### l025: prose_quality (rejected)

> Jörn immediately rejects the preliminaries opener (“The later computations
> replace a boundary-dynamics problem by finite data from the facets of a
> polytope”) as sloppy and asks whether Codex should have flagged this wider
> class of writing-quality issues.

### l026: prose_quality (problem_identified)

> Jörn next objects to “We will use ... without further comment.”

The l025 record additionally limits its applicability to the “17:30 snapshot only.” These are exact recorded evidence fields, not a claim that all surrounding coordinator wording is a direct human transcript.

## Mapping the local prose objections

| Human objection | Baseline | Prerequisite specialist | Evidence |
| --- | --- | --- | --- |
| l025: section 2 opener is sloppy | **Missed** | **Missed** | Neither quotes or objects to “The later computations replace a boundary-dynamics problem by finite data from the facets of a polytope.” Baseline concerns address terminology, Sobolev notation, and the capacity domain; the specialist flags an analytic prerequisite bridge. Those are different writing problems. |
| l026: “We will use ... without further comment.” | **Missed** | **Missed** | Neither identifies the notation sentence as a prose defect or asks to remove its announcement. The specialist's praise for familiar linear algebra in section 2.5 concerns a different passage and is not a detection of this sentence. |

Neither review expressly endorses either human-rejected sentence, so these are misses rather than explicit local disagreements. The specialist praises the opening of **section 2.6**, where the dual problem's purpose is explained; that is not praise of the **section 2** opener rejected in l025. Its broader favorable assessment of mathematical explanation also does not constitute human-confirmed prose readiness.

The source record does not specify a complete taxonomy or full causal analysis of “sloppy” prose. This scoring therefore does not invent one. The two known objections suffice to establish these passage-level misses without extrapolating that every passage is defective.

## Relevant qualifications and withdrawal

The associated anchor explicitly records a withdrawal of the coordinator's implied prose-readiness claim, not a withdrawal of Jörn's two objections:

> Coordinator withdraws the implied prose
> readiness claim: the preflight had been restricted to material mathematical
> prerequisites/inference defects and explicitly excluded a style inventory.
> That narrower check was not evidence for prose readiness.

It further says:

> A new sequential
> prose review is underway, with the restriction withdrawn and human examples
> provided without making them an exhaustive taxonomy. The known opener is not
> counted as a new detection. Jörn is asked to pause this section's human review.

These statements describe the historical review process, not either fresh frozen review. They make the limitation consequential: a prerequisite-focused check cannot establish prose readiness merely by finding few mathematical explanation gaps. The fresh specialist's analytic concern may still be useful, but does not answer the two known prose objections. This historical evidence also does not establish that Jörn completed reading section 2.

The anchor describes deletion of the notation sentence in a scratch copy and a revised section opener. It expressly keeps the fixed PDF unchanged. Those later repairs are not new detections by either frozen reviewer. The supplied section 2 extraction retains both objection targets.

## Novel concerns remain unadjudicated

| Frozen reviewer concern | Status against the eligible human evidence |
| --- | --- |
| Baseline: contact and Lagrangian classifications need meanings beyond operational equations | **Unadjudicated.** Neither eligible human label addresses these definitions. |
| Baseline: Sobolev curve notation needs a bridge from absolutely continuous curves | **Unadjudicated.** The prose objections do not resolve whether the stated audience needs this bridge. |
| Baseline: compact-body capacity domain does not explain the unbounded cylinder normalization | **Unadjudicated.** No eligible human label adjudicates the domain presentation. |
| Specialist: Sobolev regularity and distributional variational equations need one analytic bridge | **Unadjudicated.** It overlaps the baseline's curve-space concern and extends it to weak equations, but neither part is human-confirmed here. |

The specialist's decision not to treat operational contact terminology, background capacity properties, or the explicitly imported theorem as missing derivations is also unadjudicated. The baseline's contrary concerns on terminology and domain are reviewer differences, not human-scored disagreements. The two eligible labels establish no confirmed false-positive reviewer concerns; they also do not establish that the novel concerns are valid or that unmentioned text is clean.

## Evidence limits

This is a selected historical section with two recorded local prose objections, not exhaustive annotation, a mathematical correctness audit, or a representative accuracy test. No precision, recall, or comparative accuracy estimate follows. The consistent audience override supports a fair comparison between these two fresh reviews, but does not imply that the historical human feedback explicitly adopted every part of that background specification.

Despite its filename, the anchor distinguishes an initial 17:00 PDF from feedback after switching to a fixed **17:30** PDF. The eligible labels belong to the latter. Its header's 17:00 file hash is therefore not used as an identity claim for this section 2 sample. This scoring verifies frozen-review hashes and the presence of the quoted text in the supplied extraction; it does not establish binary identity with the historical 17:30 PDF. Freezing before this scoring pass does not establish historical non-exposure of the broader harness to c004.
