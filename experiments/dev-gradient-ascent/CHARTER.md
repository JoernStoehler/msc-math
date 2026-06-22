# Dev Gradient Ascent Charter

Status: objective charter for `experiments/dev-gradient-ascent/`.

Current scope note: after this charter was first written, the active question
broadened from only "develop/promote a gradient-ascent method" to "study
`sys(a)` and HK branch behavior across local, semi-local, and effectively
global perturbation scales, so we know what a fixed ascent method has to
handle." The gradient-ascent method remains the main downstream consumer, but
new branch-cartography work should be read through this broader scope. See
[branch-cartography/README.md](branch-cartography/README.md).

This file defines the real objective of the top-level gradient-ascent
development suite so future work does not collapse into a smaller coding
milestone, a one-sided success narrative, or a proxy artifact that is mistaken
for the target property.

This file is not a task queue. It is not a promise that every question listed
here will be answered before thesis submission.

Overruled by: Jörn/Kai thesis-success decisions, source code and generated
artifacts in `experiments/`, proof-bearing surfaces in `formal/` and
`experiments/hko-local-maximum/`, and later reviewed updates to this charter.

## Objective

Develop a gradient-ascent method for the nonsmooth high-dimensional function
`sys(a)` whose ascent endpoints are local maxima on the quotient, and develop
the experiment surface needed to find, test, improve, reject, or caveat such a
method.

This objective is meant to make the following thesis statement true, or to make
clear how the statement must be weakened:

```text
We developed a gradient-ascent method for sys(a). The problem is nonsmooth and
high-dimensional because the controlling HK sigma branches can change. The
method reaches local maxima on the quotient, or reaches a weaker endpoint
condition whose scope and caveats are explicitly stated. The resulting fixed-F
runs determine what the thesis can truthfully say about local optimization in
the hostile sys-search landscape.
```

For generic non-HKO endpoints, this suite does not target theorem-grade
arbitrary-endpoint local maximality. HKO theorem-strength local maximality is
owned by `experiments/hko-local-maximum/` and its formal/Sage certificate
surfaces.

## Desired Property And Weaker Outcomes

Desired final property:

```text
The ascent endpoints are local maxima of sys on the quotient.
```

This is the target property. The actual outcome may still be compatible with
thesis success if it is weaker, but only if Jörn/Kai accept the resulting claim
strength, caveats, and costs. The list below is not a whitelist. It records
examples of weaker outcome shapes that could still be thesis-success-compatible
after review.

Examples of weaker outcome shapes:

- Most retained endpoint classes pass the local-maximum checks, and remaining
  classes are explicitly tagged rather than silently included.
- Some endpoints still have possible improvements, but further optimization is
  too compute-intensive relative to thesis value.
- Some endpoints would require new one-time method work whose expected thesis
  value is lower than rerunning/writing with the current method and caveats.
- The method is retained with scoped wording: it reaches endpoints locally
  stable under the implemented diagnostics, not all true local maxima.
- The method fails as a final optimizer but still gives a thesis-useful caveat
  or explains why the downstream search claim must be weakened.

The checks and artifacts do not decide acceptance by themselves. They inform
the Jörn/Kai thesis-success judgment by making the outcome, caveats, and costs
explicit.

## Target Properties, Checks, And Gaps

Use this pattern when adding experiments or interpreting outputs:

1. State the target property directly.
2. State the check or artifact that bears on it.
3. State the gap between the check and the target property.

Do not make a proxy the target. A diagnostic, sampling run, pro/con narrative,
or favorable artifact is not itself success.

Current target/check/gap map:

| Target property | Checks and artifacts | Known gap |
| --- | --- | --- |
| Endpoint `a0` is a local maximum of `sys` on the quotient. | Quotient/transversal branch diagnostics, maximin/common-ascent checks, local probes, endpoint diagnostics. | Generic checks are finite and heuristic; they can miss branch/germ behavior or unsampled improving directions. |
| The method responds to nonsmooth branch changes rather than only smooth single-branch ascent. | Run traces that record near-active branch sets, step policies, action gaps, branch switching, and stop reasons. | A trace can show what the implementation did, not that the chosen model includes all relevant local branches. |
| The method reaches the retained endpoint condition from search-relevant starts. | Rerunnable ascent runs from random or retained starts with endpoint diagnostics. | Success on sampled starts does not prove success for all starts. Failures must be tagged rather than silently dropped. |
| The method can be used at fixed-`F` datascience scale. | Runtime, exact-evaluation count, failure-rate, and LICCA/local smoke reports. | A cheap run may not predict production cost; production cost can make further optimization not worth thesis time. |
| The retained thesis wording says what happened. | Thesis-claim packet or analysis packet that states observations, caveats, failed checks, accepted weaker outcomes, and Jörn/Kai decisions. | A packet still needs human thesis-success review; artifacts cannot approve their own claim strength. |

## Question Sets

The suite should preserve two related question sets. They are not a task queue.
They help agents choose high-value experiments and understand why an unanswered
question can still matter.

### Questions Answerable By A Complete Model

These questions describe what a complete local model of the method would make
answerable. Thesis success does not require all of them to be answered.

- What degeneracy regime is a point in, measured by how many sigmas are close
  to the minimum action?
- Which sigmas, branch domains, or branch germs are relevant near a point?
- Over what radius does a local model predict `sys(a0 + t d)`?
- When is single-branch ascent enough?
- When is near-active multi-branch ascent needed?
- How do branch-domain assumptions fail or remain harmless in practice?
- What does a local maximum look like on the quotient/transversal slice?
- How does convergence behavior vary with degeneracy?
- How much runtime and how many exact evaluations are needed by regime?
- Which apparent failures are optimizer bugs, diagnostic bugs, branch-domain
  issues, or genuine geometry?
- Are there counterexamples where the endpoint diagnostic passes but an
  improvement exists?

### Questions Useful To Answer Now

These questions are expected to move thesis success forward when answered, even
partially. A non-answer can also be useful if it narrows the next experiment or
records a real obstruction.

- What known failure mode makes current ascent endpoints non-local-maximal?
- Can near-active multi-branch ascent climb narrow-gap ridges better than
  single-branch ascent?
- Which branch-window tolerance policy gives stable enough behavior without
  hiding relevant sigmas?
- Which adaptive, tiny, or finite step rules avoid ridge and boundary stalling?
- Can random-start ascent reach high-degeneracy endpoint candidates within
  reasonable budgets?
- Do produced endpoints pass the chosen local-stability diagnostic?
- Do local probes or adversarial perturbations find improvements after the
  method stops?
- What trace fields are needed to debug failures and make thesis claims
  checkable?
- Does rerunning fixed-`F` datascience with the fixed method change the
  hostile-landscape result?
- Which final ablations explain retained design choices without becoming a
  broad optimizer survey?

### Prioritized Overlap

Most paths to success should prioritize questions in the overlap:

1. degeneracy regime;
2. near-active branch selection;
3. ridge/cusp step behavior;
4. endpoint local-stability diagnostics;
5. compute budget by regime;
6. failure classification.

These are current high-value axes, not a restriction on allowed thinking.
Other questions may become higher value after early results narrow the
hypothesis space.

## Artifact Roles

Artifacts should be named by what they are, not by how they affect belief.

- **Run trace:** per-iteration optimizer state, branch set, direction, step,
  stop reason, and failure mode.
- **Branch-set diagnostic:** action gaps, tolerance windows, near-active sigma
  counts, selected branches, and branch-selection failure statuses.
- **Branch cartography:** paired records for a selected `a0` and nearby
  sampled points, including target best-sigma visibility, transition changes,
  candidate-window misses, and branch-domain failure classifications.
- **Local geometry probe:** evaluated behavior of `sys(a0 + t d)` near selected
  points, with direction and radius selection recorded.
- **Endpoint diagnostic:** checks bearing on whether an endpoint is a local
  maximum on the quotient or on a stated weaker endpoint condition.
- **Stress test:** targeted attempt to find an improvement, missing branch,
  branch-domain failure, or diagnostic/model mismatch.
- **Compute-budget report:** runtime, exact-evaluation counts, cache behavior,
  failure counts, and scale-relevant resource use.
- **Method comparison:** controlled comparison between retained variants or
  ablations, used only when it bears on retained method choices.
- **Thesis-claim packet:** cleaned reproducible subset of results with
  observations, caveats, failed checks, accepted weaker outcomes, and the claim
  wording it permits.

## Local Progress During Development

A development step makes local progress when it changes at least one of these
states:

- narrows a core question about degeneracy, branch selection, step behavior,
  endpoint diagnostics, compute budget, or failure classification;
- produces a reproducible diagnostic artifact on real `sys(a)` data;
- finds a contradiction, counterexample, or failure mode that prevents a false
  method claim;
- improves the adversarial-check surface, for example by making it easier to
  search for post-stop improvements or missing-branch behavior;
- removes a misleading artifact or marks it clearly as setup, historical, or
  not claim-bearing;
- makes the next real experiment cheaper or less ambiguous.

A development step does not make local progress merely because it compiles,
writes synthetic artifacts, improves `sys` on a few starts, adds a new variant
name, or lengthens the pro side of a method argument without comparably checking
failure modes.

## Readiness To Ask For Promotion

The method is ready to ask whether it should leave active development when:

- the retained method is named and documented by algorithm, tolerances, stop
  reasons, expected failure modes, and compute budget;
- endpoint diagnostics have run on a documented retained sample, with sample
  rule, exclusions, and known biases written down;
- traces cover the degeneracy regimes encountered by the search pipeline or
  explain which regimes remain untested;
- compute-budget reports are enough to plan, run, or reject fixed-`F`
  datascience reruns;
- retained design choices have at least minimal ablation results, failure-mode
  results, or cost reasons;
- known bad-old-ascent failure modes are fixed, outside the retained claim, or
  explicitly still present;
- unresolved risks are listed next to positive results, including risks that
  would weaken thesis wording;
- downstream integration points are named: what code moves to
  `exp-sys-landscape` or crates, what artifacts move to analysis or
  thesis-claim packets, and what remains development-only.

Readiness to ask for promotion does not mean all complete-model questions are
answered. It means unanswered questions no longer block the retained thesis
method claim or the next downstream rerun under the recorded caveats and
Jörn/Kai judgment.

## Out Of Scope Unless Reopened

- A general semialgebraic branch/germ certificate for arbitrary endpoints.
- Theorem-strength local maximality for generic non-HKO endpoints.
- Broad optimizer surveys whose variants do not bear on retained method
  choices.
- Runtime profiling that belongs in `experiments/performance/`.
- Solver correctness/regression checks that belong in `experiments/verification/`.
- Derivative or numerical-error validation that belongs in
  `experiments/dev-quadratic-program/numerics-audit/`.

## Dangers Of This Charter

This charter is meant to prevent scope collapse, but it can itself be misused.
Future agents should watch for these failure modes:

- Treating the desired property as a proof obligation for every generic
  endpoint. The charter names the real target first, but the suite may still
  succeed with weaker outcomes if Jörn/Kai accept the resulting claim strength
  and caveats.
- Treating the weaker-outcome examples as a whitelist. They are examples of
  outcome shapes, not authorization to accept any result matching their surface
  form.
- Treating the question lists as a task queue. They are a map for choosing
  high-value work, not a requirement to answer everything.
- Treating artifact production as progress. A run trace, diagnostic, or report
  matters only when it changes what is known, what can be ruled out, what must
  be caveated, or what should be tried next.
- Treating adversarial checks as ritual. Failure-mode checks must be capable of
  changing the method, claim wording, or stop decision.
- Treating promotion readiness as automatic promotion. The charter defines when
  to ask whether to promote; it does not approve promotion or merge by itself.
- Treating the charter as more authoritative than source truth. Code behavior,
  generated outputs, proof artifacts, retained thesis text, and Jörn/Kai
  decisions overrule stale charter text.

## Current Partial Setup State

The current package contains:

- a schema-smoke command for artifact shape;
- a real-data branch degeneracy diagnostic;
- a branch-cartography reference surface for paired `(a0, data(a0))` and
  `(a, data(a), relation_to_a0)` records across finite perturbation scales;
- a local geometry probe that emits run traces, endpoint diagnostics, endpoint
  direction scans, and compute-budget reports;
- split-run selectors for resumable retained-sample checks;
- summary aggregators for split runs;
- endpoint-scan and run-trace reports that inspect detailed JSONL rows;
- a trace-policy sweep for cheap threshold reclassification.

The current method candidate is
`iterative_observed_multi_direction_probe`, documented in
[METHOD-CANDIDATE.md](METHOD-CANDIDATE.md). It tries all locally generated
directions and finite trace steps, including directions whose local branch
prediction is negative, and accepts only after recomputed `sys` improves above
the effective threshold
`max(min_observed_delta, min_observed_relative_delta * abs(base_sys))`.

The current retained panel has two fixtures in each degeneracy regime
(`large_gap`, `narrow_gap`, `high_degeneracy`). It found no above-threshold
post-stop endpoint direction-scan row at relative threshold `1e-3`, but it did
find positive-below-threshold rows. The largest positive row is about `0.756`
of the effective endpoint threshold. This supports the current finite endpoint
condition on a small retained panel; it is not a local-maximality certificate.

The current promotion decision packet is
[PROMOTION-READINESS.md](PROMOTION-READINESS.md). It states the retained
candidate, endpoint condition, evidence, caveats, unresolved risks, downstream
integration points, and the decision reserved for Jörn/Kai. Its conclusion is:
ready to ask for a promotion decision, not ready to mark this charter complete.
It is not the current scope controller for branch-cartography or the
local-to-global branch-behavior study.
