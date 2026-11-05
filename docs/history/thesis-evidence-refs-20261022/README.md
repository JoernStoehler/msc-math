# Thesis evidence references and focused checks on a clean checkout

> **Absorbed 5 November 2026** from local branch `claude/thesis-evidence-refs-20261022`
> (tip `8c01e887`). Only the new evidence files, the `symmetric_partner_theorem_anchor.rs`
> test and a pentagon README pointer were taken. The branch's edits to
> `thesis/chapters/13-numerics.tex` and `14-code-data.tex` were **not** taken, so statements
> below that those chapters "now" say something describe the branch, not main.

22 October 2026, Claude agent, worktree of `9667e351` (the commit that published
the agent-checked 105-page PDF). Scope: do the repository paths and finite
checks cited by the selected thesis resolve and pass from a clean checkout?
This is evidence for those cited items only; it is not whole-PDF acceptance,
human proof acceptance or full reproduction.

## Path resolution

[`thesis-path-resolution.tsv`](thesis-path-resolution.tsv) lists every
`\path`/`\texttt` repository path in `thesis/` with its object type at HEAD and
at the cited public commit `1bec3063`. Every path resolves at HEAD (bare
directory names resolve relative to the path the sentence introduces).
One defect: Section 15 said the original rotated-pentagon packet is in the
public commit but gave only its current `topics/` path, which does not exist
there. The packet is at `experiments/regular-products/pentagon-rotation-formula-proof/`
in `1bec3063` (transcript identical; classifier differs only by a removed
shebang line). [`14-code-data.tex`](../../../thesis/chapters/14-code-data.tex)
now gives both paths, as it already did for the HKO packet. The thesis rebuilt
(105 pages, no undefined references); page 82 was inspected at 100 dpi.

The live public `main` is `cf3cb699` (14 October), which already contains the
`topics/` layout. The thesis still cites the 18 September commit; changing the
cited public commit is a publication choice and was not made.

## Checks rerun

| Thesis claim | Command | Result |
| --- | --- | --- |
| Second ten-facet local maximum: 24 vertices, 624 cyclic orders, gradient rank 22, min coefficient 3/36617, symmetry rank 15, chart rank 40, `U''=-13/432` (Section 7.7) | packet README commands for `verify.py` and `code/independent_audit.py`, sympy 1.14.0 | both PASS; all listed values match ([summary](non-hko-summary.json), [independent](non-hko-independent.json)) |
| Four-test production correspondence (Section 14) | `cargo test --locked -p exp-dev-quadratic-program --release --test selected_route_correspondence` | 4/4 pass ([log](correspondence-gate.log)) |
| 88-product audit: 1835 weights, 173 496 objectives, 0 violations, 23 559 raw sign disagreements (Section 14) | recomputed from tracked `sample5.jsonl` | all match; 0 ternary mismatches, 0 exact disagreements |
| Symmetric-partner theorem (Section 11) | new `crates/symplectic/tests/symmetric_partner_theorem_anchor.rs` | `c = λ_B(K)` to <2e-16 on 7 bodies; equality only for `B`, `3B` ([log](symmetric-partner-anchor.log)) |

## Not checked

Sage was not available on this host, so `thesis/appendices/hko_core.py`,
the HKO `verify.sage.py` and the rotated-pentagon classifier were not rerun.
The data-science producers were not run.

## Producer rerun (second step)

Same worktree at `ff8b9a7d`, with no prior `target/` directory. Outputs were
written under `/tmp/msc-math/thesis-repro-ablation/`; the cited ones are kept in
[`ablation-rerun/`](ablation-rerun/).

`topics/capacity-and-qp/development/tools/general_algorithm_ablation/run.sh`
built and ran in 93 s. A second `qp-general-selected-numerics` run matched the
first except for timing lines. Against Section 14.5:

| Thesis number | Rerun ([verification](ablation-rerun/verification.txt), [numerics](ablation-rerun/numerics.txt)) | Result |
| --- | --- | --- |
| 249 KKT systems; 0 wrong determinate decisions; 0 beta/Q radius violations; 15 indeterminate | `hybrid_total`: compared 249, true_false 0, false_true 0, violations 0/0, indeterminate 15 | match |
| max beta error/radius `0.00149` | `1.482445e-3` | **mismatch** in the third digit |
| max Q error/radius `0.00081` | `8.087322e-4` | match |
| 88 words / 3 general bodies; 1271 words / 3 product controls; no capacity or minimizer-class miss; exact capacity in interval | words 9+22+57 and 23+86+1162; all mismatch counters 0 | match |
| largest central-value error `2.31e-14` | `2.309264e-14` | match |

`0.00149` has been in the packet's `RESULTS.md` since `0546e450` and was not
remeasured after later source changes. Section 14.5 now says the beta ratio was
"below 0.0015". That holds for both the retained report and this rerun.
`RESULTS.md` keeps its dated measurement. The thesis rebuilt (105 pages, no
undefined references), and page 80 was inspected at 100 dpi. The tracked PDF
snapshot from `9667e351` still shows the earlier wording.

The external `polytope-datasets` snapshot `f7bc6be8…` was already in the local
cache. `verify-cache` passed, and the cited LFS objects match by SHA-256
([hashes](ablation-rerun/sha256.txt)):

- [`join10240.py`](ablation-rerun/join10240.py) recomputes the 10,240-product
  join from Section 14.5 ([output](ablation-rerun/join10240.json)). It finds
  1,024 rows per bucket, a unique cache match for every row, every winner of
  length six, none of length seven or more, and the scalar capacity equal to
  the winner's action in all 10,240 rows. The other 100 cache product rows
  also have length-six winners. All of this matches.
- `retrospective-revalidation/verify_import.py` matched all 14,336 source
  geometries to the table rows, as Section 15 says
  ([receipt](ablation-rerun/import-verification.json)).

Still unverified: the Sage checks, the data-science producers, and the timing
outputs (`algorithms.txt` and `end-to-end.txt`), which the thesis does not cite.
