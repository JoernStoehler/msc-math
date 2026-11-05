# Patterns discovered on 17 October 2026

> **Provenance (absorbed 5 November 2026):** copied unchanged from local branch
> `codex/patterns-2026-10-17` (tip `4442275c`). The branch's thesis edits, thesis/review,
> docs/history audit dirs and dashboard changes were not taken, so references here to
> those, or to thesis text "now" containing patterns material, describe the branch, not
> main. Unreviewed on main; the proposals remain proposals. Companion directory:
> `topics/products/pentagon-arbitrary-partner/cone-certificate-20261017/`.

Start with the illustrated [GALLERY.md](GALLERY.md). The new [UPPER-TAIL.md](UPPER-TAIL.md) reports the contact-invariant local prediction law, blind nonregular tests and failed broader transfer; [PROPOSED-CHAPTER-TEXT.md](PROPOSED-CHAPTER-TEXT.md) remains the earlier proposal. [SUCCESS.md](SUCCESS.md) preserves the ranked success pictures written before computation. This packet answers Jörn's request for more geometric insight; no thesis source was edited.

## What survives

- **A local upper-tail contact law:** a canonical product invariant predicts leading systolic loss near HKO and passes two fresh nonregular panels. Its wider random-product transfer fails; this is not the requested broad invariant or a global-bound theorem. See [UPPER-TAIL.md](UPPER-TAIL.md).

- **Opposite symmetry interventions:** exact triangular controls start at ratio 1/2 and symmetrize to 2/9 or 8/9 depending on relative position. A paired retained-data scan shows why a negative asymmetry correlation should not be interpreted as an improving deformation.
- **A threshold invisible to degree-four moments:** the regular-pentagon product has constant vertex moments through degree four while its known exact ratio crosses one over a gap of exactly 1/10. Harmonic five first carries the missing factor angle.
- **An exact simplex mechanism:** signed ridge ordering gives sys=18/(χ²R²), sharp envelope 50≤sys·R²≤162 and sharp simplex ceiling 3/4. All 512 retained simplex rows match the reconstructed formulas numerically.
- **An affine heptagon theorem:** a new 48-case integer interval certificate extends the known rotation formula to all independent affine images of regular heptagons. Its sharp maximum is 0.9174081964…<1, including a complete equality class.

The predictor and width-ceiling failures are supporting falsifiers, not marginal model-score improvements. The packet does not claim a new above-one body, a classification of arbitrary near-maximal polygon products, human proof acceptance, or external novelty. The general odd-polygon affine question remains open.

## Evidence ownership and access

| Directory | Owns |
|---|---|
| [product-shape/](product-shape/REPORT.md) | Exact rational symmetrization controls, all-product geometry scan, 26 width-ceiling survivors and top retained shape panels |
| [features/](features/REPORT.md) | Degree-four moment proof, harmonic-five control, bucket holdouts/tail tests and historical word-contract audit |
| [simplex/](simplex/REPORT.md) | Signed-ridge derivation, exact 20-vertex polytope lemma and 512-row formula reconstruction |
| [regular/](regular/REPORT.md) | Breadth screen, full tensor containment proof, exact 48-case certificate and heptagon equality class |
| [reviews/](reviews/) | Independent scientific/exact-text review outcomes for the selected packets |

Inspected sources include the repository entry/research guide; empirical population/schema/restoration accounts; product reduction, width and affine proof owners; chapter 8 and its appendix; and task-specific scripts, retained inputs and reviews. These focused reads do not constitute an exhaustive literature or repository survey. The proof packets state their imported mathematical assumptions and independent review scope.

The registered 14,336-body source cache and invariant cache were verified offline before interpretation. Dataset snapshot: `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96`. Historical capacities and numerical volumes are preserved as such. Later current-capacity corroboration has separate [scalar/interval/unresolved accounting](../topics/empirical-studies/retrospective-revalidation/README.md); this packet claims no certified historical systolic-ratio intervals. HKO is a separate known reference, not a sampled exceedance.

## Verification and reproduction

Four proof checks require only Python's standard library and no empirical inputs:

```bash
python3 patterns/product-shape/exact_controls.py
python3 patterns/simplex/ridge_controls.py
python3 patterns/regular/certificate.py verify patterns/regular/heptagon-certificate.json
python3 patterns/simplex/exact_polytope.py > /tmp/msc-math/patterns/simplex-proof-check.json
```

The last command enumerates 134,596 active bases; expect roughly ten seconds on this workstation. It writes only stdout. Numerical scans declare dependencies in their reports/scripts and require the verified retained inputs. No full dataset producer, production capacity evaluator or volume evaluator is needed.

```bash
python3 scripts/artifacts.py verify-cache polytope-datasets
python3 scripts/artifacts.py verify-cache polytope-invariant-table
# Substitute the source-cache path reported by verify-cache below.
uv run --offline patterns/product-shape/analyze.py \
  --source /path/to/polytope-datasets/files/random-product.jsonl \
  --out /tmp/msc-math/patterns/reproduce-product
OPENBLAS_NUM_THREADS=1 python3 patterns/features/analyze.py \
  --repo "$PWD" --source-cache /path/to/polytope-datasets/files \
  --out /tmp/msc-math/patterns/reproduce-features
uv run --offline patterns/simplex/analyze.py \
  --source /path/to/polytope-datasets/files/random.jsonl \
  --output /tmp/msc-math/patterns/reproduce-simplex
```

NumPy is needed for the feature scan; its `--figure` option also needs Matplotlib. Other inline-dependency scripts use `uv`. The offline commands assume the Python wheels are already cached; an absent dependency is an explicit setup requirement. Heptagon discovery scripts write adjacent JSON, so run a scratch copy as its report directs. Scientific proof acceptance does not rest on the plotting or discovery software.

[verification.json](verification.json) records the assembled checks and their limits. `manifest.json` pins the final packet's selected bytes (excluding itself). Figures were visually inspected, with a simplex tick-overlap repair before retention. Exact certificates were run from their retained paths; the regular theorem and the product/moment packets received independent mathematical/code review. This is no claim of formal proof-assistant verification.

## Scope, resources and questions

Independent lanes began with bounded local screening, compared distinct methods, then selected explanatory survivors. The product scan took about 14 seconds; the exact heptagon verifier about 0.1 seconds; the complete simplex certificate about 9 seconds. These are measured components, not a full-run spending estimate. No large producer rerun, cloud job, reset consumption, publication or external message was performed. Most Python plotting dependencies were already cached; the product lane's dependency setup fetched packages, including a reported 5.1 MiB fonttools download, despite its narrower no-network lane instruction. No scientific data or outputs were published. Account quota and billed spending were not measured and remain unknown.

Four optional questions were recorded near the start in `/home/joern/agent-ops/queue/thesispat.md` and `pm` group `msc-math/thesis/patterns`; a fifth prior question from the orchestrator was observed in that group. Searches for answers between chunks returned none during this run. Unanswered priors did not create an approval or a reason to wait. The queue's Done section binds the completed deliverables and checks. The manuscript proposals remain proposals; selecting them is separate from completing this discovery packet.

[review: /root/message_reviewer]
[review: /root/run_reviewer]
