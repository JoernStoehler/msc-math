# Handoff: Simple Minimizer Proof

## What is this file

`simple-minimizer-proof.tex` is an agent-written proof of the Simple Minimizer Theorem (Theorem 5.4 / `thm:simple-minimizer`). It was never approved by Jörn. It is a rough draft at best.

## What happened

1. A previous agent session wrote the proof from scratch, based on HK2017 Section 3 and Jörn's talk notes.
2. Jörn reviewed it and gave feedback (see below).
3. Before Jörn's feedback was addressed, the session ran out of context.
4. A continuation session made unauthorized edits to the file, damaging parts Jörn had implicitly accepted during review.
5. The damaged version was committed as `35ea135` with message "WIP: simple minimizer proof (damaged, needs Jörn's review)".

**Neither the current version nor any previous version was approved by Jörn.** The whole file is agent-written draft.

## Jörn's review feedback (from the previous session)

Jörn reviewed the pre-damage version and flagged these issues:

1. **def:primal-problem** (lines ~59-74): Terminology issue. The definition references "closed characteristic" but the relationship between the Hamiltonian inclusion and the boundary constraint needs clarification. Question: does the Hamiltonian inclusion $-J_0\dot\gamma \in \partial g_K^2(\gamma)$ alone force $g_K(\gamma) \equiv 1$?

2. **Step 3, the $\nu = 1$ argument** (lines ~200-223): Incomplete. Has a `[TODO]` marker. The argument that the Lagrange multiplier $\nu$ equals 1 at the dual minimizer is not finished. This requires Jörn's mathematical input.

3. **Step 4 rescaling** (lines ~356-394): The rescaling construction was disputed. Two mathematically different approaches exist:
   - Space+time rescaling: $z^{(4)}(t) = c \cdot z'''(t/c)$ with $c = T/A(z''')$
   - Pure time reparametrization: $z^{(4)}(t) = z'''(t/c)$ with $c = A(z''')/T$

   The current file has the pure-time version (from the unauthorized edit). Neither version has been verified as correct.

4. **Steps 1, 2, 3, 5** all have `[TODO: details]` markers. The overall structure is there but the proofs are sketches.

## What the file does

The proof has two parts:
- **Section 6.1 (Primal-Dual Equivalence):** Clarke's Dual Action Principle adapted to polytopes. Shows that minimizing action over Reeb orbits on $\partial K$ is equivalent to minimizing $I_K(z)$ over centered closed curves.
- **Section 6.2 (Simplification Algorithm):** 5-step construction turning an arbitrary dual minimizer into a simple one (piecewise linear, pure Reeb vectors, each facet visited at most once).

It is `\input`-ed at line 684 of `chapter-algorithm.tex`.

## Reference materials for rewriting

- **HK2017** (`papers/`): Section 3 contains the 1-page sketch this proof expands.
- **Clarke 1979, 1981**: Original dual action principle for smooth convex bodies.
- **correspondence.tex**: Conventions for $J_0$, $\omega_0$, $\lambda_0$, action, Reeb vectors.
- **chapter-algorithm.tex**: All definitions referenced by the proof (`def:closed-characteristic`, `def:reeb-orbit-smooth`, `def:ehz-capacity`, `def:gauge-function`, `def:support-function`, `def:hamiltonian`, `def:reeb-orbit-polytope`, `def:simple-reeb-orbit`).

## Recommended approach for next agent

1. Read HK2017 Section 3 first.
2. Read the existing file to understand the structure.
3. Ask Jörn which parts to prioritize — this is a large proof with many open issues.
4. Do NOT treat any part of the existing file as verified. Every line is agent-written draft.
