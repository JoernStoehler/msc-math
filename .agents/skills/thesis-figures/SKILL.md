---
name: thesis-figures
description: Use when Codex decides whether to create, creates, edits, integrates, reviews, or delegates work on a thesis-facing or experiment-support figure, plot, diagram, sketch, screenshot presented as a figure, visual data display, or figure-like composite. Owns the complete figure workflow from reader purpose and mathematical design through producer code, thesis placement, rendered-page review, and regeneration.
---

# Thesis Figures

A figure succeeds when its rendered form helps the intended thesis reader
understand, compare, remember, or assess something more effectively than prose
alone, without inviting a material false inference. Concrete properties such as
font size, cropping, contrast, camera angle, and line width matter through that
downstream effect; satisfying them does not by itself make a figure useful.

Read `references/reader-effects.md` for every nontrivial figure. Also read:

- `references/geometry-and-process.md` for mathematical diagrams, projected
  geometry, trajectories, small multiples in local coordinates, and algorithm
  sequences;
- `references/quantitative-plots.md` for empirical or numerical plots;
- `references/review.md` before reviewing a candidate or assigning figure
  review.

## Figure Brief

Before implementing a new figure or materially redesigning one, establish the
following in a short `/tmp/` brief or an equivalent task-local note:

- downstream use and intended reader state;
- reader questions and intended inference;
- relationships that must be perceptible;
- comparisons that must remain valid and comparisons the display invalidates;
- the most likely material false inference;
- explanatory, evidentiary, diagnostic, or proof-input status;
- intended thesis location and rendered size;
- source truth, producer, and regeneration owner.

Infer these from the thesis and owner-local context when possible. Ask Jörn only
when the crux depends on his taste, private context, or an unsettled thesis
choice. Do not turn the brief into a questionnaire or durable bureaucracy.

## Develop And Integrate

1. Inspect the surrounding thesis text, existing producers/assets, and final
   output constraints before choosing an encoding.
2. Choose the cheapest rough render that tests whether the visual concept
   answers the reader questions. Do not polish or build extensive validation
   around an untested concept.
3. Let producer code own fonts, sizes, colors, labels, and layout. Use
   `experiments/figure_config.py` when relevant, and run dependency-declaring
   Python producers with `uv run --script`.
4. Assert mathematical or data invariants needed to keep computed figures from
   depicting an impossible or misidentified object. State whether the figure
   explains, provides evidence, or enters a proof.
5. Keep experiment figures beside their producer. Deliberately copy selected
   publication assets into the self-contained `thesis/` tree; never patch a
   generated output by hand. Record whether an owned figure is a draft,
   candidate, rejected, or thesis-ready when future consumers could otherwise
   mistake its status.
6. Integrate the candidate at its intended size and inspect the exact rendered
   thesis pages at normal reading scale. Standalone source images are not a
   substitute because viewers enlarge them and omit float placement, captions,
   neighboring prose, and page competition.
7. Review reader effect before spending work on final polish and provenance.
   Repair the concept first, then finish regeneration, ownership, and build
   checks.

Keep captions and prose explanations out of the graphic unless text is a real
figure label. Use captions to identify the takeaway and resolve likely false
inferences about projection, coordinates, scaling, selection, or epistemic
status. Put producer provenance outside the thesis caption unless provenance is
itself reader-relevant.

## Review And Completion

Review the rendered thesis pages, not merely the producer or isolated output.
For an important new mathematical figure, separate cold reader-effect review
from mathematical-meaning review; one agent may perform both only as genuinely
separate passes. Use the protocol in `references/review.md`.

A figure is ready when:

- required information is perceptible at normal final-PDF reading size;
- the reviewer's interpretation substantially matches the intended inference;
- the stated reader questions can be answered with less effort than from prose
  alone;
- visual encodings support the intended comparisons and do not invite a
  material undisclosed false inference;
- the mathematics, data, caption, and epistemic claims agree;
- producer ownership, regeneration, thesis copies, and final build are sound;
- remaining defects are genuinely independent polish rather than blocked
  understanding.

Report the reader use, output locations, producer command, rendered-page review,
and any intentionally deferred polish. Do not call a figure ready merely
because it builds, passes invariant checks, or satisfies nominal style values.
