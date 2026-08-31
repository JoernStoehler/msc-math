# Capacity/Orbit API Refactor Goal Guardrail

Date: 2026-05-04
Repo context: `/workspaces/msc-math`
Status: draft goal guardrail under active correction by Joern. This is not an
API target and not an implementation plan. Do not treat the objective as
accepted until Joern explicitly says it is correct.

## Read This First After Compaction

If the session was compacted, read this file before touching any capacity API
target draft. Do not reconstruct the goal from memory. Do not replace this file
with a paraphrase. Do not "repair" an API target by mixing in compatibility
layers or current implementation shapes.

The previous draft at `/tmp/msc-math-capacity-api-target.md` was invalidated.
Do not use it as the refactor target.

## Source Statements From Joern

These are the controlling statements. Future agents must not replace them with
a shorter paraphrase.

> Please first get up to speed, then discuss with me what the ambitious
> refactoring target will be - if we do another minor incremental change then
> we will never arrive at where we need to.

> Suggestion: we write a /tmp/*.md file with the whole new api surface we want
> to achieve (ambitious!) and indicate where symbols are similar/represent
> specializations of sth that would be awkward to abstract (e.g. the different
> "capacity from polytope" functions that only differ in numerical type & the
> final guarantees they give are basically very similar and should be grouped
> together therefor)

> Did you consider real consumers instead of imaginary ones?

> Polytope4D <- why is this in your ambitious api target?

> woah why are you aiming for the final surface to have compatibility layers?

Later corrections in this same discussion also belong to the guardrail:

- Joern stated that `Polytope4D` is an anti-pattern. Do not use it as a design
  anchor for the new API target.
- Joern rejected final-surface compatibility layers for the old
  private/internal/beta API.
- Joern objected when the draft's surface was too small and risked losing
  experiment-used functionality, specifically including geometric orbit
  computation.
- Joern objected to invented structures, option bags, aliases, and indirection
  that were not justified by real consumers.
- Joern objected to avoiding common crates for standard iterator/combinatorics
  work.

## Merged Objective

Get up to speed on the current capacity/orbit-related API and its real
consumers. Then discuss the ambitious refactoring target with Joern. The
working artifact for that discussion is a `/tmp/*.md` file describing the whole
new API surface we want to achieve, ambitiously.

That API-surface document must group symbols that are similar or are
specializations of the same idea, even when those specializations should not be
forced under one abstraction because that would be awkward or unclear.

The document must be based on real repo consumers and real mathematical or
computational operations. It must not be based on imagined external users, a
compatibility promise, or preserving the current API shape.

`Polytope4D` is not the objective. It is a rejected design anchor and must not
appear in the ambitious final API surface.

Compatibility layers are not the objective. They are rejected final-surface
machinery for this private/internal/beta codebase. Migration mechanics belong
in a separate section and must not shape the target surface.

## Constraints Established In Discussion

- Likely two scalar paths: `f64` and exact/algebraic via `OrderedField`.
  `BigRational` is an instantiation, not a separate rational endpoint.
- Avoid three top-level capacity entrypoints unless a real consumer requires
  them.
- Avoid option structs and aliases unless a real caller makes them net-positive.
- Use flat arguments for simple choices, including the enumerator.
- Keep pruned, unpruned, and billiard enumeration as real choices.
- Use a shared predicate type like
  `PredicateVerdict::{True, False, Indeterminate}` for approximate predicate
  outcomes.
- Do not use names like `decision` if they obscure the meaning.
  `Indeterminate` is the intended third state.
- Do not invent high-level capacity result enums that do not follow from orbit
  aggregation semantics.
- For f64 orbit aggregation, reason from orbit-level admissibility verdicts and
  action intervals:
  `min_action_lower` comes from lower bounds over true or indeterminate
  candidates;
  `min_action_upper` comes from upper bounds over true candidates.
- Do not add generic telemetry/audit bags to common result types. Custom
  consumers should compose lower-level parts and collect their own
  experiment-specific fields.
- Keep reusable lower-level building blocks for consumers that need custom
  capacity algorithms.
- Prefer iterator-based enumeration over visitor APIs.
- Use common crates such as `itertools` for standard combinatorics instead of
  hand-rolling iterator/state-machine boilerplate.
- Do not lose geometric orbit computation.
- Do not lose any other experiment-used capability while making the target
  ambitious.
- The surface should be broad enough to include retained operations such as
  capacity computation, sigma enumeration, one-sigma solves, exact
  certification, f64 numerical search, derivatives/subgradients,
  billiard-specific computation, geometric orbit reconstruction, and
  experiment/data-producing workflows where they consume reusable pieces.
- The phrase "capacity from polytope" in Joern's initial example is a prompt to
  notice and group similar existing symbols. It is not approval to keep
  polytope-container-shaped final APIs.

## Additional Lessons From The V2 API-Surface Iteration

These are now part of the objective guardrail. Future agents must not repeat
these mistakes after compaction.

- Define evaluation criteria before choosing symbols. The current criteria are
  real-consumer fit, low caller burden, mathematical contract first, no fake
  abstraction, simple Rust surface, correct performance shape, scalar split
  clarity, and final-surface purity.
- Compare alternatives explicitly before presenting a target as plausible.
  At minimum compare search entrypoints, search-data exposure, billiard
  enumeration shape, and current-consumer support.
- Get independent review before claiming the API target has been iterated
  properly. Use bounded subagents or equivalent independent passes for
  real-consumer fit and Rust API surface smells.
- Structure the target by consumer layer, not as one undifferentiated list of
  `pub` symbols. The current layers are public core API, public
  experiment-support API, public geometry support, private/internal building
  blocks, compared alternatives, rejected shapes, and open checks.
- Do not list only public symbols. Private/internal building blocks must be
  named so agents know what still exists without promoting them to polished
  public API.
- Do not duplicate signatures in multiple sections. Each public/support symbol
  should be declared once; later text may mention it by name.
- Do not invent one-field wrappers or type aliases for ordinary data such as
  `DMatrix<bool>`, `Vec<[f64; 4]>`, or `Vec<ReebSegment>`. Use the ordinary
  type unless a wrapper enforces a real invariant or simplifies a real
  consumer.
- Do not add placeholder implementation comments such as "owned lazy iterator
  state" to API target signatures. Use `impl Iterator` or a real named type.
- Do not expose implementation-only billiard block enumeration as polished
  public API. `billiard_blocks` and `BilliardBlock` are rejected unless a real
  public consumer appears.
- Do not encode auto-routing or fallback mechanics as a `CapacitySearch`
  variant. Search enum variants should name algorithm families only.
- Do not use "Lagrangian facet" terminology. Facets may be q-aligned or
  p-aligned facets of a Lagrangian product; they are not themselves
  Lagrangian.
- Do not expose in-place direction mutation. If product-constrained direction
  projection is needed, use a pure helper that returns a fresh direction.
- Do not expose "signed zero" in the ordinary classifier name unless the strict
  signed-zero behavior is deliberately the public contract.
- Do not promote telemetry such as sigma counts into core capacity result
  structs. Experiment rows can count at iterator/API boundaries.
- Do not move experiment-used capabilities out of the target merely because
  they are not ordinary capacity entrypoints. Gradient from KKT data,
  product-direction projection, billiard bounce counts, and geometric orbit
  recovery are real current consumer needs.
- Treat diagnostic/spec helpers as a separate decision: they may become a
  diagnostics module or stay private/test-support, but must not be mixed into
  polished core API.
- Keep open checks explicit instead of burying uncertainty. Current open checks
  include exact billiard semantics versus separate f64/exact search enums,
  public diagnostics versus private/test-support, and whether `F64Interval`
  should enforce invariants with private fields and a checked constructor.

## Rules For Future Agents

1. Do not edit this file unless Joern explicitly asks to update the goal.
2. Do not use the invalidated target file as evidence for the desired API.
3. Before drafting a new target, inventory real consumers from the repo.
4. Separate final API design from migration mechanics.
5. Preserve capabilities by designing explicit data surfaces; do not let the
   current monolithic geometry object shape the target.
6. If compaction removed conversation context, treat this file as the guardrail
   and stop before contradicting it.
7. If the target draft starts preserving compatibility layers or current bad
   API shapes, stop and report that the draft has drifted.
8. Do not summarize the objective in chat when Joern asks for it; quote the
   source statements and then identify which derived sentence is being checked.
9. Treat the "Merged Objective" as provisional until Joern explicitly confirms
   it. If he says it is still wrong, patch this file before touching any API
   target draft.
10. Before answering that iteration is complete, verify the target against the
    evaluation criteria, compared alternatives, independent review findings,
    duplicate-symbol checks, rejected-shape checks, and open checks.
