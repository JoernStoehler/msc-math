# HKO Local Maximum Next Steps

## Primary Packet
Close exact first-order exactification in `exact-clarke/` by extending representative-seed coverage to a final rank-equal active matrix and validating the final span comparison against symmetry tangents.

Stop condition:
- produce a backend-agnostic witness bundle where `rank(active_grad_G)=25`,
- `dim(ker(G))=15`,
- and the kernel is certificate-equal to the `R^40` symmetry tangent basis.

Blockers:
- remaining two asymmetric 7-facet representative classes are unresolved;
- final packet must avoid relying on raw `150` numerical minima and must follow the reduced prototype route (`endpoint-seed-rows.json`, `midpoint-seed-rows.json`) plus additional exact reps.

Immediate commands:
1. Re-open and refresh planning surfaces in `exact-clarke/numerical-permutation-orbits.json` and resolve the two unresolved asymmetric 7-facet classes.
2. Extend exact representative rows with the same witness shape as existing seed rows (`endpoint-seed-rows.json`, `midpoint-seed-rows.json`) and regenerate the exact reduced prototype set.
3. Rebuild `reduced-sys-prototypes.json`, then regenerate the active matrix/witness bundle and run the Sage verifier:
   - `cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation -- --canonical`
   - `cd experiments/hko-local-maximum/sage-validation && sage -python analyze.py --canonical`
4. Update `exact-clarke/widened-seed-witness.json` only from scripted outputs if/when coverage is complete.

If the above cannot complete on current backend cost, explicitly log the obstruction and switch to the contingency route: re-baseline the exact `6240` sigma route with a faster backend, then rerun the same witness-vs-symmetry comparison.

## Secondary track
Keep neighborhood evidence runnable and comparable:
- continue using `perturbation-neighborhood/` for updated 10k-per-bucket LICCA smoke/production artifacts,
- keep `lagrangian-boundary/` and `cut-and-ascent/` as non-default regression checks (re-run only when exact track changes interpretation).
