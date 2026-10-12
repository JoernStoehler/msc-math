# Legacy Random-Product Sample

Question: in the original small random Lagrangian-product panel, how large were
the sampled `sys` values across polygon-pair buckets?

The retained historical answer is summarized in
`../legacy-ascent-continuation-debt.md`: 100 rows, maximum `sys = 0.794`, and
no row above `1`. The raw `random-product-sweep.jsonl` was renamed into the
data-science producer in April 2026 and the active dataset was later refreshed
and enlarged. It is no longer present under this packet. Therefore
`random_product_sweep_sys_vs_pair.png` is a legacy view, not a summary of the
current `../../polytope-datasets/random-product.jsonl`.

`main.rs` remains as the legacy producer. Its default plan requests ten accepted
products in each of the ten buckets `3 <= k <= m <= 6`, height range
`[0.8,1.2)`, and seed 42. Polygon generation shares one ChaCha8 stream across
buckets rather than using the generic producer's per-attempt Blake3 seeds.
Capacity uses the legacy billiard orbit route (including bounce/iteration
diagnostics); volume uses the rounded exact reference evaluator.

Bare invocation writes untracked temp output and cache paths but still requests
all 100 products. For a smaller behavior check:

```bash
cargo run -p exp-sys-landscape --release --bin sys-random-product-sample -- \
  --samples-per-bucket 1 --max-sides 3
```

Use `../../polytope-datasets/README.md` for active random-product data.
The legacy negative result is bounded by its old sampler and panel and is not
a classification of Lagrangian products.
