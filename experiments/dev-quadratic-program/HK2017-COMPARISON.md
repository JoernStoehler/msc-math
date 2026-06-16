# HK2017 Comparison

Status: agent-facing comparison note for how this project uses and differs from
Haim--Kislev's finite QP formulation. This is not source truth.

Overruled by: HK2017/HK2019 source paper, `formal/`, `crates/symplectic/`,
thesis text after review, experiment artifacts, and Jörn/Kai review.

## Purpose

Use this file to avoid re-deriving the same HK2017 comparison when drafting the
QP theory chapter or reopening QP implementation work. It separates:

- what HK2017 proves;
- what this thesis explains or translates;
- what this project adds algorithmically on top of the HK2017 theorem.

## Same As HK2017

- The core literature result is the finite quadratic-program formula for
  `c_EHZ` of a convex polytope.
- HK2017 Theorem 1.1 is the source theorem for the finite formula in the local
  paper source and citation cache.
- HK2017 Remark 1.4 is a Clarke dual-action-principle connection, not a theorem.
- HK2017 Theorem 1.5 supplies the simple-minimizer existence result.

## Translation Layer

The project usually states the HK problem in dual-vertex notation rather than in
HK2017's normals/heights notation.

- HK2017 uses outward unit normals `n_i` and heights `h_i = h_K(n_i)`.
- After translating so `0 in int(K)`, the project uses dual vertices
  `a_i = n_i / h_i` and writes
  `K = { x : <a_i, x> <= 1 }`.
- The variable change `b_i = beta_i h_i` turns HK2017's constraints into
  `b_i >= 0`, `sum b_i = 1`, and `sum b_i a_i = 0`.
- The project convention for `J_0` and `omega_0`, and the fixed-word sign/order
  audit, live in `formal/hk2017-qp-conventions.tex`.

## Formulation Differences To Audit

These are not project-original algorithmic contributions by themselves; they are
translation and presentation choices that must be made explicit.

- HK2017 ranges over permutations in its theorem statement. Project formal/code
  surfaces often use subsets, cyclic words, or implementation permutations.
  The equivalence between these representations should be owned by the QP core
  or algorithm statement that uses it.
- Some project surfaces currently use a fixed-word matrix orientation opposite
  to the source-backed HK2017 order. This may be globally equivalent after
  reversing words, but it is not the same fixed-word convention.
- Capacity factors must be named explicitly: a surface may optimize `Q`,
  `(1/2) beta^T H beta`, or `beta^T H beta`.

## Project Solver Layer

HK2017 gives the finite optimization problem. The project also contains
algorithms and implementation work for solving those quadratic programs.

That solver layer is project work beyond merely explaining HK2017. It includes,
depending on the surface:

- enumeration and pruning of candidate words;
- KKT-system solving;
- exact rational solving and f64 solving;
- fallback/certification behavior;
- orbit recovery;
- implementation tests, comparison tests, numerical audits, and performance
  measurements.

This solver layer is out of scope for the current established-theory content
pass. It should be handled in a later algorithm/numerics session and should not
be presented as part of the HK2017 theorem itself.

## Thesis Use

For the established-theory chapter, state the HK finite QP theorem and the
translation needed to use it in thesis notation. Mention our solver layer only
as deferred project work or as later-method context.

For algorithm chapters or implementation-facing notes, distinguish clearly:

- HK2017 theorem: why the finite problem computes the capacity;
- thesis translation: how the theorem is stated in our notation;
- project solver: how our code searches, solves, certifies, and tests the finite
  problems.
