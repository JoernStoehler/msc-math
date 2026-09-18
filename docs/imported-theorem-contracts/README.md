# Imported theorem contracts: first closure slice

Checked 18 September 2026. Owner: `imported_theorem_contracts`. This is an audit handoff, not a production patch or a complete proof review.

**Outcome:** the imported Rudolf and AAO conclusions used by the inspected assembly apply. No missing smoothness, strict-convexity, origin, dimension, sign, or factor hypothesis was found. The apparent Rudolf theorem-number discrepancy is a journal/arXiv numbering difference, not an incorrect citation. Remaining project-specific surgery and QP derivations are separate obligations.

## Actual consumers

Read the current interrupted assembly at `/tmp/msc-math-thesis-review-20260917/thesis/candidate/02-preliminaries.tex` (AAO uses around lines 167, 177, 377; normalization around 487) and `04-quadratic-program.tex` (Rudolf use around line 316), alongside recovered fragments in `thesis/candidate/recovered/thesis-candidate/`. The interrupted preliminaries already explicitly attribute the unit-period/action-one-half convention to Haim–Kislev; the older recovered fragment merely says “the cited fixed-period formulation.” Preserve the clearer attribution at integration.

This pass reused `docs/open-thesis-literature/{products,algorithms}.md`. It did not repeat the affine-pentagon Haim–Kislev formula audit, classify every project orbit, or inspect runtime correspondence.

## Rudolf: nonsmooth product billiards

Primary sources inspected: [publisher full text](https://link.springer.com/article/10.1007/s10884-022-10228-0), especially Definition 2, equation (3), Theorem 1 and its following warning; [arXiv 2203.01718v2](https://arxiv.org/pdf/2203.01718v2), pp. 1–7 and proof pp. 9–11. Publisher numbering is Definition 2/Theorem 1; preprint numbering is Definition 1.2/Theorem 1.3. The existing bibliography correctly records online publication 2022 through a stable key and journal volume 36 (2024).

The imported conclusion is existence of a capacity-minimizing **strong** billiard with at most n+1 bounce points, with a simultaneous dual polygon satisfying

- `q[j+1]-q[j] ∈ N_T(p[j])`,
- `p[j+1]-p[j] ∈ -N_K(q[j+1])`.

The length is the sum of the polar gauge `g_(T polar)` on q-increments, equivalently the support function `h_T`. For n=2 there are two or three bounce points. The theorem applies to convex bodies with nonempty interior; it needs neither smoothness nor strict convexity. The thesis explicitly assumes nondegenerate planar convex polygons with interior origins and a Lagrangian product, so its inputs satisfy the source conditions.

Do not substitute weak billiards: the source explicitly warns that for non-strictly-convex factors their infimum can be smaller. The thesis correctly retains the strong normal-cone relations. The alternating lift lies on the product boundary: one factor stays at its boundary vertex while the other traverses an interior segment. Rudolf uses `J_R(q,p)=(p,-q)=-J_0(q,p)`. Reversing the lift converts its characteristic direction to the thesis convention. Since the thesis primitive differs from `-p dq` by an exact differential, the reversed lift has thesis action equal to the positive support-function length. This verifies the imported starting curve and its action, not the subsequent splitting/merging surgery.

## AAO: nonsmooth capacity and dual attainment

Primary source inspected: [Artstein-Avidan–Ostrover, arXiv 1111.2353v3](https://arxiv.org/pdf/1111.2353v3), §2.2–2.3 (pp. 6–8), Appendix §5 (pp. 17–23). Pinpoints: Theorem 2.2 for smooth least action, Lemma 2.4 for continuous extension, Proposition 2.5 for nonsmooth dual attainment, Proposition 2.7 for nonsmooth least action, Lemmas 5.1–5.2 for weak-critical reconstruction and minimizer criticality. Their product-billiard Theorem 2.13 requires smooth strictly convex factors and must not replace Rudolf here.

The generalized/dual results themselves are stated for arbitrary convex bodies, despite the appendix noting products as its intended application. The thesis's compact full-dimensional convex bodies with interior origin fit; nonsmooth polytopes are not excluded. The source's generalized characteristic definition uses piecewise-smooth language; its variational proof works with Sobolev curves and weak critical points. For the thesis's polytope application, the explicitly Sobolev Haim–Kislev formulation supplies the direct bridge. This check does not upgrade arbitrary nonconvex or lower-dimensional inputs.

A direct normalization check avoids confusing AAO's functional with the thesis's. Let `z:[0,T]→R^4` have thesis action `A(z)=T`, and subtract its mean if necessary. Put

```
u(s) = -J_0 z(T s/(2π)) / sqrt(T),    0 ≤ s ≤ 2π.
```

Then `A(u)=1`, and

```
(π/2) ∫_0^(2π) h_K(u'(s))² ds
    = (1/4) ∫_0^T h_K(-J_0 z'(t))² dt
    = I_K(z).
```

Thus AAO's exact admissible loop space and capacity minimum transfer to the thesis's variable-period normalization. The invertible rescaling also transfers attainment. Haim–Kislev's period-one/action-one-half version instead gives `c=2 min I`; the thesis already includes that conversion correctly. AAO's Lemma 5.2 provides the nonsmooth multiplier/criticality assertion; the thesis independently calculates its sign, multiplier `ν=I/T`, and reconstruction. Nothing here licenses reconstructing a merely feasible nonminimal competitor onto the boundary.

## Remaining boundary and integration advice

No mathematical repair is requested for these imports. Preserve the strong-billiard qualification, the period normalization explanation and the proof's distinction between dual feasibility and boundary reconstruction. The finite block bound after surgery, any claim about all minimizers rather than existence of one, and runtime enumeration coverage require the other assigned reviews. This first slice is closed; it is not a claim that every imported theorem in every chapter has been checked.

Source retrieval hashes (PDF bytes; no external PDFs redistributed):

- AAO v3: `37c7001fc917de31e1fe641e5aefab81c8bafd908fb9c660a9b2c587eaef0822`.
- Rudolf v2: `fe5e822769832b8626d83efa1027bbcdc8badf838b130f70c2d192c1b1bf9754`.
