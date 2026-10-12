# Legacy Variable-Facet Ascent

Questions: could the original search improve selected `F=10` endpoints after
adding a facet, and how did several `F=10`/`F=11` continuation orders compare?

The raw `variable-f-ascent.jsonl`, its cache, and its required
`../gradient-ascent-general/gradient-ascent-general.jsonl` input are no longer
retained. The preserved answer in `../legacy-ascent-continuation-debt.md` is:
90 continuation trials; gains from `F=10` to `F=11` were common, but all
retained final values remained below `1`. The two tracked figures are legacy
views without current local analyzer inputs.

`main.rs` remains as historical producer machinery. Its smoke mode uses temp
paths:

```bash
cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent -- --smoke
```

The full legacy run is not reproducible from the current tree without
deliberately restoring or replacing its removed general-ascent input. Full
mode rejects a missing or unusable endpoint input before opening outputs or
applying `--fresh`, so it cannot silently retain an RQ2-only panel as the full
comparison. Smoke remains a separate one-probe fallback, using a fresh random
F10 start when no legacy endpoint can be loaded. Do not
interpret that missing input as a request to regenerate the old experiment.
Reopening continuation requires the separate evidence and return contract in
the debt note.
