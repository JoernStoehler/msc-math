# Executable first steps after the empirical programme

2026-09-18. Feasibility review, not executed new science. Source branch `research/empirical-viterbo-design` at `b6c63665`. This work owns only this report and its small input receipt. No producer, capacity evaluator or large table rebuild was run.

## Recommendation

Run **one genuinely target-free cross-representation discovery pass on already available rows**, alongside **one small support-deformation/branch-transition study using already retained geometries**. These buy different information. Neither requires a new full dataset, 45-feature rebuild, a sophisticated active-learning service, or a preselected theorem. Prefer the first for immediate execution: its complete input join has now been checked. The second needs a small evaluator preflight and a check against existing local-optimization studies before spending on a trajectory.

The four campaigns in the parent README are useful distinctions, but not four ready experiment specifications. A's available representation was unspecified; B's advertised initial collision inspection has already happened; C does not choose an operation or an observable; D does not yet specify what scientific value its acquisition rule buys. Turning all four into infrastructure projects would be a mistake.

## What is actually available

The [machine-readable receipt](first-steps-input-receipt.json) checks content hashes, unique IDs, joins, bucket counts and numerical-field finiteness.

* Historical table: 14,336 rows at `/home/joern/.cache/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl`, hash `607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59`. It has the original 39-feature schema, not the later 45-feature schema. The June analyzer replay is independently recorded in the DS closure packet.
* `experiments/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl`: 14,334 rows with `rho`, `nu1`, `nu2`, representative-dependent `covariance_condition`, a numerical diagnostic, bucket/source names, and IDs. All join to the historical table, and all stored sys values agree exactly. The two missing rows are explicit numerical rejection cases, not random omissions: one F8 covariance failure and one F12 symplectic-spectrum failure, both with very low historical sys. Retain their exclusion receipt.
* `residual-exemplar-seeds/artifacts/geometry-branch-inspection/branch-input-table.jsonl`: fourteen compact geometries, with dual-row f64 coordinates and stored capacity/volume/sys. Adjacent files contain per-face measurements and branch outputs. These are derived f64 inspection inputs, not substitutes for the producer's exact rational source if exact certification becomes necessary.
* The host cache currently has neither the `polytope-datasets` nor `product-bounce-distribution` family. This is a checked local absence, not proof the sources cannot be recovered. Existing compact artifacts suffice for the pilots below; do not make retrieving everything a prerequisite.

The existing covariance report already studies rho versus sys and ridge summaries. Repeating that analysis under a new campaign title is not the recommended experiment. Also, covariance condition number is **not** a symplectic invariant; it is useful here precisely as an explicitly representative-dependent comparison. The symplectic eigenvalues are dimensional (common scaling multiplies them by length squared), while their ratio is scale-free. Numerical pairing error is a quality-control field, not a geometric feature to mine as though it were one.

## Pilot 1: target-free structure across available representations

**Question.** Do combinatorial geometry, symplectic face-area distributions and vertex-covariance shape exhibit simple cross-family structure that is hidden when every analysis is ranked by correlation with sys?

**Execution, scoped to one work session.**

1. Join the two checked inputs by `poly_id` into a run-local table; retain source bucket and the two rejected IDs. Exclude sys from discovery entirely. Keep it in a separately keyed file for possible later interpretation, not in the discovery matrix.
2. Keep all nonconstant combinatorial columns, the face-area distribution columns, rho and covariance condition. Separate dimensional eigenvalues into a labelled scale-sensitive exploratory view; do not silently pool them into the scale-free view. Log positive heavy-tailed quantities; retain original definitions. Annotate exact relations such as sum = count × mean and top-level face-lattice identities so they are interpretable rather than counted as new discoveries.
3. Produce a small cross-family rank-association map with pooled and exact-bucket views. Products have fixed combinatorics within a bucket; undefined within-bucket correlations should stay undefined. Inspect departures from monotone relations with scatterplots and the actual geometry IDs at both ends, rather than immediately adding a model zoo.
4. Use one complementary discovery operation: find pairs that are close in one representation and distant in another, within the same bucket. Use ranks/standardization fit within that bucket; choose a small set spanning generic and product bodies. This asks what information the representations omit without assuming sys is the only meaningful response.
5. Return at most six inspected leads, including weak/failed ones: exact variables, domain, representative rows, potential definitional explanation, and cheapest next observation. Only after freezing that list optionally inspect sys, describing this as interpretation of an exploratory lead rather than validation.

**Why this is broad enough for a first pass.** It includes three substantively different measurement families and permits target-free and representative-dependent patterns. It is not the full intended broad programme. If its useful relations are all already explained, expand representations rather than run thirty more estimators on the same columns. Missing raw geometries are a dependency for interpreting some leads, not a reason to prohibit their discovery.

**Branch decisions.**

* A relation survives bucket separation and is not definitional: recover a few source geometries and seek a mathematical explanation or a deliberately contrary construction. It need not improve capacity prediction to deserve follow-up.
* A relation exists only between buckets: identify the relevant combinatorial/generator fact. This may itself explain the dataset, but do not call it a universal geometric law.
* A pattern depends on covariance condition: test a few controlled symplectic re-representations before deciding whether this is a coordinate effect, generator information, or a useful convention-specific heuristic.
* Everything is familiar/redundant: the next expenditure is one new representation (e.g. incidence-organized field or factor shape), chosen from the concrete missing distinction. It is not a target-free campaign failure merely because no capacity predictor improves.

**Cost judgement, not measured forecast.** Join/schema check already took under a second. On 14k rows and dozens of columns, correlations and a bounded nearest-neighbor pass should take seconds to minutes with conventional vectorized tools; plotting, checking identities and interpreting six leads likely cost 20–45 agent minutes. No capacity or feature regeneration. A 20-minute first pass can return an initial map and two inspected leads instead of pretending the entire interpretation is finished. There is no calibrated probability of new mathematics from this run.

## Pilot 2: what terminates a useful inactive-facet deformation?

**Question.** Counts of unused facets do not explain the retained two-/three-bounce contrast. How much useful deformation is available before branch competition or a geometric transition ends it, and what predicts that endpoint?

The existing mechanism packet already established that all retained winners use six facets, so repeating active-facet counts buys no information. The formal support-number derivative argument predicts a first-order sys increase when an inward-moving facet is invisible to every active minimizing datum (under its chamber hypotheses). This is a starting explanation, not an empirical novelty claim. The unresolved object is the **extent and termination** of that opportunity.

**Concrete starting input.** Use the retained 4×4 discordant pair:

* lower sys: `ca02ba26dff65c9b67a0f09eb5967d5be495c94f6e541ad8cc96b839714bf552`, stored sys 0.6237310456038816, two-bounce pattern;
* higher sys: `a1dc7a009c6219851007ae62f65cd090aa284ea054af825ed9b1ac04473e47f0`, stored sys 0.7864121397554283, three-bounce pattern.

Both dual-row geometries and minimizing-word observations are retained in the compact branch-input packet. These are selected exploratory examples, not representative estimates of a population. The 4×4 family is theorem-excluded for counterexamples; that is fine for a mechanism study. The independent family-theorem mapping supplies the precise theorem scope.

**Sequence.**

1. Check `formal/active-orbit-facet-coverage.tex` hypotheses and current branch representation; distinguish cyclic words from the same-type block permutations that previously inflated apparent multiplicity. Inspect existing branch/optimizer trajectory studies for this exact question. If they already answer it, extract the answer instead of running a duplicate trajectory.
2. Recompute both base examples with an actual current evaluator, measure walltime, and establish whether all relevant active words/weights and separate competing branch actions are observable. The existing historical fourteen-row diagnostic reports 11.8 seconds total, but this does not predict the current certified evaluator or a cold build. No hard runtime guarantee follows.
3. If observable and affordable, choose a facet unused by the union of active minimizing supports; vary its support inward at fixed normal. In normalized-dual coordinates this replaces that row `a_i` by `a_i/(1-t)`, with positive denominator. Use a small initial t, halve for a derivative sanity check, then expand until the first interesting change. Check facet survival and bounded/full-dimensional geometry at each step. Retain capacity and volume separately, minimizing support/branch, sys, and numerical status. Do not infer a capacity branch is globally minimal merely because its action was continued.
4. Refine only the first observed branch/combinatorial transition. Compare against the other pair member. Ask whether the contrast is deformation room, volume response, or competing branch arrival. Do not acquire a uniform dense grid when an adaptive bracket suffices.
5. If a simple endpoint criterion emerges, freeze it and test one other body, preferably the retained 4×6 contrast. If no criterion emerges, retain the explicit obstruction and stop rather than enlarge to an indiscriminate resampling campaign.

**Missing plumbing.** The legacy branch diagnostic accepts the retained table/provenance adapter but is not automatically the current certification route. A small driver must write perturbed geometry, call the selected evaluator, reconstruct volume/features as necessary, and preserve statuses. Access to beta/weights and complete class alternatives must be checked in that route. Exact rational geometry would be needed for a later exact counterexample/certificate, not for this first exploratory observation.

**Cost judgement.** Mathematical/setup inspection approximately 15–30 agent minutes. If the first two base evaluations and branch extraction are cheap, cap the initial path at 12 evaluations per body, with a 60-second per-body ceiling and an overall compute cap; this cap determines whether to continue, not an assertion that all evaluations will finish. Interpretation could take another 20–40 minutes. If branch observability needs substantial implementation, stop and return the exact interface gap; broad pilot 1 remains independent. No full certified scan, exact volume certification or new reusable acquisition framework is required in advance.

**Meaningful outcomes.** A new simple transition criterion motivates proof; immediate branch competition explains why an active-count account fails; a geometry wall first suggests a combinatorial descriptor; expensive/unobservable competitors reveal that this is not presently the cheap next experiment. A numerically flat action alone is neither proof of constancy nor a new theorem.

## Campaign corrections and priorities

* Collision inspection is not untouched territory. Its fourteen geometries already yielded no common direction for the tested incidence summaries. The apparent four-way branch multiplicity was explained by same-type product facet ordering; later bounce and width studies followed it. Do not rerun this story as discovery.
* The difference-body formula for the two-bounce minimum already has a formal draft and a twenty-row exact numerical check. Broad all-row rational validation was previously stopped for cost. Use it as a mathematical baseline, not a fresh candidate formula to rediscover.
* Family exclusions matter differently by goal. The theorem-mapping agent reports triangle/quadrilateral factor coverage excluding 7/10 retained product buckets from sys>1; arbitrary 5×5, 5×6 and 6×6 products are not excluded solely by side count. Preserve excluded families as structural controls. Separate theorem-eligible exposure if later evaluating counterexample acquisition.
* Acquisition-policy replay is lower priority until an actual discovered phenomenon or missing-region question defines its value. A generic diversity score is not yet a scientific outcome. The support-path bracketing above is already adaptive acquisition without new infrastructure.
* Source retrieval and covariance-geometry reconstruction can be delegated only when selected leads require them. Do not serialize the immediately runnable broad pass behind a complete historical dataset recovery.

## Unresolved choices for the research owner

No Jörn decision is needed to perform pilot 1 at exploratory scope. The owner should decide after its first output whether any lead warrants more interpretation. Pilot 2 needs a modest implementation-budget decision after the evaluator/trajectory preflight; defer to an existing answer if found. Neither pilot should delay integrating already useful science into the thesis or the parallel human writing review. Strong new mathematical leads can be passed to the external Pro workflow, but this plan does not depend on its pending pentagon result.
