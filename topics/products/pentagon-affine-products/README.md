# Pentagon products: rotation, independent linear deformations, and proof routes

This directory owns the independent linear-deformation extension and the
[all-odd rotation theorem](odd-regular-angular-width.md). The self-contained
[research note](research.tex) supplies the affine pentagon statements, proofs
and primary references. The original
ChatGPT Pro ZIP is provenance, not a required input to reading or building this
note.

## Results and boundaries

For `P5` of circumradius one with outward facet normals
`n_i=(cos(2 pi i/5),sin(2 pi i/5))`, put `w=1+cos(pi/5)`.
For nonsingular real matrices `G,H`, let `M=G^{-1}H^{-T}` and
`m(M)=max_{i,j}|n_i^T M n_j|`. The note proves

```text
c_EHZ((a+G P5) × (b+H P5)) = w² / m(M),
c_EHZ² / (2 area(a+G P5) area(b+H P5)) ≤ (3+sqrt(5))/5.
```

Translations are irrelevant. The substantive extension is independent linear
maps on the two Lagrangian factors, including shears, stretches and reflections.
Equality in the systolic bound holds exactly when
`M=t R_(pi/10) U`, where `t>0` and `U` is a dihedral orthogonal symmetry of
`conv{±n_i}`. This is a global bound inside this family, **not** a bound for
arbitrary pentagons or arbitrary ten-facet polytopes.

The independent local-maximality theorem in the
[selected HKO chapter](../../../thesis/chapters/07-hko.tex) uses feasible upper
sections, exact finite predicates and a symmetry quotient. It covers nearby
ten-facet perturbations that can break the product structure; neither maximality
statement replaces the other. The older
[`hko-local-maximality-conditions.tex`](../../../formal/hko-local-maximality-conditions.tex)
contains an unresolved smooth-envelope proof sketch, not that theorem's proof.

## Reading routes and status

1. **Keep the short rotation proof first.** The accepted analytic argument is
   [the endpoint-interpolation proof](../pro-handoffs/pentagon-generalization/math/rotated-pentagon-proof.tex).
   Its selected reader-facing version is
   [the rotation chapter](../../../thesis/chapters/09-rotated-regular-polygons.tex);
   [the earlier replacement](../../../docs/history/pentagon-chapter-v2/chapter.tex)
   remains review history.
   The argument imports the independently known HKO endpoint capacity; its
   acceptance does not imply acceptance of every subsequent prose version.
2. **Extend to independent linear factors.** Sections 2–5 of
   [research.tex](research.tex) prove sparse reduction, width-body realization,
   full matrix containment using two pentagon identities, and the sharp area
   bound with equality. This proof does not import the HKO endpoint capacity.
   The existing six-facet reduction is recalled, not claimed as new.
3. **Keep the original CAS route as history.** The retained
   [executable proof and full run](../regular-products/pentagon-rotation-formula-proof/README.md)
   classify all 3,340 orderings on the open half-domain. The earlier route
   closes the endpoints by continuity and extends by symmetry; those are
   mathematical steps outside the finite run. Neither present analytic proof
   depends on rerunning it. Preserve code and output rather than appending
   the old classification to the new chapter.
4. **All odd regular rotation profiles (19 October 2026).** The separate
   [all-odd proof](odd-regular-angular-width.md), with
   [first](odd-regular-angular-width-audit.md) and
   [second](odd-regular-angular-width-second-audit.md) independent audits, proves
   `c(P_n ×_L R_theta P_m)=(1+h_n)(1+h_m) sec d_lcm(n,m)(theta)` for every odd
   `n,m>=5`, where `h_j=cos(pi/j)` and `d_L` is distance to the nearest multiple
   of `pi/L`. Its circle-deficit method gives strict angular containment except
   for the known 5×5 edge-contact case, and hence open labelled nonregular
   width-formula neighborhoods for every other pair. The sharp systolic ratio
   in this entire rotated odd regular class is `(3+sqrt(5))/5`, with equality
   exactly at `n=m=5`, `theta=pi/10+k pi/5`. Human acceptance and novelty remain
   open; this new theorem is outside the selected thesis. The stronger
   independent-affine extension still requires tensor containment. A separate
   [diagram and short explanation](circle-deficit-explanation-20261019.md)
   isolate the weighted-radius saving; its [scoped review](circle-deficit-review-20261019.md)
   preserves source identities without changing the hashed proof.
5. **Earlier angular proofs and useful limits.** The canonical note preserves
   the 7×7 and 5×7 proofs, strict-containment stability, interpolation sharpness
   and the triangle obstruction. The separate [nonagon norm proof](nonagon-angular-width.md)
   gives a reusable spectral estimate and an exact infinite family on which
   that sufficient estimate fails; the circle-deficit argument resolves the
   broader rotation problem without claiming independent-affine containment.

Two independent agent audits found no internal gap in the affine argument and
angular extensions. The [primary-source contract](source-contract.md) now closes
the previously outstanding Haim–Kislev factor/sign/hypothesis check. These are
agent-reviewed mathematical proofs; no human proof acceptance, prose PASS or
novelty determination is asserted for the extension. The symmetric-product
interpretation is optional and not an input to the affine proof.

The [selected affine thesis extension](../../../thesis/chapters/10-affine-pentagons.tex)
is included by [the sole thesis entry point](../../../thesis/main.tex), immediately
after the rotation proof. Inclusion does not establish human proof acceptance,
prose PASS or submission readiness. Current remaining obligations belong to
[the thesis work list](../../../meta-topics/project-management/thesis-work.md); earlier
integration records are historical evidence. The separately selected
[symmetric-partner theorem](../../../thesis/chapters/10b-symmetric-partners.tex)
is now integrated as Section 11. Source/math review, the thesis build and
focused page checks passed; human proof/prose acceptance and a whole-PDF PASS
remain open. The [sharp symmetrization plateau](../pentagon-arbitrary-partner/symmetrization-plateau-20261019.md)
is separate new research outside the selected thesis.

## Source identity, attribution, and maintenance

[Source manifest](sources.json) records SHA-256 identities of imported proof and
audit inputs. The immutable Pro return remains under
[`topics/products/pro-handoffs/pentagon-return/`](../pro-handoffs/pentagon-return/).
The maintained note differs from that return by repository provenance/status
text and the audited correction that the triangle's **coefficient hull** is a
hexagon: the coefficient set also contains zero. Future mathematical edits
belong here, not in the returned bytes.

The independent-affine pentagon argument originated in the ChatGPT Pro return delivered by Jörn on
18 September 2026. Codex performed the repository integration and independent
agent audits. The earlier endpoint-interpolation argument also originated in
AI-assisted review; see the replacement chapter's provenance. Preserve both
contributions when updating the thesis AI disclosure. The HKO counterexample
and benchmark value are external results, not newly discovered here. The
all-odd circle-deficit proof and nonagon norm note originated in the fresh
19 October 2026 research swarm; their retained audits identify the authors and
independent checking scopes.

Build from this directory:

```sh
latexmk -pdf -interaction=nonstopmode -halt-on-error -outdir=build research.tex
```

The PDF is `build/research.pdf`; build products are ignored. Compilation checks
syntax and cross-references, not proof soundness or readability. No experiments,
remote data or ZIP extraction are required.
