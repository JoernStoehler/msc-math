# Equal-budget product search (S0)

## Status

Parked target-free prototype on 2026-07-12 pending the next portfolio/process
decision. No real target evaluation has run and no `artifacts/` directory is
promoted. This packet supports no empirical IID/local/CEM comparison.

The chart, evaluator, local, CEM, artifact, and fail-closed analyzer surfaces
have focused tests. Four successive fresh pre-run reviews found and drove
repairs to failure flushing, construction provenance, timing, identity,
adaptive-history reconciliation, and path metrics. The final repairs have not
received a fresh `GO` verdict. The remaining target-free uncertainty is a
cheap, honest production-driver smoke: successful real local-direction setup
made earlier synthetic versions take minutes, so the smoke must be split or
bounded without reverting to hand-built rows that bypass the production arms.

Do not resume merely because implementation exists. Reopen only after the
parent compares the remaining verification and 2,304-target-call cost against
the current M0/M2/I6 alternatives. If selected, use
`/tmp/s0-pause-handoff.md` while this local scratch file remains available;
the durable minimum is this status plus code/tests/config in this packet.

This packet asks whether fixed-constant local search or diagonal CEM discovers
better `sys` values than IID sampling per charged full target call on the fixed
`5 x 5` Lagrangian-product bucket. It runs three fixed replicates and exactly
256 target attempts per arm and replicate. It is operational numerical and
exact-geometry evidence, not theorem-grade exactness or a validation of M0.

`resolved-config.json` is the frozen research contract. The runner must emit
`artifacts/target-evaluations.jsonl`, `expensive-computation-cache.jsonl`,
`cem-generations.jsonl`, `lineages.jsonl`, `comparison-summary.json`, and
`comparison-summary.tsv`. `schemas/artifacts.schema.json` freezes the product
chart, evaluator, target/cache row, CEM-generation, candidate-ID, and lineage
interfaces before implementation. Candidate IDs hash packet version, master
seed, replicate, arm, generation or trajectory, proposal index, and
construction attempt.

Each arm/replicate owns an initially empty cache. A target request consumes one
of 256 attempts before lookup, including duplicates, cache hits, and failed
full computations. Construction rejection is uncharged but counted. CEM is
distribution-parented; local search advances only after a complete fixed-order
line-search grid. Complete returned orbit payloads stay in the packet cache;
compact target rows retain raw returned/admissible word counts, canonical
cyclic-class counts, and support lengths without another action-window run.

The primary decision uses final best `sys`, distinct-key top-eight medians,
descriptive-level counts, and normalized best-so-far AUC. Three replicates are
a robustness check, not inferential statistics. Path, chart-distance, branch,
and genealogy observations are hypothesis-generation evidence (`G`) only.
Summary generation must fail if an arm has fewer than eight distinct successful
keys, and a new trusted `sys > 1` triggers the documented flush/classify stop.

Fast target-free gates are `cargo test --lib`, the Python analyzer tests,
formatting, and strict clippy. The binary test
`synthetic_objective_exercises_production_arm_drivers` is intentionally not a
routine cheap gate in its current form: it was safely terminated after taking
minutes and must be redesigned before use. The promoted fixed run and exact
artifact command are recorded here only after an independent pre-run technical
review returns `GO`.
