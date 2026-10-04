# Symmetric-partner exposition candidate

Prepared on 3 October 2026 for the result Jörn selected for inclusion.
This is a bounded reader-facing candidate, outside `thesis/main.tex`.
It has not received human proof or prose acceptance.

## Intended placement and scientific purpose

Place after the independent-affine-pentagon section and before product-position
variation. The opening transition assumes that context. The formula covers
arbitrary centrally symmetric partners of a fixed regular pentagon, including
smooth bodies; the affine-pentagon theorem covers a different class. The latter
is not a mathematical input. The product-position theorem can use this capacity
as its Lagrangian endpoint input.

The candidate preserves the exact capacity formula, sharp ratio/equality class,
the single-reference-cover calibration proof, and the interpretation of the
width bound as symmetrized capacity. It omits the area-asymmetry threshold and
empirical plateau discussion, which would introduce another measure and require
explicit connections to selected experiment prose. Those consequences remain
available in the source packet; omission here is a first presentation choice,
not a scope rejection.

## Dependencies and checks

- Project mathematical source:
  `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/symmetric-partner-lemma.md`.
- Imported covering criterion and ten pentagonal templates:
  [Balitskiy--Mitrofanov--Polyanskii v2](https://arxiv.org/html/2603.12495v2),
  Theorems 2.5 and 3.3, Definition 3.2 and Section 6.1. The source packet cites
  v1; this candidate checks the relevant statements in v2.
- Capacity monotonicity, translation invariance and homogeneity in one factor.
- Elementary regular-decagon geometry and strict area increase under proper
  containment of full-dimensional convex bodies.

The candidate keeps Chapter 10's normal convention, calls the fixed pentagon
`Q` and the reference decagon `B`, and avoids reusing its `D` for a different
decagon. It explicitly distinguishes oriented support-function length from
Euclidean length. The figure draws the actual template placement, rather than
an unrelated shape illustration.

Two LaTeX passes produced **two A4 pages**, including one figure and the
standalone bibliography, with the selected thesis's 11pt font and 26mm margins.
Both rendered pages were visually inspected. The final log had no unresolved
citations or overfull/underfull warnings. This measures standalone layout;
integration can change numbering and page breaks. It does not establish human
readability, mathematical acceptance or publication novelty.

A bounded independent source/convention review by `research_choices` found no
material mathematical error. Its two precision suggestions were incorporated:
the definition admits degenerate templates when opposite normals exist, and
odd template rotations reverse the traversal orientation. Review remains
distinct from human acceptance.

## Rebuild

From the repository root:

```bash
mkdir -p tmp/pdfs/symmetric-partner
cd thesis/candidates/symmetric-partner
pdflatex -interaction=nonstopmode -halt-on-error -output-directory=../../../tmp/pdfs/symmetric-partner main.tex
pdflatex -interaction=nonstopmode -halt-on-error -output-directory=../../../tmp/pdfs/symmetric-partner main.tex
```

The review PDF is retained at `output/pdf/symmetric-partner-candidate.pdf`.
`section.tex` owns the candidate text and TikZ figure; `main.tex` is only its
standalone build wrapper. No producer, exact certificate or large computation
is required to build this candidate.
