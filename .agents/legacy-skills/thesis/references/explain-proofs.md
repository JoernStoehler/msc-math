# Explain Proofs

Use this reference to decide how a proof should become reader-facing
exposition. Apply `mathematical-status.md` to correctness and verification
status.

First identify why the proof belongs here. The reader may need the argument,
the reusable mechanism, the boundary of a computation, a normalization check,
or only confidence supplied by a precise citation. Proof length already spent
elsewhere is not a reason to reproduce it.

Find the proof's explanatory spine: the few changes of viewpoint or
intermediate claims without which the conclusion would look accidental. Make
that spine visible before or while presenting technical steps. State what each
non-obvious construction accomplishes and which hypothesis it uses.

Choose detail according to the reader burden. Expand convention translations,
delicate implications, and project-original steps that a citation cannot make
transparent. Compress routine algebra when the reader can reconstruct it and
when no sign, factor, degeneracy, or quantifier boundary is hidden there.

For a proof backed by generated witnesses, exact verification, or numerical
checks, also read `explain-computation-backed-claims.md`; it owns the verifier,
witness-generation, trust-boundary, and empirical-support distinctions.

Do not turn the proof into a development diary. Failed attempts and agent
review history belong in owner-local notes unless they explain a mathematical
choice the reader needs. A proof is not well explained merely because every
formal step appears; test whether the reader can say what makes the argument
work and where it would fail.
