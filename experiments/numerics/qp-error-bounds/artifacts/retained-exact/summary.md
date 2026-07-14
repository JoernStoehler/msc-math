# Retained-exact route summary

This report is derived from `raw_rows.jsonl`; exact rational values and per-sigma decisions remain only in that raw artifact.

| case | stream | f64 T/I/rej | retained exact A/R | current→retained S/M/W | retained→exact-all S/M/W | exact ms / retained ms |
|---|---:|---:|---:|---|---|---:|
| ordinary_generated_F5 | 9 | 2/0/7 | 2/0 | yes/yes/yes | yes/yes/yes | 116.0 / 72.2 |
| pinned_q4_p5 | 616 | 41/0/575 | 41/0 | yes/no/no | yes/yes/yes | 6774.4 / 440.9 |
| triangle_square_tie | 86 | 4/26/56 | 4/26 | yes/yes/yes | yes/yes/yes | 331.0 / 99.0 |
| pruning_roundoff | 9 | 2/0/7 | 2/0 | yes/yes/yes | yes/yes/yes | 116.6 / 72.2 |

The timings are wall-clock observations for the named scopes: candidate generation, ordinary `MinimaSafe`, exact recheck of every retained candidate, and exact-all reference over the supplied stream. They do not include compilation.
