# Trustworthy Computation Pattern Inventory

Use this reference when Rust work involves exact/f64 agreement, certified
predicates, numerical error bounds, fallback, profiling, observability, or
proof/code correspondence.

This is an ordered orientation list of standard patterns GPT-5.5 already knows.
It is not a recommendation table. For a concrete task, decide separately
whether a pattern is useful, harmful, too costly, or merely adjacent.

The ordering is by likely relevance for proof-backed numerical Rust work in
this thesis project.

Use this as a poor man's brainstorm seed, not as sufficient search or
assessment. The list is medium-effort babble over standard patterns, low-effort
pruning, and only repo-contextual rather than task-contextual. For high-value
work, gather the task context first, add task-specific and custom strategies
when returns justify it, and spend real effort assessing value, cost, interaction
effects, and combinations.

- **Differential testing:** run two implementations on the same inputs and
  compare their outputs. Here the typical pair is an f64 implementation against
  an exact/reference implementation.
- **Reference implementation / oracle testing:** keep a simple or trusted
  implementation whose main job is semantic truth rather than speed. Tests and
  audits use it as the oracle for expected behavior.
- **Result / error enums:** represent success and non-success with explicit
  variants instead of ambiguous `Option`, sentinel values, or stringly failure
  modes. This pattern makes failure causes part of the ordinary data model.
- **Domain enums / algebraic data types:** represent domain states as explicit
  variants such as `True`, `False`, `Indet`, or certificate states. Exhaustive
  matching then forces callers to confront each case.
- **Table-driven / data-driven analysis:** put inputs, observed values, or
  candidate-policy rows in data, then run the same analysis logic over the
  table. This is useful when many hypotheses or policies should be compared on
  the same evidence.
- **A posteriori error estimates:** compute an error estimate from the actual
  numerical result and problem instance, rather than relying only on an a priori
  worst-case bound.
- **Residual-based certification:** use the residual of a computed solution,
  together with information about the operator being solved, to certify a bound
  on the distance to an exact solution.
- **Interval enclosures / interval arithmetic:** represent a value by an
  interval guaranteed to contain the exact value. A predicate can be decided
  only when the whole interval lies on one side of the decision boundary.
- **Exact fallback:** route inputs that cannot be certified by the fast or
  approximate path to an exact implementation. The fallback is part of the
  algorithm, not merely a debug tool.
- **Property-based testing / fuzzing:** generate many inputs and check that a
  property holds. These methods search for counterexamples across a broad input
  space.
- **Counterexample-guided refinement:** use failing examples to refine the
  theorem candidate, implementation, instrumentation, or test generator. The
  counterexample remains part of the evidence base.
- **Design by contract / runtime contracts:** state a function's required
  inputs, guaranteed outputs, and allowed non-success cases. Runtime contracts
  check some of these obligations while the program runs.
- **Smart constructors:** create values with invariants only through
  constructors that check or establish those invariants. Code outside the
  constructor cannot fabricate the certified state directly.
- **Make illegal states unrepresentable:** choose types and APIs so invalid
  combinations cannot be expressed, or require an explicit escape hatch. This
  moves some correctness work from runtime checks to representation design.
- **Exhaustive pattern matching / total functions:** use enums and match
  expressions so every case must be handled. A total function defines behavior
  for all inputs in its declared domain.
- **Golden / regression tests:** store important known inputs and expected
  outputs so later changes cannot silently change behavior.
- **Metamorphic testing:** check how outputs change under input transformations
  where the mathematics predicts a relation, such as permutation, scaling, or
  relabeling.
- **Structured logging / observability:** emit machine-readable events, spans,
  metrics, or logs that let developers inspect what happened during execution.
  The pattern is about making runtime behavior queryable, not about proving
  correctness by itself.
- **Strategy pattern / policy enum:** represent interchangeable algorithms or
  policies as named choices. This is useful when the same data should be
  evaluated by several candidate decision rules.
- **Separation of concerns:** keep different responsibilities in different
  functions, modules, or data paths: for example, diagnostics, production
  certification, exact fallback, and report generation.
- **Newtype pattern:** wrap an existing type in a distinct domain type so the
  compiler distinguishes values that have the same representation but different
  meaning.
- **Units-of-measure types:** use types to distinguish quantities with different
  physical or mathematical units, dimensions, or coordinate systems.
- **Assertions / defensive programming:** use runtime checks to fail early when
  an internal assumption is violated. Assertions are most natural for
  programmer errors or impossible internal states.
- **Invariant checking:** check that maintained conditions still hold at
  selected program points, such as finite values, compatible dimensions, sorted
  intervals, or normalized data.
- **Shadow execution / dual implementation comparison:** run a secondary
  implementation alongside the main one and compare results. This is
  differential testing as an execution mode.
- **Mixed-precision algorithms:** combine different numeric precisions or
  arithmetic domains in one algorithm, such as f64 for speed and exact
  arithmetic for fallback or certification.
- **Backward error analysis:** analyze a computed result as the exact solution
  to a nearby problem. This can explain or certify numerical behavior when the
  nearby problem is acceptable.
- **Condition number analysis:** study how sensitive an output is to input
  perturbations. High condition numbers explain why small numerical errors can
  cause large output changes.
- **Verified numerics:** use numerical methods that produce mathematically
  certified enclosures or proof-backed results, often combining floating-point
  computation with interval or residual certificates.
- **Proof-carrying code, lightweight version:** represent a runtime result
  together with evidence that justifies using it, while the proof that the
  evidence is sufficient lives in code comments, formal notes, or theorem
  references.
- **Executable specifications:** write a direct executable version of the
  intended behavior and compare optimized or approximate implementations
  against it.
- **Parse, don't validate:** convert raw input into a domain representation that
  already encodes the checked properties. Later code consumes the parsed
  representation rather than repeating validation.
- **Typestate pattern:** encode protocol or state transitions in types, so
  operations are available only after the value has reached the required state.
- **Phantom types:** attach compile-time markers to a type without changing its
  runtime representation. Common uses include state markers, unit markers, or
  provenance markers.
- **Witness objects / certificates:** store explicit evidence that a claim or
  permission is valid, such as a residual bound, inverse bound, validation
  token, or proof witness.
- **Lightweight formal methods:** use practical formal techniques such as
  contracts, model checks, invariants, executable specs, or proof notes without
  committing to a full proof-assistant workflow.
- **Refinement types:** use types that include value-level predicates, such as
  positive numbers or non-empty arrays. Languages without native refinement
  types usually approximate them with smart constructors.
- **Ghost variables / ghost state:** introduce variables used for specification
  or proof but not needed by the runtime algorithm. In ordinary Rust these
  usually appear as comments, debug-only fields, or audit-only data.
- **Dependent types:** use types that depend on runtime values, allowing some
  propositions about values to be checked by the type system. Rust does not
  natively provide this pattern.
- **Mutation testing:** make small artificial changes to the code and check
  whether tests fail. This estimates whether tests are sensitive to the bugs
  they are meant to catch.
- **Feature flags / experimental flags:** use named compile-time or runtime
  switches to isolate experimental behavior, optional instrumentation, or
  alternate implementations.
- **Builder pattern:** construct complex values through a staged configuration
  object, especially when there are many optional settings.
- **Adapter pattern:** wrap one interface so it can be used through another
  expected interface.
- **Facade pattern:** expose a small, simple interface over a more complex
  subsystem.
- **Capability pattern:** represent permission to perform an operation by
  possession of a value. Code can only call privileged operations if it has the
  capability.
- **State machine pattern:** model a process as explicit states and transitions.
  This can make allowed control flow visible when an algorithm has multiple
  phases.
- **Null object pattern:** represent absence with an object that implements the
  same interface and has neutral behavior. This can simplify callers, but also
  risks hiding absence when absence matters.
- **Sentinel values:** use special values such as `-1`, `NaN`, `inf`, or magic
  strings to encode nonstandard states. This pattern is common but can obscure
  which state is being represented.
- **Full dependent-type / proof-assistant implementation:** encode definitions,
  algorithms, and proofs in a proof assistant or dependently-typed language so
  a checker verifies the proof obligations.
