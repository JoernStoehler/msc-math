# Verification Reasoning

`experiments/verification` is the experiment-only verification boundary for the thesis pipeline. It separates fast crate checks from slow, artifact-backed evidence and owns the generated datasets that support reliability claims.

The package is split into four roles:
- `correctness/` validates the axioms and high-level properties expected of the capacity implementation and provides current evidence for conformality, symplectic invariance, monotonicity, continuity, literature agreement, and unpruned/pruned/billiard agreement.
- `all-minimum/` computes trusted minimum-orbit rows from a local-first polytope pool and cross-checks minimum-action values against `ehz_capacity`.
- `orbit-recovery/` treats those trusted rows as input and validates geometric recovery using KKT rebuild + `recover_and_verify`, checking closure, facet adherence, inside-`K` compliance, and action error.
- `algorithm-comparison/` supplies performance and variant-consistency evidence that supports implementation choices.

The `all-minimum` packet is not geometric ground truth verification: it is the generator of the trusted minimum-orbit dataset used downstream. The `orbit-recovery` packet is the geometric validator for those trusted rows.

Current evidence footprint in the checked-in sources:
- `experiments/verification/correctness/main.rs` plus `correctness.jsonl` for the 6 property propositions.
- `experiments/verification/all-minimum/main.rs` plus `all-minimum.jsonl` and `all-minimum-orbits.jsonl` for canonical minimum rows.
- `experiments/verification/orbit-recovery/main.rs` plus `orbit-recovery*.jsonl` for reconstruction checks.

Canonical `all-minimum`/`orbit-recovery` runs have produced:
- 28 selected polytopes in the local-first pool and 469 trusted minima.
- full-recovery success for those 469 minima in `orbit-recovery`.
- best-effort geometry checks on a strict threshold set (`1e-6` for closure/facet/inside, `1e-5` for action) with observed errors on the order of `e-11` / `e-14`.

Downstream implication: if algorithm changes touch orbit extraction, the trust boundary means both `all-minimum` and `orbit-recovery` outputs should be refreshed before reusing cached claims; `correctness` then re-establishes the global property-level gate.
