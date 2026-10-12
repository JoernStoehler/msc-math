# Eleven-Facet Cut-and-Ascent Checks

Question: after adding one barely nonredundant facet to HKO2024 in sampled
directions, does the packet's local search find an `F=11` polytope with larger
`sys`?

`main.rs` generates its facet directions from a fixed seed, cuts the hardcoded
HKO polytope, and applies the experiment-local ascent, overshoot, and random
escape policy in `ascent.rs`. It does not import that policy from another
experiment.

The retained `m11-ascent.jsonl` has 20 completed placements. None exceeds the
HKO value under the producer's improvement threshold; all 20 retained final
polytopes still use the added facet. This is a result about the named sampler
and search policy, not all `F=11` perturbations or a local-maximality proof.

Safe smoke mode writes `m11-ascent-smoke.jsonl` rather than the tracked
artifact:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-m11-ascent -- --smoke
```

Full mode resumes by appending only missing named trials. `--fresh` first
deletes the existing tracked output and then reruns the complete panel:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-m11-ascent
cargo run -p exp-hko-local-maximum --release --bin hko-m11-ascent -- --fresh
```

Use `--fresh` only when replacing the retained panel is intended. Changes to
facet placement, the ascent/escape policy, capacity routing, stopping
thresholds, or the HKO fixture require reinterpreting the retained negative
result. The related `../neighborhood-sampling/` packet studies nearby
polytopes without this same cut-then-ascent question.
