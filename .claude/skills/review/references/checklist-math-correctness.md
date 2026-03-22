# Review Checklist: Mathematical Correctness and Clarity

Shallow correctness and clarity checks for mathematical writing (proofs, derivations, lemma statements). This checklist targets errors that are mechanical to detect but catastrophic to miss.

**This is the ONLY concern for this review.** Do not check style, formatting, cross-references, or anything else.

**Confidence levels** — be explicit:
- **High confidence**: "This is wrong because [specific reason]"
- **Moderate confidence**: "This step seems to skip [specific gap], but I may be missing something"
- **Low confidence / needs Jörn**: "I cannot verify this step — flagging for expert review"

Report uncertain findings. Jörn prefers false positives over missed errors.

## 1. Unargued claims

For every statement presented as true: is there an argument, a reference, or an explicit "by assumption"? Flag any claim that appears without justification.

Detection pattern: statements like "X holds", "we have X", "X is true" without a preceding "because", "by", "since", "from", or a reference.

## 2. Handwavy arguments

For every argument: does it make a direct logical connection from premise to conclusion? Flag arguments that gesture at a connection without making it explicit.

Detection patterns:
- "It follows that" — does it actually follow? What's the reasoning step?
- "Clearly" / "obviously" / "it is easy to see" — is it? What's the specific argument?
- "By similar reasoning" — similar to what? Is the analogy actually valid?
- "This gives us" — what operation or theorem produces this result?
- Long gap between the last concrete step and the conclusion

## 3. Missing conditions

For every operation or theorem application: are all required conditions established?

Detection patterns:
- Division by an expression → was it shown to be nonzero?
- Inverse of a matrix/operator → was invertibility established?
- Limit interchange (integral/sum/derivative) → was the convergence condition checked?
- Application of a theorem → are ALL hypotheses of that theorem verified, not just some?
- Supremum/infimum → was the set shown to be nonempty / bounded?
- Square root → was the argument shown to be nonnegative?

## 4. Logical gaps between steps

For each consecutive pair of steps in a proof: can you see how step N+1 follows from step N (plus previously established facts)? Flag any jump that requires a non-obvious intermediate step.

Detection pattern: read step N, then step N+1. Ask: "what's the one-sentence argument connecting these?" If you can't state it, flag the gap.

## 5. Quantifier errors

- Is every variable properly quantified (for all / there exists)?
- Is the quantifier order correct? (∀x ∃y vs ∃y ∀x)
- Are quantifiers in the right scope?

## 6. Clarity issues that hide errors

These are not correctness errors themselves, but they make correctness errors harder to detect — for both Jörn and agents.

- Notation used before defined
- Same symbol used for two different things
- Proof step that depends on something stated 20+ lines earlier without reference
- Definition that is technically correct but obscures what's actually happening
- Proof that establishes a result different from what was claimed (subtle mismatch between theorem statement and proof conclusion)
