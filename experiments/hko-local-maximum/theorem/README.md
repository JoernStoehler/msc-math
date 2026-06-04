# HKO Theorem Certificate Experiments

This directory contains theorem-facing artifacts for the HKO local-maximum
packet. These files support the proof route; they are not broad empirical
searches.

## Subdirectories

| Path | Role |
| --- | --- |
| `exact-witness/` | Exact witness-building scripts and JSON artifacts for the current exact proof route. |
| `active-branch-diagnostic/` | Rust diagnostic for active rows, KKT singularity, feasible-section derivative rows, symmetry tangent directions, and numerical slice/cone checks. |
| `row-bank-validation/` | Sage-backed validation surface for selected exact row-bank entries. This is not the final theorem verifier by itself. |

## Reading Order

1. `research/hko-local-maximum-status.md`
2. `research/hko-local-maximum-proof-control-packet.md`
3. `research/hko-local-maximum-proof-route-note.md`
4. `exact-witness/README.md`
5. `active-branch-diagnostic/`
6. `row-bank-validation/`
