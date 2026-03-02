# Orbit Recovery Validation

Validates the base point recovery algorithm (Lemma `lem:base-point-recovery`):
given the EHZ algorithm output (σ, β), recover the base point b = γ(0) of
the corresponding Reeb orbit and verify it produces a valid orbit on ∂K.

## Results (2026-03-02)

**112/112 polytopes pass** (7 known + 105 random, F=5..10).

Error metrics — known polytopes achieve machine epsilon; random F=10
polytopes show accumulated numerical error up to ~1e-6:
- Max closure error: 3.70e-7 (known: 1.4e-14)
- Max on-facet error: 6.85e-8 (known: 8.4e-14)
- Max inequality violation: 6.85e-8 (known: 4.9e-14)
- Max action error: 1.83e-6 (known: 3.3e-13)

Validation thresholds: 1e-6 for closure/on-facet/violation, 1e-5 for
action (looser because action accumulates rounding over the full orbit).

Solution dimension distribution:
- dim=0 (unique b): 96.4% of polytopes
- dim=1: 0.9% (lagrangian_tri_sq)
- dim=2: 2.7% (hypercube, symplectic products)

## Run

```bash
cd experiments/
cargo run --release --bin orbit_recovery    # → orbit-recovery/orbit-recovery.jsonl
python3 orbit-recovery/orbit_recovery.py    # → summary statistics
```

## Findings

1. **The algorithm works.** Zero failures across all 112 tested polytopes.
   At F=10, errors grow to ~1e-6 (from ~1e-14 at F≤8) due to
   accumulated floating-point error in the KKT solve and orbit reconstruction.
2. **Base point is generically unique** (96.4% have dim=0). Non-uniqueness
   appears only for polytopes with linearly dependent active normals (products, hypercube).
3. **Recovery is fast** (mean 0.025ms) — negligible compared to capacity computation (mean 4.7ms).
4. **Null space optimization works.** For underdetermined cases (dim>0), ternary search
   in the null space finds a valid b satisfying all inequality constraints.
