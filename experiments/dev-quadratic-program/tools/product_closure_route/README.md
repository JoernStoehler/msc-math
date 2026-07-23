# Product closure-vertex capacity route

Status: pre-production experiment for four-dimensional Lagrangian products.

This route tests the consequence of
`formal/product-qp-six-facet-reduction.tex`: a product-QP capacity maximizer
exists with at most three facets from each planar factor, and each factor has
total weight `1/2`. The algorithm therefore:

1. enumerates every vertex of each planar normalized closure polytope;
2. pairs one closure vertex from each factor;
3. evaluates all cyclic orders of their combined support, at most `5! = 120`;
4. returns `1 / (2 Q_max)` and sparse exact maximizing witnesses.

It does not solve a KKT system. It intentionally evaluates all cyclic orders
rather than encoding the billiard pattern and adjacency restrictions: the
constant bound is small, and the complete Haim--Kislev family makes the extra
orders harmless.

## Contract and scope

The hybrid route uses outward binary64 intervals for closure weights and
objectives, then exact rational arithmetic only for candidates whose intervals
can still attain the maximum. Its exact result is for the rational numbers
represented by the supplied binary64 coordinates, not for unavailable
algebraic source coordinates.

The caller must first validate that the vertices define a finite,
full-dimensional, bounded structural product with the origin in the interior.
This experiment checks finite coordinates, the exact q/p block split, and
existence of closure vertices, but it deliberately does not duplicate the
shared geometric validator.

The output contract is the scalar capacity and at least one sparse maximizing
word. It does not classify every maximizing or near-maximizing branch. That
distinction matters before migrating consumers that need more than the scalar
capacity.

## Reproduce

```bash
cargo test -p exp-dev-quadratic-program --release --lib \
  product::closure_vertex_capacity

cargo test -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route

cargo run -p exp-dev-quadratic-program --release \
  --bin qp-product-closure-route -- \
  --samples=5 --timing-repeats=1 \
  > experiments/dev-quadratic-program/tools/product_closure_route/sample5.jsonl
```

The producer compares:

- the certified hybrid result with complete exact evaluation of every
  closure-vertex candidate;
- every computed closure weight and objective interval with exact
  binary64-rational arithmetic;
- final sparse words with the exact transition relation and the six possible
  product type patterns, up to cyclic rotation;
- the old product/billiard KKT route when there are at most twelve facets; and
- the general transition-pruned exact KKT route when there are at most seven
  facets.

`raw_q_sign_mismatches` counts why a literal `q > 0` binary predicate is
unsafe. The correctness field is `ternary_q_sign_mismatches`: a nonzero value
would mean that the interval predicate made a determinate claim with the wrong
exact sign.

The binary test also checks that the factor-scale stress case remains within
the shared dual/primal coordinate validation contract.

See `RESULTS.md` for the retained interpretation. `sample5.jsonl` is generated
evidence, not a hand-edited expectation file.
