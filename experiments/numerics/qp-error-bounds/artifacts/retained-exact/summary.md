# Retained-exact route summary

This report is derived from `raw_rows.jsonl`; exact rational values and per-sigma decisions remain only in that raw artifact.

| case | stream | f64 T/I/rej | retained exact A/R | current→retained S/M/W | retained→exact-all S/M/W | exact ms / retained ms |
|---|---:|---:|---:|---|---|---:|
| ordinary_generated_F5 | 9 | 2/0/7 | 2/0 | yes/yes/yes | yes/yes/yes | 114.8 / 71.7 |
| pinned_q4_p5 | 616 | 41/0/575 | 41/0 | yes/no/no | yes/yes/yes | 6691.6 / 431.5 |
| triangle_square_tie | 86 | 4/26/56 | 4/26 | yes/yes/yes | yes/yes/yes | 323.9 / 96.9 |
| pruning_roundoff | 9 | 2/0/7 | 2/0 | yes/yes/yes | yes/yes/yes | 126.0 / 71.9 |

The timings are wall-clock observations for the named scopes: candidate generation, ordinary `MinimaSafe`, exact recheck of every retained candidate, and exact-all reference over the supplied stream. They do not include compilation.
