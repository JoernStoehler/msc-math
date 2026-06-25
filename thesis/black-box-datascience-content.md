# Black-Box Data-Science Content Notes

Status: section-local content companion for `thesis/08-black-box-datascience.tex`.
Not source truth.

Purpose: gather the data-science search-result writing inventory, result
taxonomy, and open decisions before final prose is written.

Overruled by: `experiments/sys-landscape/`, `research/sys-landscape*.md`, task
files, generated tables/figures, and Jörn/Kai review.

Lifecycle: keep while the section is being assembled. After the section is
stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Result Shape

- State the method-table result: the closed method table records no new source
  of `sys > 1` examples and no candidate-proposer for finding one, beyond
  examples already explained by HKO2024 and its symplectic images or controlled
  perturbations.
- Current Jörn estimate, 2026-06-11: about 80% that the thesis will retain a
  solid negative result here. In the remaining about 20% of outcomes, a final
  experiment may produce a positive result and the chapter should pivot
  honestly instead of defending the negative framing.
- Rows with no candidate-proposer can be stated in batches because many rows
  have the same conclusion. The main text can use a table or bullet list.

## Random Polytopes

- State how random polytopes were generated and what search question this
  tests.
- Needs source: generator, parameters, row counts, retained facets,
  normalization, and exact no-new-row claim licensed by the sample.

## Gradient Ascent

- State how local ascent was run and what search question this tests.
- Needs source: fixed-F/product/continuation variants, seed counts, stopping
  rules, escape logic, and whether the claim is local-search-only evidence.

## Rows

- Define table rows as polytopes.
- Row families may include random polytopes, products, ascent endpoints,
  continuation endpoints, and retained enumeration families.
- Avoid mixing the non-black-box known HKO2024 `n=1` positive sample into the
  black-box data-science table; methods can memorize that case without teaching
  us anything new.
- Open decision: finalize row families when the data-science writer session
  starts or the branch stabilizes.

## Columns

- Define columns as features of polytopes: symplectic invariants,
  geometric/orbit features, and metadata.
- Metadata columns are useful caveats because a method may learn data
  provenance rather than geometry.

## Result Types

- `not applicable`: the method does not fit the available data or search
  interface.
- `not run within the stated bound`: implementation, runtime, or data
  requirements exceeded the stated thesis bound.
- `implementation bug; no method verdict`: a run artifact exists but is not
  interpretable as method evidence.
- `ran with no candidate-proposer and no new validated row`: the method ran but
  recorded neither a candidate-proposer nor a validated new `sys > 1` row.
- `candidate-proposer`: a reproducible rule proposes candidate polytopes or
  rows before their `sys` values are evaluated. For data-science rows, it must
  not use endpoint labels, producer identity, optimizer provenance, or post-hoc
  inspection of `sys`.
- `validated candidate`: a run produced a new row with verified `sys > 1`.
- If a real positive result appears, report it honestly and escalate to Jörn. It
  may falsify the current "insufficient" main result and justify follow-up.
- Filter obvious false positives, for example a model finding a within-table
  association because `sys` was regressed against `sys`.

## Methods

- Batch attempted methods by verdict.
- Put detailed figures, tables, parameters, and method-specific notes in the
  data-science appendix.
- Methods come from the data-science toolboxes actually used and assigned to
  Codex agents.
- Open decision: finalize the method list during the data-science chapter
  writer session.
