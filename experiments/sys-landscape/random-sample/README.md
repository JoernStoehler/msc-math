# Legacy Random-Polytope Sample

Question: in the original small generic random-polytope panel across facet
counts, how large were the sampled `sys` values?

The retained historical answer is summarized in
`../legacy-ascent-continuation-debt.md`: 70 rows, maximum `sys = 0.739`, and no
row above `1`. The raw `random-sweep.jsonl` was renamed into the data-science
producer in April 2026 and the active dataset was later refreshed and enlarged.
It is no longer present under this packet. Therefore
`random_sweep_sys_vs_f.png` is a legacy view, not a summary of the current
`../../polytope-datasets/random.jsonl`.

`main.rs` remains as the legacy producer. Its bare invocation writes only
untracked temp output and cache paths:

```bash
cargo run -p exp-sys-landscape --release --bin sys-random-sample
```

Do not recreate the old canonical JSONL merely to make the figure look
current. Use `../../polytope-datasets/README.md` for active random data.
Changes to sampling, capacity routing, or the active producer do not update
this historical result automatically.
