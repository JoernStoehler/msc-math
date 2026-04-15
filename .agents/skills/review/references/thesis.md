# Thesis Review Checklist

Load `$thesis-tex-conventions` first.

Check:
- The file header states identity, sources, and structure.
- Agent edits inside a `% Jörn:` approved scope remove the approval marker.
- New or revised mathematical content is wrapped in `unverified` unless it is a mechanical notation update or Jörn-approved content.
- Labels and references resolve in `thesis/build/main.aux` after a thesis build.
- Thesis text is self-contained and does not require readers to inspect `library/`, `experiments/`, or `formal/`.
- Figure inclusions are pass-through `\includegraphics{file.png}` with no width or scale arguments.
- Captions state observations, not interpretations.
- Bibliography keys exist in `thesis/bibliography.bib`.

Flag thesis-scope cuts, advisor-facing framing, and mathematical judgment for Jörn rather than deciding them in review.
