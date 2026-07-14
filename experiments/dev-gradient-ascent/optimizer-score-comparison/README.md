# First optimizer score comparison

Status: bounded optimizer-development smoke; not thesis evidence.

## Question

On the same iteration-0 finite moves, does the guarded candidate-window score
rank the exact `sys` improvement better than the near-active score on the
smallest useful three-role panel? The packet measures ranking only. It does
not compare iterative schedulers, certify a local maximum, or estimate a
population prevalence.

The producer is the existing
`dev-gradient-ascent-local-geometry-probe` audit. Its audit row contains both
model predictions for each exact move, so the comparison is paired: the
near-active and candidate-window ranks are recomputed from identical
`move_key` rows and identical exact target evaluations.

## Cases and selection

The smoke uses the current branch diagnostic and polytope panel at threshold
`1e-3`, with steps `1e-3,1e-4`, iteration `0`, and fixed audit proposals.
Selection is target-qualified source material, not random sampling.

| role | polytope | current source/label | producer selection |
| --- | --- | --- | --- |
| known mechanism/disagreement | `f6be75d99a357735276fc4b6eb36b0549c823dd75faeedb4fc7506903da2f1b8` | `gradient_ascent_products`, `narrow_gap` | `--skip-fixtures-per-label 0 --max-fixtures-per-label 1` |
| ordinary source-diverse control | `3daddfde522cb04777d651814d7f88a31f6ec20c1b7ac8fc960efc3e4534104e` | `random_sample`, `large_gap` | `--skip-fixtures-per-label 2 --max-fixtures-per-label 1` |
| equality/easy control | `43d2432913e3f665557c74ae146711b03fbbdb4182479852672cf1db98dec8cc` | `random_product_sample`, `narrow_gap` | `--skip-fixtures-per-label 1 --max-fixtures-per-label 1` |

The three shards are deliberate: the producer's selector is per-label and
sorts by `input_sys`, so one command cannot select both narrow-gap ranks and
the third large-gap rank without changing the producer. Each shard has one
base state and at most six exact target evaluations; the analyzer refuses a
run above the 18-evaluation packet cap or if any requested ID disappears.

## Exact command and outputs

From the repository root:

```bash
experiments/dev-gradient-ascent/optimizer-score-comparison/run-smoke.sh \
  /tmp/sys-ds-research-lines/optimizer/first-score-comparison
```

The script first runs `cargo build --release -p exp-dev-gradient-ascent` and
only then invokes the release binary three times. It writes one producer
shard per role plus `comparison.jsonl`, `summary.json`, `commands.txt`, and
the producer provenance/budget files under the supplied `/tmp` directory.
No smoke output is tracked by this packet.

## Validation and interpretation

`analyze.py` checks that every requested case is present exactly once, every
audit row is `ok`, and each row has finite exact, near-active, and guarded
candidate-window values. It checks `target_sys - base_sys` against the
recorded observed delta, recomputes descending ranks and rank regrets, and
checks candidate-window witness action bounds, relative-window membership,
finite error bounds, and visible beta margins. It also checks that exact move
keys are unique within each shard and that the two score columns therefore
refer to the same exact move rows.

Allowed: plumbing/feasibility evidence and a bounded comparison of these
selected cases under the declared source and parameter slice. A disagreement
can motivate the next optimizer packet.

Prohibited: default-policy selection, endpoint or local-maximum claims,
random-population statements, theorem claims, or treating smoke success as
retained thesis evidence. The controls are deliberately selected and the
exact target is used for ranking audit, so this is not independent validation.

## Architecture decision and expansion stop rule

Alternatives considered were (1) commands only, (2) a new Rust wrapper, and
(3) copying/refactoring the 151 kB producer. The existing audit source already
emits paired predictions, exact target values, ranks, provenance, and witness
guards; a Python standard-library analyzer is therefore the smallest owner
that adds the missing ID selection and arithmetic checks. A wrapper or copy
would duplicate numerical code and create a second source of truth before
there is evidence of recurrence value. The three-shard layout is a selection
constraint, not a shared-cache design.

Expand only if this smoke shows a concrete ranking question that remains
decision-relevant. A larger frozen panel, reusable state bank/cache, or
producer refactor is justified only by repeated cases requiring the same
custom behavior, a downstream need to compare more than 18 exact moves, or a
measured execution cost that the current commands-only path cannot meet.
Otherwise stop here and choose the next packet from the observed mechanism.
