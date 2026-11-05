# Selected thesis: result-to-evidence map

Checked on 16 October 2026 in `/workspaces/msc-math-next-20261016`, branch
`claude/next-2026-10-16`. This map covers the result families named in
[reproducibility.md](reproducibility.md), the selected
[code/data chapter](../thesis/chapters/14-code-data.tex), and the newly selected
symmetric-partner theorem. Labels below are literal LaTeX labels, not fixed PDF
chapter numbers. Paths in code font are repository-root relative unless a cell
explicitly makes them relative to its linked owner.

The check read current sources and retained reports, checked local path
existence, counted the five identified JSONL files, and read the listed exact
summaries. It did not rerun mathematical verifiers, data producers or analyses,
retrieve external artifacts, validate proofs, or establish human acceptance.
All 39 local Markdown targets, 15 referenced manuscript labels and 22 additional
packet-relative file paths resolved in this checkout. Commands below identify
available routes, not execution receipts for this check.
Use fresh output directories under `/tmp/msc-math/`; inspect packet-local write
effects before execution.

## Mathematical results

| Selected manuscript label | Proof/evidence owner and retained object | Available executable check or producer | Comparison and remaining boundary |
| --- | --- | --- | --- |
| `thm:hko-ten-facet-local-maximum`; `subsec:hko-local-maximum-computation-with-sagemath` | [HKO certificate](../topics/variation-and-local-maxima/hko-local-maximum/theorem/README.md): `witness.json`, `verification-summary.json`; analytic feasible-section and slice argument in the selected [HKO chapter](../thesis/chapters/07-hko.tex) and its input fragments | Owner's `verify.sage.py` under Sage, on a temporary packet copy because it writes its summary; shorter printed [appendix program](../thesis/appendices/hko_core.py). Recorded successful executions: [October receipt](history/thesis-continuation-2026-10-04/hko-execution/receipt.json) | Retained summary says verified, 26 rows, rank 25 and symmetry rank 15, with positive convex weights. Compare exact predicate fields; floating diagnostic hints are not proof data. Analytic implication and ten-facet scope remain essential. |
| `thm:non-hko-ten-facet-local-maximum` | [Non-HKO packet](../topics/variation-and-local-maxima/hko-local-maximum/non-hko/README.md): `PROOF.md`, `certificate/certificate_base.json`, `verification/20260924-original-summary.json`, `verification/20260924-independent.json` | Owner's `verify.py --output-dir <fresh>` then `code/independent_audit.py --source . --rerun <fresh> --output <fresh-file>` with SymPy 1.14.0 and assertions enabled; exact commands in owner | Retained PASS summary: 624 product candidates, 24 sections, gradient rank 22, full chart rank 40, verified upper-section second derivative `-13/432`. Independent implementation evidence does not establish analytic uniform-neighborhood implication or human review. No quantified neighborhood radius. |
| `thm:rotated-pentagon-product-formula` | Maintained [analytic route](../topics/products/pentagon-affine-products/README.md), [endpoint proof](../topics/products/pro-handoffs/pentagon-generalization/math/rotated-pentagon-proof.tex), selected [rotation chapter](../thesis/chapters/09-rotated-regular-polygons.tex). Earlier [CAS packet](../topics/products/regular-products/pentagon-rotation-formula-proof/README.md): `executable_proof.sage.py`, `executable_proof.full.stdout.txt` | Current proof requires mathematical reading; earlier Sage classifier's full command is in its owner. Redirect stdout to a fresh path rather than replacing the retained transcript; keep assertions enabled | Retained stdout reports both open-domain and classified counts 3,340 and `CERTIFICATE PASSED`. Current analytic proof does not depend on the classifier. Earlier finite classification still needs reduction, sign-to-action and continuity arguments. |
| `thm:pentagon-affine-capacity` | [Affine owner](../topics/products/pentagon-affine-products/README.md): `research.tex`, `source-contract.md`, `sources.json`; selected [affine chapter](../thesis/chapters/10-affine-pentagons.tex) | Owner's `latexmk` command builds the note; thesis build compiles selected exposition. No computational certificate is an input to this analytic route | Source convention check belongs to `source-contract.md`. Compilation does not check the proof. Bound applies to independent nonsingular linear images of regular pentagons, not arbitrary pentagons or all ten-facet polytopes. |
| `thm:pentagon-symmetric-partner` | [Symmetric-partner proof](../topics/products/pentagon-arbitrary-partner/symmetric-partner-lemma.md); selected [chapter](../thesis/chapters/10-symmetric-partner.tex), including its directly drawn explanatory figure | Analytic proof and imported translation-fit criterion; `sh thesis/build.sh` checks inclusion and TeX only. The owner's floating LP evaluator and sample collection are not theorem inputs | Fixed regular pentagon and arbitrary centrally symmetric planar convex partner. Visible manuscript TODO preserves human proof/prose acceptance. Odd-regular and restricted asymmetric extensions are not selected by this inclusion. |

## Numerical and empirical results

| Selected manuscript label / passage | Evidence owner and retained object | Available executable check or producer | Comparison and remaining boundary |
| --- | --- | --- | --- |
| `sec:numerics`: 249 systems; 88/1,271-word comparisons | [General development audit](../topics/capacity-and-qp/development/tools/general_algorithm_ablation/RESULTS.md): retained `RESULTS.md` reports 97 ordinary + 108 scaled + 44 near-singular systems and the two complete word cohorts | Same owner's `run.sh <fresh-output-dir>` builds and runs selected verification, numerics, algorithm comparison and end-to-end binaries | Counts and outcomes are report-level evidence; original per-system transcripts are not retained/registered. A fresh execution creates a new evidence object. Claims concern selected development implementations; exact dyadic input and arithmetic assumptions remain part of the proof contract. |
| `sec:numerics`: 88 products; 10,240-product observation | [Product audit](../topics/capacity-and-qp/development/tools/product_closure_route/RESULTS.md): `sample5.jsonl` has 88 nonempty rows. Larger observation uses registered `polytope-datasets` files `random-product.jsonl` and `shared-cache.jsonl` | [Product README](../topics/capacity-and-qp/development/tools/product_closure_route/README.md) owns focused tests and `qp-product-closure-route` binary. Registered dataset paths require materialization | The 88-body packet supports row inspection. Larger geometry/cache join is documented in the report, not rerun here; external files are absent at their consumer paths in this checkout. Stored length-six winners do not exclude longer tied minimizers or prove completeness. |
| `sec:numerics`: production/development bridge | [Correspondence packet](../topics/capacity-and-qp/evidence/numerics-correspondence-check/README.md); current test [selected_route_correspondence.rs](../topics/capacity-and-qp/development/tests/selected_route_correspondence.rs) | `cargo test -p exp-dev-quadratic-program --release --test selected_route_correspondence` | Receipt records four passing fixture tests, with exact value/word or bound comparisons and derivative tolerance as specified there. This finite bridge does not transfer every old audit to all production inputs. |
| `sec:fresh-data-search`, `subsec:ds-sampling`: historical random/product sample and associations | [DS owner](../topics/empirical-studies/sys-datascience/README.md); [evidence closure](../topics/empirical-studies/evidence-closure/README.md), `replay-receipt.json`, `replay-associations/summary.json`; tracked [provenance table](../topics/empirical-studies/polytope-invariant-table/polytope-provenance-table.jsonl) has 14,336 rows | [Historical replay wrapper](../topics/empirical-studies/evidence-closure/replay-historical.sh) takes a fresh output directory and a cached registered invariant table. Source/table production belongs to [datasets](../topics/empirical-studies/polytope-datasets/README.md) and [invariant-table](../topics/empirical-studies/polytope-invariant-table/README.md) owners | Retained recovery receipt `source-verification.json` records 4,096 generic + 10,240 product joins. Historical replay uses its pinned 39-feature analyzer, not today's strict 45-feature reader. Original evaluator execution revision is unknown. Numerical volume and historical sys are not certified ratios. |
| `sec:fresh-data-search`: restored geometry/features and 14,335 capacity intervals | [Retrospective owner](../topics/empirical-studies/retrospective-revalidation/README.md): `source-verification.json`, `geometry-full/comparison.json`, `target-full/summary.json`, `repair/comparison.json`; compressed row outputs retained with each stage | Owner's `verify_import.py`, `inventory.py`, [geometry runbook](../topics/empirical-studies/retrospective-revalidation/geometry-full/README.md), [target runbook](../topics/empirical-studies/retrospective-revalidation/target-full/README.md), [repair runbook](../topics/empirical-studies/retrospective-revalidation/repair/README.md) | The full target summary records 14,245 successes and 91 refusals; repair summary records 37 interval and 53 norm-repair responses, giving 14,335 covered bodies. One body remains outside policy. These later computations do not recover missing historical execution receipts or certify floating-point volumes/ratios. |
| `sec:fresh-data-search`; `sec:published-code-data`: frozen scalar proposer | [Generated-candidate owner](../topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md): `artifacts/100k-promising-scalars/` contains `selected-candidates-before-sys.jsonl`, `sys-evaluation-cache.jsonl`, `selection-plan.json`, `resolved-run-config.json`, compact reports | Owner's Cargo runner takes the durable config; use its output-directory override for a fresh run. Inspect stages before execution: `all` includes target evaluation | Both named JSONL files have 1,675 rows. The 100k generated pool is not 100k capacity evaluations. Historical target v2 differs from current v3; do not merge schemas or treat a new producer as byte reproduction. Omitted full geometry/feature caches prevent replay of the original three-file target-field audit without regeneration. |
| `sec:published-code-data`: historical twelve-run first-order panel | [Panel owner](../topics/empirical-studies/sys-landscape/gradient-ascent-observed-general/README.md): `artifacts/retained-panel.jsonl`, `artifacts/summary.json` | Owner's `analyze_retained_panel.py` reads twelve separately named raw files; historical producer requires commit `dadf7aa824e3d0cd237f45d666ae894b86c600fe` as recorded in the owner | Retained JSONL has twelve rows; summary reports twelve completed runs, all capped at eight accepted moves, all with an above-threshold endpoint move. This is finite-step progress, not endpoint/local-maximality evidence. Historical v1 analyzer and current v2 producer are incompatible. |
| Selected figures and PDF | [Figure contract](figure-reproduction.md), [source rebuild receipt](history/thesis-restart-2026-10-04/source-only-rebuild.json), [regeneration receipt](history/thesis-continuation-2026-10-04/regenerated-assets-rebuild.json), [second-environment receipt](history/thesis-continuation-2026-10-04/cross-environment-repaired.json) | Figure contract owns fresh-output commands and deliberate publication-copy names; `sh thesis/build.sh`, or forced `bash thesis/check-build.sh` for its stronger build checks | October receipts establish their frozen candidate's text/raster comparisons and thirteen generated graphics. The symmetric-partner figure is now drawn in TeX. Old PDF identities and review outcomes do not apply automatically to this changed manuscript. P2 requires reconstructed current-schema input. |

No historical producer was executed for this map.

## Path and availability check

The current literal directory/file paths in the selected code/data chapter were
checked locally. The old `experiments/hko-local-maximum/theorem/` paths are
explicitly historical public-commit paths, not broken current dependencies.
The rotated-pentagon packet's two printed path fragments concatenate to the
existing `topics/products/regular-products/pentagon-rotation-formula-proof/`.
No missing current proof/evidence-owner path was found in that chapter.

The old public commit is
`1bec30637070effe7ab8b0a022ae62e83f6bbcab`. The code/data chapter records a
public-tree check on 18 September 2026; this audit did not contact GitHub or
revalidate public availability. Current `topics/` paths, the printed appendix,
new results and selected manuscript must not be described as all present in
that old commit. There is no release DOI established by this map.

The following consumer files are absent here as expected for unmaterialized
registered artifacts. Their absence is an input prerequisite, not loss of the
tracked provenance or recovery receipts:

| Registry artifact | Pinned snapshot | Missing consumer files |
| --- | --- | --- |
| `polytope-datasets` | `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96` | `topics/empirical-studies/polytope-datasets/{random.jsonl,random-product.jsonl,shared-cache.jsonl}` |
| `polytope-invariant-table` | `c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a` | `topics/empirical-studies/polytope-invariant-table/polytope-table.jsonl` |

Registry identities come from [artifacts/registry.json](../artifacts/registry.json).
The [artifact contract](artifacts.md) owns offline cache verification and
authenticated materialization; neither was executed here. Empty-cache retrieval,
clean environment reproduction, original-machine metadata recovery and selection
of substantial further reruns remain open in
[DISTANCE-TO-SUBMISSION.md](../thesis/DISTANCE-TO-SUBMISSION.md).
