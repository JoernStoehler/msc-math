# Independent Review

Review target: discussion readiness for the optimizer-line lead.

The fresh reviewer independently inspected the quotient mathematics, Rust
producer, raw and compact artifacts, analyzer, generated discussion, and
figures. The reviewer also reconstructed the quotient basis and frozen-state
selection outside the packet's analyzer.

## Verdict

Discussion-ready after bounded repairs. The generic conclusion is supported:
both frozen high states have positive quotient directions at every retained
radius, so neither should be treated as an endpoint candidate. The proposed
quotient-basis polisher is the appropriate next experiment.

## Review evidence

- Translation, scaling, and `sp(4)` generator formulas agree with the HKO
  quotient source and the `(q1,q2,p1,p2)` coordinate convention.
- Independent reconstruction found orbit rank `15`, generator condition
  numbers between `2.00` and `11.79`, maximum orbit projection
  `1.28e-15`, and maximum slice Gram error `4.44e-16`.
- Every retained signed pair is exactly opposite and identical across radii.
- Independent selection over all `3,142` frozen rows reproduces the retained
  global-best and terminal-best unknowns.
- All `216` generic probes keep valid fixed-facet geometry and collapsed
  minimum-action intervals. The HKO theorem remains separate and authoritative.
- Figures agree with the raw evidence and expose the sign separation between
  the generic states and the nominal HKO scalar.

The reviewer ran the packet's `3/3` Rust unit tests and formatting check, in
addition to the independent source, selection, quotient, capacity/geometry,
and figure passes above.

## Findings and disposition

1. The analyzer originally range-checked producer-reported direction and
   projection diagnostics while the report called them independently
   recomputed. Repaired: it now reconstructs both unknown selections and both
   negative-control witnesses, and recomputes raw direction norms, signed-pair
   equality, slice Gram matrices, cross-radius identity, denominators, step and
   delta arithmetic, and compact summaries. The report explicitly labels
   orbit projection and geometry/capacity diagnostics as range-checked.
2. The initial provenance omitted analyzer and manifest identities and stored
   absolute figure paths. Repaired: the producer explicitly hashes all `36`
   selection inputs, producer, analyzer, and Cargo manifest; the analyzer
   verifies those hashes; figure paths are repository-relative.
3. The HKO wording could imply a capacity-sign certificate despite broad
   retained intervals on `136/150` probes. Repaired: HKO is now an operational
   nominal-scalar consistency check, not a successful mathematical
   discriminator. This limitation does not affect the generic rows, whose
   intervals all collapse.
4. The negative-control wording generalized beyond two controls. Repaired to
   refer only to the two tested ordinary states.
5. Review regeneration exposed Matplotlib PDF timestamp nondeterminism.
   Repaired by omitting creation/modification metadata; two consecutive
   analyzer runs produced byte-identical discussion, analysis, PNG, and PDF
   outputs.

No unresolved review finding blocks discussion or branch handoff.
