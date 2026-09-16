# Build closure versus research dependencies

All inputs actually read to build the frozen PDF are in the candidate and
listed with old/new checksums in `dependency-manifest.json`. The following
are outside that direct build closure. Presence and routing checks below do
not establish that a producer can run or that its evidence supports a claim.

| Material | Retention and remaining boundary |
| --- | --- |
| HKO appendix executable core | `recovered/hko-cas-appendix/hko_core.py` is included byte-for-byte because LaTeX lists its contents. A PDF build does not execute Sage or verify its assertions. |
| HKO explanatory figure generator | Hidden `hko-figures/make_figures.py` and its README are copied verbatim to `support/`, with checksums. The historical output directory is absolute and must be changed before regeneration; NumPy/Matplotlib and regeneration were not tested. The two actual PDFs are already direct build assets. |
| HKO source provenance | Hidden `hko-cas-appendix/README.md` and `hko-writing-input/exact-fixed-values.json` are preserved in `support/`, with checksums. They are not freeze-verified build inputs. The original verifier, witness, and summary remain under tracked `experiments/hko-local-maximum/theorem/`; no new Sage run or expanded lineage audit was performed. |
| Original pentagon certificate | TeX excerpts and the existing proof are preserved unchanged. The executable source remains tracked at `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`, with its packet-local evidence. It was not rerun or repaired in this recovery. |
| Pentagon sample and branch figures | The rendered images are direct build assets. Producers remain under `experiments/regular-products/rotated-regular-products/` and `pentagon-rotation-empirics/`. The registered `pentagon-kkt-branch-landscape` input `experiments/regular-products/pentagon-rotation-empirics/kkt-branch-landscape.jsonl` is absent locally. Regeneration was not attempted. |
| Data-science figures | The two PDFs are direct assets. `thesis/figures/datascience/README.md` identifies their tracked analysis scripts and raw run directories. Those generator chains and empirical results were not reproduced. |
| Foundations and flow figures | The PDFs are direct assets. `thesis/figures/foundations/generate.py` and `experiments/dev-flow-graph/visualize-tube/README.md` supply ordinary-tree producer routes; their runtime dependencies and regenerated outputs were not checked. |
| Visualization screenshots | The PNGs are direct assets. `experiments/visualization/README.md` owns the scene and screenshot workflow. Rendering the screenshots again requires that workflow, browser/tooling, and scene inputs; this recovery only preserves their existing bytes. |
| Random/product datasets and invariant table | `experiments/polytope-datasets/random-product.jsonl`, `random.jsonl`, `shared-cache.jsonl`, and `experiments/polytope-invariant-table/polytope-table.jsonl` were absent at inspection. `artifacts/registry.json` registers the `polytope-datasets` and `polytope-invariant-table` snapshots. Downloading, authenticating, and verifying the remote data are separate work; no network materialization was performed. |
| Broader evidence and AI provenance | The frozen prose and its references are retained. Underlying historical runs, chats, analyses, and session records are not made complete or independently verified by recovering the TeX sources. |

The current artifact registry has 13 artifact entries and 39 consumer links;
the frozen chapter reports older counts. Those scientific/provenance claims
were deliberately left unchanged rather than silently revised during source
recovery. The current registry is the materialization route, not evidence that
every snapshot is available or that every historical dataset has been archived.

This is a bounded inventory of dependencies identified from the build inputs,
their producer notes, and the frozen availability chapter. A complete research
archive would additionally need a claim-to-evidence inventory and executable
checks of its producers; neither is implied by the successful isolated PDF build.
