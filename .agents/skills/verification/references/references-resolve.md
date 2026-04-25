# References Resolve

## Use When

Use this packet to repeatedly check bibliography, internal thesis
cross-references, theorem/proof-source references, and algorithm/method
references before review or submission.

## Property

Reader-facing references resolve to the cited object, and the cited object says
what the thesis relies on it to say.

## Starter Read Set

1. `tasks/writing.md` for current thesis assembly state.
2. `thesis/` sources and build/check scripts.
3. `formal/` sources when theorem, definition, or proof labels are cited.
4. `research/INDEX.md` and topic research notes for known gaps.
5. Topic task bundles for open proof/writeup obligations.

## Checks

1. Name the thesis surface under review.
2. Build or inspect the relevant reference layer:
   - bibliography entries and citations;
   - internal labels and cross-references;
   - theorem, definition, proof, algorithm, and method references.
3. For each unresolved or suspicious reference, classify it as:
   - missing source;
   - stale label or path;
   - source exists but does not support the cited sentence;
   - thesis wording needs a caveat;
   - Jörn-only proof/reference judgment.
4. Route each retained failure to `tasks/writing.md` or the relevant topic
   bundle.
