# Explain Computation-Backed Claims

Use this reference when thesis or formal exposition relies on generated
witnesses, exact verification, numerical checks, or other computation-bearing
support.

State the mathematical implication from checked predicates to the claimed
result before explaining implementation. Distinguish:

- the theorem or subclaim being established;
- the finite predicates checked by the verifier;
- witness generation or search, which need not be trusted when verification
  is sufficient;
- the verifier and its mathematical trust boundary;
- empirical or numerical checks used only for debugging, orientation, or
  supporting evidence.

Explain code through the reader's proof or audit obligation. Include only
short excerpts, tables, or witness fragments that materially reduce audit
cost. Keep full source, run manuals, outputs, and artifacts with their owner.

Do not turn execution success into mathematical evidence beyond the exact
conditions checked, and do not describe explanatory or empirical assets as
proof input unless the proof actually uses them.
