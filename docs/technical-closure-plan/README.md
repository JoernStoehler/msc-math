# Technical closure: bounded implementation plan

18 September 2026. Planning from research/review-workflow-design at e33c3f0f,
with direct inspection of the interrupted production checkout at
`/tmp/msc-math-thesis-review-20260917`. This is not a restart of production,
a proof acceptance decision, or a request to complete every open research route.

## Decision

The inspected obligations mostly concern proof placement, precise guarantees,
and integration. No newly demonstrated central proof gap emerged. That statement
is narrower than a full mathematical audit: the QP bridge and flow/variation
boundaries were inspected, while accepted HKO and product results were not
reproved. Numerical implementation support needs a finite correspondence gate,
not a new solver-verification project. Historical DS targets need their own
provenance qualification; current certification does not transfer backwards.

Execute four bounded assignments below in parallel after selecting their exact
source versions. Their initial checks can run during writing-workflow research.
Actual prose changes belong to coordinated production, not competing worktrees.
Estimated times are agent-work judgments, not measured runtimes or a promised
critical path. If a decisive check fails, report the exact broken implication
before expanding scope.

## QP: make the available argument explicit

**Directly inspected by delegated scout:** interrupted `04-quadratic-program.tex`
currently attributes the headline formula to HK around lines 94–112. Its later
lines 120–159 construct a dual curve from every positive feasible q with
A=I=T=1/(2q); earlier assembled duality gives d=c. Thus c<=1/(2q), hence
q<=1/(2c). The simple minimizing orbit theorem and the converse construction
supply equality. There is a present argument, not an identified missing theorem.

**Assignment Q:** derive the formula from those two directions in reader order;
keep HK attribution and the imported duality/simple-minimizer foundations
explicit. Move or include the short construction before its use. Trace J/sign,
normalization and factor two across the selected preliminaries, orbit chapter
and QP chapter. The six-facet product theorem/algorithm/proof ordering is already correct in
the interrupted draft (lines 455/464/494); do not schedule that as an unfixed
request. Its earlier twelve-facet algorithm could optionally move before its
proof, but that is a different editorial choice. Explain that support reduction computes the scalar value; it
need not preserve all physical orbit representatives or derivatives.

**Stop when:** a reviewer can follow both inequalities without invoking the
headline formula being proved; all cited earlier results are actually assembled;
normalizations agree; claimed algorithm consequences follow from stated results.
Estimate 1–2 agent-hours including independent check. If the selected replacement
preliminaries no longer provide the duality result, restore that dependency or
state the actual imported foundation; do not silently invent a new independent
proof of all symplectic duality. Full reduction reaudit is unnecessary absent a
specific contradiction.

## Flow: retain the theorem and describe its implementation separately

**New source checks:** interrupted `05-flow-graph.tex` lines 195–225 states
nonzero pairings, independence up to four normals and finite-orbit regularity;
it rejects an empty closed tube before solving its fixed point. The feared
empty-domain control-flow omission is already repaired in that draft. Lines
397–415 distinguish rational implementation inputs and bounded comparison
examples. Production `main.tex` still selects the recovered flow chapter, so
this prepared repair is not automatically in the PDF.

**Assignment F:** compare the chosen chapter's theorem hypotheses/pruning/
termination chain to `formal/flow-graph-real-algorithm.tex` and the runtime
boundary in `crates/symplectic/src/algorithms/flow_graph/README.md`. Preserve
conditional correctness and chamber-relative genericity, both retained scope.
Check the specific singular/empty/short-word cases and action cutoff semantics
against `exact_search.rs` and `exact_tube.rs`. Use the existing proof-risk packet
and focused regression entry points if integration or source drift warrants it.
Do not rerun its `--full` producer without inspecting writes and cost.

**Stop when:** all theorem predicates and actual runtime rejections are located,
with no claim that runtime validates the entire theorem domain or that scalar
QP agreement validates word-level correspondence. Do not add support for
products, zero-pairing inputs, algebraic exact search, retired f64 FG or rotation
pruning. Estimate 1–2 hours; a theorem/code mismatch relevant to an asserted
capacity result is an escalation, not permission for broad implementation work.

## Variation: preserve the right distinction, avoid a false prerequisite

**New direct check:** recovered `06-variation.tex` lines 245–270 makes a smooth
finite-envelope formula conditional on complete local coverage and persistent
gaps. Lines 316–344 expressly exclude claiming an arbitrary non-generic
first-order theory. Lines 346–354 explain HKO's feasible-section upper functions;
HKO does not need continuation of optimizing branches or a complete catalogue
of every nearby germ. The April draft
`formal/sys-first-order-local-behavior.md` concerns that broader unresolved
problem and reads older HKO routes. It must not be used to reopen the accepted
feasible-section proof.

**Assignment V:** check formula conventions and the interface from generic
branch derivatives to numerical search and HKO upper bounds. Map the described
finite-difference and exact/f64 checks to `crates/symplectic/src/derivatives.rs`,
`src/exact/derivatives.rs`, and `src/algorithms/hk2017/tests_capacity_derivative.rs`.
Those tests exist; this planning task did not execute them. Reconcile older
12-start illustration with the DS packet's newer 64-start/448-run evidence if
the latter is adopted, retaining evaluator provenance and finite-step scope.

**Stop when:** generic theorem, search heuristic and HKO certificate are distinct
and every reported empirical validation has an owner. Estimate 0.5–1 hour;
differentiating a minimum-support value reduction as if it preserved every
branch would be a substantive defect requiring a focused correction. A new
semialgebraic germ classification is not a thesis prerequisite.

## Numerics and DS: establish the actual guarantee at each boundary

**Inspected owner surfaces:** `docs/capacity-calculation-map.md`, general/product
algorithm tool results, numerical chapter, and the selected-route correspondence
test. The retained 249-system, 88/1271-word and 88-product counts describe the
selected development routes. They are not by themselves a demonstration of
production-code equivalence. The finite gate already exists at
`experiments/dev-quadratic-program/tests/selected_route_correspondence.rs`.
Run it with `cargo test -p exp-dev-quadratic-program --release --test selected_route_correspondence`.
It covers general simplex/hypercube bounds, an F5 exact action-window case,
F10 derivatives and three product certificates. This task did not run it.

**Assignment N:** create a compact claim→formal lemma→selected route→production
entry point→comparison/test map for the claims actually retained in the chapter.
Check source drift, then run the named correspondence gate if applicable.
Verify that the exact KKT witness is used only for its proved feasibility/value
purpose; physical orbit placement, fixed-word optimality and exhaustive outer
support coverage are separate implications. Check exact binary64-target input,
interval/indeterminate contracts, fallback semantics and final scalar enclosure.
Do not expand into complete public-solver formalization unless retained claims
actually require that standard.

Name `capacity_4d` for the general certified guarantee rather than implying
all Rust entry points share it. The chapter calls a 2.31e-14 discrepancy a
reported midpoint discrepancy, while its source RESULTS says printed central
value; verify the underlying output or retain the source wording.

Remove the numerical chapter's obsolete assertions that the *current* pentagon
proof uses Sage; preserve the genuine HKO Sage role and any accurately described
historical exploration. This is an identified integration edit, not applied to
the paused draft by this planning agent.

**DS boundary:** the independently completed packet at
`/workspaces/msc-math/.worktrees/ds-evidence-closure/docs/ds-evidence-closure/README.md`
reproduces the historical association with its original 39-feature analyzer.
It does not recover actual historical evaluator execution receipts. Do not
label the old table/optimizer targets certified by current code. Legacy
`capacity_auto` branch windows remain different from certified scalar capacity;
current panels with enclosed capacity and f64 volume do not certify ratio
intervals. Source geometry/current 45-feature rebuild is not needed merely to
report the historical association. A frozen stratified re-evaluation tests
historical-target reliability; a fresh independent population tests transfer.
These are different optional studies, neither automatically required.

**Stop when:** every thesis numerical claim uses its actual guarantee, selected
copy evidence is correctly attributed, and the bounded production gate passes
or its concrete discrepancy is exposed. Estimate 1–2 hours plus compilation.
A failure involving outer enumeration completeness or interval soundness can
invalidate this estimate; do not conceal it with broad prose weakening.

## Dependencies, ordering and integration handoff

- Q and V share notation, so freeze conventions and cross-review their boundary.
  F depends on the simple-minimizer statement, not on a rewritten QP proof.
- N can run independently; DS source recovery has already completed and should
  be reused, not repeated. A historical-data rerun is not needed for N.
- Assembly owner should decide the actual chapter versions first. Final
  cross-references and rendered explanations are integration work after these
  bounded content checks, not mathematical proof evidence.
- HKO core replay was **reported earlier**, not rerun here. Preserve its accepted
  feasible-section argument and named certificate; check final source/listing
  identity when assembling. No new global HKO audit is scheduled.
- Analytic pentagon and restricted ridge-law support are prepared. Coordinate
  conventions through the DS packet; do not regenerate obsolete exhaustive
  branch certificates for an analytic proof.
- Existing production modifications remain uncommitted in their own checkout;
  they are not modified or silently adopted here.

## Incidental correction and evidence limits

Commit `decce652` only fixes the flow README's nonexistent old thesis source
pointer, routing through the recovered file and active main. No theorem, code,
producer output or thesis prose changed. The plan is a separate commit.

This planning pass performed source inspection and delegated source scouting,
not builds, Rust tests, exact producers or mathematical acceptance review.
The useful output is a bounded assignment map and explicit stopping rules,
not a new global PASS label. No secret contents were read.
