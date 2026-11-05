# Optional passage: placement, support and insertion cost

Status: separate candidate for the thesis owner. This does not replace the pending containment paragraph or its human question, and no selected manuscript file is changed. Root owns selection, integration, final scientific/message review, commit and cleanup.

## End state and necessary work

The candidate should give an MSc reader a new mathematical distinction: conditional balance is weaker than jointly realizable independent factor choices; equality of a relaxed lower bound and extracted feasible upper bound turns that distinction into a certificate. The finite panel shows when this worked, while HKO proves where universal sharpness of the relaxation fails. A chronology of the research would not establish this connection.

The draft is an explanation candidate, not a ready-to-insert certified manuscript extension. Before insertion, the owner must (1) select this extra scope rather than merely add accumulated research, (2) supply the imported sparse/six-facet and two-shadow implications and the bound argument in a resolvable proof location, (3) bind the finite sample, exact certificates and HKO coordinates to retained accessible evidence, and (4) review the actual resulting prose and references. Omitting any step respectively risks scope drift, an unsupported lower bound, unresolvable numerical claims, or an unreviewed mathematical statement.

## Placement and notation

Suggested Chapter 8 placement: after `What could explain the bulk association?`, just before `Using evaluated ratios to guide search` (`thesis/chapters/08-data-science.tex:474`). It would make a separate short subsection, provisionally `Compatible factor choices`. This moves from limitations of scalar scores to a constructive signed-geometry mechanism before evaluation-driven search. It should not be presented as a demonstrated cause of the ridge-area correlation or of its conditional attenuation.

Use the already selected Chapter 4 notation $u_i,v_j,C_q,C_p,\alpha,\gamma$, not the research reports' $a_i,b_j,C_A,C_B$. Chapter 4 defines normalized closures at `thesis/chapters/04-quadratic-program.tex:491` and proves the six-facet product result at line 501. Its factorization is labelled `eq:quadratic-program-product-factorization`; the capacity formula is `thm:quadratic-program-algorithm-hk2019-hk-formula`. The draft says “two angular orders” only as a placeholder for an explicit theorem/lemma reference supplied on integration.

**Real insertion prerequisite:** the selected manuscript has the sparse/six-facet reduction but does not presently state the two-shadow order theorem. A maximum over just two LPs cannot be asserted to bound the full $Q_{\max}$ merely because each LP relaxes its own fixed word. Import and explain the sparse + two-shadow implication, including reversal of the negative-shadow order. Alternatively maximize over all words to define a valid theoretical relaxation, but that would not be the two-LP computation actually certified here. Short prose alone does not close this dependency.

The passage also imports the conditional-closure LP bound and rank-one extraction implication; an appendix lemma/proof sketch should retain the explicit constraints and the elementary sandwich proof. HKO's strict gap uses the known exact pentagon capacity plus an exact LP dual; its factor is scale-invariant, so the main paragraph need not introduce the transformed normalization $c=5$.

## Alternatives, rubrics and choice

Considered alternatives were a short mechanism passage, a chronological account of tested selectors, a full new theorem/proof subsection, or no addition.

Three plausible rubrics were insight per reading effort, proof completeness, and empirical predictive success. Their failure modes differ: brevity can conceal proof imports; completeness can bury Chapter 8 under optimization algebra; predictive agreement can reward previously exposed labels and ignore failures at the extremal control. Use insight as the main objective, with visible proof/evidence obligations as constraints rather than a numerical average.

Ordinal ratings below use 0=poor, 1=partial, 2=good, 3=strong; they express editorial judgment, not measured reader outcomes.

| Option | Mathematical insight | Reader accessibility | Insertion cost | Scope fidelity |
|---|---:|---:|---|---:|
| Short mechanism passage + proof/evidence references | 3 | 2 | Moderate: missing order lemma and certificate references | 3 |
| Chronological research account | 1 | 1 | Moderate: more claims and many method names | 1 |
| Full LP and order theorem subsection | 3 | 1 | High: proof, conventions, and data protocol | 2 |
| No addition | 0 | 3 | None | 3 |

Choose the short mechanism passage as the optional candidate because it explains what the newer evidence changes and makes the obstruction visible in about a page. Choose no addition if the theorem/evidence integration cannot be completed within the selected hand-in scope. The ratings do not authorize extra research or a manuscript change.

## Source anchors

All following pattern paths are relative to `/workspaces/msc-math-worktrees/patterns-2026-10-17/`. The thesis paths above are relative to `/workspaces/msc-math-worktrees/thesis-2026-10-16/`. Access was focused local reading of these reports and exact result/verification schemas, with no experiment, web, exhaustive search or novelty assessment.

- `patterns/upper-tail/conditional-coupling/REPORT.md:9`: normalized closures; lines 13–23: signed coefficients, factor $1/4$, LP constraints and rank-one inclusion; lines 25–31: two-shadow-to-global implication and capacity sandwich.
- `patterns/upper-tail/conditional-coupling/REPORT.md:69`: exact HKO primal/dual values; lines 77–81: strict ratio $L/Q_{\rm true}=(1+3/\sqrt5)/2$ and incompatibility conclusion. Matrix rank greater than one alone does not prove nonseparability: mixtures can have higher rank.
- `patterns/upper-tail/mixed-area-order/REPORT.md:19`: two-shadow theorem; lines 27–44: proof with a sparse factor; lines 58–64: combination with sparse reduction. Its negative-shadow value is negated, equivalently the word is reversed. The proof does not claim the two words suffice for arbitrary nonseparable couplings over all possible words.
- `patterns/upper-tail/conditional-rounding/REPORT.md:25`: conditional row/column extraction and opposite response; line 29: rank-one optimizer implication. “Rank one” is sufficient, not necessary, for exact extraction.
- `patterns/upper-tail/conditional-rounding/REPORT.md:65`: exact HKO extraction; line 67: five closure-vertex coverage explains one explicit circulant optimizer, not every optimizer.
- `patterns/upper-tail/conditional-validation/REPORT.md:13`: frozen sample membership and retrospective exposure; lines 27–45: agreement criterion, counts, bound reduction and post-score rank-one observation; line 47: nine unresolved intervals and why neither rank nor interval proves a true gap.
- `patterns/upper-tail/conditional-validation/verification.json`: PASS for 35 rational controls, 70 LP certificates, 470 conditional responses, two algebraic HKO LPs and exact $Q=1/10$ extraction. `owner-verification.json` independently reconstructs geometry, objectives and metrics. These are computational checks under the imported mathematics, not human mathematical acceptance.

## Exact field identities and safe interpretation

In `patterns/upper-tail/conditional-validation/results.json`:

- `rows[*].A` and `.B` are the rational dual-vertex lists, corresponding to draft $u_i,v_j$.
- `rows[*].variants[*].W` is an exact retained coupling; `.Q` its LP optimum; `.dual` and `.dual_slack` its certificate; `.word` and `.shadow_sign` fix conventions.
- `rows[*].rounding.candidates[*].alpha`, `.gamma`, `.word`, `.Q`, `.capacity` bind each actual feasible factor pair and its direct HK value. `.rounding.best` is the maximizing extracted candidate.
- `rows[*].capacity_lower = 1/(2 max(variants[*].Q))`; `.capacity = 1/(2 rounding.best.Q)`; `.exact_capacity` is exact equality of these rational endpoints, independently checked in `verification.json`.
- `.historical_capacity` is a numerical label, not a certificate. `.capacity` agrees with it within relative tolerance $10^{-10}$ on all 35 rows; `summary.all.capacity.agreement = 35`.
- `summary.all.n = 35`, `summary.all.exact_capacities` has length 26; `summary.tail.n = 26`, `summary.tail.exact_capacities` has length 17; `summary.bulk.n = 9` and all nine are exact. Thus the nine unresolved capacities are all tails.
- `rows[*].incumbent_capacity` is the strongest retained feasible comparator, and `.standalone_over_incumbent = capacity/incumbent_capacity`. For `random_5x5_s3_227`, $1-C_{\rm new}/C_{\rm incumbent}=0.0149595998778\ldots$. This is a capacity upper-bound reduction, not a systolic-ratio increase or new ratio exclusion.
- `prior_exact_capacity` means the old witness equals this newly computed LP lower bound; it does not mean a prior certificate already existed. Avoid “26 newly discovered capacities”: most old numerical values and 24 attaining feasible witnesses were known already.
- `hko.exact_recovery = true`, `hko.true_Q = {"rational":"1/10","sqrt5":"0"}` refer to the transformed exact HKO normalization, not circumradius-one coordinates. The draft uses the invariant objective ratio to avoid conflating these coordinate conventions.

Do not infer universal exactness, prospective holdout performance, a scalar predictor, an explanation of the ridge correlation, or global HKO maximality from this finite panel. Nine unequal intervals do not prove nine genuine relaxation gaps. The separate rational strict-gap pilot is not one of the 35 frozen test bodies and is deliberately omitted from the short draft.

## Lower-cost alternative and containment-only route

[selection draft](SELECTION-DRAFT.md) is a second actual prose candidate. It keeps the main insight—signed joint geometry suggests compatible support pairs—and reports only independently feasible upper bounds, numerical agreement and the known HKO control. It makes no global capacity lower-bound claim, no 26/35 exactness claim and no two-shadow sufficiency claim. Its imported facts are the existing HK formula, the selected product closure reduction, elementary conditional feasibility and retained witness checks. Fixing either full word still yields feasible candidates regardless of whether those two words suffice globally. Consequently it avoids the new angular-order theorem appendix. For HKO it reports recovery of the already known capacity, not a newly proved general sharpness result.

An LP definition or brief methods note is still needed so readers can resolve how the matrix was selected; this is smaller than importing the two-shadow theorem and exact HKO gap proof. The owner should retain an accessible finite-evidence packet and clarify that the numerical labels had been exposed. The lower-cost candidate does not explain the 26 exact certificates; that omission is an explicit editorial choice to avoid silently importing their proof chain.

Compare three realistic manuscript routes:

| Route | Added insight | Mathematical/evidence integration | Editorial recommendation |
|---|---|---|---|
| Existing containment-only paragraph | A geometric relation interprets the feasible support construction | Existing pending scope/evidence question; no new joint LP | Keep as the baseline already under review |
| Add qualitative joint-selection passage ([selection draft](SELECTION-DRAFT.md)) | Conditional vs independent factor choices explains a genuinely different selector | Define conditional matrix, link finite witnesses; no new global-order theorem | Preferred extra passage if the owner selects more data-chapter insight within a small insertion budget |
| Add exact-certificate passage ([exact-certificate draft](EXACT-CERTIFICATE-DRAFT.md)) | Also explains why rank one closes a capacity interval and why HKO obstructs the relaxation | Import sparse + two-shadow implication, LP sandwich/extraction proof, HKO exact gap and certificate evidence | Higher-cost optional expansion; short prose alone is insufficient |

The main rubric should be useful mathematical understanding per integration hour, constrained by correctness and resolvable evidence. This rubric could wrongly favor vague suggestive prose, so the low-cost draft still states the exact feasible-witness guarantee. A theorem-completeness rubric could wrongly force a larger chapter; a numerical-agreement rubric could wrongly call exposed labels prediction. These risks favor the qualitative passage over mandatory theorem expansion when hand-in speed matters. None of the three routes is selected for integration by this scratch packet, and this comparison does not supersede the current containment question.

[review: /root/gap_crux]
