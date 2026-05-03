# hk2017-search

## Correspondence Contract

- Formal surface: `formal/capacity-algorithms.tex:\ref{alg:ehz}`,
  `formal/ehz-kkt-system.tex:\ref{lem:kkt}`.
- Durable Rust surface: `crates/symplectic/src/algorithms/hk2017/`,
  `crates/symplectic/src/algorithms/orbit_search.rs`.
- Independent comparison surface: `experiments/verification/sage/` once the
  reusable Sage validator is promoted there; until then, topic-local Sage
  probes remain under experiment ownership.
- Shared semantic claim: exhaustive HK2017-style sigma search computes the
  EHZ capacity on the stated polytope class, subject to the formal
  preconditions and any documented exact-fallback path.

## Verification Contract

- Fast smoke/regression coverage lives in the symplectic crate test surface.
- Broader cross-implementation comparisons and benchmark banks live in
  `experiments/verification/`.
- Known gap during this migration: the reusable Sage surface is still being
  consolidated under `experiments/verification/sage/`.
