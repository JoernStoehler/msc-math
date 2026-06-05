# Rotated Regular Polygons Content Notes

Status: thesis-local content-gathering notes, not source truth.

Purpose: gather the rotated-regular-polygons side-result packet, source
pointers, proof status, exact computation record, and writing guidance needed
to draft `thesis/rotated-regular-polygons.tex`.

Overruled by: `FACTSHEET.md`, exact artifacts in
`experiments/regular-products/`, formal proof files,
task files, and Jörn/Kai review.

Lifecycle: keep while the rotated-regular-polygons thesis section is being
assembled. After the section is stable, either delete this file or reduce it to
a short maintenance index. Do not cite this file as evidence in thesis prose.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Start Here

Use this file as a writing dashboard, not as a linear report.

1. **Write first:** open `thesis/rotated-regular-polygons.tex` and this
   companion side by side. Start from `Drafting Map By Thesis Subsection`
   below, not from the detailed source sections.
2. **Proof status:** the exact Sage certificate proves the open half-domain
   `0 < theta < pi/10`. The endpoint and mirror steps are ordinary
   mathematical arguments: EHZ Hausdorff continuity and the factor-swap mirror.
3. **Main theorem:** on `0 <= theta <= pi/5`,
   `sys(P_5 x_L R(theta)P_5) =
   ((5 + 2*sqrt(5))/10) * sec(min(theta, pi/5 - theta))^2`.
4. **Main source of computation:** the full stdout artifact
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt`.
5. **Do not reread by default:** the old formal draft
   `formal/pentagon-rotation-capacity.tex`, full Sage source, and dense
   empirical tables. Use them only when a detail is needed.

The most useful sections during writing are:

1. `Drafting Map By Thesis Subsection`;
2. `Vocabulary Used Below`;
3. `Proof Dependency Table`;
4. `Endpoint And Symmetry Notes`;
5. `Body-Level Computation Explanation Plan`;
6. `Writing Warnings`;
7. `Likely Kai Questions`.

## Drafting Map By Thesis Subsection

This section mirrors `thesis/rotated-regular-polygons.tex`.

### Empirical Curves

- **Write:** introduce the structured regular-product family and show that the
  pentagon curve suggested a clean formula. Keep this short.
- **Use:** `lagrangian_products_5x5.png` if one broad curve figure is wanted;
  `labeled_pentagons_theta.png` if the angle and labels need visual setup.
- **Say explicitly:** sampled sweeps motivated the formula; they are not proof
  input.
- **Skip:** dense signature-state tables unless an appendix needs empirical
  diagnostics.
- **Lookup:** `Empirical Artifact Recommendations` and `Asset Inventory For
  Writing`.

### Formula For The Pentagon Product

- **Write:** state the formula on `0 <= theta <= pi/5`, then reduce the proof
  to `0 <= theta <= pi/10`.
- **Use:** symmetry lemmas
  `lem:rotation-fundamental-domain` and
  `lem:odd-regular-factor-swap-mirror`.
- **Write:** compute the active 2-bounce branch on the half-domain:
  action `((1 + cos(pi/5))^2)/cos(theta)`, then convert to
  `((5 + 2*sqrt(5))/10) * sec(theta)^2` using constant volume.
- **Normalization:** `P_5` is the regular pentagon with circumradius `1`, so
  `area(P_5) = (5/2) sin(2*pi/5)` and
  `vol(P_5 x_L R(theta)P_5) = area(P_5)^2`.
- **Endpoint checklist:** the Sage certificate is open-domain only. Close
  `theta=0` and `theta=pi/10` by EHZ Hausdorff continuity, then mirror
  `theta` to `pi/5 - theta` by the equal odd-pentagon factor swap.
- **Say explicitly:** the active branch gives the candidate value; the Sage
  certificate proves no other raw sigma is smaller on the open half-domain.
- **Lookup:** `One-Page Proof Map`, `Proof Dependency Table`,
  `Field And Sign Method`, and `Endpoint And Symmetry Notes`.

### Computation With SageMath

- **Write:** present the computation as an exact finite certificate, not as a
  numerical experiment.
- **Use:** the sequence `exact field -> raw sigmas -> KKT solve -> cell sign
  checks -> accepted statuses -> full run`.
- **Report:** raw sigma count `3340`; full run printed
  `CERTIFICATE PASSED`; stdout artifact is the source for exact counts.
- **Say explicitly:** the script is fail-closed because
  `requires_manual_review` is not accepted.
- **Skip:** full progress logs, old status names, and implementation details
  that do not correspond to a mathematical proof obligation.
- **Lookup:** `Body-Level Computation Explanation Plan`, `Status Meanings`,
  `Sage Source Listing Guide`, and `Current Exact Certificate`.

## Lookup Index

- **Formula:** `Result Packet`, `Proof Dependency Table`.
- **Endpoint:** `Endpoint And Symmetry Notes`.
- **Symmetry:** `Endpoint And Symmetry Notes`,
  `formal/lagrangian-product-rotation-symmetry.tex`.
- **Volume:** `Proof Dependency Table`, `Body-Level Computation Explanation
  Plan`.
- **Field choice:** `Field And Sign Method`.
- **Raw sigmas:** `Body-Level Computation Explanation Plan`, item 6.
- **Status meanings:** `Status Meanings`.
- **Figures:** `Empirical Artifact Recommendations`, `Asset Inventory For
  Writing`.
- **What to omit:** `Thesis-Worthy Versus Companion-Only`, `Writing Warnings`.
- **Review prep:** `Likely Kai Questions`, `Review Gates`.

## Vocabulary Used Below

- **Systolic ratio:** `sys(K) = c_EHZ(K)^2 / (2 vol(K))`.
- **Open half-domain:** the interval `0 < theta < pi/10`. This is the domain
  certified directly by Sage.
- **Closed half-domain:** the interval `0 <= theta <= pi/10`. The endpoints
  are added by continuity.
- **Full pentagon domain:** the interval `0 <= theta <= pi/5`. The second half
  is added by the factor-swap mirror.
- **Active branch:** the 2-bounce branch that gives the candidate formula.
- **Raw sigma:** a cyclic facet sequence produced by the block enumeration
  before quotienting by symmetries or removing duplicate representations.
- **Beta:** the KKT multiplier vector for a fixed raw sigma. Strict feasibility
  means all beta coordinates are positive.
- **Q_sigma:** the quadratic value from the KKT solution. When it is positive
  and nonzero, the corresponding action is `1/(2 Q_sigma)`.
- **Gap:** the branch action minus the active-branch action. Positive gap means
  that branch is strictly above the candidate.
- **Cell:** an open subinterval after cutting at all relevant zeros and poles
  of beta coordinates, `Q_sigma`, and the gap.
- **Accepted status:** one of the explicit proof exits accepted by the script.
  Any unrecognized case falls into `requires_manual_review`, which is not
  accepted.
- **Fail-closed:** the script succeeds only if every raw sigma lands in an
  accepted status; otherwise an assertion fails.

## Result Packet

The thesis-facing side result is a packet, not only one formula.

1. Main formula target:
   For the regular pentagon product
   `P_5 x_L R(theta)P_5`, on the fundamental domain
   `0 <= theta <= pi/5`,
   the thesis-facing systolic-ratio formula is

   ```text
   sys(P_5 x_L R(theta)P_5)
     = ((5 + 2*sqrt(5)) / 10) * sec(min(theta, pi/5 - theta))^2.
   ```

   Source pointers:
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`;
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt`;
   `experiments/regular-products/README.md`;
   `formal/pentagon-rotation-capacity.tex`;
   `thesis/rotated-regular-polygons.tex`.

2. Active branch calculation:
   On `0 <= theta <= pi/10`, the active 2-bounce branch has action

   ```text
   ((1 + cos(pi/5))^2) / cos(theta).
   ```

   After converting from capacity to systolic ratio using
   `sys = c^2/(2 vol)` and the rotation-invariant volume, this gives

   ```text
   ((5 + 2*sqrt(5)) / 10) * sec(theta)^2.
   ```

   Source pointers:
   `formal/pentagon-rotation-capacity.tex`, proposition
   `prop:pentagon-rotation-two-bounce`;
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`.

3. Exact executable proof:
   Sage enumerates the raw 2- and 3-bounce combinatorics on the open
   half-domain, solves the exact KKT systems, cuts the interval at all relevant
   beta, `Q`, and gap zeros/poles, and certifies that no feasible branch lies
   below the active branch.

   Source pointers:
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py`;
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt`;
   `experiments/regular-products/README.md`.

   Code-reading guide:
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof_audit_guide.md`.

4. Empirical and figure support:
   The sampled sweeps and plots motivated the formula and should be presented
   only as empirical context or illustration, not as proof input.

   Source pointers:
   `experiments/regular-products/pentagon-rotation-empirics/theta-sweep.jsonl`;
   `experiments/regular-products/pentagon-rotation-empirics/analyze.py`;
   `experiments/regular-products/pentagon-rotation-empirics/three_bounce_branch_actions.png`;
   `experiments/regular-products/pentagon-rotation-empirics/signature_state_table_full.png`;
   `experiments/regular-products/pentagon-rotation-empirics/signature_state_table_competitive.png`;
   `experiments/regular-products/pentagon-rotation-empirics/labeled_pentagons_theta.png`;
   `experiments/regular-products/pentagon-rotation-empirics/trajectory_projections_theta14.png`;
   `experiments/regular-products/pentagon-rotation-empirics/trajectory_projections_theta14_affine.png`;
   `experiments/regular-products/pentagon-rotation-empirics/minimum_orbit_projection_viewer.html`;
   `experiments/regular-products/pentagon-rotation-empirics/minimum_orbit_projection_dataset.jsonl`;
   `experiments/regular-products/pentagon-rotation-empirics/build_interactive_orbit_viewer.py`;
   `experiments/regular-products/README.md`.

5. Broader rotated-polygon context:
   The section may also mention empirical curves for other rotated regular
   polygon products, but the theorem-strength packet described here is the
   pentagon product.

   Source pointers:
   `thesis/rotated-regular-polygons.tex`;
   `experiments/regular-products/`;
   `experiments/regular-products/rotated-regular-products/README.md`.

Guard: do not collapse item 4 into item 3. The proof route is the exact Sage
certificate; sampled sweeps are sanity checks and explanatory figures.

## Cross-Section Placement

Keep product-related thesis material separated by role:

1. `thesis/rotated-regular-polygons.tex`:
   the regular-product side result. This section owns the pentagon formula, the
   body-level Sage proof architecture, selected empirical motivation figures,
   and the endpoint/symmetry close.

2. `thesis/black-box-datascience.tex`:
   black-box and search-method evidence. Product random samples and product
   gradient-ascent runs may appear there as negative search evidence, but the
   pentagon formula theorem is not part of the black-box result.

3. `thesis/published-code-data.tex`:
   reproducibility promises and artifact pointers. This is the right place to
   mention the durable proof stdout artifact
   `experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt`.

4. `formal/pentagon-rotation-capacity.tex`:
   stale source material for old notation, active-branch calculations, and
   continuity ideas. Do not use it as current proof status.

## Content Readiness

Current content status:

1. theorem target:
   ready as a content claim, subject to Jörn/Kai review of final wording and
   endpoint phrasing.

2. active branch calculation:
   ready as a content claim. The executable proof checks the branch action and
   systolic-ratio prefactor. The old formal file remains useful source
   material for this calculation, but it is not current status.

3. lower-bound certificate:
   ready as an executable exact proof for the open half-domain
   `0 < theta < pi/10`. The full-run stdout artifact is the source for exact
   status counts.

4. endpoint and symmetry step:
   route agreed. Use EHZ Hausdorff continuity for the endpoints, and use
   rotation/reflection plus the equal odd-pentagon factor-swap mirror for the
   symmetry reduction. This remains a thesis-writing obligation, not a
   computational gap.

5. empirical figures:
   ready as supporting assets. They are not proof input.

6. broader regular-product context:
   ready only as empirical context. Do not promote the broad sweep to a theorem.

7. code-listing material:
   mostly ready. The executable Sage source is concise enough to quote selected
   blocks. A separate non-executed annotated `.sage.py` file is not recommended
   at this stage because it would duplicate the proof surface and could drift.
   Use the listing guide below instead.

## One-Page Proof Map

Use this as the high-level order before filling in details.

1. State the pentagon formula.
2. Reduce first to `0 <= theta <= pi/5` by simultaneous rotations and
   reflection, then to the half-domain `0 <= theta <= pi/10` by the identical
   odd-pentagon factor-swap symmetry.
3. Note that rotating the second factor preserves area of that factor, hence
   the volume of the Lagrangian product is constant.
4. Compute the active 2-bounce branch and get the candidate value.
5. Explain that the lower bound is an exact finite certificate over raw 2- and
   3-bounce sigmas.
6. Define the raw sigma enumeration at the level needed for the thesis:
   blocks are single facets or ordered adjacent facet pairs; q- and p-blocks
   are interleaved cyclically.
7. State the transition-sign constancy lemma on the open half-domain.
8. Explain the exact field:
   `Frac(K[t])`, `K` the real subfield of `CyclotomicField(20)`, and
   `t = tan(theta/2)`.
9. Explain the rational-function sign method:
   roots of numerator and denominator cut the interval; signs are constant on
   cells.
10. State the full Sage result:
   all open-domain raw sigmas are classified into accepted statuses and the
   full run prints `CERTIFICATE PASSED`.
11. Handle `theta = 0`, `theta = pi/10`, and the mirrored half-domain by the
   continuity/symmetry argument chosen in the final thesis writeup.

This is a writing map. It is not itself a proof.

## Proof Dependency Table

Use this table to keep the proof sources separated while writing.

| Proof component | Current status | Main source |
| --- | --- | --- |
| Formula target on `0 <= theta <= pi/5` | ready subject to final wording review | this companion; `executable_proof.sage.py`; `formal/pentagon-rotation-capacity.tex` as stale source material |
| Reduction to `0 <= theta <= pi/5` | ready, needs thesis wording | `formal/lagrangian-product-rotation-symmetry.tex`, label `lem:rotation-fundamental-domain` |
| Further reduction to `0 <= theta <= pi/10` | ready, needs thesis wording | `formal/lagrangian-product-rotation-symmetry.tex`, label `lem:odd-regular-factor-swap-mirror`; `formal/pentagon-rotation-capacity.tex` as stale source material |
| Finite reduction to 2- and 3-bounce raw sigmas | source available, needs thesis-facing wording | `formal/billiard-capacity-algorithm.tex`, labels `thm:billiard-characterization`, `thm:bounce-bound`, and `alg:billiard`; eventual thesis home likely `thesis/quadratic-program-algorithm-hk2019.tex` or nearby preliminaries |
| Pentagon normalization | ready | `formal/pentagon-rotation-capacity.tex`, definition of `P_5`; `executable_proof.sage.py`, functions `pentagon_normals` and `dual_vertices` |
| Volume invariance and volume value | ready as a simple mathematical argument | `formal/pentagon-rotation-capacity.tex`, active-branch calculation; `executable_proof.sage.py`, function `systolic_ratio_prefactor` |
| Capacity-to-systolic-ratio conversion | ready, checked by Sage preflight | `sys = c_EHZ^2/(2 vol)`; `executable_proof.sage.py`, function `systolic_ratio_prefactor` |
| Active 2-bounce branch action | ready, checked by Sage preflight | `executable_proof.sage.py`, functions `minimum_action` and `assert_formula_checks`; `formal/pentagon-rotation-capacity.tex`, proposition `prop:pentagon-rotation-two-bounce` |
| Systolic-ratio prefactor | ready, checked by Sage preflight | `executable_proof.sage.py`, function `systolic_ratio_prefactor` |
| Raw sigma enumeration | ready as executable certificate input | `executable_proof.sage.py`, functions `blocks`, `enumerate_k_bounce_sigmas`, and `transition_pruned_sigmas_open` |
| Transition-sign constancy on open half-domain | ready as exact Sage assertion | `executable_proof.sage.py`, function `transition_table_open` |
| KKT branch classification | ready as exact Sage assertion | `executable_proof.sage.py`, function `classify_sigma` |
| Open half-domain lower bound | ready from full Sage run | `executable_proof.sage.py`; `executable_proof.full.stdout.txt` is the source for exact counts |
| Endpoints `theta=0`, `theta=pi/10` | ready, needs thesis wording and final citation choice | Chaidez-Hutchings `CH2021` continuity statement; `formal/combinatorial-boundary-regularity.tex`, label `prop:sys-continuous` |
| Mirror to `pi/10 <= theta <= pi/5` | ready, needs thesis wording | `formal/lagrangian-product-rotation-symmetry.tex`, label `lem:odd-regular-factor-swap-mirror` |

This table is also the best answer to the question “what should Kai trust?”:
the open-domain lower bound is the exact Sage certificate; the endpoints and
symmetry are ordinary mathematical writeup obligations.

## Current Exact Certificate

Current script:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

Current reproduction command after CLI cleanup:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500
```

Current development-prefix command:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Current full stdout artifact:

```text
SageMath version: 10.7
runtime: 2126.34 seconds = 35 minutes 26.34 seconds
stdout: experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

These are cached facts copied from the stdout artifact. If they matter for the
final text, re-check the stdout file rather than treating this companion as
source truth.

The full run was regenerated after the folder migration and CLI cleanup. The
stdout file is the source for exact status counts and the final runtime.

Full-run stdout source:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

Use this stdout artifact as the source for exact status counts. Do not maintain
copied count tables in notes unless they are checked against the stdout during
thesis writing.

The full run printed:

```text
CERTIFICATE PASSED in 2126.34s
```

Post-CLI-cleanup prefix checks:

```text
--limit 5: passed in 4.08s
--limit 50: passed in 14.09s after transition-table cleanup
```

Rerun the full certificate only after changing the script, Sage version, facet
conventions, enumeration logic, or claimed formula.

## Empirical Artifact Recommendations

The empirical artifacts are valuable for orientation and figures. They should
not be described as proof input.

Most thesis-useful:

1. `labeled_pentagons_theta.png`
   Use to explain the facet/vertex labels and the meaning of the rotation
   parameter.

2. `minimum_orbit_projection_viewer.html`
   Use while writing and possibly as a linked supplemental artifact. It gives a
   slider over the sampled theta mesh, shows Q/P projections of the minimizing
   orbit, and has an affine/raw projection toggle. It is especially useful for
   explaining why projected breakpoints can coincide and why the endpoint has
   many tied sampled minimizers.

3. `trajectory_projections_theta14_affine.png`
   Best static figure for the active branch geometry at a representative
   angle. Prefer this over the raw projection if only one projection figure is
   used in the thesis.

4. `three_bounce_branch_actions.png`
   Useful for explaining the old empirical question: which 3-bounce families
   looked competitive before the exact exhaustive proof replaced the manual
   shortlist.

Useful but probably not main-text figures:

1. `signature_state_table_competitive.png`
   Good for showing sampled admissible/minimal states of the competitive
   signatures. It is dense, so it may belong in appendix or not at all.

2. `signature_state_table_full.png`
   More complete but too dense for the main narrative.

3. `trajectory_projections_theta14.png`
   The raw recovered projection is useful for debugging and for explaining why
   projection can merge several breakpoints. The affine version is clearer for
   thesis exposition.

Interactive viewer source/output:

```text
producer: build_interactive_orbit_viewer.py
input: theta-sweep.jsonl
jsonl output: minimum_orbit_projection_dataset.jsonl
html output: minimum_orbit_projection_viewer.html
```

The HTML is standalone and embeds the dataset, but the JSONL is also committed
so the data can be inspected without opening a browser.

## Asset Inventory For Writing

Copy thesis publication assets into `thesis/` before including them in the
final PDF. The paths below are source artifacts, not thesis-owned publication
paths.

Recommended main-text candidates:

The image sizes below are cached inspection facts, not source truth. Re-check
the files if exact dimensions matter for layout.

| Source artifact | Size | Suggested use |
| --- | ---: | --- |
| `experiments/regular-products/pentagon-rotation-empirics/labeled_pentagons_theta.png` | `795x410` | Define labels and rotation convention. |
| `experiments/regular-products/rotated-regular-products/lagrangian_products_5x5.png` | `795x509` | Show the empirical pentagon curve and the special value near `18 deg`. |
| `experiments/regular-products/pentagon-rotation-empirics/trajectory_projections_theta14_affine.png` | `670x464` | Show representative minimizing orbit geometry. |

Possible appendix or companion-only assets:

| Source artifact | Size | Suggested use |
| --- | ---: | --- |
| `experiments/regular-products/pentagon-rotation-empirics/three_bounce_branch_actions.png` | `795x795` | Explain why 3-bounce competitors were investigated before the exhaustive certificate. |
| `experiments/regular-products/pentagon-rotation-empirics/signature_state_table_competitive.png` | `774x782` | Dense sampled state table for competitive signatures. |
| `experiments/regular-products/pentagon-rotation-empirics/signature_state_table_full.png` | `774x782` | Complete sampled state table; likely too dense for main text. |
| `experiments/regular-products/pentagon-rotation-empirics/trajectory_projections_theta14.png` | `657x464` | Raw projection; useful mainly to understand projection coincidences. |
| `experiments/regular-products/rotated-regular-products/lagrangian_products_7x7.png` | `795x509` | Broader empirical contrast if the section discusses other regular products. |
| `experiments/regular-products/rotated-regular-products/lagrangian_products_polygon_pairs.png` | `794x696` | Broad regular-pair sweep; likely appendix or omitted. |

Interactive/supporting artifacts:

| Source artifact | Current content |
| --- | --- |
| `experiments/regular-products/pentagon-rotation-empirics/minimum_orbit_projection_viewer.html` | Standalone HTML viewer with embedded data. |
| `experiments/regular-products/pentagon-rotation-empirics/theta-sweep.jsonl` | `145` sampled rows, angles `0.0` to `36.0` degrees. |
| `experiments/regular-products/pentagon-rotation-empirics/minimum_orbit_projection_dataset.jsonl` | `145` viewer rows, angles `0.0` to `36.0` degrees. |

Do not include the interactive viewer directly in the PDF. Use it while writing
or cite it as supplemental material only if the final publication bundle has a
clear place for HTML artifacts.

## What The Code Proves

The script asserts:

1. facet conventions match the experiment convention;
2. the active 2-bounce sigma has the expected action;
3. the systolic-ratio prefactor simplifies to `(5 + 2*sqrt(5))/10`;
4. mixed transition signs have no roots in the open half-domain;
5. the open-domain transition-pruned raw sigma count is `3340`;
6. representative statuses behave as expected in preflight;
7. every classified raw sigma lands in the explicit accepted status set.

The current decisive assertions are:

```python
assert len(sigmas) == 3340
assert classification.status in ACCEPTED_STATUSES, classification
```

## Status Meanings

The accepted statuses have proof meanings:

```text
no_kkt_solution
  The KKT system is inconsistent.

zero_q_identity
  Q_sigma is identically zero, so the chosen KKT branch has no finite positive
  action. Singular zero-Q cases are not accepted unless forced-zero-beta is
  separately proved.

singular_kkt_forced_zero_beta
  The KKT system is singular, but every solution has a beta coordinate forced
  to be identically zero, so no strictly feasible branch exists.

not_feasible_on_open_domain
  After cutting at all beta, Q, and gap zeros/poles, there is no open cell
  where all betas and Q are positive.

zero_gap_identity
  The branch action equals the active branch action identically. This is a tie
  or duplicate raw-sigma representation, not a lower competitor.

strict_gap_positive_on_feasible_open_domain
  The branch has feasible open cells, but the action gap is positive on every
  feasible cell.
```

`requires_manual_review` is the fail-closed fallback. The full run proves that
it never occurs.

## Sage Source Listing Guide

Use the executable source itself for any listings:

```text
experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

Do not create a second annotated `.sage.py` proof file unless the thesis needs a
long appendix that cannot be served by direct listings from the executable
source. A duplicate annotated source would create a second object that can
become stale.

Current useful listing blocks:

The line ranges below are cached from the current source and are intentionally
only navigation hints. Regenerate line numbers before quoting.

1. proof contract and exact field setup:
   cached source lines `1-60`.
   Shows the half-domain target, the `--limit` contract, the field
   `Frac(K[t])`, the endpoint `tan(pi/20)`, and the accepted statuses.

2. pentagon geometry and KKT system:
   cached source lines `94-170`.
   Shows the rotation parameterization, dual facets, symplectic form, KKT
   matrix, `Q_sigma`, and action formula.

3. rational sign certificate:
   cached source lines `197-277`.
   Shows the exact root-cutting method and one-sample-per-cell sign check.

4. raw sigma enumeration and transition-sign constancy:
   cached source lines `285-382`.
   Shows block enumeration, facet adjacency, mixed transition signs, and the
   transition-pruned open-domain raw sigma list.

5. branch classification:
   cached source lines `390-485`.
   Shows the status logic: inconsistent KKT systems, zero `Q`, singular
   forced-zero-beta cases, feasible-cell checks, and strict positive gaps.

6. preflight and full certificate loop:
   cached source lines `493-578`.
   Shows active branch assertions, raw count assertion, representative status
   checks, accepted-status assertion, and the condition for printing
   `CERTIFICATE PASSED`.

Before quoting with line numbers in the thesis, regenerate line numbers from
the current source with:

```bash
nl -ba experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py
```

Suggested thesis usage:

1. Main text: quote no code, or quote only a small status/assertion block.
2. Body computation subsection: explain the code module by module. Quote only
   the short snippets that make each module's proof obligation concrete.
3. Avoid quoting the full file unless Kai explicitly wants an appendix-level
   executable proof listing.

## Body-Level Computation Explanation Plan

The thesis should not ask Kai to think through the Sage code unaided. It should
explain the executable proof as a sequence of mathematical reductions, each
matched to a small code component and an assertion.

Recommended body-level order:

1. Exact parameter field.
   Mathematical claim: all branch expressions are rational functions in
   `t = tan(theta/2)` over the real pentagon coefficient field.
   Code component: field setup, `cos_theta`, `sin_theta`.
   What Kai should check: the field is exact and contains the pentagon
   constants; endpoint comparisons are made in `AA`.

2. Pentagon product geometry.
   Mathematical claim: the ten dual facets are the five `q` pentagon facets and
   the five rotated `p` pentagon facets in `(q1,q2,p1,p2)` coordinates.
   Code component: `pentagon_normals`, `rotate`, `dual_vertices`,
   `assert_facet_conventions`.
   What Kai should check: the facet convention matches the figures and the
   mathematical notation.

3. KKT branch solve for a raw sigma.
   Mathematical claim: fixing a raw cyclic facet sequence gives one exact KKT
   linear system; its beta solution determines `Q_sigma` and the action
   `1/(2Q_sigma)` when `Q_sigma` is nonzero.
   Code component: `kkt_matrix`, `q_value`, `solve_kkt_branch`.
   What Kai should check: the matrix encodes the stated KKT equations and the
   normalization `sum beta_i = 1`.

4. Active branch.
   Mathematical claim: the intended 2-bounce branch has action
   `((1 + cos(pi/5))^2)/cos(theta)` and gives the stated systolic-ratio
   prefactor after dividing by the constant volume.
   Code component: `minimum_action`, `systolic_ratio_prefactor`,
   `assert_formula_checks`.
   What Kai should check: this is the candidate branch, not the exhaustive
   lower-bound certificate.

5. Exact sign checking.
   Mathematical claim: a rational function has constant sign on each cell cut
   by its zeros and poles.
   Code component: `real_roots_in_open_half_domain`, `sign_certificate`,
   `open_domain_cells`, `sign_at`.
   What Kai should check: signs are decided by exact algebraic root isolation,
   not by floating-point sampling.

6. Raw sigma enumeration and transition pruning.
   Mathematical claim: the relevant 2- and 3-bounce raw sigmas are built from
   single facets and ordered adjacent-facet pairs, then pruned by exact
   transition signs that are constant on the open half-domain.
   Code component: `blocks`, `enumerate_k_bounce_sigmas`,
   `transition_table_open`, `transition_pruned_sigmas_open`.
   What Kai should check: the enumeration is intentionally raw; no
   canonicalization theorem is needed.

7. Branch classification.
   Mathematical claim: every raw sigma either has no KKT branch, is not
   strictly feasible, ties the active branch identically, or has strictly
   larger action on every feasible open cell.
   Code component: `classify_sigma`.
   What Kai should check: the status meanings cover all accepted exits and the
   fallback `requires_manual_review` is not accepted in the full run.

8. Full certificate loop.
   Mathematical claim: all `3340` open-domain raw sigmas are classified into
   accepted proof statuses.
   Code component: `run_preflight`, `run_certificate`.
   What Kai should check: the run asserts the raw count, asserts every status
   is accepted, and prints `CERTIFICATE PASSED` only for an unlimited run.

This explanation should be prose-first. Code snippets are evidence for each
module, not the main exposition. A Sage appendix can contain the full source or
longer excerpts after the body has already explained the proof architecture.

## Field And Sign Method

Use this explanation in thesis-facing form, but rewrite it in Jörn's style.

1. `QQ` is not sufficient because the unrotated pentagon already has constants
   `cos(k*pi/10)` and `sin(k*pi/10)`.
2. The expression field is `Frac(K[t])`, where `K` is the maximal totally real
   subfield of `CyclotomicField(20)` and `t = tan(theta/2)`.
3. With this parameter,
   `sin(theta) = 2t/(1+t^2)` and
   `cos(theta) = (1-t^2)/(1+t^2)`, so all KKT branch expressions are rational
   functions in `t`.
4. Exact endpoint and root comparisons use Sage's real algebraic field `AA`.
   The endpoint `theta = pi/10` corresponds to `t = tan(pi/20)`.
5. A rational function can change sign only at a zero or a pole. The script
   therefore cuts the open interval at all relevant roots and checks one exact
   algebraic sample point in each cell.

## Source Relationship And Stale Material

`formal/pentagon-rotation-capacity.tex` is useful but stale.

Useful pieces:

1. target formula;
2. half-domain symmetry statement;
3. active 2-bounce calculation;
4. continuity lemmas and terminology that can be recycled.

Stale pieces:

1. the file still contains a historical 3-bounce lemma title saying
   ``current blocker'';
2. the final proof should not rely on the old shortlisted-family approach;
3. the exact executable proof now replaces the old manual 3-bounce blocker
   strategy.
4. the file contains historical references to the deleted `cas_witnesses.py`
   artifact. Do not copy those references into thesis prose.

When writing, use the formal file as source material, not as current status.

## Thesis-Worthy Versus Companion-Only

Likely thesis-worthy:

1. theorem statement;
2. volume invariance;
3. active branch action and systolic-ratio prefactor;
4. exact field choice;
5. rational-function sign lemma;
6. raw sigma count `3340`;
7. accepted status summary;
8. command or source path for the Sage certificate;
9. statement that sampled sweeps are sanity checks only.

Probably companion-only:

1. historical `1902.57s` superseded run;
2. old `--smoke` and `--full` CLI history;
3. progress lines at `0000/3340`, `0500/3340`, etc.;
4. detailed status examples unless one example makes the proof more readable;
5. performance-policy discussion;
6. canonical-signature alternatives.

## Endpoint And Symmetry Notes

The executable proof certifies the open half-domain

```text
0 < theta < pi/10.
```

The theorem statement wants

```text
0 <= theta <= pi/10
```

and then the mirrored half-domain

```text
pi/10 <= theta <= pi/5.
```

Agreed writing route:

1. state exactly which continuity result upgrades the open-domain certificate
   to closed endpoints;
2. state what happens at `theta = 0`;
3. state what happens at `theta = pi/10`;
4. state the symmetry mapping `theta` to `pi/5 - theta`.

Endpoint route:

1. Use continuity of the EHZ capacity under Hausdorff convergence of convex
   bodies.
2. In this one-parameter family, `theta -> P_5 x_L R(theta)P_5` is
   Hausdorff-continuous.
3. The four-volume is constant because rotation preserves the area of the
   second pentagon.
4. Therefore `sys(theta) = c_EHZ(theta)^2 / (2 vol)` is continuous.
5. Since the Sage certificate proves equality with the active formula on
   `0 < theta < pi/10`, and the active formula is continuous, equality extends
   to `theta = 0` and `theta = pi/10` by one-sided limits.

Symmetry route:

1. Simultaneous rotation of both factors gives period `2*pi/5` for equal
   regular pentagons.
2. Reflection gives `sys(theta) = sys(-theta)`, so this reduces to
   `0 <= theta <= pi/5`.
3. The further mirror `theta -> pi/5 - theta` does not follow from that lemma
   alone.
4. It follows from the symplectic factor swap for identical odd polygons:
   swapping the factors sends `P x_L R(theta)P` to `R(theta)P x_L (-P)`;
   rotating both factors by `-theta` gives `P x_L R(pi - theta)P`; for a
   regular pentagon, `R(pi)P = R(pi/5)P` modulo the `2*pi/5` rotational
   symmetry, hence this is equivalent to `P x_L R(pi/5 - theta)P`.
5. The formal source now records this as
   `lem:odd-regular-factor-swap-mirror`.

Likely source pointers:

1. `formal/combinatorial-boundary-regularity.tex`, especially
   `prop:sys-continuous`;
2. `formal/pentagon-rotation-capacity.tex`, continuity lemmas around
   `lem:pentagon-rotation-three-bounce-continuity`;
3. `formal/lagrangian-product-rotation-symmetry.tex`, especially
   `lem:rotation-fundamental-domain` and
   `lem:odd-regular-factor-swap-mirror`.
4. `papers/ch2021/s1_introduction_and_main_results.tex`, Chaidez-Hutchings
   introduction statement that symplectic capacities on smooth compact convex
   domains extend continuously to all compact convex sets in the Hausdorff
   topology.
5. `papers/citation-index.md`, rows for EHZ continuity and Haim-Kislev's
   polytope formula.

Do not hide the open-domain/endpoint distinction. Kai may ask this first.

## Likely Kai Questions

1. Why is the proof finite?
   Answer direction: use the existing billiard/KKT finite enumeration theorem,
   then specialize to the Lagrangian-product block enumeration. The certificate
   proves all raw 2- and 3-bounce sigmas generated by that reduction.

2. Why can Sage decide signs rigorously?
   Answer direction: exact rational functions over a number field; exact root
   isolation in `AA`; one sign sample per cell.

3. Why is `QQ` not enough?
   Answer direction: pentagon normals already need cyclotomic real constants.

4. Why trust `not_feasible_on_open_domain`?
   Answer direction: it is not the old whole-interval beta check. The script
   cuts at every beta, `Q`, and gap root/pole and checks every cell.

5. What about singular KKT systems?
   Answer direction: inconsistent systems are `no_kkt_solution`; singular
   systems are accepted only when a beta coordinate is forced to be zero for
   every solution.

6. Why not use canonical signatures?
   Answer direction: raw sigmas avoid an extra proof obligation that
   canonicalization preserves feasibility and gaps.

7. What is the role of empirical curves?
   Answer direction: motivation and visualization only, not proof input.

8. What about endpoints?
   Answer direction: open certificate plus continuity and symmetry. Be explicit
   about this step.

## Writing Warnings

1. Do not say the proof is numerical.
2. Do not say sampled sweeps prove the formula.
3. Do not present the old `not_strictly_feasible_open` status as a proof
   status.
4. Do not write from the formal file's historical current-blocker text without
   updating it to the new executable proof.
5. Do not imply `zero_gap_identity` gives lower competitors.
6. Do not hide that the executable proof is open-domain and that endpoints are
   handled by a separate continuity/symmetry argument.
7. Do not overstate broader rotated-polygon empirical curves as theorem-level.

## Remaining Writing Decisions

Closed non-writing items:

1. exact open half-domain Sage certificate for all 3340 raw sigmas;
2. exact active branch formula and prefactor check;
3. exact transition-sign constancy on the open half-domain;
4. current CLI simplification: default full certificate, `--limit N` prefix.

Still writing or review work:

1. write the endpoint/symmetry argument clearly;
2. decide how much Sage detail belongs in the main section versus appendix;
3. decide whether to include empirical figures and which ones;
4. get Jörn review of theorem wording and status interpretations;
5. get Kai review of computation-as-proof framing if time allows.

## Review Gates

1. Jörn math review:
   theorem statement, endpoint/symmetry wording, status meanings.

2. Jörn writing review:
   how much code/Sage detail to include in the main text.

3. Kai theorem framing review:
   exact proof-by-computation framing and whether the accepted status
   explanation is enough for trust.

4. Optional appendix review:
   if detailed Sage output is moved to
   `thesis/appendix-sagemath-computations.tex`.
