# Frozen review records

## Reader prompt

Read-only independent reader review. File `/workspaces/msc-math/.worktrees/ds-human-feedback-revision/experiments/writing-quality/runs/20260918-human-feedback-revision/revised.md`. Audience MSc mathematician acquainted with symplectic capacity, Lagrangian products and systolic ratio, not assumed familiar with data science; surrounding chapter not assembled. Judge relevance/emphasis/prerequisite explanations/transitions/paragraph argument, not just truth. Do not infer PASS from lack of defects; give exact spans and concrete obstacles, distinguish preferences. You have not been supplied human annotation list; this is not blind to revision existence. Figures currently being generated so flag what they must show but do not count their temporary absence as defect. Return review in message; no writing edits. 5min bounded.

The reviewed bytes are now `review-input.md`. Reader used fresh context.

## Reader result (verbatim)

The section has a coherent argument: observed ordering → exact example → limitation on deformation → prospective candidate selection. The rank-correlation explanation is sufficient for the stated audience, and the product-face calculation supplies a useful geometric bridge. I would still request revision before accepting the selection section’s conclusion.

Concrete reader obstacles:

- **“geometric selection rules were applied … selected and control bodies”**: neither the rules nor the controls are described. Consequently, the first experiment’s maximum of \(0.868\) establishes failure to find a violation, but gives the reader no evidence that selection helped. Briefly identify the rules and explain how controls were chosen; report the comparison relevant to enrichment, or state that this experiment only supplies a search outcome.
- **“A further test changed the source of the polygons, using separately area-normalized factors.”** Area normalization does not tell the reader what changed about the source distribution. Likewise, **“a ridge-based rule fixed in advance”** with **“disjoint controls”** does not identify either population. This paragraph is carrying evidence for robustness, but the reader cannot tell what robustness was tested. Name the changed generator, the selection rule, and the control construction. A short sentence for each would suffice.
- **“Together these experiments support using ridge geometry to enrich particular candidate pools.”** The second experiment compares two halves already selected for low \(R\); it supports the additional largest-ridge-fraction criterion *conditional on low \(R\)*. It does not by itself establish the value of low-\(R\) selection against an unselected pool. Make that distinction explicit, and tie the broader enrichment claim to an actual control comparison.
- **“Their endpoints have systolic ratios \(3/4\) and \(1/2\), respectively.”** The preceding sentence labels these paths numerical, but the fractions sound exact. State whether the endpoint values are proved or numerical, and identify the endpoint bodies if that makes the obstruction intelligible. As written, the fractions contribute little to explaining why the deformation intuition fails.
- **“The exact pentagon relation gives one explanation for its usefulness”**: the exact calculation establishes that the inverse association occurs in one special family. It has not explained why selection works in the fresh random pools. “Supplies an exact example of the association” would match the demonstrated connection more closely.

Figure requirements:

- Figure 1 must make the behavior **within** component populations visible, because the text explicitly uses it to address the limitation of the pooled correlation. Label \(R\), \(\widehat{\operatorname{sys}}\), population types, and facet/polygon sizes clearly.
- If practical, the opposing paths would benefit from a small plot or explicit endpoint/start values showing simultaneous decrease. This is a presentation preference, not a necessary additional experiment.

A smaller self-containment issue: the caption’s **“specified random generators”** promises more specification than the text provides. Heights, seed, and size counts do not identify the geometric sampling distribution. A reference to a generator description would resolve this without adding substantial methods prose.

## Reader follow-up

Prompt: Quick follow-up review revised.md selection section only; review-input.md retains original. Now explicit generator/rule/control/mean table, eliminated vague first100k result and endpoint fractions, fixed mechanism overclaim. Material obstacles remaining? no overall PASS request.

Result (verbatim):

The main assessability obstacles are resolved: the rule, comparison population, sample counts, and observed differences are now visible.

Two wording issues remain:

- **“than controls from a different source”** can suggest selected bodies and controls came from different generators. Say “When applied to a different polygon generator, the same rule selected bodies with larger … than controls from that generator.”
- **“isolates the additional benefit of less concentrated ridge area”** overstates the half-versus-half comparison: other geometric properties can differ between those halves. Say “shows an additional association with less concentrated ridge area within the low-\(R\) group.”

No other material reader obstacle emerged in this focused reread.

## Meaning-check prompt

Read-only source fidelity check of revised.md at `/workspaces/msc-math/.worktrees/ds-human-feedback-revision/experiments/writing-quality/runs/20260918-human-feedback-revision/`. Compare primary evidence where needed (ds-evidence-closure packet, original selection/transfer/path reports). Especially path statement boundary and whether selection claims accurate rather than adopting tentative human interpretation. Check formulas and numeric statements, no capacity reruns. Return exact actionable issues plus sources checked within5min. Do not grade prose PASS. No edits.

Meaning reviewer inherited task context, including supplied human feedback.

## Meaning-check result (verbatim)

No material fidelity error found; no prose acceptance verdict.

One small repair: the opening asks why face areas are “informative about capacity,” whereas the observed association concerns the **systolic ratio**. Use that term, or “capacity relative to volume.”

Verified:

- Pentagon formulas and constants match the supported analytic account.
- Path statement correctly limits the reversal to numerical values at retained steps; it does not claim a theorem along the continuous paths.
- Concentration comparison is against the complementary half: difference \(0.0339924696\), positive in 8/10 buckets; the predeclared criterion required 7/10 and positive overall.
- Transfer statement matches one separately area-normalized source, two buckets and 91 distinct evaluations.
- Historical counts and rounded correlations match the reproduced account.

Sources checked: `docs/ds-evidence-closure/{README,search-account,ridge-mathematics}.md`; original `ridge-endpoint-path/INTERPRETATION.md`; `alternative-source-transfer/POST-TARGET-ACCOUNT.md`; concentration `selection-plan.json`, `evaluation-report.json`, `validation-verdict.json`, `selection-summary.tsv`, and `review.md`. No capacity reruns or edits.

## Meaning follow-up

Prompt: Quick follow-up read latest revised.md final3 paragraphs: generator describes common supports tangentialized then normalized, dual eligibility, hash controls, new mean table. Check against main.rs/README/analysis.json. Opening now will change to systolic ratio. Return only any discrepancy.

Result (verbatim):

One discrepancy: **“controls … without using … geometric descriptors”** is too strong. Hash ordering ignores descriptor values, but control eligibility excludes the union of both ridge-selected and covariance-selected bodies (`main.rs`, `arm_union`/`rest`).

Suggested replacement: “Sixteen controls were chosen by a fixed hash ordering from bodies outside both geometric selection sets, before capacity evaluation.”

Generator description and all four rounded table means match the implementation and `analysis.json`.

## Disposition

All actionable wording findings above are repaired in final revised.md. Optional opposing-path plot not added; this trial already has actual empirical plots and does not claim full final visual design. No additional full-reader review or human review has occurred after these changes.
