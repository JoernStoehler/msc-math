# Reader Effects

## Quality Through Downstream Use

Classify the figure's main use from the actual thesis context; these are common
uses, not an exhaustive taxonomy.

- **Orientation:** form a mental model of an unfamiliar object, geometry, or
  relation.
- **Process explanation:** externalize states, transitions, persistence, and
  termination so the reader need not hold the sequence in working memory.
- **Comparison:** make selected differences, similarities, or trends directly
  assessable.
- **Evidence or verification:** let the reader assess empirical support,
  numerical behavior, or a construction.
- **Navigation and memory:** provide a stable compact reference for concepts
  reused later.

Evaluate broad quality criteria through that use:

- **Perceptible:** required content can be seen and distinguished in the final
  PDF at normal reading size.
- **Interpretable:** the reader can identify the objects, encodings,
  relationships, and reading order.
- **Task-effective:** the figure answers its reader questions more efficiently
  than the available prose.
- **Non-misleading:** the display does not invite material false comparisons or
  geometric, statistical, or epistemic inferences.
- **Integrated:** prose and caption direct attention to the intended takeaway
  and disclose limitations that affect interpretation.
- **Economical:** the understanding gained justifies page space and reader
  attention.

## Diagnose Causes, Judge Effects

Reason along the chain

```text
visual property -> perceptual distinction -> reader interpretation -> thesis use
```

Font size, contrast, line weight, cropping, object occupancy, resolution, color,
layout, and whitespace are common causes. They are not independent definitions
of quality. For example, a nominally adequate font does not make an overcrowded
sequence interpretable, and increasing font size does not repair ambiguous
depth in a wireframe.

Prefer findings that name the blocked reader task and then diagnose the visual
cause. “The transition from P53 to P31 cannot be followed because its arrow
crosses the next state label” is more actionable and better prioritized than
“arrow placement is awkward.”

## Final-PDF Scale

Judge perceptibility on the rendered thesis page at a realistic whole-page or
normal reading view. An isolated vector graphic can be enlarged indefinitely
and therefore conceals physical-size failures. Check all information required
for the intended use, not only text: geometry may occupy too little area,
curves may be indistinguishable, or panels may be too dense even when every
label can technically be decoded after zooming.

Nominal style constraints such as minimum font sizes are useful lint rules and
regression guards. They reduce common failures but do not replace rendered-page
inspection or reader-effect review.
