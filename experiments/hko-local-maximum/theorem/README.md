# HKO Theorem Certificate Experiments

This directory contains theorem-facing artifacts for the HKO local-maximum
packet. These files support the proof route; they are not broad empirical
searches.

## Subdirectories

| Path | Role |
| --- | --- |
| `exact-witness/` | Exact witness-building scripts and JSON artifacts for the current exact proof route. |
| `active-branch-diagnostic/` | Rust diagnostic for active rows, KKT singularity, symmetry tangent directions, and numerical slice/cone checks. |
| `row-bank-validation/` | Sage-backed validation surface for selected exact row-bank entries. This is not the final theorem verifier by itself. |

## Reading Order

1. `research/hko-local-maximum-status.md`
2. `research/hko-local-maximum-proof-route-note.md`
3. `exact-witness/README.md`
4. `active-branch-diagnostic/`
5. `row-bank-validation/`
