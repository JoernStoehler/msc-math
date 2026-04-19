# Crosspolytope Experiment

`experiments/crosspolytope` is a one-off experiment package for the 4D crosspolytope capacity.

Current role:
- `main/` — dedicated Rust binary for symmetric backtracking and checkpointed search with canonical output.

Inputs/outputs are local to this package (`crosspolytope.jsonl`), and status is consumed by the main library (`known_polytopes.rs`) as a filled-in constant.

