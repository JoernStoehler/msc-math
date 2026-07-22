# Sys-Datascience Research Closeout

Date: 2026-07-22.

Status: current routing addendum to
`final-research-account-2026-07-12.md`. The earlier account still owns its
reviewed random/product claims. This file records the later known-seed work,
unpromoted pilots, portfolio decisions, and current research frontier. It is a
decision and navigation owner, not an experiment-metric owner or launch queue.

The exploration phase is closed for allocation purposes, not because the
landscape is understood. New data-science work must beat the current thesis
integration work or another research line after charging implementation,
review, interpretation, and delayed-integration cost.

## Later Evidence And Decisions

### SD-LM-R1: five known `sys >= 1` cases

Kind: reviewed empirical result and allocation decision.

The root `experiments/local-maxima-check/` packet compares three conjectural
equality local maxima, the proved HKO local maximum, and the rotated-pentagon
crossing. The finite screen recovered the theory-derived improving pentagon
direction and found no material improvement at the triangle--hexagon,
square--square, or Chaidez--Hutchings targets.

This supports conjecturing fixed-facet local maximality at the three equality
targets. It does not prove it: generic quotient/random probes can miss the
known pentagon improving family, and the target capacity intervals are too
broad for interval-separated target conclusions.

Decision `LMC-D1` in the owner README closes further allocation to generic
known-seed probing. Reopen only for a new independently motivated `sys >= 1`
seed, a materially more complete/exact local branch model, a stronger negative
method, or a named theorem-level thesis use. This is not an instruction to
ignore later evidence; it records why another finite probe presently costs
more than its expected update.

### SD-CH-R1: exact Chaidez--Hutchings fixture

Kind: reviewed exact computation.

`experiments/verification/ch2021-six-vertex/` exactly reproduces
`F=9`, `V=1/8`, `c_EHZ=1/2`, and `sys=1`. Its exhaustive word computation has
141 exact minimizers. Nine of the eighteen actual two-faces are Lagrangian, so
the ordinary Chaidez--Hutchings combinatorial-flow formulation is not
well-posed for this body, while the HK method remains applicable. The owning
packet contains the reproducible command and claim boundary.

### SD-FO-T1: fixed-normal first-order balance theorem

Kind: agent-proved mathematics; independently agent-reviewed; not Jörn-reviewed.

The proposed merge candidate promotes the proof developed on
`facet-coverage-lemma`. In a stable fixed-normal support chamber, it derives a
directional formula for `c_EHZ` and a convex-balance
necessary condition for local maximality of `sys`. Consequences include:

- every facet must occur in at least one minimizing orbit, although the orbit
  may depend on the facet;
- an uncovered facet supplies an explicit first-order improving support
  perturbation;
- coverage alone is weaker than the full convex-balance condition; and
- an interior balance is a strict first-order certificate on the support
  slice, while boundary balance leaves higher-order behavior open.

The scope excludes arbitrary normal perturbations and nonsimple/chamber
transitions. The independent review repaired a word-action sign error and an
overstated example claim. The theorem is the strongest mathematical candidate
from this cycle, but it must remain marked agent mathematics until Jörn reviews
it. Merging the formal note preserves the proof; it does not accept it as
thesis mathematics.

### SD-HKO-P1: transverse-ray pilot

Kind: technically reviewed pilot; reproducible packet proposed for promotion.

The packet developed on `sys-hko-rays-run` evaluates 32 frozen directions in
one concrete 25-dimensional affine slice transverse to the chosen symmetry
directions at HKO. Every sampled ray had a nominal above-to-below transition;
midpoint radii in the chosen coordinate norm ranged approximately from 0.044
to 0.112, with median approximately 0.078. No sampled re-entry or chart/gauge
failure occurred.

This is finite, coordinate-dependent evidence about typical sampled
directions. It is not an inradius, a global quotient construction, a
star-shapedness claim, or evidence against thin or lower-dimensional
connections. This merge candidate promotes the isolated producer, frozen
basis, manifests, evaluations, and summaries rather than only citing a branch
hash. The packet is durable empirical evidence if merged; a reader-facing
thesis figure remains a separate value and presentation decision.

### SD-OR-P1: regular 3-by-6 orientation pilot and retention failure

Kind: bounded negative pilot plus reusable correctness finding; not promoted.

The unmerged `sys-orientation-run` branch found no `sys > 1` case among four
target-blind orientations of one regular 3-by-6 equality body. This disfavors
an easy broad positive orientation region for that frozen panel, but does not
test a narrow local equality cone and is not a useful standalone thesis claim.

The run also exposed a more general correctness issue: its theorem frontend
enumerated 373 candidates, while the floating-point retention stage kept 129,
and the exact capacity lay just beyond a residual-derived interval endpoint.
The branch adds a complete-supplied-stream exact-solve API and tests. This can
remove one avoidable source of false certification, but cannot establish that
the supplied stream was itself complete. Extracting that API is a separate
library-correctness decision; it should not be bundled with the negative
orientation pilot.

### SD-EQ-P1 and SD-FSO-P1: disposable feasibility pilots

Kind: unpromoted exploratory work.

- `branch-equality-manifold` demonstrates Newton correction onto equality of
  two selected candidate actions in one restricted pentagon-product chart.
  Third candidates frequently undercut the selected pair, and the run found no
  `sys > 1` case. It is useful implementation experience, not evidence for a
  broad manifold sampler or a selected optimization program.
- `fixed-shape-orientation-search` found some orientation-dependent improvement
  below one but no new positive example. Its proposed successor was withdrawn
  after portfolio review.

Neither result currently justifies maintenance and review of its exploratory
implementation. Their scientific dispositions are recorded here; their code
is not a selected merge dependency.

## What The Combined Evidence Changes

- The known-seed route supplied a useful comparison and conjectures but no new
  `sys > 1` family. The positive pentagon control demonstrates why finite
  random misses cannot certify a local maximum.
- HKO should be treated locally through a transverse slice, while empirical
  rays describe only the chosen finite panel. More IID rays would add little;
  a continuation would need either a proof-oriented certified boundary or an
  adversarial thin-connection question.
- Existing finite-gap-aware local prediction under
  `experiments/dev-gradient-ascent/` and `experiments/dev-sys-prediction/`
  should be extended or tested rather than reimplemented under a new name.
- Low ridge magnitude remains a useful sub-threshold enrichment family, but
  lower is not an optimizer objective and the extreme tail has already failed
  a hardening gate. Repeating the same scalar at larger scale has low expected
  information value.
- Symplectic alignment can matter at fixed Euclidean shape, but present pilots
  do not identify a broad favorable orientation region or a causal scalar
  mediator.

## Thesis Uses

Current candidate uses, in priority order:

1. Keep the merged five-case local-maxima comparison in the bounded-search
   account. Its evidence status and placement are already adequate.
2. After Jörn's mathematical review, adapt `SD-FO-T1` into a compact theorem in
   the first-order perturbation discussion. Preserve the complete argument in
   `formal/`; do not infer thesis acceptance from agent review.
3. Use the measured July research episode to replace abstract AI-workflow prose
   with a concrete cost-versus-result example if it improves the existing
   AI-use discussion. The episode produced useful scientific and correctness
   findings, but its USD 371.25 audited shadow API cost exceeded the
   experiment-agent allocation; most cost was agent/context/review work rather
   than computation. A thesis passage must distinguish observation, causal
   interpretation, and later workflow choice.
4. Consider `SD-HKO-P1` only if a small quantitative panel helps the reader
   understand the scale and anisotropy of the chosen local slice. The arbitrary
   coordinate norm and inability to exclude thin connections must be visible.
5. Use the complete-stream retention failure only if a concrete example makes
   the numerical chapter's existing warning easier to understand. The chapter
   already states the conceptual limitation.

The equality-manifold and fixed-shape pilots, four-point 3-by-6 negative, and
exploratory code volume are not thesis results at current evidence.

## Current Research Frontier

These are assessed idea families, not approved experiments:

1. **Diverse fixed-shape alignment.** Compare genuinely different Euclidean
   shapes under symplectic orientation, rather than treating one exact product
   shape as the research object. Value would come from distinguishing
   Euclidean-shape limitations from alignment limitations.
2. **New invariant families.** Audit whether all-pairs symplectic Gram data,
   Williamson covariance ratios, or affine baselines add information beyond
   facet/source buckets and ridge summaries. Test at most the first two
   nonredundant families that survive a target-free audit.
3. **Capacity versus volume response.** Existing traces may reveal whether
   high `sys` changes are capacity-driven, volume-driven, or compensating. This
   can generate better hypotheses without new target evaluations.
4. **First-order balance screen.** If `SD-FO-T1` is accepted, evaluate its
   convex-balance obstruction on known/generated candidates. Its value is
   conditional on a computationally stable representation of the active
   weights and on the theorem's scope matching the candidate chart.
5. **Orbit-aware shallow truncation.** A deliberately risky intervention that
   could test whether removing nonessential facets preserves or improves the
   active mechanism. It requires strong exact/control checks and currently
   ranks below the existing-data routes.

Further idea generation remains valuable because discovery value is
heavy-tailed. This list is a compact current comparison surface, not a claim
that the best future idea has already been generated.

## Stop, Defer, Reject, And Finish

These words describe different cost decisions:

- **Finish now** when delaying would cause more expected reconstruction,
  integration, or correctness cost than completing the bounded work while its
  context is live.
- **Defer** only when waiting lowers expected total project cost while
  preserving enough state to resume. State the event that would reverse the
  comparison and the preservation needed to avoid rediscovery.
- **Stop/close** when no currently plausible outcome repays another stage, but
  record evidence that could change that conclusion.
- **Reject** when the proposed question or comparison would not support the
  intended decision even if executed successfully.

Accordingly:

| Work | Decision now | Why this lowers expected total project cost | Cost of waiting / reversal condition |
| --- | --- | --- | --- |
| Known-seed finite probing | close | another finite miss adds little after the pentagon false-negative control | reopen for a stronger method, new seed, or theorem use |
| Fixed-normal theorem | finish review/promotion as a separate candidate | mathematical context and repaired proof are live; losing them would cause expensive reconstruction | Jörn may reject or request repair; do not write theorem-strength thesis prose first |
| HKO ray pilot | promote the reproduction packet; defer thesis presentation | review was already performed and future reproduction is plausible, so preserving producer and data now costs less than reconstructing them later | reopen the thesis-display decision when the HKO chapter needs a quantitative local panel |
| Exact complete-stream API | defer crate extraction | Main already has an experiment-local exact-all-visited-sigma route; another public API adds review and maintenance before a named consumer | reopen when a crate consumer cannot reasonably use the existing route or needs the certified aggregation types |
| Equality-manifold prototype | stop | present restricted result did not select a scientific successor; polishing would maintain an unselected chart | reopen only for a named equality-sampling or optimization consumer |
| Fixed-shape orientation prototype | stop | no positive case and successor lacked portfolio advantage | reopen for a diverse-shape comparison with a specified decision |
| Quick ridge atlas | do not promote | it mostly reproduces the durable enrichment/hardening conclusion and is cheap to recompute | promote only if a reader-facing plot is selected |
| Session-cost script | separate harness review | useful for future budget control, but frozen skill changes require an exact independent gate | delay risks another opaque budget overrun; candidate already exists on `session-cost-script` |

## Workflow Lessons With Evidence

The July episode supports this current operating hypothesis:

- gate the scientific question and material cost before implementation;
- run the smallest scientifically valid pilot;
- separate execution from interpretation and successor selection;
- promote only the result whose later consumer repays reconstruction and
  maintenance;
- keep mathematical completeness, controls, frozen selection, provenance,
  invalid/unavailable states, claim boundaries, and serious review even in a
  disposable pilot;
- postpone generic APIs, schemas, attestation, and publication polish until a
  consumer justifies them;
- compare delegation against doing the work in the context-owning session,
  including transfer, cached-input, misunderstanding, and review cost.

“Throwaway” means disposable implementation, not disposable validity. A clean
rewrite should receive the question, frozen inputs, controls, and known failure
regressions; when independence matters, do not coach it with expected numeric
answers or an architecture merely because the prototype used one.

Code length has no independent sign. The objective is the lowest expected
total cost of a trustworthy scientific answer, including later review,
debugging, reproduction, integration, and maintenance.

The audited research episode is recoverable from Codex session
`019f706b-45c4-70b0-aada-7edc8d45c292`; its cost reconstruction used the
project's recorded cached-input pricing formula. Session history is process
evidence, not a substitute for scientific artifacts.

## Next-Agent Route

1. Read this addendum, then use the earlier final account only for the detailed
   random/product evidence it owns.
2. Read `research-ledger.md` for the compact current belief state and
   `next-session-candidates.md` for current gates.
3. Follow `portfolio-review-contract.md` before proposing nontrivial new
   experiments.
4. Treat the promoted HKO packet as bounded finite-panel evidence. Treat the
   remaining unpromoted pilot branches as dispositions, not accepted evidence
   or an implementation queue.
5. Ask Jörn about mathematical acceptance, thesis value, or another actual
   stakeholder crux only after locally establishing the evidence and cost
   comparison.
