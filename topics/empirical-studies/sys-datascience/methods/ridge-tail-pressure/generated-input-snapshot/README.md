# Generated 1M input snapshot

This directory retains the minimal exact inputs needed by the ridge-tail
analyzer.  These files were copied unchanged from the reviewed 1M source run;
the SHA-256 values below are both their original-source and retained-copy
identities.

| file | SHA-256 |
|---|---|
| `one-m-ridge-sum/selection-plan.json` | `4bf777f56fcb07fe18163863594ca90e047f0e402ae7cc4b0e5ea179d6d6e68d` |
| `one-m-ridge-sum/selected-candidates-before-sys.jsonl` | `1ea72e528e4d65776e217e1017e835f949c56ef2dadccaf83f6d0aa34e119587` |
| `one-m-ridge-sum/sys-evaluation-cache.jsonl` | `c07825434c9b12e2774619dadaf5ba8876f02406e891c9c3713a15de6c2c4914` |

The omitted full `candidate-feature-table.jsonl` is a 2.5 GB feature scan. It
is optional identity-audit material, not a reproduction dependency: selected
and evaluated snapshot rows retain the `(k,m)` and selection-feature values
the analysis checks.  When locally available, pass it to
`analyze.py --one-m-feature-table PATH` for the additional full-scan check.
