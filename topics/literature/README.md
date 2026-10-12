# Literature sources and search records

This topic owns the project's cached source collection, citation-numbering checks,
and source-search records. It records what source material is available and where
project interpretations live. A cached paper is evidence for what its authors
state; it does not by itself establish a project proof, implementation result,
or active thesis claim.

## Collection and provenance

The [source collection](sources/README.md) contains three LaTeX source
snapshots with figures: Haim–Kislev's 2017 paper on the EHZ
capacity of polytopes, Chaidez–Hutchings' 2021 paper on Reeb dynamics on
four-dimensional convex polytopes, and Haim–Kislev–Ostrover's 2024
counterexample paper. Their source directories retain the source files and
figures as collected. Git history records a 30-file initial source/figure
import in commit `ce7aa06f`. It does not provide a per-file license or a
complete upstream revision/URL manifest for all three snapshots; do not infer
publication or redistribution permission from their presence here. Retain
source notices and check the published version when quoting or relying on
numbered results.

Two adjacent project notes analyze the HK2017 reference MATLAB implementation
and check the coordinate and symplectic sign conventions. They are project
analyses, not source-paper text, and their conclusions must be followed to the
named source and mathematical owner. The collection README lists their exact
paths and distinguishes them from cached inputs.

`citation-index.md` records checked theorem and section numbering, source
availability, and a set of numerical-linear-algebra and symplectic-geometry
references. Its May 2026 date and its coverage of `crates/**/math.tex` are
historical metadata, not a claim that every current citation or source path has
been rechecked. The source-availability table distinguishes repository source,
external-only source, and local-only copies. `sources/.gitignore` excludes
unauthorized local books and PDF-only references from version control; their
absence from Git is intentional.

## Search scope and interpretation owners

The retained material has two distinct scopes. The cached collection is a
bounded working set for source checks, centered on three papers repeatedly
used by the thesis and formal development. The [open-thesis literature map](open-thesis/README.md)
records targeted searches for neighboring algorithm work, products, HKO local
maximality, geometric alternatives, and mathematical-discovery case studies.
Its notes preserve exact editions and theorem/section pinpoints, read depth,
limitations, and search/reopening records. It is neither a comprehensive
bibliography nor a certificate of priority or novelty; abstract-only leads are
identified as such. The [algorithm prior-art comparison](algorithm-prior-art/README.md)
is a narrower follow-up comparing two specific algorithms. Its proposed
related-work inserts are editorial proposals, not approved thesis prose.

Broader references are indexed where project work recorded them; availability
and numbering assertions should be checked against the linked publication
before use. Searches for a new claim should start from the mathematical topic
and its source trail rather than treating this collection as exhaustive.

- The HK2017 capacity formula, word conventions, and quadratic-program
  translation belong to [capacity and QP](../capacity-and-qp/README.md).
- The CH2021 geometric comparison and the limits of transferring its flow-graph
  ideas belong to [flow graphs](../flow-graphs/README.md).
- The HKO2024 counterexample and product-specific project arguments belong to
  [products](../products/README.md); its broader convex-geometric inputs route
  through [polytope geometry](../polytope-geometry/README.md).

These topic accounts own mathematical interpretation and project claims. This
literature account makes no claim promotion and does not replace their proof,
implementation, evidence, or thesis links.
