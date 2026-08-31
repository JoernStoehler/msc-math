# Figure Review

Use this reference before accepting a new or materially changed figure or when
assigning figure review. Also use `model-figure-reader-effects.md`.

## Review Inputs

Use the exact rendered thesis pages at normal reading scale. Include the figure
brief, surrounding prose, caption, and producer/source contract, but stage them
as described below. Do not substitute a standalone high-resolution asset for
the integrated pages.

## Pass 1: Cold Reader Effect

Inspect the pages before reading the developer's intended interpretation or
implementation account. Report:

- what attracts attention first;
- what objects and relationships appear to be shown;
- the inferred reading or traversal order;
- comparisons the display appears to invite;
- the conclusion or purpose inferred from the page;
- content that cannot be perceived or distinguished without unusual zooming.

This pass tests what the figure communicates, not whether the reviewer can
confirm the developer's explanation after being told it.

## Pass 2: Brief-Relative Meaning

Read the figure brief, surrounding prose, and caption. Check:

- which reader questions the figure answers and which remain costly or
  unanswered;
- where actual interpretation differs from the intended inference;
- whether the intended use is better served than by prose alone;
- which material false inference is most likely;
- whether coordinates, projections, scaling, selection, uncertainty, and
  epistemic status are disclosed when they change interpretation.

## Pass 3: Mathematical And Production Integrity

Inspect the source data and producer as needed. Check that mathematical objects,
labels, transitions, plotted values, and invariants agree; the caption does not
overclaim; the producer owns formatting and regeneration; experiment and thesis
copies have the intended relation; and the final thesis build uses the reviewed
asset.

## Findings

Prioritize conceptual blockers before local polish. For a material finding,
give:

```text
Severity:
Blocked reader task:
Observed interpretation or perceptual failure:
Likely visual cause:
Smallest plausible repair:
```

Do not reject a figure only because it departs from a familiar style, and do
not accept it only because labels are legible or nominal style values pass.

## Review Scope

Use one proportionate review for a conventional figure. For an important new
mathematical diagram or geometric process figure, separate cold reader-effect
and mathematical-meaning review, using two agents when independence materially
improves confidence. For an evidence-bearing plot, include an experiment or
quantitative-claim review. Reuse the same reviewer to confirm repairs to its
findings; use a fresh reviewer when an unprimed interpretation is the evidence
needed.

## Readiness Judgment

Treat a figure as ready only when the required information is perceptible at
normal final-PDF size; actual interpretation substantially matches the
intended inference; the named reader task is easier than from prose alone;
encodings support valid comparisons without an undisclosed material false
inference; mathematics, data, caption, and epistemic role agree; and producer,
regeneration, thesis copy, and build state are sound. Remaining defects must be
independent polish rather than blocked understanding.

This is a contextual judgment, not a sufficient checklist. Report the reader
use, reviewed output, producer route, rendered-page review, and intentionally
deferred polish.
