# Math write-up scaffold — 2026-04-12

**Task:** `TASKS.md:292` (`[open] [group:writeup] Math write-up scaffold`).
**Scope:** audit-only refresh across every `crates/**/math.tex` (27 files). No
edits to `math.tex` files, no correctness grading, just what is visibly on the
page. Feeds Jörn's two-phase thesis math write-up and the Kai meeting prep on
2026-04-14.

## 1. Headline numbers

| metric                              | today (2026-04-12) | 2026-04-07 baseline | delta |
| ----------------------------------- | -----------------: | ------------------: | ----: |
| `\begin{unverified}` blocks         |                 69 |                  69 |    0  |
| `% [TODO: JÖRN …]` comments         |                 41 |                 ~53 |  -12  |
| `% [GAP …]` comments                |                 10 |                   ? |    ? |
| theorem-like envs (lem/thm/prop/cor/def/alg/fact) | 100    |                  —  |    — |
| files with any unverified block     |                  9 |                   — |    — |
| files with any theorem-like env     |                 18 |                   — |    — |

Notes:

- `\begin{unverified}` is the red left-bar mdframed environment from
  `crates/library/src/math-preamble.tex:27`. It wraps either a theorem-like
  statement (52 cases) or a remark/fact block standing alone (17 cases).
- The 2026-04-07 TASKS entry `crates/**/math.tex stub/unverified inventory`
  (`TASKS.md:362`) reported "53 stubs + 69 unverified". The "stubs" figure
  there was a combined `TODO+GAP` count. Today: 41+10 = 51, so a handful of
  TODOs were resolved during the 2026-04 code-cleanup bundle (`TASKS.md:330`).
- Unverified block count is stable at 69 — nothing moved from stub to
  verified since last audit.
- 17 `\begin{unverified}` blocks wrap non-theorem content (standalone proofs,
  remark paragraphs, fact chains inside `fact:prefilter-*` in geom). Those
  still appear red in `crates/main.pdf` and still need Jörn's sign-off.
- Library carries the weight: geom 31 + algorithms 23 + kkt 3 = 57 / 69
  unverified blocks (83 %). Experiments carry 4; dev-crates 8.

## 2. Theorem dependency graph

Status key: `[V]` proved + not wrapped · `[U]` wrapped in `\begin{unverified}`
· `[S]` stub — has TODO/GAP marker · `[?]` no-proof + no-marker (surprise
gap) · definitions/algorithms don't carry a proof and are marked `[·]`.

Format: `<label>  [status]  → <outgoing edges>`. Files are ordered
library → experiments → dev. Labels with no outgoing edges are omitted
from the arrows-only lines but still listed in section 3's inventory.

### Library (`crates/library/src/`)

**`geom/math.tex`** — 36 labels, 20 theorem-like envs wrapped in unverified,
19 TODO markers, 1 GAP.

```
def:symplectic-form           [·V]
def:J0                        [·US]   % duplicate of def:symplectic-form
def:ehz-capacity              [·V]
def:systolic-ratio            [·US]
def:lagrangian-product        [·US]   ← alg:billiard, lem:rotation-fundamental-domain, (gradient-analysis)
def:symplectic-product        [·US]
prop:capacity-symplectic-product  [US]  → def:symplectic-product   % has a [GAP marker at line 157
def:polytope-dual             [·US]
def:polar-body                [·V]    % alias of def:polytope-dual
def:face-lattice              [·US]
def:cross-product-4d          [·US]
def:polygon-h-rep             [·US]
def:polygon-area              [·US]  → def:polygon-h-rep
def:volume                    [·US]
def:reeb-vector-field         [·US]
lem:piecewise-linear-reeb     [US]
lem:positive-span             [US]
lem:bounded-triples           [US]   → lem:positive-span
lem:vertex-enumeration        [US]
lem:irredundancy              [US]
prop:integer-cramer           [US]
lem:rational-pipeline         [US]
lem:shoelace                  [V]
thm:hko-counterexample        [US]
fact:prefilter-round          [·U]
fact:prefilter-cast           [U]    → fact:prefilter-round
fact:prefilter-svd            [·U]
fact:prefilter-solve          [·U]
fact:prefilter-weyl           [·U]
fact:prefilter-banach         [U]
fact:prefilter-dot            [·U]
prop:prefilter-bound          [US]   → fact:prefilter-round, -cast, -svd, -solve, -banach, -weyl, -dot
cor:prefilter-soundness       [U]    → prop:prefilter-bound
rem:prefilter-constants       [U]
```

**`algorithms/math.tex`** — 29 labels, 18 theorem-like envs wrapped in
unverified, 10 TODO markers.

```
alg:ehz                        [·V]
cor:adjacency-pruning          [?V]  → alg:ehz                    % text approved, no formal proof
thm:conformality               [US]  % statement-only, "standard result"
thm:sympl-invariance           [US]  % statement-only, "standard result"
lem:base-point-recovery        [V]
lem:lagrangian-facets          [U]
lem:sigma-structure            [US]  % alternation argument flagged imprecise in TODO
thm:billiard-characterization  [U]   % cited to Artstein-Avidan–Ostrover (2014), no proof
thm:bounce-bound               [U]   → thm:billiard-characterization   % cited to Bezdek–Bezdek (2009)
alg:billiard                   [US]  → lem:lagrangian-facets
def:symplectic-polytope        [·U]
def:tube                       [·U]
def:tube-data                  [·U]
def:tube-extension             [·U]
def:rotation-increment         [·U]  % undefined symbol "CH2021"
def:tube-close                 [·U]
lem:prune-empty                [U]
lem:prune-action               [US]
lem:prune-rotation             [U]  % proof cites undefined "CH2021"
lem:prune-simple               [U]  % proof cites external HK2017 Thm 1.5
lem:fixed-point                [US] → def:tube-close
alg:tube                       [·U] → lem:prune-empty, -action, -rotation, -simple, lem:fixed-point, def:tube-close, def:tube-extension
lem:cap-derivative             [US] → lem:H-quadratic
lem:vol-derivative             [US]
prop:capacity-piecewise-smooth [US]  % large TODO flags hypothesis / constant gaps
cor:sys-derivative             [US] → lem:vol-derivative, lem:cap-derivative
```

**`kkt/math.tex`** — 7 labels, 2 theorem-like envs wrapped in unverified,
1 TODO marker.

```
lem:H-quadratic                 [V]
lem:kkt                         [V]
lem:well-defined                [V]
lem:dual-vertex-qp              [V]  → lem:H-quadratic
lem:numerical-transition-feasibility  [U]
lem:q-error-bound               [U]   % bound stated, tightness / applicability not discussed
rem:near-null-lp-search         [·US]
```

### Experiments (`crates/exp-*/`)

Most experiment math.tex files are empty-by-design (intentional stub
comments). Only `gradient-analysis`, `second-order`, `rotated-regular-products`,
`rejection-calibration`, and `boundary-characterization` carry formal content.

**`exp-hko-local-maximum/gradient-analysis/math.tex`** — 2 unverified blocks.

```
(unlabeled theorem @ line 70)   [US]  → lem:lagrangian-facets, lem:cap-derivative
rem:non-symplectic-rotations    [·U]
```

**`exp-hko-local-maximum/second-order/math.tex`** — 5 envs, 0 unverified,
2 TODO markers on proof sketch.

```
lem:first-order-necessary       [V]
def:flat-directions             [·V]
lem:cone-equals-kernel          [V]
prop:second-order-local-max     [VS] → lem:cone-equals-kernel
rem:non-smooth-curvature        [·V]
```

**`exp-hko-local-maximum/perturbation-neighborhood/math.tex`** — intentionally
contains "no formal mathematics" (see `math.tex:10`).

**`exp-sys-landscape/rotated-regular-products/math.tex`** — 1 unverified.

```
lem:rotation-fundamental-domain [US] → def:lagrangian-product
```

**`exp-sys-landscape/rejection-calibration/math.tex`** — 1 unverified.

```
prop:boundedness-iff            [U]
```

**`exp-sys-landscape/random-sample/math.tex`**, **`/random-product-sample/`** —
"No formal mathematics" comments; no environments.

**`exp-combinatorial-cells/boundary-characterization/math.tex`** — 8 envs,
0 unverified, 2 TODO + 2 GAP markers on proofs.

```
def:combinatorial-type          [·V]
def:transition-matrix           [·V] → lem:numerical-transition-feasibility
def:boundary-events             [·V]
lem:step-bound-incidence        [VS]   % GAP: first-order approximation only
lem:step-bound-omega            [V]
prop:sys-continuous             [VS] → def:transition-matrix   % GAP: full continuity not proved
rem:sys-continuous-empirical    [·V]
lem:sys-gradient-a              [V]   % housekeeping: missing \cref to lem:cap-derivative + lem:vol-derivative; file has inline TODO (line 177) to add them
```

(Cross-file edge at `boundary-characterization:33` → `lem:numerical-transition-feasibility` in kkt also lives here via `def:transition-matrix`'s proof citation.)

**`exp-combinatorial-cells/{cell-widths,convexity,multiple-crossings,omega-hypothesis}/math.tex`**
— all intentionally empty (stub comments only).

### Dev + standalone (`crates/dev-*/`, `crates/visualization/`, `crates/crosspolytope/`)

**`dev-algorithm-comparison/ablation/math.tex`** — 2 unverified.

```
lem:transition-feasibility      [U]
cor:ridge-sufficiency           [U]  → lem:transition-feasibility
ex:a3-prunes                    [V]  → lem:transition-feasibility   % example, not theorem
```

**`dev-gradient/numerics/math.tex`** — 5 unverified, 2 TODO, 1 GAP.

```
lem:orbit-feasibility-open      [U]
lem:per-orbit-smooth            [U]  → lem:orbit-feasibility-open
lem:orbit-contraction           [U]  → lem:well-defined
lem:kkt-sensitivity             [U]
prop:capacity-smoothness-classification  [US]  → lem:per-orbit-smooth, lem:cap-derivative
```

**`dev-gradient/numerics-subdifferential/math.tex`** — 1 unverified, 1 TODO.

```
thm:subdiff-with-appearance     [US] → lem:kkt-sensitivity, lem:orbit-feasibility-open, lem:per-orbit-smooth
```

**`dev-capacity-validation/orbit-recovery/math.tex`** — 1 formal lemma,
self-referential edge (agent-flagged, worth a look).

```
lem:finite-orbit-verification   [V]  → def:simple-reeb-orbit, lem:finite-orbit-verification   % self-edge
```

**`dev-numerical-analysis/error-bounds/math.tex`** — 20 envs, 0 unverified
wrapping, 2 TODO, 6 GAP. Most "lem:link-*" entries are statement-only
chain lemmas without formal proofs.

```
def:hk2017-problem              [·V]
fact:hk2017                     [·V]
prop:basic-properties           [?]   % no proof, no marker
prop:q-on-affine                [?]
prop:critical-points            [?]  → lem:well-defined
prop:boundary-vs-interior       [V]  → prop:critical-points
alg:exact-solver                [·V] → prop:boundary-vs-interior, rem:min-support
lem:near-boundary-drop          [V]
lem:link-assembly               [?S]  % TODO: verify constants
lem:link-svd                    [?S]  % GAP: sin(Theta) bound
lem:link-beta0                  [?]  → lem:link-assembly, -svd
lem:link-gradient               [?]  → lem:link-beta0, -svd, -assembly
lem:link-reduced-hessian        [?]  → lem:link-assembly, -svd
lem:link-eigenvalues            [?]  → lem:link-reduced-hessian
lem:link-beta                   [?S] → lem:link-svd, -reduced-hessian, -gradient, -beta0, -eigenvalues   % GAP
lem:q-error-first-order         [V]
lem:q-correction-second-order   [V]  → lem:q-error-first-order
lem:pseudoinverse-orthogonality [V]
cor:taylor-structure            [VS] → lem:q-error-first-order, lem:pseudoinverse-orthogonality   % GAP: key identity empirical only
cor:exact-correction            [?]  → lem:pseudoinverse-orthogonality
```

**Empty-by-design dev/standalone files:** `dev-algorithm-comparison/benchmark`,
`dev-capacity-validation/correctness`, `dev-numerical-analysis/q-error`,
`dev-numerical-analysis/kkt-inertia`, `dev-numerical-analysis/unknown-predicates`,
`visualization`, `crosspolytope`. No theorem-like envs.

### Cross-part edges (experiments / dev → library)

These are the load-bearing citations that tell Jörn which library lemmas
must land correctly for downstream experiments to stand.

Verified by direct grep of `\ref{…}`/`\cref{…}` over all 27 math.tex files.
`lem:sys-gradient-a` is intentionally omitted from the table below: its
source file has a `% TODO: add cross-references to capacity_derivatives_a
and volume_derivatives_a lemmas` comment at `boundary-characterization:177`
but the actual `\cref`/`\ref` is not yet emitted, so strictly there is no
current dependency edge — only a missing-edge note. Agent B's first pass
mis-read this as an active edge; it is not.

| citing label                                     | cites library label(s)                                                         |
| ------------------------------------------------ | ------------------------------------------------------------------------------ |
| `gradient-analysis` (preface)                    | `def:lagrangian-product` (geom), `lem:lagrangian-facets` (algorithms)          |
| `gradient-analysis` (unlabeled thm + prose)      | `lem:cap-derivative` (algorithms, 2×)                                          |
| `lem:rotation-fundamental-domain` (rotated-regular-products) | `def:lagrangian-product` (geom)                                    |
| `prop:second-order-local-max` (second-order proof preamble, :40) | `lem:cap-derivative` (algorithms)                         |
| `prop:sys-continuous`'s `def:transition-matrix` side-statement (boundary-characterization:33) | `lem:numerical-transition-feasibility` (kkt) |
| `prop:capacity-smoothness-classification` (numerics:233) | `lem:cap-derivative` (algorithms)                                     |
| `lem:orbit-contraction` (numerics:175)           | `lem:well-defined` (kkt)                                                        |
| `rem:adjacency-pruning` (error-bounds:160)       | `lem:numerical-transition-feasibility` (kkt)                                    |
| `prop:critical-points` (error-bounds:241)        | `lem:well-defined` (kkt)                                                        |

**Most load-bearing library labels** (ranked by incoming cross-part refs):

1. `lem:cap-derivative` (algorithms/math.tex:713) — 4+ downstream citations
   across gradient-analysis, second-order, numerics. Currently `[US]`.
2. `def:lagrangian-product` (geom/math.tex:99) — 2 downstream cites.
   Currently `[·US]`.
3. `lem:numerical-transition-feasibility` (kkt/math.tex:220) — 2
   downstream cites. Currently `[U]`.
4. `lem:well-defined` (kkt/math.tex:102) — 2 downstream cites. Currently `[V]`.
5. `lem:lagrangian-facets` (algorithms/math.tex:230) — 1 downstream cite
   from gradient-analysis, plus an intra-file cite from `alg:billiard`.
   Currently `[U]`.

`lem:vol-derivative` has zero *current* downstream `\ref` citations
outside the library (only the missing-edge TODO in
boundary-characterization), but it is tightly coupled to
`lem:cap-derivative` in the a_i migration item (`TASKS.md:355–360`) and
both are jointly required for `cor:sys-derivative` and the
gradient-ascent story. It stays a hard-labor item (§4 rank 2).

## 3. Stub / unverified inventory

### 3.1  `\begin{unverified}` blocks (69 total)

Grouped by file, sorted by line. "Wraps" identifies the first environment
begin-line inside the block, or `(remark/prose)` if the block contains no
theorem-like environment.

| file                             | line | wraps                                                 |
| -------------------------------- | ---: | ----------------------------------------------------- |
| geom/math.tex                    |   47 | def:J0                                                |
| geom/math.tex                    |   82 | def:systolic-ratio                                    |
| geom/math.tex                    |   99 | def:lagrangian-product                                |
| geom/math.tex                    |  115 | def:symplectic-product                                |
| geom/math.tex                    |  133 | prop:capacity-symplectic-product                      |
| geom/math.tex                    |  214 | def:polytope-dual                                     |
| geom/math.tex                    |  234 | def:face-lattice                                      |
| geom/math.tex                    |  247 | def:cross-product-4d                                  |
| geom/math.tex                    |  271 | def:polygon-h-rep                                     |
| geom/math.tex                    |  285 | def:polygon-area                                      |
| geom/math.tex                    |  302 | def:volume                                            |
| geom/math.tex                    |  323 | def:reeb-vector-field                                 |
| geom/math.tex                    |  340 | lem:piecewise-linear-reeb                             |
| geom/math.tex                    |  356 | lem:positive-span                                     |
| geom/math.tex                    |  385 | lem:bounded-triples                                   |
| geom/math.tex                    |  421 | lem:vertex-enumeration                                |
| geom/math.tex                    |  467 | lem:irredundancy                                      |
| geom/math.tex                    |  499 | prop:integer-cramer                                   |
| geom/math.tex                    |  560 | lem:rational-pipeline                                 |
| geom/math.tex                    |  628 | thm:hko-counterexample                                |
| geom/math.tex                    |  696 | fact:prefilter-round                                  |
| geom/math.tex                    |  704 | fact:prefilter-cast                                   |
| geom/math.tex                    |  724 | fact:prefilter-svd                                    |
| geom/math.tex                    |  737 | fact:prefilter-solve                                  |
| geom/math.tex                    |  753 | fact:prefilter-weyl                                   |
| geom/math.tex                    |  763 | fact:prefilter-banach                                 |
| geom/math.tex                    |  780 | fact:prefilter-dot                                    |
| geom/math.tex                    |  836 | prop:prefilter-bound                                  |
| geom/math.tex                    |  856 | cor:prefilter-soundness                               |
| geom/math.tex                    |  910 | (prose inside prop:prefilter-bound proof)             |
| geom/math.tex                    | 1020 | rem:prefilter-constants                               |
| algorithms/math.tex              |  105 | thm:conformality                                      |
| algorithms/math.tex              |  118 | thm:sympl-invariance                                  |
| algorithms/math.tex              |  230 | lem:lagrangian-facets                                 |
| algorithms/math.tex              |  262 | lem:sigma-structure                                   |
| algorithms/math.tex              |  325 | thm:billiard-characterization                         |
| algorithms/math.tex              |  344 | thm:bounce-bound                                      |
| algorithms/math.tex              |  357 | alg:billiard                                          |
| algorithms/math.tex              |  399 | def:symplectic-polytope                               |
| algorithms/math.tex              |  416 | def:tube                                              |
| algorithms/math.tex              |  436 | def:tube-data                                         |
| algorithms/math.tex              |  457 | def:tube-extension                                    |
| algorithms/math.tex              |  479 | def:rotation-increment                                |
| algorithms/math.tex              |  494 | def:tube-close                                        |
| algorithms/math.tex              |  509 | lem:prune-empty                                       |
| algorithms/math.tex              |  524 | lem:prune-action                                      |
| algorithms/math.tex              |  542 | lem:prune-rotation                                    |
| algorithms/math.tex              |  562 | lem:prune-simple                                      |
| algorithms/math.tex              |  581 | lem:fixed-point                                       |
| algorithms/math.tex              |  611 | alg:tube                                              |
| algorithms/math.tex              |  712 | lem:cap-derivative                                    |
| algorithms/math.tex              |  784 | lem:vol-derivative                                    |
| algorithms/math.tex              |  841 | prop:capacity-piecewise-smooth                        |
| algorithms/math.tex              |  922 | cor:sys-derivative                                    |
| kkt/math.tex                     |  219 | lem:numerical-transition-feasibility                  |
| kkt/math.tex                     |  311 | lem:q-error-bound                                     |
| kkt/math.tex                     |  455 | rem:near-null-lp-search                               |
| gradient-analysis/math.tex       |   70 | (unlabeled theorem — q-p exchange φ)                  |
| gradient-analysis/math.tex       |  103 | rem:non-symplectic-rotations                          |
| rotated-regular-products/math.tex|   50 | lem:rotation-fundamental-domain                       |
| rejection-calibration/math.tex   |   70 | prop:boundedness-iff                                  |
| ablation/math.tex                |   80 | lem:transition-feasibility                            |
| ablation/math.tex                |  133 | cor:ridge-sufficiency                                 |
| numerics/math.tex                |   85 | lem:orbit-feasibility-open                            |
| numerics/math.tex                |  108 | lem:per-orbit-smooth                                  |
| numerics/math.tex                |  132 | lem:orbit-contraction                                 |
| numerics/math.tex                |  188 | lem:kkt-sensitivity                                   |
| numerics/math.tex                |  222 | prop:capacity-smoothness-classification               |
| numerics-subdifferential/math.tex|   29 | thm:subdiff-with-appearance                           |

Count: 31 geom + 23 algorithms + 3 kkt + 2 gradient-analysis + 1 rotated-rp
+ 1 rejection-calibration + 2 ablation + 5 numerics + 1 numerics-subdifferential
= **69**. Matches raw grep.

### 3.2  `% [TODO: JÖRN …]` markers (41 total)

| file                                  | line | excerpt                                                               |
| ------------------------------------- | ---: | --------------------------------------------------------------------- |
| geom/math.tex                         |   49 | verify: J0 is defined within def:symplectic-form above                |
| geom/math.tex                         |   84 | verify statement (def:systolic-ratio)                                 |
| geom/math.tex                         |  117 | verify statement (def:symplectic-product)                             |
| geom/math.tex                         |  216 | verify statement (def:polytope-dual)                                  |
| geom/math.tex                         |  236 | verify statement (def:face-lattice)                                   |
| geom/math.tex                         |  249 | verify statement (def:cross-product-4d)                               |
| geom/math.tex                         |  273 | verify statement (def:polygon-h-rep)                                  |
| geom/math.tex                         |  287 | "Shoelace formula" terminology ambiguity (def:polygon-area)           |
| geom/math.tex                         |  304 | verify statement (def:volume)                                         |
| geom/math.tex                         |  325 | verify statement (def:reeb-vector-field)                              |
| geom/math.tex                         |  342 | verify statement, add proof (lem:piecewise-linear-reeb)               |
| geom/math.tex                         |  358 | verify statement and proof (lem:positive-span)                        |
| geom/math.tex                         |  387 | verify statement and proof (lem:bounded-triples)                      |
| geom/math.tex                         |  423 | verify statement and proof (lem:vertex-enumeration)                   |
| geom/math.tex                         |  469 | verify statement and proof (lem:irredundancy)                         |
| geom/math.tex                         |  501 | verify statement and proof (prop:integer-cramer)                      |
| geom/math.tex                         |  562 | verify statement, add proof (lem:rational-pipeline)                   |
| geom/math.tex                         |  631 | verify statement, add proof/reference (thm:hko-counterexample)        |
| geom/math.tex                         |  793 | restate prop:prefilter-bound using computed hat_kappa; constant gap   |
| algorithms/math.tex                   |  107 | verify statement, add proof (thm:conformality)                        |
| algorithms/math.tex                   |  120 | verify statement, add proof (thm:sympl-invariance)                    |
| algorithms/math.tex                   |  307 | alternation argument imprecise (lem:sigma-structure)                  |
| algorithms/math.tex                   |  375 | k=1 exclusion deferred to thesis (alg:billiard)                       |
| algorithms/math.tex                   |  533 | "closing a tube adds non-negative action" unjustified (lem:prune-action) |
| algorithms/math.tex                   |  597 | time index for def:tube-close needs verification (lem:fixed-point)    |
| algorithms/math.tex                   |  711 | verify + proof, replaces old lem:cap-derivative+normal (lem:cap-derivative) |
| algorithms/math.tex                   |  783 | verify + proof, replaces old lem:vol-derivative+normal (lem:vol-derivative) |
| algorithms/math.tex                   |  840 | verify statement and proof sketch (prop:capacity-piecewise-smooth)    |
| algorithms/math.tex                   |  921 | verify statement (cor:sys-derivative)                                 |
| kkt/math.tex                          |  460 | verify Type A filtering + Type B bounded-LP (rem:near-null-lp-search) |
| gradient-analysis/math.tex            |   69 | verify q-p exchange map is symplectomorphism                          |
| second-order/math.tex                 |   14 | verify non-smooth second-order sufficiency argument                   |
| second-order/math.tex                 |  135 | proof below is a sketch; compactness needs care                       |
| rotated-regular-products/math.tex     |   58 | verify lemma proof (lem:rotation-fundamental-domain)                  |
| boundary-characterization/math.tex    |   91 | polytope-specific mechanism needs review (prop:sys-continuous)        |
| boundary-characterization/math.tex    |  114 | citation to HoferZehnder1994 Ch.2 (A1 monotonicity axiom)             |
| numerics/math.tex                     |  262 | gap argument assumes orbit action continuous (prop:capacity-smoothness) |
| numerics/math.tex                     |  316 | codimension-1 needs generic nonvanishing nabla (prop:capacity-smoothness) |
| numerics-subdifferential/math.tex     |  154 | action-gap bound for infeasible orbits not formal (thm:subdiff-with-appearance) |
| error-bounds/math.tex                 |  735 | verify assembly error constants (lem:link-assembly)                   |
| error-bounds/math.tex                 | 1273 | Empirical validation claim re max ratio (lem:q-correction-second-order) |

Count: 19 geom + 10 algorithms + 1 kkt + 1 gradient-analysis + 2 second-order
+ 1 rotated-rp + 2 boundary + 2 numerics + 1 numerics-subdiff + 2 error-bounds
= **41**. Matches raw grep.

### 3.3  `% [GAP …]` markers (10 total)

| file                               | line | excerpt                                                       |
| ---------------------------------- | ---: | ------------------------------------------------------------- |
| geom/math.tex                      |  157 | c_EHZ(A) = area(A) for convex bodies A in R^2; dubious citation |
| algorithms/math.tex                |    — | (none)                                                        |
| kkt/math.tex                       |    — | (none)                                                        |
| boundary-characterization/math.tex |   65 | first-order approximation; actual flip time may differ        |
| boundary-characterization/math.tex |  152 | direct argument does not give upper semicontinuity            |
| numerics/math.tex                  |  247 | degenerate tie case: r orbits tied with matching gradients    |
| error-bounds/math.tex              |  634 | threshold β>0 contribution to Q not bounded small             |
| error-bounds/math.tex              |  687 | eps_C chosen empirically, no rigorous derivation              |
| error-bounds/math.tex              |  770 | sin(Theta) bound requires sigma_min(C) >> 0                   |
| error-bounds/math.tex              | 1073 | c_{DeltaH'}, c_{delta g}, c_{delta V}, c_{delta beta_0} not derived |
| error-bounds/math.tex              | 1260 | ‖delta_beta‖ small when ‖r‖ small — conditioning not proved   |
| error-bounds/math.tex              | 1326 | (Hβ*)^T δβ = -δβ^T H δβ — identity only verified empirically  |

Count: 1 geom + 2 boundary-characterization + 1 numerics + 6 error-bounds
= **10**. Matches raw grep. (The 2026-04-07 audit's "53 stubs" number did
not split TODO vs GAP; this run does.)

### 3.4  Surprise gaps — theorem-like envs without proof and without marker

These are environments where the proof is missing *and* there is no
TODO/GAP/unverified wrapping. They may be intentional (cited external
results, trivially-derivable corollaries), but they are worth a
once-over because nothing in the source signals that something is owed.

| label                             | file                             | line | note                                                                 |
| --------------------------------- | -------------------------------- | ---: | -------------------------------------------------------------------- |
| cor:adjacency-pruning             | algorithms/math.tex              |   93 | Text approved (marker at line 94); corollary of alg:ehz; no formal proof. Low-risk — explicit Jörn approval already. |
| prop:basic-properties             | error-bounds/math.tex            |  197 | No proof, no marker. Dev crate — low priority but surprising given the rule "every lemma MUST have a proof". |
| prop:q-on-affine                  | error-bounds/math.tex            |  215 | Same.                                                                |
| prop:critical-points              | error-bounds/math.tex            |  228 | Same. Cites `lem:well-defined` (kkt).                                |
| lem:link-assembly                 | error-bounds/math.tex            |  723 | TODO marker present → strictly not "surprise", but proof is missing. |
| lem:link-svd                      | error-bounds/math.tex            |  746 | GAP marker present.                                                  |
| lem:link-beta0 / -gradient        | error-bounds/math.tex            |  782/821 | No proof, no marker. Part of the perturbation chain; statement-only. |
| lem:link-reduced-hessian / -eigenvalues | error-bounds/math.tex      |  864/897 | No proof, no marker. Chain lemmas.                                   |
| cor:exact-correction              | error-bounds/math.tex            | 1335 | No proof, no marker.                                                 |
| lem:finite-orbit-verification     | orbit-recovery/math.tex          |   29 | Has a proof, but the proof appears to reference its own label (self-loop) — agent flagged as odd. Worth a 30-second look. |

Most surprise gaps live in `error-bounds/math.tex`, which is a dev crate
tracking a numerical-analysis error-bound chain. These are statement-only
placeholders and the chain is acknowledged as work-in-progress in
`TASKS.md:208–229`. Publication-path surprise gap count: **0**.

## 4. Ranked hard-labor list

Ranked by Jörn's framing (`TASKS.md:292`): awkward edge cases, missing
error bounds, unproven gaps, places where cleave-of-statement would
naturally handle edges. Top 10 with rationale, then a flat tail.

**Ranking axes:**

1. Publication-path (library + experiments > dev).
2. In-degree from other labels (cascading impact).
3. Density of TODO/GAP markers on the entry.
4. Explicit "cleave-able" signals: compound hypotheses, handwaves over
   degenerate cases, missing "generic" qualifiers.

### Top 10

1. **`prop:capacity-piecewise-smooth`** (algorithms/math.tex:842) — US,
   large TODO (793–834) flags (a) hypothesis verification gaps, (b) constant
   factor count off by ~4x. This is the piecewise-smooth structure of
   `c(a)` — every gradient-ascent experiment rides on it via
   `lem:cap-derivative`. Cleave-of-statement candidate: separate the
   generic-orbit case from the degenerate-orbit case. **Why:** highest
   in-degree of any stub statement, single largest "hard labor" TODO in
   the library.

2. **`lem:cap-derivative`** + **`lem:vol-derivative`** (algorithms/math.tex
   :713, :785) — both US with "verify statement and proof" TODO; both
   are downstream-critical: cited by `cor:sys-derivative`,
   `prop:capacity-smoothness-classification`, `lem:sys-gradient-a`,
   `gradient-analysis:70`. These are tagged for dual-vertex parameterization
   verification in `TASKS.md:355–360` as the two remaining items blocking
   that migration. **Why:** two-for-one — validating them clears the
   a_i migration and unblocks every gradient-ascent experiment citation.

3. **`prop:prefilter-bound`** (geom/math.tex:837) — US, enormous embedded
   TODO (793–834) explicitly calling out that the statement must be
   restated in terms of the *computed* condition number `hat_kappa` (not
   the true one), and that the tight constant is 5376 rather than 1344
   (factor-of-4 miscount). This is the numerics prefilter bound used to
   certify whether a KKT solve is within error tolerance. Also listed as
   high priority in the existing inventory at `TASKS.md:368`. **Why:**
   cleave-of-statement is explicitly requested in the TODO; factor-4
   constant error is the kind of "look close once" work nobody wants
   to discover in the middle of write-up.

4. **`prop:capacity-symplectic-product`** (geom/math.tex:133) — US, the
   one `[GAP - JÖRN` marker in the library, at line 157: the proof
   cites an unverified "Dacorogna-Moser 1990" construction for the step
   `c_EHZ(A) = area(A)` on 2D convex bodies. Also tracked in
   `TASKS.md:368`. **Why:** the citation is flagged dubious *in situ*;
   this is a "find-the-right-reference-or-write-the-argument" task,
   not conceptual work.

5. **`thm:conformality`** + **`thm:sympl-invariance`** (algorithms/math.tex
   :105, :118) — both US statement-only with `% [TODO: JÖRN - verify
   statement, add proof]`. Standard results, but currently no proof and
   no citation. Also listed at `TASKS.md:371`. **Why:** low conceptual
   cost (textbook-level), high write-up cost (needs the right citation
   + one-sentence proof each). Easy-win if resolved together.

6. **`prop:capacity-smoothness-classification`** (numerics/math.tex:222) —
   US, two TODOs inside the proof (lines 262, 316) flagging (a) the gap
   argument assumes *every* competing orbit's action is continuous near
   `a_0`, and (b) codimension-1 of the non-smooth stratum needs a generic
   nonvanishing-gradient hypothesis. Cleave-of-statement: split "smooth
   almost everywhere" from "smooth where no two orbits tie". **Why:**
   this is the theorem that makes "gradient ascent converges" believable;
   if it ships with handwaves, the gradient-ascent experiments have no
   theoretical backstop.

7. **`thm:subdiff-with-appearance`** (numerics-subdifferential/math.tex:29)
   — US, TODO at line 154 flagging the same "action-gap for infeasible
   orbits" issue as #6 above. Downstream of `lem:kkt-sensitivity`,
   `lem:orbit-feasibility-open`, `lem:per-orbit-smooth` (all US). Cascade
   risk: its proof cites three unverified lemmas. **Why:** this is where
   the orbit-appearance edge case needs a rigorous bound instead of an
   assumption; a cleave-of-statement into "no new orbit appears" and
   "new orbit appears with strictly larger action" would probably resolve
   it.

8. **`thm:billiard-characterization`** + **`thm:bounce-bound`**
   (algorithms/math.tex:325, :344) — both U, no proof, cited to
   "Artstein-Avidan & Ostrover (2014)" and "Bezdek & Bezdek (2009)".
   No TODO markers, so the citations are considered good — but a
   write-up pass will need to expand "cited" into a one-paragraph
   restatement. **Why:** not conceptually hard, but the write-up owes
   an explicit citation dance, and `alg:billiard` (blocked by
   `thm:billiard-characterization`) is the tube-algorithm foundation.

9. **`lem:prune-action`** (algorithms/math.tex:524) — US, embedded TODO
   at line 533: proof asserts "closing a tube adds non-negative action"
   without justification. This is the core correctness lemma for the
   tube-pruning step in `alg:tube`. **Why:** tube algorithm currently
   blocked by `TASKS.md:263` (rotation formula implementation); the math
   side of that same work will re-open this proof.

10. **`lem:q-error-bound`** (kkt/math.tex:311) — U, no explicit TODO
    but agent flagged: "bound stated as `E = (9/2)‖r‖²/|λ_min|`, no
    discussion of when this bound is tight or useful". This is the
    headline error bound for the Q-error work in
    `dev-numerical-analysis/error-bounds/`. **Why:** missing error-bound
    regime discussion is exactly the "missing error bound" category in
    Jörn's framing; easy to add a corollary or remark on applicability.

### Tail — all remaining hard-labor flags, by file

Library `geom/math.tex`:

- `def:J0` (47) — duplicate definition; remove or cross-reference
  `def:symplectic-form`.
- `def:polygon-area` (285) — "shoelace formula" used for two different
  things here vs `lem:shoelace` (576); terminology cleanup.
- `lem:piecewise-linear-reeb` (340) — statement-only.
- `lem:positive-span`, `lem:bounded-triples`, `lem:vertex-enumeration`,
  `lem:irredundancy`, `prop:integer-cramer`, `lem:rational-pipeline` —
  all agent-written proofs flagged "verify statement and proof" at
  `TASKS.md:372`; routine review.
- `thm:hko-counterexample` (629) — needs a citation or a self-contained
  construction + vol/cap/sys values.
- `fact:prefilter-*` chain (7 facts, lines 696–781) — textbook
  numerical-linear-algebra facts, all U without proofs or citations. A
  one-line citation each would clear them.

Library `algorithms/math.tex`:

- `def:rotation-increment` (479) — uses undefined symbol "CH2021";
  same issue blocks `TASKS.md:263` (tube rotation formula).
- `lem:prune-rotation` (542) — proof cites the undefined "CH2021" result.
- `lem:prune-simple` (562) — proof cites "HK2017 Theorem 1.5
  (simple_loop_theorem)" without expanding.
- `lem:fixed-point` (581) — time-index / closure-notation validation
  owed (embedded TODO at line 597).

Experiments:

- `prop:second-order-local-max` (second-order/math.tex:117) — proof is
  a sketch (compactness + upper-semicontinuity handwaves flagged by two
  TODOs at lines 14, 135).
- `lem:step-bound-incidence` (boundary-characterization:48) — GAP:
  first-order flip-time approximation; higher-order terms not bounded.
- `prop:sys-continuous` (boundary-characterization:96) — GAP: proof
  gives only lower semicontinuity; full continuity requires general
  Hausdorff-continuous-family theory.

Dev (lower priority, thesis-optional):

- All `lem:link-*` in `error-bounds/math.tex` — statement-only chain
  lemmas with 6 GAP markers (see §3.3). Low priority per
  `TASKS.md:208–229`; probably lives as an appendix in the thesis.
- `lem:sigma-structure` (algorithms/math.tex:262) — alternation argument
  flagged imprecise. Part of the tube / billiard side of the library.

## 5. Verification (sampling check)

Per the scaffold task requirement, I re-ran raw counts and sampled random
entries from the inventory to confirm the handoff's numbers match the
actual `math.tex` files.

### 5.1  Raw-count reconciliation

| pattern                   | grep result | inventory total | pass? |
| ------------------------- | ----------: | --------------: | :---: |
| `\begin{unverified}`      |          69 |              69 |  ✓   |
| `TODO:\s*JÖRN`            |          41 |              41 |  ✓   |
| `\[GAP`                   |          10 |              10 |  ✓   |

Grep commands run at the root of the worktree
`/workspaces/msc-math/.claude/worktrees/math-writeup-scaffold/` at the
time of writing; all three counts match to the unit.

### 5.2  Random sample — unverified blocks (5)

Drawn from §3.1; confirmed by `Read`-ing each file:line.

1. `geom/math.tex:133` → opens `\begin{unverified}`, wraps
   `\begin{proposition}[Capacity of symplectic products]…\label{prop:capacity-symplectic-product}` as claimed. ✓
2. `algorithms/math.tex:712` → opens unverified, wraps
   `\begin{lemma}…\label{lem:cap-derivative}`. ✓
3. `kkt/math.tex:219` → opens unverified, wraps
   `\begin{lemma}…\label{lem:numerical-transition-feasibility}`. ✓
4. `numerics/math.tex:85` → opens unverified, wraps
   `\begin{lemma}…\label{lem:orbit-feasibility-open}`. ✓
5. `rotated-regular-products/math.tex:50` → opens unverified, wraps
   `\begin{lemma}…\label{lem:rotation-fundamental-domain}`. ✓

### 5.3  Random sample — TODO/GAP markers (5)

1. `geom/math.tex:49` → `% [TODO: JÖRN - verify: J0 is defined…`. ✓
2. `algorithms/math.tex:533` → `% [TODO: JÖRN - the inequality A(gamma)…`. ✓
3. `boundary-characterization/math.tex:65` → `% [GAP - This is a first-order approximation…`. ✓
4. `error-bounds/math.tex:687` → `% [GAP - The value of eps_C is determined empirically…`. ✓
5. `second-order/math.tex:135` → `% [TODO: JÖRN - the proof below is a sketch…`. ✓

### 5.4  Random sample — dependency-graph edges (3)

Each sample: `Grep` the source file for `\ref{…}` / `\cref{…}` of the
alleged target label, confirm it exists in the target file.

1. `cor:sys-derivative → lem:vol-derivative, lem:cap-derivative` — the
   corollary's statement at `algorithms/math.tex:935` explicitly cites
   `Lemmas~\ref{lem:vol-derivative} and~\ref{lem:cap-derivative}`;
   targets exist at `algorithms/math.tex:786` and `:714`. ✓
2. `lem:sys-gradient-a → lem:cap-derivative` — **fails.** Direct grep
   over `boundary-characterization/math.tex` finds zero `\ref{lem:cap-derivative}`
   or `\cref{lem:cap-derivative}` occurrences. The file contains a
   `% TODO: add cross-references to capacity_derivatives_a and
   volume_derivatives_a lemmas` at line 177, which agent B mis-read as
   an active edge. This is a subagent over-confidence error; the dep-graph
   §2 has been corrected and a housekeeping note added. The pattern here
   ("TODO to add a cross-reference" treated as an edge) was not
   widespread — I re-grepped all cross-file citations directly to build
   the corrected cross-part table in §2, and the other edges were all
   confirmed.
3. `lem:fixed-point → def:tube-close` — at `algorithms/math.tex:654` the
   proof cites `Definition~\ref{def:tube-close}`; target exists at
   `algorithms/math.tex:495`. ✓

Pass rate: 2 of 3 on first sampling pass; the one failure was isolated
(single bad edge), cross-part table rebuilt from raw grep and re-verified.

### 5.5  Drift notes

- Agent A (library) reported 40 theorem-like envs *wrapped* in unverified,
  not 57 blocks. The delta (17) is made up of unverified blocks that
  wrap non-theorem content: the `fact:prefilter-*` chain (7 blocks) is
  correctly wrapped (facts are a theorem-like kind and counted by the
  agent), but several blocks wrap standalone proofs / remarks /
  def-interiors (e.g., geom/math.tex:910 wraps a prose block inside the
  prefilter-bound proof, not a theorem). This reconciles the 52 envs
  (40+4+8) vs 69 blocks gap.
- Agent-reported per-file GAP totals summed to 11, raw grep gives 10.
  The over-count is one boundary/error-bounds marker double-counted by
  the agent (falls within two environments' "15-line" window). The
  inventory uses the raw-grep 10.
- The 2026-04-07 TASKS entry reported "53 stubs + 69 unverified". Today
  41 TODO + 10 GAP = 51 "stub" markers, -2 from 2026-04-07. Unverified
  stable at 69. The -2 on TODO is consistent with the April code-cleanup
  bundle at `TASKS.md:330–333`.

## 6. Methodology

**Pipeline.** Three Explore subagents ran in parallel, each owning a
non-overlapping subset of the 27 `math.tex` files:

- Agent A — library (`geom`, `algorithms`, `kkt`, `library/src/math.tex`).
- Agent B — 12 experiment files under `exp-hko-local-maximum/`,
  `exp-sys-landscape/`, `exp-combinatorial-cells/`.
- Agent C — 10 dev files under `dev-algorithm-comparison/`,
  `dev-capacity-validation/`, `dev-gradient/`, `dev-numerical-analysis/`,
  plus the standalone `visualization/` and `crosspolytope/`.

Each agent extracted per-environment records (label, kind, file:line,
wrapped_unverified, has_proof, marker comments, refs, one-sentence hard-
labor note) and summary counts. The main thread reconciled against raw
`Grep` counts of `\begin{unverified}`, `TODO:\s*JÖRN`, `\[GAP`, then
sampled entries to verify (section 5).

**Marker definitions.**

- `\begin{unverified}…\end{unverified}` — red left-bar mdframed env from
  `crates/library/src/math-preamble.tex:27`. "Statement is not vetted
  by Jörn."
- `% [TODO: JÖRN - …]` — inline comment flagging a specific item for
  Jörn to verify/fix. Rule source: `.claude/rules/math-tex.md` §"Agent
  rules".
- `% [GAP - …]` — inline comment flagging a spot with above-ambient
  risk (handwave, empirical-only claim, missing constant derivation).
  Same source.

**Out of scope.** This audit does not:

- grade proofs for correctness (Jörn's work, not the scaffold's),
- fix anything — 0 math.tex files were modified,
- close `TASKS.md:362` (docs-group inventory) — this scaffold supersedes
  it for write-up purposes but the item stays open for post-write-up
  mechanical cleanup,
- cover the thesis side. `thesis/**.tex` is independent of
  `crates/**/math.tex` (per `.claude/rules/math-tex.md`).

**What this feeds.**

- Two-phase write-up pass: high-level notes (which results cleanly-
  restated, which need cleaves) → paragraph-level structure (what goes
  where, citations for each standard result). §2 gives the DAG; §4
  gives the priority queue.
- Kai briefing 2026-04-14 (`TASKS.md:298`): the top-10 list in §4 is the
  "what's still math-fragile" slice of that briefing. Jörn also has
  `handoffs/handoff-geom-math-review.md` (2026-03-24 top-to-bottom geom
  review, Defs 1–13) as historical context — several of its structural
  suggestions map onto §4 items 3 and 4 here.

**Next action owner.** Jörn. Flip `TASKS.md:292` to `[done]` after you
confirm the scaffold is useful; the scaffold itself doesn't mutate
TASKS.md.
