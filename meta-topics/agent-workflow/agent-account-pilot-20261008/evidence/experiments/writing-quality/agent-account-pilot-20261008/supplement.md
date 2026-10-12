# Companion notes for a downstream scientific author

Prototype, 8 October 2026; no human acceptance implied. The
[scientific packet](../../../docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/symmetric-partner-lemma.md)
is the main account. This supplement accompanies it and collects interfaces
and presentation decisions; it supplies no new theorem or proof check.

**Conventions to carry across the interface.** The fixed factor is the regular
pentagon (Q) with circumradius one and outward normals
(n_i=(\cos(2\pi i/5),\sin(2\pi i/5))). Its partner is (K), and the
reference decagon is (B=\tfrac12(Q-Q)^\circ). Preserve that orientation:
the equality class allows translations and positive dilations of (B), not
arbitrary rotations relative to (Q). The product is Lagrangian;
(c=c_{\mathrm{EHZ}}), and the ratio uses denominator
(2\operatorname{area}(K)\operatorname{area}(Q)). The packet's (D=Q-Q)
must not overwrite the preceding thesis section's different decagon (D).
Oriented edge length is the support-function length (s_Q(v)), not
Euclidean length; odd template rotations reverse traversal orientation.

**Inclusion status and placement.** The
[current backlog](../../../docs/coordination/thesis-work.md) records Jörn's
3 October selection for inclusion, while proof acceptance and presentation
remain open. The result is currently omitted from the selected manuscript.
The [reader-facing candidate](../../../docs/history/symmetric-partner-candidate-2026-10-03/README.md)
places it after the independent-affine-pentagon section and before
product-position variation. Those are exposition and endpoint interfaces;
the affine theorem is not a proof input. Comparing compact and method-first
presentations remains an open authoring choice.

**Figure and optional consequences.** Reuse or adapt the actual translated
template placement in
[section.tex](../../../docs/history/symmetric-partner-candidate-2026-10-03/section.tex):
the longest edge joins antipodal decagon vertices; the third vertex lies inside
the incircle. Its caption should retain the rotation qualification alongside
the oriented-length explanation. The standalone candidate was visually checked
at two pages; integration can change layout. It includes the symmetrized-capacity
interpretation, but omits the area-asymmetry threshold and empirical plateau
discussion. Those omissions are presentation choices, not rejected results;
including them requires connections to the selected experiment prose.

**Evidence boundaries.** The candidate checks the imported criterion in BMP
v2; the original packet cites v1. Retain attribution and version provenance.
The [focused source check](../../../docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/source-check.md)
supports only a bounded nonduplication search, not publication novelty.
Numerical reference fits check implementation, not the proof. Neither this
formula nor the symmetric optimum establishes (c=W) for asymmetric partners
or global optimality of the HKO partner. Building the existing candidate needs
no new producer run or certificate computation.
