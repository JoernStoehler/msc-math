# Route Demonstrations

Purpose: keep executable examples that explain why the QP capacity route is
not replaced by a tempting simpler route.

These files are research demonstrations, not an importable API. Nobody
currently plans to import this code. Future consumers are expected to read and
run the tests, then copy-edit a small route fragment if they need a simpler
heuristic and do not care about the missing guarantees.

Add a file here only when it preserves a concrete failure, cost blowup, or
scope distinction that explains why the route architecture exists. Do not add
files merely to cover every enum variant, route name, fixture, or parameter
combination.

Keep each file focused on one simplification or failure class. Do not build a
route-by-fixture dashboard here. If a future agent needs a table comparing many
routes and fixtures, it can write an ad-hoc test or analysis script from these
independent examples.

Run the packet with:

```bash
cargo test -p exp-dev-quadratic-program route_demonstrations --lib
```

## Current Coverage

- `unpruned_enumeration_count_blowup`: unpruned HK enumeration is a count-level
  reference route, not an ordinary capacity route.
- `literal_f64_pruning`: literal f64 predicates can silently prune a real
  transition and return the wrong capacity.
- `conservative_pruning_still_f64`: keeping indeterminate transitions fixes
  that pruning miss on the same fixture.
- `conservative_pruning_count_blowup`: keeping indeterminate transitions can
  substantially increase the sigma stream before any KKT solve runs.
- `beta_margin_indeterminate`: a literal f64 `beta > 0` check can reject an
  exactly positive KKT point whose smallest beta is below the route's f64
  decision scale.
- `near_singular_kkt_false_positive`: a near-singular KKT system can make f64
  accept a sigma that exact binary64 rational KKT rejects.
- `q_error_bound_not_certificate`: the current f64 KKT q-error bound is not a
  total error certificate against exact binary64 rational KKT.
- `f64_value_not_certificate`: a correct-looking f64 scalar can still leave the
  minimizing set undecided.
- `retained_candidate_fallback_limit`: exact fallback over retained candidates
  is exact only for the candidate set it receives.
- `guarded_route_safe_refusal`: guarded routes should reject or request fallback
  rather than inventing a scalar on invalid or ambiguous inputs.
- `lp_transition_policy_no_edge_advantage`: LP facet-pair transitions are a
  tried f64 route variant with no observed edge-fixture advantage so far.
- `product_rounding_changes_input`: product rounding solves tiny f64 off-block
  drift by explicitly changing the input, without a stored capacity-distortion
  bound.
- `near_redundant_removal_is_bounded_surrogate`: near-redundant facet removal
  can replace ambiguous direct f64 output by a bounded surrogate.
- `product_billiard_reduces_product_sigma_count`: product/billiard enumeration
  is a product-specialized speedup measured by visited sigmas.

The cost demonstration for the exact transition-pruned reference route lives in
`experiments/performance/src/bin/capacity_route_costs.rs`, because it needs
runtime and hardware context instead of a unit-test assertion.

## Fallback Modes

Fallback-mode demonstrations should be framed as scope distinctions:

```text
mode X solves problem X but not problem Y
```

Do not add one file per `OrbitGuaranteeMode` just because the enum has multiple
variants. Add a demonstration when a concrete retained-candidate example shows
why one of these distinctions matters:

- `BoundSafe` can make the reported minimum action interval endpoint-safe, but
  does not classify every retained minimizer or near-minimizer.
- `MinimaSafe` can resolve retained candidates that overlap the minimum window,
  but does not classify retained candidates outside that window.
- `AllSafe` can classify every retained candidate, but still cannot certify
  sigmas that candidate generation never retained.

The existing `retained_candidate_fallback_limit` file already demonstrates the
last global limitation.
