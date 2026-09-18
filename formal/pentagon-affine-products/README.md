# Pentagon products: rotation, independent linear deformations, and proof routes

This directory owns the maintained mathematical development of the independent
linear-deformation extension. Start with [research.tex](research.tex), a
self-contained note with statements, proofs and primary references. The original
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

The independent local-maximality theorem in
[`hko-local-maximality-conditions.tex`](../hko-local-maximality-conditions.tex)
retains its own feasible-section/symmetry hypotheses. It covers nearby
perturbations that can break the product structure; neither maximality statement
replaces the other.

## Reading routes and status

1. **Keep the short rotation proof first.** The accepted analytic argument is
   [the endpoint-interpolation proof](../../docs/pro-handoffs/pentagon-generalization/math/rotated-pentagon-proof.tex).
   Its reader-facing version is
   [the replacement chapter](../../docs/history/pentagon-chapter-v2/chapter.tex).
   The argument imports the independently known HKO endpoint capacity; its
   acceptance does not imply acceptance of every subsequent prose version.
2. **Extend to independent linear factors.** Sections 2–5 of
   [research.tex](research.tex) prove sparse reduction, width-body realization,
   full matrix containment using two pentagon identities, and the sharp area
   bound with equality. This proof does not import the HKO endpoint capacity.
   The existing six-facet reduction is recalled, not claimed as new.
3. **Keep the original CAS route as history.** The retained
   [executable proof and full run](../../experiments/regular-products/pentagon-rotation-formula-proof/README.md)
   classify all 3,340 open-domain orderings and cover exceptional parameters.
   This was the earlier exhaustive proof route; neither present analytic proof
   depends on rerunning it. Preserve code and output rather than appending the
   old classification to the new chapter.
4. **Optional further mathematics.** The canonical note also preserves complete
   7×7 and 5×7 rotation profiles, strict-containment stability, interpolation
   sharpness and the triangle obstruction. Those results are not automatically
   added to the agreed thesis chapter. Their angular containment does not imply
   arbitrary independent affine heptagon formulas.

Two independent agent audits found no internal gap in the affine argument and
angular extensions. The [primary-source contract](source-contract.md) now closes
the previously outstanding Haim–Kislev factor/sign/hypothesis check. These are
agent-reviewed mathematical proofs; no human proof acceptance, prose PASS or
novelty determination is asserted for the extension. The symmetric-product
interpretation is optional and not an input to the affine proof.

The [provisional thesis extension](../../thesis/chapters/10-affine-pentagons.tex)
is a separate reader-facing representation. It was initially delivered unwired;
this proposed integration checkout now includes it after the unchanged rotation
proof. This does not establish prose acceptance or select the final assembly.
The integration ledger records its adopted status and remaining obligations.

## Source identity, attribution, and maintenance

[Source manifest](sources.json) records SHA-256 identities of imported proof and
audit inputs. The immutable Pro return remains under
[`docs/pro-handoffs/pentagon-return/`](../../docs/pro-handoffs/pentagon-return/).
The maintained note differs from that return by repository provenance/status
text and the audited correction that the triangle's **coefficient hull** is a
hexagon: the coefficient set also contains zero. Future mathematical edits
belong here, not in the returned bytes.

The new argument originated in the ChatGPT Pro return delivered by Jörn on
18 September 2026. Codex performed the repository integration and independent
agent audits. The earlier endpoint-interpolation argument also originated in
AI-assisted review; see the replacement chapter's provenance. Preserve both
contributions when updating the thesis AI disclosure. The HKO counterexample
and benchmark value are external results, not newly discovered here.

Build from this directory:

```sh
latexmk -pdf -interaction=nonstopmode -halt-on-error -outdir=build research.tex
```

The PDF is `build/research.pdf`; build products are ignored. Compilation checks
syntax and cross-references, not proof soundness or readability. No experiments,
remote data or ZIP extraction are required.
