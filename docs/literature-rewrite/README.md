# Literature-context replacement packet

This directory contains a fresh replacement, not a repair of the rejected
introduction paragraph.

- `evidence-map.md` records the accepted claims, direct primary-source links,
  version dates, pinpoints and scope limits.
- `replacement-context.tex` is the integration-ready LaTeX passage. It uses
  three subsections organized around the post-counterexample mathematical
  questions, the product/local boundary, and the role of finite capacity
  methods.
- `references.bib` contains every bibliography entry used by the passage.
- `standalone.tex` and `standalone.pdf` provide a reviewable rendering.
- `luna-product-context.md`, when present, is the independent Luna discovery
  note used to audit the product-exclusion and nonsmooth-cuts claims; it is
  background evidence rather than prose to integrate.

The passage deliberately does not claim a comprehensive literature survey or
publication-priority determination. It positions the thesis contributions
against the directly relevant primary results checked on 16 September 2026.
It was compiled with `latexmk -pdf -interaction=nonstopmode -halt-on-error
standalone.tex`; the final log had no citation, box or other LaTeX warnings.
