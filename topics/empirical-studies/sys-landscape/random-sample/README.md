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

`main.rs` remains as the legacy producer. Its default plan requests 70 accepted
bodies: ten at each `F=5..10`, five at each `F=11,12`, height range
`[0.8,1.2)`, and master seed 42. Candidates use per-attempt Blake3 seeding;
capacity uses the legacy orbit-producing auto route, and volume uses the
rounded exact reference evaluator. This is distinct from the current
production scalar-capacity contract.

Bare invocation writes untracked temp output and cache paths but still runs
that entire plan. For a smaller behavior check, cap both population axes:

```bash
cargo run -p exp-sys-landscape --release --bin sys-random-sample -- \
  --samples-per-f 1 --max-f 5
```

Do not recreate the old canonical JSONL merely to make the figure look
current. Use `../../polytope-datasets/README.md` for active random data.
Changes to sampling, capacity routing, or the active producer do not update
this historical result automatically.
