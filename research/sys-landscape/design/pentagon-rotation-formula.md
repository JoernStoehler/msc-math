# Pentagon Rotation Formula: Logbook

## Research Question

Determine and prove the explicit formula for
\[
\mathrm{sys}\!\left(P_5 \times_L R(\theta) P_5\right)
\]
on the fundamental domain `0 <= theta <= pi/5`, together with the minimizing
orbit-family structure that explains the HKO midpoint.

## Current Status

Active exploratory worktree effort. The current conjecture is
\[
\mathrm{sys}(\theta) = \frac{5 + 2\sqrt{5}}{10}\sec^2(\theta)
\quad \text{for } 0 \le \theta \le \pi/10,
\]
mirrored by `theta -> pi/5 - theta` on the second half of the fundamental
domain.

The current leading mechanism is:

1. A single 2-bounce family, represented by a vertex joined to a point on the
   opposite edge, stays minimal on `0 <= theta <= pi/10`.
2. At `theta = pi/10`, that family degenerates to the HKO diagonal.
3. The midpoint carries a tie: minimizing 2-bounce and 3-bounce orbits coexist.

Owned sweep status after the minima-safe rerun:

- `145` sampled angles on `0 <= theta <= pi/5` at `0.25 degree` resolution.
- Max observed error against the conjectured formula:
  `1.375e-09`.
- After collapsing raw `sigma` data to affine Q/P bounce blocks and quotienting
  by dihedral symmetry:
  - one 2-bounce affine class persists on `0 <= theta < pi/10`;
  - the midpoint `theta = pi/10` shows a tie pile of `67` two-bounce and `35`
    three-bounce solver minima;
  - one mirrored 2-bounce affine class persists on `pi/10 < theta <= pi/5`.

## Owned Artifacts In This Worktree

| File | Role |
|------|------|
| `experiments/sys-landscape/pentagon-rotation-formula/main.rs` | owned theta sweep and orbit-class dump |
| `experiments/sys-landscape/pentagon-rotation-formula/analyze.py` | branch normalization and formula checks |
| `experiments/sys-landscape/pentagon-rotation-formula/theta-sweep.jsonl` | generated per-theta orbit data |
| `formal/sys-landscape/pentagon-rotation-formula.tex` | private proof draft and theorem skeleton |

## Method Surface

### Data to collect per theta

- capacity and systolic ratio;
- all tied admissible minimal orbits, not only the solver-selected representative;
- sigma, subset, beta, q, admissibility, and bounce count for each tied orbit;
- enough metadata to normalize tied orbit classes across theta.

### Acceptance checks

The exploration closes only when all of the following hold:

1. The empirical sweep isolates the active minimal branch structure on
   `0 <= theta <= pi/10`.
2. The formal draft proves the candidate formula on that interval and extends it
   by symmetry to `0 <= theta <= pi/5`.
3. The midpoint tie with the HKO diagonal is explained inside the proof rather
   than treated as an unexplained numerical coincidence.

## Open Proof Obligations

1. State the angle convention and translate it cleanly to the HKO paper's
   `90 degree` rotation.
2. Identify a canonical 2-bounce family on `0 <= theta <= pi/10`.
3. Derive the minimizing edge parameter `lambda(theta)` from one support-switch
   or orthogonality condition.
4. Derive the capacity formula `c(theta) = c(0) sec(theta)`.
5. Prove no competing 2-bounce or 3-bounce family beats that branch away from
   the midpoint.
6. Record exactly what is proved rigorously and what is currently only
   computationally sanity-checked.

## Logic Surface For The 3-Bounce Exclusion

The current proof question is not "are there finitely many `3`-bounce
orbits?" but "what is the right finite object on which a continuity argument
is actually safe?"

### What is genuinely finite

For Lagrangian products, the billiard frontend enumerates finitely many
candidate `sigma` with pattern `([Q|QQ][P|PP])^k` for `k in {2,3}`. In that
sense, the `3`-bounce search surface is finite.

However, a raw `sigma` is not automatically the same thing as a single global
continuous branch `theta -> action(theta)`. For a fixed `sigma`, the KKT solve
can:

- be strictly feasible on one open interval of `theta`,
- fail feasibility on another interval because some `beta_k < 0`,
- hit a boundary where some `beta_k = 0` and the orbit contracts,
- or, in principle, pass through a point where the KKT matrix becomes singular.

So the safe branch object is:

- one connected interval on which a fixed `sigma` has
  `M_sigma(theta)` non-singular, `beta_sigma(theta) > 0`, and `Q_sigma(theta) > 0`.

On each such interval, the per-`sigma` action is smooth, hence continuous.

### Safe continuity argument

Let `g(theta)` be the proven `2`-bounce candidate capacity on
`0 <= theta <= pi/10`.

For each `3`-bounce `sigma` and each connected strict-feasibility interval
`I` of that `sigma`, define `A_sigma(theta)` on `I`.

Then the following implication is logically safe:

1. If `A_sigma(theta) > g(theta)` for almost every `theta in I`,
2. then `A_sigma(theta) >= g(theta)` for every `theta in I`,

because `A_sigma - g` is continuous on `I`.

This is the correct use of continuity in the finite-branch setting.

### Where continuity alone is not enough

The dangerous step is to jump from "finitely many raw `sigma`" to "the full
`3`-bounce lower envelope is continuous everywhere." The repo's existing
continuity notes already warn that the naive feasible-set argument gives only
lower semicontinuity, not full continuity, when orbit feasibility changes.

The remaining exceptional points are:

- contraction boundaries where some `beta_k = 0`;
- possible singular-KKT points;
- points where a strictly infeasible `sigma` becomes feasible.

Only the first of these is already under direct control: if some `beta_k = 0`,
the orbit contracts to a shorter `sigma'` with the same action. In particular,
a `3`-bounce boundary point of this type reduces to a `2`-bounce comparison.

### Consequence for proof strategy

This means approach `3` is valid only in the following sharpened form:

- prove that every genuine `3`-bounce orbit lies on one of finitely many
  strict-feasibility branch intervals;
- prove `A_sigma > g` almost everywhere on each such interval;
- prove every exceptional endpoint either contracts to `2`-bounce with equal
  action, or cannot occur in this family.

Without the endpoint analysis, the generic-nonminimality argument is
incomplete. With it, the argument becomes a real proof route.

### What would make approach 3 materially stronger

The strongest useful statement would not be
"generically, `3`-bounce is not the minimum", but rather one of:

- for each `3`-bounce `sigma`, the strict-feasibility set is a finite union of
  intervals and `A_sigma > g` on each interval;
- or generically, no strict `3`-bounce KKT solution exists at all;
- or every exceptional `3`-bounce solution is a contraction point and hence
  automatically reduces to `2`-bounce.

Any of those would turn the continuity idea into a usable proof mechanism
instead of a slogan.

### Formal reduction now written down

The private draft now states the reduction explicitly instead of hiding it in a
gap comment:

- `lem:pentagon-rotation-three-bounce-smooth`:
  for a fixed `3`-bounce `sigma`, the strict-feasibility set
  `U_sigma = {theta : M_sigma non-singular, beta_sigma > 0, Q_sigma > 0}`
  is open, and the per-orbit action `A_sigma(theta)` is smooth on each
  connected component of `U_sigma`;
- `lem:pentagon-rotation-three-bounce-continuity`:
  on such a component, `A_sigma > g` almost everywhere implies
  `A_sigma >= g` everywhere, where `g(theta)` is the explicit `2`-bounce
  capacity;
- `lem:pentagon-rotation-three-bounce-reduction`:
  the global blocker is reduced to two concrete tasks:
  1. prove the almost-everywhere strict inequality on every strict-feasibility
     component;
  2. control admissible endpoint or singular cases by contraction or a separate
     nonminimality argument.

So the current gap is now narrower and more honest:

- interior strict-feasibility intervals need a branchwise lower bound or a
  theta-uniform sliding/stationarity argument;
- endpoints still need to be classified as contraction points, midpoint
  equality cases, or impossible minimizers.

### Current empirical shortlist of relevant 3-bounce families

The committed legend and branch plots already isolate a small competitive
surface, even without the missing `three-bounce-branches.jsonl` artifact:

- closest open-interval competitors:
  - `Q:0-1-23  P:2-3-01`
  - `Q:0-1-34  P:3-4-01`
  both appear on the full sampled open interval `0.0..17.75 degree`;
- next visible competitor tier:
  - `Q:0-1-3  P:0-2-3`
  - `Q:0-1-3  P:1-3-4`
  - `Q:0-2-34  P:0-3-12`
  - `Q:0-2-34  P:2-4-01`
- endpoint signal:
  `Q:0-2-34  P:2-4-01` disappears before the midpoint, at `17.75 degree`,
  so it is a concrete candidate for a contraction-endpoint analysis.

This does not prove anything by itself, but it means a branchwise proof does not
have to start from the full raw `sigma` soup. The first serious lower-bound or
sliding attempt should probably target these few signatures first, then explain
why the rest are farther away automatically.

## Session Resume Note (2026-04-18)

This worktree now has an owned experiment surface, a readable figure set, and a
private proof draft. A later session should start from these files, not from
chat history:

- `experiments/sys-landscape/pentagon-rotation-formula/main.rs`
- `experiments/sys-landscape/pentagon-rotation-formula/analyze.py`
- `formal/sys-landscape/pentagon-rotation-formula.tex`
- `experiments/sys-landscape/pentagon-rotation-formula/signature_legend.txt`

### Stable empirical outputs

The current generated outputs are:

- `three_bounce_branch_actions.png`
- `signature_state_table_full.png`
- `signature_state_table_competitive.png`
- `labeled_pentagons_theta.png`
- `trajectory_projections_theta14.png`

The key empirical picture is stable across the owned smoke and canonical runs:

- one `2`-bounce affine signature is minimal on `0 <= theta < pi/10`;
- the midpoint `theta = pi/10 = 18 degree` has a genuine tie pile containing
  both minimizing `2`-bounce and `3`-bounce orbits;
- the sampled formula error against
  `((5 + 2 sqrt(5)) / 10) sec^2(theta)` on `0 <= theta <= pi/10` is
  `1.375e-09`.

The readable signature notation is now direct:

- `k` means the facet `e_k`;
- `ij` means the vertex `e_i \cap e_j`;
- signatures are written as `Q:...  P:...`, not as arbitrary `S<n>` ids.

### Proof status

The private draft writes out the active `2`-bounce branch calculation and
isolates the remaining gap. More precisely:

- `formal/sys-landscape/pentagon-rotation-formula.tex` derives the
  `lambda(theta)` formula for the vertex-to-opposite-edge branch;
- it derives the corresponding `sec` / `sec^2` law on `0 <= theta <= pi/10`;
- it does not yet prove the exclusion of competing `3`-bounce branches.

So the theorem statement remains conjectural, while the `2`-bounce upper-bound
mechanism is already written in a checkable but still formally marked
`unverified` form.

### Highest-value next step

Do not spend more time on figure polish first. The main blocker is now the
`3`-bounce exclusion. The next useful pass should do one of these:

1. prove branchwise lower bounds for each relevant `3`-bounce support type;
2. prove a strict-feasibility / contraction statement strong enough to make the
   continuity route valid at every endpoint;
3. if neither proof route moves, leave the proof gap explicit and keep the
   empirical branch plots as evidence only.

### Regeneration checks

Use these commands to refresh the owned artifacts:

- `cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula`
- `cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula -- --three-bounce-branches`
- `cd experiments/sys-landscape/pentagon-rotation-formula && uv run analyze.py`
- `cd formal && latexmk -pdf -interaction=nonstopmode main.tex`

## Resume Point

The owned sweep is done and the branch table is stable after affine
normalization. The live blocker is now purely mathematical:
turn the 3-bounce exclusion from an HKO-style sign computation into a written
lemma that is clean enough to trust.
