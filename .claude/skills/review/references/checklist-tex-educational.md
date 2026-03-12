# Review Checklist: LaTeX Pedagogical Quality (Phase 2)

Detection rules for audience fit, emphasis, and semantic anti-patterns.
Run on clean files (after phase 1 fixes).

## 1. Four Audiences Check

For each section/subsection, check whether it works for all four audiences:

**Human readers** (Jörn, Kai, Elizabeth):
- Is the main result stated upfront?
- Can proofs be skipped without losing the thread?
- Is geometric intuition provided?

**Imaginary master student**:
- Can this be followed linearly without external references?
- Are all definitions stated in full?
- Is the prerequisite knowledge reasonable (linear algebra, analysis, basic topology, intro symplectic geometry, intro optimization)?

**QC agents**:
- Are words used with clear, specific meanings?
- Can each proof step be immediately confirmed as following directly?
- Is anything stated incorrectly (even if non-fatal)?

**Downstream agents** (Rust implementers):
- Are all properties listed (including unused ones)?
- Are concrete values and example calculations provided where needed?

## 2. Forward References

- Flag forward references that force the reader to jump ahead.
- Acceptable: "We will prove this in Section X" (deferred proof).
- Problematic: "Using the notation from Section X below" (notation not yet available).

## 3. Emphasis Proportional to Importance

- Is a full section dedicated to a trivial identity?
- Is an important result buried in a remark?
- Do example calculations get more space than the core results?

## 4. Semantic Anti-Patterns

**AP2: Restating what a definition already says**
Detection: If text after "i.e." or "equivalently" is a direct translation of the preceding statement into different notation, flag it — delete the restatement.

**AP6: Conditions that are always satisfied**
Detection: For each condition in a definition, check whether it's trivially satisfied by the objects the definition applies to. If yes, flag it.

**AP9: Using notation without nearby definition**
Detection: For each notation symbol in a definition/lemma environment, check: is it (a) standard (omega_0, det, etc.), (b) defined within the same environment, or (c) cross-referenced? If none, flag it.

**AP10: Mixing literature citations with novel analysis**
Detection: For each remark containing `\cite`, check whether the remark also contains forward references to our own lemmas/remarks or phrases like "our KKT systems", "we therefore", "for our application". If both present, flag — should be split.

## 5. Geometric Definitions First

- Are definitions given geometrically first, with formulas derived?
- Detection: definitions that jump straight to coordinate expressions without geometric motivation.

## 6. Standard Definitions

- Are standard definitions used exactly as in the literature?
- If a different form is used, is there a lemma proving equivalence?
