# HKO Theorem Certificate Experiments

This directory contains theorem-facing artifacts for the HKO local-maximum
packet. These files support the proof route; they are not broad empirical
searches.

Current theorem-facing certificate: `feasible-section-certificate/`.
The older `exact-witness/` route is retained as route history and possible
fallback material; it is not the current theorem certificate.

## Subdirectories

| Path | Role |
| --- | --- |
| `feasible-section-certificate/` | Current theorem-facing feasible-section certificate. Rust exports candidate choices; Sage constructs and verifies exact witness propositions. |
| `active-branch-diagnostic/` | Rust generator/provenance diagnostic for active rows, KKT singularity, feasible-section derivative rows, symmetry tangent directions, and numerical slice/cone checks. |
| `row-bank-validation/` | Sage-backed validation surface for selected exact row-bank entries. This is not the final theorem verifier by itself. |
| `route-history/` | Maintained summaries of the exact geometry, active-minima bookkeeping, and abandoned representative route facts that still matter for thesis understanding. |
| `exact-witness/` | Older exact representative-route scripts and generated artifacts. This route verified useful partial facts but did not close the current theorem certificate. |

## Reading Order

1. `research/hko-local-maximum-status.md`
2. `research/hko-local-maximum-proof-control-packet.md`
3. `research/hko-local-maximum-proof-route-note.md`
4. `feasible-section-certificate/README.md`
5. `route-history/README.md` if the question is why the current route replaced
   the older exact representative route
6. `active-branch-diagnostic/` if the question is candidate-generation
   provenance
7. `row-bank-validation/`
8. `exact-witness/README.md` only for raw old-route artifacts or fallback work
