# HKO Local Maximum Proof-Route Checkpoint

Epistemic status: preservation note from an active proof-design session. This is
not final proof text and not a theorem certificate. It records the current
content-level argument, current numerical evidence, and the exact gaps that
must be closed before thesis-strength use.

Source session:
- Codex thread id: `019e8dca-b040-7613-a2fa-1dd1009254e3`.
- Thread name from session index: `Assess HKO local maximum proof`.
- Rollout log:
  `/home/vscode/.codex/sessions/2026/06/03/rollout-2026-06-03T14-02-11-019e8dca-b040-7613-a2fa-1dd1009254e3.jsonl`.
- Date of this checkpoint: 2026-06-03.

The rollout log is the source truth for what was said in chat. This note is
the maintained summary of the proof-route content from that session.

Refresh trigger: update this note if the active-branch diagnostic changes its
row counts, if Sage replaces the Rust-only evidence, if the singular-KKT rows
get a rigorous branch-gradient theory, or if the theorem target is weakened.

## 1. Purpose

This note preserves the proof route discussed in the source session. The main
reader is a future agent trying to finish the non-writing side of the HKO
local-maximum result.

The note intentionally focuses on one purpose: preserving the heterogeneous
proof content that is currently too developed to leave only in chat, but not
finished enough to move into polished thesis prose.

## 2. Target Theorem Content

The intended theorem is:

1. Work in the ten-facet dual-vertex chart `a in R^40` for HKO2024.
2. Quotient by the natural `sys` symmetries:
   translations, scaling, and the identity component of `Sp(4)`.
3. At the HKO point `a0`, the symmetry tangent space has dimension `15`.
4. Choose a `25`-dimensional slice transverse to those symmetry directions.
5. Prove that every nonzero slice direction has strictly negative first-order
   upper slope for `sys`.
6. Conclude that HKO is a strict local maximum on the quotient, inside the
   ten-facet model.

This is the theorem target only if the exact certificate closes. Current repo
claim status still treats broad HKO local maximality as conjectural; see
`research/hko-local-maximum-status.md`.

## 3. One-Sided Branch-Certificate Principle

The important simplification is that the local-maximum implication does not
require a complete catalogue of all branches.

For a branch `b`, let `S_b(a)` denote the systolic-ratio value obtained from
the branch action and the volume, whenever the branch is valid near `a0`.
Because the actual capacity is the minimum action over valid branches, the
actual systolic ratio satisfies

```text
sys(a) <= S_b(a)
```

for every valid selected branch `b`.

Therefore a sufficient certificate is:

1. Each selected branch is valid near `a0` in the directions where it is used.
2. Each selected branch has `S_b(a0) = sys(a0)`.
3. For every nonzero slice direction `h`, at least one selected branch has

   ```text
   D S_b(a0)[h] < 0.
   ```

Then along that direction,

```text
sys(a0 + t h) <= S_b(a0 + t h) < sys(a0)
```

for all sufficiently small positive `t`, assuming the branch has the required
first-order expansion with a controlled remainder.

This is why padded-zero `(sigma,beta)` pairs do not have to be listed merely
for completeness. They matter only if the selected positive branches fail to
cover all directions, or if the selected branches are not valid theorem
witnesses.

## 4. Convex-Hull Criterion On The Slice

The gradients should be used as covectors restricted to the `25`-dimensional
slice. Equivalently, project the ambient `R^40` rows to coordinates in a chosen
slice basis.

Let `g_1,...,g_N` be the projected rows for the selected branch gradients. A
convenient exact certificate is:

1. the projected rows span the `25`-dimensional slice;
2. there are coefficients `lambda_i > 0` with

   ```text
   sum_i lambda_i = 1,
   sum_i lambda_i g_i = 0.
   ```

These two facts imply that `0` is in the relative interior of the convex hull
of the projected rows. Hence for every nonzero slice direction `h`, some row
satisfies `<g_i,h> < 0`.

The non-strict condition `0 in conv{g_i}` alone only gives `<= 0` in every
direction. For strict local maximality on the quotient, the certificate should
prove the relative-interior condition, or an equivalent quantitative
separation statement.

The ambient rows should also annihilate the `15` symmetry tangent directions.
For an exact proof, Sage should verify this exactly. Numerically, the current
diagnostic found this to high precision.

## 5. Current Rust Diagnostic Evidence

Source command, in branch/worktree `hko-active-branch-diagnostic`:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic
```

The default output is the ignored smoke file
`experiments/hko-local-maximum/theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json`.
If that file is absent, regenerate it with the command above. Do not treat the
ignored smoke JSON as durable source truth.

Current smoke result from 2026-06-03:

1. `150` f64 strict-positive active branches were found at `a0`.
2. The action differences from the generated HKO minimum were between `0` and
   `4.440892098500626e-15`.
3. The exact symmetry tangent rank was `15`.
4. The exact slice dimension was `25`.
5. The projected f64 rows from all `150` active branches had rank `25`.
6. The smallest singular value of the all-branch projected row matrix was
   about `0.1762753505137759`.
7. The f64 LP found `0` in the convex hull with residual about
   `5.545614137174304e-10`.
8. The LP witness used positive weights on all `150` rows, and those positive
   rows had projected rank `25`.
9. The ambient f64 rows annihilated the symmetry tangent directions up to
   maximum absolute dot product about `7.2e-11`.

This is strong numerical evidence for the one-sided certificate if all `150`
rows are legitimate branch-gradient witnesses.

## 6. Singular-KKT Obstruction

The same diagnostic flags KKT singularity:

1. `44` branches have f64 KKT nullity `0`.
2. `106` branches have f64 KKT nullity `1`.
3. Every nonsingular branch has `sigma_len = 6`.
4. Every singular branch has `sigma_len = 7`.
5. Using only the `44` nonsingular rows gives projected rank `23`, not `25`.
6. The nonsingular-only LP still finds a convex-hull feasibility witness, but
   the positive-weight rows have projected rank `21`, so it does not certify
   strict negative slope in every slice direction.

Consequence:

1. The current proof route cannot simply discard singular KKT rows.
2. The current smooth implicit-function gradient formula is valid for the
   nonsingular rows but not automatically valid for the `106` singular rows.
3. The missing two slice dimensions appear to require either singular-family
   theory, additional nonsingular or padded-zero branches not currently in the
   diagnostic, or a different certificate.

This is the main mathematical obstruction after the 2026-06-03 diagnostic.

Thesis-facing takeaway to preserve:

```text
The witness is forced to use singular positive-beta seven-facet branches
because the nonsingular active branches cover only rank 23 of the
25-dimensional quotient slice; the later padded-once diagnostic did not find a
nonsingular minimum-action workaround.
```

## 7. Why Padded-Zero Branches Look Less Urgent

The current optimistic point is that all `150` rows used in the numerical
full-rank convex-hull witness have strictly positive f64 `beta` coordinates.
So the current all-row certificate does not rely on a branch whose base beta
lies on the boundary `beta_i = 0`.

This matters because a branch with `beta_i = 0` at `a0` may fail to be defined
on an open neighborhood and may only exist in some perturbation directions.
Avoiding those branches would make the certificate much simpler.

However, this is not yet a proof that padded-zero branches are irrelevant:

1. The Rust diagnostic intentionally did not enumerate padded-zero pairs.
2. If the singular `sigma_len = 7` rows cannot be justified, then padded-zero
   or other right-active germs may be needed to fill the two missing slice
   dimensions.
3. If the theorem route changes from a one-sided upper-bound certificate to an
   exact derivative computation for `sys`, then branch completeness and
   padded-zero right-active germs become relevant again.

So the current status is:

```text
No padded-zero branches appear needed for the optimistic one-sided route,
provided the singular positive-beta rows become theorem-valid.
```

## 8. How The Singular Rows Might Be Saved

Existing exact-witness artifacts suggest a plausible route for the singular
seven-facet rows:

1. `research/hko-local-maximum-exact-witness.md` records that the seven-facet
   exact minima are consistent with equality-case trajectories from the HKO
   minimizing family.
2. `experiments/hko-local-maximum/theorem/exact-witness/segment-gradient-reduction.json`
   and `segment-a-gradient-reduction.json` record exact segment-gradient
   reduction facts.
3. The `segment-a-gradient-reduction.json` theorem-use field says the exact
   capacity row on the seven-facet KKT segment is a degree-2 polynomial in the
   segment parameter and agrees with interpolation through `0`, `1/2`, and
   `1`.
4. The segment endpoints coincide exactly with the corresponding six-facet
   endpoint rows.

Possible proof path:

1. Treat each singular seven-facet row as a row from a verified local family,
   not as the derivative of an isolated nonsingular optimizer.
2. Prove in LaTeX that the selected row is a valid first-order upper branch for
   the action/sys value, or prove the required directional one-sided inequality
   directly from the exact family.
3. Implement the exact family checks in Sage so the final witness verifies
   row validity without trusting Rust's singular KKT derivative computation.

This would avoid the heavy general semialgebraic active-germ route, but it
still requires a rigorous singular-family argument.

## 9. Endpoint Rows And The Minimizing Family

The existence of nonsingular active rows is not in conflict with the HKO
minimizing family.

Current geometric example:

1. The nonsingular row `sigma = [0, 1, 7, 3, 9, 5]` has f64 KKT nullity `0`.
2. Its q-plane boundary points are the pentagon vertices
   `(cos(2pi/5), sin(2pi/5))`, `(cos(2pi/5), -sin(2pi/5))`, and `(1,0)`.
3. The q-motion is the triangle through those three vertices.
4. The active q-facets at the three q-plane boundary points are `[0,1]`,
   `[3,4]`, and `[0,4]`.

Interpretation:

1. A six-facet endpoint row can be isolated inside its own positive-beta
   support chart.
2. The family direction can leave that chart by adjoining an extra facet whose
   beta coordinate is zero at the endpoint and positive along the seven-facet
   family.
3. In that enlarged seven-facet chart, the KKT matrix can be singular and
   carry the family direction.

This point should be mentioned in the thesis or proof explanation because Kai
may reasonably ask how nonsingular active branches can coexist with the
HKO2024 minimizing family.

## 10. Padded-Endpoint Alternative

There is a possible alternative to using the singular seven-facet rows
directly.

Start with a nonsingular six-facet endpoint row. Insert one missing facet into
the cyclic word. If the resulting seven-facet equality-constrained KKT system
is nonsingular, has minimum action, has exactly one beta coordinate equal to
zero, and all other beta coordinates are positive, then the equality-branch
solution is smooth.

This would not produce an ordinary full halfplane in direction space. It would
produce a one-sided branch usable only in directions where the zero beta
coordinate becomes positive. Therefore a theorem certificate using these rows
needs both:

1. the `D_a sys` inequality for the equality branch; and
2. the linearized activation condition for the zero beta coordinate.

So these rows contribute cones, not full halfplanes. They may still be useful
if the union of those cones covers the missing slice directions.

The Rust diagnostic
`experiments/hko-local-maximum/theorem/active-branch-diagnostic/main.rs` now records a
f64 triage list of such one-facet padded extensions.

Current padded-extension diagnostic from 2026-06-03:

1. Starting from the `44` nonsingular six-facet active rows, the diagnostic
   generated `1232` one-facet insertion sources.
2. After quotienting by cyclic rotation, these gave `955` unique padded
   seven-facet words.
3. `137` of the unique padded words had singular f64 KKT matrices and were
   dropped by the smooth padded-row filter.
4. `818` had nonsingular f64 KKT matrices, but every one had at least one
   negative beta coordinate in the direct equality-constrained KKT solution.
5. No nonsingular padded row had minimum action.
6. No nonsingular padded row had exactly one zero beta coordinate with the
   other beta coordinates positive.
7. `105` of the `106` active singular seven-facet rows appeared among the
   singular padded-extension words.

Interpretation:

```text
The simple nonsingular padded-once alternative currently has no surviving
rows. The family-relevant padded words appear to be singular, not nonsingular
smooth equality branches.
```

This supports the thesis-facing explanation in Section 6: the singular rows
are not merely a bookkeeping complication, but currently appear to be the rows
that supply the two quotient-slice directions missing from the smooth-only
witness.

## 11. Candidate Sage Decision Problem

A feasible theorem-facing decision problem could ask Sage to verify a witness
with the following data.

Branch rows:

1. `sigma`;
2. exact `beta`;
3. exact branch action;
4. exact `D_a action` row or exact family-gradient data;
5. exact `D_a volume` row, shared across branches;
6. exact `D_a sys` row.

Symmetry data:

1. the `15` symmetry tangent generators in `R^40`;
2. exact rank `15`;
3. an exact slice basis, for example a basis of the kernel of the transpose of
   the symmetry tangent matrix.

Branch checks:

1. `beta_i > 0` for the currently selected positive branches;
2. `sum_i beta_i = 1`;
3. `sum_i beta_i a_{sigma(i)} = 0`;
4. branch action equals the exact HKO2024 minimum action;
5. nonsingular rows have exact KKT nullity `0`;
6. singular rows have a separate exact family or one-sided-gradient
   certificate;
7. the `D_a sys` row agrees with the proved formula or family certificate.

Cone checks:

1. each `D_a sys` row annihilates the `15` symmetry tangent generators;
2. projected rows have rank `25`;
3. an exact convex-combination witness puts `0` in the relative interior of the
   convex hull of the projected rows.

The proof then only needs to show:

```text
Sage decision problem true
=> selected branches give a one-sided decreasing upper branch in every
   nonzero quotient direction
=> HKO is locally maximal in the ten-facet quotient model.
```

No equivalence theorem and no complete branch enumeration are needed for this
implication.

## 12. Proof Gaps To Close

The non-writing work remaining is concentrated in these gaps.

1. Exact row generation:
   produce the final witness rows over `Q(tan(pi/5))`, not f64.

2. Exact row validity:
   verify closure, normalization, positivity, action equality, and branch
   feasibility for every selected row.

3. Nonsingular gradient formula:
   prove the formula for `D_a action` and `D_a sys` in the positive-beta,
   KKT-nullity-zero case, and make Sage check the exact row.

4. Singular row theory:
   either prove a singular-family/family-gradient theorem for the `106`
   seven-facet rows, or replace them with theorem-valid rows.

5. Convex-hull certificate:
   replace the f64 LP witness by exact rational/algebraic coefficients or by a
   Sage-verifiable exact sign/rank certificate.

6. Local implication:
   write the finite-branch Taylor or one-sided expansion argument that turns
   relative-interior convex-hull coverage into strict local maximality on the
   quotient.

7. Padded-zero fallback:
   only if the singular positive-beta rows cannot be used, investigate
   padded-zero/right-active germs or additional nonsingular rows.

## 13. Current Triage

Best next mathematical question:

```text
Can the singular sigma_len = 7, beta > 0 rows be turned into exact valid
one-sided branch-gradient witnesses?
```

Reason:

1. If yes, the current `150`-row numerical result already has the right shape:
   rank `25`, strict convex-hull coverage, and symmetry annihilation.
2. If no, the nonsingular subset is rank `23`, so the current smooth-only
   theorem route is missing two slice dimensions.
3. Padded-zero branches are not the first thing to enumerate unless the
   singular-row route fails or exact checks reveal that the `150`-row coverage
   was f64-only noise.

## 14. Relation To Existing Notes

This note should be read with:

1. `research/hko-local-maximum-status.md` for current claim strength.
2. `research/hko-local-maximum.md` for broader HKO evidence and task state.
3. `research/hko-local-maximum-exact-witness.md` for exact-witness artifacts.
4. `research/sys-first-order-local-behavior.md` for the heavier general theory
   that handles singular KKT systems and right-active germs.
5. `thesis/hko-local-maximum-content.md` for thesis packet structure.

If this route succeeds, the final thesis writeup should not reproduce this
whole note. It should use this note to assemble:

1. theorem statement;
2. Sage witness contract;
3. branch-to-local-maximum implication proof;
4. short explanation of why completeness of all branches is not needed for the
   one-sided certificate;
5. precise handling of singular seven-facet branches.
