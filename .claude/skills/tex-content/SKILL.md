---
name: tex-content
description: LaTeX content and correctness standards for .tex files. Load when writing mathematical content, proofs, definitions, or theorem statements. Covers four audiences, correctness criteria, self-containedness, proof writing structure, notation consistency, and citation verification.
---

# LaTeX Content Conventions

## Four Audiences

Every line of LaTeX must work for all four audiences simultaneously:

1. **Human readers** (Jörn, Kai, Elizabeth) — want the main result upfront, will skim definitions, all proofs are skippable, value algorithm/proof ideas/geometric intuition
2. **Imaginary master student** (nominal target) — typical math master background, every definition stated in full, must follow linearly without external references
3. **QC agents** (verification) — verify one chunk at a time, every proof step must immediately confirm "yes, that follows directly", words must have clear specific meanings, never state anything incorrect even if non-fatal
4. **Downstream agents** (Rust implementers, test writers) — need full detail in all definitions/lemmas/proofs, need ALL properties listed (including unused ones), need concrete values and example calculations

## Correctness

We write mathematics, code, and documentation in a clear, detailed, explicit, structured, verifiable way:
- "clear" = easy to understand, not vague or ambiguous
- "explicit" = relevant implications already spelled out
- "detailed" = all steps included for verification
- "structured" = organized into modular chunks
- "verifiable" = reader can check correctness via local validity checks

We refactor, simplify, and improve until verification becomes straightforward.

## Content Rules

1. **Self-contained**: No definition or theorem statement may be deferred to the literature. Every definition and theorem is stated in full.
2. **Deferred proofs**: A proof MAY be deferred to the literature ONLY IF the theorem exceeds thesis scope AND the proof is not relevant — only the theorem is.
3. **Notation consistency**: Must match `correspondence.tex` exactly. No synonyms, no alternative forms, no "equivalent" restatements unless a lemma proving equivalence is included.
4. **Writing rule**: Proofs cannot cite external sources mid-proof. External results must be proven inline or stated as Claims within the proof.
5. **Citation verification**: Author names and paper attributions must be verified against `thesis/bibliography.bib` or `papers/`. Never produce author names from memory.

## Proof Writing

**Structure**: Assumptions → Claim → Overview → Steps → Conclusion

**Level of detail**:
- Detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave

**Agent limitations — what agents CAN do:**
- Turn natural language descriptions into proofs
- Improve proof writing, fix errors, detect suspicious steps

**What agents CANNOT do:**
- Provide final high-reliability verification — that must come from Jörn
- Agent skill at spotting errors is specifically "only okay"

**Every proof must pass Jörn's verification after every edit.**

## Emphasis and Structure

- **Emphasis proportional to importance**: Don't dedicate a full section to a trivial identity
- **Geometric definitions first, formulas derived**: E.g., action = integral of Liouville form, not the coordinate expression
- **Standard definitions**: Use them exactly as in the literature. If you use a different form, state a lemma proving equivalence

## Rust Cross-References

Rust `///` doc comments reference thesis proofs using `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` format. When editing a theorem or lemma in the thesis, grep `crates/src/` for the label to find affected Rust comments.

## The Core Rule

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code. "The data shows Y" requires reading the data. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`.

**Citation verification:** Never produce author names, paper titles, or literature attributions from memory. Always verify against `thesis/bibliography.bib` or `papers/`.
