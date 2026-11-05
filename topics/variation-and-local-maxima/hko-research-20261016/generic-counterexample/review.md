# Review and execution record

16 October 2026. Root authored the fixture and argument; native subagent
`/root/hko_map` independently reviewed them read-only. It reran `verify.py` under
`python -O` into `/tmp/msc-math/hko-review/20261016/generic-counterexample` and
compared regenerated fixture and certificate byte-for-byte with the retained
files. It separately evaluated all 210 four-row determinants by the permutation
formula and all candidate vertices by Cramer's rule, confirming 25 distinct simple
vertices, ten incident vertices per facet, and exact agreement between retained
binary64 numbers and their dyadic rational definitions.

The reviewer approved the radical intervals, gauge/inclusion inequalities,
strict decimal lower bound and exclusion of nontrivial affine product splittings.
Its verdict was PASS with no mathematical objection. This is independent agent
review, not human acceptance, formal verification or a novelty determination.

Root compiled and executed the production Rust control with `cargo run --offline
--release --locked -p exp-hko-local-maximum --bin hko-generic-counterexample`.
It returned the general capacity route and 25 exact vertices, with capacity
interval `[3.4409255148185856,3.4409255148211946]` and exact rational volume.
The resulting exact-arithmetic outward ratio enclosure is retained in
[capacity.json](capacity.json). This independently corroborates strict violation
under the repository's production capacity contract; it is not a proof of that
implementation's full correspondence on every input.
