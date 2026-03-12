# Review Checklist: Experiment Observations, Interpretation, and Notes (Phase 2)

Detection rules for experiment writeup accuracy and quality. Run on clean files (after phase 1 fixes).

---

## Part A: Factual Accuracy (Observations)

Verify reported facts in `.tex` writeups against actual data files.

### What Counts as a Factual Claim

- **Statistics**: "mean sys = 0.87", "73% of polytopes have sys < 1" → compute from JSONL.
- **Counts**: "27 polytopes", "10 facets" → count in JSONL.
- **Extremes**: "maximum sys = 1.03", "the pentagon achieves the highest sys" → verify from data.
- **Comparisons**: "Lagrangian products have higher sys than general polytopes" → compute both distributions.
- **Figure descriptions**: "Figure 3 shows a clear clustering around sys = 0.9" → read the PNG.
- **Code outputs**: "the assertion passed for all inputs" → check `_output.txt`.

### How to Verify

1. **JSONL data**: Read `experiments/<name>/<name>.jsonl`, parse JSON, compute the claimed statistic.
2. **Figures**: Read the `.png` file and visually check if the description matches.
3. **Stdout captures**: Read `experiments/<name>/<name>_output.txt`.
4. **Cross-experiment claims**: May require reading data from multiple experiments.

### Red Flags

- Claims without data source (where did this number come from?).
- Claims that are close but not exact (rounding errors, stale data).
- Claims about "all" or "none" that might have exceptions.
- Missing `% [TODO: JÖRN -` or `% [GAP -` markers on unverifiable claims.

---

## Part B: Interpretation Quality

Check that writing correctly separates facts from interpretation and doesn't overreach.

### 1. Unlabeled Interpretation

Interpretations include:
- Causal claims: "X causes Y", "because of X", "due to X"
- Explanatory claims: "this is explained by", "the reason is"
- Mechanistic claims: "the algorithm exploits this by"

These MUST be labeled as interpretation in the text, not presented as findings.
Detection: grep for "because", "due to", "causes", "explains", "the reason", "this means" — check if the surrounding text frames these as interpretation.

### 2. Causal Claims from Correlations

- "Higher facet count leads to lower sys" — causal claim.
- "Higher facet count correlates with lower sys" — observation.
- Detection: grep for "leads to", "results in", "produces", "creates" — these imply causation.

### 3. Overreach Beyond Data

- Claims about "all polytopes" when only a sample was tested.
- Claims about "general" behavior from specific examples.
- Claims about statistical significance without p-values or confidence intervals.
- Extrapolation beyond the tested range.

### 4. Omitted Patterns

- Read the data and figures. Are there visible patterns NOT mentioned in the writeup?
- This is the "don't omit things" rule — the writeup should report what's there.

### 5. Editorial Language

Flag language that editorializes rather than reports:
- "Surprisingly", "interestingly", "remarkably" — inject the author's reaction.
- "Unfortunately", "sadly" — inject judgment.
- "Clearly", "obviously" — can mask non-obvious claims.
- Exception: brief editorial framing is OK if it helps the reader focus.

### 6. Balanced Reporting

- Are limitations and caveats mentioned?
- Are negative results reported alongside positive ones?
- Is the "so what" question addressed without overstating?

---

## Part C: Notes/README Quality

Check experiment README.md files for structure and completeness.

### README Structure

Each experiment's `README.md` should document:
- What the experiment does (goal/question).
- Current status (where it sits on the investigative spectrum).
- Key findings so far.
- How to run (commands for binary and script).
- Any caveats or known limitations.

### Assumptions Documented

- If the experiment assumes data files exist, say which ones and how to generate them.
- Example: "Assumes benchmark.jsonl exists. Run: cd experiments/ && cargo run --bin benchmark --release"

### Philosophy Alignment

- Experiments are **always investigative** — the README should not claim an experiment is "finished" or "stable."
- Language should reflect the continuous spectrum: "current findings", "so far", not "final results."
- No discrete stage labels ("Phase 1 complete") — use descriptive status instead.

### Staleness Check

- Do the README's claims match the current data files?
- Does the README reference files that exist?
- Are there data files or figures not mentioned in the README?
