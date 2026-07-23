# Legacy Random-Product Sample

Question: in the original small random Lagrangian-product panel, how large were
the sampled `sys` values across polygon-pair buckets?

The retained historical answer is summarized in
`../legacy-ascent-continuation-debt.md`: 100 rows, maximum `sys = 0.794`, and
no row above `1`. The raw `random-product-sweep.jsonl` was renamed into the
data-science producer in April 2026 and the active dataset was later refreshed
and enlarged. It is no longer present under this packet. Therefore
`random_product_sweep_sys_vs_pair.png` is a legacy view, not a summary of the
current `../../sys-datascience/produce/random-product.jsonl`.

`main.rs` remains as the legacy producer. Its bare invocation writes only
untracked temp output and cache paths:

```bash
cargo run -p exp-sys-landscape --release --bin sys-random-product-sample
```

Use `../../sys-datascience/produce/README.md` for active random-product data.
The legacy negative result is bounded by its old sampler and panel and is not
a classification of Lagrangian products.
