# Numerics Content Notes

Section-local companion for `thesis/11-numerics.tex`.  Kai's requested role is
one high-level trust-boundary paragraph, not a numerical-analysis chapter.

## Claim-to-source map

| Reader-facing claim | Source truth | Strength and boundary |
| --- | --- | --- |
| The HKO theorem uses an exact SageMath finite check; Rust/f64 data only select or order candidates. | `experiments/hko-local-maximum/theorem/README.md`, `verify.sage.py`, and `thesis/07-hko-local-maximum-exact-certificate.tex` | Theorem-facing after combination with the hand proof. The verifier reconstructs exact algebraic objects and does not trust f64 acceptance. |
| The rotated-pentagon lower bound uses exact algebraic sign and branch checks. | `experiments/regular-products/pentagon-rotation-formula-proof/README.md`, `executable_proof.sage.py`, its full stdout artifact, and `thesis/09-rotated-regular-polygons-exact-certificate.tex` | Theorem-facing executable certificate after the finite-enumeration reduction. Empirical plots are not proof inputs. |
| Rust has exact rational KKT-stationarity reference routines and candidate-local exact fallback. | `crates/symplectic/src/kkt/rational_solver.rs`, `crates/symplectic/src/algorithms/orbit_search.rs`, and its exact-fallback tests | Exact equations and positivity for each resolved rational-input word. It does not establish the per-word maximum, recheck every retained word, or prove that an earlier f64 generator retained every minimizer. |
| Selected f64 QP paths use route-local true/false/indeterminate classifications. | `crates/symplectic/src/kkt/mod.rs`, `experiments/dev-quadratic-program/src/f64_route/orbit.rs`, and `route_demonstrations/` | Static-margin numerical labels, not theorem-backed predicates. Consumers reject, propagate, or resolve indeterminate candidates according to their own contract. |
| The supported flow-graph control algorithm is exact rational. The earlier binary64 prototype lacked a sound true/false/indeterminate predicate contract and was retired when project time ended. | `crates/symplectic/src/algorithms/flow_graph/README.md`, `formal/flow-graph-real-algorithm.tex` | No FG f64 output is current thesis evidence. Selected exact F5/F6/F7 checks are implementation evidence, not proof of the idealized algorithm or CH2021 scope. |
| Generic perturbation and residual estimates are conditional and do not certify the current public f64 solver. | `formal/hk2017-qp-precision.tex`, current KKT API documentation, and `experiments/dev-quadratic-program/src/route_demonstrations/q_error_bound_not_certificate.rs` | Developer-facing framework with explicit gaps and unvalidated constants. Do not promote it to an appendix theorem or a total error guarantee. |
| The numerics audit compares f64 KKT quantities and predicates with typed reference oracles on a fixed emitted context bank. | `experiments/dev-quadratic-program/numerics-audit/README.md`, producer source, and generated `report.md` | Empirical regression/robustness evidence only. HKO rows use exact arithmetic on the stored binary64 values, not the algebraic HKO object. |

## Audit identity and recomputation

No quantitative audit metric is retained in `11-numerics.tex`, so the thesis
does not depend on a cached `/tmp` artifact.  The current deterministic evidence
producer is:

```bash
cargo run -p exp-dev-qp-numerics-audit --release --bin audit-numerical-errors -- \
  --mode evidence --out-dir /tmp/numerics-audit-evidence
python3 experiments/dev-quadratic-program/numerics-audit/scripts/summarize_observations.py \
  /tmp/numerics-audit-evidence
```

The producer records `input_pair_kind`, `oracle_kind`, object, sigma, and sample
policy in every relevant row.  Any later quantitative thesis sentence must name
its context denominator and oracle kind and point to an identified generated
report.  In particular, `exact_binary64_input` must not be rewritten as
algebraic HKO evidence.

## Deliberate omissions

- No figure or table is used: one paragraph solves the reader problem without
  adding a second visual explanation of trust levels.
- The numerics-proofs appendix is omitted.  The available generic precision
  notes contain explicit gaps and do not support a reader-facing
  certified-solver result.
- The SageMath certificate appendix is omitted.  Its trust-boundary table
  duplicated Sections 7 and 9.  The unique reproduction facts belong in
  Section 12 with the code and data account.
- Reopen the numerics-proofs appendix only for a named thesis claim whose
  numerical proof cannot be understood at its owning section.  Reopen a
  SageMath appendix only when verifier detail cannot be placed with the theorem
  it serves or in the code/data reproduction account.  Reopen quantitative
  audit reporting only when a named thesis sentence benefits and the generated
  artifact identity and denominator are durable.
