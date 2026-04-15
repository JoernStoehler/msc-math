# Formal Math Review Checklist

Load `$formal-math-conventions` first. For Rust-linked math, also load `$rust-conventions`.

Check:
- Labels follow the allowed prefix set and are unique across `formal/**/*.tex`.
- Statements have proofs unless marked with an explicit Jörn TODO or GAP.
- Cross-references resolve in the relevant build.
- Rust label references point to statements that match the code's computation.
- The proof states the preconditions used by algebraic operations, inversions, compactness claims, limiting arguments, or genericity assumptions.
- Notation is defined before use or imported from a clearly cited earlier definition.
- Experiment formal files describe formal derivations, not empirical interpretation.

Never claim a proof is correct. Report surface gaps, missing assumptions, and mismatches.
