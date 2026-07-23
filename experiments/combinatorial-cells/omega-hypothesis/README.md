# Omega-Obstacle Hypothesis

Question: do small absolute symplectic pairings between adjacent facet normals
help produce high `sys`, and does the local `sys` gradient favor decreasing
those pairings?

`main.rs` has two roles:

- it generates the package-level `../polytopes.jsonl` cache from fixed plans
  and seeds;
- it writes this packet's `omega-obstacle.jsonl`.

The current retained result is negative evidence against the
near-Lagrangian-ridge hypothesis. It does not show that every useful
symplectic feature is absent.

The producer refreshes tracked evidence and has no smoke mode:

```bash
cargo run -p exp-combinatorial-cells --release --bin cell-omega
```

`uv run analyze.py` reads `omega-obstacle.jsonl` and rewrites this directory's
tracked diagnostic figures. Use either command only when its artifact refresh
is intended.

Other package binaries consume `../polytopes.jsonl`. When this producer,
sampling plan, cache schema, or cached capacity/sigma semantics change, use
repository search for that exact path to find the affected consumers.
